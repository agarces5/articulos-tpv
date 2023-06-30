use yew::prelude::*;

use crate::{
    components::articulo_card::ArticuloCard,
    context::filters::{Filter, FilterContext},
    hooks::use_articles::use_articles,
    models::articulo::ListArticulo,
};

#[function_component(ArticleList)]
pub fn article_list() -> Html {
    let articles: ListArticulo = use_articles();
    // Get filters
    let filter_context = use_context::<FilterContext>()
        .expect("Se esta intentando acceder al contexto de Filtros fuera del Provider");
    let filters = Filter::new(
        filter_context.cajtpv.to_owned(),
        filter_context.familia.to_owned(),
        filter_context.panel.to_owned(),
    );
    // Filter articles
    let articles = articles.filter(filters);

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
        <section class={"art-list grid-container"}>
            { articles }
        </section>
    }
}
