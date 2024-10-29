use crate::helpers::spawn_app;

#[tokio::test]
async fn patterns_returns_a_200_for_valid_303_request_body() {
    // Arrange
    let app = spawn_app().await;
    let body = r#"
        {
            "name": "First Pattern",
            "device": "TB303",
            "data": {
                "length": 2,
                "steps": [
                    {
				        "note": "Cs",
				        "stem": "UP",
				        "accent": true,
				        "slide": true,
				        "time": "REST"
			        },
			        {
				        "note": "C",
				        "stem": "DOWN",
				        "accent": true,
				        "slide": true,
				        "time": "REST"
			        }
                ] 
            }
        }"#;

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
    let response = app.post_patterns(body.into()).await;

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn patterns_persists_the_new_pattern() {
    // Arrange
    let app = spawn_app().await;
    let body = r#"
        {
            "name": "First Pattern",
            "device": "TB303",
            "data": {
                "length": 2,
                "steps": [
                    {
				        "note": "Cs",
				        "stem": "UP",
				        "accent": true,
				        "slide": true,
				        "time": "REST"
			        },
			        {
				        "note": "C",
				        "stem": "DOWN",
				        "accent": true,
				        "slide": true,
				        "time": "REST"
			        }
                ] 
            }
        }"#;

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
    app.post_patterns(body.into()).await;

    // Assert
    let saved = sqlx::query!("SELECT name, device, data FROM patterns",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved pattern.");

    assert_eq!(saved.name, "First Pattern");
    assert_eq!(saved.device, "TB303");
    assert_eq!(saved.data["length"], 2);
    assert_eq!(saved.data["steps"][0]["note"], "Cs");
    assert_eq!(saved.data["steps"][0]["stem"], "UP");
    assert_eq!(saved.data["steps"][0]["accent"], true);
    assert_eq!(saved.data["steps"][0]["slide"], true);
    assert_eq!(saved.data["steps"][0]["time"], "REST");
    assert_eq!(saved.data["steps"][1]["note"], "C");
    assert_eq!(saved.data["steps"][1]["stem"], "DOWN");
    assert_eq!(saved.data["steps"][1]["accent"], true);
    assert_eq!(saved.data["steps"][1]["slide"], true);
    assert_eq!(saved.data["steps"][1]["time"], "REST");
}

#[tokio::test]
async fn patterns_returns_a_200_for_valid_909_request_body() {
    // Arrange
    let app = spawn_app().await;
    let body = r#"
        {
            "name": "First 909 Pattern",
            "device": "TR909",
            "data": {
                "length": 2,
                "steps": [
                    {
                        "accent": true,
                        "bd": true,
                        "sd": true,
                        "lt": true,
                        "mt": true,
                        "ht": true,
                        "rs": false,
                        "cp": false,
                        "oh": true,
                        "ch": true,
                        "cr": true,
                        "ri": true
                    },
                    {
                        "accent": true,
                        "bd": true,
                        "sd": true,
                        "lt": true,
                        "mt": true,
                        "ht": true,
                        "rs": false,
                        "cp": false,
                        "oh": false,
                        "ch": true,
                        "cr": true,
                        "ri": true
                    }
                ]
            }
        }"#;

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
    let response = app.post_patterns(body.into()).await;

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn patterns_returns_a_400_when_device_wont_match_json() {
    // Arrange
    let app = spawn_app().await;
    let body = r#"
        {
            "name": "First Pattern",
            "device": "TR909",
            "data": {
                "length": 2,
                "steps": [
                    {
				        "note": "Cs",
				        "stem": "UP",
				        "accent": true,
				        "slide": true,
				        "time": "REST"
			        },
			        {
				        "note": "C",
				        "stem": "DOWN",
				        "accent": true,
				        "slide": true,
				        "time": "REST"
			        }
                ] 
            }
        }"#;

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
    let response = app.post_patterns(body.into()).await;

    // Assert
    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn patterns_returns_a_400_for_invalid_request_body() {
    // Arrange
    let app = spawn_app().await;
    let body = r#"
        {
            "name": "First 909 Pattern",
            "device": "TR909",
            "data": {
                "length": 16,
                "bd": ["something wrong"]
            }
        }"#;

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
    let response = app.post_patterns(body.into()).await;

    // Assert
    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn patterns_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;

    let no_name = r#"
        {
            "device": "TB303",
            "data": {
                "length": 16,
                "notes": ["C","C#","C","B","C","C#","C","B","C","C#","C","B","C","C#","C","B"]
            }
        }"#;

    let no_device = r#"
        {
            "name": "First Pattern",
            "data": {
                "length": 16,
                "notes": ["C","C#","C","B","C","C#","C","B","C","C#","C","B","C","C#","C","B"]
            }
        }"#;

    let no_data = r#"
        {
            "name": "First Pattern",
            "device": "303",
        }"#;

    let test_cases = vec![
        (no_name, "missing the name"),
        (no_device, "missing the device"),
        (no_data, "missing the data"),
        ("", "missing all values"),
    ];
    for (invalid_body, error_message) in test_cases {
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
        let response = app.post_patterns(invalid_body.into()).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[tokio::test]
async fn patterns_returns_a_400_when_fields_are_present_but_invalid() {
    // Arrange
    let app = spawn_app().await;

    let empty_name = r#"{
	    "name": "",
	    "device": "TB303",
	    "data": {
			"length": 1,
			"steps": [
			    {
                    "note": "C",
                    "stem": "up",
                    "accent": true,
                    "slide": true,
                    "time": 1
                }
			] 
	    }
    }"#;

    let empty_device = r#"{
	    "name": "demo",
	    "device": "",
	    "data": {
			"length": 1,
			"steps": [
			    {
                    "note": "C",
                    "stem": "up",
                    "accent": true,
                    "slide": true,
                    "time": 1
                }
			] 
	    }
    }"#;

    let invalid_device = r#"{
        "name": "demo",
        "device": "broken tb303",
        "data": {
    		"length": 1,
    		"steps": [
    		    {
                    "note": "C",
                    "stem": "up",
                    "accent": true,
                    "slide": true,
                    "time": 1
                }
    		]
        }
    }"#;

    let test_cases = vec![
        (empty_name, "empty name"),
        (empty_device, "empty device"),
        (invalid_device, "invalid device"),
    ];

    for (body, description) in test_cases {
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
        let response = app.post_patterns(body.into()).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request when the payload was {}.",
            description
        );
    }
}

#[tokio::test]
async fn patterns_fails_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    let body = r#"
        {
            "name": "First Pattern",
            "device": "TB303",
            "data": {
                "length": 2,
                "steps": [
                    {
				        "note": "Cs",
				        "stem": "UP",
				        "accent": true,
				        "slide": true,
				        "time": "REST"
			        },
			        {
				        "note": "C",
				        "stem": "DOWN",
				        "accent": true,
				        "slide": true,
				        "time": "REST"
			        }
                ] 
            }
        }"#;

    sqlx::query!("ALTER TABLE patterns DROP COLUMN data;",)
        .execute(&app.db_pool)
        .await
        .unwrap();

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
    let response = app.post_patterns(body.into()).await;

    // Assert
    assert_eq!(500, response.status().as_u16());
}
