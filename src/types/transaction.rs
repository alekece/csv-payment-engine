use crate::{types::Client, Error, Result};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// A view into a single entry in a CSV
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TransactionView {
  #[serde(rename = "type")]
  pub operation_type: String,
  #[serde(rename = "tx")]
  pub transaction_id: u32,
  #[serde(rename = "client")]
  pub client_id: u16,
  pub amount: Option<f32>,
}

pub enum Operation {
  /// A deposit is a credit to the client’s asset account
  Deposit(f32),
  /// A withdraw is a debit to the client’s asset account
  Withdraw(f32),
  /// A dispute represents a client’s claim that a transaction was erroneous and should be reverse
  Dispute,
  /// A resolve represents a resolution to a dispute, releasing the associated held funds
  Resolve,
  /// A chargeback is the final state of a dispute and represents the client reversing a transaction
  Chargeback,
}

impl TryFrom<(String, Option<f32>)> for Operation {
  type Error = crate::Error;

  fn try_from((operation_type, amount): (String, Option<f32>)) -> Result<Self> {
    Ok(match operation_type.as_str() {
      "deposit" => Ok(Operation::Deposit(amount.ok_or(Error::MissingAmount)?)),
      // NOTE: there is a inconsistency in the document
      // ("withdrawal" in example and "withdraw" in the precision section)
      "withdraw" | "withdrawal" => Ok(Operation::Withdraw(amount.ok_or(Error::MissingAmount)?)),
      "dispute" => Ok(Operation::Dispute),
      "resolve" => Ok(Operation::Resolve),
      "chargeback" => Ok(Operation::Chargeback),
      _ => Err(Error::UnknownOperation(operation_type)),
    }?)
  }
}

pub enum TransactionType {
  Input(f32),
  Output(f32),
}

/// Indicates the status of a transaction.
///
/// A transaction status defaults on `NonExecuted` status.
#[derive(Copy, Clone)]
enum TransactionStatus {
  NonExecuted,
  Disputed,
  Executed,
}

pub struct Transaction {
  id: u32,
  r#type: TransactionType,
  status: TransactionStatus,
}

impl Transaction {
  /// Creates input transaction.
  ///
  /// New input transaction is always non executed.
  pub fn new_input(id: u32, amount: f32) -> Self {
    Self {
      id,
      r#type: TransactionType::Input(amount),
      status: TransactionStatus::NonExecuted,
    }
  }

  /// Creates output transaction.
  ///
  /// New output transaction is always non executed.
  pub fn new_output(id: u32, amount: f32) -> Self {
    Self {
      id,
      r#type: TransactionType::Output(amount),
      status: TransactionStatus::NonExecuted,
    }
  }

  /// Returns either input or output transaction amount.
  fn get_amount(&self) -> f32 {
    match self.r#type {
      TransactionType::Input(amount) => amount,
      TransactionType::Output(amount) => amount,
    }
  }

  /// Executes either deposit or withdraw for `client`.
  ///
  /// The transaction will be mark as executed if the client has enough
  /// funds in withdraw case.
  ///
  /// # Errors
  /// An error is returned if the transaction is already executed or disputed.
  pub fn execute(&mut self, client: &mut Client) -> Result<()> {
    if match self.status {
      TransactionStatus::NonExecuted => Ok(match &self.r#type {
        TransactionType::Input(amount) => client.deposit_funds(*amount),
        TransactionType::Output(amount) => client.withdraw_funds(*amount),
      }),
      // Both dispute and executed status means the transaction has been executed
      _ => Err(Error::AlreadyExecutedTransaction(self.id)),
    }? {
      self.status = TransactionStatus::Executed;
    }

    Ok(())
  }

  pub fn dispute(&mut self, client: &mut Client) -> Result<()> {
    if match self.status {
      TransactionStatus::Executed => Ok(client.block_funds(self.get_amount())),
      _ => Err(Error::DisputeTransactionError(self.id)),
    }? {
      self.status = TransactionStatus::Disputed;
    }

    Ok(())
  }

  pub fn resolve(&mut self, client: &mut Client) -> Result<()> {
    if match self.status {
      TransactionStatus::Disputed => Ok(client.release_funds(self.get_amount())),
      _ => Err(Error::ResolveTransactionError(self.id)),
    }? {
      self.status = TransactionStatus::Executed;
    }

    Ok(())
  }

  pub fn chargeback(&mut self, client: &mut Client) -> Result<()> {
    if match self.status {
      TransactionStatus::Disputed => Ok(client.chargeback_funds(self.get_amount())),
      _ => Err(Error::ChargebackTransactionError(self.id)),
    }? {
      self.status = TransactionStatus::Executed;
    }

    Ok(())
  }
}
