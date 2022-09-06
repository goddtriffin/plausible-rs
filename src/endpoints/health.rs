use crate::{Error, Plausible};
use reqwest::{RequestBuilder, StatusCode};
use serde::{Deserialize, Serialize};

impl Plausible {
    /// Monitor the status of the Plausible Analytics API.
    ///
    /// # Errors
    ///
    /// Will return `Err` if an error occurred while creating/sending the request,
    /// if it failed to decode the response's bytes, if the response's status code was not a
    /// success, or if it failed to serialize the response bytes into `HealthResponse`.
    pub async fn health(&self) -> Result<HealthResponse, Error> {
        // create request
        let request: RequestBuilder = self.client.get(format!("{}/api/health", self.base_url));

        // send request, get response
        let response = request.send().await?;

        // parse status code and returned bytes
        let status_code: StatusCode = response.status();
        let bytes = response.bytes().await?;

        // check if failure
        if !status_code.is_success() {
            return Err(Error::RequestFailed { bytes, status_code });
        }

        // success
        let response: HealthResponse = serde_json::from_slice(&bytes)?;
        Ok(response)
    }
}

/// Response returned from 'GET /api/health' API.
///
/// If the individual status is healthy, expect the string value to equal `"ok"`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::module_name_repetitions)]
pub struct HealthResponse {
    pub clickhouse: String,
    pub postgres: String,
}
