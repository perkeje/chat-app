
#[macro_use]
extern crate diesel;
//use actix_cors::Cors;
use actix_web::{
    http,
    web::{self},
    App, HttpResponse, HttpServer,
};

// pub mod jwt;
// pub mod middleware;
pub mod models;
pub mod routes;
pub mod schema;
pub mod state;
pub mod valid;
pub mod services;

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

    //cfg.service(web::resource("/login").route(web::post().to(routes::auth::login::handle)));
    cfg.service(web::resource("/register").route(web::post().to(routes::auth::register::handle)));
    // cfg.service(
    //     web::resource("/tasks")
    //         .route(web::post().to(routes::tasks::new::handle))
    //         .route(web::get().to(routes::tasks::user_tasks::handle))
    //         .route(web::delete().to(routes::tasks::delete_all::handle))
    //         .wrap(crate::middleware::auth::AuthGuard),
    // );
    // cfg.service(
    //     web::resource("/tasks/{id}")
    //         .route(web::get().to(routes::tasks::get_specific::handle))
    //         .route(web::put().to(routes::tasks::update::handle))
    //         .route(web::delete().to(routes::tasks::delete_specific::handle))
    //         .wrap(crate::middleware::auth::AuthGuard),
    // );

    // cfg.service(
    //     web::resource("/tasks/{id}/check")
    //         .route(web::put().to(routes::tasks::check::handle))
    //         .wrap(crate::middleware::auth::AuthGuard),
    // );
}

// fn setup_cors() -> Cors {
//     Cors::default()
//         .send_wildcard()
//         .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
//         .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
//         .allowed_header(http::header::CONTENT_TYPE)
//         .max_age(3600)
// }