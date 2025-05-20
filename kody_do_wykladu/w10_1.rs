////////// Pionek

#[derive(Clone, PartialEq, Copy)]
enum Pionek {
    Kolko,
    Krzyzyk,
}

impl Pionek {
    fn repr(&self) -> char {
        match self {
            Self::Kolko => 'o',
            Self::Krzyzyk => 'x',
        }
    }
}

////////// Wsp + Ruch

type Wsp = (usize, usize);

struct Ruch {
    wsp: Wsp,
    pionek: Pionek,
}

////////// Wynik

#[derive(Clone, Copy)]
enum Wynik {
    Kolko,
    Krzyzyk,
    Remis,
}

impl Wynik {
    fn from_pole(pole: Pole) -> Self {
        match pole {
            Pole::Zajete(Pionek::Krzyzyk) => Self::Krzyzyk,
            Pole::Zajete(Pionek::Kolko) => Self::Kolko,
            Pole::Puste => panic!("to nie powinno się zdarzyć! [wygrana pustego pola?]"),
        }
    }
}

////////// Pole

#[derive(Clone, PartialEq, Copy)]
enum Pole {
    Puste,
    Zajete(Pionek),
}

impl Pole {
    fn repr(&self) -> char {
        match self {
            Self::Puste => ' ',
            Self::Zajete(pionek) => pionek.repr(),
        }
    }

    fn zajete(&self) -> bool {
        matches!(self, Self::Zajete(_))
    }
}

////////// Plansza

struct Plansza {
    zaw: Vec<Vec<Pole>>,
    wynik: Option<Wynik>,
}

impl Plansza {
    fn new() -> Self {
        Self {
            zaw: vec![vec![Pole::Puste; 3]; 3],
            wynik: None,
        }
    }

    fn wysokosc(&self) -> usize {
        self.zaw.len()
    }

    fn szerokosc(&self) -> usize {
        self.zaw[0].len()
    }

    fn czy_ruch_poprawny(&self, ruch: &Ruch) -> bool {
        ruch.wsp.0 < self.wysokosc()
            && ruch.wsp.1 < self.szerokosc()
            && !self.pole(ruch.wsp).zajete()
    }

    fn wykonaj_ruch(&mut self, ruch: &Ruch) {
        self.zaw[ruch.wsp.0][ruch.wsp.1] = Pole::Zajete(ruch.pionek);
    }

    fn czy_koniec(&mut self) -> bool {
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

    fn wynik_partii(&self) -> Wynik {
        self.wynik
            .expect("to nie powinno się zdarzyć! [sprawdzenie wyniku przed końcem partii?]")
    }

    fn pole(&self, wsp: Wsp) -> Pole {
        self.zaw[wsp.0][wsp.1]
    }
}

////////// GraczCzlowiek

struct GraczCzlowiek {
    pionek: Pionek,
    imie: String,
}

impl GraczCzlowiek {
    fn opis(&self) -> String {
        format!("{} ({})", self.imie, self.pionek.repr())
    }

    fn wybierz_ruch(&self, interfejs: &InterfejsTekstowy) -> Ruch {
        interfejs.spytaj_o_ruch(&self.opis(), self.pionek)
    }
}

////////// Ustawienia

struct Ustawienia {
    imie_x: String,
    imie_o: String,
    pionek_rozpoczynajacy: Pionek,
}

impl Ustawienia {
    fn utworz_gracza_x(&self) -> GraczCzlowiek {
        GraczCzlowiek {
            imie: self.imie_x.clone(),
            pionek: Pionek::Krzyzyk,
        }
    }

    fn utworz_gracza_o(&self) -> GraczCzlowiek {
        GraczCzlowiek {
            imie: self.imie_o.clone(),
            pionek: Pionek::Kolko,
        }
    }
}

////////// InterfejsTekstowy

use std::io::Write;

struct InterfejsTekstowy;

impl InterfejsTekstowy {
    fn pokaz_ruch(&self, ruch: &Ruch) {
        println!(
            "\nPostawiono: {} na polu {:?}.",
            ruch.pionek.repr(),
            ruch.wsp
        );
    }

    fn pobierz_ustawienia(&self) -> Ustawienia {
        let imie_o = Self::wczytaj_napis("Imię gracza o: ");
        let imie_x = Self::wczytaj_napis("Imię gracza x: ");
        let kto_zaczyna_pom =
            Self::wczytaj_napis_z_podanych("Kto zaczyna", vec!["x".to_string(), "o".to_string()]);
        let pionek_rozpoczynajacy = if kto_zaczyna_pom == "x" {
            Pionek::Krzyzyk
        } else {
            Pionek::Kolko
        };
        Ustawienia {
            imie_x,
            imie_o,
            pionek_rozpoczynajacy,
        }
    }

