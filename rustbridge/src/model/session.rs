#[derive(Queryable)]
pub struct Session {
  pub id: i32,
  pub session_key: String,
  pub user_id: i32,
}
