use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class={"nav"}>
            <div class={"card"}>{"Base de datos"}</div>
            <div class={"card"}>{"Login"}</div>
        </nav>
    }
}
