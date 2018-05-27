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
    type Model;
    fn get_by_id(&self) -> Result<Self::Model, Error>;
    fn get_all(&self) -> Result<Vec<Self::Model>, Error>;
    fn save(&self) -> Result<(), Error>;
    fn update(&self) -> Result<(), Error>;
    fn delete(&self) -> Result<(), Error>; 
}
