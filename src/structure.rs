use serde::{Deserialize, Serialize};

/// Declaration for the movie struct contaning fields: "Title",
/// "Year","Rated","Released","Runtime","Genre","Director","Writer",
/// "Actors","Language","Country","Awards","Poster","Metascore",
/// "ImdbRating", "ImdbVotes", "ImdbID", "Type" and "Response"
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Movie {
    pub Title: Option<String>,
    pub Year: Option<String>,
    pub Rated: Option<String>,
    pub Released: Option<String>,
    pub Runtime: Option<String>,
    pub Genre: Option<String>,
    pub Director: Option<String>,
    pub Writer: Option<String>,
    pub Actors: Option<String>,
    pub Language: Option<String>,
    pub Country: Option<String>,
    pub Awards: Option<String>,
    pub Poster: Option<String>,
    pub Metascore: Option<String>,
    pub ImdbRating: Option<String>,
    pub ImdbVotes: Option<String>,
    pub ImdbID: Option<String>,
    pub Type: Option<String>,
    pub Response: Option<String>,
}

