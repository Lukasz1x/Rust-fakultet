use zad3::urna::Urna;
use zad3::randgen::RandGen;

fn main() {
    println!("Boole");

    let mut urna = Urna::<bool>::new(RandGen::new(123));

    let a: Option<bool> = urna.losuj_z_us();
    println!("{:?}", a.is_none());
    let a: Option<bool> = urna.losuj_bez_us();
    println!("{:?}", a.is_none());


    urna.doloz(true);
    urna.doloz(false);
    urna.doloz(false);
    urna.doloz(true);

    println!("{:?}", urna.rozmiar() == 4);
    let y: Option<bool> = urna.losuj_z_us();
    println!("{:?}", y.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    let x: Option<bool> = urna.losuj_bez_us();
    println!("{:?}", x.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    println!("{:?}", x != y);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 2);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 1);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 0);
    let z: Option<bool> = urna.losuj_z_us();
    println!("{:?}", z.is_none());
    println!("{:?}", urna.rozmiar() == 0);
}