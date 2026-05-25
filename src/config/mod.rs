//! Runtime configuration helpers — currently just a tiny .env loader.
//! No external crate for this on purpose: this is a teaching app and a
//! 50-line loader is easier to read than another dependency.

mod dotenv;

pub use dotenv::load_dotenv;
