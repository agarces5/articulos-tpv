use yew::hook;

use crate::models::cajtpv::Cajtpv;

#[hook]
pub fn use_cajtpv() -> Vec<Cajtpv> {
    // Get cajtpv
    let cajtpv_json = include_str!("../mocks/cajtpv.json");

    serde_json::from_str(cajtpv_json).expect("Failed to conver cajtpv.json into model")
}
