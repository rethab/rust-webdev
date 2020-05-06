use actix_web::{web, App, HttpServer, Responder};

async fn hi() -> impl Responder {
    "hello, world".to_owned()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hi))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
