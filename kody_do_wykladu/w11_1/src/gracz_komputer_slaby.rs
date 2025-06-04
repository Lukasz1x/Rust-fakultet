use crate::gracz::Gracz;
use crate::interfejs_tekstowy::InterfejsTekstowy;
use crate::pionek::Pionek;
use crate::plansza::Plansza;
use crate::ruch::Ruch;

////////// GraczKomputerSlaby

pub struct GraczKomputerSlaby {
    pionek: Pionek,
}

impl GraczKomputerSlaby {
    pub fn new(pionek: Pionek) -> Self {
        Self { pionek }
    }
}

impl Gracz for GraczKomputerSlaby {
    fn pionek(&self) -> Pionek {
        self.pionek
    }

    fn imie(&self) -> &str {
        &"Komputer Słaby"
    }

    fn wybierz_ruch(&self, _interfejs: &InterfejsTekstowy, plansza: &Plansza) -> Ruch {
        for w in 0..3 {
            for k in 0..3 {
                if !plansza.pole((w, k)).zajete() {
                    return Ruch {
                        wsp: (w, k),
                        pionek: self.pionek(),
                    };
                }
            }
        }
        panic!("ruch przy zapełnionej planszy -- w tym miejscu aplikacja nie powinna się znaleźć!");
        //         unreachable!();
    }
}
