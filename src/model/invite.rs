use chrono::NaiveDateTime;

#[derive(Queryable, Serialize)]
pub struct Invite {
    pub id: i32,
    pub workshop: i32,
    pub email: String,
    pub attending: bool,
    pub pending: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
