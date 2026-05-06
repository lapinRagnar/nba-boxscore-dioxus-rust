use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");


// ========== STRUCTURES DE DONNÉES NBA ==========

#[derive(Debug, Clone, PartialEq)]
struct Team {
    name: String,
    city: String,
    logo: String, // emoji ou chemin d'image
}

#[derive(Debug, Clone, PartialEq)]
struct PlayerStats {
    name: String,
    number: u8,
    position: String, // "PG", "SG", "SF", "PF", "C"
    minutes: String,
    points: u16,
    rebounds: u8,
    assists: u8,
    steals: u8,
    blocks: u8,
    turnovers: u8,
    fouls: u8,
    field_goals: (u8, u8),      // (réussis, tentés)
    three_points: (u8, u8),     // (réussis, tentés)
    free_throws: (u8, u8),      // (réussis, tentés)
}

#[derive(Debug, Clone, PartialEq)]
struct TeamBoxScore {
    team: Team,
    players: Vec<PlayerStats>,
    total_points: u16,
    quarter_scores: [u8; 4], // Q1, Q2, Q3, Q4
}

#[derive(Debug, Clone, PartialEq)]
struct Match {
    id: u32,
    home_team: TeamBoxScore,
    away_team: TeamBoxScore,
    date: String,
    arena: String,
    attendance: u32,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.7/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-orange-50",
            
            // Header style NBA
            div {
                class: "bg-orange-600 text-white py-6 shadow-lg",
                h1 {
                    class: "text-4xl font-bold text-center",
                    "🏀 NBA Box Score"
                }
            }
            
            // Contenu principal
            div {
                class: "container mx-auto px-4 py-12",
                div {
                    class: "max-w-2xl mx-auto bg-white rounded-lg shadow-xl p-8",
                    h2 {
                        class: "text-3xl font-bold text-gray-800 mb-4",
                        "Hello World!"
                    }
                    p {
                        class: "text-gray-600 text-lg",
                        "Prêt à créer le box score NBA parfait avec Dioxus!"
                    }
                }
            }
        }
    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Blog { id: 1 },
                "Blog"
            }
        }

        Outlet::<Route> {}
    }
}

/// Echo component that demonstrates fullstack server functions.
#[component]
fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        div {
            id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                oninput:  move |event| async move {
                    let data = echo_server(event.value()).await.unwrap();
                    response.set(data);
                },
            }

            if !response().is_empty() {
                p {
                    "Server echoed: "
                    i { "{response}" }
                }
            }
        }
    }
}

/// Echo the user input on the server.
#[post("/api/echo")]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
