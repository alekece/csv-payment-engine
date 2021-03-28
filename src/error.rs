use std::io;
use thiserror::Error;

/// The errors that may occurs when using this crate.
#[derive(Debug, Error)]
pub enum Error {
  #[error(transparent)]
  IoError(#[from] io::Error),
  #[error(transparent)]
  CsvError(#[from] csv::Error),
  #[error("Invalid transaction: missing amount")]
  MissingAmount,
  #[error("Unknown operation '{0}'")]
  UnknownOperation(String),
  #[error("Duplicated transaction '{0}")]
  DuplicatedTransaction(u32),
  #[error("Transaction '{0}' already executed")]
  AlreadyExecutedTransaction(u32),
  #[error("Could not dispute transaction '{0}'")]
  DisputeTransactionError(u32),
  #[error("Could not resolve transaction '{0}'")]
  ResolveTransactionError(u32),
  #[error("Could not chargeback transaction '{0}'")]
  ChargebackTransactionError(u32),
}
