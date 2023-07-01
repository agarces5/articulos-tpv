use yew::prelude::*;

use crate::models::articulo::{Articulo, Precio};

#[derive(Properties, PartialEq)]
pub struct PrecioProps {
    pub art: Articulo,
}

#[function_component(PrecioItem)]
pub fn precio_item(props: &PrecioProps) -> Html {
    // Get first detail in the article
    let detalle = props.art.detalles.first().cloned().unwrap_or_default();
    html! {
        {
        detalle
            .precios
            .into_iter()
            .map(|precio: Precio| html! {<li key={precio.tipotarifa.as_ref()}>{format!("{}",precio)}</li>})
            .collect::<Html>()
        }
    }
}
