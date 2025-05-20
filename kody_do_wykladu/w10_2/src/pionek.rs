////////// Pionek

#[derive(Clone, PartialEq, Copy)]
pub enum Pionek {
    Kolko,
    Krzyzyk,
}

impl Pionek {
    pub fn repr(&self) -> char {
        match self {
            Self::Kolko => 'o',
            Self::Krzyzyk => 'x',
        }
    }
}
