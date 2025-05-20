use crate::pionek::Pionek;
use crate::pole::Pole;

////////// Wynik

#[derive(Clone, Copy)]
pub enum Wynik {
    Kolko,
    Krzyzyk,
    Remis,
}

impl Wynik {
    pub fn from_pole(pole: Pole) -> Self {
        match pole {
            Pole::Zajete(Pionek::Krzyzyk) => Self::Krzyzyk,
            Pole::Zajete(Pionek::Kolko) => Self::Kolko,
            Pole::Puste => panic!("to nie powinno się zdarzyć! [wygrana pustego pola?]"),
        }
    }
}
