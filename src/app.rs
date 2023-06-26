use yew::prelude::*;

use crate::{
    components::{
        articles_list::ArticleList, controls::Controls, families::Families,
        modification_list::ModificationList, navbar::Navbar, panels::Panels,
    },
    context::{filters::FilterContextProvider, modifications::ModificationContextProvider},
};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <FilterContextProvider>
            <ModificationContextProvider>
                <div class={"grid"}>
                    <Navbar />
                    <ModificationList />
                    <Panels />
                    <Families />
                    <ArticleList />
                    <Controls />
                </div>
            </ModificationContextProvider>
        </FilterContextProvider>
    }
}
