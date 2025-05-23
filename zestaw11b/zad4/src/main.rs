#[derive(Eq, PartialEq, Debug)]
enum Klasa
{
    ekonomiczna,
    biznesowa,
    pierwsza
}

#[derive(Eq, PartialEq, Debug)]
enum Status
{
    zarezerwowane,
    odwolane,
}

#[derive(Debug)]
struct Rezerwacja
{
    pasazer: String,
    numer_lotu: String,
    klasa_podrozy: Klasa,
    status: Status
}

impl Rezerwacja
{
    fn new(pasazer: String, numer_lotu: String, klasa_podrozy: Klasa, status: Status) -> Self
    {
        Self
        {
            pasazer,
            numer_lotu,
            klasa_podrozy,
            status
        }
    }

    fn odwolaj(&mut self)
    {
        self.status = Status::odwolane;
    }
}

#[derive(Debug)]
struct SystemRezerwacji
{
    rezerwacje: Vec<Rezerwacja>
}

impl SystemRezerwacji
{
    fn new() -> Self
    {
        Self
        {
            rezerwacje: Vec::new()
        }
    }

    fn dodaj_rezerwacje(&mut self, rezerwacja: Rezerwacja)
    {
        self.rezerwacje.push(rezerwacja);
    }

    fn odwolaj_na_podstawie_nazwiska(&mut self, nazwisko: String)
    {
        for r in &mut self.rezerwacje
        {
            if r.pasazer == nazwisko
            {
                r.odwolaj();
            }
        }
    }

    fn zlicz_na_podstawie_klasy_i_statusu(&self, status: Status, klasa: Klasa) -> usize
    {
        let mut ile = 0;
        for i in &self.rezerwacje
        {
            if i.klasa_podrozy == klasa && i.status == status
            {
                ile+=1;
            }
        }
        ile
    }

    fn wylistuj_pasazerow_w_klasie(&self, klasa: Klasa)
    {
        let mut pasazerowie = Vec::new();
        for i in &self.rezerwacje
        {
            if i.klasa_podrozy == klasa
            {
                pasazerowie.push((*i).pasazer.clone());
            }
        }
        pasazerowie.sort();
        println!("Pasazerowie w klasie {:?}:", klasa);
        for p in pasazerowie
        {
            println!("{}", p);
        }
    }
}



fn main() {
    let mut system = SystemRezerwacji::new();
    system.dodaj_rezerwacje(Rezerwacja::new("Wójcik".to_string(), "MH370".to_string(), Klasa::pierwsza, Status::odwolane));
    system.dodaj_rezerwacje(Rezerwacja::new("Kowalski".to_string(), "LO123".to_string(), Klasa::ekonomiczna, Status::zarezerwowane));
    system.dodaj_rezerwacje(Rezerwacja::new("Nowak".to_string(), "FR456".to_string(), Klasa::biznesowa, Status::zarezerwowane));
    system.dodaj_rezerwacje(Rezerwacja::new("Wiśniewska".to_string(), "LH789".to_string(), Klasa::pierwsza, Status::zarezerwowane));
    system.dodaj_rezerwacje(Rezerwacja::new("Dąbrowski".to_string(), "BA101".to_string(), Klasa::ekonomiczna, Status::odwolane));
    system.dodaj_rezerwacje(Rezerwacja::new("Zieliński".to_string(), "AF202".to_string(), Klasa::biznesowa, Status::zarezerwowane));
    system.dodaj_rezerwacje(Rezerwacja::new("Kamińska".to_string(), "LO123".to_string(), Klasa::ekonomiczna, Status::zarezerwowane));
    system.dodaj_rezerwacje(Rezerwacja::new("Lewandowski".to_string(), "FR456".to_string(), Klasa::biznesowa, Status::zarezerwowane));
    system.dodaj_rezerwacje(Rezerwacja::new("Krawczyk".to_string(), "LH789".to_string(), Klasa::pierwsza, Status::odwolane));
    system.dodaj_rezerwacje(Rezerwacja::new("Mazur".to_string(), "BA101".to_string(), Klasa::ekonomiczna, Status::zarezerwowane));
    system.odwolaj_na_podstawie_nazwiska("Lewandowski".to_string());
    println!("{}", system.zlicz_na_podstawie_klasy_i_statusu(Status::zarezerwowane, Klasa::ekonomiczna));
    system.wylistuj_pasazerow_w_klasie(Klasa::pierwsza);
    println!("\n\n{:?}", system);


}