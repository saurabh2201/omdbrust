use actix_web::{web, App, HttpServer, web::Data};
use std::env;
use dotenv::dotenv;
mod controller;
mod service;
mod structure;

/// struct ServiceContainer having field "user" which is instance of a function
/// "userService" from mod service
pub struct ServiceContainer {
    user: service::UserService,
}

/// Implementation of ServiceContainer
impl ServiceContainer {
    pub fn new(user: service::UserService) -> Self {
        ServiceContainer { user }
    }
}

pub struct AppState {
    service_container: ServiceContainer,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let mongo_url = env::var("MONGO_URI").expect("Mongo URL not connected");
    let client = mongodb::sync::Client::with_uri_str(mongo_url).unwrap();
    let db = client.database("MovieData");
    let user_collection = db.collection("MovieCollection");

    HttpServer::new(move || {
        let service_container =
            ServiceContainer::new(service::UserService::new(user_collection.clone()));
        let app = AppState { service_container };
        App::new()
            .app_data(Data::new(app).clone())
            .route("/getmovies", web::get().to(controller::find))
            .route("/insertmovies", web::get().to(controller::insert))
    })
    .bind("127.0.0.1:3030")?
    .run()
    .await
}
