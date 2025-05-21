#[derive(Debug, PartialEq, Clone)]
enum Jednostka
{
    Sztuki(f64),
    Litry(f64),
    Kilogramy,
}

#[derive(Debug, PartialEq, Clone)]
enum Przechowywanie
{
    Zamrazarka,
    Chlodziarka,
    Normalne,
}

#[derive(Debug, PartialEq, Clone)]
struct Towar
{
    nazwa: String,
    jednostka: Jednostka,
    waga_jednostkowa_w_kg: f64,
    przechowywanie: Przechowywanie
}

impl Towar
{
    fn new(nazwa: &str, jednostka: Jednostka, przechowywanie: Przechowywanie) -> Option<Self>
    {
        let waga_jednostkowa_w_kg = match &jednostka {
            Jednostka::Kilogramy => 1.0,
            Jednostka::Sztuki(waga) => *waga,
            Jednostka::Litry(waga) => *waga,
        };

        if waga_jednostkowa_w_kg < 0.0
        {
            return None;
        }

        Some(Self{
            nazwa: nazwa.to_string(),
            waga_jednostkowa_w_kg,
            jednostka,
            przechowywanie

        })
    }
}

#[derive(Debug)]
struct Zamowienie
{
    vec: Vec<(Towar, f64)>
}

impl Zamowienie
{
    fn new() -> Zamowienie
    {
        Self{
            vec: Vec::new()
        }
    }

    fn waga(&self) -> f64
    {
        let mut w=0.0;
        for i in self.vec.iter()
        {
            w+= (*i).0.waga_jednostkowa_w_kg*(*i).1;
        }
        w
    }

    fn waga_wg_warunku(&self, war: Przechowywanie) -> f64
    {
        let mut w=0.0;
        for i in self.vec.iter()
        {
            if (*i).0.przechowywanie == war
            {
                w += (*i).0.waga_jednostkowa_w_kg * (*i).1;
            }
        }
        w
    }

    fn dodaj_towar(&mut self, towar: Towar, ilosc: f64)
    {
        if ilosc<0.0
        {
            println!("Ilość jest poniżej 0!");
            return;
        }
        if let Jednostka::Sztuki(x) = towar.jednostka
        {
            if ilosc.fract() != 0.0
            {
                println!("Ilość dla jednostki sztuki powinna być całkowita");
                return;
            }
        }
        for i in self.vec.iter_mut()
        {
            if (*i).0.nazwa == towar.nazwa
            {
                (*i).1+=ilosc;
                return;
            }
        }
        self.vec.push((towar, ilosc));
    }
}

fn main() {
    let mut zamowienie = Zamowienie::new();

    let mleko = Towar::new("Mleko", Jednostka::Litry(1.0), Przechowywanie::Chlodziarka).unwrap();
    let lody = Towar::new("Lody", Jednostka::Kilogramy, Przechowywanie::Zamrazarka).unwrap();
    let jajka = Towar::new("Jajka", Jednostka::Sztuki(0.06), Przechowywanie::Normalne).unwrap();
    let jogurt = Towar::new("Jogurt", Jednostka::Sztuki(0.4), Przechowywanie::Chlodziarka).unwrap();

    // Dodawanie poprawnych towarów
    zamowienie.dodaj_towar(mleko.clone(), 3.0);
    zamowienie.dodaj_towar(jogurt.clone(), 10.0);
    zamowienie.dodaj_towar(lody.clone(), 2.0);
    zamowienie.dodaj_towar(jajka.clone(), 12.0);

    // Próba dodania z błędną ilością sztuk (niecałkowita)
    zamowienie.dodaj_towar(jajka.clone(), 2.5); // powinien wypisać błąd

    // Próba dodania ujemnej ilości
    zamowienie.dodaj_towar(mleko.clone(), -1.0); // powinien wypisać błąd

    println!("Zamówienie: {:?}", zamowienie);
    println!(
        "Sumaryczna waga: {} kg",
        zamowienie.waga()
    );
    println!(
        "Waga chłodzonych produktów: {} kg",
        zamowienie.waga_wg_warunku(Przechowywanie::Chlodziarka)
    );
}
