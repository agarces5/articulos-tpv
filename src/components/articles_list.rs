use yew::prelude::*;

use crate::{components::articulo_card::ArticuloCard, hooks::use_articles::use_articles};

#[function_component(ArticleList)]
pub fn article_list() -> Html {
    let articles = use_articles();

    // Render articles
    let articles = articles
        .iter()
        .map(move |article| {
            html! {
                <ArticuloCard articulo={article.clone()} />
            }
        })
        .collect::<Html>();

    html! {
        <section class={"art-list"}>
            { articles }
        </section>
    }
}
