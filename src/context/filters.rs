use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Filter {
    pub cajtpv: String,
    pub familia: String,
    pub panel: String,
}

impl Filter {
    pub fn new(cajtpv: String, familia: String, panel: String) -> Self {
        Self {
            cajtpv,
            familia,
            panel,
        }
    }
}

impl Reducible for Filter {
    type Action = Filter;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Filter { ..action }.into()
    }
}

pub type FilterContext = UseReducerHandle<Filter>;

#[derive(Properties, Debug, PartialEq)]
pub struct FilterProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(FilterContextProvider)]
pub fn filter_context_provider(props: &FilterProviderProps) -> Html {
    let filter = use_reducer(|| Filter::new("all".to_string(), "0104".to_string(), "".to_string()));

    html! {
        <ContextProvider<FilterContext> context={filter}>
            {props.children.clone()}
        </ContextProvider<FilterContext>>
    }
}
