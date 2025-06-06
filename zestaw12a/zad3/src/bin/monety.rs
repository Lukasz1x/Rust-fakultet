use zad3::urna::Urna;
use zad3::randgen::RandGen;

#[derive(PartialEq, Copy, Clone)]
enum Moneta
{
    Orzel,
    Reszka,
}

fn main() {
    println!("Monety");

    let mut urna = Urna::<Moneta>::new(RandGen::new(123));

    let a: Option<Moneta> = urna.losuj_z_us();
    println!("{:?}", a.is_none());
    let a: Option<Moneta> = urna.losuj_bez_us();
    println!("{:?}", a.is_none());


    urna.doloz(Moneta::Orzel);
    urna.doloz(Moneta::Reszka);
    urna.doloz(Moneta::Orzel);
    urna.doloz(Moneta::Reszka);

    println!("{:?}", urna.rozmiar() == 4);
    let y: Option<Moneta> = urna.losuj_z_us();
    println!("{:?}", y.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    let x: Option<Moneta> = urna.losuj_bez_us();
    println!("{:?}", x.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    println!("{:?}", x != y);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 2);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 1);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 0);
    let z: Option<Moneta> = urna.losuj_z_us();
    println!("{:?}", z.is_none());
    println!("{:?}", urna.rozmiar() == 0);
}