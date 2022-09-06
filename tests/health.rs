use plausible_rs::{Error, HealthResponse, Plausible};

#[tokio::test]
async fn test() {
    let plausible: Plausible = Plausible::new();

    // get API health
    let health_result: Result<HealthResponse, Error> = plausible.health().await;
    assert!(health_result.is_ok());
}
