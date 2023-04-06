use actix_web::{web, HttpResponse, Responder, Error};
use serde::{Deserialize, Serialize};
use crate::structure::Movie;
use crate::service::{UserService};


async fn get_movie(name: String) -> Result<Movie, Error> {
    let url = format!("http://www.omdbapi.com/?t={}&apikey=62008499", name);
    let response = reqwest::get(&url).await;
    let movie: Movie = response.expect("No movie found").json().await.unwrap();
    Ok(movie)
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
pub struct Someform {
    name: String,
}

pub async fn find(app_data: web::Data<crate::AppState>, movie_name: web::Form<Someform>) -> impl Responder {
    let result = web::block(move || app_data.service_container.user.find_doc(movie_name.name.to_string())).await.unwrap();
    match result {
        Ok(result) => HttpResponse::Ok().json(result.unwrap()),
        Err(e) => {
            println!("Error while getting , {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn insert(app_data: web::Data<crate::AppState>, movie_name: web::Form<Someform>) -> impl Responder {
    let data = get_movie(movie_name.name.to_string()).await;
    let result = web::block(move || app_data.service_container.user.insert_doc(&data.unwrap())).await.unwrap();
    match result {
        Ok(result) => HttpResponse::Ok().json(result.unwrap()),
        Err(e) => {
            println!("Error while getting , {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}