#![recursion_limit = "256"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod card;
mod custom;
mod tabs;
mod wrapper;

use wrapper::Wrapper;

pub struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        log::info!("render smash");
        html! {<Wrapper/>}
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    yew::App::<Model>::new().mount(yew::utils::document().get_element_by_id("stage").unwrap());

    Ok(())
}
