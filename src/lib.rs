use gloo::console::log;
use yew::prelude::*;

mod components;

use components::main_title::MainTitle;

#[function_component(App)]
pub fn app() -> Html {
  

    log!("App init");

    html! {
        <>
        <MainTitle title="Teste!!!!!" class="text-4xl text-center"/>

            <p> {"Text"} </p>
        

        </>
    }
}