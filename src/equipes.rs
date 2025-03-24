use rocket::{get, serde::Serialize};

use crate::{ResponseResult, ressources::Ressources, villageois::Villageois};

#[derive(Serialize)]
#[serde(crate = "rocket::serde", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TypeEquipe {
    Etu,
    Pro,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Equipe {
    #[serde(rename = "idEquipe")]
    id: String,
    nom: String,
    r#type: String,
    villageois: Vec<Villageois>,
    ressources: Vec<Ressources>,
}

#[get("/")]
pub fn equipes() -> ResponseResult<Vec<Equipe>> {}

#[get("/<id>")]
pub fn equipes_id(id: &str) -> ResponseResult<Equipe> {}
