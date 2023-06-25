use yew::prelude::*;

use crate::models::familias::Familia;

#[function_component(Families)]
pub fn families() -> Html {
    // Get families
    let familias_json = include_str!("../mocks/familias.json");
    let familias: Vec<Familia> =
        serde_json::from_str(familias_json).expect("Failed to conver familias.json into model");
    // Render families
    let familias_html = familias
        .iter()
        .map(|familia| {
            html! { <div class={"family card"}>{familia.nombre()}</div> }
        })
        .collect::<Html>();
    html! {
        <section class={"families grid-container"}>
            {familias_html}
        </section>
    }
}
