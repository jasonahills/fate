// fate [--db-file]
// fate decide
// fate decisions create
// fate decisions list
// fate review [--check]
// fate read

// As a user, I want to be able to make a decision, explain my reasons for it and how I made the decision, make predictions about how it will go, and set a time to revisit the decision

// As a user, I want to be able to review a decision (before and after reading what I have read in previous reviews);  The flow is read original decision (and why, etc.), then talk about what has actually happened (folllow-through, results), then read previous reviews, then give any additional feedback regarding them.

// TODO
// [ ] connect to sqlite database
// [ ] opt
// [ ] write decision to db
// [ ] write review to db
// [ ] use vi for long-form answers

use crate::opt::{Command, DecideOpt, InitOpt, ListOpt, Opt, ReadOpt, ReviewOpt};
use log::debug;
use rusqlite::{params, Connection};
use structopt::StructOpt;

mod opt;

fn main() {
  pretty_env_logger::init();
  let opt = Opt::from_args();
  debug!("opt {:?}", opt);

  let conn = Connection::open(opt.db_file).unwrap();

  match opt.command {
    Command::Decide(o) => decide(conn, o),
    Command::Init(o) => init(conn, o),
    Command::List(o) => list(conn, o),
    Command::Read(o) => read(conn, o),
    Command::Review(o) => review(conn, o),
  }
  .unwrap();
}

fn decide(conn: Connection, decide_opt: DecideOpt) -> anyhow::Result<()> {
  Ok(())
}
fn init(conn: Connection, init_opt: InitOpt) -> anyhow::Result<()> {
  conn.execute(
    "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  data            BLOB
                  )",
    params![],
  )?;
  Ok(())
}
fn list(conn: Connection, list_opt: ListOpt) -> anyhow::Result<()> {
  Ok(())
}
fn read(conn: Connection, read_opt: ReadOpt) -> anyhow::Result<()> {
  Ok(())
}
fn review(conn: Connection, review_opt: ReviewOpt) -> anyhow::Result<()> {
  Ok(())
}
