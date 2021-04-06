use crate::db::{Decision, DecisionCreate, Review, ReviewCreate};
use crate::opt::{Command, DecideOpt, InitOpt, ListOpt, Opt, ReadOpt, ReviewOpt};
use log::debug;
use rusqlite::Connection;
use structopt::StructOpt;

mod db;
mod opt;
mod prompts;

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

fn decide(mut conn: Connection, _decide_opt: DecideOpt) -> anyhow::Result<()> {
  // TODO: use vim or the like for longer-form decisions.
  let title = prompts::prompt_string("Provide a title for your decision:")?;
  let description = prompts::prompt_string("What is your decision, in detail?")?;
  let reason = prompts::prompt_string("Why are you making this decision?")?;
  let prediction =
    prompts::prompt_string("What do you think will happen because of this decision?")?;
  let review_at = prompts::review_at()?;

  let d = DecisionCreate {
    title,
    description,
    reason,
    prediction,
    review_at,
  };

  println!("Inserting decision");
  d.insert(&mut conn)?;
  Ok(())
}

fn init(mut conn: Connection, _init_opt: InitOpt) -> anyhow::Result<()> {
  db::init(&mut conn)?;
  Ok(())
}
fn list(mut conn: Connection, _list_opt: ListOpt) -> anyhow::Result<()> {
  let decisions = Decision::all(&mut conn)?;
  for decision in decisions {
    println!(
      "{}\t{}\t{}",
      decision.created_at.date(),
      decision.id,
      decision.title
    )
  }
  Ok(())
}
fn read(mut conn: Connection, read_opt: ReadOpt) -> anyhow::Result<()> {
  let decision = Decision::get(&mut conn, read_opt.decision_id)?;
  let reviews = Review::get_by_decision(&mut conn, read_opt.decision_id)?;

  println!("{}", decision);
  for review in reviews {
    println!("{}", review);
  }
  Ok(())
}
fn review(mut conn: Connection, review_opt: ReviewOpt) -> anyhow::Result<()> {
  let ReviewOpt { check } = review_opt;
  let needs_review = Decision::get_needing_review(&conn)?;

  let decision = if let Some(decision) = needs_review.first() {
    decision
  } else {
    println!("No reviews required");
    std::process::exit(0);
  };

  if check {
    println!("{} reviews required", needs_review.len());
    std::process::exit(1);
  }

  println!("{}", decision);

  let followthrough = prompts::prompt_string("Describe the followthrough on this decision.")?;
  let reason_reflection =
    prompts::prompt_string("Reflect on the reason you gave for the decision.")?;
  let prediction_reflection = prompts::prompt_string("Reflect on the prediction you made.")?;

  let reviews = Review::get_by_decision(&mut conn, decision.id)?;

  for review in reviews {
    println!("{}", review);
  }

  let additional_notes = prompts::prompt_string(
    "What additional notes would you like to make about this decision and your previous reviews?",
  )?;

  let review_again_at = prompts::review_at()?;

  let review_create = ReviewCreate {
    decision_id: decision.id,
    reason_reflection,
    prediction_reflection,
    followthrough,
    additional_notes,
    review_again_at,
  };

  review_create.insert(&mut conn)?;

  Ok(())
}
