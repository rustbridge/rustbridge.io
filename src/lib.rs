#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

pub extern crate comrak;
pub extern crate rocket;
pub extern crate rocket_contrib;

pub extern crate failure;
pub extern crate failure_derive;

#[macro_use]
pub extern crate serde_derive;

#[macro_use]
extern crate diesel;

extern crate data_encoding;
extern crate ring;

pub mod db;
pub mod model;
pub mod schema;
pub mod website;
pub mod route;
pub mod form;

#[cfg(test)]
mod tests {}
