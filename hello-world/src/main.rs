use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};

// GET a hello world response on the root path
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

// POST an echo response returning the request body on the /echo path
// example: curl -X POST 127.0.0.1:8080/echo --json "{ "baraus":"baraus"}"
// will return { baraus:baraus}
#[post("/echo")]
async fn echo(req: String) -> impl Responder {
    HttpResponse::Ok().body(req)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello manual, world!")
}

// Main function to run the Actix web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create a new Actix web server
    HttpServer::new(|| {
        // Create a new Actix web application
        // It will register Routes and share state across handlers in the same scope
        App::new()
            .service(hello) // binding hello
            .service(echo) // binding echo
            .route("/hey", web::get().to(manual_hello)) // binding manual_hello using method instead of macro
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
