use crate::gracz_czlowiek::GraczCzlowiek;
use crate::pionek::Pionek;

////////// Ustawienia

pub struct Ustawienia {
    pub imie_x: String,
    pub imie_o: String,
    pub pionek_rozpoczynajacy: Pionek,
}

impl Ustawienia {
    pub fn utworz_gracza_x(&self) -> GraczCzlowiek {
        GraczCzlowiek {
            imie: self.imie_x.clone(),
            pionek: Pionek::Krzyzyk,
        }
    }

    pub fn utworz_gracza_o(&self) -> GraczCzlowiek {
        GraczCzlowiek {
            imie: self.imie_o.clone(),
            pionek: Pionek::Kolko,
        }
    }
}
