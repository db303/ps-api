use {
    crate::domain::{User, UserEmail, UserName},
    crate::email_client::EmailClient,
    crate::routes::auth::signup::generate_activation_token,
    crate::startup::ApplicationBaseUrl,
    crate::utils::{error_chain_fmt, get_error_response},
    actix_web::{http::StatusCode, web, HttpResponse, ResponseError},
    anyhow::Context,
    chrono::Utc,
    sqlx::{Executor, PgPool, Postgres, Transaction},
    std::collections::HashMap,
    uuid::Uuid,
};

const TEMPLATE_ID: u64 = 3886368;
const PASSWORD_RESET_EMAIL_SUBJECT: &str = "PatternSaver.com Account Password Reset Instructions";

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PasswordResetRequest {
    email: String,
}

#[derive(serde::Serialize)]
pub struct PasswordResetResponse {
    status: String,
    data: serde_json::Value,
}

#[tracing::instrument(
    name = "Send password reset email",
    skip(request, pool, email_client, base_url)
)]
pub async fn request_password_reset(
    request: web::Json<PasswordResetRequest>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    base_url: web::Data<ApplicationBaseUrl>,
) -> Result<HttpResponse, PasswordResetError> {
    let _user = if let Some(user) = get_user_from_email(&pool, &request.0.email)
        .await
        .map_err(PasswordResetError::UnexpectedError)?
    {
        let mut transaction = pool
            .begin()
            .await
            .context("Failed to acquire a Postgres connection from the pool")?;

        delete_password_reset_token(&mut transaction, user.user_id)
            .await
            .context("Failed deleting password reset token")?;

        let reset_token = generate_activation_token();
        store_password_reset_token(&mut transaction, user.user_id, &reset_token)
            .await
            .context("Failed to store password reset token")?;

        transaction
            .commit()
            .await
            .context("Failed to commit SQL transaction to store a password reset token.")?;

        send_password_reset_email(
            &email_client,
            user.username,
            user.email,
            &base_url.0,
            &reset_token,
        )
        .await
        .context("Failed to send a password reset email.")?;
    };

    let response = web::Json(PasswordResetResponse {
        status: "success".to_string(),
        data: serde_json::Value::Null,
    });

    Ok(HttpResponse::Accepted().json(response))
}

#[tracing::instrument(
    name = "Send password reset email",
    skip(email_client, username, email, base_url, reset_token)
)]
pub async fn send_password_reset_email(
    email_client: &EmailClient,
    username: UserName,
    email: UserEmail,
    base_url: &str,
    reset_token: &str,
) -> Result<(), reqwest::Error> {
    let password_reset_link = format!(
        "{}/auth/reset_password?reset_token={}",
        base_url, reset_token
    );

    let mut variables = HashMap::new();
    variables.insert(String::from("password_reset_link"), password_reset_link);
    variables.insert(String::from("username"), username.as_ref().to_string());

    email_client
        .send_email(
            &email,
            PASSWORD_RESET_EMAIL_SUBJECT,
            &TEMPLATE_ID,
            variables,
        )
        .await
}

#[tracing::instrument(name = "Get User from email", skip(email, pool))]
pub async fn get_user_from_email(
    pool: &PgPool,
    email: &str,
) -> Result<Option<User>, anyhow::Error> {
    let row = sqlx::query!(
        r#"SELECT user_id, username, email FROM users WHERE email = $1"#,
        email,
    )
    .fetch_optional(pool)
    .await
    .context("Failed to perform a query to retrieve stored user.")?
    .map(|row| User {
        user_id: row.user_id,
        username: UserName::parse(row.username).unwrap(),
        email: UserEmail::parse(row.email).unwrap(),
    });

    Ok(row)
}

#[tracing::instrument(name = "Delete activation token", skip(user_id, transaction))]
pub async fn delete_password_reset_token(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
) -> Result<(), sqlx::Error> {
    let query = sqlx::query!(
        r#"DELETE FROM password_reset_tokens WHERE user_id = $1"#,
        user_id,
    );
    transaction.execute(query).await?;
    Ok(())
}

#[derive(thiserror::Error)]
pub enum PasswordResetError {
    #[error("Something went wrong.")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for PasswordResetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for PasswordResetError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::UnexpectedError(_) => {
                HttpResponse::build(self.status_code()).json(get_error_response(self.to_string()))
            }
        }
    }
    fn status_code(&self) -> StatusCode {
        match self {
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[tracing::instrument(
    name = "Store password reset token in the database",
    skip(reset_token, transaction)
)]
pub async fn store_password_reset_token(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    reset_token: &str,
) -> Result<(), StorePasswordResetTokenError> {
    let query = sqlx::query!(
        r#"
    INSERT INTO password_reset_tokens (reset_token, user_id, created_at)
    VALUES ($1, $2, $3)
        "#,
        reset_token,
        user_id,
        Utc::now()
    );
    transaction.execute(query).await.map_err(StorePasswordResetTokenError)?;
    Ok(())
}

pub struct StorePasswordResetTokenError(sqlx::Error);

impl std::error::Error for StorePasswordResetTokenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl std::fmt::Debug for StorePasswordResetTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl std::fmt::Display for StorePasswordResetTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A database error was encountered while trying to store a password reset token."
        )
    }
}
