use yew::prelude::*;

pub type AppRenderer = yew::Renderer<App>;

#[function_component]
pub fn App() -> Html {
    html! {
        <p>{ "Hello, world!" }</p>
    }
}
