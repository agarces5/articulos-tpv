use yew::prelude::*;

use crate::{components::card::Card, context::modifications::ModificationContext};

#[function_component(ModificationList)]
pub fn modification_list() -> Html {
    let modification_ctx = use_context::<ModificationContext>()
        .expect("Se esta intentando acceder al contexto de Modificaciones fuera del Provider");
    let modified_articles = modification_ctx.mod_articles.to_owned();

    // Render
    let render_mod = modified_articles
        .values()
        .map(|art| {
            let onclick = Callback::from(|_| {});
            html! {
                <Card articulo={art.clone()} {onclick} />
            }
        })
        .collect::<Html>();

    html! {
        <section class={"mod-list flex-container"}>
            <h2>{"Modificaciones"}</h2>
            {render_mod}
        </section>
    }
}
