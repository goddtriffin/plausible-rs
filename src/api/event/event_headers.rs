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
