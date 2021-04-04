// fate [--db-file]
// fate decide
// fate decisions create
// fate decisions list
// fate review [--check]
// fate read

// As a user, I want to be able to make a decision, explain my reasons for it and how I made the decision, make predictions about how it will go, and set a time to revisit the decision

// As a user, I want to be able to review a decision (before and after reading what I have read in previous reviews);  The flow is read original decision (and why, etc.), then talk about what has actually happened (folllow-through, results), then read previous reviews, then give any additional feedback regarding them.

// TODO
// [ ] write review to db
// [ ] use vi for long-form answers

use crate::decide::decide;
use crate::opt::{Command, InitOpt, ListOpt, Opt, ReadOpt, ReviewOpt};
use log::debug;
use rusqlite::Connection;
use structopt::StructOpt;

mod decide;
mod opt;
mod queries;

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
  println!("Done.")
}

fn init(mut conn: Connection, _init_opt: InitOpt) -> anyhow::Result<()> {
  // TODO: do both of these atomically
  queries::init(&mut conn)?;
  Ok(())
}
fn list(_conn: Connection, _list_opt: ListOpt) -> anyhow::Result<()> {
  unimplemented!()
}
fn read(_conn: Connection, _read_opt: ReadOpt) -> anyhow::Result<()> {
  unimplemented!()
}
fn review(conn: Connection, review_opt: ReviewOpt) -> anyhow::Result<()> {
  let ReviewOpt { check } = review_opt;
  let needs_review = queries::get_decisions_needing_review(&conn)?;
  if check {
    let num_need_review = needs_review.len();
    if num_need_review == 0 {
      println!("No reviews required");
      std::process::exit(0);
    } else {
      println!("{} reviews required", num_need_review);
      std::process::exit(1);
    }
  }
  println!("needs review {:?}", needs_review);
  // TODO: actually perform the review.
  Ok(())
}
