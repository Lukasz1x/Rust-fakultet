use crate::pionek::Pionek;

////////// Wsp + Ruch

pub type Wsp = (usize, usize);

pub struct Ruch {
    pub wsp: Wsp,
    pub pionek: Pionek,
}
