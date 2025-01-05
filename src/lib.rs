pub mod auth;
pub mod coin;
pub mod err;
pub mod fund;
pub mod math;
pub mod valid;

pub use auth::*;
pub use coin::*;
pub use err::*;
pub use fund::*;
pub use math::*;
pub use valid::*;

/// Type alias for `std::result::Result` with contract defaults.
pub(crate) type Result<T = (), E = XcosmError> = core::result::Result<T, E>;
