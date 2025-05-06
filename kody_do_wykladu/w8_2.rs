#[derive(PartialEq, Copy, Clone)]
enum Rodzaj {
    Plus,
    Minus,
    Razy,
    Podzielic,
    Liczba
}

struct Element {
    rodzaj: Rodzaj,
    wartosc: f64
}

type Stos = Vec<f64>;

fn wykonaj_dzialanie(rodzaj: Rodzaj, a: f64, b: f64) -> f64 {
    if rodzaj == Rodzaj::Plus {
        a + b
    } else if rodzaj == Rodzaj::Minus {
        a - b
    } else if rodzaj == Rodzaj::Razy {
        a * b
    } else if rodzaj == Rodzaj::Podzielic {
        a / b
    } else {
        panic!("niespodziewana Liczba (z typu Rodzaj)")
    }
}

fn oblicz_onp(kolejka: &Vec<Element>) -> Option<f64> {
    let mut stos = Stos::new();
    for e in kolejka {
        if e.rodzaj == Rodzaj::Liczba {
            stos.push(e.wartosc);
        } else {
            let b = stos.pop()?;
            let a = stos.pop()?;
            let wynik = wykonaj_dzialanie(e.rodzaj, a, b);
            stos.push(wynik);
        }
    }
    let wynik_koncowy = stos.pop();
    if !stos.is_empty() {
        return None;
    }
    return wynik_koncowy;
}

fn main() {
    let mut k = Vec::new();
    k.push(Element{rodzaj:Rodzaj::Liczba, wartosc:34.5});
    println!("{:?}", oblicz_onp(&k));
    k.push(Element{rodzaj:Rodzaj::Liczba, wartosc:1.0});
    k.push(Element{rodzaj:Rodzaj::Liczba, wartosc:0.5});
    k.push(Element{rodzaj:Rodzaj::Plus, wartosc:0.0});
    k.push(Element{rodzaj:Rodzaj::Razy, wartosc:0.0});
    println!("{:?}", oblicz_onp(&k));

    let k1 = Vec::new();
    println!("{:?}", oblicz_onp(&k1));

    let mut k2 = Vec::new();
    k2.push(Element{rodzaj:Rodzaj::Liczba, wartosc:1.0});
    k2.push(Element{rodzaj:Rodzaj::Liczba, wartosc:0.5});
    println!("{:?}", oblicz_onp(&k2));  // None

    let mut k3 = Vec::new();
    k3.push(Element{rodzaj:Rodzaj::Liczba, wartosc:1.0});
    k3.push(Element{rodzaj:Rodzaj::Liczba, wartosc:0.5});
    k3.push(Element{rodzaj:Rodzaj::Plus, wartosc:0.0});
    k3.push(Element{rodzaj:Rodzaj::Plus, wartosc:0.0});
    k3.push(Element{rodzaj:Rodzaj::Plus, wartosc:0.0});
    println!("{:?}", oblicz_onp(&k3));  // None
}