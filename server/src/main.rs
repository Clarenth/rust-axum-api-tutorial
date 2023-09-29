#![allow(unused)] // for beginning the project. Removed later.

use std::net::SocketAddr;

use axum::{
  response::{Html, IntoResponse},
  routing::get,
  Router, extract::Query,
};
use serde::Deserialize;

#[tokio::main]
async fn main() 
{
  let routes_hello = Router::new().route
  (
    "/hello",
    get(handler_hello)
  );

  // startregion: -----Start Sever -----
  let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
  print!("->> LITENING on {addr}\n");
  axum::Server::bind(&addr).serve(routes_hello.into_make_service()).await.unwrap();
  // endregion: -----Start Sever -----


  // startregion -----Handler Hello -----
  // optional param for passing to handler
  #[derive(Debug, Deserialize)]
  struct HelloParams
  {
    name: Option<String>,
  }

  async fn handler_hello(params: Query<HelloParams>) -> impl IntoResponse
  {
    println!("->> {:<12} - handler_hello", "HANDLER");

    Html("Hello <strong>World!!!</strong>")
  }
  // endregion -----Handler Hello -----

}
