use std::env;

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {

    let port = env::var("PORT").expect("Missing port number");
    let port = port.parse::<u16>().expect("Invalid port given");
    
    let hc = httpc_test::new_client(format!("http://localhost:{port}"))?;

    hc.do_get("/hello?name=jen").await?.print().await?;
    hc.do_get("/hello2/Malina").await?.print().await?;

    hc.do_post("/api/login", json!({ "username": "MaxFalk", "pwd": "Test123"})).await?.print().await?;

    let req_create_ticket = hc.do_post("/api/tickets", json!({
        "title": "Ticket AAA"
    }));
    req_create_ticket.await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    hc.do_delete("/api/tickets/2").await?.print().await?;

    Ok(())
}