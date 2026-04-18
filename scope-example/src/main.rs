use actix_web::{App, HttpServer, web};

async fn index() -> impl actix_web::Responder {
    "Hello, World from index!"
}

// Needed for async main function for actix-web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            // Add prefix to route attached to it
            web::scope("/app")
                // GET route handled by /app/index.html
                // will respond with "Hello, World from index!"
                .route("/index.html", web::get().to(index)),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
