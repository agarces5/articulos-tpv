use yew::prelude::*;

#[function_component(Families)]
pub fn families() -> Html {
    html! {
        <section class={"families grid-container"}>
            <div class={"family card"}>{"FAMILIA 1"}</div>
            <div class={"family card"}>{"FAMILIA 2"}</div>
            <div class={"family card"}>{"FAMILIA 3"}</div>
            <div class={"family card"}>{"FAMILIA 4"}</div>
            <div class={"family card"}>{"FAMILIA 5"}</div>
        </section>
    }
}
