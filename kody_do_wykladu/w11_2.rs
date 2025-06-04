struct Osoba {
    imie: String,
}

struct Samochod<'a> {
    opis: String,
    wlasciciel: &'a Osoba,
}

fn main() { 
    let o1 = Osoba {imie: "Edek".to_string()};
    let s1 = Samochod {opis: "zielony opel".to_string(), wlasciciel: &o1};
    let s2 = Samochod {opis: "żółty fiat".to_string(), wlasciciel: &o1};
    {
        let s3 = Samochod {opis: "czarny ford".to_string(), wlasciciel: &o1};
    }
}