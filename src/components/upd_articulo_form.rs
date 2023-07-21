use crate::models::articulo::Articulo;
use web_sys::HtmlDialogElement;
use yew::prelude::*;

use crate::hooks::use_cajtpv::use_cajtpv;

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct UpdateArticuloFormProps {
    pub art: Articulo,
    pub refe: NodeRef,
}

#[function_component(UpdateArticuloForm)]
pub fn upd_art_form(props: &UpdateArticuloFormProps) -> Html {
    let art = props.art.to_owned();
    let modal_ref = props.refe.to_owned();
    let cajas = use_cajtpv();
    html! {
        <form onsubmit={ move |e: SubmitEvent| {
            e.prevent_default();
            // Cerramos el modal al hacer submit
            modal_ref.cast::<HtmlDialogElement>().unwrap().close();
        }} >
            <div class="form-body">
                <div class="form-group">
                    <label for="_id">{"Articulo"}</label>
                    <input type="text" id="_id" class="form-control" value={art.id.to_string()} readonly=true />
                </div>
                <div class="form-group">
                    <label for="nombre">{"Nombre:"}</label>
                    <input type="text" id="nombre" class="form-control" placeholder="AGUA 0,5L" value={art.nombre} />
                </div>
                <div class="form-group">
                    <label for="familia"></label>
                    <input type="text" id="familia" class="form-control" placeholder="BEBIDAS" value={art.familia} />
                </div>
                <div class="form-group">
                {
                    cajas.iter().map(|caja| html! {
                        <>
                            <input type="checkbox" id={caja.id()} class="form-control" value={art.detalles.iter().find(|det| det.cajtpv == caja.id()).map(|det| det.cajtpv.to_owned())} checked={art.detalles.iter().find(|det| det.cajtpv == caja.id()).is_some()} />
                            <label for={caja.id()}>{caja.nombre()}</label>
                        </>
                    }).collect::<Html>()
                }
                </div>
                <div class="form-group">
                    <label for="detalles"></label>
                    <input type="text" id="detalles" class="form-control" placeholder="TODO: DETALLES" />
                </div>
            </div>
            <div class="modal-footer border-top-0 d-flex justify-content-center">
                <button type="submit" class="btn btn-success">{"Submit"}</button>
            </div>
        </form>
    }
}
