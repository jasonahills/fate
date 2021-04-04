use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DecisionCreate {
  pub title: String,
  pub description: String,
  pub reason: String,
  pub prediction: String,
  pub review_at: DateTime<Utc>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Decision {
  pub id: u32,
  pub title: String,
  pub description: String,
  pub reason: String,
  pub prediction: String,
  pub review_at: DateTime<Utc>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DecisionUpdate {
  pub id: u32,
  pub review_at: DateTime<Utc>,
}

impl DecisionCreate {
  pub fn insert(&self, conn: &mut Connection) -> Result<()> {
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

pub fn get_decisions_needing_review(conn: &Connection) -> Result<Vec<Decision>> {
  let mut stmt = conn.prepare(
    "
 SELECT id, title, description, reason, prediction, review_at 
 FROM decisions 
 WHERE review_at < \"now\"
",
  )?;
  let rows = stmt.query_map(rusqlite::NO_PARAMS, |row| {
    Ok(Decision {
      id: row.get(0)?,
      title: row.get(1)?,
      description: row.get(2)?,
      reason: row.get(3)?,
      prediction: row.get(4)?,
      review_at: row.get(5)?,
    })
  })?;

  rows.collect()
}
