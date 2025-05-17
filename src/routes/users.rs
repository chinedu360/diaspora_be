use actix_web::{ HttpResponse, Responder, HttpRequest};

pub async fn register_user(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}