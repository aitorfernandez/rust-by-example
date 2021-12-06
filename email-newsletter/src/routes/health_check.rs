use actix_web::HttpResponse;

// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
