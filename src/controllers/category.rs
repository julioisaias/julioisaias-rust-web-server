use actix_web::{get, web, HttpResponse};

#[get("/show")]
pub async fn find_all() -> HttpResponse {
    HttpResponse::Ok().body("Show categories")
}

#[get("/show/{id}")]
pub async fn find_one(path: web::Path<(u32,)>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Category detail: {}", path.into_inner().0))
}
