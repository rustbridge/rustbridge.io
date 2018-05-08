use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Workshop { 
  pub id: i32,
  pub name: String,
  pub description: String,
  pub location: String,
  pub date: String,
  pub start_time: String,
  pub end_time: String,
  pub private: bool,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}
