use super::TeamBoxScore;

#[derive(Debug, Clone, PartialEq)]
pub struct Match {
    pub id: u32,
    pub home_team: TeamBoxScore,
    pub away_team: TeamBoxScore,
    pub date: String,
    pub arena: String,
    pub attendance: u32,
}