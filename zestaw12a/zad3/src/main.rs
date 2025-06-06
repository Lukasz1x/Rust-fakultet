use zad3::urna::Urna;
use zad3::randgen::RandGen;

#[derive(PartialEq, Copy, Clone)]
enum Moneta
{
    Orzel,
    Reszka,
}




fn main() {
    println!("Litery");
    let mut urna = Urna::<char>::new(RandGen::new(123));

    let a: Option<char> = urna.losuj_z_us();
    println!("{:?}", a.is_none());
    let a: Option<char> = urna.losuj_bez_us();
    println!("{:?}", a.is_none());


    urna.doloz('a');
    urna.doloz('b');
    urna.doloz('c');
    urna.doloz('d');

    println!("{:?}", urna.rozmiar() == 4);
    let y: Option<char> = urna.losuj_z_us();
    println!("{:?}", y.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    let x: Option<char> = urna.losuj_bez_us();
    println!("{:?}", x.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    println!("{:?}", x != y);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 2);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 1);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 0);
    let z: Option<char> = urna.losuj_z_us();
    println!("{:?}", z.is_none());
    println!("{:?}", urna.rozmiar() == 0);

    println!("+++++++++++++++++++++++++++++++++");
    println!("Liczby");

    let mut urna = Urna::<i32>::new(RandGen::new(123));

    let a: Option<i32> = urna.losuj_z_us();
    println!("{:?}", a.is_none());
    let a: Option<i32> = urna.losuj_bez_us();
    println!("{:?}", a.is_none());


    urna.doloz(1);
    urna.doloz(2);
    urna.doloz(3);
    urna.doloz(4);

    println!("{:?}", urna.rozmiar() == 4);
    let y: Option<i32> = urna.losuj_z_us();
    println!("{:?}", y.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    let x: Option<i32> = urna.losuj_bez_us();
    println!("{:?}", x.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    println!("{:?}", x != y);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 2);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 1);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 0);
    let z: Option<i32> = urna.losuj_z_us();
    println!("{:?}", z.is_none());
    println!("{:?}", urna.rozmiar() == 0);

    println!("+++++++++++++++++++++++++++++++++");
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

    println!("+++++++++++++++++++++++++++++++++");
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

    println!("+++++++++++++++++++++++++++++++++");
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