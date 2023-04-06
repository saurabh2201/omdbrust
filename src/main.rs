use actix_web::{web, App, HttpServer};
use std::env;
mod service;
mod structure;
mod controller;

/// struct ServiceContainer having field "user" which is instance of a function 
/// "userService" from mod service 
pub struct ServiceContainer {
    user: service::UserService
}

/// Implementation of ServiceContainer
impl ServiceContainer {
    pub fn new(user: service::UserService) -> Self {
        ServiceContainer {
             user 
            }
    }
}

pub struct AppState {
    service_container: ServiceContainer
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongo_url = env::var("MONGO_URL").expect("Mongo URL not connected");
    let client = mongodb::sync::Client::with_uri_str(mongo_url).unwrap();
    let db = client.database("Moviedata");
    let user_collection = db.collection("Moviecollection");


    HttpServer::new(move || {
        let service_container = ServiceContainer::new(service::UserService::new(user_collection.clone()));
        App::new()
        .app_data( AppState {service_container})
        .route("/getmovies",web::get().to(controller::find(app_data, movie_name)))
    })
    .bind("127.0.0.0:3000")?
    .run()
    .await
    
    }   

