use crate::structure::Movie;
use actix_web::{web, Error, HttpResponse, Responder};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};


pub async fn get_movie(name: String) -> Result<Movie, Error> {
    let url = format!("http://www.omdbapi.com/?t={}&apikey=62008499", name);
    let response = reqwest::get(&url).await;
    let movie: Movie = response.expect("No movie found").json().await.unwrap();
    Ok(movie)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Someform {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SomeErrorStruct{
    error: String,
}
pub async fn find(
    app_data: web::Data<crate::AppState>,
    movie_name: web::Form<Someform>,
) -> impl Responder {
    let name  = movie_name.name.to_string();
    if name.trim().is_empty() {
        HttpResponse::BadRequest().status(StatusCode::BAD_REQUEST).body("Invalid Input".to_string())
    } else {
            let _appdata = app_data.clone();
            let new_appdata = app_data.clone();
            let result = web::block(move || {
                _appdata
                    .service_container
                    .user
                    .find_doc(movie_name.name.to_string())
            }).await.unwrap().unwrap();
            if let Some(_x) = result.clone() {
                HttpResponse::Ok().json(result)
            } else {
                let data = get_movie(name).await.unwrap();
                let response_ = data.Response.as_ref().unwrap().to_owned();
                if response_ == "False" {
                    HttpResponse::NotFound().status(StatusCode::NOT_FOUND).body("No Movie with this name.")
                } else {
                    let insert_result  = web::block(move || {app_data.service_container.user.insert_doc(&data)}).await.unwrap().unwrap();
                    let final_result = web::block(move || {new_appdata.service_container.user.find_doc_by_id(insert_result)}).await.unwrap().unwrap();
                    HttpResponse::Ok().json(final_result)
                }
            }
        }
}       