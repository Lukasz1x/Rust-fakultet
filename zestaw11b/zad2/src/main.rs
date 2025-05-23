#[derive(Debug, Eq, PartialEq)]
enum Status
{
    Otwarte,
    Przetwarzane,
    Zamkniete(Rezultat)
}

#[derive(Debug, Eq, PartialEq)]
enum Rezultat
{
    Rozwiazane,
    Odrzucone,
    Nieaktualne,
    BrakDanych,
    Inne(String)
}

#[derive(Debug, Eq, PartialEq)]
struct Zgloszenie
{
    id: usize,
    tytul: String,
    status: Status
}

impl Zgloszenie
{
    fn new(id: &mut usize, tytul: String, status: Status) -> Self
    {
        (*id)+=1;
        Self
        {
            id: (*id)-1,
            tytul,
            status
        }
    }

    fn zmien_status(&mut self, nowy_status: Status)
    {
        self.status = nowy_status;
    }
}

fn wypisz_zgloszenia_o_statusie(zgloszenia: &Vec<Zgloszenie>, filtr: fn(&Status) -> bool)
{
    for z in zgloszenia
    {
        if filtr(&(*z).status)
        {
            println!("{:?}", *z);
        }
    }
}

fn main() {
    let mut next_id = 1;
    let mut zgloszenia = Vec::new();
    zgloszenia.push(Zgloszenie::new(&mut next_id, "test".to_string(), Status::Zamkniete(Rezultat::Rozwiazane)));
    zgloszenia.push(Zgloszenie::new(&mut next_id, "Nie działa drukarka".to_string(), Status::Otwarte));
    zgloszenia.push(Zgloszenie::new(&mut next_id, "Brak dostępu do internetu".to_string(), Status::Przetwarzane));
    zgloszenia.push(Zgloszenie::new(&mut next_id, "Problem z logowaniem do systemu".to_string(), Status::Zamkniete(Rezultat::Rozwiazane)));
    zgloszenia.push(Zgloszenie::new(&mut next_id, "Zgłoszenie testowe".to_string(), Status::Zamkniete(Rezultat::Nieaktualne)));
    zgloszenia.push(Zgloszenie::new(&mut next_id, "Wniosek o nowe konto".to_string(), Status::Zamkniete(Rezultat::Odrzucone)));
    zgloszenia.push(Zgloszenie::new(&mut next_id, "Zapytanie o aktualizację oprogramowania".to_string(), Status::Zamkniete(Rezultat::BrakDanych)));
    zgloszenia.push(Zgloszenie::new(&mut next_id, "Prośba o instalację nowego pakietu Office".to_string(), Status::Otwarte));
    zgloszenia.push(Zgloszenie::new(&mut next_id, "Komputer się nie włącza".to_string(), Status::Zamkniete(Rezultat::Inne("Sprzęt do wymiany".to_string()))));
    zgloszenia.push(Zgloszenie::new(&mut next_id, "Zgłoszenie duplikatu".to_string(), Status::Zamkniete(Rezultat::Inne("Duplikat innego zgłoszenia".to_string()))));

    zgloszenia[2].zmien_status(Status::Zamkniete(Rezultat::Rozwiazane));

    println!("{:?}", zgloszenia);
    println!();
    wypisz_zgloszenia_o_statusie(&zgloszenia, |s| matches!(s, Status::Zamkniete(_)));


}
