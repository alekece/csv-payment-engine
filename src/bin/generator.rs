use csv::Writer;
use csv_payment_engine::types::TransactionView;
use std::io::stdout;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
  size: u32,
}

fn main() -> anyhow::Result<()> {
  let opt = Opt::from_args();

  let mut writer = Writer::from_writer(stdout());

  for transaction in (0..opt.size).into_iter().map(|index| TransactionView {
    operation_type: String::from("deposit"),
    transaction_id: index,
    client_id: 1,
    amount: Some(0.123),
  }) {
    writer.serialize(transaction)?;
  }

  writer.flush()?;

  Ok(())
}
