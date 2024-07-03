use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello luowen world!")
}

#[get("/again")]
async fn again() -> impl Responder {
    HttpResponse::Ok().body("Hello2 world again!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting HTTP server: go to http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index)
            .service(again)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

