use dioxus::prelude::*;
use crate::data::get_games;

#[component]
pub fn Home() -> Element {
    let games = get_games();
    let east_games: Vec<_> = games.iter().filter(|g| g.conference == "East").collect();
    let west_games: Vec<_> = games.iter().filter(|g| g.conference == "West").collect();
    
    rsx! {
        div {
            class: "min-h-screen bg-gray-900",
            
            // Header
            div {
                class: "bg-gradient-to-r from-blue-900 to-gray-900 text-white py-8",
                div {
                    class: "container mx-auto px-4",
                    h1 {
                        class: "text-5xl font-black text-center mb-2",
                        "🏀 NBA"
                    }
                    p {
                        class: "text-center text-gray-400 text-lg",
                        "Demi-finales de Conférence"
                    }
                }
            }
            
            // Navigation tabs
            div {
                class: "border-b border-gray-700 bg-gray-800",
                div {
                    class: "container mx-auto px-4",
                    div {
                        class: "flex space-x-8",
                        div { class: "px-4 py-3 text-orange-500 border-b-2 border-orange-500 font-bold", "MATCHS" }
                        div { class: "px-4 py-3 text-gray-400 hover:text-white cursor-pointer", "PHASE FINALE" }
                        div { class: "px-4 py-3 text-gray-400 hover:text-white cursor-pointer", "CLASSEMENT" }
                        div { class: "px-4 py-3 text-gray-400 hover:text-white cursor-pointer", "STATS" }
                        div { class: "px-4 py-3 text-gray-400 hover:text-white cursor-pointer", "JOUEURS" }
                    }
                }
            }
            
            // Contenu principal - 2 colonnes
            div {
                class: "container mx-auto px-4 py-8",
                div {
                    class: "grid grid-cols-1 lg:grid-cols-2 gap-8",
                    
                    // Colonne Conférence Est
                    div {
                        class: "bg-gray-800 rounded-xl overflow-hidden",
                        div {
                            class: "bg-gray-700 px-6 py-4",
                            h2 {
                                class: "text-xl font-bold text-white",
                                "Conférence Est"
                            }
                        }
                        div {
                            class: "divide-y divide-gray-700",
                            for game in east_games {
                                div {
                                    class: "p-4 hover:bg-gray-750 transition-colors",
                                    
                                    div {
                                        class: "flex justify-between items-start",
                                        div {
                                            class: "flex-1",
                                            div {
                                                class: "flex justify-between items-center",
                                                div {
                                                    class: "flex items-center gap-3 mb-2",
                                                    span { class: "text-gray-400 text-sm", "vs" }
                                                    span { class: "font-bold text-white", "{game.home_team}" }
                                                }
                                                if let Some(score) = game.home_score {
                                                    span { class: "text-2xl font-bold text-white", "{score}" }
                                                } else if game.status == "Demain" {
                                                    span { class: "text-orange-500 font-bold", "Demain" }
                                                }
                                            }
                                            div {
                                                class: "flex justify-between items-center",
                                                div {
                                                    class: "flex items-center gap-3",
                                                    span { class: "text-gray-500", "{game.away_team}" }
                                                }
                                                if let Some(score) = game.away_score {
                                                    span { class: "text-2xl font-bold text-white", "{score}" }
                                                } else if game.status == "Demain" {
                                                    span { class: "text-orange-500 font-bold", "{game.time}" }
                                                }
                                            }
                                        }
                                    }
                                    
                                    div {
                                        class: "mt-3 pt-3 border-t border-gray-700 flex justify-between items-center",
                                        if game.is_live {
                                            span { class: "text-red-500 text-sm font-bold animate-pulse", "LIVE" }
                                        } else if game.status == "Terminé" {
                                            span { class: "text-gray-500 text-sm", "Terminé" }
                                            span { class: "text-orange-400 text-sm cursor-pointer hover:underline", "RÉCIT DU MATCH →" }
                                        } else {
                                            span { class: "text-gray-500 text-sm", "{game.time}" }
                                            span { class: "text-orange-400 text-sm cursor-pointer hover:underline", "APERÇU DU MATCH →" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    // Colonne Conférence Ouest
                    div {
                        class: "bg-gray-800 rounded-xl overflow-hidden",
                        div {
                            class: "bg-gray-700 px-6 py-4",
                            h2 {
                                class: "text-xl font-bold text-white",
                                "Conférence Ouest"
                            }
                        }
                        div {
                            class: "divide-y divide-gray-700",
                            for game in west_games {
                                div {
                                    class: "p-4 hover:bg-gray-750 transition-colors",
                                    
                                    div {
                                        class: "flex justify-between items-start",
                                        div {
                                            class: "flex-1",
                                            div {
                                                class: "flex justify-between items-center",
                                                div {
                                                    class: "flex items-center gap-3 mb-2",
                                                    span { class: "text-gray-400 text-sm", "vs" }
                                                    span { class: "font-bold text-white", "{game.home_team}" }
                                                }
                                                if let Some(score) = game.home_score {
                                                    span { class: "text-2xl font-bold text-white", "{score}" }
                                                } else if game.status == "Demain" {
                                                    span { class: "text-orange-500 font-bold", "Demain" }
                                                }
                                            }
                                            div {
                                                class: "flex justify-between items-center",
                                                div {
                                                    class: "flex items-center gap-3",
                                                    span { class: "text-gray-500", "{game.away_team}" }
                                                }
                                                if let Some(score) = game.away_score {
                                                    span { class: "text-2xl font-bold text-white", "{score}" }
                                                } else if game.status == "Demain" {
                                                    span { class: "text-orange-500 font-bold", "{game.time}" }
                                                }
                                            }
                                        }
                                    }
                                    
                                    div {
                                        class: "mt-3 pt-3 border-t border-gray-700 flex justify-between items-center",
                                        if game.is_live {
                                            span { class: "text-red-500 text-sm font-bold animate-pulse", "LIVE" }
                                        } else if game.status == "Terminé" {
                                            span { class: "text-gray-500 text-sm", "Terminé" }
                                            span { class: "text-orange-400 text-sm cursor-pointer hover:underline", "RÉCIT DU MATCH →" }
                                        } else {
                                            span { class: "text-gray-500 text-sm", "{game.time}" }
                                            span { class: "text-orange-400 text-sm cursor-pointer hover:underline", "APERÇU DU MATCH →" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // Footer
            div {
                class: "text-center py-6 text-gray-500 text-sm border-t border-gray-800 mt-8",
                p { "Heure locale à Paris" }
                p { class: "mt-2", 
                    span { class: "text-orange-400 cursor-pointer hover:underline", "Voir plus" }
                    span { " →" }
                }
            }
        }
    }
}