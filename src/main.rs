use dioxus::prelude::*;

mod models;
mod data;
mod components;
mod utils;

use components::{Navbar, Home, BoxScoreDisplay};
use data::get_mock_match;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/match/:id")]
    MatchDetail { id: u32 },
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
fn MatchDetail(id: u32) -> Element {
    let match_data = get_mock_match();
    rsx! {
        div {
            class: "min-h-screen bg-gray-50 py-8",
            div {
                class: "bg-orange-700 text-white py-6 shadow-lg mb-8",
                h1 {
                    class: "text-4xl font-bold text-center",
                    "🏀 NBA Box Score - Match #{id}"
                }
            }
            div {
                class: "container mx-auto px-4",
                BoxScoreDisplay { match_data }
            }
        }
    }
}

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-gray-50 py-8",
            div {
                class: "bg-orange-700 text-white py-6 shadow-lg mb-8",
                h1 {
                    class: "text-4xl font-bold text-center",
                    "📝 Blog Post #{id}"
                }
            }
            div {
                class: "container mx-auto px-4 max-w-3xl",
                div {
                    class: "bg-white rounded-lg shadow-lg p-8",
                    p { class: "text-gray-600 text-lg", "Contenu du blog post #{id}..." }
                    Link {
                        to: Route::Home {},
                        class: "mt-6 inline-block bg-orange-600 text-white px-6 py-2 rounded-lg hover:bg-orange-700 transition-colors",
                        "← Retour à l'accueil"
                    }
                }
            }
        }
    }
}