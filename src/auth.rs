use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

pub type AuthResult<T=()> = Result<T, AuthError>;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum AuthError {
  #[error("Requestor is not authorized")]
  Unauthorized {},
}

/// Auth handler.
#[cw_serde]
pub enum Authorized<T: Eq+ToString=Addr> {
  /// Single authorized address.
  One(T),
  /// Multiple authorized addresses.
  Many(Vec<T>),
}

impl<T: Eq+ToString> Authorized<T> {
  /// Authorize a single requestor.
  ///
  /// Requires requestor to match authorized.
  pub fn authorize(&self, requestor: &T) -> AuthResult {
    match self {
      Authorized::One(authorized) => {
        if authorized != requestor {
          return Err(AuthError::Unauthorized {});
        }
      }
      Authorized::Many(authorized) => {
        if !authorized.contains(requestor) {
          return Err(AuthError::Unauthorized {});
        }
      }
    };
    Ok(())
  }

  /// Authorize any of the requestors.
  ///
  /// Requires at least one of `requestors` to match authorized.
  pub fn authorize_any(&self, requestors: &Vec<T>) -> AuthResult {
    match match self {
      Authorized::One(authorized) => requestors.contains(authorized),
      Authorized::Many(authorized) => requestors.iter().any(|r| authorized.contains(r)),
    } {
      true => Ok(()),
      false => Err(AuthError::Unauthorized {}),
    }
  }

  /// Authorize all of the requestors.
  ///
  /// Requires all of `requestors` to match authorized.
  pub fn authorize_all(&self, requestors: &Vec<T>) -> AuthResult {
    match match self {
      Authorized::One(authorized) => requestors.contains(authorized),
      Authorized::Many(authorized) => requestors.iter().all(|r| authorized.contains(r)),
    } {
      true => Ok(()),
      false => Err(AuthError::Unauthorized {}),
    }
  }

  /// Authorize at least `min` of the requestors.
  ///
  /// Requires at least `min` of `requestors` to match authorized.
  pub fn authorize_at_least(&self, requestors: &Vec<T>, min: u32) -> AuthResult {
    match match self {
      Authorized::One(authorized) => requestors.contains(authorized),
      Authorized::Many(authorized) => {
        requestors.iter().filter(|r| authorized.contains(r)).count() as u32 >= min
      }
    } {
      true => Ok(()),
      false => Err(AuthError::Unauthorized {}),
    }
  }
}
