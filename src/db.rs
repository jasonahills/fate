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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Decision {
  pub id: u32,
  pub title: String,
  pub description: String,
  pub reason: String,
  pub prediction: String,
  pub review_at: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
}

impl Decision {
  pub fn get(conn: &mut Connection, decision_id: DecisionId) -> Result<Decision> {
    conn.query_row("SELECT id, title, description, reason, prediction, review_at, created_at FROM decisions WHERE id = ?1", params![decision_id], |row| {
      Ok(Decision {
        id: row.get(0)?,
        title: row.get(1)?,
        description: row.get(2)?,
        reason: row.get(3)?,
        prediction: row.get(4)?,
        review_at: row.get(5)?,
        created_at: row.get(6)?,
      })
    })
  }

  pub fn all(conn: &mut Connection) -> Result<Vec<Decision>> {
    let mut stmt = conn.prepare(
      "
      SELECT id, title, description, reason, prediction, review_at, created_at
      FROM decisions
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
        created_at: row.get(6)?,
      })
    })?;

    rows.collect()
  }

  pub fn get_needing_review(conn: &Connection) -> Result<Vec<Decision>> {
    let mut stmt = conn.prepare(
      "
   SELECT id, title, description, reason, prediction, review_at, created_at
   FROM decisions 
   WHERE datetime(review_at) <= datetime(\'now\')
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
        created_at: row.get(6)?,
      })
    })?;

    rows.collect()
  }
}

impl std::fmt::Display for Decision {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "Decision: {}", self.title)?;
    writeln!(f, "\tid: {}", self.id)?;
    writeln!(f, "\tcreated_at: {:?}", self.created_at)?;
    writeln!(f, "\tdescription: {}", self.description)?;
    writeln!(f, "\treason: {}", self.reason)?;
    writeln!(f, "\tprediction: {}", self.prediction)?;
    Ok(())
  }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DecisionUpdate {
  pub id: u32,
  pub review_at: DateTime<Utc>,
}

pub type DecisionId = u32; //TODO newtype

pub struct ReviewCreate {
  pub decision_id: DecisionId,
  pub reason_reflection: String,
  pub prediction_reflection: String,
  pub followthrough: String,
  pub additional_notes: String,
  pub review_again_at: DateTime<Utc>,
}

impl ReviewCreate {
  pub fn insert(self, conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute(
      "UPDATE decisions SET review_at = ?1 WHERE id = ?2",
      params![self.review_again_at, self.decision_id],
    )?;
    tx.execute(
      "
    INSERT INTO 
    reviews(decision_id, reason_reflection, prediction_reflection, followthrough, additional_notes) 
    VALUES (?1, ?2, ?3, ?4, ?5)
    ",
      params![
        self.decision_id,
        self.reason_reflection,
        self.prediction_reflection,
        self.followthrough,
        self.additional_notes,
      ],
    )?;

    tx.commit()
  }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Review {
  pub id: ReviewId,
  pub decision_id: DecisionId,
  pub reason_reflection: String,
  pub prediction_reflection: String,
  pub followthrough: String,
  pub additional_notes: String,
  pub created_at: DateTime<Utc>,
}

impl Review {
  pub fn get_by_decision(conn: &mut Connection, decision_id: DecisionId) -> Result<Vec<Review>> {
    let mut stmt = conn.prepare(
      "
      SELECT id, decision_id, reason_reflection, prediction_reflection, followthrough, additional_notes, created_at
      FROM reviews 
      WHERE decision_id = ?1
  ",
    )?;
    let rows = stmt.query_map(params![decision_id], |row| {
      Ok(Review {
        id: row.get(0)?,
        decision_id: row.get(1)?,
        reason_reflection: row.get(2)?,
        prediction_reflection: row.get(3)?,
        followthrough: row.get(4)?,
        additional_notes: row.get(5)?,
        created_at: row.get(6)?,
      })
    })?;

    rows.collect()
  }
}

impl std::fmt::Display for Review {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "Review:")?;
    writeln!(f, "\tid: {}", self.id)?;
    writeln!(f, "\tcreated_at: {:?}", self.created_at)?;
    writeln!(f, "\tfollowthrough: {}", self.followthrough)?;
    writeln!(f, "\treason_reflection: {}", self.reason_reflection)?;
    writeln!(f, "\tprediction_reflection: {}", self.prediction_reflection)?;
    writeln!(f, "\tadditional_notes: {}", self.additional_notes)?;
    Ok(())
  }
}

