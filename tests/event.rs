use bytes::Bytes;
use plausible_rs::{Error, EventHeaders, EventPayload, Plausible, PropValue, PAGEVIEW_EVENT};
use std::collections::HashMap;

#[tokio::test]
async fn test() {
    let plausible: Plausible = Plausible::new();

    // collect payload
    let domain: String = String::from("example.com");
    let name: String = PAGEVIEW_EVENT.to_string();
    let url: String = format!("https://{}/test", domain);
    let referrer: String = String::from("https://www.toddgriffin.me/");
    let screen_width: usize = 2560;
    let props: HashMap<String, PropValue> = HashMap::from([(
        String::from("author"),
        PropValue::from(String::from("Todd Everett Griffin")),
    )]);
    let payload: EventPayload = EventPayload::new(
        domain,
        name,
        url,
        Some(referrer),
        Some(screen_width),
        Some(props),
    );

    // collect headers
    let user_agent: String = String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36");
    let x_forwarded_for: String = String::from("127.0.0.1");
    let headers: EventHeaders = EventHeaders::new(user_agent, x_forwarded_for);

    // post Event
    let event_result: Result<Bytes, Error> = plausible.event(headers, payload).await;
    assert!(event_result.is_ok());
}
