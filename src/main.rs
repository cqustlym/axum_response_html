use axum::{
    Router,
    routing::get,
    response::Html,
};

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127,0,0,1],3000));
    let app = Router::new().route("/", get(test));
    
    println!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn test() -> Html<&'static str>{
    Html("<h1>This is a page for testing!</h1>")
}