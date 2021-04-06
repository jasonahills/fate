use std::path::PathBuf;
use structopt::StructOpt;
#[structopt(name = "fate", about = "A tool for tracking decisions and results")]
#[derive(Clone, StructOpt, Debug)]
pub struct Opt {
  #[structopt(subcommand)]
  pub command: Command,
  #[structopt(env = "FATE_DB_FILE", short, long)]
  pub db_file: PathBuf,
}
#[derive(Clone, Debug, StructOpt)]
pub enum Command {
  Decide(DecideOpt),
  Init(InitOpt),
  List(ListOpt),
  Read(ReadOpt),
  Review(ReviewOpt),
}
#[derive(Clone, Debug, StructOpt)]
pub struct DecideOpt {}
#[derive(Clone, Debug, StructOpt)]
pub struct InitOpt {}

#[derive(Clone, Debug, StructOpt)]
pub struct ListOpt {}
#[derive(Clone, Debug, StructOpt)]
pub struct ReadOpt {
  #[structopt(name = "DECISION_ID")]
  pub decision_id: crate::queries::DecisionId,
}

#[derive(Clone, Debug, StructOpt)]
pub struct ReviewOpt {
  #[structopt(short, long)]
  pub check: bool,
}
