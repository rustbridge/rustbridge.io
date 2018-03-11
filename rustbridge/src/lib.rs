#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

pub extern crate rocket;
pub extern crate rocket_contrib;
pub extern crate comrak;

pub extern crate failure;
pub extern crate failure_derive;
pub extern crate display_derive;

#[macro_use] pub extern crate serde_derive;

pub mod website;
pub mod route;

#[cfg(test)]
mod tests {
}
