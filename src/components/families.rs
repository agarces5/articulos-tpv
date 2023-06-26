use web_sys::HtmlElement;
use yew::prelude::*;

use crate::{
    context::filters::{Filter, FilterContext},
    models::familias::Familia,
};

#[function_component(Families)]
pub fn families() -> Html {
    // Leemos los filtros
    let filter_context = use_context::<FilterContext>()
        .expect("Se esta intentando acceder al contexto de Filtros fuera del Provider");

    // Get families
    let familias_json = include_str!("../mocks/familias.json");
    let familias: Vec<Familia> =
        serde_json::from_str(familias_json).expect("Failed to conver familias.json into model");
    // Creamos el callback
    let set_filter = Callback::from(move |e: MouseEvent| {
        let filter_context = filter_context.clone();
        let target = e
            .target_dyn_into::<HtmlElement>()
            .unwrap()
            .get_attribute("value");
        if let Some(value) = target {
            filter_context.dispatch(Filter::new(
                filter_context.cajtpv.to_string(),
                value,
                filter_context.panel.to_owned(),
            ));
        }
    });
    // Render families
    let familias_html = familias
        .iter()
        .map(|familia| {
            let set_filter = set_filter.clone();
            html! {
                <button
                    class={"family card"}
                    onclick={set_filter}
                    value={familia.familia().to_string()}>
                        {familia.nombre()}
                </button>
            }
        })
        .collect::<Html>();
    html! {
        <section class={"families grid-container"}>
            {familias_html}
        </section>
    }
}
