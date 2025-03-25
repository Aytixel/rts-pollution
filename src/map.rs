use rocket::{get, serde::Serialize};

use crate::{ResponseResult, building::Building, resource::Resources, team::Team};

#[derive(Serialize)]
#[serde(crate = "rocket::serde", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BiomeType {
    Plain,
    Desert,
    Lake,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Biome {
    name: BiomeType,
    description: String,
    allowed_buildings: Vec<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TerrainType {
    Forrest,
    Rocks,
    Grove,
    CoalDeposit,
    OreDeposit,
    SmallWoodsAndRockPile,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Terrain {
    name: TerrainType,
    description: String,
    available_resources: Resources,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TileBuilding {
    id: String,
    progression: u32,
    owner: Team,
    details: Building,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Tile {
    x: u64,
    y: u64,
    biome: Biome,
    terrain: Terrain,
    building: Option<TileBuilding>,
    buildable: bool,
    owner: Team,
    resources: Vec<Resources>,
}

#[get("/")]
pub fn map() -> ResponseResult<Vec<Tile>> {
    ResponseResult::NoContent(())
}
