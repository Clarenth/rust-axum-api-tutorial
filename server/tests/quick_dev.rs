#![allow(unused)]

use anyhow::{
  Result, Ok,
};
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> 
{
  let hc = httpc_test::new_client("http://localhost:8080")?;

  hc.do_get("/hello?name=Nick").await?.print().await?;
  hc.do_get("/hello2/Nicholas").await?.print().await?;
  //hc.do_get("/src/main.rs").await?.print().await?;

  let req_login = hc.do_post(
    "/api/login", 
    json!(
      {
        "username": "crag@tarr.gov",
        "password": "password",

      })
    );
    req_login.await?.print().await?;
  hc.do_get("/hello2/Nicholas").await?.print().await?;

  let req_create_ticket = hc.do_post(
    "/api/tickets",
    json!({
      "title": "Ticket AAA",
    }),
  );
  req_create_ticket.await?.print().await?;

  hc.do_get("/api/tickets").await?.print().await?;

  Ok(())
}