//! A Rust library for the [Plausible Analytics API](https://plausible.io/docs/events-api).
//!
//! ## Example
//!
//! Record a `pageview` event!
//!
//! `PLAUSIBLE_DOMAIN=<domain> cargo run --example event_inline`
//!
//! ```rust
//! use plausible_rs::{EventHeaders, EventPayload, Plausible, PropValue, PAGEVIEW_EVENT};
//! use std::collections::HashMap;
//! use std::env;
//!
//! #[tokio::main]
//! async fn main() {
//!     let domain: String = String::from("example.com");
//!
//!     Plausible::new().event(
//!         EventHeaders::new(
//!             String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36"),
//!             String::from("127.0.0.1")
//!         ),
//!         EventPayload::new(
//!             domain.clone(),
//!             PAGEVIEW_EVENT.to_string(),
//!             format!("https://{}/test", domain),
//!             Some(String::from("https://www.toddgriffin.me/")),
//!             Some(2560),
//!             Some(HashMap::from([(
//!                 String::from("author"),
//!                 PropValue::from(String::from("Todd Everett Griffin")),
//!             )]))
//!         )
//!     ).await.unwrap();
//! }
//! ```
//!
//! For more examples, check out the `examples` directory within the repository.

mod endpoints;
mod error;
mod plausible_analytics;

pub use endpoints::*;
pub use error::*;
pub use plausible_analytics::*;
