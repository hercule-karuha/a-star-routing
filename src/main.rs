mod handler;
mod test_helper;

use axum::{routing::post, Router, Server};
use handler::*;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/route", post(a_star_routing));

    let addr = "0.0.0.0:3000".parse().unwrap();
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
