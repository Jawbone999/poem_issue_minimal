use std::net::Ipv4Addr;

use poem::{listener::TcpListener, Server, Route};
use poem_openapi::{OpenApiService, OpenApi, payload::PlainText, Tags};

#[derive(Tags)]
enum Tag {
    Info
}

struct Api;

#[OpenApi]
impl Api {
    #[oai(method = "get", path = "/info", tag = "Tag::Info")]
    async fn index(&self) -> PlainText<String> {
        PlainText("Hello!".into())
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let api_service =
        OpenApiService::new(Api, "Example API", "1.0");
    
    let ui = api_service.swagger_ui();

    let app = Route::new()
        .at("/", ui)
        .nest("/", api_service);

    Server::new(TcpListener::bind((Ipv4Addr::LOCALHOST, 3000)))
        .run(app)
        .await
}