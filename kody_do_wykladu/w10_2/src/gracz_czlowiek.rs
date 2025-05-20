use crate::interfejs_tekstowy::InterfejsTekstowy;
use crate::pionek::Pionek;
use crate::ruch::Ruch;

////////// GraczCzlowiek

pub struct GraczCzlowiek {
    pub pionek: Pionek,
    pub imie: String,
}

impl GraczCzlowiek {
    pub fn opis(&self) -> String {
        format!("{} ({})", self.imie, self.pionek.repr())
    }

    pub fn wybierz_ruch(&self, interfejs: &InterfejsTekstowy) -> Ruch {
        interfejs.spytaj_o_ruch(&self.opis(), self.pionek)
    }
}
