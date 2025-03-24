use rocket::{
    get,
    serde::{Deserialize, Serialize},
};

use crate::ResponseResult;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NomRessource {
    Bois,
    Pierre,
    Fer,
    Charbon,
    Nourriture,
    Energie,
    Point,
    Pollution,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TypeRessource {
    Recoltable,
    Produite,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Ressource {
    #[serde(rename = "idRessource")]
    id: String,
    description: String,
    nom: NomRessource,
    r#type: TypeRessource,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Ressources {
    ressource: NomRessource,
    quantite: i64,
}

#[get("/")]
pub fn ressources() -> ResponseResult<Vec<Ressource>> {}
