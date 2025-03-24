use rocket::{get, serde::Serialize};

use crate::{ResponseResult, batiments::Batiment, equipes::Equipe, ressources::Ressources};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Biome {
    identifiant: String,
    nom: String,
    description: String,
    #[serde(rename = "batimentsContructible")]
    batiments_constructible: Vec<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Terrain {
    identifiant: String,
    nom: String,
    description: String,
    #[serde(rename = "ressourcesPresente")]
    ressources_presente: Ressources,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CaseBatiment {
    progression: u32,
    identifiant: String,
    proprietaire: Equipe,
    #[serde(rename = "detailBatiment")]
    detail_batiment: Batiment,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Case {
    coord_x: u64,
    coord_y: u64,
    biome: Biome,
    terrain: Terrain,
    batiment_construit: Option<CaseBatiment>,
    accessible: bool,
    proprietaire: Equipe,
    ressources: Vec<Ressources>,
}

#[get("/map")]
pub fn map() -> ResponseResult<Vec<Case>> {}
