use std::io::Write;

fn main() {
    let imie = wczytaj_napis("Imię? ");
    let wiek = wczytaj_usize("Ile masz lat? ");
    let ul = wczytaj_f64("Ulubiona liczba? ");
    println!("Cześć, {imie}, lat {wiek}!");
    println!("Twoja ulubiona liczba: {ul}...");
}

fn wczytaj_napis(prompt: &str) -> String {
    let mut odp = String::new();
    print!("{prompt}");
    std::io::stdout().flush().expect("???: problem z flush");
    std::io::stdin().read_line(&mut odp).expect("???: problem z read_line");
    //return odp.trim_end_matches('\n').to_string();    //to działa tylko na linux, bo na windowsie koniec linii to "\r\n"
    return odp.trim_end().to_string();                  //to usuwa wszystkie białe znaki na końcu '\r', '\n', '\t', ' '
}

fn wczytaj_usize(prompt: &str) -> usize {
    loop {
        let odp = wczytaj_napis(prompt);
        if let Ok(wynik) = odp.parse() {
            return wynik;
        } else {
            println!("Błąd, podaj ponownie!");
        }
    }
}

fn wczytaj_f64(prompt: &str) -> f64 {
    loop {
        let odp = wczytaj_napis(prompt);
        //if let Ok(wynik) = odp.parse::<f64>() {   // turbofish
        if let Ok(wynik) = odp.parse() {   // turbofish
            return wynik;
        } else {
            println!("Błąd, podaj ponownie!");
        }
    }
}
