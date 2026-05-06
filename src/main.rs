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


// ========== DONNÉES MOCK ==========

fn get_mock_match() -> Match {
    // Lakers (domicile)
    let lakers = Team {
        name: String::from("Lakers"),
        city: String::from("Los Angeles"),
        logo: String::from("💜"),
    };
    
    let lakers_players = vec![
        PlayerStats {
            name: String::from("LeBron James"),
            number: 23,
            position: String::from("SF"),
            minutes: String::from("36:24"),
            points: 28,
            rebounds: 8,
            assists: 10,
            steals: 2,
            blocks: 1,
            turnovers: 3,
            fouls: 2,
            field_goals: (10, 19),
            three_points: (2, 5),
            free_throws: (6, 8),
        },
        PlayerStats {
            name: String::from("Anthony Davis"),
            number: 3,
            position: String::from("PF"),
            minutes: String::from("34:12"),
            points: 24,
            rebounds: 12,
            assists: 3,
            steals: 1,
            blocks: 4,
            turnovers: 2,
            fouls: 3,
            field_goals: (9, 16),
            three_points: (1, 3),
            free_throws: (5, 6),
        },
        PlayerStats {
            name: String::from("D'Angelo Russell"),
            number: 1,
            position: String::from("PG"),
            minutes: String::from("32:08"),
            points: 18,
            rebounds: 3,
            assists: 7,
            steals: 1,
            blocks: 0,
            turnovers: 2,
            fouls: 1,
            field_goals: (6, 14),
            three_points: (3, 8),
            free_throws: (3, 4),
        },
    ];
    
    let lakers_box = TeamBoxScore {
        team: lakers,
        players: lakers_players,
        total_points: 112,
        quarter_scores: [28, 30, 26, 28],
    };
    
    // Warriors (extérieur)
    let warriors = Team {
        name: String::from("Warriors"),
        city: String::from("Golden State"),
        logo: String::from("🔵"),
    };
    
    let warriors_players = vec![
        PlayerStats {
            name: String::from("Stephen Curry"),
            number: 30,
            position: String::from("PG"),
            minutes: String::from("35:42"),
            points: 32,
            rebounds: 5,
            assists: 6,
            steals: 2,
            blocks: 0,
            turnovers: 4,
            fouls: 2,
            field_goals: (11, 24),
            three_points: (6, 14),
            free_throws: (4, 4),
        },
        PlayerStats {
            name: String::from("Klay Thompson"),
            number: 11,
            position: String::from("SG"),
            minutes: String::from("33:15"),
            points: 22,
            rebounds: 4,
            assists: 2,
            steals: 1,
            blocks: 1,
            turnovers: 1,
            fouls: 2,
            field_goals: (8, 18),
            three_points: (4, 10),
            free_throws: (2, 2),
        },
    ];
    
    let warriors_box = TeamBoxScore {
        team: warriors,
        players: warriors_players,
        total_points: 108,
        quarter_scores: [25, 28, 30, 25],
    };
    
    Match {
        id: 1,
        home_team: lakers_box,
        away_team: warriors_box,
        date: String::from("2026-05-15"),
        arena: String::from("Crypto.com Arena"),
        attendance: 18997,
    }
}


// ========== COMPOSANTS D'AFFICHAGE ==========

