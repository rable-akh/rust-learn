use axum::{Json, Router, routing::{post}};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookies, Cookie};

use crate::{Error, Result, web};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_json))
}

async fn api_json(cookie: Cookies, playload: Json<LoginPlayload>) -> Result<Json<Value>>{

    // TODO:: Implement reql db/auth loginc.
    if playload.username !="demo" || playload.pwd != "pass" {
        return Err(Error::LoginFail);
    }
    // TODO: Set cookie
    cookie.add(Cookie::new(web::AUTH_TOKEN, "blablabla"));

    // Todo: create
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)

}

#[derive(Debug, Deserialize)]
struct LoginPlayload {
    username: String,
    pwd: String
}