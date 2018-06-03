#[derive(FromForm, Debug)]
pub struct Invite {
    id: usize,
    email: String,
}

impl Invite {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn email(&self) -> &str {
        &self.email[..]
    }
}
