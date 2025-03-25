use rocket::{
    get,
    serde::{Deserialize, Serialize},
};

use crate::ResponseResult;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResourceName {
    Wood,
    Stone,
    Iron,
    Coal,
    Food,
    Energy,
    Point,
    Pollution,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResourceType {
    Harvestable,
    Produced,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Resource {
    name: ResourceName,
    r#type: ResourceType,
    description: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Resources {
    resource: ResourceName,
    quantity: i64,
}

#[get("/")]
pub fn resources() -> ResponseResult<Vec<Resource>> {
    ResponseResult::NoContent(())
}
