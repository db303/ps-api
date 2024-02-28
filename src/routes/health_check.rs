use actix_web::HttpResponse;

#[utoipa::path(
    get,
    path = "/health_check",
    responses(
        (status = 200, description = "Pet found successfully"),
        (status = NOT_FOUND, description = "Pet was not found")
    ),
)]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
