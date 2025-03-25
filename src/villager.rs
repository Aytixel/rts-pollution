use rocket::{
    get, post,
    serde::{Deserialize, Serialize, json::Json},
};

use crate::{ResponseResult, building::BuildingType, resource::ResourceName};

#[derive(Serialize)]
#[serde(crate = "rocket::serde", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VillagerType {
    Peon,
    SickPeon,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct VillagerTypeDescription {
    name: VillagerType,
    description: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Villager {
    id: String,
    name: String,
    r#type: VillagerTypeDescription,
    timestamp: String,
    available: bool,
    x: u64,
    y: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Deserialize)]
#[serde(
    crate = "rocket::serde",
    rename_all = "SCREAMING_SNAKE_CASE",
    tag = "kind",
    content = "action"
)]
pub enum ActionCall {
    PlaceBuilding(BuildingType),
    Build,
    Recycle,
    Harvest(ResourceName),
    Move(Direction),
}

#[get("/<id_team>/villager")]
pub fn villagers(id_team: &str) -> ResponseResult<Vec<Villager>> {
    ResponseResult::NoContent(())
}

#[get("/<id_team>/villager/<id_villager>")]
pub fn villager(id_team: &str, id_villager: &str) -> ResponseResult<Villager> {
    ResponseResult::NoContent(())
}

#[post("/<id_team>/villager/<id_villager>/action-call", data = "<action>")]
pub fn villager_action(
    id_team: &str,
    id_villager: &str,
    action: Json<ActionCall>,
) -> ResponseResult<()> {
    ResponseResult::NoContent(())
}
