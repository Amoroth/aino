pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    #[allow(dead_code)]
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            red: r,
            green: g,
            blue: b
        }
    }

    pub fn success() -> Color {
        // todo #944 involve config?
        Color {
            red: 166,
            green: 227,
            blue: 161
        }
    }

    pub fn warning() -> Color {
        // Color {
        //     red: 250,
        //     green: 179,
        //     blue: 135
        // }
        Color {
            red: 249,
            green: 226,
            blue: 175
        }
    }

    pub fn error() -> Color {
        Color {
            red: 243,
            green: 139,
            blue: 168
        }
    }
}

pub fn colorize(color: Color, input: &str) -> String {
    format!("\x1b[38;2;{};{};{}m", color.red, color.green, color.blue) + input + "\x1b[0m"
}

#[allow(dead_code)]
pub fn bg_colorize(color: Color, input: &str) -> String {
    format!("\x1b[48;2;{};{};{}m", color.red, color.green, color.blue) + input + "\x1b[0m"
}
