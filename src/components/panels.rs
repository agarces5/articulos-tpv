use yew::prelude::*;

use crate::{
    context::filters::{Filter, FilterContext},
    models::{paneles::ListOfPanels, paneles_dto::*},
};

#[function_component(Panels)]
pub fn panels() -> Html {
    // Recibimos la lista de paneles
    let paneles_json = include_str!("../mocks/paneles.json");
    let lista_de_paneles: Vec<PanelDTO> =
        serde_json::from_str(paneles_json).expect("Fallo al serializar paneles.json");
    let lista_de_paneles: ListOfPanels = ListOfPanelsDTO(lista_de_paneles).into();

    // Leemos los filtros del contexto
    let filters_ctx = use_context::<FilterContext>()
        .expect("Se esta intentando usar el contexto de FILTROS fuera del provider");
    let filters = Filter::new(
        filters_ctx.cajtpv.to_owned(),
        filters_ctx.familia.to_owned(),
        filters_ctx.panel.to_owned(),
    );

    // Filtramos
    let paneles_filtrados: Vec<_> = lista_de_paneles.filter(filters);

    // Construimos el html
    let paneles_render = paneles_filtrados
        .iter()
        .map(|(_id, panel)| {
            html! { <div class={"panel card"}>{&panel.nombre}</div> }
        })
        .collect::<Html>();
    html! {
        <section class={"panels flex-container"}>
            <h2>{"Paneles"}</h2>
            {paneles_render}
        </section>
    }
}