// TODO: newtype
pub type ReviewId = u32;

pub fn init(conn: &mut Connection) -> Result<()> {
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

#[cfg(test)]
mod test {
  use chrono::Duration;

  use super::*;

  /// The clock for our tests is different from the clock for our db.  We use db time
  /// so that we don't need to pass in all our timestamps from the application.
  fn db_now(conn: &mut Connection) -> DateTime<Utc> {
    conn
      .query_row("SELECT datetime('now')", rusqlite::NO_PARAMS, |row| {
        row.get(0)
      })
      .unwrap()
  }

  #[test]
  fn db_integration() {
    let db_file = tempfile::NamedTempFile::new().unwrap();
    let mut conn = Connection::open(db_file.path()).unwrap();

    let today = db_now(&mut conn);
    let tomorrow = today + Duration::days(1);

    init(&mut conn).unwrap();

    assert_eq!(Decision::all(&mut conn).unwrap().len(), 0);

    let decision_review_tomorrow = DecisionCreate {
      title: "my title".to_string(),
      description: "my description".to_string(),
      reason: "my reason".to_string(),
      prediction: "my prediction".to_string(),
      review_at: tomorrow,
    };
    decision_review_tomorrow.insert(&mut conn).unwrap();
    assert_eq!(Decision::all(&mut conn).unwrap().len(), 1);
    assert_eq!(Decision::get_needing_review(&mut conn).unwrap().len(), 0);

    let decision_review_today = DecisionCreate {
      title: "my title 2".to_string(),
      description: "my description 2".to_string(),
      reason: "my reason 2".to_string(),
      prediction: "my prediction 2".to_string(),
      review_at: today,
    };
    decision_review_today.insert(&mut conn).unwrap();
    assert_eq!(Decision::all(&mut conn).unwrap().len(), 2);
    let decisions_needing_review = Decision::get_needing_review(&mut conn).unwrap();
    assert_eq!(decisions_needing_review.len(), 1);

    let decision_id_today = decisions_needing_review[0].id;
    let decision_review_today = Decision::get(&mut conn, decision_id_today).unwrap();
    assert_eq!(decisions_needing_review[0], decision_review_today);
    assert_eq!(&decision_review_today.title, "my title 2");
    assert_eq!(&decision_review_today.description, "my description 2");
    assert_eq!(&decision_review_today.reason, "my reason 2");
    assert_eq!(&decision_review_today.prediction, "my prediction 2");
    assert_eq!(decision_review_today.review_at, today);

    assert_eq!(
      Review::get_by_decision(&mut conn, decision_id_today)
        .unwrap()
        .len(),
      0
    );

    let review_create = ReviewCreate {
      decision_id: decision_id_today,
      reason_reflection: "my reason reflection".to_string(),
      prediction_reflection: "my prediction reflection".to_string(),
      followthrough: "my followthrough".to_string(),
      additional_notes: "my additional notes".to_string(),
      review_again_at: tomorrow,
    };
    review_create.insert(&mut conn).unwrap();

    let reviews = Review::get_by_decision(&mut conn, decision_id_today).unwrap();
    assert_eq!(reviews.len(), 1);

    let review = &reviews[0];
    assert_eq!(&review.reason_reflection, "my reason reflection");
    assert_eq!(&review.prediction_reflection, "my prediction reflection");
    assert_eq!(&review.followthrough, "my followthrough");
    assert_eq!(&review.additional_notes, "my additional notes");

    let updated_decision = Decision::get(&mut conn, decision_id_today).unwrap();
    assert_eq!(updated_decision.review_at, tomorrow);
  }
}
