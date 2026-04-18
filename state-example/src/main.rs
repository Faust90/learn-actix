use actix_web::{App, HttpServer, get, web};

pub struct AppState {
    app_name: String,
}

// state is injected into the handler via web::Data<AppState>
#[get("/")]
async fn index(state: web::Data<AppState>) -> impl actix_web::Responder {
    let shared_state = &state.app_name;
    actix_web::HttpResponse::Ok().body(format!("Hello, {}!", shared_state))
}
#[get("/bye")]
async fn bye(state: web::Data<AppState>) -> impl actix_web::Responder {
    let shared_state = &state.app_name;
    actix_web::HttpResponse::Ok().body(format!("Byebye, {}!", shared_state))
}

// Needed for async main function for actix-web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // inject AppState struct into the app data
            // must be wrapped in web::Data::new()
            // so that it can be shared across handlers
            .app_data(web::Data::new(AppState {
                app_name: String::from("BARAUS"),
            }))
            // bind the handlers
            .service(index)
            .service(bye)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
