use yew::prelude::*;

#[function_component(ModificationList)]
pub fn modification_list() -> Html {
    html! {
        <section class={"mod-list flex-container"}>
            <h2>{"Modificaciones"}</h2>
            <div class={"articulo card"}>{"Modificacion 1"}</div>
            <div class={"articulo card"}>{"Modificacion 2"}</div>
            <div class={"articulo card"}>{"Modificacion 3"}</div>
            <div class={"articulo card"}>{"Modificacion 4"}</div>
            <div class={"articulo card"}>{"Modificacion 5"}</div>
        </section>
    }
}
