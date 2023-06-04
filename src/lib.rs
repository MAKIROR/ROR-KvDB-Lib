pub use store::kv::{DataStore,Value};
pub use client::Client;

pub mod store;
pub mod client;
mod request;
mod error;
mod cmd;