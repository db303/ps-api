use crate::helpers::spawn_app;
use serde_json::Value;

#[tokio::test]
async fn post_pattern_tb303_returns_401_for_unauthorized_requests() {
    // Arrange
    let app = spawn_app().await;
    let body = get_valid_data();

    // Act
    let response = app.post_patterns_tb303(body.into()).await;

    // Assert
    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn post_pattern_tb303_persists_the_new_pattern() {
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

    let saved = sqlx::query!(
        "SELECT author, title, efx_notes, waveform, cutoff_frequency, resonance, env_mod, decay, accent FROM patterns_tb303")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved pattern");

    assert_eq!(saved.author, Some("Author 1".to_string()));
    assert_eq!(saved.title, Some("My first pattern".to_string()));
    assert_eq!(saved.efx_notes, Some("Some notes".to_string()));
    assert_eq!(saved.waveform, Some("sawtooth".to_string()));
    assert_eq!(saved.cutoff_frequency, Some(10));
    assert_eq!(saved.resonance, Some(20));
    assert_eq!(saved.env_mod, Some(30));
    assert_eq!(saved.decay, Some(40));
    assert_eq!(saved.accent, Some(50));
}

#[tokio::test]
async fn post_pattern_tb303_returns_400_when_required_fields_are_missing() {
    // Arrange
    let app = spawn_app().await;
    let valid_data = get_valid_data();

    let fields_to_remove = vec!["title", "steps"];

    // Act - Part 1 - Login
    app.post_login(
        serde_json::json!({
            "username": &app.test_user.username,
            "password": &app.test_user.password
        })
        .to_string(),
    )
    .await;

    for field in fields_to_remove {
        let mut data: Value = serde_json::from_str(&valid_data).unwrap();
        data.as_object_mut().unwrap().remove(field);

        let body = serde_json::to_string(&data).unwrap();

        // Act - Part 2 - Create pattern
        let response = app.post_patterns_tb303(body.into()).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request when the {} field was missing",
            field
        );
    }
}

#[tokio::test]
async fn post_pattern_tb303_returns_400_when_text_fields_are_present_but_invalid() {
    // Arrange
    let app = spawn_app().await;
    let valid_data = get_valid_data();

    let invalid_values = vec![
        ("author", vec!["", " "]),
        ("title", vec!["", " "]),
        ("efx_notes", vec!["", " "]),
        ("waveform", vec!["", "unknown_waveform", "123"]), // Examples of invalid waveform values
    ];

    // Act - Part 1 - Login
    app.post_login(
        serde_json::json!({
            "username": &app.test_user.username,
            "password": &app.test_user.password
        })
        .to_string(),
    )
    .await;

    for (field, invalid_values) in invalid_values {
        for invalid_value in invalid_values {
            let mut data: Value = serde_json::from_str(&valid_data).unwrap();
            data.as_object_mut()
                .unwrap()
                .insert(field.to_string(), serde_json::json!(invalid_value));

            let body = serde_json::to_string(&data).unwrap();

            // Act - Part 2 - Create pattern
            let response = app.post_patterns_tb303(body.into()).await;

            // Assert
            assert_eq!(
                400,
                response.status().as_u16(),
                "The API did not return a 400 Bad Request when the {} field was invalid: {}",
                field,
                invalid_value
            );
        }
    }
}

#[tokio::test]
async fn post_pattern_tb303_returns_400_when_numeric_fields_are_out_of_bounds() {
    // Arrange
    let app = spawn_app().await;
    let valid_data = get_valid_data();

    let numeric_fields = vec!["cut_off_freq", "resonance", "env_mod", "decay", "accent"];

    let invalid_values = vec![-1, 361]; // Values outside the range 0-360

    // Act - Part 1 - Login
    app.post_login(
        serde_json::json!({
            "username": &app.test_user.username,
            "password": &app.test_user.password
        })
        .to_string(),
    )
    .await;

    for field in numeric_fields {
        for invalid_value in &invalid_values {
            let mut data: Value = serde_json::from_str(&valid_data).unwrap();
            data.as_object_mut()
                .unwrap()
                .insert(field.to_string(), serde_json::json!(invalid_value));

            let body = serde_json::to_string(&data).unwrap();

            // Act - Part 2 - Create pattern
            let response = app.post_patterns_tb303(body.into()).await;

            // Assert
            assert_eq!(
                400,
                response.status().as_u16(),
                "The API did not return a 400 Bad Request when the {} field was out of bounds: {}",
                field,
                invalid_value
            );
        }
    }
}

