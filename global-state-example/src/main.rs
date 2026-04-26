use std::sync::Mutex;

use actix_web::{App, HttpServer, get, web};

pub struct AppState {
    app_name: String,
    counter: Mutex<i32>, // Mutex is used to make the counter mutable in a thread-safe way
}

// state is injected into the handler via web::Data<AppState>
#[get("/")]
async fn index(state: web::Data<AppState>) -> impl actix_web::Responder {
    let shared_state = &state.app_name;
    let mut counter = state.counter.lock().unwrap(); // acquire lock and unwrap, panics if poisoned
    *counter += 1; // mutate counter
    actix_web::HttpResponse::Ok().body(format!("Hello, {}! Counter: {}", shared_state, counter))
}
#[get("/bye")]
async fn bye(state: web::Data<AppState>) -> impl actix_web::Responder {
    let shared_state = &state.app_name;
    let mut counter = state.counter.lock().unwrap(); // acquire lock and unwrap, panics if poisoned
    *counter = 0; // mutate counter
    actix_web::HttpResponse::Ok().body(format!("Byebye, {}! Counter: {}", shared_state, counter))
}

// Needed for async main function for actix-web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // As per official documentation:
    // Note that Data should be constructed outside the HttpServer::new closure if shared,
    // potentially mutable state is desired. Data is cheap to clone; internally, it uses an Arc.
    let global_counter = web::Data::new(AppState {
        app_name: String::from("ASDRUBAIL"),
        counter: Mutex::new(0),
    });

    // move global_counter into the closure so it can be cloned inside
    HttpServer::new(move || {
        App::new()
            // inject AppState struct into the app data
            .app_data(global_counter.clone())
            // bind the handlers
            .service(index)
            .service(bye)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
