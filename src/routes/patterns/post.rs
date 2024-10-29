use {
    crate::authentication::UserId,
    crate::domain::{Device, NewPattern, PatternData, PatternName, PatternRequestData},
    crate::utils::error_chain_fmt,
    actix_web::{http::StatusCode, web, HttpResponse, ResponseError},
    anyhow::Context,
    chrono::Utc,
    sqlx::PgPool,
    std::convert::TryInto,
    uuid::Uuid,
};

#[derive(serde::Deserialize, Debug)]
pub struct PatternRequest {
    name: String,
    device: Device,
    data: PatternRequestData,
}

#[derive(serde::Serialize)]
pub struct PatternResponse {
    status: String,
    data: PatternResponseData,
}

#[derive(serde::Serialize)]
pub struct PatternResponseData {
    id: Uuid,
}

#[derive(serde::Serialize)]
pub struct PatternErrorResponse {
    status: String,
    message: String,
}

impl TryInto<NewPattern> for PatternRequest {
    type Error = String;

    fn try_into(self) -> Result<NewPattern, Self::Error> {
        let name = PatternName::parse(self.name)?;
        let device = self.device;
        let data = PatternData::parse(self.data, &device)?;
        Ok(NewPattern { name, device, data })
    }
}

#[derive(thiserror::Error)]
pub enum CreatePatternError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[tracing::instrument(
    name = "Adding new pattern",
    skip(pattern, pool),
    fields(
        pattern_name = %pattern.name,
        pattern_device = %pattern.device,
    )
)]
pub async fn create_pattern(
    pattern: web::Json<PatternRequest>,
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<web::Json<PatternResponse>, CreatePatternError> {
    // let user_id = user_id.into_inner();

    let new_pattern = pattern
        .0
        .try_into()
        .map_err(CreatePatternError::ValidationError)?;

    let pattern_id = insert_pattern(&pool, &new_pattern, &user_id)
        .await
        .context("Failed to insert new pattern in the database.")?;

    Ok(web::Json(PatternResponse {
        status: "success".to_string(),
        data: PatternResponseData { id: pattern_id },
    }))
}

#[tracing::instrument(
    name = "Saving new pattern details in the database",
    skip(new_pattern, pool)
)]
pub async fn insert_pattern(
    pool: &PgPool,
    new_pattern: &NewPattern,
    user_id: &Uuid,
) -> Result<Uuid, sqlx::Error> {
    let new_pattern_data = serde_json::to_value(&new_pattern.data).unwrap();
    let pattern_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO patterns (id, name, device, data, user_id, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        pattern_id,
        new_pattern.name.as_ref(),
        new_pattern.device.to_string(),
        new_pattern_data,
        user_id,
        Utc::now()
    )
    .execute(pool)
    .await?;
    Ok(pattern_id)
}

impl std::fmt::Debug for CreatePatternError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for CreatePatternError {
    fn error_response(&self) -> HttpResponse {
        let error = PatternErrorResponse {
            status: "error".to_string(),
            message: self.to_string(),
        };

        HttpResponse::build(self.status_code()).json(web::Json(error))
    }

    fn status_code(&self) -> StatusCode {
        match self {
            CreatePatternError::ValidationError(_) => StatusCode::BAD_REQUEST,
            CreatePatternError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
