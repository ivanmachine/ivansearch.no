use warp::Filter;

#[tokio::main]
async fn main() {
    // Define the "hello" route
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    println!("Starting server on 0.0.0.0:8000");

    // Start the server on all interfaces (0.0.0.0)
    warp::serve(hello)
        .run(([0, 0, 0, 0], 3000))
        .await;
}
