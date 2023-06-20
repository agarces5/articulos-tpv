use yew::prelude::*;

use crate::components::{
    articles_list::ArticleList, controls::Controls, families::Families,
    modification_list::ModificationList, navbar::Navbar, panels::Panels,
};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class={"grid"}>
            <Navbar />
            <ModificationList />
            <Panels />
            <Families />
            <ArticleList />
            <Controls />
        </div>
    }
}
