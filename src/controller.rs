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

pub async fn controller(app_data: web::Data<crate::AppState>, movie_name: web::Form<Someform>) -> impl Responder {
    //let result = web::block(move || app_data.service_container.user.find_doc(movie_name.name.to_string())).await;
     let name = movie_name.name.to_string();
    if  name.trim().is_empty() {
        
        let response_  = Some(bson::doc! {"message":"Bad request"});
        HttpResponse::BadRequest().status(StatusCode::BAD_REQUEST).json(response_.unwrap())

    } else {

        let mongo_movie_data = UserService::find_doc(&UserService,name.to_string()).unwrap();

        let result_data = match mongo_movie_data {
            None => {
                    let data = get_movie(name.to_string()).await;
                    let response_ = data.as_ref().unwrap().Response.to_owned().unwrap();
                    if response_ == "False" {

                        let response = Some(bson::doc! {"message":"Not found"});
                        HttpResponse::NotFound().status(StatusCode::NOT_FOUND).json(response)

                    } else {

                        let object_id = insert_doc(&data.unwrap()).await.unwrap();
                        let find_data = find_movie_by_id(object_id).await.unwrap();
                        HttpResponse::Ok().status(StatusCode::OK).json(find_data.unwrap())
                    }
                },
            _    => HttpResponse::Ok().json(mongo_movie_data.unwrap())
        };  
        result_data
    }
}