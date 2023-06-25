use yew::prelude::*;

use crate::models::{
    articulo::ListArticulo,
    articulo_dto::{ArticuloDTO, ListArticuloDTO},
};

#[function_component(ArticleList)]
pub fn article_list() -> Html {
    // Get the articles
    let articles = include_str!("../mocks/articulos_input.json");
    gloo::console::log!("{:?}", articles);
    let articles: Vec<ArticuloDTO> =
        serde_json::from_str(articles).expect("Unable to parse articulos.json");
    let articles: ListArticulo = ListArticuloDTO::new(articles).into();
    // Filter articles
    let articles = articles.filter("0001", "0101");

    // Render articles
    let articles_render = articles
        .iter()
        .map(|(_, article)| {
            html! {
                <div key={article.articulo} class={"articulo card"}>{&article.nombre}</div>
            }
        })
        .collect::<Html>();

    html! {
        <section class={"art-list grid-container"}>
            { articles_render }
        </section>
    }
}
