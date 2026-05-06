#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    pub id: u32,
    pub home_team: String,
    pub away_team: String,
    pub home_score: Option<u16>,
    pub away_score: Option<u16>,
    pub status: String,
    pub time: String,
    pub conference: String,
    pub is_live: bool,
}