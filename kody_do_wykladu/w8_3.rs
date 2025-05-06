#[derive(PartialEq, Copy, Clone)]
enum Element {
    Plus,
    Minus,
    Razy,
    Podzielic,
    Liczba(f64)
}

type Stos = Vec<f64>;

fn wykonaj_dzialanie(rodzaj: Element, a: f64, b: f64) -> f64 {
    if rodzaj == Element::Plus {
        a + b
    } else if rodzaj == Element::Minus {
        a - b
    } else if rodzaj == Element::Razy {
        a * b
    } else if rodzaj == Element::Podzielic {
        a / b
    } else {
        panic!("niespodziewana Liczba (z typu Element)")
    }
}

fn oblicz_onp(kolejka: &Vec<Element>) -> Option<f64> {
    let mut stos = Stos::new();
    for e in kolejka {
        if let Element::Liczba(wartosc) = *e {
            stos.push(wartosc);
        } else {
            let b = stos.pop()?;
            let a = stos.pop()?;
            let wynik = wykonaj_dzialanie(*e, a, b);
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
    k.push(Element::Liczba(34.5));
    println!("{:?}", oblicz_onp(&k));
    k.push(Element::Liczba(1.0));
    k.push(Element::Liczba(0.5));
    k.push(Element::Plus);
    k.push(Element::Razy);
    println!("{:?}", oblicz_onp(&k));

    let k1 = Vec::new();
    println!("{:?}", oblicz_onp(&k1));

    let mut k2 = Vec::new();
    k2.push(Element::Liczba(1.0));
    k2.push(Element::Liczba(0.5));
    println!("{:?}", oblicz_onp(&k2));  // None

    let mut k3 = Vec::new();
    k3.push(Element::Liczba(1.0));
    k3.push(Element::Liczba(0.5));
    k3.push(Element::Plus);
    k3.push(Element::Plus);
    k3.push(Element::Plus);
    println!("{:?}", oblicz_onp(&k3));  // None
}