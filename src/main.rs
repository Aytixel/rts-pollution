use rocket::{
    Responder, launch, routes,
    serde::{Serialize, json::Json},
};

mod building;
mod map;
mod resource;
mod team;
mod villager;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ErrorResponse {
    code: u32,
    message: String,
    timestamp: String,
}

#[derive(Responder)]
enum ResponseResult<T> {
    #[response(status = 200)]
    Ok(Json<T>),
    #[response(status = 204)]
    NoContent(()),
    #[response(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[response(status = 401)]
    Unauthorized(Json<ErrorResponse>),
    #[response(status = 403)]
    Forbidden(Json<ErrorResponse>),
    #[response(status = 404)]
    NotFound(Json<ErrorResponse>),
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/team",
            routes![
                team::teams,
                team::team,
                villager::villagers,
                villager::villager,
                villager::villager_action,
            ],
        )
        .mount("/map", routes![map::map])
        .mount("/resource", routes![resource::resources])
        .mount(
            "/building",
            routes![
                building::buildings,
                building::building,
                building::available_buildings,
            ],
        )
}
