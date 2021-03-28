mod engine;
mod error;
pub mod types;

pub use engine::PaymentEngine;
pub use error::Error;

/// A `Result` alias where the `Err` case is `crate::Error`
type Result<T> = std::result::Result<T, Error>;
