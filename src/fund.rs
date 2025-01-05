use std::collections::{hash_map::Entry, HashMap};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Api, Coin, CosmosMsg, MessageInfo};
use derive_more::{Deref, DerefMut};

use crate::{
  ApiValidator, CoinSet, IntoResult, Result, TryMinusMut, TryPlusMut, Validator, XcosmError,
};

#[cw_serde]
#[derive(Copy)]
pub struct Claim(u32);

impl Claim {
  pub fn bps(&self) -> u32 {
    self.0
  }

  pub fn claim(&self, funds: &CoinSet) -> Result<CoinSet> {
    let mut claimed = funds.clone();
    for (_, amount) in claimed.iter_mut() {
      *amount = self.claim_amount(amount.u128())?.into();
    }
    Ok(claimed)
  }

  pub fn claim_amount(&self, total: u128) -> Result<u128> {
    total
      .checked_mul(self.bps() as u128)
      .ok_or_else(|| XcosmError::MathOverflow {})?
      .checked_div(100000u128)
      .ok_or_else(|| XcosmError::MathDivByZero {})
      .into_result()
  }
}

#[cw_serde]
#[derive(Deref, DerefMut)]
pub struct Distribution(HashMap<Addr, Claim>);

impl Distribution {
  pub fn new(claims: HashMap<Addr, Claim>) -> Self {
    Distribution(claims)
  }

  pub fn claims(&self) -> &HashMap<Addr, Claim> {
    &self.0
  }

  pub fn total_bps(&self) -> Result<u32> {
    let total = self.claims().iter().map(|(_, claim)| claim.bps()).sum();
    if total > 10000 {
      return XcosmError::FundsOverclaimed {}.into();
    }
    Ok(total)
  }

  pub fn with_remainder_to(&self, addr: Addr) -> Result<Self> {
    let rem_claim = Claim(10000 - self.total_bps()?);
    let mut claims = self.claims().clone();
    match claims.entry(addr) {
      Entry::Vacant(entry) => {
        entry.insert(rem_claim);
      }
      Entry::Occupied(mut entry) => {
        let claim = entry.get_mut();
        claim.0 += rem_claim.bps();
      }
    }
    Ok(Self(claims))
  }

  pub fn distribute_coins(&self, from: &Addr, funds: &CoinSet) -> Result<CosmosMsg> {
    if self.claims().len() == 0 {
      return XcosmError::FundsUnclaimed {}.into();
    }
    let mut rem = funds.clone();
    let mut claimed = self
      .claims()
      .iter()
      .map(|(addr, claim)| {
        let claimed = claim.claim(funds)?;
        rem.try_minus_mut(&claimed)?;
        Ok((addr, claim.claim(funds)?))
      })
      .collect::<Result<Vec<(&Addr, CoinSet)>>>()?;
    // give remainder to first claim
    // TODO make this behavior configurable
    claimed
      .first_mut()
      .map(|(_, coins)| coins.try_plus_mut(&rem))
      .transpose()?
      .ok_or_else(|| {
        XcosmError::any(
          "Unexpected error: claims are not empty but claimed funds are empty".to_string(),
        )
      })?;
    funds.send_many(from, claimed).into_result()
  }
}

impl Default for Distribution {
  fn default() -> Self {
    Self::new(HashMap::new())
  }
}

impl From<HashMap<Addr, Claim>> for Distribution {
  fn from(claims: HashMap<Addr, Claim>) -> Self {
    Self(claims)
  }
}

impl Into<DistributionMsg> for Distribution {
  fn into(self) -> DistributionMsg {
    DistributionMsg(
      self
        .iter()
        .map(|(addr, claim)| (addr.to_string(), *claim))
        .collect(),
    )
  }
}

#[cw_serde]
#[derive(Deref, DerefMut)]
pub struct DistributionMsg(HashMap<String, Claim>);

impl<'a> ApiValidator<'a, Distribution> for &DistributionMsg {
  fn api_validate(self, api: &dyn Api) -> Result<Distribution> {
    self
      .iter()
      .map(|(addr_str, claim)| Ok::<_, XcosmError>((api.validate(&addr_str)?, *claim)))
      .collect::<Result<HashMap<Addr, Claim>>>()
      .map(Into::into)
  }
}

pub trait MessageFunds {
  fn expect_funds(&self, expected: impl IntoIterator<Item = Coin>) -> Result;
  fn expect_funds_exact(&self, expected: impl IntoIterator<Item = Coin>) -> Result;
  fn expect_no_funds(&self) -> Result;
  fn fund_set(&self) -> Result<CoinSet>;
}

impl MessageFunds for MessageInfo {
  fn expect_funds<'a>(&self, expected: impl IntoIterator<Item = Coin>) -> Result {
    self.fund_set()?.expect_coins_exact(expected)
  }

  fn expect_funds_exact<'a>(&self, expected: impl IntoIterator<Item = Coin>) -> Result {
    self.fund_set()?.expect_coins_exact(expected)
  }

  fn expect_no_funds(&self) -> Result {
    self.fund_set()?.expect_none()
  }

  fn fund_set(&self) -> Result<CoinSet> {
    self.funds.clone().try_into()
  }
}
