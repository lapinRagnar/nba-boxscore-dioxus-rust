use crate::models::Game;

pub fn get_games() -> Vec<Game> {
    vec![
        // Conférence Est
        Game {
            id: 1,
            home_team: String::from("Knicks"),
            away_team: String::from("76ers"),
            home_score: Some(137),
            away_score: Some(98),
            status: String::from("Terminé"),
            time: String::from("RÉCIT DU MATCH"),
            conference: String::from("East"),
            is_live: false,
        },
        Game {
            id: 2,
            home_team: String::from("Knicks"),
            away_team: String::from("76ers"),
            home_score: None,
            away_score: None,
            status: String::from("Demain"),
            time: String::from("01:00"),
            conference: String::from("East"),
            is_live: false,
        },
        Game {
            id: 3,
            home_team: String::from("Pistons"),
            away_team: String::from("Cavaliers"),
            home_score: Some(111),
            away_score: Some(101),
            status: String::from("Terminé"),
            time: String::from("9:54"),
            conference: String::from("East"),
            is_live: false,
        },
        // Conférence Ouest
        Game {
            id: 4,
            home_team: String::from("Spurs"),
            away_team: String::from("Timberwolves"),
            home_score: Some(102),
            away_score: Some(104),
            status: String::from("Terminé"),
            time: String::from("9:02"),
            conference: String::from("West"),
            is_live: false,
        },
        Game {
            id: 5,
            home_team: String::from("Thunder"),
            away_team: String::from("Lakers"),
            home_score: Some(108),
            away_score: Some(90),
            status: String::from("Terminé"),
            time: String::from("5:09"),
            conference: String::from("West"),
            is_live: false,
        },
        Game {
            id: 6,
            home_team: String::from("Spurs"),
            away_team: String::from("Timberwolves"),
            home_score: None,
            away_score: None,
            status: String::from("Demain"),
            time: String::from("03:30"),
            conference: String::from("West"),
            is_live: false,
        },
    ]
}