use crate::models::articulo::ListArticulo;
use crate::models::articulo_dto::{ArticuloDTO, ListArticuloDTO};
use yew::hook;

#[hook]
pub fn use_articles() -> ListArticulo {
    // Get the articles
    let articles = include_str!("../mocks/articulos_input.json");
    let articles: Vec<ArticuloDTO> =
        serde_json::from_str(articles).expect("Unable to parse articulos.json");
    ListArticuloDTO::new(articles).into()
}
