use yew::prelude::*;

pub struct Card {
    children: Children,
    title: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub title: String,
}

impl Component for Card {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Card {
            children: props.children,
            title: props.title,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.children = props.children;
        self.title = props.title;
        true
    }

    fn view(&self) -> Html {
        log::info!("render card");
        html! {
            <div class="smash-card">
                <div class="smash-card-title">{self.title.as_str()}</div>
                <div class="smash-card-content">
                    {self.children.render()}
                </div>
            </div>
        }
    }
}
