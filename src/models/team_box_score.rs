use super::{Team, PlayerStats};

#[derive(Debug, Clone, PartialEq)]
pub struct TeamBoxScore {
    pub team: Team,
    pub players: Vec<PlayerStats>,
    pub total_points: u16,
    pub quarter_scores: [u8; 4],
}