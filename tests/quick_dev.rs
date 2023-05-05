#![allow(unused)]

use anyhow::Result;
use serde_json::json;
use std::{error::Error, fs::read_to_string};


#[tokio::test]
async fn quick_dev() -> Result<(), Box<dyn Error>> {

    let hc = httpc_test::new_client("http://localhost:8080/")?;

    hc.do_get("hello/barlolo").await?.print().await?;

    let login = hc.do_post("api/login", json!({
        "username": "demo",
        "pass": "pass"
    }));

    login.await?.print().await;

    Ok(())
}