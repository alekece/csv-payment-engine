use csv::{ReaderBuilder, Writer};
use csv_payment_engine::PaymentEngine;
use std::{io::stdout, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
  path: PathBuf,
  #[structopt(short, long, default_value = "4096")]
  buffer_capacity: usize,
}

fn main() -> anyhow::Result<()> {
  let opt = Opt::from_args();
  let mut engine = PaymentEngine::default();

  {
    let mut reader = ReaderBuilder::new()
      .has_headers(true)
      .buffer_capacity(opt.buffer_capacity)
      .from_path(opt.path)?;

    engine.compute(&mut reader)?;
  }

  {
    let mut writer = Writer::from_writer(stdout());

    engine.dump(&mut writer)?;
  }

  Ok(())
}
