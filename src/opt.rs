use std::path::PathBuf;
use structopt::StructOpt;
#[structopt(name = "fate", about = "A tool for following up on decisions")]
#[derive(Clone, StructOpt, Debug)]
pub struct Opt {
  #[structopt(subcommand)]
  pub command: Command,
  #[structopt(env = "FATE_DB_FILE", short, long)]
  pub db_file: PathBuf,
}
#[derive(Clone, Debug, StructOpt)]
pub enum Command {
  #[structopt(about = "Make decisions")]
  Decide(DecideOpt),
  #[structopt(about = "Set up a database (this should typically only be run once)")]
  Init(InitOpt),
  // TODO: consider moving into the decision subcommand, or giving this more options around reviews.
  #[structopt(about = "List decisions")]
  List(ListOpt),
  // TODO: allow user to browse decisions if no decision id passed in.
  #[structopt(about = "Read decisions and their reviews")]
  Read(ReadOpt),
  #[structopt(about = "Add reviews to decisions")]
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
  pub decision_id: crate::db::DecisionId,
}

#[derive(Clone, Debug, StructOpt)]
pub struct ReviewOpt {
  #[structopt(short, long)]
  pub check: bool,
}
