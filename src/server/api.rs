use crate::server::controller;
use crate::server::session;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/wordle/new")]
async fn newgame(ctrl: web::Data<controller::Controller>) -> actix_web::Result<String> {
    println!("/wordle/new");
    let id = ctrl.new_game();
    return Ok(serde_json::to_string(&id).unwrap());
    // return id;
}

#[get("/wordle/{sess_id}/guess/{word}")]
async fn guess(
    ctrl: web::Data<controller::Controller>,
    path: web::Path<(session::SessionId, String)>,
) -> actix_web::Result<String> {
    let (id, word) = path.into_inner();
    let res = ctrl.guess(id, &word).unwrap();
    return Ok(serde_json::to_string(&res).unwrap());
}

pub fn wordle_config(cfg: &mut web::ServiceConfig) {
    cfg.service(newgame).service(guess);
}
