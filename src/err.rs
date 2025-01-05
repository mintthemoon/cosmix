use cosmwasm_std::StdError;
use derive_more::{Display, Error, From};

/// Contract error.
#[derive(Debug, Display, Error, From)]
pub enum XcosmError {
  /// Coins do not meet the expected amount.
  #[from(ignore)]
  #[display("Coins must contain at least {expect}")]
  CoinsInsufficient { expect: String },

  /// Action requires exact coins.
  #[from(ignore)]
  #[display("Coins must be exactly {expect}")]
  CoinsNotExact { expect: String },

  /// Expected no coins, but received some.
  #[display("Coins must not be provided")]
  CoinsNotAllowed {},

  /// Coins lists cannot have duplicate denoms.
  #[from(ignore)]
  #[display("Coins must not contain duplicates: {denom:?}")]
  CoinsDuplicate { denom: Option<String> },

  /// Input/output match error for sending coins.
  #[display("Input and output coins must have equal values")]
  CoinsMismatch {},

  #[display("Fund claims must not exceed 100%")]
  FundsOverclaimed {},

  #[display("Fund claims must not be empty")]
  FundsUnclaimed {},

  #[display("Overflow in math operation")]
  MathOverflow {},

  #[display("Underflow in math operation")]
  MathUnderflow {},

  #[display("Divide by zero in math operation")]
  MathDivByZero {},

  /// Validate error.
  #[from(ignore)]
  #[display("Not a valid {kind}: {reason}")]
  Invalid { kind: String, reason: String },

  /// CosmWasm standard error.
  CosmWasm { source: StdError },

  /// Auth error.
  #[display("Not authorized for this action")]
  Unauthorized {},

  /// Action disabled error.
  #[display("This action is disabled")]
  Disabled {},

  /// Input parsing error.
  #[display("Unable to parse input value")]
  Parse {},

  #[display("Error: {msg}")]
  Any { msg: String },
}

impl XcosmError {
  pub fn coins_insufficient(expected: impl Into<String>) -> Self {
    Self::CoinsInsufficient {
      expect: expected.into(),
    }
  }

  pub fn coins_not_exact(expected: impl Into<String>) -> Self {
    Self::CoinsNotExact {
      expect: expected.into(),
    }
  }

  pub fn coins_not_allowed() -> Self {
    Self::CoinsNotAllowed {}
  }

  pub fn disabled() -> Self {
    Self::Disabled {}
  }

  pub fn parse() -> Self {
    Self::Parse {}
  }

  pub fn unauthorized() -> Self {
    Self::Unauthorized {}
  }

  pub fn invalid(kind: impl Into<String>, msg: impl Into<String>) -> Self {
    Self::Invalid {
      kind: kind.into(),
      reason: msg.into(),
    }
  }

  pub fn any(err: impl Into<String>) -> Self {
    Self::Any { msg: err.into() }
  }
}

impl<T> Into<Result<T, XcosmError>> for XcosmError {
  fn into(self) -> Result<T, XcosmError> {
    Err(self)
  }
}

impl Into<StdError> for XcosmError {
  /// Convert contract error into CosmWasm standard error.
  fn into(self) -> StdError {
    match self {
      XcosmError::CosmWasm { source } => source,
      _ => StdError::generic_err(self.to_string()),
    }
  }
}

impl From<cosmwasm_std::CoinsError> for XcosmError {
  /// Convert a [`CoinsError`] into an [`XcosmError`].
  fn from(e: cosmwasm_std::CoinsError) -> Self {
    match e {
      cosmwasm_std::CoinsError::DuplicateDenom => XcosmError::CoinsDuplicate { denom: None },
    }
  }
}

impl From<cosmwasm_std::OverflowError> for XcosmError {
  fn from(_: cosmwasm_std::OverflowError) -> Self {
    Self::MathOverflow {}
  }
}

/// Trait for conversions between result types.
pub trait IntoResult<T, E> {
  /// Convert result to target type.
  fn into_result(self) -> Result<T, E>;
}

impl<T, E, F: Into<E>> IntoResult<T, E> for Result<T, F> {
  fn into_result(self) -> Result<T, E> {
    self.map_err(Into::into)
  }
}

pub trait FromResult<T, E> {
  fn from_result(res: Result<T, E>) -> Self;
}

impl<T, E: Into<F>, F> FromResult<T, E> for Result<T, F> {
  fn from_result(res: Result<T, E>) -> Self {
    res.map_err(Into::into)
  }
}
