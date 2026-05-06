#[allow(dead_code)]
pub fn get_percentage(made: u8, attempted: u8) -> String {
    if attempted == 0 {
        return String::from("0%");
    }
    let pct = (made as f32 / attempted as f32) * 100.0;
    format!("{:.0}%", pct)
}