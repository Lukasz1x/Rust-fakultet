# Zestaw 8a
1. Oprogramuj typ:
```rs
struct RandGen {
    seed: i64
}
```
tak by działało (i wyświetlało za każdym razem `true`):
```rs
fn main() {
    let mut generator1 = RandGen::new(123);
    let a = generator1.gen_range(3, 15);
    let b = generator1.gen_range(3, 15);
    let c = generator1.gen_range(3, 15);

    let mut generator2 = RandGen::new(123);
    let a2 = generator2.gen_range(3, 15);
    let b2 = generator2.gen_range(3, 15);
    let c2 = generator2.gen_range(3, 15);

    println!("{}", a == a2);
    println!("{}", b == b2);
    println!("{}", c == c2);

    println!("{}", a >= 3);
    println!("{}", b >= 3);
    println!("{}", c >= 3);

    println!("{}", a <= 15);
    println!("{}", b <= 15);
    println!("{}", c <= 15);
}
```
2. Stwórz typ `Urna`, który będzie mógł przechowywać znaki i losować z nich (za pomocą naszego generatora). Poniższe testy powinny dawać za każdym razem true.
```rs
fn main() {
    let mut urna = Urna::new(RandGen::new(123));

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
}
```