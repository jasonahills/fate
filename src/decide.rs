use crate::opt::DecideOpt;
use crate::queries::DecisionCreate;
use chrono::{DateTime, Duration, Utc};
use rusqlite::Connection;
use std::io::stdin;

pub fn decide(mut conn: Connection, _decide_opt: DecideOpt) -> anyhow::Result<()> {
  let s = stdin();
  // TODO: use vim or the like for longer-form decisions.
  println!("Provide a title for your decision:");
  let mut title = String::new();
  s.read_line(&mut title)?;

  println!("What is your decision, in detail?");
  let mut description = String::new();
  s.read_line(&mut description)?;

  println!("Why are you making this decision?");
  let mut reason = String::new();
  s.read_line(&mut reason)?;

  println!("What do you think will happen because of this decision?");
  let mut prediction = String::new();
  s.read_line(&mut prediction)?;

  let review_at = review_at_selection()?;

  // It's a little silly to create it just to pull the pieces out immediately after, but I
  // like the clarity of doing so for now.
  // TODO: see if I can pass structs into rusqlite more directly.
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

fn review_at_selection() -> anyhow::Result<DateTime<Utc>> {
  // TODO: better date picker
  let s = stdin();
  let days = loop {
    println!("In how many days should we review this decision?");
    let mut days = String::new();
    s.read_line(&mut days)?;
    println!("days {:?}", days);
    match days.trim().parse::<i64>() {
      Ok(days) => break days,
      Err(_) => println!("Enter a number, please."),
    }
  };
  Ok(Utc::now() + Duration::days(days))
}
