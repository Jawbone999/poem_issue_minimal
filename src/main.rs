use std::{net::Ipv4Addr, sync::Arc};

use poem::{listener::TcpListener, Server, Route, EndpointExt, IntoEndpoint, http::Method, Endpoint};
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
    // Solution from https://github.com/poem-web/poem/issues/613#issuecomment-1655066001
    // Thanks!
    let api_service = OpenApiService::new(Api, "Example API", "1.0");
    let ui = Arc::new(api_service.swagger_ui().map_to_response());
    let api_service_endpoint = Arc::new(api_service.into_endpoint().map_to_response());

    let handler = poem::endpoint::make(move |req| {
        let ui = ui.clone();
        let api_service_endpoint = api_service_endpoint.clone();
        async move {
            if req.method() == Method::GET && req.uri().path() == "/" {
                ui.call(req).await
            } else {
                api_service_endpoint.call(req).await
            }
        }
    });
    
    let app = Route::new()
        .nest("/", handler);

    Server::new(TcpListener::bind((Ipv4Addr::LOCALHOST, 3000)))
        .run(app)
        .await
}