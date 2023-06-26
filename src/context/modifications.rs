use std::{collections::HashMap, rc::Rc};
use yew::prelude::*;

use crate::models::articulo::Articulo;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Modification {
    pub mod_articles: HashMap<u32, Articulo>,
}

impl Modification {
    pub fn new(mod_articles: HashMap<u32, Articulo>) -> Self {
        Self { mod_articles }
    }
}

impl Reducible for Modification {
    type Action = Modification;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Modification { ..action }.into()
    }
}

pub type ModificationContext = UseReducerHandle<Modification>;

#[derive(Properties, Debug, PartialEq)]
pub struct ModificationProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ModificationContextProvider)]
pub fn modification_context_provider(props: &ModificationProviderProps) -> Html {
    let modification = use_reducer(Modification::default);

    html! {
        <ContextProvider<ModificationContext> context={modification}>
            {props.children.clone()}
        </ContextProvider<ModificationContext>>
    }
}
