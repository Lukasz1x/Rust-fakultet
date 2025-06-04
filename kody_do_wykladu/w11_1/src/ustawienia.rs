use crate::gracz::Gracz;
use crate::gracz_czlowiek::GraczCzlowiek;
use crate::gracz_komputer_slaby::GraczKomputerSlaby;
use crate::pionek::Pionek;

////////// Ustawienia

pub struct Ustawienia {
    pub imie_x: String,
    pub imie_o: String,
    pub pionek_rozpoczynajacy: Pionek,
}

impl Ustawienia {
    pub fn utworz_gracza_x(&self) -> Box<dyn Gracz> {
        if self.imie_x.is_empty() {
            Box::new(GraczKomputerSlaby::new(Pionek::Krzyzyk))
        } else {
            Box::new(GraczCzlowiek::new(self.imie_x.clone(), Pionek::Krzyzyk))
        }
    }

    pub fn utworz_gracza_o(&self) -> Box<dyn Gracz> {
        if self.imie_o.is_empty() {
            Box::new(GraczKomputerSlaby::new(Pionek::Kolko))
        } else {
            Box::new(GraczCzlowiek::new(self.imie_o.clone(), Pionek::Kolko))
        }
    }
}
