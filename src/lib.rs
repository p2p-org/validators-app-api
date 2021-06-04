#[macro_use]
extern crate serde_derive;

pub mod client;
pub mod models;

pub mod prelude {
    pub use super::{
        client::Client,
        models::{Network, ValidatorsOrder},
    };
}
