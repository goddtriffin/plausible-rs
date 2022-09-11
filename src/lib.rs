//! A Rust library for the [Plausible Analytics API](https://plausible.io/docs/events-api).
//!
//! ## Example
//!
//! Record a `pageview` event!
//!
//! `PLAUSIBLE_DOMAIN=<domain> cargo run --example event_inline`
//!
//! ```rust no_run
//! use plausible_rs::{EventHeaders, EventPayload, Plausible, PropValue, PAGEVIEW_EVENT};
//! use std::collections::HashMap;
//! use std::env;
//!
//! #[tokio::main]
//! async fn main() {
//!     let domain: String = env::var("PLAUSIBLE_DOMAIN")
//!         .expect("set env var `PLAUSIBLE_DOMAIN` to name of site in Plausible");
//!
//!     Plausible::new().event(
//!         EventHeaders::new(
//!             String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36"),
//!             String::from("127.0.0.1")
//!         ),
//!         EventPayload::builder(
//!             domain.clone(),
//!             PAGEVIEW_EVENT.to_string(),
//!             format!("https://{}/test", domain)
//!         )
//!         .referrer(String::from("https://www.toddgriffin.me/"))
//!         .screen_width(2560)
//!         .props(HashMap::from([(
//!                 String::from("author"),
//!                 PropValue::from(String::from("Todd Everett Griffin")),
//!         )]))
//!         .build()
//!     ).await.unwrap();
//! }
//! ```
//!
//! For more examples, check out the `examples` directory within the repository.

mod api;
mod error;
mod plausible_analytics;

pub use api::*;
pub use error::*;
pub use plausible_analytics::*;
