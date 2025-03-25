use rocket::{
    get,
    serde::{Deserialize, Serialize},
};

use crate::{
    ResponseResult,
    map::TerrainType,
    resource::{ResourceName, Resources},
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BuildingType {
    LumberjackCabin,
    Sawmill,
    StoneCuttingWorkshop,
    Quarry,
    IronExcavator,
    IronMine,
    CoalMiningWorkshop,
    CoalMine,
    Mill,
    Farm,
    Port,
    WindTurbine,
    CoalPowerPlant,
    MethanePowerPlant,
    BiomassPowerPlant,
    HydroTurbine,
    ForestryFacility,
    RenewableEnergyPlant,
    CarbonSink,
    Watchtower,
    Observatory,
    Market,
    Museum,
    Library,
    Theater,
    LargeStatue,
    CapitolBuilding,
    CruiseShip,
    LargeLibrary,
    Castle,
    NuclearFusionReactor,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BonusTriggerType {
    Auto,
    Harvest,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct BonusTrigger {
    trigger: BonusTriggerType,
    description: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct BuildingBonus {
    id: String,
    resource: ResourceName,
    quantity: i64,
    trigger: BonusTrigger,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Building {
    r#type: BuildingType,
    description: String,
    construction_count: u16,
    is_wonder: bool,
    buildable_terrain_type: Vec<TerrainType>,
    cost_per_turn: Vec<Resources>,
    construction_cost: Vec<Resources>,
    construction_bonus: Vec<Resources>,
    bonus: Vec<BuildingBonus>,
}

#[get("/")]
pub fn buildings() -> ResponseResult<Vec<Building>> {
    ResponseResult::NoContent(())
}

#[get("/<id>")]
pub fn building(id: &str) -> ResponseResult<Building> {
    ResponseResult::NoContent(())
}

#[get("/available")]
pub fn available_buildings() -> ResponseResult<Vec<Building>> {
    ResponseResult::NoContent(())
}
