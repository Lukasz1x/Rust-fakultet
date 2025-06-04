use kik600::gra::Gra;
use kik600::interfejs_tekstowy::InterfejsTekstowy;

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
