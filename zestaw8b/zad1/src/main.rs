#[derive(Debug, PartialEq)]
enum Jednostka
{
    Sztuki(f64),
    Litry(f64),
    Kilogramy,
}

#[derive(Debug)]
enum Przechowywanie
{
    Zamrazarka,
    Chlodziarka,
    Normalne,
}

#[derive(Debug)]
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

fn main() {
    // Towar z jednostką Kilogramy (poprawny)
    let ziemniaki = Towar::new("Ziemniaki", Jednostka::Kilogramy, Przechowywanie::Normalne);
    match ziemniaki {
        Some(t) => println!("Poprawnie utworzono:\n{:?}\n", t),
        None => println!("Błąd przy tworzeniu ziemniaków\n"),
    }

    // Towar z jednostką Litry (poprawny)
    let mleko = Towar::new("Mleko 2%", Jednostka::Litry(1.05), Przechowywanie::Chlodziarka);
    match mleko {
        Some(t) => println!("Poprawnie utworzono:\n{:?}\n", t),
        None => println!("Błąd przy tworzeniu mleka\n"),
    }

    // Towar z jednostką Sztuki (ujemna waga)
    let puszka = Towar::new("Puszka kukurydzy", Jednostka::Sztuki(-0.25), Przechowywanie::Normalne);
    match puszka {
        Some(t) => println!("Poprawnie utworzono:\n{:?}\n", t),
        None => println!("❌ Nie udało się utworzyć puszki (ujemna waga)\n"),
    }

    let jablka = Towar::new("Jabłka ligol", Jednostka::Kilogramy, Przechowywanie::Normalne);
    match jablka
    {
        Some(t) =>  println!("Poprawnie utworzono:\n{:?}\n", t),
        None => println!("Błąd przy tworzeniu jablek\n"),
    }


}
