use yew::prelude::*;

#[derive(Properties, PartialEq, )]
pub struct MainTitleProps {
  pub title: String,
  #[prop_or("".to_string())]
  pub class: String
}

#[function_component(MainTitle)]
pub fn main_title(props: &MainTitleProps) -> Html {

  html! {
    <h1 class={&props.class} >{ &props.title }</h1>
  }
}
