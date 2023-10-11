use actix_web::{get, web, HttpResponse, Responder};

mod controller;
pub mod api;
mod model;
mod session;

pub use controller::Controller;


async fn index() -> impl Responder {
    "Hello world!"
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// #[get("/play")]
// async fn newgame() -> impl Responder{
//     todo!("Right now use only files");
// }

pub fn wordle_config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello)
        .service(web::scope("/wordle").route("/index.html", web::get().to(index)));
}
