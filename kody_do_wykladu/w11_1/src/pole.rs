use crate::pionek::Pionek;

////////// Pole

#[derive(Clone, PartialEq, Copy)]
pub enum Pole {
    Puste,
    Zajete(Pionek),
}

impl Pole {
    pub fn repr(&self) -> char {
        match self {
            Self::Puste => ' ',
            Self::Zajete(pionek) => pionek.repr(),
        }
    }

    pub fn zajete(&self) -> bool {
        matches!(self, Self::Zajete(_))
    }
}
