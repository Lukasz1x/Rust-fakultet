use crate::interfejs_tekstowy::InterfejsTekstowy;
use crate::pionek::Pionek;
use crate::plansza::Plansza;
use crate::ruch::Ruch;

////////// Gracz

pub trait Gracz {
    fn pionek(&self) -> Pionek;
    fn imie(&self) -> &str;
    fn wybierz_ruch(&self, interfejs: &InterfejsTekstowy, plansza: &Plansza) -> Ruch;

    fn opis(&self) -> String {
        format!("{} ({})", self.imie(), self.pionek().repr())
    }
}
