use rocket::serde::json::Json;

use crate::enums::demon::DemonRace;
use crate::enums::demon::Resistance;
use crate::routes::demon::Demon;

pub fn get_demon() -> Demon {
    let test_demon = Demon {
        race: DemonRace::Foul,
        level: 1,
        name: String::from("Will o' Wisp"),
        hp: 30,
        mp: 18,
        str: 4,
        mag: 5,
        vit: 4,
        agi: 5,
        luck: 3,
        phys: Resistance::Resists,
        fire: Resistance::Weak,
        ice: Resistance::Weak,
        elec: Resistance::Weak,
        force: Resistance::Weak,
        expel: Resistance::Weak,
        death: Resistance::Weak,
        mind: Resistance::None,
        nerve: Resistance::None,
        curse: Resistance::None,
    };
    test_demon
}

pub fn create_demon(demon: Json<Demon>) -> Demon {
    let new_demon: Demon = Demon {
        race: demon.race,
        level: demon.level,
        name: String::from(demon.name.as_str()),
        hp: demon.hp,
        mp: demon.mp,
        str: demon.str,
        mag: demon.mag,
        vit: demon.vit,
        agi: demon.agi,
        luck: demon.luck,
        phys: demon.phys,
        fire: demon.fire,
        ice: demon.ice,
        elec: demon.elec,
        force: demon.force,
        expel: demon.expel,
        death: demon.death,
        mind: demon.mind,
        nerve: demon.nerve,
        curse: demon.curse,
    };
    new_demon
}
