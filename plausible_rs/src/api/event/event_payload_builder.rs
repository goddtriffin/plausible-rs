use crate::{EventPayload, PropValue};
use std::collections::HashMap;

/// Request body parameters for the 'POST /api/event' API.
///
/// This is a Builder for `EventPayload`.
#[derive(Debug, Clone)]
pub struct EventPayloadBuilder {
    /// Domain name of the site in Plausible.
    ///
    /// This is the domain name you used when you added your site to your Plausible account.
    /// It doesn't need to be an actual domain name, so when adding your mobile app to Plausible,
    /// you could insert the mobile app name in the domain name field
    pub domain: String,

    /// Name of the event.
    ///
    /// Can specify `pageview` which is a special type of event in Plausible.
    /// All other names will be treated as custom events.
    pub name: String,

    /// URL of the page where the event was triggered.
    ///
    /// If the URL contains UTM parameters, they will be extracted and stored.
    /// When using the script, this is set to window.location.href.
    ///
    /// The URL parameter will feel strange in a mobile app but you can manufacture something that
    /// looks like a web URL. If you name your mobile app screens like page URLs, Plausible will
    /// know how to handle it.
    /// So for example, on your login screen you could send something like:
    ///
    /// ```text
    /// event: pageview
    /// url: app://localhost/login
    /// ```
    ///
    /// The pathname (/login) is what will be shown as the page value in the Plausible dashboard.
    pub url: String,

    /// Referrer for this event.
    ///
    /// When using the standard tracker script, this is set to `document.referrer`.
    ///
    /// Referrer values are processed heavily for better usability.
    /// Consider referrer URLS like `m.facebook.com/some-path` and `facebook.com/some-other-path`.
    /// It's intuitive to think of both of these as coming from a single source: Facebook.
    /// In the first example the `referrer` value would be split into `visit:source == Facebook`
    /// and `visit:referrer == m.facebook.com/some-path`.
    ///
    /// Plausible uses the open source [referer-parser](https://github.com/snowplow-referer-parser/referer-parser)
    /// database to parse referrers and assign these source categories.
    ///
    /// When no match has been found, the value of the referrer field will be parsed as an URL.
    /// The hostname will be used as the `visit:source` and the full URL as the `visit:referrer`.
    /// So if you send `https://some.domain.com/example-path`, it will be parsed as follows:
    /// `visit:source == some.domain.com` `visit:referrer == some.domain.com/example-path`.
    pub referrer: Option<String>,

    /// Width of the screen.
    ///
    /// When using the script, this is set to `window.innerWidth`.
    pub screen_width: Option<usize>,

    /// Custom properties for the event.
    ///
    /// See: <https://plausible.io/docs/custom-event-goals#using-custom-props>
    ///
    /// Custom properties only accepts scalar values such as strings, numbers and booleans.
    /// Data structures such as objects, arrays etc. aren't accepted.
    pub props: Option<HashMap<String, PropValue>>,
}

impl EventPayloadBuilder {
    #[must_use]
    pub const fn new(domain: String, name: String, url: String) -> Self {
        Self {
            domain,
            name,
            url,
            referrer: None,
            screen_width: None,
            props: None,
        }
    }

    pub fn referrer(&mut self, referrer: String) -> &mut Self {
        self.referrer = Some(referrer);
        self
    }

    pub fn screen_width(&mut self, screen_width: usize) -> &mut Self {
        self.screen_width = Some(screen_width);
        self
    }

    pub fn props(&mut self, props: HashMap<String, PropValue>) -> &mut Self {
        self.props = Some(props);
        self
    }

    #[must_use]
    pub fn build(&self) -> EventPayload {
        EventPayload::new(
            self.domain.clone(),
            self.name.clone(),
            self.url.clone(),
            self.referrer.clone(),
            self.screen_width,
            self.props.clone(),
        )
    }
}
