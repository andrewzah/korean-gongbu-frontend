use rand::prelude::*;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

mod utils;

use korean_nums::{
    NumberSystem,
    hangeul_from_money,
    hangeul_from_int,
};


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize, Deserialize)]
struct KoreanInteger {
    number: u128,
    hangeul: String,
}

#[wasm_bindgen]
pub fn init_hook() {
    utils::set_panic_hook()
}

#[wasm_bindgen]
pub fn random_korean_int(lower_str: &str, upper_str: &str, num_system: &str) -> String {
    let lower = lower_str.parse::<u128>().expect("unable to parse lower into u128");
    let upper = upper_str.parse::<u128>().expect("unable to parse lower into u128");

    let roll = random_int(lower, upper);

    let hangeul = match num_system {
        "pure" => hangeul_from_int(roll, NumberSystem::PureKorean),
        _ => hangeul_from_int(roll, NumberSystem::SinoKorean),
    };

    let result = KoreanInteger {
        number: roll,
        hangeul: hangeul
    };

    serde_json::to_string(&result)
        .expect("Unable to use serde_json::to_string")
}

#[wasm_bindgen]
pub fn random_korean_money(lower_str: &str, upper_str: &str) -> String {
    let lower = lower_str.parse::<u128>().expect("Unable to parse u128");
    let upper = lower_str.parse::<u128>().expect("Unable to parse u128");

    let roll = random_money(lower, upper);
    let hangeul = hangeul_from_int(roll, NumberSystem::SinoKorean);

    let result = KoreanInteger {
        number: roll,
        hangeul
    };

    serde_json::to_string(&result).unwrap()
}

fn random_int(lower: u128, upper: u128) -> u128 {
    let mut rng = rand::thread_rng();
    rng.gen_range(lower, upper + 1)
}

// korean money doesn't use cents
// generate relatively even numbers, e.g.
// 81,500
// 101,000
// TODO
fn random_money(lower: u128, upper: u128) -> u128 {
    let mut rng = rand::thread_rng();
    rng.gen_range(lower, upper + 1)
}
