use {
    crate::authentication::UserId,
    crate::domain::{
        Author, EFXNotes, Knob, NewTB303Pattern, NewTB303Step, Note, Number, Stem, Time, Title,
        Waveform,
    },
    crate::utils::error_chain_fmt,
    actix_web::{http::StatusCode, web, HttpResponse, ResponseError},
    anyhow::Context,
    chrono::Utc,
    sqlx::{Executor, PgPool, Postgres, Transaction},
    std::convert::TryInto,
    utoipa::ToSchema,
    uuid::Uuid,
};

#[derive(serde::Deserialize, Debug, ToSchema)]
pub struct PatternTB303Request {
    #[schema(example = "user123")]
    author: Option<String>,
    #[schema(example = "My first pattern", required = true)]
    title: String,
    #[schema(example = "This is a cool pattern")]
    efx_notes: Option<String>,
    #[schema(example = "sawtooth")]
    waveform: Option<String>,
    #[schema(example = "100")]
    cut_off_freq: Option<i32>,
    #[schema(example = "50")]
    resonance: Option<i32>,
    #[schema(example = "25")]
    env_mod: Option<i32>,
    #[schema(example = "50")]
    decay: Option<i32>,
    #[schema(example = "75")]
    accent: Option<i32>,
    steps: Vec<StepTB303>,
}

#[derive(serde::Deserialize, Debug, ToSchema)]
pub struct StepTB303 {
    #[schema(example = "1", required = true)]
    pub number: i32,
    #[schema(example = "C")]
    pub note: Option<String>,
    #[schema(example = "up")]
    pub stem: Option<String>,
    #[schema(example = "note")]
    pub time: String,
    #[schema(example = "true")]
    pub accent: Option<bool>,
    #[schema(example = "false")]
    pub slide: Option<bool>,
}

#[derive(serde::Serialize, ToSchema)]
pub struct PatternTB303Response {
    #[schema(example = "success")]
    status: String,
    data: PatternTB303ResponseData,
}

#[derive(serde::Serialize, ToSchema)]
pub struct PatternTB303ResponseData {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    id: String,
}

#[derive(serde::Serialize)]
pub struct PatternErrorResponse {
    status: String,
    message: String,
}

impl TryInto<NewTB303Pattern> for PatternTB303Request {
    type Error = String;

