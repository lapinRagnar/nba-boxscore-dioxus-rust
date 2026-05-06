use crate::models::{Match, Team, TeamBoxScore, PlayerStats};

pub fn get_mock_match() -> Match {
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