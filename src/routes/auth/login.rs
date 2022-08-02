use crate::models::auth::AuthUser;
use crate::state::app::AppState;
use actix_web::{web, HttpResponse};

pub async fn handle(state: web::Data<AppState>, user: web::Json<AuthUser>) -> HttpResponse {
    match user.into_inner().authenticate(&state.get_connectinon()) {
        Ok((user, token)) => HttpResponse::Ok().insert_header(("jwt", token)).json(user),
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}
