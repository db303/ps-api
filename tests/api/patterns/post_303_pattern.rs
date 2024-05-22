use crate::helpers::spawn_app;

#[tokio::test]
async fn it_returns_401_for_unauthorized_requests() {
    // Arrange
    let app = spawn_app().await;
    let body = get_valid_data();

    // Act
    let response = app.post_patterns_tb303(body.into()).await;

    // Assert
    assert_eq!(401, response.status().as_u16());
} 

#[tokio::test]
async fn it_returns_200_for_authorized_requests() {
    // Arrange
    let app = spawn_app().await;
    let body = get_valid_data();

    // // Act - Part 1 - Login
    app.post_login(
        serde_json::json!({
            "username": &app.test_user.username,
            "password": &app.test_user.password
        })
        .to_string(),
    )
    .await;

    // Act - Part 2 - Create pattern
    let response = app.post_patterns_tb303(body.into()).await;

    // Assert
    assert_eq!(200, response.status().as_u16());

}

fn get_valid_data() -> String {
    r#"{
        "author": "Pattern 1",
        "title": "My first pattern",
        "efx_notes": "Some notes",
        "waveform": "sawtooth",
        "cut_off_freq": 100,
        "resonance": 100,
        "env_mod": 100,
        "decay": 100,
        "accent": 100
    }
    "#.to_string()
}
