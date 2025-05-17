use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use std::net::TcpListener;
use crate::routes::{create_item, health_check, register_user};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey, there!")
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    println!("{:?}", listener);
    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/health_check", web::get().to(health_check))
            .route("/users/register", web::post().to(register_user))
            .route("/items", web::post().to(create_item))
    })
    .listen(listener)?
    .run();
    Ok(server)
}