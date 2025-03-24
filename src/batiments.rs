use rocket::{
    get,
    serde::{Deserialize, Serialize},
};

use crate::{
    ResponseResult,
    ressources::{NomRessource, Ressources},
};

#[derive(Serialize)]
#[serde(crate = "rocket::serde", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TypeTerrain {
    Plaine,
    Desert,
    Lac,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TypeBatiment {
    CabaneDeBucheron,
    Scierie,
    AtelierDeTailleDePierre,
    Carriere,
    ExcavatriceAFer,
    MineDeFer,
    AtelierDeCharbonnier,
    MineDeCharbon,
    Moulin,
    Ferme,
    Port,
    Eolienne,
    CentraleElectriqueAuCharbon,
    CentraleAuMethane,
    CentraleABiomasse,
    TurbineHydrolique,
    InstallationForestiere,
    UsineDeRenouvellement,
    PuitsDeCarbon,
    TourDeGuet,
    Observatoire,
    Marche,
    Musee,
    Bibliotheque,
    Theatre,
    GrandeStatue,
    Capitole,
    BateauDeCroisiere,
    GrandeBibliotheque,
    Chateau,
    ReacteurAFusionNucleaire,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CibleBonus {
    Equipe,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeclancheurBonus {
    Auto,
    Recolte,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct DeclanchementBonus {
    declancheur: DeclancheurBonus,
    #[serde(rename = "descriptionDeclencheur")]
    description: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct BonusBatiment {
    #[serde(rename = "idRessource")]
    id: String,
    ressource: NomRessource,
    quantite: i64,
    cible_bonus: CibleBonus,
    declenchement: DeclanchementBonus,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Batiment {
    id: String,
    description: String,
    r#type: TypeBatiment,
    #[serde(rename = "tempsConstruction")]
    temps_construction: usize,
    #[serde(rename = "estUneMerveille")]
    est_une_merveille: bool,
    #[serde(rename = "contructibleSur")]
    constructible_sur: Vec<TypeTerrain>,
    #[serde(rename = "coutParTour")]
    cout_par_tour: Vec<Ressources>,
    #[serde(rename = "coutConstruction")]
    cout_construction: Vec<Ressources>,
    #[serde(rename = "bonusConstruction")]
    bonus_construction: Vec<Ressources>,
    bonus: Vec<BonusBatiment>,
}

#[get("/")]
pub fn batiments() -> ResponseResult<Vec<Batiment>> {}

#[get("/disponible")]
pub fn batiments_disponible() -> ResponseResult<Vec<Batiment>> {}

#[get("/<id>")]
pub fn batiments_id(id: &str) -> ResponseResult<Batiment> {}
