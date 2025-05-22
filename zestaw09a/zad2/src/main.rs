enum Blad
{
    BrakBledu,
    ZlyFormatPliku,
    PlikNieIstnieje(String),
    PlikZbytDuzy(f64, f64)
}

impl Blad
{
    fn pokaz_komunikat(&self)
    {
        match &self {
            Blad::BrakBledu => println!("Brak błędu"),
            Blad::ZlyFormatPliku => println!("Zły format pliku!"),
            Blad::PlikNieIstnieje(s) => println!("Plik o nazwie \"{}\" nie istnieje", s),
            Blad::PlikZbytDuzy(a, b) => println!("Rozmiar pliku ({a}) przekracza maksymalny rozmiar ({b})")
        }
    }
}

fn main() {
    let b = Blad::BrakBledu;
    b.pokaz_komunikat();
    let b = Blad::ZlyFormatPliku;
    b.pokaz_komunikat();
    let s = "dane/goscie.csv".to_string();
    let b = Blad::PlikNieIstnieje(s);
    b.pokaz_komunikat();
    let b = Blad::PlikZbytDuzy(27.3, 10.0);
    b.pokaz_komunikat();
}
