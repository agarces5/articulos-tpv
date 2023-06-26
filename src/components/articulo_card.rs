use yew::prelude::*;

use crate::{
    components::card::Card,
    context::modifications::{Modification, ModificationContext},
    models::articulo::Articulo,
};

#[derive(Debug, Clone, Default, Properties, PartialEq)]
pub struct ArticuloCardProps {
    pub articulo: Articulo,
}

#[function_component(ArticuloCard)]
pub fn articulo_card(props: &ArticuloCardProps) -> Html {
    // Traemos el contexto
    let modif = use_context::<ModificationContext>().unwrap();
    let articulo = props.articulo.to_owned();
    let callback_article = articulo.clone();
    let onclick = {
        move |_: MouseEvent| {
            let article = callback_article.clone();
            let mut articles = modif.mod_articles.clone();
            articles.entry(article.articulo).or_insert(article);
            modif.dispatch(Modification::new(articles))
        }
    };
    html! {
            <Card {articulo} {onclick} />
    }
}
