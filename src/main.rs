use console_error_panic_hook::set_once as set_panic_hook;
use yew_training_project::App;

fn main() {
    set_panic_hook();
    yew::Renderer::<App>::new().render();

}
