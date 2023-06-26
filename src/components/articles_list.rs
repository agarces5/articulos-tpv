use yew::prelude::*;

use crate::{
    components::articulo_card::ArticuloCard,
    context::filters::{Filter, FilterContext},
    models::{
        articulo::ListArticulo,
        articulo_dto::{ArticuloDTO, ListArticuloDTO},
    },
};

#[function_component(ArticleList)]
pub fn article_list() -> Html {
    // Get the articles
    let articles = include_str!("../mocks/articulos_input.json");
    let articles: Vec<ArticuloDTO> =
        serde_json::from_str(articles).expect("Unable to parse articulos.json");
    let articles: ListArticulo = ListArticuloDTO::new(articles).into();
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