#[tokio::test]
async fn post_pattern_tb303_returns_400_when_step_field_value_is_invalid() {
    // Arrange
    let app = spawn_app().await;
    let valid_data = get_valid_data();

    let fields_and_invalid_values = vec![
        ("note", vec!["", " ", "invalid_note"]),
        ("stem", vec!["", " ", "invalid_stem"]),
        ("time", vec!["", " ", "invalid_time"]),
        ("slide", vec!["", " ", "invalid_slide"]),
        ("accent", vec!["", " ", "invalid_accent"]),
    ];

    // Act - Part 1 - Login
    app.post_login(
        serde_json::json!({
            "username": &app.test_user.username,
            "password": &app.test_user.password
        })
        .to_string(),
    )
    .await;

    for (field, invalid_values) in fields_and_invalid_values {
        for invalid_value in invalid_values {
            let mut data: Value = serde_json::from_str(&valid_data).unwrap();
            data["steps"][0][field] = serde_json::json!(invalid_value);

            let body = serde_json::to_string(&data).unwrap();

            // Act - Part 2 - Create pattern
            let response = app.post_patterns_tb303(body.into()).await;

            // Assert
            assert_eq!(
                400,
                response.status().as_u16(),
                "The API did not return a 400 Bad Request when the {} field was invalid: {}",
                field,
                invalid_value
            );
        }
    }
}

#[tokio::test]
async fn post_pattern_tb303_returns_400_when_required_step_fields_are_missing() {
    // Arrange
    let app = spawn_app().await;
    let valid_data = get_valid_data();

    let fields_to_remove = vec!["note", "stem", "time", "accent", "slide"];

    // Act - Part 1 - Login
    app.post_login(
        serde_json::json!({
            "username": &app.test_user.username,
            "password": &app.test_user.password
        })
        .to_string(),
    )
    .await;

    for field in fields_to_remove {
        let mut data: Value = serde_json::from_str(&valid_data).unwrap();
        data["steps"][0].as_object_mut().unwrap().remove(field);

        let body = serde_json::to_string(&data).unwrap();

        // Act - Part 2 - Create pattern
        let response = app.post_patterns_tb303(body.into()).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request when the {} field was missing",
            field
        );
    }
}

#[tokio::test]
async fn post_pattern_tb303_returns_400_when_there_is_more_than_16_steps() {
    // Arrange
    let app = spawn_app().await;
    let body = get_valid_data();

    let mut data: Value = serde_json::from_str(&body).unwrap();
    let steps = data["steps"].as_array_mut().unwrap();

    for _ in 0..16 {
        steps.push(steps[0].clone());
    }

    let body = serde_json::to_string(&data).unwrap();

    // Act - Part 1 - Login
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
    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn post_pattern_tb303_fails_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    let body = get_valid_data();

    // Act - Part 1 - Login
    app.post_login(
        serde_json::json!({
            "username": &app.test_user.username,
            "password": &app.test_user.password
        })
        .to_string(),
    )
    .await;

    // Destroy the  database
    sqlx::query!("ALTER TABLE patterns_tb303 DROP COLUMN title;",)
        .execute(&app.db_pool)
        .await
        .unwrap();

    // Act - Part 2 - Create pattern
    let response = app.post_patterns_tb303(body.into()).await;

    // Assert
    assert_eq!(500, response.status().as_u16());
}

fn get_valid_data() -> String {
    r#"{
        "author": "Author 1",
        "title": "My first pattern",
        "efx_notes": "Some notes",
        "waveform": "sawtooth",
        "cut_off_freq": 10,
        "resonance": 20,
        "env_mod": 30,
        "decay": 40,
        "accent": 50,
        "steps": [
            {
                "note": "C",
                "stem": "up",
                "time": "note",
                "accent": true,
                "slide": false
            },
            {
                "note": "D",
                "stem": "down",
                "time": "note",
                "accent": false,
                "slide": true
            },
            {
                "note": "E",
                "stem": "up",
                "time": "note",
                "accent": true,
                "slide": false
            },
            {
                "note": "F",
                "stem": "down",
                "time": "note",
                "accent": false,
                "slide": true
            },
            {
                "note": "G",
                "stem": "up",
                "time": "note",
                "accent": true,
                "slide": false
            },
            {
                "note": "A",
                "stem": "down",
                "time": "note",
                "accent": false,
                "slide": true
            },
            {
                "note": "B",
                "stem": "up",
                "time": "note",
                "accent": true,
                "slide": false
            },
            {
                "note": "Ch",
                "stem": "down",
                "time": "note",
                "accent": false,
                "slide": true
            },
            {
                "note": "C",
                "stem": "up",
                "time": "note",
                "accent": true,
                "slide": false
            },
            {
                "note": "D",
                "stem": "down",
                "time": "note",
                "accent": false,
                "slide": true
            },
            {
                "note": "E",
                "stem": "up",
                "time": "note",
                "accent": true,
                "slide": false
            },
            {
                "note": "F",
                "stem": "down",
                "time": "note",
                "accent": false,
                "slide": true
            },
            {
                "note": "G",
                "stem": "up",
                "time": "note",
                "accent": true,
                "slide": false
            },
            {
                "note": "A",
                "stem": "down",
                "time": "note",
                "accent": false,
                "slide": true
            },
            {
                "note": "B",
                "stem": "up",
                "time": "note",
                "accent": true,
                "slide": false
            },
            {
                "note": "Ch",
                "stem": "down",
                "time": "note",
                "accent": false,
                "slide": true
            }
        ]
    }
    "#
    .to_string()
}
