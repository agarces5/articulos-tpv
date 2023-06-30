use crate::context::filters::{Filter, FilterContext};
use crate::models::articulo::{Articulo, ListArticulo};
use crate::models::articulo_dto::{ArticuloDTO, ListArticuloDTO};
use yew::hook;

#[hook]
pub fn use_articles() -> Vec<Articulo> {
    // Get the articles
    let articles = include_str!("../mocks/articulos_input.json");
    let articles: Vec<ArticuloDTO> =
        serde_json::from_str(articles).expect("Unable to parse articulos.json");
    let articles: ListArticulo = ListArticuloDTO::new(articles).into();
    // Get filters
    let filter_context = yew::use_context::<FilterContext>()
        .expect("Se esta intentando acceder al contexto de Filtros fuera del Provider");
    let filters = Filter::new(
        filter_context.cajtpv.to_owned(),
        filter_context.familia.to_owned(),
        filter_context.panel.to_owned(),
    );
    // Filter articles
    articles.filter(filters)
}
