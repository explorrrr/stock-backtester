use actix_web::{get, Error, HttpResponse};

#[get("/health_check")]
async fn health_check() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body(""))
}