#[component]
fn BoxScoreDisplay(match_data: Match) -> Element {
    rsx! {
        div {
            class: "max-w-6xl mx-auto",
            
            // Header du match
            div {
                class: "bg-white rounded-lg shadow-lg p-6 mb-6",
                div {
                    class: "flex justify-between items-center",
                    // Équipe extérieure (gauche)
                    div {
                        class: "text-center flex-1",
                        div {
                            class: "text-5xl mb-2",
                            "{match_data.away_team.team.logo}"
                        }
                        h3 {
                            class: "text-2xl font-bold text-gray-800",
                            "{match_data.away_team.team.city} {match_data.away_team.team.name}"
                        }
                        div {
                            class: "text-4xl font-bold text-orange-600 mt-2",
                            "{match_data.away_team.total_points}"
                        }
                    },
                    
                    // VS central
                    div {
                        class: "text-center px-8",
                        div {
                            class: "text-3xl font-bold text-gray-500",
                            "VS"
                        }
                        div {
                            class: "text-sm text-gray-500 mt-2",
                            "{match_data.date}"
                        }
                    },
                    
                    // Équipe domicile (droite)
                    div {
                        class: "text-center flex-1",
                        div {
                            class: "text-5xl mb-2",
                            "{match_data.home_team.team.logo}"
                        }
                        h3 {
                            class: "text-2xl font-bold text-gray-800",
                            "{match_data.home_team.team.city} {match_data.home_team.team.name}"
                        }
                        div {
                            class: "text-4xl font-bold text-orange-600 mt-2",
                            "{match_data.home_team.total_points}"
                        }
                    }
                },
                
                // Infos match
                div {
                    class: "text-center mt-4 pt-4 border-t border-gray-200 text-gray-600",
                    p {
                        "📍 {match_data.arena} • 👥 {match_data.attendance} spectateurs"
                    }
                }
            }
            
            // Scores par quart-temps
            div {
                class: "bg-white rounded-lg shadow-lg p-6 mb-6 overflow-x-auto",
                h4 {
                    class: "text-xl font-bold text-gray-800 mb-4",
                    "Scores par quart-temps"
                }
                table {
                    class: "w-full border-collapse",
                    thead {
                        class: "bg-gray-100",
                        tr {
                            th { class: "border p-3 text-left", "" }
                            th { class: "border p-3 text-center", "Q1" }
                            th { class: "border p-3 text-center", "Q2" }
                            th { class: "border p-3 text-center", "Q3" }
                            th { class: "border p-3 text-center", "Q4" }
                            th { class: "border p-3 text-center", "TOTAL" }
                        }
                    }
                    tbody {
                        // Équipe extérieure
                        tr {
                            td { 
                                class: "border p-3 font-semibold",
                                "{match_data.away_team.team.logo} {match_data.away_team.team.city}"
                            }
                            for quarter in match_data.away_team.quarter_scores.iter() {
                                td { class: "border p-3 text-center", "{quarter}" }
                            }
                            td { 
                                class: "border p-3 text-center font-bold text-orange-600",
                                "{match_data.away_team.total_points}"
                            }
                        }
                        // Équipe domicile
                        tr {
                            td { 
                                class: "border p-3 font-semibold",
                                "{match_data.home_team.team.logo} {match_data.home_team.team.city}"
                            }
                            for quarter in match_data.home_team.quarter_scores.iter() {
                                td { class: "border p-3 text-center", "{quarter}" }
                            }
                            td { 
                                class: "border p-3 text-center font-bold text-orange-600",
                                "{match_data.home_team.total_points}"
                            }
                        }
                    }
                }
            }
            
            // Statistiques joueurs (à venir)
            div {
                class: "bg-white rounded-lg shadow-lg p-6",
                h4 {
                    class: "text-xl font-bold text-gray-800 mb-4",
                    "Statistiques individuelles"
                }
                div {
                    class: "text-center text-gray-500",
                    "(Tableau des stats à venir)"
                }
            }
        }
    }
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
    let match_data = get_mock_match();
    
    rsx! {
        div {
            class: "min-h-screen bg-orange-50 py-8",
            
            // Header style NBA
            div {
                class: "bg-orange-600 text-white py-6 shadow-lg mb-8",
                h1 {
                    class: "text-4xl font-bold text-center",
                    "🏀 NBA Box Score"
                }
            }
            
            // Box Score
            div {
                class: "container mx-auto px-4",
                BoxScoreDisplay { match_data: match_data }
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
