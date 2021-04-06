use chrono::{DateTime, Duration, Utc};
use std::io::stdin;

pub fn review_at() -> anyhow::Result<DateTime<Utc>> {
  // TODO: better date picker
  let s = stdin();
  let days = loop {
    println!("In how many days should we review this decision?");
    let mut days = String::new();
    s.read_line(&mut days)?;
    match days.trim().parse::<i64>() {
      Ok(days) => break days,
      Err(_) => println!("Enter a number, please."),
    }
  };
  Ok(Utc::now() + Duration::days(days))
}

pub fn prompt_string(prompt: &str) -> anyhow::Result<String> {
  // TODO check if there any disadvantage of repeatedly getting stdin handles.
  let s = stdin();
  println!("{}", prompt);
  let mut to_return = String::new();
  s.read_line(&mut to_return)?;
  Ok(to_return.trim().to_string())
}
