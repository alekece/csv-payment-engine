use crate::{types::*, Error, Result};
use csv::{Reader, Writer};
use std::{collections::HashMap, convert::TryFrom, io::Read};

/// A simple payment engine for CSV.
///
/// This engine can compute synchronously multiple
/// CSV both from file or raw data.
#[derive(Default)]
pub struct PaymentEngine {
  clients: HashMap<u16, Client>,
  transactions: HashMap<u32, Transaction>,
}

impl PaymentEngine {
  /// Computes data from CVS `reader`.
  ///
  /// # Errors
  /// The computation fails if :
  /// - CSV entry is ill-formed
  /// - Transaction is duplicated
  /// - Any logical error during the dispute mechanism
  pub fn compute<T: Read>(&mut self, reader: &mut Reader<T>) -> Result<()> {
    for result in reader.deserialize() {
      let TransactionView {
        operation_type,
        transaction_id,
        client_id,
        amount,
      } = result?;

      let mut client = self
        .clients
        .entry(client_id)
        .or_insert_with(|| Client::new(client_id));

      let transaction = self.transactions.get_mut(&transaction_id);

      match Operation::try_from((operation_type.clone(), amount))? {
        // handles duplicated transaction
        Operation::Deposit(..) | Operation::Withdraw(..) if transaction.is_some() => {
          Err(Error::DuplicatedTransaction(transaction_id))
        }
        Operation::Deposit(amount) => {
          let mut transaction = Transaction::new_input(transaction_id, amount);

          transaction.execute(&mut client)?;
          self.transactions.insert(transaction_id, transaction);

          Ok(())
        }
        Operation::Withdraw(amount) => {
          let mut transaction = Transaction::new_output(transaction_id, amount);

          transaction.execute(&mut client)?;
          self.transactions.insert(transaction_id, transaction);

          Ok(())
        }
        // skips either dispute, resolve and chargeback operation when
        // transaction is not found.
        Operation::Dispute | Operation::Resolve | Operation::Chargeback
          if transaction.is_none() =>
        {
          Ok(())
        }
        Operation::Dispute => transaction.unwrap().dispute(&mut client),
        Operation::Resolve => transaction.unwrap().resolve(&mut client),
        Operation::Chargeback => transaction.unwrap().chargeback(&mut client),
      }?;
    }
    Ok(())
  }

  pub fn dump<T: std::io::Write>(&self, writer: &mut Writer<T>) -> Result<()> {
    for client in self
      .clients
      .iter()
      .map(|(_, client)| ClientView::from(client.clone()))
    {
      writer.serialize(client)?;
    }

    writer.flush()?;

    Ok(())
  }
}
