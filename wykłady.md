# Spis treści:
- [Wykład 1](#Wykład-1)
- [Wykład 2](#Wykład-2)
- [Wykład 3](#Wykład-3)

# Wykład 1

www.rust-lang.org

hasło: kaktus

cechy Rusta:
- programowanie systemowe -- zwykłe C lub C++
- brak odśmiecania (garbage collectora) (python i java mają)
- wysoka wydajność - porównywalna z C i C++
- bezpieczeństwo:
    - kontrolowane zarządzanie pamięcią
        - bardzo często na etapie kompilacji
        - pożyczanie (borrow checker)
    - unikanie ***UB*** (undefined behaviour)
- ścisłość
- wymagający kompilator
- jawność intencji 
- zasada najmniejszego zaskoczenia
- wymagający kompilator -- ale przyjazny
- bardzo silne (brak niejawnej zamiany typów) i statyczne typowanie (typy zmiennych znane na etapie kompilacji)
    - im silniejsze typowanie w języku tym więcej błędów zostanie wykrytych na etapie kompilacji
- wnioskowanie typów
- domyślna niemutowalność danych (nie ma zmiennych tylko są stałe)
- programowanie strukturalne
- programowanie funkcyjne 
- programowanie obiektowe -- bez dziedziczenia
    - (ale z interfejsami)

otoczenie:
- środowisko budowania cargo
- menedżer pakietów
- inne narzędzia
- dokumentacja


trzy języki:
- bezpieczny Rust
- niebezpieczny Rust
- makra Rusta (automatyzacje, które wykonują się podczas kompilacji)

#### Najprostszy program:
```rs
fn main(){
    println!("Hello, world!");
}
```
# Wykład 2
Funkcje są potrzebne do programowania strukturalnego.

`println!()` zapisujemy z wykrzyknikiem, ponieważ jest to makro (a nie prawdziwa funkcja). Makra da się przeciążać a funkcji nie.

Wnioskowanie o typie nie polega na tym że przypisany jest domyślny, tylko jest to ustalane przez wnioskowanie na ostatnim etapie kompilacji.
Domyślne typy `i32`, `f64`

Spójność typów jest wymagana. Konwersji można dokonać w następujący sposób:
```rs
let a :i32 = 10;
let b :f64 = 20.5;
println!("{}", a as f64 + b);

let a = 10;
let b = 20;
println!("{}", (a<b) as i32 * 30); 
```
Brak operator trójargumentowego, ale jego funkcjonalnośc można uzyskać w następujący sposób:
```rs
let a = 10;
let b = 20;
let x = if a<b {30} else {0};
```
Nawiasy klamrowe przyjmują wartość ostatniej wartości po ***ostatnim średniku*** (jak nie ma średnika to jest zwracane wsystko co jest w nawiasch klamrowych). 
Dla poniższego przykładu zwracay jest `y` lub `0`.
```rs
let x = if a<b {let y = 30; y} else {0}
```
Tryb debugerski
```rs
let a = 'x';
println!("{}, a")       //output: x
println!("{:?}, a")     //output: 'x'
```

```rs
let a = 'ń';                //typ znakowy (char) ma 4 bajty
println!("{}", a)           //output: ń
println!("{:?}", a)         //output: 'ń'
println!("{}", a as u32)    //outpyt: 324
```
Sposoby zapisywania liczb:
```rs
let a  = 1_00_003;
let b = 0xfa;
let c = 0o721;
let d =0b0011_1010;
let e =b'a';  //not sure czy tak
```
Pętla nieskończona:
```rs
loop{
    println!("x");
}
```

Pętla `loop` musi się wykonać raz lub do pierwszej instrukcji `break`, dlatego dozwolone jest zwracanie wartości przez `break`,natomiast pętla `while` może wcale się nie wywołać dlatego w niej nie jest dopuszczony `break` ze zwracaniem.
# ***uzupełnic kod ze zdjęć***

#### Funkcje

```rs
fn powiekszon_o_1_v1(x :i32) -> i32
{
    x+1  //można pisać return x+1;
}

fn powiekszon_o_1_v2(mut x :i32) -> i32
{
    x+=1;
    x
}

fn powiekszon_o_1_v2(x :&mut i32) //referencja do mutowalnej zmiennej
{
    *x+=1;
}

fn main() // ->()  main zwraca wartość pustą
{
    let a = 12;
    let b = powiekszon_o_1_v1(a);
    println!("{}", a==12);  //true
    println!("{}", b==13);  //true
    let c = powiekszon_o_1_v2(a);
    println!("{}", a==12);  //true
    println!("{}", c==13);  //true

    powiekszon_o_1(&mut a);
    println!("{}", a==13);  //true
    powiekszon_o_1(&mut a);
    println!("{}", a==14);  //true
}
```

# Wykład 3
cooming soon...



