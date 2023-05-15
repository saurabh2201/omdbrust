/// Importing all necessary modules
use crate::structure::Movie;
use mongodb::{error::Error, sync::Collection, bson::{oid::ObjectId, Document}};

///Declaring struct Userservice which represents bson document.
#[derive(Clone)]
pub struct UserService {
    pub collection: mongodb::sync::Collection<bson::Document>,
}

/// Implementation of "UserService" struct.
impl UserService {
    /// Creating a new UserService.
    pub fn new(collection: Collection<bson::Document>) -> UserService {
        UserService { collection }
    }
    /// Inserting a document of movie in MongoDB and returning the
    /// result of InsertOneResult which contains inserted id of the document.
    pub fn insert_doc(&self, movie_name: &Movie) -> Result<ObjectId, Error> {
        let insertion = self.collection.insert_one(
            bson::doc! {
                    "title" : movie_name.Title.as_ref(),
                    "year" : movie_name.Year.as_ref(),
                    "rated" : movie_name.Rated.as_ref(),
                    "released" : movie_name.Released.as_ref(),
                    "runtime" : movie_name.Runtime.as_ref(),
                    "genre" : movie_name.Genre.as_ref(),
                    "director" : movie_name.Director.as_ref(),
                    "writer" : movie_name.Writer.as_ref(),
                    "actors" : movie_name.Actors.as_ref(),
                    "language" : movie_name.Language.as_ref(),
                    "country" : movie_name.Country.as_ref(),
                    "awards" : movie_name.Awards.as_ref(),
                    "poster" : movie_name.Poster.as_ref(),
                    "metascore" : movie_name.Metascore.as_ref(),
                    "imdbRating" : movie_name.imdbRating.as_ref(),
                    "imdbVotes" : movie_name.imdbVotes.as_ref(),
                    "imdbID" : movie_name.imdbID.as_ref(),
                    "type" : movie_name.Type.as_ref(),
                    "response" : movie_name.Response.as_ref()
            },
            None,
        );
        let _id = insertion.unwrap().inserted_id.as_object_id().unwrap();
        println!("{:?}", _id);
        Ok(_id)
    }

    
    // Finding a movie with a given movie name.
    pub fn find_doc(&self, movie_name: String) -> Result<Option<Document>, Error> {
        self.collection.find_one(
            bson::doc! {
                "title":{"$regex":movie_name, "$options":"i"}
            },
            None,
        )
    }
    // Finding a movie with a given movie name.
    pub fn find_doc_by_id(&self, id: ObjectId) -> Result<Option<Document>, Error> {
        self.collection.find_one(
            bson::doc! {
                "_id": id,
            },
            None,
        )
    }
}
