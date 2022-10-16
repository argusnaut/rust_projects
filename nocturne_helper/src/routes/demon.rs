use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

use crate::enums::demon::DemonRace;
use crate::enums::demon::Resistance;
use crate::services;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Demon {
    pub race: DemonRace,
    pub level: i32,
    pub name: String,
    pub hp: i32,
    pub mp: i32,
    pub str: i32,
    pub mag: i32,
    pub vit: i32,
    pub agi: i32,
    pub luck: i32,
    pub phys: Resistance,
    pub fire: Resistance,
    pub ice: Resistance,
    pub elec: Resistance,
    pub force: Resistance,
    pub expel: Resistance,
    pub death: Resistance,
    pub mind: Resistance,
    pub nerve: Resistance,
    pub curse: Resistance,
}

#[get("/demon/get-demon")]
pub fn get_demon() -> Json<Demon> {
    Json(services::demon::get_demon())
}

#[post("/demon/create-demon", format = "json", data = "<demon>")]
pub fn create_demon(demon: Json<Demon>) -> Json<Demon> {
    Json(services::demon::create_demon(demon))
}
