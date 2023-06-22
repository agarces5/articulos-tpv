use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub articulo: usize,
    pub nombre: String,
    pub familia: String,
    pub tpvs: Vec<Cajtpv>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cajtpv {
    pub cajtpv: String,
    pub precios: Vec<Precio>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Precio {
    pub tipotarifa: String,
    pub precio: i32,
}

#[function_component(ArticleList)]
pub fn article_list() -> Html {
    let articles = include_str!("../mocks/articulos.json");

    // gloo::console::log!("{}", articles);

    let articles =
        serde_json::from_str::<Vec<Article>>(articles).expect("Unable to parse articulos.json");
    let articles_render = articles.iter().map(|article| {
        html! {
            <div key={article.articulo} class={"articulo card"}>{&article.nombre}</div>
        }
    });

    html! {
        <section class={"art-list grid-container"}>
            { articles_render.into_iter().collect::<Html>() }
            <div class={"articulo card"}>{"articulo 1"}</div>
            <div class={"articulo card"}>{"articulo 2"}</div>
            <div class={"articulo card"}>{"articulo 3"}</div>
            <div class={"articulo card"}>{"articulo 4"}</div>
            <div class={"articulo card"}>{"articulo 5"}</div>
            <div class={"articulo card"}>{"articulo 6"}</div>
            <div class={"articulo card"}>{"articulo 7"}</div>
            <div class={"articulo card"}>{"articulo 8"}</div>
            <div class={"articulo card"}>{"articulo 9"}</div>
            <div class={"articulo card"}>{"articulo 10"}</div>
            <div class={"articulo card"}>{"articulo 11"}</div>
            <div class={"articulo card"}>{"articulo 12"}</div>
            <div class={"articulo card"}>{"articulo 13"}</div>
            <div class={"articulo card"}>{"articulo 14"}</div>
            <div class={"articulo card"}>{"articulo 15"}</div>
            <div class={"articulo card"}>{"articulo 16"}</div>
            <div class={"articulo card"}>{"articulo 17"}</div>
            <div class={"articulo card"}>{"articulo 18"}</div>
            <div class={"articulo card"}>{"articulo 19"}</div>
            <div class={"articulo card"}>{"articulo 20"}</div>
        </section>
    }
}
