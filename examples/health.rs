use plausible_rs::{HealthResponse, Plausible};

#[tokio::main]
async fn main() {
    let response: HealthResponse = Plausible::new().health().await.unwrap();
    println!("{}", serde_json::to_string(&response).unwrap());
}
