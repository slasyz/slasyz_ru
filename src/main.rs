use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /api/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("api" / String)
        .map(|name| format!("Hello, {}!", name));

    // TODO: access log

    warp::serve(hello)
        .run(([127, 0, 0, 1], 8001))
        .await;
}
