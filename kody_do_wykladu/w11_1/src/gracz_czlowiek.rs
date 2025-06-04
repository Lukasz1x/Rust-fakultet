use crate::gracz::Gracz;
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
}

impl Gracz for GraczCzlowiek {
    fn pionek(&self) -> Pionek {
        self.pionek
    }

    fn imie(&self) -> &str {
        &self.imie
    }

    fn wybierz_ruch(&self, interfejs: &InterfejsTekstowy, _plansza: &Plansza) -> Ruch {
        interfejs.spytaj_o_ruch(&self.opis(), self.pionek)
    }
}
