# plausible-rs

[![Version](https://img.shields.io/crates/v/plausible-rs)](https://crates.io/crates/plausible-rs)
[![Docs](https://docs.rs/plausible-rs/badge.svg)](https://docs.rs/plausible-rs)
[![License](https://img.shields.io/crates/l/plausible-rs)](https://crates.io/crates/plausible-rs)

A Rust library for the [Plausible Analytics API](https://plausible.io/docs/events-api).

## Features

- [X] [Events API](https://plausible.io/docs/events-api)
- [ ] [Stats API](https://plausible.io/docs/stats-api) (TODO)
- [ ] [Sites API](https://plausible.io/docs/sites-api) (TODO)

## Examples

### Events API

Record a `pageview` event!

Useful for server-side tracking by sending analytics directly to the Plausible Analytics API.

`PLAUSIBLE_DOMAIN=<domain> cargo run --example event`

```rust
#[tokio::main]
async fn main() {
    let domain: String = env::var("PLAUSIBLE_DOMAIN")
        .expect("set env var `PLAUSIBLE_DOMAIN` to name of site in Plausible");

    Plausible::new().event(
        EventHeaders::new(
            String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36"),
            String::from("127.0.0.1")
        ),
        EventPayload::new(
            domain.clone(),
            PAGEVIEW_EVENT.to_string(),
            format!("https://{}/test", domain),
            Some(String::from("https://www.toddgriffin.me/")),
            Some(2560),
            Some(HashMap::from([(
                String::from("author"),
                PropValue::from(String::from("Todd Everett Griffin")),
            )]))
        )
    ).await.unwrap();
}
```

## Developers

Built with: `Rust 1.63`.

Project is under active maintenance - even if there are no recent commits! Please submit an issue / bug request if you the library needs updating for any reason!

### Feature Requests

#### Implement the rest of the features: Stats API, Sites API

Currently, I only have a use-case for Plausible's server-side analytics tracking via the Events API, so I haven't 
prioritized developing the rest of the endpoints for the Stats API and the Sites API.

I fully intend to implement all of those features so that this library can do everything the Plausible API allows.

If you have a dire need for any of those endpoints, please ping me via an issue on Github and I'll know to prioritize that work.
If you're feeling extra adventurous and/or REALLY need those endpoints implemented, please send a pull request :)

### Commands

- `make lint`
    - Lints the codebase via `cargo fmt`.
- `make test`
    - Tests the codebase via:
        - `cargo fmt`
        - `cargo check`
        - `cargo clippy` (with insanely strict defaults)
        - `cargo test`.

## Credits

Made with 🤬 and 🥲 by [Todd Everett Griffin](https://www.toddgriffin.me/).

`plausible-rs` is open source under the [MIT License](https://github.com/goddtriffin/plausible-rs/blob/master/LICENSE).