use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Custom {
    pub content: String,
}

impl Component for Custom {
    type Message = ();
    type Properties = Self;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Custom {
            content: props.content,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        self.content = _props.content;
        true
    }

    fn view(&self) -> Html {
        log::info!("render noti");
        html! {<div class="custom"><div>{&self.content}</div></div>}
    }
}
