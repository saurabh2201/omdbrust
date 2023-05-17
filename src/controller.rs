use crate::structure::Movie;
use actix_web::{web, Error, HttpResponse, Responder};
use bson::{oid::ObjectId, Document};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use crate::service::UserService;


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
            //let name = movie_name.clone().name.to_string();
            let result = web::block(move || {
                app_data
                    .service_container
                    .user
                    .find_doc(movie_name.name.to_string())
            }).await.unwrap().unwrap();
            //let data = get_movie(name).await.unwrap();
            //let insert_result  = web::block(move || {app_data.service_container.user.insert_doc(&data)}).await.unwrap().unwrap();
            if let Some(_x) = result.clone() {
                HttpResponse::Ok().json(result)
            } else {
                //let err = Some(bson::doc! {"message": "Not found"});
                let data = get_movie(name).await.unwrap();
                let insert_result  = web::block(move || {app_data.service_container.user.insert_doc(&data)}).await.unwrap().unwrap();
                
                HttpResponse::NotFound().status(StatusCode::NOT_FOUND).body("Not found".to_string())
            }
            //match result {
            //    Ok(result) => HttpResponse::Ok().json(result.unwrap()),
            //    Err(e) => {
            //        println!("Error while getting , {:?}", e);
            //        HttpResponse::InternalServerError().finish()
            //    }
            //}
            //Ok(result)
        }
}       

pub async fn insert(
    app_data: web::Data<crate::AppState>,
    movie_name: web::Form<Someform>,
) -> Result<ObjectId, Error> {
    let data = get_movie(movie_name.name.to_string()).await.unwrap();
    let result = web::block(move || {
        app_data
            .service_container
            .user
            .insert_doc(&data)
    }).await.unwrap().unwrap();
    //match result {
    //    Ok(result) => HttpResponse::Ok().json(result),
    //    Err(e) => {
    //        println!("Error while getting , {:?}", e);
    //        HttpResponse::InternalServerError().finish()
    //    }
    //}
    Ok(result)
}


pub async fn find_by_id(
    app_data: web::Data<crate::AppState>,
    movie_name: web::Form<Someform>,
) -> Result<Option<Document>, Error> {
    let id  = insert(app_data.clone(), movie_name).await.unwrap();
    let result = web::block(move || {
        app_data
            .service_container
            .user
            .find_doc_by_id(id)
    }).await.unwrap().unwrap();
    //match result  {
    //    Ok(result) => HttpResponse::Ok().json(result),
    //    Err(e) => {
    //        println!("Error while getting , {:?}", e);
    //        HttpResponse::InternalServerError().finish()
    //    }       
    //}
    Ok(result)
}


//pub async fn find_movie(
//    app_data: web::Data<crate::AppState>,
//    movie_name: web::Form<Someform>,
//) -> impl Responder {
//    let someform = movie_name.clone();
//    //let name = movie_name.clone().name.to_string();
//    if movie_name.name.to_string().trim().is_empty() {
//        let response = Some(bson::doc! {"message": "Invalid Input"});
//        HttpResponse::BadRequest().status(StatusCode::BAD_REQUEST).json(response.unwrap())
//    } else {
//        let found_data = find(app_data.clone(), movie_name).await.unwrap();
//        match found_data {
//            Some(_) => HttpResponse::Ok().json(found_data.unwrap()),
//            None => {
//                let new_data = get_movie(someform.name.to_string()).await;
//                let new_response = new_data.as_ref().unwrap().Response.to_owned().unwrap();
//                if new_response == "False" {
//                    let final_response = Some(bson::doc ! {"message":"No movie found"});
//                    HttpResponse::NotFound().status(StatusCode::NOT_FOUND).json(final_response)
//                } else {
//                    let inserted_id = insert(app_data.clone(), movie_name).await.unwrap();
//                    let doc_by_id = find_by_id(app_data.clone(), movie_name).await.unwrap();
//                    HttpResponse::Ok().status(StatusCode::OK).json(doc_by_id.unwrap())
//                }
//                
//            },
//        }
//    }     
//}