pub mod invite;
pub mod salt;
pub mod user;
pub mod workshop;

use failure::{Error, ResultExt};

pub trait Sanitize {
    fn sanitize(&mut self) -> Result<(), Error>;
}

pub trait Validate {
    fn validate(&mut self) -> Result<(), Error>;
}

pub trait Resource {
    fn save(&self) -> Result<(), Error>;
    fn update(&self) -> Result<(), Error>;
    fn delete(&self) -> Result<(), Error>; 
}
