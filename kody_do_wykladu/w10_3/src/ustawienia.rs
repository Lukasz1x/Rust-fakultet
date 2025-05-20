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
        GraczCzlowiek::new(self.imie_x.clone(), Pionek::Krzyzyk)
    }

    pub fn utworz_gracza_o(&self) -> GraczCzlowiek {
        GraczCzlowiek::new(self.imie_o.clone(), Pionek::Kolko)
    }
}
