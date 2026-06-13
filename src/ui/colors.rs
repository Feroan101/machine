use colored::Color;

pub const PRIMARY: Color = Color::Cyan;
pub const SECONDARY: Color = Color::TrueColor { r: 128, g: 128, b: 128 }; // DarkGray equivalent
pub const SUCCESS: Color = Color::Green;
pub const WARNING: Color = Color::Yellow;
pub const ERROR: Color = Color::Red;
pub const INFO: Color = Color::Blue;

pub fn get_severity_color(usage: f32) -> Color {
    if usage > 90.0 {
        ERROR
    } else if usage > 70.0 {
        WARNING
    } else {
        SUCCESS
    }
}
