use crate::pionek::Pionek;
use crate::plansza::Plansza;
use crate::ruch::Ruch;
use crate::ustawienia::Ustawienia;
use crate::wynik::Wynik;

////////// InterfejsTekstowy

use std::io::Write;

pub struct InterfejsTekstowy;

impl InterfejsTekstowy {
    pub fn pokaz_ruch(&self, ruch: &Ruch) {
        println!(
            "\nPostawiono: {} na polu {:?}.",
            ruch.pionek.repr(),
            ruch.wsp
        );
    }

    pub fn pobierz_ustawienia(&self) -> Ustawienia {
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

    pub fn oglos_wynik(&self, wynik: &Wynik, opis: &str) {
        println!(
            "{}",
            match wynik {
                Wynik::Remis => "Remis...".to_string(),
                _ => format!("Wygrywa {opis}!"),
            }
        )
    }

    pub fn komunikat_o_niepoprawnym_ruchu(&self, ruch: &Ruch) {
        println!(
            "Postawienie {} na polu {:?} jest niemożliwe!",
            ruch.pionek.repr(),
            ruch.wsp
        );
    }

    pub fn pokaz_plansze(&self, plansza: &Plansza) {
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
        odp.trim_end_matches('\n').to_string()
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

    pub fn spytaj_o_ruch(&self, opis: &str, pionek: Pionek) -> Ruch {
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
