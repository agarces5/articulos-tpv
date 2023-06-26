use yew::prelude::*;

use crate::models::articulo::{Articulo, Precio};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CardProps {
    pub articulo: Articulo,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    // Renderizar los precios
    let render_precios = |detalle: Precio| {
        html! {
            <li key={detalle.tipotarifa.as_ref()}>{format!("{}",detalle)}</li>
        }
    };
    let article = props.articulo.to_owned();
    let detalle = article.detalles.first().cloned().unwrap_or_default();
    let caja_render = detalle
        .precios
        .iter()
        .cloned()
        .map(render_precios)
        .collect::<Html>();

    let onclick = &props.onclick;
    html! {
        <button key={article.articulo} class={"articulo card"} {onclick}>
            <h4>{&props.articulo.nombre}</h4>
            <p>{&props.articulo.articulo}</p>
            <ul>
                {caja_render}
            </ul>
        </button>
    }
}
