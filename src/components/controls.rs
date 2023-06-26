use crate::context::filters::{Filter, FilterContext};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(Controls)]
pub fn controls() -> Html {
    let input_state = use_state(String::default);
    let input_ref = use_node_ref();
    let filter_context = use_context::<FilterContext>()
        .expect("Se esta intentando acceder al contexto de Filtros fuera del Provider");

    let input = input_ref.clone();
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let input_value = input.cast::<HtmlInputElement>();
        let input_value = match input_value {
            Some(input) => input.value(),
            None => String::default(),
        };

        let filter = Filter::new(
            input_value,
            filter_context.familia.to_owned(),
            filter_context.panel.to_owned(),
        );
        filter_context.dispatch(filter);
    });
    let onchange = {
        let input_state = input_state.clone();
        Callback::from(move |e: Event| {
            let input_element = e.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = input_element {
                input_state.set(input.value());
            }
        })
    };
    html! {
        <section class={"controls"}>
            <h2>{"Controles"}</h2>
            <div class={"css-controles"}>
                <form {onsubmit}>
                    <label>{"Cajtpv: "}</label>
                    <input type="text" placeholder="all, 0001 ..." value={input_state.to_string()} ref={input_ref} {onchange} />
                </form>
                <div style={"display:flex;gap:1rem"} >
                    <button class={"control-button"} >{"SCRIPT"}</button>
                    <button class={"control-button"} >{"EJECUTAR"}</button>
                </div>
            </div>
        </section>
    }
}
