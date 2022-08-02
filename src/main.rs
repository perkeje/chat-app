#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    chat_app::start().await
}
