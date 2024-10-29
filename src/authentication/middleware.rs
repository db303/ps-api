use {
    crate::session_state::TypedSession,
    crate::utils::e500,
    actix_web::{
        body::MessageBody,
        dev::{ServiceRequest, ServiceResponse},
        error::InternalError,
        FromRequest, HttpMessage, HttpResponse,
    },
    actix_web_lab::middleware::Next,
    std::ops::Deref,
    uuid::Uuid,
};

#[derive(Copy, Clone, Debug)]
pub struct UserId(Uuid);

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for UserId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub async fn reject_anonymous_users(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let session = {
        let (http_request, payload) = req.parts_mut();
        TypedSession::from_request(http_request, payload).await
    }?;

    match session.get_user_id().map_err(e500)? {
        Some(user_id) => {
            req.extensions_mut().insert(UserId(user_id));
            next.call(req).await
        }
        None => Err(InternalError::from_response(
            anyhow::anyhow!("The user has not logged in"),
            HttpResponse::Unauthorized().finish(),
        )
        .into()),
    }
}
