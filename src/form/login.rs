#[derive(FromForm)]
pub struct Login {
    // Validated by browser, non-empty, email format
    email: String,
    // Validated by browser as non-empty
    password: String,
}

impl Login {
    pub fn email(&self) -> &str {
        &self.email[..]
    }

    pub fn password(&self) -> &str {
        &self.password[..]
    }
}
