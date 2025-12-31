pub struct Icons;

impl Icons {
    pub const TRAIL: &str = ">>";
    pub const SUCCESS: &str = "[OK]";
    pub const WARNING: &str = "[!]";
    pub const LINK: &str = "->";
    pub const ELEVATION: &str = "^^";
    pub const SUN: &str = "[*]";
    pub const ALERT: &str = "[!]";

    pub fn weather(code: u32) -> &'static str {
        match code {
            0 => "☼",
            1..=3 => "◐",
            45 | 48 => "≡",
            51 | 53 | 55 => "~",
            56 | 57 => "~",
            61 | 63 | 65 => "\\",
            66 | 67 => "\\",
            71 | 73 | 75 => "*",
            77 => "*",
            80..=82 => "~",
            85 | 86 => "*",
            95 => "^",
            96 | 99 => "^",
            _ => "?",
        }
    }
}
