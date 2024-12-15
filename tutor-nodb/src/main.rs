mod handlers;
mod routes;
mod state;
mod models;

use std::{io, sync::Mutex};

use actix_web::{web, App, HttpServer};
use routes::general_routes;
use state::AppState;
use crate::routes::course_routes;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(Vec::new()),
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
