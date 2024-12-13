use warp::Filter;
use nano_id::{base58, base62};

#[tokio::main]
async fn main() {
    // Route for generating a base62 ID (vulnerable)
    let base62_route = warp::path("base62")
        .map(|| {
            let id = base62::<10>();
            warp::reply::json(&id)
        });

    // Route for generating a base58 ID (vulnerable)
    let base58_route = warp::path("base58")
        .map(|| {
            let id = base58::<10>();
            warp::reply::json(&id)
        });

    // Combine routes
    let routes = base62_route.or(base58_route);

    // Start the server on localhost:3030
    println!("Server running at http://127.0.0.1:3030/");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

