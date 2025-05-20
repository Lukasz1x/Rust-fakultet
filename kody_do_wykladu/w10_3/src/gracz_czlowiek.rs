use crate::interfejs_tekstowy::InterfejsTekstowy;
use crate::pionek::Pionek;
use crate::plansza::Plansza;
use crate::ruch::Ruch;

////////// GraczCzlowiek

pub struct GraczCzlowiek {
    pionek: Pionek,
    imie: String,
}

impl GraczCzlowiek {
    pub fn new(imie: String, pionek: Pionek) -> Self {
        Self { pionek, imie }
    }

    pub fn pionek(&self) -> Pionek {
        self.pionek
    }

    pub fn imie(&self) -> &String {
        &self.imie
    }

    pub fn opis(&self) -> String {
        format!("{} ({})", self.imie, self.pionek.repr())
    }

    pub fn wybierz_ruch(&self, interfejs: &InterfejsTekstowy, plansza: &Plansza) -> Ruch {
        interfejs.spytaj_o_ruch(&self.opis(), self.pionek)
    }
}
