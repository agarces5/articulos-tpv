use crate::context::filters::{Filter, FilterContext};
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[function_component(Controls)]
pub fn controls() -> Html {
    let filter_context = use_context::<FilterContext>()
        .expect("Se esta intentando acceder al contexto de Filtros fuera del Provider");

    let onchange = {
        Callback::from(move |e: Event| {
            let input_element = e.target_dyn_into::<HtmlSelectElement>();
            if let Some(input) = input_element {
                let filter = Filter::new(
                    input.value(),
                    filter_context.familia.to_owned(),
                    filter_context.panel.to_owned(),
                );
                filter_context.dispatch(filter)
            }
        })
    };
    html! {
        <section class={"controls"}>
            <h2>{"Controles"}</h2>
            <div class={"css-controles"}>
                <div>
                    <label htmlfor={"input-cajas"}>{"Cajtpv: "}</label>
                    <select id={"input-cajas"} {onchange} >
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
