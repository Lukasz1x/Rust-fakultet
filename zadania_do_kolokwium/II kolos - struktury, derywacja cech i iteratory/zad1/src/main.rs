use std::cmp::min;
use std::collections::HashMap;
use std::ptr::hash;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Ocena {
    A,
    B,
    C,
    D,
    E,
    F
}

#[derive(Debug)]
struct Student {
    imie: String,
    nazwisko: String,
    oceny: Vec<Ocena>
}

#[derive(Debug)]
struct Dziennik {
    studenci: Vec<Student>
}

impl Student
{
    fn new(imie: String, nazwisko: String, oceny: Vec<Ocena>) -> Self
    {
        Self{
            imie,
            nazwisko,
            oceny
        }
    }

    fn srednia(&self) -> Option<f32>
    {
        if self.oceny.len()==0
        {
            None
        }else {
            Some(self.oceny.iter().map(|a| match a {
                Ocena::A => 5.0,
                Ocena::B => 4.0,
                Ocena::C => 3.0,
                Ocena::D => 2.0,
                Ocena::E => 1.0,
                Ocena::F => 0.0
            }).sum::<f32>() / self.oceny.len() as f32)
        }
    }
}

impl Dziennik
{
    fn new() -> Self
    {
        Self{
            studenci: Vec::new()
        }
    }

    fn dodaj_studenta(&mut self, imie: String, nazwisko: String)
    {
        self.studenci.push(Student::new(imie, nazwisko, Vec::new()));
    }

    fn dodaj_ocene(&mut self, nazwisko: &str, ocena: Ocena)
    {
        for s in &mut self.studenci
        {
            if s.nazwisko == nazwisko
            {
                (*s).oceny.push(ocena.clone());
            }
        }
    }

    fn srednia_ocen(&self, nazwisko: &str) -> Option<f32>
    {
        for s in &self.studenci
        {
            if s.nazwisko == nazwisko
            {
                return s.srednia();
            }
        }
        None
    }

    fn najlepsi_studenci(&self, min_avg: f32) -> Vec<String>
    {
        let mut nazwiska = Vec::new();
        for s in &self.studenci
        {
            if let srednia = s.srednia().unwrap_or(0.0)
            {
                if srednia > min_avg
                {
                    nazwiska.push(s.nazwisko.clone());
                }
            }
        }
        nazwiska
    }

    fn statystyka_ogolna_ocen(&self) -> HashMap<Ocena, usize>
    {
        let mut hashmapa = HashMap::new();
        for s in &self.studenci
        {
            for o in &s.oceny
            {
                *hashmapa.entry((*o).clone()).or_insert(0) += 1;
            }
        }
        hashmapa
    }
}


fn main() {
    let mut d = Dziennik::new();
    d.dodaj_studenta("Marcin".to_string(), "Kowalski".to_string());
    d.dodaj_ocene("Kowalski", Ocena::A);
    d.dodaj_ocene("Kowalski", Ocena::B);

    d.dodaj_studenta("Anna".to_string(), "Nowak".to_string());
    d.dodaj_ocene("Nowak", Ocena::A);
    d.dodaj_ocene("Nowak", Ocena::C);

    d.dodaj_studenta("Tomasz".to_string(), "Zieliński".to_string());
    d.dodaj_ocene("Zieliński", Ocena::B);
    d.dodaj_ocene("Zieliński", Ocena::C);
    d.dodaj_ocene("Zieliński", Ocena::C);

    d.dodaj_studenta("Ewa".to_string(), "Wiśniewska".to_string());
    d.dodaj_ocene("Wiśniewska", Ocena::A);
    d.dodaj_ocene("Wiśniewska", Ocena::A);
    d.dodaj_ocene("Wiśniewska", Ocena::B);

    d.dodaj_studenta("Jan".to_string(), "Wójcik".to_string());
    d.dodaj_ocene("Wójcik", Ocena::D);
    d.dodaj_ocene("Wójcik", Ocena::E);

    d.dodaj_studenta("Katarzyna".to_string(), "Kamińska".to_string());
    d.dodaj_ocene("Kamińska", Ocena::A);
    d.dodaj_ocene("Kamińska", Ocena::B);
    d.dodaj_ocene("Kamińska", Ocena::C);
    d.dodaj_ocene("Kamińska", Ocena::F);

    d.dodaj_studenta("Piotr".to_string(), "Lewandowski".to_string());
    d.dodaj_ocene("Lewandowski", Ocena::B);
    d.dodaj_ocene("Lewandowski", Ocena::C);

    d.dodaj_studenta("Magdalena".to_string(), "Dąbrowska".to_string());
    d.dodaj_ocene("Dąbrowska", Ocena::A);
    d.dodaj_ocene("Dąbrowska", Ocena::B);
    d.dodaj_ocene("Dąbrowska", Ocena::C);
    d.dodaj_ocene("Dąbrowska", Ocena::D);

    d.dodaj_studenta("Andrzej".to_string(), "Szymański".to_string());
    d.dodaj_ocene("Szymański", Ocena::E);
    d.dodaj_ocene("Szymański", Ocena::F);

    println!("Średnia ocen dla Magdaleny Dąbrowskiej: {}", d.srednia_ocen("Dąbrowska").unwrap_or(0.0));
    println!("Najlepsi uczniowie: {:?}", d.najlepsi_studenci(4.0));
    println!("Statystyka ogólna ocen: {:?}", d.statystyka_ogolna_ocen());

}
