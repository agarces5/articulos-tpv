use crate::context::filters::{Filter, FilterContext};
use web_sys::HtmlSelectElement;
use yew::prelude::*;

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

#[function_component(Controls)]
pub fn controls() -> Html {
    let filter_ctx = use_context::<FilterContext>()
        .expect("Se esta intentando acceder al contexto de Filtros fuera del Provider");

    html! {
        <section class={"controls"}>
            <h2>{"Controles"}</h2>
            <div class={"css-controles"}>
                <div>
                    <label htmlfor={"input-cajas"}>{"Cajtpv: "}</label>
                    <select id={"input-cajas"} onchange={handle_change(filter_ctx)} >
                        <option value={"all"}>{"Todas"}</option>
                        <option value={"0001"}>{"0001"}</option>
                        <option value={"0002"}>{"0002"}</option>
                        <option value={"0007"}>{"0007"}</option>
                    </select>
                </div>
                <div style={"display:flex;gap:1rem"} >
                    <button class={"control-button"} >{"SCRIPT"}</button>
                    <button class={"control-button"} >{"EJECUTAR"}</button>
                </div>
            </div>
        </section>
    }
}
