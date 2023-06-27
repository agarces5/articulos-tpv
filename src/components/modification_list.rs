use yew::prelude::*;

use crate::context::modifications::ModificationContext;

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
            let precio = art
                .some_detail()
                .unwrap_or_default()
                .precios
                .first()
                .cloned()
                .unwrap_or_default()
                .precio;
            html! {
                <button key={art.articulo} class={"articulo card"} {onclick}>
                    <h4>{&art.nombre}</h4>
                    <p>{art.articulo}</p>
                    <p>{format!("{} €",precio) }</p>
                </button>
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
