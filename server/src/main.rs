#![allow(unused)] // for beginning the project. Removed later.
use std::net::SocketAddr;
use axum::{
  extract::{Query, Path}, http::{Method, Uri}, middleware, response::{Html, IntoResponse, Response}, routing::{get, Route, get_service}, Json, Router
};
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use serde::Deserialize;
use uuid::Uuid;

pub use self::error::{Error, Result};
use crate::model::ModelController;

mod ctx;
mod error;
mod log;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()>
{
  let mc = ModelController::new().await?;

  let routes_api = web::routes_tickets::routes(mc.clone())
    .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

  let routes_all = Router::new()
    .merge(routes_hello())
    .merge(web::routes_login::routes())
    .nest("/api", routes_api)
    .layer(middleware::map_response(main_response_mapper))
    .layer(CookieManagerLayer::new())
    .layer(middleware::from_fn_with_state(mc.clone(), web::mw_auth::mw_ctx_resolver))
    .fallback_service(routes_static());

  // startregion: -----Start Sever -----
  let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
  
  print!("->> LITENING on {addr}\n");

  axum::Server::bind(&addr)
    .serve(routes_all.into_make_service())
    .await
    .unwrap();
  // endregion: -----Start Sever -----

  async fn main_response_mapper(
    ctx: Option<Ctx>,
    url: Uri,
    req_method: Method,
    res: Response
  ) -> Response
  {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    // Get the eventual response error
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // If client error, build the new response
    let error_response = client_status_error
      .as_ref()
      .map(|&(ref status_code, ref client_error)| {
        let client_error_body = json!({
          "error": {
            "type": client_error.as_ref(),
            "req_uuid": uuid.to_string(),
          }
        });
        print!("  ->> client_error_body: {client_error_body}");
        // build the new response from the clinet_error_body
        (*status_code, Json(client_error_body)).into_response()
      });

    // build and log the server log line
    let client_error = client_status_error.unzip().1;
    log_request(uuid, req_method, uri, ctx, service_erorr, client_error).await;
    // log_request(uuid, req_method, uri, ctx, service_erorr, client_error).await;

    println!();
    error_response.unwrap_or(res)
  }

  fn routes_static() -> Router
  {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
  }

  // region: ----Routes Hello -----
  fn routes_hello() -> Router
{
    Router::new()
    .route("/hello", get(handler_hello))
    .route("/hello2/:name", get(handler_hello2))
  }

  // startregion -----Handler Hello -----
  // optional param for passing to handler
  #[derive(Debug, Deserialize)]
  struct HelloParams
  {
    name: Option<String>,
  }
  
  // at handler_hello we are demostrating the use of URL query extractors
  async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse
  {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"));
  }

  // handler_hello2 used the value in the path for routing
  async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse
  {
    println!("->> {:<12} - handler_hello - {name:?}", "HANDLER");

    Html(format!("Hello2 <strong>{name}</strong>"));
  }
  // endregion -----Handler Hello -----

  Ok(())

}
