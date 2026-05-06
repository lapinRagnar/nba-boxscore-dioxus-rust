use dioxus::prelude::*;
use crate::models::Match;
// use crate::utils::get_percentage;

#[component]
pub fn BoxScoreDisplay(match_data: Match) -> Element {
    rsx! {
        div {
            class: "max-w-6xl mx-auto",
            
            // Header du match
            div {
                class: "bg-white rounded-lg shadow-lg p-6 mb-6",
                div {
                    class: "flex justify-between items-center",
                    // Équipe extérieure
                    div {
                        class: "text-center flex-1",
                        div { class: "text-5xl mb-2", "{match_data.away_team.team.logo}" }
                        h3 {
                            style: "color: black; font-size: 24px; font-weight: bold;",
                            "{match_data.away_team.team.city} {match_data.away_team.team.name}"
                        }
                        div {
                            style: "color: #ea580c; font-size: 36px; font-weight: bold; margin-top: 8px;",
                            "{match_data.away_team.total_points}"
                        }
                    },
                    
                    // VS central
                    div {
                        class: "text-center px-8",
                        div {
                            style: "color: #6b7280; font-size: 30px; font-weight: bold;",
                            "VS"
                        }
                        div {
                            style: "color: #6b7280; font-size: 14px; margin-top: 8px;",
                            "{match_data.date}"
                        }
                    },
                    
                    // Équipe domicile
                    div {
                        class: "text-center flex-1",
                        div { class: "text-5xl mb-2", "{match_data.home_team.team.logo}" }
                        h3 {
                            style: "color: black; font-size: 24px; font-weight: bold;",
                            "{match_data.home_team.team.city} {match_data.home_team.team.name}"
                        }
                        div {
                            style: "color: #ea580c; font-size: 36px; font-weight: bold; margin-top: 8px;",
                            "{match_data.home_team.total_points}"
                        }
                    }
                },
                
                // Infos match
                div {
                    style: "text-align: center; margin-top: 16px; padding-top: 16px; border-top: 1px solid #e5e7eb; color: #4b5563;",
                    p {
                        "📍 {match_data.arena} • 👥 {match_data.attendance} spectateurs"
                    }
                }
            }
            
            // Scores par quart-temps
            div {
                class: "bg-white rounded-lg shadow-lg p-6 mb-6 overflow-x-auto",
                h4 {
                    style: "color: black; font-weight: bold; font-size: 20px; margin-bottom: 16px;",
                    "Scores par quart-temps"
                }
                table {
                    style: "width: 100%; border-collapse: collapse; color: black;",
                    thead {
                        tr {
                            style: "background-color: #e5e7eb;",
                            th { style: "border: 1px solid #d1d5db; padding: 12px; text-align: left; color: black;", "" }
                            th { style: "border: 1px solid #d1d5db; padding: 12px; text-align: center; color: black;", "Q1" }
                            th { style: "border: 1px solid #d1d5db; padding: 12px; text-align: center; color: black;", "Q2" }
                            th { style: "border: 1px solid #d1d5db; padding: 12px; text-align: center; color: black;", "Q3" }
                            th { style: "border: 1px solid #d1d5db; padding: 12px; text-align: center; color: black;", "Q4" }
                            th { style: "border: 1px solid #d1d5db; padding: 12px; text-align: center; color: black;", "TOTAL" }
                        }
                    }
                    tbody {
                        tr {
                            td { 
                                style: "border: 1px solid #d1d5db; padding: 12px; font-weight: 600; color: black;",
                                "{match_data.away_team.team.logo} {match_data.away_team.team.city}"
                            }
                            for quarter in match_data.away_team.quarter_scores.iter() {
                                td { style: "border: 1px solid #d1d5db; padding: 12px; text-align: center; color: black;", "{quarter}" }
                            }
                            td { 
                                style: "border: 1px solid #d1d5db; padding: 12px; text-align: center; font-weight: bold; color: #ea580c;",
                                "{match_data.away_team.total_points}"
                            }
                        }
                        tr {
                            td { 
                                style: "border: 1px solid #d1d5db; padding: 12px; font-weight: 600; color: black;",
                                "{match_data.home_team.team.logo} {match_data.home_team.team.city}"
                            }
                            for quarter in match_data.home_team.quarter_scores.iter() {
                                td { style: "border: 1px solid #d1d5db; padding: 12px; text-align: center; color: black;", "{quarter}" }
                            }
                            td { 
                                style: "border: 1px solid #d1d5db; padding: 12px; text-align: center; font-weight: bold; color: #ea580c;",
                                "{match_data.home_team.total_points}"
                            }
                        }
                    }
                }
            }
            
            // Statistiques joueurs
            div {
                class: "bg-white rounded-lg shadow-lg p-6",
                h4 {
                    style: "color: black; font-weight: bold; font-size: 20px; margin-bottom: 16px;",
                    "Statistiques individuelles"
                }
                
                // Lakers
                div {
                    class: "mb-6",
                    h5 {
                        style: "color: #9333ea; font-weight: bold; font-size: 18px; margin-bottom: 8px;",
                        "💜 Los Angeles Lakers"
                    }
                    table {
                        style: "width: 100%; border-collapse: collapse; color: black;",
                        thead {
                            tr {
                                style: "background-color: #e5e7eb;",
                                th { style: "border: 1px solid #d1d5db; padding: 8px; text-align: left; color: black;", "Joueur" }
                                th { style: "border: 1px solid #d1d5db; padding: 8px; text-align: center; color: black;", "Pts" }
                                th { style: "border: 1px solid #d1d5db; padding: 8px; text-align: center; color: black;", "Rbd" }
                                th { style: "border: 1px solid #d1d5db; padding: 8px; text-align: center; color: black;", "Ast" }
                                th { style: "border: 1px solid #d1d5db; padding: 8px; text-align: center; color: black;", "Min" }
                            }
                        }
                        tbody {
                            for player in &match_data.home_team.players {
                                tr {
                                    td { style: "border: 1px solid #d1d5db; padding: 8px; color: black;", "{player.name} #{player.number}" }
                                    td { style: "border: 1px solid #d1d5db; padding: 8px; text-align: center; font-weight: bold; color: black;", "{player.points}" }
                                    td { style: "border: 1px solid #d1d5db; padding: 8px; text-align: center; color: black;", "{player.rebounds}" }
                                    td { style: "border: 1px solid #d1d5db; padding: 8px; text-align: center; color: black;", "{player.assists}" }
                                    td { style: "border: 1px solid #d1d5db; padding: 8px; text-align: center; color: black;", "{player.minutes}" }
                                }
                            }
                        }
                    }
                }
                
                // Warriors
                div {
                    h5 {
                        style: "color: #2563eb; font-weight: bold; font-size: 18px; margin-bottom: 8px;",
                        "🔵 Golden State Warriors"
                    }
                    table {
                        style: "width: 100%; border-collapse: collapse; color: black;",
                        thead {
                            tr {
                                style: "background-color: #e5e7eb;",
                                th { style: "border: 1px solid #d1d5db; padding: 8px; text-align: left; color: black;", "Joueur" }
                                th { style: "border: 1px solid #d1d5db; padding: 8px; text-align: center; color: black;", "Pts" }
                                th { style: "border: 1px solid #d1d5db; padding: 8px; text-align: center; color: black;", "Rbd" }
                                th { style: "border: 1px solid #d1d5db; padding: 8px; text-align: center; color: black;", "Ast" }
                                th { style: "border: 1px solid #d1d5db; padding: 8px; text-align: center; color: black;", "Min" }
                            }
                        }
                        tbody {
                            for player in &match_data.away_team.players {
                                tr {
                                    td { style: "border: 1px solid #d1d5db; padding: 8px; color: black;", "{player.name} #{player.number}" }
                                    td { style: "border: 1px solid #d1d5db; padding: 8px; text-align: center; font-weight: bold; color: black;", "{player.points}" }
                                    td { style: "border: 1px solid #d1d5db; padding: 8px; text-align: center; color: black;", "{player.rebounds}" }
                                    td { style: "border: 1px solid #d1d5db; padding: 8px; text-align: center; color: black;", "{player.assists}" }
                                    td { style: "border: 1px solid #d1d5db; padding: 8px; text-align: center; color: black;", "{player.minutes}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}