use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ModalProps {
    pub _ref: NodeRef,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    html! {

    <div id="myModal" class="modal" ref={props._ref.clone()}>

      <div class="modal-content">
        <span class="close">{"&times;"}</span>
        <p>{"Some text in the Modal.."}</p>
      </div>

    </div>

    }
}
