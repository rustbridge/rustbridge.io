#[derive(Queryable)]
pub struct Salt {
    pub id: i32,
    pub salt: String,
}
