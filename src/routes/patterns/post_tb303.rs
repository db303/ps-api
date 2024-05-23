use {
    crate::authentication::UserId,
    crate::domain::{
        Author, EFXNotes, Knob, NewTB303Pattern, NewTB303Step, Note, Stem, Time, Title, Waveform,
    },
    crate::utils::error_chain_fmt,
    actix_web::{http::StatusCode, web, HttpResponse, ResponseError},
    anyhow::Context,
    chrono::Utc,
    sqlx::{Executor, PgPool, Postgres, Transaction},
    std::convert::TryInto,
    uuid::Uuid,
};

#[derive(serde::Deserialize, Debug)]
pub struct PatternTB303Request {
    author: Option<String>,
    title: String,
    efx_notes: Option<String>,
    waveform: Option<String>,
    cut_off_freq: Option<i32>,
    resonance: Option<i32>,
    env_mod: Option<i32>,
    decay: Option<i32>,
    accent: Option<i32>,
    steps: Vec<StepTB303>,
}

#[derive(serde::Deserialize, Debug)]
pub struct StepTB303 {
    pub note: String,
    pub stem: String,
    pub time: String,
    pub accent: bool,
    pub slide: bool,
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

impl TryInto<NewTB303Pattern> for PatternTB303Request {
    type Error = String;

    fn try_into(self) -> Result<NewTB303Pattern, Self::Error> {
        let author = self.author.map(Author::parse).transpose()?;
        let title = Title::parse(self.title).map_err(|e| e.to_string())?;
        let efx_notes = self.efx_notes.map(EFXNotes::parse).transpose()?;
        let cut_off_freq = self.cut_off_freq.map(Knob::parse).transpose()?;
        let resonance = self.resonance.map(Knob::parse).transpose()?;
        let env_mod = self.env_mod.map(Knob::parse).transpose()?;
        let decay = self.decay.map(Knob::parse).transpose()?;
        let accent = self.accent.map(Knob::parse).transpose()?;
        let waveform = self.waveform.map(Waveform::parse).transpose()?;

        let steps = self
            .steps
            .iter()
            .map(|step| {
                let note = Note::parse(step.note.clone()).map_err(|e| e.to_string())?;
                let stem = Stem::parse(step.stem.clone()).map_err(|e| e.to_string())?;
                let time = Time::parse(step.time.clone()).map_err(|e| e.to_string())?;
                let accent = step.accent;
                let slide = step.slide;

                Ok(NewTB303Step {
                    note,
                    stem,
                    time,
                    accent,
                    slide,
                })
            })
            .collect::<Result<Vec<NewTB303Step>, String>>()?;

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

#[tracing::instrument(name = "Adding new pattern", skip(pattern, pool))]
pub async fn create_tb303_pattern(
    pattern: web::Json<PatternTB303Request>,
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<web::Json<PatternResponse>, CreatePatternError> {
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

    transaction
        .commit()
        .await
        .context("Failed to commit the transaction to save tb303 pattern.")?;

    Ok(web::Json(PatternResponse {
        status: "success".to_string(),
        data: PatternResponseData { id: pattern_id },
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
