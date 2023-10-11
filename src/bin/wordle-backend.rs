// mod server;
use actix_web::web;
use wordle_solver::server;
use wordle_solver::server::api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    return actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .app_data(web::Data::new(server::Controller::new()))
            .configure(api::wordle_config)
    })
    .bind(("localhost", 8080))?
    .run()
    .await;
}
