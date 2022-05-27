// components for use in the app views

use yew::{function_component, html, use_state};
use chrono::Datelike;

// header component
#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <h1>{ "Webassembly Home Page" }</h1>
        </header>
    }
}

// content component
#[function_component(Content)]
pub fn content() -> Html {
    html! {
        <section>
            <h2>{ "(Written in pure Rust...no JS allowed)"}</h2>
        </section>
    }
}

// footer component
#[function_component(Footer)]
pub fn footer() -> Html {
    let time = use_state(|| chrono::Local::now().date() );
    html! {
        <footer>
            <p>{ "Copyright" } {" "} { time.year()} {" "} { "Jeffery D Mitchell All Rights Reserved" }</p>
        </footer>
    }
}