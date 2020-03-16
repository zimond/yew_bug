use yew::prelude::*;

use crate::card::Card;
use crate::custom::Custom;
use crate::tabs::Tabs;

pub struct Wrapper {
    timeout: yew::services::TimeoutService,
    task: yew::services::timeout::TimeoutTask,
}

pub enum Msg {
    Update,
}

impl Component for Wrapper {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut timeout = yew::services::TimeoutService::new();
        let task = timeout.spawn(
            std::time::Duration::from_secs(1),
            link.callback(|_| Msg::Update),
        );
        Wrapper { timeout, task }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        log::info!("render sims");
        html! {
        <Tabs>
            <div>
                <Card title="above header"/>
                <h1>{"tab 1"}</h1>
            </div>
            <div>{"dumb"}</div>
            <div>{"dumb"}</div>
            <div>
                <Custom content="tab 2"/>
                {html!{}}
            </div>
        </Tabs>
        }
    }
}
