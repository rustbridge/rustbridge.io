use std;
use rocket;

pub enum Error {
    Io(std::io::Error),
    Status(rocket::http::Status),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<rocket::http::Status> for Error {
    fn from(err: rocket::http::Status) -> Error {
        Error::Status(err)
    }
}


