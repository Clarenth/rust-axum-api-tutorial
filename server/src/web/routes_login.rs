use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookies, Cookie};

use crate::{Error, Result, web};

#[derive(Debug, Deserialize)]
struct LoginPayload
{
  username: String,
  password: String,
}

pub fn routes() -> Router
{
  Router::new()
  .route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>>
{
  println!("->> {:<12} - api_login", "HANDLER");
  
  // TODO: Implement real login and db auth
  if payload.username != "crag@tarr.gov" || payload.password != "password"
  {
    return  Err(Error::LoginFail);
  }

  // FUTURE FIX: implement real auth token
  cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));
  // status 200 response body
  let body = Json(json!({
    "result":
    {
      "success": true
    }
  }));

  Ok(body)
}