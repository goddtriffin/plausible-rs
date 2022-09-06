use crate::{Error, Plausible};
use bytes::Bytes;
use reqwest::{RequestBuilder, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const PAGEVIEW_EVENT: &str = "pageview";

impl Plausible {
    /// Records a pageview or custom event.
    ///
    /// When using this endpoint, it's crucial to send the HTTP headers correctly,
    /// since these are used for unique user counting.
    ///
    /// # Errors
    ///
    /// Will return `Err` if an error occurred while creating/sending the request,
    /// if it failed to decode the response's bytes, or if the response's status code was not a
    /// success.
    pub async fn event(
        &self,
        headers: EventHeaders,
        payload: EventPayload,
    ) -> Result<Bytes, Error> {
        // create request
        let request: RequestBuilder = self
            .client
            .post(format!("{}/api/event", self.base_url))
            .header("Content-Type", "application/json")
            .header("User-Agent", headers.user_agent)
            .header("X-Forwarded-For", headers.x_forwarded_for);

        // send request, get response
        let response = request.json(&payload).send().await?;

        // parse status code and returned bytes
        let status_code: StatusCode = response.status();
        let bytes = response.bytes().await?;

        // check if failure
        if !status_code.is_success() {
            return Err(Error::RequestFailed { bytes, status_code });
        }

        // success
        Ok(bytes)
    }
}

/// Request headers for the 'POST /api/event' API.
#[derive(Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct EventHeaders {
    /// The raw value of User-Agent is used to calculate the user_id which identifies a
    /// [unique visitor](https://plausible.io/data-policy#how-we-count-unique-users-without-cookies)
    /// in Plausible.
    ///
    /// User-Agent is also used to populate the Devices report in your Plausible dashboard.
    /// The device data is derived from the open source database
    /// [device-detector](https://github.com/matomo-org/device-detector).
    /// If your User-Agent is not showing up in your dashboard, it's probably because it is not
    /// recognized as one in the device-detector database.
    pub user_agent: String,

    /// Used to get the IP address of the client.
    ///
    /// The IP address is used to calculate the user_id which identifies a
    /// [unique visitor](https://plausible.io/data-policy#how-we-count-unique-users-without-cookies)
    /// in Plausible. The raw value is anonymized and not stored.
    /// If the header contains a comma-separated list (as it should if the request is sent through
    /// a chain of proxies), then the first valid IP address from the list is used.
    ///
    /// More information can be found on
    /// [MDN docs](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Forwarded-For).
    pub x_forwarded_for: String,
}

impl EventHeaders {
    #[must_use]
    pub const fn new(user_agent: String, x_forwarded_for: String) -> Self {
        Self {
            user_agent,
            x_forwarded_for,
        }
    }
}

/// Request body parameters for the 'POST /api/event' API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::module_name_repetitions)]
pub struct EventPayload {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer: Option<String>,

    /// Width of the screen.
    ///
    /// When using the script, this is set to `window.innerWidth`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_width: Option<usize>,

    /// Custom properties for the event.
    ///
    /// See: <https://plausible.io/docs/custom-event-goals#using-custom-props>
    ///
    /// Custom properties only accepts scalar values such as strings, numbers and booleans.
    /// Data structures such as objects, arrays etc. aren't accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<HashMap<String, PropValue>>,
}

impl EventPayload {
    #[must_use]
    pub const fn new(
        domain: String,
        name: String,
        url: String,
        referrer: Option<String>,
        screen_width: Option<usize>,
        props: Option<HashMap<String, PropValue>>,
    ) -> Self {
        Self {
            domain,
            name,
            url,
            referrer,
            screen_width,
            props,
        }
    }
}

/// Custom properties only accepts scalar values such as strings, numbers and booleans.
/// Data structures such as objects, arrays etc. aren't accepted.
// Implementation on how to constrain types easily from: https://stackoverflow.com/a/52582432/11767294
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropValue {
    // string
    String(String),

    // bool
    Bool(bool),

    // numbers
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),

    F32(f32),
    F64(f64),
}

impl From<String> for PropValue {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<bool> for PropValue {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}

impl From<u8> for PropValue {
    fn from(u: u8) -> Self {
        Self::U8(u)
    }
}

impl From<u16> for PropValue {
    fn from(u: u16) -> Self {
        Self::U16(u)
    }
}

impl From<u32> for PropValue {
    fn from(u: u32) -> Self {
        Self::U32(u)
    }
}

impl From<u64> for PropValue {
    fn from(u: u64) -> Self {
        Self::U64(u)
    }
}

impl From<u128> for PropValue {
    fn from(u: u128) -> Self {
        Self::U128(u)
    }
}

impl From<usize> for PropValue {
    fn from(u: usize) -> Self {
        Self::Usize(u)
    }
}

impl From<i8> for PropValue {
    fn from(i: i8) -> Self {
        Self::I8(i)
    }
}

impl From<i16> for PropValue {
    fn from(i: i16) -> Self {
        Self::I16(i)
    }
}

impl From<i32> for PropValue {
    fn from(i: i32) -> Self {
        Self::I32(i)
    }
}

impl From<i64> for PropValue {
    fn from(i: i64) -> Self {
        Self::I64(i)
    }
}

impl From<i128> for PropValue {
    fn from(i: i128) -> Self {
        Self::I128(i)
    }
}

impl From<isize> for PropValue {
    fn from(i: isize) -> Self {
        Self::Isize(i)
    }
}

impl From<f32> for PropValue {
    fn from(f: f32) -> Self {
        Self::F32(f)
    }
}

impl From<f64> for PropValue {
    fn from(f: f64) -> Self {
        Self::F64(f)
    }
}
