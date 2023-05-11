use crate::structure::Movie;
use actix_web::{web, Error, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

async fn get_movie(name: String) -> Result<Movie, Error> {
    let url = format!("http://www.omdbapi.com/?t={}&apikey=62008499", name);
    let response = reqwest::get(&url).await;
    let movie: Movie = response.expect("No movie found").json().await.unwrap();
    Ok(movie)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Someform {
    name: String,
}

pub async fn find(
    app_data: web::Data<crate::AppState>,
    movie_name: web::Form<Someform>,
) -> impl Responder {
    let result = web::block(move || {
        app_data
            .service_container
            .user
            .find_doc(movie_name.name.to_string())
    })
    .await
    .unwrap();
    match result {
        Ok(result) => HttpResponse::Ok().json(result.unwrap()),
        Err(e) => {
            println!("Error while getting , {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn insert(
    app_data: web::Data<crate::AppState>,
    movie_name: web::Form<Someform>,
) -> impl Responder {
    let data = get_movie(movie_name.name.to_string()).await.unwrap();
    let result = web::block(move || {
        app_data
            .service_container
            .user
            .insert_doc(&data)
    })
        .await
        .unwrap();
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("Error while getting , {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
