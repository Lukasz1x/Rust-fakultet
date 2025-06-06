use zad3::urna::Urna;
use zad3::randgen::RandGen;

fn main() {
    println!("Napisy");

    let mut urna = Urna::<&str>::new(RandGen::new(123));

    let a: Option<&str> = urna.losuj_z_us();
    println!("{:?}", a.is_none());
    let a: Option<&str> = urna.losuj_bez_us();
    println!("{:?}", a.is_none());


    urna.doloz(&"Napis1");
    urna.doloz(&"Napis2");
    urna.doloz(&"Napis3");
    urna.doloz(&"Napis4");

    println!("{:?}", urna.rozmiar() == 4);
    let y: Option<&str> = urna.losuj_z_us();
    println!("{:?}", y.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    let x: Option<&str> = urna.losuj_bez_us();
    println!("{:?}", x.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    println!("{:?}", x != y);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 2);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 1);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 0);
    let z: Option<&str> = urna.losuj_z_us();
    println!("{:?}", z.is_none());
    println!("{:?}", urna.rozmiar() == 0);
}