use web_sys::HtmlElement;
use yew::prelude::*;

use crate::{
    components::modal::Modal,
    models::articulo::{Articulo, Precio},
};

#[derive(Debug, Clone, Default, Properties, PartialEq)]
pub struct ArticuloCardProps {
    pub articulo: Articulo,
}

#[function_component(ArticuloCard)]
pub fn articulo_card(props: &ArticuloCardProps) -> Html {
    let article = props.articulo.to_owned();
    let detalle = article.detalles.first().cloned().unwrap_or_default();
    let caja_render = detalle
        .precios
        .into_iter()
        .map(|precio: Precio| html! {<PrecioItem {precio} />})
        .collect::<Html>();
    let btn_upd = use_node_ref();
    html! {
        <div key={article.articulo} class={"articulo card"}>
            <div style={"display: flex; justify-content:space-between; align-items:center"}>
                <h4 style={"width:100%;"}>{&props.articulo.nombre}</h4>
                <div class={"css-controles"}>
                    <button class={"svg-button"} onclick={handle_click_update(btn_upd.clone())} >
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960"><path d="M180-180h44l443-443-44-44-443 443v44Zm614-486L666-794l42-42q17-17 42-17t42 17l44 44q17 17 17 42t-17 42l-42 42Zm-42 42L248-120H120v-128l504-504 128 128Zm-107-21-22-22 44 44-22-22Z"/>
                        </svg>
                    </button>
                    <button class={"svg-button"} onclick={handle_click_delete()}>
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960"><path d="M261-120q-24.75 0-42.375-17.625T201-180v-570h-41v-60h188v-30h264v30h188v60h-41v570q0 24-18 42t-42 18H261Zm438-630H261v570h438v-570ZM367-266h60v-399h-60v399Zm166 0h60v-399h-60v399ZM261-750v570-570Z"/></svg>
                    </button>
                </div>
            </div>
            // <p>{&props.articulo.articulo}</p>
            <ul style={"margin-top: 0.5rem;"}>
                {caja_render}
            </ul>
        <Modal _ref={btn_upd}/>
        </div>
    }
}

fn handle_click_update(_ref: NodeRef) -> Callback<MouseEvent> {
    Callback::from(move |_: MouseEvent| {
        let div = _ref.cast::<HtmlElement>();
        if let Some(div) = div {
            // gloo::console::log!(div.style().set_css_text("display: block;"));

            div.style().set_css_text("display: block;");
        }
    })
}
fn handle_click_delete() -> Callback<MouseEvent> {
    Callback::from(move |_: MouseEvent| {})
}

#[derive(Properties, PartialEq)]
struct PrecioProps {
    precio: Precio,
}

#[function_component(PrecioItem)]
fn precio_item(props: &PrecioProps) -> Html {
    let detalle = props.precio.to_owned();
    html! {
        <li key={detalle.tipotarifa.as_ref()}>{format!("{}",detalle)}</li>
    }
}
