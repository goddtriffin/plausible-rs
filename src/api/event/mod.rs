mod event_headers;
mod event_payload;
mod event_payload_builder;
mod prop_value;

use crate::{Error, Plausible};
use bytes::Bytes;
pub use event_headers::*;
pub use event_payload::*;
pub use event_payload_builder::*;
pub use prop_value::*;
use reqwest::{RequestBuilder, StatusCode};

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
