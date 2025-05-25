#[derive(Clone)]
struct Recenzja {
    uzytkownik: String,
    ocena: u8,
    komentarz: String
}

struct Ksiazka {
    tytul: String,
    autor: String,
    recenzje: Vec<Recenzja>
}

struct Biblioteka
{
    ksiazki: Vec<Ksiazka>
}


impl Recenzja
{
    fn new(uzytkownik: String, ocena: u8, komentarz: String) -> Self
    {
        Self
        {
            uzytkownik,
            ocena,
            komentarz,
        }
    }
}

impl Ksiazka
{
    fn new(tytul: String, autor: String, recenzje: Vec<Recenzja>) -> Self
    {
        Self
        {
            tytul,
            autor,
            recenzje
        }
    }

    fn srednia(&self) -> Option<f32>
    {
        if self.recenzje.len() < 1
        {
            return None;
        }
        let mut suma = 0.0;
        for r in &self.recenzje
        {
            suma += r.ocena as f32;
        }
        Some(suma/self.recenzje.len() as f32)
    }

    fn dodaj_recenzje(&mut self, recenzja: Recenzja)
    {
        self.recenzje.push(recenzja);
    }
}

impl Biblioteka {
    fn new() -> Self
    {
        Self
        {
            ksiazki: Vec::new()
        }
    }

    fn dodaj_ksiazke(&mut self, tytul: String, autor: String)
    {
        self.ksiazki.push(Ksiazka::new(tytul, autor, Vec::new()));
    }

    fn dodaj_recenzje(&mut self, tytul: & str, recenzja: Recenzja)
    {
        for r in &mut self.ksiazki
        {
            if r.tytul == tytul
            {
                r.dodaj_recenzje(recenzja.clone());
            }
        }
    }

    fn srednia_ocena(&self, tytul: & str) -> Option<f32>
    {
        for r in &self.ksiazki
        {
            if r.tytul == tytul
            {
                return r.srednia()
            }
        }
        None
    }

    fn najlepiej_oceniane(&self,liczba: usize) -> Vec<String>
    {
        let mut ksiazki :Vec<(String, f32)> = Vec::new();
        self.ksiazki.iter().for_each(|x| ksiazki.push(((*x).tytul.clone(), x.srednia().unwrap_or(0.0))));
        ksiazki.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
        ksiazki.iter().map(|(a,b)| a.clone()).rev().take(liczba).collect()

    }

    fn komentarze_dla_autora(&self, autor: & str) -> Vec<String>
    {
        let mut komentarze = Vec::new();
        for r in &self.ksiazki
        {
            if r.autor == autor
            {
                (*r).recenzje.iter().for_each(|x| komentarze.push((*x).komentarz.clone()));
            }
        }
        komentarze
    }
}

fn main() {
    let mut b = Biblioteka::new();
    // Dodawanie książek
    b.dodaj_ksiazke("Mistrz i Małgorzata".to_string(), "Bułhakow".to_string());
    b.dodaj_ksiazke("Zbrodnia i kara".to_string(), "Dostojewski".to_string());
    b.dodaj_ksiazke("Rok 1984".to_string(), "Orwell".to_string());
    b.dodaj_ksiazke("Proces".to_string(), "Kafka".to_string());
    b.dodaj_ksiazke("Lalka".to_string(), "Prus".to_string());

    // Dodawanie recenzji
    b.dodaj_recenzje("Mistrz i Małgorzata", Recenzja::new("Anna".to_string(), 5, "Arcydzieło!".to_string()));
    b.dodaj_recenzje("Mistrz i Małgorzata", Recenzja::new("Jan".to_string(), 4, "Trochę dziwne, ale dobre".to_string()));
    b.dodaj_recenzje("Zbrodnia i kara", Recenzja::new("Karol".to_string(), 5, "Poruszająca historia".to_string()));
    b.dodaj_recenzje("Zbrodnia i kara", Recenzja::new("Ala".to_string(), 4, "Ciężka, ale ważna książka".to_string()));
    b.dodaj_recenzje("Rok 1984", Recenzja::new("Monika".to_string(), 5, "Przerażająco aktualne".to_string()));
    b.dodaj_recenzje("Rok 1984", Recenzja::new("Bartek".to_string(), 5, "Klasyka dystopii".to_string()));
    b.dodaj_recenzje("Proces", Recenzja::new("Tomek".to_string(), 3, "Nie do końca zrozumiałem".to_string()));
    b.dodaj_recenzje("Lalka", Recenzja::new("Kasia".to_string(), 4, "Długa, ale wciągająca".to_string()));
    b.dodaj_recenzje("Lalka", Recenzja::new("Ola".to_string(), 3, "Nie moja bajka".to_string()));

    println!("Top 3: {:?}", b.najlepiej_oceniane(3));
    println!("Komentarze dla Prusa: {:?}", b.komentarze_dla_autora("Prus"));
    println!("Średnia ocena \"Lalki\": {:?}", b.srednia_ocena("Lalka").unwrap_or(0.0));
}
