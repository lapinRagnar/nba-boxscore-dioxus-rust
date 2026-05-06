pub mod navbar;
pub mod home;
pub mod box_score;
pub mod regular_season;  // ← Ajoute cette ligne

pub use navbar::Navbar;
pub use home::Home;
pub use box_score::BoxScoreDisplay;
pub use regular_season::RegularSeason;