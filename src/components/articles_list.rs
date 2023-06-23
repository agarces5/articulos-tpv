use yew::prelude::*;

use crate::models::{
    articulo::{ListArticulo, ListArticuloDTO},
    articulo_dto::ArticuloDTO,
};

#[function_component(ArticleList)]
pub fn article_list() -> Html {
    // Get the articles
    let articles = include_str!("../mocks/articulos_input.json");
    gloo::console::log!("{:?}", articles);
    let articles: Vec<ArticuloDTO> =
        serde_json::from_str(articles).expect("Unable to parse articulos.json");
    let articles: ListArticulo = ListArticuloDTO(articles).into();
    let articles = articles.0;

    // Render articles
    let articles_render = articles.iter().map(|(_, article)| {
        html! {
            <div key={article.articulo} class={"articulo card"}>{&article.nombre}</div>
        }
    });

    html! {
        <section class={"art-list grid-container"}>
            { articles_render.into_iter().collect::<Html>() }
        </section>
    }
}
