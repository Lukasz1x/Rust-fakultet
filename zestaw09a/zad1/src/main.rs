#[derive(PartialEq, PartialOrd)]
enum Karty
{
    Trefl,
    Karo,
    Kier,
    Pik
}

fn main() {
    println!("{}", Karty::Kier<Karty::Pik);
    println!("{}", Karty::Karo<Karty::Kier);
    println!("{}", Karty::Trefl<Karty::Karo);
    println!("{}", Karty::Kier>Karty::Pik);
    println!("{}", Karty::Karo>Karty::Kier);
    println!("{}", Karty::Trefl>Karty::Karo);
    println!("{}", Karty::Trefl==Karty::Pik);
    println!("{}", Karty::Kier==Karty::Kier);
}
