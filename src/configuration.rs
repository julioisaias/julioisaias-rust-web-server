use actix_web::web;

#[path="controllers.rs"]
mod ctrl;

pub fn cfg(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::scope("/users")
                .service(ctrl::user::find_all)
                .service(ctrl::user::find_one)
    )
    .service(
        web::scope("/categories")
                .service(ctrl::category::find_all)
                .service(ctrl::category::find_one)
    );
}