    fn try_into(self) -> Result<NewTB303Pattern, Self::Error> {
        // Helper function to reduce code repetition
        fn parse_optional<T, U, F>(opt: Option<U>, parse_fn: F) -> Result<Option<T>, String>
        where
            F: FnOnce(U) -> Result<T, String>,
        {
            opt.map(parse_fn).transpose().map_err(|e| e.to_string())
        }

        let author = parse_optional(self.author, Author::parse)?;
        let title = Title::parse(self.title).map_err(|e| e.to_string())?;
        let efx_notes = parse_optional(self.efx_notes, EFXNotes::parse)?;
        let cut_off_freq = parse_optional(self.cut_off_freq, |v| Knob::parse(v))?;
        let resonance = parse_optional(self.resonance, |v| Knob::parse(v))?;
        let env_mod = parse_optional(self.env_mod, |v| Knob::parse(v))?;
        let decay = parse_optional(self.decay, |v| Knob::parse(v))?;
        let accent = parse_optional(self.accent, |v| Knob::parse(v))?;
        let waveform = parse_optional(self.waveform, Waveform::parse)?;

        let steps: Result<Vec<NewTB303Step>, String> = self
            .steps
            .into_iter()
            .map(|step| {
                Ok(NewTB303Step {
                    number: Number::parse(step.number).map_err(|e| e.to_string())?,
                    note: parse_optional(step.note, Note::parse)?,
                    stem: parse_optional(step.stem, Stem::parse)?,
                    time: Time::parse(step.time).map_err(|e| e.to_string())?,
                    accent: step.accent,
                    slide: step.slide,
                })
            })
            .collect();

        let steps = steps?;
        if steps.len() > 16 {
            return Err("A pattern can only have up to 16 steps.".to_string());
        }

        Ok(NewTB303Pattern {
            author,
            title,
            efx_notes,
            waveform,
            cut_off_freq,
            resonance,
            env_mod,
            decay,
            accent,
            steps,
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

#[utoipa::path(
    request_body = PatternTB303Request,
    post,
    path = "/api/v1/patterns/tb303",
    responses(
    (status = 200, description = "Pattern created successfully", body = PatternTB303Response),
    (status = 400, description = "Invalid input"),
    (status = 500, description = "Internal server error")
    ),
)]
#[tracing::instrument(name = "Adding new pattern", skip(pattern, pool))]
pub async fn create_tb303_pattern(
    pattern: web::Json<PatternTB303Request>,
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<web::Json<PatternTB303Response>, CreatePatternError> {
    let user_id = user_id.into_inner();
    let new_pattern = pattern
        .0
        .try_into()
        .map_err(CreatePatternError::ValidationError)?;

    let mut transaction = pool
        .begin()
        .await
        .context("Failed to start a new transaction.")?;

    let pattern_id = insert_pattern(&mut transaction, &new_pattern, &user_id)
        .await
        .context("Failed to insert new pattern in the database.")?;

    insert_steps_tb303(&mut transaction, pattern_id, &new_pattern.steps)
        .await
        .context("Failed to insert new pattern steps in the database.")?;

    transaction
        .commit()
        .await
        .context("Failed to commit the transaction to save tb303 pattern.")?;

    Ok(web::Json(PatternTB303Response {
        status: "success".to_string(),
        data: PatternTB303ResponseData {
            id: pattern_id.to_string(),
        },
    }))
}

#[tracing::instrument(
    name = "Saving new tb303 pattern in the database",
    skip(new_pattern, transaction, user_id)
)]
pub async fn insert_pattern(
    transaction: &mut Transaction<'_, Postgres>,
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
        new_pattern.author.as_ref().map(|a| a.as_ref()),
        new_pattern.title.as_ref(),
        new_pattern.efx_notes.as_ref().map(|e| e.as_ref()),
        new_pattern.waveform.as_ref().map(|w| w.as_ref()),
        new_pattern
            .cut_off_freq
            .as_ref()
            .map(|c| c.as_ref())
            .unwrap_or(&0),
        new_pattern
            .resonance
            .as_ref()
            .map(|r| r.as_ref())
            .unwrap_or(&0),
        new_pattern
            .env_mod
            .as_ref()
            .map(|e| e.as_ref())
            .unwrap_or(&0),
        new_pattern.decay.as_ref().map(|d| d.as_ref()).unwrap_or(&0),
        new_pattern
            .accent
            .as_ref()
            .map(|a| a.as_ref())
            .unwrap_or(&0),
        Utc::now(),
        Utc::now()
    );

    transaction.execute(query).await?;

    Ok(pattern_id)
}

#[tracing::instrument(
    name = "Saving new tb303 pattern steps in the database",
    skip(transaction, pattern_id, steps)
)]
pub async fn insert_steps_tb303(
    transaction: &mut Transaction<'_, Postgres>,
    pattern_id: Uuid,
    steps: &[NewTB303Step],
) -> Result<(), sqlx::Error> {
    for step in steps {
        let step_id = Uuid::new_v4();
        let query = sqlx::query!(
            r#"
            INSERT INTO steps_tb303 (
                step_id,
                pattern_id,
                number,
                note,
                stem,
                time,
                accent,
                slide,
                created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            step_id,
            pattern_id,
            step.number.as_ref(),
            step.note.as_ref().map(|n| n.as_ref()),
            step.stem.as_ref().map(|s| s.as_ref()),
            step.time.as_ref(),
            step.accent.unwrap_or(false),
            step.slide.unwrap_or(false),
            Utc::now()
        );

        transaction.execute(query).await?;
    }

    Ok(())
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
