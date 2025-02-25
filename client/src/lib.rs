mod admin;
mod buzzer;
mod spectator;

use admin::Admin;
use buzzer::Buzzer;
use spectator::Spectator;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/admin")]
    Admin,
    #[at("/buzzer")]
    Buzzer,
    #[at("/spectator")]
    Spectator,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Admin => html! { <Admin/> },
        Route::Buzzer => html! { <Buzzer/> },
        Route::Spectator => html! { <Spectator/> },
    }
}

#[function_component]
fn Main() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}

#[wasm_bindgen(start)]
fn main() {
    yew::Renderer::<Main>::new().render();
}
