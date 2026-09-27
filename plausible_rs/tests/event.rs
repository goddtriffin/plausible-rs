use bytes::Bytes;
use plausible_rs::{Error, EventHeaders, EventPayload, PAGEVIEW_EVENT, Plausible, PropValue};
use std::collections::HashMap;

#[tokio::test]
async fn test() {
    let plausible: Plausible = Plausible::new();

    // collect payload
    let domain: String = String::from("example.com");
    let payload: EventPayload = EventPayload::builder(
        domain.clone(),
        PAGEVIEW_EVENT.to_string(),
        format!("https://{domain}/test"),
    )
    .referrer(String::from("https://www.toddgriffin.me/"))
    .screen_width(2560)
    .props(HashMap::from([(
        String::from("author"),
        PropValue::from(String::from("Todd Everett Griffin")),
    )]))
    .build();

    // collect headers
    let headers: EventHeaders = EventHeaders::new(
        String::from(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36",
        ),
        String::from("127.0.0.1"),
    );

    // post Event
    let event_result: Result<Bytes, Error> = plausible.event(headers, payload).await;
    assert!(event_result.is_ok());
}
