use crate::pole::Pole;
use crate::ruch::Ruch;
use crate::ruch::Wsp;
use crate::wynik::Wynik;

////////// Plansza

pub struct Plansza {
    zaw: Vec<Vec<Pole>>,
    wynik: Option<Wynik>,
}

impl Plansza {
    pub fn new() -> Self {
        Self {
            zaw: vec![vec![Pole::Puste; 3]; 3],
            wynik: None,
        }
    }

    pub fn wysokosc(&self) -> usize {
        self.zaw.len()
    }

    pub fn szerokosc(&self) -> usize {
        self.zaw[0].len()
    }

    pub fn czy_ruch_poprawny(&self, ruch: &Ruch) -> bool {
        ruch.wsp.0 < self.wysokosc()
            && ruch.wsp.1 < self.szerokosc()
            && !self.pole(ruch.wsp).zajete()
    }

    pub fn wykonaj_ruch(&mut self, ruch: &Ruch) {
        self.zaw[ruch.wsp.0][ruch.wsp.1] = Pole::Zajete(ruch.pionek);
    }

    pub fn czy_koniec(&mut self) -> bool {
        // działa tylko dla tradycyjnego KiK (3x3)
        if self.zaw[0][0].zajete()
            && (self.zaw[0][0] == self.zaw[0][1] && self.zaw[0][1] == self.zaw[0][2]
                || self.zaw[0][0] == self.zaw[1][1] && self.zaw[1][1] == self.zaw[2][2]
                || self.zaw[0][0] == self.zaw[1][0] && self.zaw[1][0] == self.zaw[2][0])
        {
            self.wynik = Some(Wynik::from_pole(self.zaw[0][0]));
            return true;
        }
        if self.zaw[1][1].zajete()
            && (self.zaw[1][1] == self.zaw[1][2] && self.zaw[1][1] == self.zaw[1][0]
                || self.zaw[1][1] == self.zaw[0][2] && self.zaw[1][1] == self.zaw[2][0]
                || self.zaw[1][1] == self.zaw[2][1] && self.zaw[1][1] == self.zaw[0][1])
        {
            self.wynik = Some(Wynik::from_pole(self.zaw[1][1]));
            return true;
        }
        if self.zaw[2][2].zajete()
            && (self.zaw[2][2] == self.zaw[2][1] && self.zaw[2][2] == self.zaw[2][0]
                || self.zaw[2][2] == self.zaw[1][2] && self.zaw[2][2] == self.zaw[0][2])
        {
            self.wynik = Some(Wynik::from_pole(self.zaw[2][2]));
            return true;
        }
        for linia in &self.zaw {
            for pole in linia {
                if !pole.zajete() {
                    return false;
                }
            }
        }
        self.wynik = Some(Wynik::Remis);
        true
    }

    pub fn wynik_partii(&self) -> Wynik {
        self.wynik
            .expect("to nie powinno się zdarzyć! [sprawdzenie wyniku przed końcem partii?]")
    }

    pub fn pole(&self, wsp: Wsp) -> Pole {
        self.zaw[wsp.0][wsp.1]
    }
}
