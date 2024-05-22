use {
    crate::authentication::UserId,
    crate::domain::{NewTB303Pattern, Author, EFXNotes, Knob, Title, Waveform},
    crate::utils::error_chain_fmt,
    actix_web::{http::StatusCode, web, HttpResponse, ResponseError},
    anyhow::Context,
    chrono::Utc,
    sqlx::{Executor, PgPool, Postgres, Transaction},
    std::convert::TryInto,
    uuid::Uuid,
};


#[derive(serde::Deserialize, Debug)]
pub struct TB303PatternRequest {
    author: String,
    title: String,
    efx_notes: String,
    waveform: String,
    cut_off_freq: i32,
    resonance: i32,
    env_mod: i32,
    decay: i32,
    accent: i32
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

impl TryInto<NewTB303Pattern> for TB303PatternRequest {
    type Error = String;

    fn try_into(self) -> Result<NewTB303Pattern, Self::Error> {
        let author = Author::parse(self.author)?;
        let title = Title::parse(self.title)?;
        let efx_notes = EFXNotes::parse(self.efx_notes)?;
        let cut_off_freq = Knob::parse(self.cut_off_freq)?;
        let resonance = Knob::parse(self.resonance)?;
        let env_mod = Knob::parse(self.env_mod)?;
        let decay = Knob::parse(self.decay)?;
        let accent = Knob::parse(self.accent)?;
        let waveform = Waveform::parse(self.waveform)?;

        Ok(NewTB303Pattern {
            author,
            title,
            efx_notes,
            waveform,
            cut_off_freq,
            resonance,
            env_mod,
            decay,
            accent
        })
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
        pattern_name = %pattern.author,
        pattern_title = %pattern.title,
    )
)]
pub async fn create_tb303_pattern(
    pattern: web::Json<TB303PatternRequest>,
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<web::Json<PatternResponse>, CreatePatternError> {
    let user_id = user_id.into_inner();

    let new_pattern = pattern.0.try_into().map_err(CreatePatternError::ValidationError)?;

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
    new_pattern: &NewTB303Pattern,
    user_id: &Uuid,
) -> Result<Uuid, sqlx::Error> {

    let pattern_id = Uuid::new_v4();

    let query = sqlx::query!(
        r#"
        INSERT INTO patterns_tb303 (
            pattern_id,
            user_id,
            author,
            title,
            efx_notes,
            waveform,
            cutoff_frequency,
            resonance,
            env_mod,
            decay,
            accent,
            updated_at,
            created_at )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        "#,
        pattern_id,
        user_id,
        new_pattern.author.as_ref(),
        new_pattern.title.as_ref(),
        new_pattern.efx_notes.as_ref(),
        new_pattern.waveform.as_ref(),
        new_pattern.cut_off_freq.as_ref(),
        new_pattern.resonance.as_ref(),
        new_pattern.env_mod.as_ref(),
        new_pattern.decay.as_ref(),
        new_pattern.accent.as_ref(),
        Utc::now(),
        Utc::now()
    );

    query.execute(pool).await?;

    Ok(pattern_id)
}



impl std::fmt::Debug for CreatePatternError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for CreatePatternError {
    fn status_code(&self) -> StatusCode {
        match self {
            CreatePatternError::ValidationError(_) => StatusCode::BAD_REQUEST,
            CreatePatternError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error = PatternErrorResponse {
            status: "error".to_string(),
            message: self.to_string(),
        };

        HttpResponse::build(self.status_code()).json(web::Json(error))
    }
}
