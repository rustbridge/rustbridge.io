use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Salt {
    pub id: i32,
    pub salt: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
