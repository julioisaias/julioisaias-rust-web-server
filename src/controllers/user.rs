use actix_web::{get, web, HttpResponse};

#[get("/show")]
pub async fn find_all() -> HttpResponse {
    HttpResponse::Ok().body("Show users")
}

#[get("/show/{id}")]
pub async fn find_one(path: web::Path<(u32,)>) -> HttpResponse {
    HttpResponse::Ok().body(format!("User detail: {}", path.into_inner().0))
}
