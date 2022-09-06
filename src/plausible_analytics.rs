use reqwest::Client;

pub const BASE_URL: &str = "https://plausible.io";

/// Plausible Analytics client.
#[derive(Debug, Clone)]
pub struct Plausible {
    pub(crate) client: Client,
    pub(crate) base_url: String,
}

impl Plausible {
    /// Create a new Plausible Analytics client with a brand new `reqwest::Client`.
    ///
    /// The `client` is initialized with TLS certs from
    /// [webpki roots](https://github.com/rustls/webpki-roots) by default.
    #[must_use]
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: BASE_URL.to_string(),
        }
    }

    /// Create a new Plausible Analytics client with a given `reqwest::Client`.
    #[must_use]
    pub fn new_with_client(client: Client) -> Self {
        Self {
            client,
            base_url: BASE_URL.to_string(),
        }
    }
}

impl Default for Plausible {
    /// Defaults to `Self::new()`.
    fn default() -> Self {
        Self::new()
    }
}
