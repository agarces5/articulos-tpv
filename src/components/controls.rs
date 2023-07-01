use crate::{
    context::filters::{Filter, FilterContext},
    hooks::use_cajtpv::use_cajtpv,
};
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[function_component(Controls)]
pub fn controls() -> Html {
    let filter_ctx = use_context::<FilterContext>()
        .expect("Se esta intentando acceder al contexto de Filtros fuera del Provider");
    // Get cajtpv list
    let cajtpv_list = use_cajtpv();

    html! {
        <section class={"controls css-controles"}>
            <div>
                <label htmlfor={"input-cajas"}>{"Cajtpv: "}</label>
                <select id={"input-cajas"} onchange={handle_change(filter_ctx)} >
                    <option value={"all"}>{"Todas"}</option>
                    {
                        cajtpv_list.iter().map(|cajtpv| {
                            html! {<option value={cajtpv.id()}>{format!("{} - {}", cajtpv.id(), cajtpv.nombre())}</option>}
                        }).collect::<Html>()
                    }
                </select>
            </div>
            <div style={"display:flex;gap:1rem"} >
                <button class={"control-button"} >{"SCRIPT"}</button>
                <button class={"control-button"} >{"EJECUTAR"}</button>
            </div>
        </section>
    }
}

fn handle_change(filter_ctx: UseReducerHandle<Filter>) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let input_element = e.target_dyn_into::<HtmlSelectElement>();
        if let Some(input) = input_element {
            let filter = Filter::new(
                input.value(),
                filter_ctx.familia.to_owned(),
                filter_ctx.panel.to_owned(),
            );
            filter_ctx.dispatch(filter)
        }
    })
}
