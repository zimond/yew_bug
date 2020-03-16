use yew::prelude::*;

pub struct Tabs {
    link: ComponentLink<Self>,
    selected: usize,
    children: Children,
    onchange: Callback<usize>,
}

pub enum Msg {
    IndexChanged(usize),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    #[prop_or_else(Callback::noop)]
    pub onchange: Callback<usize>,
}

impl Component for Tabs {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Tabs {
            selected: 0,
            link,
            children: props.children,
            onchange: props.onchange,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let Msg::IndexChanged(index) = msg;
        self.selected = index;
        self.onchange.emit(self.selected);
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.children = props.children;
        self.onchange = props.onchange;
        true
    }

    fn view(&self) -> Html {
        log::info!("rende tabs");
        html! {
            <div class="tabs">
            <div class="tab-buttons">
                {for (0..4).map(|index| {
                    html!{<>
                        <input style="display: none;" checked={self.selected == index} id={index} type="radio"/>
                        <label style="width: 50px;border:1px solid grey;display: inline-block;" for={index} onclick=self.link.callback(move |_| Msg::IndexChanged(index))>{index}</label>
                    </>}
                })}
            </div>
            <div class="tab-stack">{ self.children.iter().nth(self.selected).unwrap_or_else(|| html!{}) }</div>
        </div>}
    }
}
