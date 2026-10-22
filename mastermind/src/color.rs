#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Pink,
    Purple,
}

impl Color {
    pub fn from_index(index: usize) -> Color {
        match index {
            0 => Color::Red,
            1 => Color::Green,
            2 => Color::Blue,
            3 => Color::Yellow,
            4 => Color::Pink,
            5 => Color::Purple,
            _ => panic!("Invalid color index : {}", index)
        }
    }
}
