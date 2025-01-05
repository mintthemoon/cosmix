use cosmwasm_std::Coin;

use crate::{CoinSet, Result, XcosmError};

pub trait TryPlus<T> {
  type Output;
  type Error;

  fn try_plus(&self, other: T) -> Result<Self::Output, Self::Error>;
}

impl TryPlus<&Coin> for CoinSet {
  type Output = Self;
  type Error = XcosmError;

  fn try_plus(&self, other: &Coin) -> Result<Self> {
    let mut res = self.clone();
    let mut is_err = false;
    res
      .entry(other.denom.clone())
      .and_modify(|amount| match (*amount).checked_add(other.amount) {
        Ok(amt) => *amount = amt,
        Err(_) => is_err = true,
      });
    match is_err {
      true => XcosmError::MathOverflow {}.into(),
      false => Ok(res),
    }
  }
}

impl TryPlus<&CoinSet> for CoinSet {
  type Output = Self;
  type Error = XcosmError;

  fn try_plus(&self, other: &CoinSet) -> Result<Self> {
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
      true => XcosmError::MathOverflow {}.into(),
      false => Ok(res),
    }
  }
}

pub trait TryPlusMut<T> {
  type Error;

  fn try_plus_mut(&mut self, other: T) -> Result<(), Self::Error>;
}

impl TryPlusMut<&Coin> for CoinSet {
  type Error = XcosmError;

  fn try_plus_mut(&mut self, other: &Coin) -> Result {
    let mut is_err = false;
    self.entry(other.denom.clone()).and_modify(|amount| {
      match (*amount).checked_sub(other.amount) {
        Ok(amt) => *amount = amt,
        Err(_) => is_err = true,
      }
    });
    match is_err {
      true => XcosmError::MathOverflow {}.into(),
      false => Ok(()),
    }
  }
}

impl TryPlusMut<&CoinSet> for CoinSet {
  type Error = XcosmError;

  fn try_plus_mut(&mut self, other: &CoinSet) -> Result {
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
      true => XcosmError::MathOverflow {}.into(),
      false => Ok(()),
    }
  }
}

pub trait TryMinus<T> {
  type Output;
  type Error;

  fn try_minus(&self, other: T) -> Result<Self::Output, Self::Error>;
}

impl TryMinus<&Coin> for CoinSet {
  type Output = Self;
  type Error = XcosmError;

  fn try_minus(&self, other: &Coin) -> Result<Self> {
    let mut res = self.clone();
    let mut is_err = false;
    res
      .entry(other.denom.clone())
      .and_modify(|amount| match (*amount).checked_sub(other.amount) {
        Ok(amt) => *amount = amt,
        Err(_) => is_err = true,
      });
    match is_err {
      true => XcosmError::MathUnderflow {}.into(),
      false => Ok(res),
    }
  }
}

impl TryMinus<&CoinSet> for CoinSet {
  type Output = Self;
  type Error = XcosmError;

  fn try_minus(&self, other: &CoinSet) -> Result<Self> {
    let mut res = self.clone();
    let mut is_err = false;
    for (denom, amount) in other.iter() {
      res.entry(denom.clone()).and_modify(|self_amount| {
        match (*self_amount).checked_sub(*amount) {
          Ok(amt) => *self_amount = amt,
          Err(_) => is_err = true,
        }
      });
    }
    match is_err {
      true => XcosmError::MathUnderflow {}.into(),
      false => Ok(res),
    }
  }
}

pub trait TryMinusMut<T> {
  type Error;

  fn try_minus_mut(&mut self, other: T) -> Result<(), Self::Error>;
}

impl TryMinusMut<&Coin> for CoinSet {
  type Error = XcosmError;

  fn try_minus_mut(&mut self, other: &Coin) -> Result {
    let mut is_err = false;
    self.entry(other.denom.clone()).and_modify(|amount| {
      match (*amount).checked_sub(other.amount) {
        Ok(amt) => *amount = amt,
        Err(_) => is_err = true,
      }
    });
    match is_err {
      true => XcosmError::MathUnderflow {}.into(),
      false => Ok(()),
    }
  }
}

impl TryMinusMut<&CoinSet> for CoinSet {
  type Error = XcosmError;

  fn try_minus_mut(&mut self, other: &CoinSet) -> Result {
    let mut is_err = false;
    for (denom, amount) in other.iter() {
      self.entry(denom.clone()).and_modify(|self_amount| {
        match (*self_amount).checked_sub(*amount) {
          Ok(amt) => *self_amount = amt,
          Err(_) => is_err = true,
        }
      });
    }
    match is_err {
      true => XcosmError::MathUnderflow {}.into(),
      false => Ok(()),
    }
  }
}
