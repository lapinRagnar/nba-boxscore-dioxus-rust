#[derive(Debug, Clone, PartialEq)]
pub struct PlayerStats {
    pub name: String,
    pub number: u8,
    pub position: String,
    pub minutes: String,
    pub points: u16,
    pub rebounds: u8,
    pub assists: u8,
    pub steals: u8,
    pub blocks: u8,
    pub turnovers: u8,
    pub fouls: u8,
    pub field_goals: (u8, u8),
    pub three_points: (u8, u8),
    pub free_throws: (u8, u8),
}