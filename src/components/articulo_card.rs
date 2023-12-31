use yew::prelude::*;

use crate::{
    components::{
        btn_container::BtnContainer, modal::Modal, precio_item::PrecioItem,
        upd_articulo_form::UpdateArticuloForm,
    },
    models::articulo::Articulo,
};

#[derive(Debug, Clone, Default, Properties, PartialEq)]
pub struct ArticuloCardProps {
    pub articulo: Articulo,
}

#[function_component(ArticuloCard)]
pub fn articulo_card(props: &ArticuloCardProps) -> Html {
    let art = props.articulo.to_owned();
    let btn_upd = use_node_ref();
    html! {
        <div key={art.id} class={"articulo-card"}>
            <div style={"display: flex; justify-content:space-between; align-items:center"}>
                <h4 style={"width:100%;"}>{&props.articulo.nombre}</h4>
                <BtnContainer refe={btn_upd.clone()} />
            </div>
            <ul style={"margin-top: 0.5rem;"}>
                <PrecioItem art={art.clone()}/>
            </ul>
        <Modal refe={btn_upd.clone()}>
            <UpdateArticuloForm {art} refe={btn_upd.clone()} />
        </Modal>
        </div>
    }
}