    fn oglos_wynik(&self, wynik: &Wynik, opis: &str) {
        println!(
            "{}",
            match wynik {
                Wynik::Remis => "Remis...".to_string(),
                _ => format!("Wygrywa {opis}!"),
            }
        )
    }

    fn komunikat_o_niepoprawnym_ruchu(&self, ruch: &Ruch) {
        println!(
            "Postawienie {} na polu {:?} jest niemożliwe!",
            ruch.pionek.repr(),
            ruch.wsp
        );
    }

    fn pokaz_plansze(&self, plansza: &Plansza) {
        for w in 0..plansza.wysokosc() {
            if w > 0 {
                Self::pokaz_wypleniacz(plansza.szerokosc(), '+', '-');
            }
            Self::pokaz_wiersz(plansza, w);
        }
    }

    fn wczytaj_napis_z_podanych(prompt: &str, podane: Vec<String>) -> String {
        loop {
            let odp = Self::wczytaj_napis(&(prompt.to_string() + "(" + &podane.join(", ") + "): "));
            if podane.contains(&odp) {
                return odp;
            } else {
                println!("Błąd! Podaj ponownie...");
            }
        }
    }

    fn wczytaj_napis(prompt: &str) -> String {
        let mut odp = String::new();
        print!("{}", prompt);
        std::io::stdout()
            .flush()
            .expect("fatalny problem ze standardowym wyjściem");
        std::io::stdin()
            .read_line(&mut odp)
            .expect("fatalny problem ze standardowym wejściem");
        // odp.trim_end_matches('\n').to_string()
        odp.trim_end().to_string()
    }

    fn wczytaj_usize(prompt: &str) -> usize {
        loop {
            let odp = Self::wczytaj_napis(prompt);
            if let Ok(res) = odp.parse() {
                return res;
            } else {
                println!("Błąd! Podaj ponownie...");
            }
        }
    }

    fn spytaj_o_ruch(&self, opis: &str, pionek: Pionek) -> Ruch {
        println!("\n{}: podaj ruch...", opis);
        let w = Self::wczytaj_usize("wiersz (od 0 do ...): ");
        let k = Self::wczytaj_usize("kolumna (od 0 do ...): ");
        Ruch {
            wsp: (w, k),
            pionek,
        }
    }

    fn pokaz_wiersz(plansza: &Plansza, w: usize) {
        Self::pokaz_wypleniacz(plansza.szerokosc(), '|', ' ');
        for k in 0..plansza.szerokosc() {
            if k > 0 {
                print!("|");
            }
            print!(" {} ", plansza.pole((w, k)).repr());
        }
        println!();
        Self::pokaz_wypleniacz(plansza.szerokosc(), '|', ' ');
    }

    fn pokaz_wypleniacz(sz: usize, sep: char, z: char) {
        for k in 0..sz {
            if k > 0 {
                print!("{sep}");
            }
            print!("{z}{z}{z}")
        }
        println!();
    }
}

////////// Gra

struct Gra {
    interfejs: InterfejsTekstowy,
    gracze: Vec<GraczCzlowiek>,
    indeks_biezacego_gracza: usize,
}

impl Gra {
    fn new(
        gracz0: GraczCzlowiek,
        gracz1: GraczCzlowiek,
        pionek_rozpoczynajacy: Pionek,
        interfejs: InterfejsTekstowy,
    ) -> Result<Self, String> {
        if gracz0.pionek == gracz1.pionek {
            return Err("Gracze nie mogą mieć tego samego pionka!".to_string());
        }
        let indeks_biezacego_gracza = if gracz0.pionek == pionek_rozpoczynajacy {
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

    fn aktualny_gracz(&self) -> &GraczCzlowiek {
        &self.gracze[self.indeks_biezacego_gracza]
    }

    fn zmien_gracza(&mut self) {
        self.indeks_biezacego_gracza = 1 - self.indeks_biezacego_gracza;
    }

    fn graj(&mut self) -> Wynik {
        let mut plansza = Plansza::new();
        self.interfejs.pokaz_plansze(&plansza);
        loop {
            let ruch = self.aktualny_gracz().wybierz_ruch(&self.interfejs);
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

////////// main

fn main() -> Result<(), String> {
    let interfejs = InterfejsTekstowy;
    let ustawienia = interfejs.pobierz_ustawienia();
    let gra = Gra::new(
        ustawienia.utworz_gracza_o(),
        ustawienia.utworz_gracza_x(),
        ustawienia.pionek_rozpoczynajacy,
        interfejs,
    );
    gra?.graj();
    Ok(())
}