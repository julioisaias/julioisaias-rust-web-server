use actix_web::{App, HttpServer};

mod configuration;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .configure(configuration::cfg)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Example http://localhost:8080/users/show/2