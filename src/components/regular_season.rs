use dioxus::prelude::*;
use crate::data::get_regular_season_games;

#[component]
pub fn RegularSeason() -> Element {
    let games = get_regular_season_games();
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
                        "Saison Régulière 2025-2026"
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
                        div { class: "px-4 py-3 text-gray-400 hover:text-white cursor-pointer", "CLASSEMENT" }
                        div { class: "px-4 py-3 text-gray-400 hover:text-white cursor-pointer", "STATS" }
                        div { class: "px-4 py-3 text-gray-400 hover:text-white cursor-pointer", "JOUEURS" }
                    }
                }
            }
            
            // Filtres (aujourd'hui, demain, etc.)
            div {
                class: "container mx-auto px-4 py-4",
                div {
                    class: "flex gap-3",
                    div { class: "px-4 py-2 bg-orange-600 text-white rounded-full text-sm font-bold cursor-pointer", "AUJOURD'HUI" }
                    div { class: "px-4 py-2 bg-gray-700 text-gray-300 rounded-full text-sm font-bold cursor-pointer hover:bg-gray-600", "DÉMATIN" }
                    div { class: "px-4 py-2 bg-gray-700 text-gray-300 rounded-full text-sm font-bold cursor-pointer hover:bg-gray-600", "DATE" }
                }
            }
            
            // Statistiques rapides (optionnel)
            div {
                class: "container mx-auto px-4 mb-4",
                div {
                    class: "grid grid-cols-2 md:grid-cols-4 gap-4",
                    div {
                        class: "bg-gray-800 rounded-lg p-4 text-center",
                        p { class: "text-gray-400 text-sm", "Matchs aujourd'hui" }
                        p { class: "text-2xl font-bold text-white", "7" }
                    }
                    div {
                        class: "bg-gray-800 rounded-lg p-4 text-center",
                        p { class: "text-gray-400 text-sm", "Équipes en action" }
                        p { class: "text-2xl font-bold text-white", "14" }
                    }
                    div {
                        class: "bg-gray-800 rounded-lg p-4 text-center",
                        p { class: "text-gray-400 text-sm", "Points par match" }
                        p { class: "text-2xl font-bold text-white", "113.2" }
                    }
                    div {
                        class: "bg-gray-800 rounded-lg p-4 text-center",
                        p { class: "text-gray-400 text-sm", "En direct" }
                        p { class: "text-2xl font-bold text-red-500", "2" }
                    }
                }
            }
            
            // Contenu principal - 2 colonnes
            div {
                class: "container mx-auto px-4 py-4",
                div {
                    class: "grid grid-cols-1 lg:grid-cols-2 gap-8",
                    
                    // Colonne Conférence Est
                    div {
                        class: "bg-gray-800 rounded-xl overflow-hidden",
                        div {
                            class: "bg-gray-700 px-6 py-4",
                            div {
                                class: "flex justify-between items-center",
                                h2 {
                                    class: "text-xl font-bold text-white",
                                    "Conférence Est"
                                }
                                span { class: "text-orange-400 text-sm", "7 matchs" }
                            }
                            p { class: "text-gray-400 text-sm mt-1", "Eastern Conference" }
                        }
                        div {
                            class: "divide-y divide-gray-700",
                            for game in east_games {
                                div {
                                    class: "p-4 hover:bg-gray-750 transition-colors",
                                    
                                    // Entête du match
                                    div {
                                        class: "flex justify-between items-start",
                                        div {
                                            class: "flex-1",
                                            div {
                                                class: "flex justify-between items-center mb-2",
                                                div {
                                                    class: "flex items-center gap-3",
                                                    span { class: "text-gray-400 text-sm", "vs" }
                                                    span { class: "font-bold text-white text-lg", "{game.home_team}" }
                                                }
                                                if let Some(score) = game.home_score {
                                                    span { class: "text-2xl font-bold text-white", "{score}" }
                                                } else if game.status == "Demain" {
                                                    span { class: "text-orange-500 font-bold", "Demain" }
                                                } else if game.is_live {
                                                    span { class: "text-red-500 font-bold text-sm animate-pulse", "LIVE" }
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
                                                } else if game.is_live {
                                                    span { class: "text-gray-400 text-sm", "{game.time}" }
                                                }
                                            }
                                        }
                                    }
                                    
                                    // Footer du match
                                    div {
                                        class: "mt-3 pt-3 border-t border-gray-700 flex justify-between items-center",
                                        if game.is_live {
                                            div {
                                                class: "flex items-center gap-2",
                                                span { class: "w-2 h-2 bg-red-500 rounded-full animate-pulse" }
                                                span { class: "text-red-500 text-sm font-bold", "EN DIRECT" }
                                            }
                                            span { class: "text-orange-400 text-sm cursor-pointer hover:underline", "APERÇU LIVE →" }
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
                            div {
                                class: "flex justify-between items-center",
                                h2 {
                                    class: "text-xl font-bold text-white",
                                    "Conférence Ouest"
                                }
                                span { class: "text-orange-400 text-sm", "8 matchs" }
                            }
                            p { class: "text-gray-400 text-sm mt-1", "Western Conference" }
                        }
                        div {
                            class: "divide-y divide-gray-700",
                            for game in west_games {
                                div {
                                    class: "p-4 hover:bg-gray-750 transition-colors",
                                    
                                    // Entête du match
                                    div {
                                        class: "flex justify-between items-start",
                                        div {
                                            class: "flex-1",
                                            div {
                                                class: "flex justify-between items-center mb-2",
                                                div {
                                                    class: "flex items-center gap-3",
                                                    span { class: "text-gray-400 text-sm", "vs" }
                                                    span { class: "font-bold text-white text-lg", "{game.home_team}" }
                                                }
                                                if let Some(score) = game.home_score {
                                                    span { class: "text-2xl font-bold text-white", "{score}" }
                                                } else if game.status == "Demain" {
                                                    span { class: "text-orange-500 font-bold", "Demain" }
                                                } else if game.is_live {
                                                    span { class: "text-red-500 font-bold text-sm animate-pulse", "LIVE" }
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
                                                } else if game.is_live {
                                                    span { class: "text-gray-400 text-sm", "{game.time}" }
                                                }
                                            }
                                        }
                                    }
                                    
                                    // Footer du match
                                    div {
                                        class: "mt-3 pt-3 border-t border-gray-700 flex justify-between items-center",
                                        if game.is_live {
                                            div {
                                                class: "flex items-center gap-2",
                                                span { class: "w-2 h-2 bg-red-500 rounded-full animate-pulse" }
                                                span { class: "text-red-500 text-sm font-bold", "EN DIRECT" }
                                            }
                                            span { class: "text-orange-400 text-sm cursor-pointer hover:underline", "APERÇU LIVE →" }
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
                p { "© 2026 NBA Media Ventures, LLC. Tous droits réservés." }
                div {
                    class: "flex justify-center gap-6 mt-2",
                    span { class: "hover:text-orange-400 cursor-pointer", "Conditions d'utilisation" }
                    span { class: "hover:text-orange-400 cursor-pointer", "Politique de confidentialité" }
                    span { class: "hover:text-orange-400 cursor-pointer", "Préférences cookies" }
                }
            }
        }
    }
}