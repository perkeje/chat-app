use crate::models::auth::user::User;
use crate::services::jwt::verify;
use crate::state::app::AppState;
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn handle(state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let token = match req.match_info().get("token") {
        Some(id) => id,
        None => return HttpResponse::BadRequest().finish(),
    };

    let id = match verify(token.to_string()) {
        Ok(user) => user.id,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match User::confirm_registration(id, &state.get_connectinon()) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}
