mod components;

use components::about::About;
use components::education::Education;
use components::experience::Experience;
use components::footer::Footer;
use components::hero::Hero;
use components::navbar::Navbar;
use components::skills::Skills;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Chakra+Petch:wght@400;600;700&family=Barlow:wght@300;400;500;600&family=Share+Tech+Mono&display=swap"
        }
        Router::<Route> {}
    }
}

/// Home page - single page portfolio
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}
        About {}
        Skills {}
        Experience {}
        Education {}
        Footer {}
    }
}
