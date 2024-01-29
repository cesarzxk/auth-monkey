pub mod routes;
pub mod handlers;
pub mod models;

#[tokio::main]
async fn main() {
    println!("Server started at http://localhost:8000");
    warp::serve(routes::routes()).run(([127, 0, 0, 1], 3000)).await;
}