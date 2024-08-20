use cosmwasm_std::Coin;

use crate::CoinSet;

pub type MathResult<T=()> = Result<T, MathError>;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum MathError {
  #[error(transparent)]
  Container(#[from] ContainerError),
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum ContainerError {
  #[error("Overflow in math operation")]
  Overflow {},

  #[error("Underflow in math operation")]
  Underflow {},
}

impl From<cosmwasm_std::OverflowError> for MathError {
  fn from(_: cosmwasm_std::OverflowError) -> Self {
    ContainerError::Overflow {}.into()
  }
}

impl TryPlus<&Coin> for CoinSet {
  type Output = Self;
  type Error = MathError;

  fn try_plus(&self, other: &Coin) -> MathResult<Self> {
    let mut res = self.clone();
    let mut is_err = false;
    res
      .entry(other.denom.clone())
      .and_modify(|amount| match (*amount).checked_add(other.amount) {
        Ok(amt) => *amount = amt,
        Err(_) => is_err = true,
      });
    match is_err {
      true => Err(ContainerError::Overflow {}.into()),
      false => Ok(res),
    }
  }
}

impl TryPlusMut<&Coin> for CoinSet {
  type Error = MathError;

  fn try_plus_mut(&mut self, other: &Coin) -> MathResult {
    let mut is_err = false;
    self.entry(other.denom.clone()).and_modify(|amount| {
      match (*amount).checked_sub(other.amount) {
        Ok(amt) => *amount = amt,
        Err(_) => is_err = true,
      }
    });
    match is_err {
      true => Err(ContainerError::Underflow {}.into()),
      false => Ok(()),
    }
  }
}

impl TryPlus<&CoinSet> for CoinSet {
  type Output = Self;
  type Error = MathError;

  fn try_plus(&self, other: &CoinSet) -> MathResult<Self> {
    let mut res = self.clone();
    let mut is_err = false;
    for (denom, amount) in other.iter() {
      res.entry(denom.clone()).and_modify(|self_amount| {
        match (*self_amount).checked_add(*amount) {
          Ok(amt) => *self_amount = amt,
          Err(_) => is_err = true,
        }
      });
    }
    match is_err {
      true => Err(ContainerError::Overflow {}.into()),
      false => Ok(res),
    }
  }
}

impl TryPlusMut<&CoinSet> for CoinSet {
  type Error = MathError;

  fn try_plus_mut(&mut self, other: &CoinSet) -> MathResult {
    let mut is_err = false;
    for (denom, amount) in other.iter() {
      self.entry(denom.clone()).and_modify(|self_amount| {
        match (*self_amount).checked_add(*amount) {
          Ok(amt) => *self_amount = amt,
          Err(_) => is_err = true,
        }
      });
    }
    match is_err {
      true => Err(ContainerError::Overflow {}.into()),
      false => Ok(()),
    }
  }
}

pub trait TryPlus<T> {
  type Output;
  type Error;

  fn try_plus(&self, other: T) -> Result<Self::Output, Self::Error>;
}

pub trait TryPlusMut<T> {
  type Error;

  fn try_plus_mut(&mut self, other: T) -> Result<(), Self::Error>;
}

pub trait TryMinus<T> {
  type Output;
  type Error;

  fn try_minus(&self, other: T) -> Result<Self::Output, Self::Error>;
}

pub trait TryMinusMut<T> {
  type Error;

  fn try_minus_mut(&mut self, other: T) -> Result<(), Self::Error>;
}

impl TryMinus<&Coin> for CoinSet {
  type Output = Self;
  type Error = MathError;

  fn try_minus(&self, other: &Coin) -> MathResult<Self> {
    let mut res = self.clone();
    let mut is_err = false;
    res
      .entry(other.denom.clone())
      .and_modify(|amount| match (*amount).checked_sub(other.amount) {
        Ok(amt) => *amount = amt,
        Err(_) => is_err = true,
      });
    match is_err {
      true => Err(ContainerError::Underflow {}.into()),
      false => Ok(res),
    }
  }
}

impl TryMinusMut<&Coin> for CoinSet {
  type Error = MathError;

  fn try_minus_mut(&mut self, other: &Coin) -> MathResult {
    let mut is_err = false;
    self.entry(other.denom.clone()).and_modify(|amount| {
      match (*amount).checked_sub(other.amount) {
        Ok(amt) => *amount = amt,
        Err(_) => is_err = true,
      }
    });
    match is_err {
      true => Err(ContainerError::Underflow {}.into()),
      false => Ok(()),
    }
  }
}