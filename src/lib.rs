mod routes;

use axum::Server;
use routes::create_router;

pub async fn run() {
    // an Axum router or an instance of router
    let app = create_router();
    
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        // into_make_service() is a method, and is part of 'Router' trait, converts a router into 'MakeService' implementation
        // 'MakeService' is a trait from tower crate, it creates new service for each incoming connection
        // basically into_make_service() on a router instance returns a 'MakeService' implementation, allowing router instance
        // to be used as a service factory
        .serve(app.into_make_service())
        .await
        .unwrap();
}