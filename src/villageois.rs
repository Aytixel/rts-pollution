use rocket::{
    get, post,
    serde::{Deserialize, Serialize, json::Json},
};

use crate::{ResponseResult, batiments::TypeBatiment, ressources::NomRessource};

#[derive(Serialize)]
#[serde(crate = "rocket::serde", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NomVillageois {
    Peon,
    PeonMalade,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TypeVillageois {
    nom: NomVillageois,
    description: String,
    #[serde(rename = "mutliplicateurDeCooldown")]
    mutliplicateur_de_cooldown: f64,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Villageois {
    #[serde(rename = "idVillageois")]
    id: String,
    nom: String,
    r#type: TypeVillageois,
    #[serde(rename = "dateDerniereAction")]
    date_derniere_action: String,
    disponible: bool,
    #[serde(rename = "positionX")]
    position_x: u64,
    #[serde(rename = "positionY")]
    position_y: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NomAction {
    DeplacementDroite,
    DeplacementGauche,
    DeplacementHaut,
    DeplacementBas,
    Recolter,
    CommencerConstruction,
    Construire,
    RecyclerBatiment,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde", untagged)]
pub enum DemandeActionReference {
    Recolte(NomRessource),
    Batiment(TypeBatiment),
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DemandeAction {
    action: NomAction,
    reference: Option<String>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ActionResponse {
    ressource: u64,
}

#[get("/<id_equipe>/villageois")]
pub fn villageois(id_equipe: &str) -> ResponseResult<Vec<Villageois>> {}

#[get("/<id_equipe>/villageois/<id_villageois>")]
pub fn villageois_id(id_equipe: &str, id_villageois: &str) -> ResponseResult<Villageois> {}

#[post(
    "/<id_equipe>/villageois/<id_villageois>/demander-action",
    data = "<action>"
)]
pub fn villageois_action(
    id_equipe: &str,
    id_villageois: &str,
    action: Json<DemandeAction>,
) -> ResponseResult<ActionResponse> {
}
