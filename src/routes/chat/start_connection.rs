use crate::models::chat::lobby::Lobby;
use crate::services::ws::WsConn;
use actix::Addr;
use actix_web::{web::Data, web::Payload, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::str::FromStr;
use uuid::Uuid;

pub async fn handle(req: HttpRequest, stream: Payload, srv: Data<Addr<Lobby>>) -> HttpResponse {
    let group_id: Uuid = match req.match_info().get("group_id") {
        Some(id) => Uuid::from_str(id).unwrap(),
        None => return HttpResponse::BadRequest().finish(),
    };
    let ws = WsConn::new(group_id, srv.get_ref().clone());

    match ws::start(ws, &req, stream) {
        Ok(res) => res,
        Err(e) => HttpResponse::Ok().body(e.to_string()),
    }
}
