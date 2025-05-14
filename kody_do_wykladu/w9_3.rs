enum Pionek {
    Kolko,
    Krzyzyk,
}

enum Pole {
    Puste,
    Zajete(Pionek),
}

struct Plansza(Vec<Vec<Pole>>);

struct Gra {
    plansza: Plansza,
    interfejs: InterfejsTekstowy,
    gracze: Vec<GraczCzlowiek>,
    indeks_biezacego_gracza: usize,
}

struct GraczCzlowiek {
    pionek: Pionek,
    imie: String,
}

struct InterfejsTekstowy;

////////// main

fn main() -> Result<(), String> {
    let interfejs = InterfejsTekstowy;
    let ustawienia = interfejs.pobierz_ustawienia();
    let gra = Gra::new(
        ustawienia.utworz_gracza_o(),
        ustawienia.utworz_gracza_x(),
        ustawienia.pionek_rozpoczynajacy,
        interfejs,
    );
    gra?.graj();
    Ok(())
}
