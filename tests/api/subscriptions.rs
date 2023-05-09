use crate::helpers::spawn_app;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let body = format!(
        "name={}&email={}",
        urlencoding::encode("le guin"),
        urlencoding::encode("ursula_le_guin@gmail.com")
    );

    // Act
    let response = app.post_subscriptions(body).await;

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin")
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        (
            format!("name={}", urlencoding::encode("le guin")),
            "missing the email",
        ),
        (
            format!("email={}", urlencoding::encode("ursula_le_guin@gmail.com")),
            "missing the name",
        ),
        (String::from(""), "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app.post_subscriptions(invalid_body).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_empty() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        (format!("name=&email={}", "johhny@bingus.com"), "empty name"),
        (format!("name={}&email=", "Johnny Bingus"), "empty email"),
        (
            format!("name=&email={}", "totally-not-an-email"),
            "invalid email",
        ),
    ];

    for (body, description) in test_cases {
        // Act
        let response = app.post_subscriptions(body).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 OK when the payload was {}.",
            description
        );
    }
}
