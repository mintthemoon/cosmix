use cosmwasm_std::{Addr, Api};

pub type ValidateResult<T=()> = Result<T, ValidateError>;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum ValidateError {
  #[error("Not a valid {kind:?}: {reason:?}")]
  NotValid { kind: String, reason: String },
}

/// Data validation trait.
pub trait Validator<T, U> {
  /// Validate a value.
  fn validate(self, val: T) -> Result<U, ValidateError>;
}

pub trait ApiValidator<'a, T> {
  fn api_validate(self, api: &'a dyn Api) -> ValidateResult<T>;
}

impl<'a, T: ApiValidator<'a, U>, U> Validator<T, U> for &'a dyn Api {
  fn validate(self, val: T) -> ValidateResult<U> {
    val.api_validate(self)
  }
}

impl<'a, T: AsRef<str>> ApiValidator<'a, Addr> for &'a T {
  fn api_validate(self, api: &'a dyn Api) -> ValidateResult<Addr> {
    api
      .addr_validate(self.as_ref())
      .map_err(|err| ValidateError::NotValid {
        kind: "address".to_string(),
        reason: err.to_string(),
      })
  }
}
