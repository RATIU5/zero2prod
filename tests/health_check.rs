

// `tokio::test` is the testing equivalant of `tokio::main`.
// It also spares you from having to sprcify the `#[test]` attribute.
// Run `cargo expand --test health_check` to inspect what code is generated

#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app().await.expect("Failed to spawn app");

    // We need reqwest
    let client = reqwest::Client::new();

    // Act
    let response = client
    .get("http://127.0.0.1:8000/health_check")
    .send()
    .await
    .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> Result<(), std::io::Error> {
    zero2prod::run().await
}
