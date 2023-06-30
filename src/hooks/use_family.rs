use yew::hook;

use crate::models::familias::Familia;

#[hook]
pub fn use_family() -> Vec<Familia> {
    // Get families
    let familias_json = include_str!("../mocks/familias.json");

    serde_json::from_str(familias_json).expect("Failed to conver familias.json into model")
}
