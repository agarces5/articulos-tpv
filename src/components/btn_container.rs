use web_sys::HtmlDialogElement;
use yew::prelude::*;

use crate::components::{delete_icon::DeleteIcon, edit_icon::EditIcon};

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct BtnContainerProps {
    pub refe: NodeRef,
}
#[function_component(BtnContainer)]
pub fn btn_container(props: &BtnContainerProps) -> Html {
    html! {
        <div class={"btn-container"}>
            <button class={"svg-button"} onclick={handle_click_update(props.refe.clone())} >
                <EditIcon />
            </button>
            <button class={"svg-button"} onclick={handle_click_delete()}>
                <DeleteIcon />
            </button>
        </div>
    }
}
fn handle_click_update(_ref: NodeRef) -> Callback<MouseEvent> {
    Callback::from(move |_: MouseEvent| {
        let div = _ref.cast::<HtmlDialogElement>();
        if let Some(div) = div {
            // div.style().set_css_text("display: block;");
            div.show_modal()
                .unwrap_or_else(|e| gloo::console::error!(e));
            gloo::console::log!(div.open());
        } else {
            gloo::console::log!(div);
        }
    })
}
fn handle_click_delete() -> Callback<MouseEvent> {
    Callback::from(move |_: MouseEvent| {})
}
