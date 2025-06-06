# Zestaw 12a
1. Stwórz typ generyczny `Urna` (na bazie konkretnego z Zestawu 8a), który będzie mógł przechowywać dane dowolnego (w miarę możliwości) typu i losować z nich (za pomocą naszego generatora z Zestawu 8a). Poniższe testy powinny dawać za każdym razem `true`.
```rs
fn main() {
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
}
```
2. Dopisz analogioczne testy dla typów konkretnych:
- `Urna<i32>`
- `Urna<bool>`
- `Urna<String>`
- `Urna<Moneta>`, gdzie `Moneta` jest typem własnym wyliczeniowym o dwóch wartościach (`Orzel` oraz `Reszka`)
3. Utwórz aplikację wielomodułową -- umieszczając każdy z typów `RandGen` oraz `Urna` w osobnym module/pliku, a każdy z tetsów w osobnej binarce.