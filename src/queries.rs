use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result};
pub struct DecisionCreate {
  pub title: String,
  pub description: String,
  pub reason: String,
  pub prediction: String,
  pub review_at: DateTime<Utc>,
}

// TODO: actual type
pub type Decision = ();

impl DecisionCreate {
  pub fn insert(&self, conn: &mut Connection) -> Result<Decision> {
    conn.execute(
      "
  INSERT INTO 
  decisions(title, description, reason, prediction, review_at) 
  VALUES (?1, ?2, ?3, ?4, ?5)
  ",
      params![
        self.title,
        self.description,
        self.reason,
        self.prediction,
        self.review_at
      ],
    )?;
    Ok(())
  }
}

pub fn init(conn: &mut Connection) -> Result<()> {
  // conn.execute(
  //   "CREATE TABLE stuff (
  //     id INTEGER PRIMARY KEY,
  //     title TEXT NOT NULL
  //   )",
  //   params![],
  // )?;

  let tx = conn.transaction()?;
  tx.execute(
    "
    CREATE TABLE decisions (
      id              INTEGER PRIMARY KEY,
      created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      title           TEXT NOT NULL,
      description     TEXT NOT NULL,
      reason          TEXT NOT NULL,
      prediction      TEXT NOT NULL,
      review_at       TIMESTAMP
    )
  ",
    params![],
  )?;
  tx.execute(
    "
  CREATE TABLE reviews (
    id                       INTEGER PRIMARY KEY,
    created_at               TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    decision_id              INTEGER NOT NULL,
    reason_reflection        TEXT NOT NULL,
    prediction_reflection    TEXT NOT NULL,
    followthrough            TEXT NOT NULL,
    additional_notes         TEXT NOT NULL,
    FOREIGN KEY(decision_id) REFERENCES decisions(id)
  )
  ",
    params![],
  )?;
  tx.commit()?;
  Ok(())
}
