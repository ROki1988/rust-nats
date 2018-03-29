#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

pub extern crate native_tls;
extern crate serde;
extern crate serde_json;

pub use client::*;
pub use errors::*;
pub use tls_config::*;

mod client;
mod errors;
mod stream;
mod tls_config;
