use crate::models::user::NewUser;
use crate::valid::new_user_req::NewUserReq;
use crate::state::app::AppState;
use actix_web::{web, HttpResponse,ResponseError};
use validr::Validation;

pub async fn handle(user: web::Json<NewUserReq>, state: web::Data<AppState>) -> HttpResponse{
    match user.into_inner().validate() {
        Ok(data) => {
            let unwraped_mail = match data.email {
                Some(em) => em,
                None => "".to_string(),
            };

            match NewUser::create(&state.get_connectinon(), &unwraped_mail, &data.pass).await {
                Ok(new_user) => HttpResponse::Ok().json(new_user),
                Err(_) => HttpResponse::Ok().body("error"),
            }
        }
        Err(e) => e.error_response(),
    }
}