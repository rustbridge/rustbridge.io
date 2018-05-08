use chrono::NaiveDateTime;

#[derive(Queryable, Serialize)]
pub struct Workshop {
    pub id: i32,
    pub name: String,
    pub organizer: i32,
    pub description: String,
    pub location: String,
    pub date: NaiveDateTime,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub private: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
