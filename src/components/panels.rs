use yew::prelude::*;

#[function_component(Panels)]
pub fn panels() -> Html {
    html! {
        <section class={"panels flex-container"}>
            <h2>{"Paneles"}</h2>
            <div class={"panel card"}>{"PANEL 1"}</div>
            <div class={"panel card"}>{"PANEL 2"}</div>
            <div class={"panel card"}>{"PANEL 3"}</div>
            <div class={"panel card"}>{"PANEL 4"}</div>
            <div class={"panel card"}>{"PANEL 5"}</div>
        </section>
    }
}
