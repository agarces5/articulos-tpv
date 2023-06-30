use web_sys::HtmlElement;
use yew::prelude::*;

use crate::{
    context::filters::{Filter, FilterContext},
    hooks::use_family::use_family,
    models::familias::Familia,
};

fn set_filter(filter_ctx: UseReducerHandle<Filter>) -> Callback<MouseEvent> {
    // Creamos el callback
    Callback::from(move |e: MouseEvent| {
        let target = e
            .target_dyn_into::<HtmlElement>()
            .unwrap()
            .get_attribute("value");
        if let Some(value) = target {
            filter_ctx.dispatch(Filter::new(
                filter_ctx.cajtpv.to_string(),
                value,
                filter_ctx.panel.to_owned(),
            ));
        }
    })
}

#[function_component(Families)]
pub fn families() -> Html {
    // Leemos los filtros
    let filter_context = use_context::<FilterContext>()
        .expect("Se esta intentando acceder al contexto de Filtros fuera del Provider");
    // Get families
    let familias: Vec<Familia> = use_family();
    // Render families
    let familias_html = familias
        .iter()
        .map(|familia| {
            html! {
                <button
                    class={"family card"}
                    onclick={set_filter(filter_context.clone())}
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
