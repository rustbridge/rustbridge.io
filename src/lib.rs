#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

pub extern crate comrak;
pub extern crate rocket;

#[macro_use]
pub extern crate rocket_contrib;

#[macro_use]
pub extern crate failure;
pub extern crate failure_derive;

#[macro_use]
pub extern crate serde_derive;

#[macro_use]
extern crate diesel;
extern crate chrono;

extern crate data_encoding;
extern crate ring;

extern crate lettre;
extern crate lettre_email;

pub mod db;
pub mod form;
pub mod model;
pub mod route;
pub mod schema;
pub mod website;

#[cfg(test)]
mod tests {}
