#[macro_use]
extern crate diesel;
//use actix_cors::Cors;
use actix_web::{
    http,
    web::{self},
    App, HttpResponse, HttpServer,
};

// pub mod middleware;
pub mod models;
pub mod routes;
pub mod schema;
pub mod services;
pub mod state;
pub mod valid;

pub async fn start() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(crate::state::app::initialize_pool()))
            //.wrap(setup_cors())
            .configure(crate::config)
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(HttpResponse::Ok)));

    cfg.service(web::resource("/login").route(web::post().to(routes::auth::login::handle)));
    cfg.service(web::resource("/register").route(web::post().to(routes::auth::register::handle)));
    cfg.service(
        web::resource("/confirm/{token}").route(web::get().to(routes::auth::confirmation::handle)),
    );
}

// fn setup_cors() -> Cors {
//     Cors::default()
//         .send_wildcard()
//         .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
//         .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
//         .allowed_header(http::header::CONTENT_TYPE)
//         .max_age(3600)
// }
