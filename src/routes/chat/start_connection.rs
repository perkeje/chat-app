use crate::models::auth::user::User;
use crate::models::chat::lobby::Lobby;
use crate::services::ws::WsConn;
use actix::Addr;
use actix_web::{web::Data, web::Payload, HttpMessage, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::str::FromStr;
use uuid::Uuid;

pub async fn handle(req: HttpRequest, stream: Payload, srv: Data<Addr<Lobby>>) -> HttpResponse {
    let group_id: Uuid = match req.match_info().get("group_id") {
        Some(id) => Uuid::from_str(id).unwrap(),
        None => return HttpResponse::BadRequest().finish(),
    };

    let user = match req.extensions_mut().remove::<User>() {
        Some(u) => u,
        None => return HttpResponse::BadRequest().finish(),
    };

    let id = match Uuid::from_str(&user.id) {
        Ok(uuid) => uuid,
        Err(e) => return HttpResponse::Ok().body(e.to_string()),
    };

    let ws = WsConn::new(group_id, srv.get_ref().clone(), id);

    match ws::start(ws, &req, stream) {
        Ok(res) => res,
        Err(e) => HttpResponse::Ok().body(e.to_string()),
    }
}
