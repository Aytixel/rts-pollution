use rocket::{
    Responder, launch, routes,
    serde::{Serialize, json::Json},
};

mod batiments;
mod equipes;
mod monde;
mod ressources;
mod villageois;

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
            "/equipes",
            routes![
                equipes::equipes,
                equipes::equipes_id,
                villageois::villageois,
                villageois::villageois_id,
                villageois::villageois_action,
            ],
        )
        .mount("/monde", routes![monde::map])
        .mount("/ressources", routes![ressources::ressources])
        .mount(
            "/batiments",
            routes![
                batiments::batiments,
                batiments::batiments_id,
                batiments::batiments_disponible
            ],
        )
}
