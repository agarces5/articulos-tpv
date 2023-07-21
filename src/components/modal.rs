use web_sys::HtmlDialogElement;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ModalProps {
    pub refe: NodeRef,
    pub children: Children,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let modal_ref = props.refe.clone();
    let children = props.children.to_owned();
    let onclick = { move |_| modal_ref.cast::<HtmlDialogElement>().unwrap().close() };
    let close_icon = '\u{00D7}';
    html! {
        <dialog class={"modal"} ref={props.refe.clone()}>
            <div class={"modal-content"}>
                <button class={"close"} {onclick}>{close_icon}</button>
                {children}
            </div>
        </dialog>
    }
}
