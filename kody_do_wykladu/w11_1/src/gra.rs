use crate::gracz::Gracz;
use crate::interfejs_tekstowy::InterfejsTekstowy;
use crate::pionek::Pionek;
use crate::plansza::Plansza;
use crate::wynik::Wynik;

////////// Gra

pub struct Gra {
    interfejs: InterfejsTekstowy,
    gracze: Vec<Box<dyn Gracz>>,
    indeks_biezacego_gracza: usize,
}

impl Gra {
    pub fn new(
        gracz0: Box<dyn Gracz>,
        gracz1: Box<dyn Gracz>,
        pionek_rozpoczynajacy: Pionek,
        interfejs: InterfejsTekstowy,
    ) -> Result<Self, String> {
        if gracz0.pionek() == gracz1.pionek() {
            return Err("Gracze nie mogą mieć tego samego pionka!".to_string());
        }
        let indeks_biezacego_gracza = if gracz0.pionek() == pionek_rozpoczynajacy {
            0
        } else {
            1
        };
        Ok(Self {
            gracze: vec![gracz0, gracz1],
            indeks_biezacego_gracza,
            interfejs,
        })
    }

    fn aktualny_gracz(&self) -> &Box<dyn Gracz> {
        &self.gracze[self.indeks_biezacego_gracza]
    }

    fn zmien_gracza(&mut self) {
        self.indeks_biezacego_gracza = 1 - self.indeks_biezacego_gracza;
    }

    pub fn graj(&mut self) -> Wynik {
        let mut plansza = Plansza::new();
        self.interfejs.pokaz_plansze(&plansza);
        loop {
            let ruch = self
                .aktualny_gracz()
                .wybierz_ruch(&self.interfejs, &plansza);
            if plansza.czy_ruch_poprawny(&ruch) {
                plansza.wykonaj_ruch(&ruch);
                self.interfejs.pokaz_ruch(&ruch);
                self.interfejs.pokaz_plansze(&plansza);
                if plansza.czy_koniec() {
                    break;
                }
                self.zmien_gracza();
            } else {
                self.interfejs.komunikat_o_niepoprawnym_ruchu(&ruch);
            }
        }
        let wynik = plansza.wynik_partii();
        self.interfejs
            .oglos_wynik(&wynik, &self.aktualny_gracz().opis());
        wynik
    }
}
