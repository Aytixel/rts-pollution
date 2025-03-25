use rocket::{get, serde::Serialize};

use crate::{ResponseResult, resource::Resources, villager::Villager};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Team {
    id: String,
    name: String,
    villagers: Vec<Villager>,
    resources: Vec<Resources>,
}

#[get("/")]
pub fn teams() -> ResponseResult<Vec<Team>> {
    ResponseResult::NoContent(())
}

#[get("/<id>")]
pub fn team(id: &str) -> ResponseResult<Team> {
    ResponseResult::NoContent(())
}
