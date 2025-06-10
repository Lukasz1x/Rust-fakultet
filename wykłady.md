Orginalny plik zawiera kolory, których nie widać na podglądzie na Githubie, więc warto go pobrać i otworzyć w czymś lepszym.
# Spis treści:
- [Wykład 1](#wykład-1)
- [Wykład 2](#wykład-2)
- [Wykład 3](#wykład-3)
- [Wykład 4](#wykład-4)
- [Wykład 5](#wykład-5)
- [Wykład 6](#wykład-6)
- [Wykład 7](#wykład-7)
- [Wykład 8](#wykład-8)
- [Wykład 9](#wykład-9)
- [Wykład 10](#wykład-10)
- [Wykład 11](#wykład-11)
- [Wykład 12](#wykład-12)
- [Wykład 13](#wykład-13)

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
Nawiasy klamrowe przyjmują wartość ostatniej wartości po ***ostatnim średniku*** (jak nie ma średnika to jest zwracane wszystko co jest w nawiasch klamrowych). 
Dla poniższego przykładu zwracay jest `y` lub `0`.
```rs
let x = if a<b {let y = 30; y} else {0}
```
Tryb debugerski
```rs
let a = 'x';
println!("{}", a)           // output: x
println!("{:?}", a)         // output: 'x'
```

```rs
let a = 'ń';                // typ znakowy (char) ma 4 bajty
println!("{}", a)           // output: ń
println!("{:?}", a)         // output: 'ń'
println!("{}", a as u32)    // output: 324
```
Sposoby zapisywania liczb:
```rs
let a = 1_000_003;          // 1000003
let b = 0xfa;               // 250 
let c = 0o721;              // 465
let d = 0b0011_1010;        // 58
let e = b'a';               // 97 - pod e zostanie przypisana wartość litery 'a' z tabeli ASCII
```
Pętla nieskończona:
```rs
loop{
    println!("x");
}
```

Pętla `loop` musi się wykonać raz lub do pierwszej instrukcji `break`, dlatego dozwolone jest zwracanie wartości przez `break`, natomiast pętla `while` może wcale się nie wywołać (warunek nigdy nie jest spełniony, np. `1 > 2`), dlatego w niej nie jest dopuszczony `break` ze zwracaniem.

```rs
let mut i = 0;
let x = while i < 10 {
    print!("{i} ");
    i+=1;
};
print!("{x:?}");            // output: 0 1 2 3 4 5 6 7 8 9 ()
```
```rs
let mut i = 0;
let x = loop {
    if i >= 10 {
        break;
    }
    print!("{i} ");
    i+=1;
};
print!("{x:?}");            // output: 0 1 2 3 4 5 6 7 8 9 ()
```
```rs
let mut i = 0;
let x = loop {
    if i >= 10 {
        break 999;          // break może zwracać wartość tylko w przypadku pętli loop
    }
    print!("{i} ");
    i+=1;
};
print!("{x:?}");            // output: 0 1 2 3 4 5 6 7 8 9 999
```
#### Funkcje

```rs
fn powiekszona_o_1_v1(x :i32) -> i32
{
    x+1                     // można pisać return x+1;
}

fn powiekszona_o_1_v2(mut x :i32) -> i32
{
    x+=1;
    x
}

fn powiekszona_o_1(x :&mut i32) //referencja do mutowalnej zmiennej
{
    *x+=1;
}

fn main() // ->()  main zwraca wartość pustą
{
    let mut a = 12;
    let b = powiekszona_o_1_v1(a);
    println!("{}", a==12);  //true
    println!("{}", b==13);  //true
    let c = powiekszona_o_1_v2(a);
    println!("{}", a==12);  //true
    println!("{}", c==13);  //true

    powiekszona_o_1(&mut a);
    println!("{}", a==13);  //true
    powiekszona_o_1(&mut a);
    println!("{}", a==14);  //true
}
```

# Wykład 3
Przekazywanie parametru:
- na własność (przez wartość)
    - przez kopiowanie
    - przez przeniesienie
- przez pożyczkę/referencję `&`
- przez pożyczkę/referencję mutowalną `&mut`

| Deklaracja | Zmienna mutowalna? | Referencja mutowalna? | Mozna zmieniac zawartosc? | Mozna zmieniac referencje? | Na chłopski rozum |
|--------------------|--------------------|------------------------|----------------------------|-----------------------------|--
| `x: &i32` | ❌ NIE | ❌ NIE | ❌ NIE | ❌ NIE | stały wskaźnik na stałą
| `mut x: &i32` | ✅ TAK | ❌ NIE | ❌ NIE | ✅ TAK | zmienny wskaźnik na stałą (zmienny, ponieważ w trakcie działania programu można zmienić na co wskazuje)
| `x: &mut i32` | ❌ NIE | ✅ TAK | ✅ TAK | ❌ NIE | stały wskaźnik na zmienną (wskaźnik cały czas wskazuje na jedną zmienną, ale przez dereferencję `*` można zmienić wartość tej zmiennej)
| `mut x: &mut i32` | ✅ TAK | ✅ TAK | ✅ TAK | ✅ TAK | zmienny wskaźnik na zmienną (czyli można zmienić na co wskazuje i można zmienić wartość tej zmiennej)
```rs
let a = 5;
let x: &i32 = &a;   // &a - ampersand, żeby móc przypisać adres a pod x
println!("{}", *x); // out: 5
// 🚨 w wypisywaniu działa bez gwiazdki, ale nie jest to do końca poprawne więc lepiej dać tę gwiazdkę


let b = 6;
let mut y: &i32 = &b;
println!("{}", *y); // out: 6
y = x;
println!("{}", *y); // out: 5
y = &b;
println!("{}", *y); // out: 6


let mut c = 7;
let z: &mut i32 = &mut c; // przy mutowalnej referencji trzeba to podkreślić i dopisać mut (wszystko w Rust musi być intencjonalne i jawne, a nie przypadkowe)
println!("{}", *z); // out: 7
*z += 1;
println!("{}", *z); // out: 8


let mut w: &mut i32 = &mut c;
println!("{}", *w); // out: 8
*w += 1;
println!("{}", *w); // out: 9
let mut d = 10;
w = &mut d;
println!("{}", *w); // out: 10
*w += 1;
println!("{}", *w); // out: 11
```

```rs
fn swap(x: &mut i32, y: &mut i32)
{
    let pom = *x;
    *x=*y;
    *y=pom;
}

fn main()
{
    let mut a=10;
    let mut b=20;
    swap(&mut a, &mut b);
    dbg!(a);
    dbg!(b);

    //swap(&mut a, &mut a);  //nie można pożyczyć tej samej rzeczy 2 razy
    //dbg!(a);
}
```
📌 Analiza funkcji `swap`:
- `x: &mut i32` i `y: &mut i32` to mutowalne referencje (`&mut`), które pozwalają na zmianę wartości zmiennych przekazanych do funkcji.
- `*x` oraz `*y` oznacza dereferencję, czyli dostęp do wartości pod wskaźnikiem.
- Zmienna pomocnicza `pom` pozwala przechować tymczasowo wartość x przed nadpisaniem.

📌 Dlaczego `swap(&mut a, &mut a);` jest błędne?
- Rust nie pozwala na więcej niż jedną mutowalną referencję do tej samej zmiennej w tym samym miejscu w kodzie.
- Gdyby było to dozwolone, funkcja swap zmieniłaby tę samą wartość dwa razy, co mogłoby prowadzić do nieokreślonego zachowania w innych językach (np. w C++ mogłoby się udać, ale wynik byłby nieprzewidywalny).
```rs
fn powitaj_v1(imie: &String) {
    println!("Witaj, {imie}!")
}

fn powitaj_v2(imie: str) {
    println!("Witaj, {imie}!")
}

fn powitaj_v3(imie: &str) {
    println!("Witaj, {imie}!")
}

fn main() {
    powitaj_v1("Edek"); // nie zadziała, bo Edek jest &str, a String to struct
    powitaj_v2("Edek"); // nie zadziała, bo Edek jest &str, a funkcja przyjmuje str, jest tu jakiś problem, że str ma rozmiar nieznany podczas kompilacji??
    powitaj_v3("Edek"); // zadziała
}
```
### Analiza wersji funkcji powitalnych
❌ `powitaj_v1(imie: &String)`
- **Błąd:** Oczekuje referencji `&String`, czyli obiektu typu `String` przechowywanego na stercie.
- **Problem:** `"Edek"` jest typu `&str`, a nie `String`, więc Rust nie dokona automatycznej konwersji z `&str` do `&String`.
- **Rozwiązanie:** Można przekazać referencję do `String`, np. `powitaj_v1(&String::from("Edek"))`.

❌ `powitaj_v2(imie: str)`
- **Błąd:** `str` to niekompletny typ (tzw. unsized type, DST – Dynamically Sized Type).
- **Problem:** `str` nie ma określonego rozmiaru w czasie kompilacji, więc Rust nie wie, ile pamięci zaalokować.
- **Poprawne podejście:** Używa się `&str`, czyli referencji do ciągu znaków.

✅ `powitaj_v3(imie: &str)`
- Oczekuje `&str`, czyli referencji do ciągu znaków, co jest zgodne z `"Edek"` (`&str`).
- Działa poprawnie, ponieważ Rust automatycznie używa "Edek" jako `&str`.

📌 Wskazówki
- Aby akceptować zarówno `String`, jak i `&str`, najlepszą opcją jest `&str`, bo `String` można przekazać jako `&str` za pomocą `&my_string` lub `my_string.as_str()`.
- `String` jest przekazywany na własność, ale `&str` jest lekką referencją, co czyni je bardziej uniwersalnym wyborem dla funkcji akceptujących tekst.

```rs
fn powitaj_v1(imie: &str) // używanie &str jest bardziej użytecznie w nagłówku funkcji niż &String
{
    println!("Witaj, {imie}!")
}

fn powitaj_v2(imie: String) {
    println!("Witaj, {imie}!")
}

fn powitaj_v3(imie: &String) {
    println!("Witaj, {imie}!")
}


// wszystko działa
fn main() { 
    powitaj_v1("Edek");
    let imie1 = "Felek".to_string();
    let imie2 = String::from("Balbina");
    powitaj_v3(&imie1);
    powitaj_v2(imie2.clone()); // String jest potencjalnie bardzo dużą wartością, więc nie ma kopiowania stringów, żeby programista zrobił to sam pisząc .clone()
    powitaj_v1(&imie1); // dozwolona jest niejawna (automatyczna) konwersja z &String na &str
    powitaj_v1(&imie2); // to samo co wyżej
}
```
- `String` nie implementuje `Copy`, więc jego przekazanie do funkcji przenosi własność.
- `&String` można przekonwertować na `&str` (Rust automatycznie dokona dereferencji).
- Kopiowanie String wymaga jawnego użycia `.clone()`, ponieważ kopiowanie dużych obiektów mogłoby być kosztowne.

📌 Dlaczego `&str` jest lepsze niż `&String`?

Jeśli funkcja ma przyjmować tekst, lepiej używać `&str` niż `&String`, ponieważ:

✅ `&str` akceptuje zarówno `String`, jak i zwykłe literały (`"tekst"`)\
✅ `&String` działa tylko dla `String`, więc nie przyjmie `&str`

```rs
fn powitaj_v0(tab: [i32; 4]) { // bez & musi być rozmiar
    println!("Witaj, {tab:?}!")
}

fn powitaj_v1(tab: &[i32]) {
    println!("Witaj, {tab:?}!")
}

fn powitaj_v2(tab: &Vec<i32>) {
    println!("Witaj, {tab:?}!")
}

fn powitaj_v3(tab: Vec<i32>) {
    println!("Witaj, {tab:?}!")
}

fn main() {
    let tab0 = [1,4,90,34]; 
    powitaj_v0(tab0);               // out: Witaj, [1, 4, 90, 34]!
    powitaj_v1(&tab0);              // out: Witaj, [1, 4, 90, 34]!
    powitaj_v1(&[15,3,20]);         // out: Witaj, [15, 3, 20]!
    let tab1 = vec![3,5,7,10,3,4,5,6];
    let tab2 = Vec::from([4,10,3,9,87]);
    powitaj_v2(&tab1);              // out: Witaj, [3, 5, 7, 10, 3, 4, 5, 6]!
    powitaj_v3(tab2.clone());       // out: Witaj, [4, 10, 3, 9, 87]!
    powitaj_v1(&tab1);              // out: Witaj, [3, 5, 7, 10, 3, 4, 5, 6]!
    powitaj_v1(&tab2);              // out: Witaj, [4, 10, 3, 9, 87]!
}
```
📌 `powitaj_v0(tab: [i32; 4])`
- **Argument:** Oczekuje tablicy o stałym rozmiarze `[i32; 4]`.
- Tablica w Rust ma stały rozmiar, więc ta funkcja przyjmuje dokładnie tablicę o czterech elementach.

📌 `powitaj_v1(tab: &[i32])`
- **Argument:** Przyjmuje referencję do ciagu `i32` (`&[i32]`), czyli tablicy o zmiennym rozmiarze.
- Funkcja działa, bo `&[i32]` to **referencja do jakiejkolwiek tablicy (lub wektora)** typu `i32`.
- ***Jest to najbardziej uniwersalny sposób zapisu.***

📌 `powitaj_v2(tab: &Vec<i32>)`
- **Argument:** Przyjmuje referencję do wektora `Vec<i32>`.
- Musisz przekazać referencję (`&tab1`), ponieważ funkcja oczekuje referencji do wektora, a nie samego wektora. Dzięki referencji nie kopiujesz wektora, co jest bardziej wydajne.

📌 `powitaj_v3(tab: Vec<i32>)`
- **Argument:** Przyjmuje wektor `Vec<i32>` przez wartość.
- ❗ Czy `Vec` jest kopiowany?
    - Ważne: `Vec<i32`> nie implementuje `Copy`, tylko `Clone`. To oznacza, że przekazanie `Vec` przez wartość nie robi fizycznej kopii danych ze sterty, tylko przenosi `ownership` wskaźników i danych, ***a oryginał nie może być używany po przekazaniu.***
    - Ale jeśli wywołasz `tab2.clone()`, wtedy:
        - Rust **tworzy nową kopię danych na stercie**, czyli wszystko jest zdublowane.
        - Oryginalny `tab2` zostaje nietknięty i można go nadal używać.
    - Koszt `clone()`: jeśli wektor zawiera dużo danych — `clone()` może być drogi czasowo i pamięciowo, bo kopiuje wszystko.
        ```rs
        let tab2 = Vec::from([1, 2, 3]);
        powitaj_v3(tab2.clone());  // działa, bo przekazujemy **kopię** - ta linia zamiast tej poniżej
        powitaj_v3(tab2);    // tab2 jest przenoszone
        powitaj_v1(&tab2);   // ❌ Błąd: tab2 już nie należy do main()!
        ```
        
```rs
fn wyswietl_jeden(t: &[i32], i: usize){
    println!("{}", t[i]);
}

fn main() {
    let tab0 = [1, 4, 90, 34];
    println!("{}", tab0[2]); 
    wyswietl_jeden(&tab0, 12); // program spanikuje, ponieważ próbuje odwołać się poza indeksem tablicy
}
```
#### Różne sposoby tworzenia tablic:
```rs
// Tablica z powtórzonymi wartościami [wartość; ile_razy]
let tab = [0; 6];                   // [0, 0, 0, 0, 0, 0]       typ: [i32; 6]
let jedynki = [1; 4];               // [1, 1, 1, 1]             typ: [i32; 4]
let zera_bool = [false; 3];         // [false, false, false]    typ: [bool; 3]

// Tablica z konkretnymi wartościami
let liczby = [10, 20, 30, 40];      // [10, 20, 30, 40]         typ: [i32; 4]
let znaki = ['a', 'b', 'c'];        // ['a', 'b', 'c']          typ: [char; 3]

// Tablica z jawnie określonym typem
let liczby: [i32; 3] = [5, 6, 7];   // [5, 6, 7]
let znaki: [char; 2] = ['x', 'y'];  // ['x', 'y']
```

# Wykład 4

W rust nie ma wyjątków, ponieważ są nie efektywne i nie strukruralne. Instrukcja return jest skokiem i jest nie strukturalna. Zapis bez return jest lepszy ponieważ funkcja się po prostu kończy i dokładnie wiadomo gdzie się wraca. Wyjątki zaburzają silne typowanie.
```cpp
int f(int x);
int g(int x);
```
Powyższe funckje w cpp mogą mieć ten sam typ zwracany i takie same argumenty, ale mogą mieć inne wyjątki i to zabuża silne typowanie. 
```rs
fn wyswietl_jeden(t: &[i32], i: usize)
{
    println!("{:?}", t.get(i));     // typ Option<&i32>
}

fn main() {
    let tab0 = [1,4,90,34];
    wyswietl_jeden(&tab0, 1);       // Some(4)
    wyswietl_jeden(&tab0, 12);      // None
}
```
```rs
fn wyswietl_jeden(t: &[i32], i: usize)
{
    let x = t.get(i);
    println!("{:?}", x)
    println!("{:?}", x.is_none());  
    println!("{:?}", x.unwrap_or(&-1));             // to daje domyślną wartość
    //println!("{:?}", x.unwrap_or_default());      // ale w tym przypadku referencja do int nie ma domyślnego typu
    println!("{:?}", x.unwrap());                   // to daje panike
}

fn main() {
    let tab0 = [1,4,90,34];
    wyswietl_jeden(&tab0, 1);   
    wyswietl_jeden(&tab0, 12);    
}
```
🚨 `str` jest statyczny więc nie można go zmieniać, w przeciwienstwie do `String`.
```rs
fn main()
{
    let s0 :&str = "Witaj, świecie!";
    let mut s1 :String = "Ala ma kota".to_string();
    let s2 :String = String::new();
    let s3 :String = String::from("Pies i kot.");

    s1.push_str(" i psa");
    s1.push('.');
    println!("{s1:?}");                     // "Ala ma kota i psa"

    println!("{:?}", s0.get(1..2));         // Some("i")
    println!("{:?}", "pies.".len());        // 4
    println!("{:?}", "pień.".len());        // 5
    println!("{:?}", s0.get(6..9));         // Some(" ś")
    println!("{:?}", s0.get(7..10));        // Some("św")
    println!("{:?}", s0.get(8..11));        // None
    println!("{:?}", s0.get(9..12));        // Some("wie")

    println!("{:?}", s0.chars());           // Chars(['W', 'i', 't', 'a', 'j', ',', ' ', 'ś', 'w', 'i', 'e', 'c', 'i', 'e', '!'])
    println!("{:?}", s0.bytes());           // Bytes(Copied { it: Iter([87, 105, 116, 97, 106, 44, 32, 197, 155, 119, 105, 101, 99, 105, 101, 33]) })
    
    println!("{:?}", s0.chars().nth(3));    // Some('a')
    println!("{:?}", s0.bytes().nth(3));    // Some(97)

    println!("{:?}", s0.chars().nth(7));    // Some('ś')
    println!("{:?}", s0.bytes().nth(7));    // Some(197)

    println!("{:?}", s0.chars().nth(8));    // Some('w')
    println!("{:?}", s0.bytes().nth(8));    // Some(155)

    println!("{:?}", s0.chars().nth(15));   // None
    println!("{:?}", s0.bytes().nth(15));   // Some(33)
}
```
Znaki które dają się zapisać na jednym bajcie są zapisywane tylko w jednym, natomiast dla przykładu litera `ń` zajmuje 2 bajty. Funkcja `get()` dla napisów operuje na bajtach.
```rs
fn main()
{
    let mut s1 :String = "Ala ma żółtego kota".to_string();

    s1.push_str(" i psa");
    s1.push('.');
    println!("{s1:?}");                 // "Ala ma żółtego kota i psa."

    println!("{:?}", s1.find('a'));     // Some(2)
    println!("{:?}", s1.find('x'));     // None
    println!("{:?}", s1.find("a"));     // Some(2)
    println!("{:?}", s1.find("kot"));   // Some(18)

    let s2 = s1.replace("kota", "szczura");

    println!("{:?}", s1);               // "Ala ma żółtego kota i psa."
    println!("{:?}", s2);               // "Ala ma żółtego szczura i psa."

    let b = s1.as_bytes();
    println!("{b:?}");                  // [65, 108, 97, 32, 109, 97, 32, 197, 188, 195, 179, 197, 130, 116, 101, 103, 111, 32, 107, 111, 116, 97, 32, 105, 32, 112, 115, 97, 46]

    //let c = s1.as_chars();            // tego nie ma
    //println!("{c:?}");
}
```
```rs
fn main()
{
    let mut s1 :String = "Ala ma żółtego kota".to_string();

    for i in 3..7
    {
        print!(" {i}");             // 3 4 5 6
    }
    
    let x =3..7;
    for i in x 
    {
        print!(" {i}");             // 3 4 5 6
    }

    for z in s1.chars()             
    {
        print!(" {z:?}");           //  'A' 'l' 'a' ' ' 'm' 'a' ' ' 'ż' 'ó' 'ł' 't' 'e' 'g' 'o' ' ' 'k' 'o' 't' 'a'
    }

    for z in s1.bytes()
    {
        print!(" {z:?}");           // 65 108 97 32 109 97 32 197 188 195 179 197 130 116 101 103 111 32 107 111 116 97
    }

    for z in s1.chars().rev()
    {
        print!(" {z:?}");           // 'a' 't' 'o' 'k' ' ' 'o' 'g' 'e' 't' 'ł' 'ó' 'ż' ' ' 'a' 'm' ' ' 'a' 'l' 'A'
    }

    let s2 :String = s1.chars().rev().collect();
    println!("{s2:?}");             // "atok ogetłóż am alA"

    let w1 :Vec<char> = s1.chars().rev().collect();
    println!("{w1:?}");             // ['a', 't', 'o', 'k', ' ', 'o', 'g', 'e', 't', 'ł', 'ó', 'ż', ' ', 'a', 'm', ' ', 'a', 'l', 'A']

    let w2 :Vec<_> = s1.chars().rev().collect();
    println!("{w2:?}");             // ['a', 't', 'o', 'k', ' ', 'o', 'g', 'e', 't', 'ł', 'ó', 'ż', ' ', 'a', 'm', ' ', 'a', 'l', 'A']
}
```
```rs
fn main()
{
    for z in "kolacja".chars().step_by(2)
    {
        println!("{z:?}");          // 'k' 'l' 'c' 'a'
    }

    for z in [3, 10, 4, 87, 92, 34]
    {
        println!("{z:?}");          // 3 10 4 87 92 34
    }

    for z in [3, 10, 4, 87, 92, 34].iter().step_by(2)
    {
        println!("{z:?}");          // 3 4 92
    }as

    for z in (3..5).chain(7..10)
    {
        println!("{z:?}");			// 3 4 7 8 9
    }

    for z in (3..5).zip(7..10)
    {
        println!("{z:?}");			//  (3, 7) (4, 8)
    }

    let p = (1, 2.4, 'z');
    println!("{}", p.1);			// 2.4
}
```

# Wykład 5
### Problem czytelników i pisarzy w Rust
W Rust mechanizm pożyczania (borrowing) zapewnia bezpieczeństwo dostępu do danych i zapobiega błędom konkurencyjnego dostępu do pamięci. Można go rozpatrywać w kontekście problemu czytelników i pisarzy, gdzie mamy dwie operacje:
- Czytelnicy (readers) – mogą jednocześnie odczytywać dane, o ile nikt ich nie modyfikuje.
- Pisarze (writers) – mogą modyfikować dane, ale muszą mieć do nich wyłączny dostęp.

.|Read only|Mutable|
-- |--|--
Private (pojedynczy właściciel) | ✅ Można pożyczać wiele razy| ✅ Można pożyczyć mutowalnie|
Shared (współdzielony zasób) | ✅ Można pożyczać wiele razy| ☠️ Błąd – nie można jednocześnie modyfikować i pożyczać niemutowalnie

```rs
fn show(a: &i32, b:&i32)
{
    println!("{a} {b}");
}

fn swap(a: &mut i32, b: &mut i32)
{
    let pom = *a;
    *a=*b;
    *b=pom;
}

fn add_to(a: &mut i32, b: &i32)
{
    *a+=b;
}

fn main()
{
    let mut x=10;
    let mut y=200;
    swap(&mut x, &mut y);
    show(&x, &y);           // 200 10
    show(&x, &x);           // 200 200
    // swap(&mut x, &mut x);

    let mut t = [1, 30, 45];
    show(&t[0], &t[2]);     // 1 45
    // swap(&mut t[0], &mut t[2]);

    add_to(&mut x, &y);
    show(&x, &y);           // 210 10
    // add_to(&mut x, &x);
}
```
- <code><span style="color: cyan">swap(&mut x, &mut x);</code>
    - Nie można pożyczyć `x` jako **mutable** (`&mut x`) dwa razy w tym samym wywołaniu funkcji.
    - **Rust wymaga wyłącznego dostępu** do wartości przy mutowalnym referencji.
    - `swap` przyjmuje dwa mutowalne odniesienia (`&mut i32`), a tu próbujemy przekazać dwa razy `x`, co powoduje konflikt dostępu.
- <code><span style="color: cyan">swap(&mut t[0], &mut t[2]);</code>
    - Rust traktuje tablicę jako **jedną strukturę**, a `t[0]` i `t[2]` to tylko jej elementy.
    - Kompilator wykrywa, że próbujemy pożyczyć różne elementy tej samej tablicy jako mutowalne w tym samym czasie.
    - W Rust mutowalne pożyczanie dotyczy całego obiektu, więc nie można pożyczyć dwóch elementów tablicy jednocześnie jako `&mut`
- <code><span style="color: cyan">add_to(&mut x, &x);</code>
    - Funkcja `add_to` wymaga:
        - mutowalnego odniesienia `&mut a`
        - niemutowalnego odniesienia `&b`
    - Ale przekazujemy x jednocześnie jako:
        - `&mut x` (pierwszy argument)
        - `&x` (drugi argument)
    - Rust **nie pozwala na jednoczesne mutowanie i niemutowanie tej samej zmiennej**, ponieważ może to prowadzić do **race condition** (konfliktów dostępu do pamięci).

```rs
fn f1() -> i32 {5}

fn f2() -> Option<i32> {Some(5)}                            // warianty: Some(wynik), None

fn f3() -> Result<i32, std::io::Error> {Ok(5)}              // warianty: Ok(wynik), Err(opis_błędu)

fn f4() -> Result<i32, std::string::ParseError> {Ok(5)}     // warianty: Ok(wynik), Err(opis_błędu)

fn main()
{
    let x = Some('x');
    let mut y = None;
    let a :Result<bool, &str> = Err("kot");
    let mut b = Ok(3.5);

    println!("{}", x.unwrap());                 // x
    // println!("{}", y.unwrap());              
    // println!("{}", a.unwrap());
    println!("{}", b.unwrap());                 // 3.5

    println!("{}", x.unwrap_or('a'));           // x
    println!("{}", y.unwrap_or(342));           // 342
    println!("{}", a.unwrap_or(true));          // true
    println!("{}", b.unwrap_or(-1.3));          // 3.5

    println!("{}", y.unwrap_or_default());      // 0
    println!("{}", a.unwrap_or_default());      // false

    println!("{}", x.is_none());                // false
    println!("{}", y.is_some());                // false
    println!("{}", a.is_ok());                  // false
    println!("{}", b.is_err());                 // false

    println!("{:?}", a.ok());                   // None
    println!("{:?}", a.err());                  // Some("kot")
    println!("{:?}", b.ok());                   // Some(3.5)
    println!("{:?}", b.err());                  // None

    // println!("{}", y.expect("Spodziewałem się liczby"));

    y = Some(123);
    b = Err(true);
}
```
- <code><span style="color: cyan">println!("{}", y.unwrap());</code>
    - `y` jest zadeklarowane jako `None`, więc `unwrap()` nie może zwrócić wartości i powoduje panikę. 
    - `unwrap()` działa poprawnie tylko wtedy, gdy zmienna zawiera `Some(...)`         
- <code><span style="color: cyan">println!("{}", a.unwrap());</code>
    - `a` jest `Err("kot")`, więc `unwrap()` nie może zwrócić wartości i również powoduje panikę. 
    - `unwrap()` na `Result<T,E>` działa poprawnie tylko wtedy, gdy zmienna zawiera `Ok(...)`
- <code><span style="color: cyan">println!("{}", y.expect("Spodziewałem się liczby"));</code>
    - `y` jest `None`, więc `expect(...)` zadziała tak samo jak `unwrap()`, ale zamiast domyślnego komunikatu Rust, wyrzuci panikę z dostosowaną wiadomością:  
    <samp><span style="color: orange">thread 'main' panicked at 'Spodziewałem się liczby'



```rs
fn main()
{
    let x = Some('x');
    let z = Some('z');
    let y: Option<char> = None;
    let w: Option<char> = None;

    println!("{:?}", x.and(z));
    println!("{:?}", x.and(y));
    println!("{:?}", w.and(z));
    println!("{:?}", w.and(y));

    println!("{:?}", x.or(z));
    println!("{:?}", x.or(y));
    println!("{:?}", w.or(z));
    println!("{:?}", w.or(y));
}
```
### Dlaczego w `Option<T>` nie ma operacji `xor`?
W Rust metody `and()` i `or()` dla `Option<T>` działają analogicznie do operacji logicznych **AND** i **OR**:
- `and()` zwraca Some tylko wtedy, gdy oba operandy są Some, w przeciwnym razie zwraca None.
- `or()` zwraca pierwszy Some, jeśli istnieje, inaczej zwraca drugi operand.

Teraz zastanówmy się, dlaczego nie ma `xor()`.

#### 1. `XOR` dla wartości logicznych
Działanie operatora `XOR` (wyłączne OR, "exclusive OR") dla wartości logicznych wygląda tak:

A|B|A ⊕ B
:--:|:--:|:--:
0|0|0
0|1|1
1|0|1
1|1|0

`XOR` zwraca `true` tylko wtedy, gdy dokładnie jeden z operandów jest `true`.

#### 2. Czy `Option<T>` pasuje do `XOR`?
Zastosujmy tę logikę do `Option<T>`:

`Option<T>`	|`Option<T>`	|`xor()` wynik?
:--|:--|:--
`None`	|`None`	|`None` ?
`Some(x)`	|`None`	|`Some(x)` ?
`None`	|`Some(y)`	|`Some(y)` ?
`Some(x)`	|`Some(y)`	|**?? (problem)**

Pierwsze trzy przypadki wydają się sensowne, ale co zrobić w przypadku `Some(x) ⊕ Some(y)`?
- Wartość `XOR` zakłada jednoznaczny wynik.
- Ale co jeśli `x != y`? Który `Some` powinien zostać zwrócony?
- Trzeba by było jakoś arbitralnie wybrać `x` lub `y`, co nie jest jednoznaczne i może prowadzić do nieintuicyjnych wyników.

```rs
fn srednia(tab: & [f64]) -> Option<f64>
{
    if tab.len() == 0
    {
        None
    }else
    {
        let s: f64 = tab.iter().sum();
        Some(s/(tab.len() as f64))
    }
}

fn ile_powyzej_sredniej(tab: &[f64]) -> Option<usize>
{
    let sr = srednia(&tab)?;  // ? - robi to co jest zakomentowane niżej
    // if sr.is_none()
    // {
    //     return None;
    // }
    // let sr = sr.unwarp();
    let mut ile = 0;
    for x in tab 
    {
        if *x > sr
        {
            ile += 1;
        }
    }
    Some(ile)

}

fn srednia2(tab: & [f64]) -> Result<f64>
{
    if tab.len() == 0
    {
        Err("Pusta tablica!".to_string())
    }else
    {
        let s: f64 = tab.iter().sum();
        Ok(s/(tab.len() as f64))
    }
}

fn ile_powyzej_sredniej(tab: &[f64]) -> Result<usize>
{
    let sr = srednia(&tab)?;  // ? - robi to co jest zakomentowane niżej
    // let sr = srednia(&tab); 
    // if sr.is_err()
    // {
    //     return Err(sr.err().unwrap());
    // }
    // let sr = sr.unwarp();
    let mut ile = 0;
    for x in tab 
    {
        if *x > sr
        {
            ile += 1;
        }
    }
    Ok(ile)

}

fn main() {
    println!("{:?}", srednia(&[1.4, 3.2]));
    println!("{:?}", srednia(&[3.2]));
    println!("{:?}", srednia(&[]));
}
```
Operator `?` automatycznie obsługuje błędy, sprawiając, że kod jest czytelniejszy.

# Wykład 6

### Kolokwium:
- 4-6 zadań (krótkich)
- krótki program lub funkcja do napisania w każdym
- materiał:
    - podstawy programowania strukturalnego: pętle, ify, funkcje, operacje arytmetyczne, logiczne, używanie zmiennych
    - różne sposoby przekazywania parametrów do funkcji: pożyczki, mutowalność, niemutowalność
    - używanie napisów: String, str
    - tablice: Vec, []

### Iteratory:
- **Iteratory w Rust są leniwe** – oznacza to, że nie wykonują żadnych operacji, dopóki nie zostaną faktycznie użyte (np. w pętli lub metodzie typu `collect`).
- Pętla `for i in 0..n { v[i] }` iteruje po indeksach i odwołuje się do elementów wektora `v` przez indeksowanie.
- Pętla `for x in v { x }` przechodzi bezpośrednio po elementach wektora `v`.
- **Funkcje w Rust wymagają jawnego określenia typu argumentów oraz typu zwracanego.**
- **Domknięcia (lambdy) przechwytują swoje środowisko**, czyli mogą korzystać z lokalnych zmiennych, które były dostępne w momencie ich utworzenia.
- **Zwykłe funkcje mogą odwoływać się tylko do globalnych zmiennych**, ale nie do zmiennych lokalnych spoza swojego ciała.

```rs
for z in "katastrofa".chars().step_by(2) {
    print!("{} ", z);
}
//output: k t s r f
```
Metoda `.chars()` zamienia ciąg znaków na iterator, `.step_by()` sprawia że iteracja odbywa się co drugi znak.
```rs
for z in [1, 3, 4, 10]
{
    print!("{} ", z);
}
//output: 1 3 4 10
```
Kod iteruje po elementach tablicy [1, 3, 4, 10] i wypisuje każdy z nich. Tablica jest iterowana bezpośrednio, więc wartości są przekazywane bez indeksowania.
```rs
for z in [1, 3, 4, 10].iter()
{
    print!("{} ", z);
}
//output: 1 3 4 10
```
To samo co wyżej ale przy użyciu `.iter()`.
```rs
for z in (0..).step_by(5)
{
    print!("{} ", z);
}
//output: 0 5 10 15 20 25 30 35 40 45 50 55 60 65 70 75 80 85 90 95 100 ...
```
Kod generuje nieskończoną sekwencję liczb naturalnych, zaczynając od `0`, i wypisuje je, zwiększając co `5` dzięki `.step_by(5)`. Program będzie działał (praiwe) w nieskończoność.
```rs
for z in (0..5).chain(50..55)
{
    print!("{} ", z);
}
//output: 0 1 2 3 4 50 51 52 53 54 
```
Kod iteruje najpierw po zakresie `0..5`, a następnie po `50..55`, łącząc je metodą `.chain()`.
```rs
let v: Vec<_> = (0..5).chain(50..55).collect();
print!("{:?}", v);
//output: [0, 1, 2, 3, 4, 50, 51, 52, 53, 54]
```
Kod tworzy wektor `v`, który zawiera liczby z połączonych zakresów `0..5` i `50..55`, używając `.chain()` i `.collect()`. `Vec<_>` to sposób na zadeklarowanie wektora w Rust, gdzie typ elementów wektora jest wnioskowany przez kompilator (dzięki użyciu `_` jako symbolu typu).
```rs
let v: std::collections::HashSet<_> = (0..5).chain(50..55).collect();
print!("{:?}", v);
```
Kod tworzy `HashSet` z elementów pochodzących z połączonych zakresów `0..5` oraz `50..55`, używając metody `.chain()` i `.collect()`. `HashSet` automatycznie eliminuje duplikaty, więc w przypadku powtarzających się elementów, zostaną one zapisane tylko raz.\
**🚨 Uwaga!** `HashSet` przechowuje elementy w nieuporządkowanej kolejności. Ostateczna kolejność elementów może różnić się przy każdym uruchomieniu programu, ponieważ HashSet nie gwarantuje zachowania kolejności.
```rs
for z in (0..5).zip(50..55)
{
    print!("{:?} ", z);
}
//output: (0, 50) (1, 51) (2, 52) (3, 53) (4, 54)
```
Metoda `.zip()` łączy elementy z dwóch zakresów: `0..5` i `50..55`, tworząc pary, gdzie pierwszy element pochodzi z pierwszego zakresu, a drugi z drugiego. `.zip()` zwraca krotkę, zawierającą te elementy.
```rs
for z in (0..5).zip("buteleczka".chars())
{
    print!("{:?} ", z);
}
//output: (0, 'b') (1, 'u') (2, 't') (3, 'e') (4, 'l')
```
W przypadku, gdy drugi iterator (`"buteleczka".chars()`) jest dłuższy niż pierwszy (`0..5`), metoda `.zip()` zatrzyma się, gdy któryś z iteratorów się skończy. W tym przypadku `0..5` ma tylko 5 elementów, podczas gdy `"buteleczka".chars()` ma więcej znaków. Pary będą tworzone do momentu, gdy skończy się krótszy iterator, czyli `0..5`. Dalsze znaki w `"buteleczka"` nie będą już dołączane do wyników.
```rs
for z in (0..).zip("buteleczka".chars())
{
    print!("{:?} ", z); 
}
//output: (0, 'b') (1, 'u') (2, 't') (3, 'e') (4, 'l') (5, 'e') (6, 'c') (7, 'z') (8, 'k') (9, 'a')
```
W przypadku, gdy używamy nieskończonego zakresu (`0..`) w połączeniu z iteratorami z `"buteleczka".chars()`, `.zip()` będzie tworzyć pary aż do momentu, gdy skończy się drugi iterator, czyli `chars()` z `"buteleczka"`. Ponieważ długość `"buteleczka"` to 11 znaków, pętla zatrzyma się po 11 iteracjach.
```rs
for z in "buteleczka".chars().enumerate()
{
    print!("{:?} ", z);
}
//output: (0, 'b') (1, 'u') (2, 't') (3, 'e') (4, 'l') (5, 'e') (6, 'c') (7, 'z') (8, 'k') (9, 'a')
```
Metoda `.enumerate()` iteruje po elementach ciągu, zwracając pary, gdzie pierwszy element to indeks, a drugi to wartość (w tym przypadku znak). Dzięki temu możemy uzyskać zarówno indeks, jak i znak.
```rs
for (i, x) in "buteleczka".chars().enumerate()
{
    print!("{i} {x} ",);
}
//output: 0 b 1 u 2 t 3 e 4 l 5 e 6 c 7 z 8 k 9 a
```
Działa to tak samo jak poprzednio, ale tym razem krotka `(indeks, znak)` jest od razu rozpakowywana w zmienne `i` i `x` w każdej iteracji. Dzięki temu nie trzeba odwoływać się do elementów krotki osobno.
```rs
for z in "buteleczka".chars().take(5)
{
    print!("{:?} ",z );
}
//output: 'b' 'u' 't' 'e' 'l' 
```
W tym przypadku, metoda `.take(5)` ogranicza liczbę elementów do pierwszych `5` znaków z łańcucha `"buteleczka"`. Dzięki temu, pętla iteruje tylko po pierwszych pięciu znakach.
```rs
for z in (0..).take(5)
{
    print!("{:?} ",z );
}
//output: 0 1 2 3 4
```
W tym przypadku, metoda `.take(5)` ogranicza liczbę elementów do pierwszych 5 z nieskończonego zakresu (`0..`). Pętla więc wypisuje pierwsze pięć liczb naturalnych zaczynając od 0. Metoda `.take(5)` zatrzymuje iterację po 5 elementach, nawet jeśli zakres jest nieskończony.
```rs
let x = "buteleczka".chars().min();
let y = "buteleczka".chars().max();
println!("{:?}", x);
//output: Some('a')
println!("{:?}", y);
//output: Some('z')
```
Metody `.min()` i `.max()` w Rust działają na iteratorach i zwracają najmniejszy lub największy element w kolekcji, zgodnie z porównaniem domyślnym (np. dla znaków według porządku leksykalnego).
W tym przypadku, dla łańcucha `"buteleczka"`, `.min()` znajdzie najmniejszy znak, a `.max()` największy.
Zwrócone wartości są typu `Option<...>`, dlatego nawet jeśli mamy wynik, jest to opakowane w `Some(...)`.
```rs
let x = "".chars().min();
let y = "".chars().max();
println!("{:?}", x);
//output: None
println!("{:?}", y);
//output: None
```
Wprzypadku pustego łańcucha `""`, zarówno `.min()`, jak i `.max()` nie znajdą żadnych elementów, ponieważ iterator `.chars()` na pustym ciągu nie zawiera żadnych znaków. Obie metody zwrócą `None`.
```rs
let s :u16 = [1, 4, 36].iter().sum();
println!("{:?}", s);
//output: 41
```
W tym przypadku, metoda `.sum()` jest używana do zsumowania wartości z iteratora, który pochodzi z tablicy `[1, 4, 36]`.
```rs
let s :u8 = [].iter().sum();
println!("{:?}", s);
//output: 0
```
W tym przypadku, kod próbuje zsumować elementy z pustej tablicy `[]`. Metoda `.sum()` działa na iteratorze, ale ponieważ tablica jest pusta, wynik sumy będzie wynosił `0`.
```rs
fn podnies_do_kwadratu(n: i32) -> i32
{
    n*n
}

fn main() {
    let v :Vec<_> = (0..).map(podnies_do_kwadratu).take(5).collect();
    println!("{:?}", v);
    //output: [0, 1, 4, 9, 16]
}
```
W tym przypadku, funkcja `podnies_do_kwadratu` przyjmuje liczbę typu `i32` i zwraca jej kwadrat. W funkcji `main`, używamy tego jako funkcji przekazywanej do metody `.map()`, która podnosi każdą liczbę z zakresu `0..` do kwadratu.
- `.map(podnies_do_kwadratu)` stosuje funkcję `podnies_do_kwadratu` do każdego elementu w iteratorze.
- `.take(5)` ogranicza liczbę elementów do pierwszych 5 wyników.
- `.collect()` zbiera wyniki w wektorze.
```rs
let v :Vec<_> = (0..).map(|x| x * x).take(5).collect();
println!("{:?}", v);
//output: [0, 1, 4, 9, 16]
```
W tym przypadku, używamy lambdy `|x| x * x`, która przyjmuje argument `x` i zwraca jego kwadrat, aby zastosować ją w metodzie `.map()` na zakresie `0..`. Dzięki temu, każda liczba w zakresie jest podnoszona do kwadratu, a wynik jest zbierany do wektora za pomocą `.take(5)` i `.collect()`.
```rs
let mut a = 4;
let v :Vec<_> = (0..).map(|x| x + 2 + a).take(5).collect();
println!("{:?}", v);
//output: [6, 7, 8, 9, 10]
```
 Funkcja lambda `|x| x + 2 + a` dodaje do każdej liczby z zakresu `0..` wartość 2 oraz zmienną `a`. Zmienna `a` jest dostępna wewnątrz lambdy, mimo że została zadeklarowana poza nią, ponieważ lambda w Rust może "zamykać" (capture), "przechwytywać" zmienne z otaczającego ją środowiska i używać ich w swoim ciele.
 ```rs
let v :Vec<_> = (1..=10).map(|n| n * n).collect();
println!("{:?}", v);
//output: [1, 4, 9, 16, 25, 36, 49, 64, 81, 100]
 ```
W tym przypadku, kod wykorzystuje zakres `1..=10`, który generuje liczby od 1 do 10 włącznie, podnosi je do kwadratu i zapisuje do wektora.
```rs
let v :Vec<_> = (1..=100).filter(|n| n % 10 == 1).collect();
println!("{:?}", v);
//output: [1, 11, 21, 31, 41, 51, 61, 71, 81, 91]
```
W tym przypadku, zakres `1..=100` generuje liczby od 1 do 100 (w tym 100). Metoda `.filter(|n| n % 10 == 1)` filtruje liczby, wybierając tylko te, które mają resztę 1 po podzieleniu przez 10 (czyli liczby kończące się na 1). `.collect()` zbiera wyniki do wektora.
```rs
let x: Option<i32> = (1..=100).reduce(|acc, x| acc + x);
println!("{:?}", x);
//output: Some(5050)
```
Metoda `.reduce(|acc, x| acc + x)` działa podobnie do .sum(), ale zwraca wynik jako `Option<i32>`. Działa w ten sposób:
- `acc` (akumulator) zaczyna się od pierwszego elementu (tutaj 1).
- `x` to kolejne elementy z zakresu `1..=100`.
- W każdej iteracji do `acc` dodawana jest wartość `x`, aż do przetworzenia wszystkich elementów.

Ponieważ zakres `1..=100` nie jest pusty, `.reduce()` zwróci `Some(wynik)`.

```rs
let x: Option<i32> = (1..=100).filter(|n|n>&100000).reduce(|acc, x|acc+x);
println!("{:?}", x);
//output: None
```
Jeśli iterator jest pusty, `.reduce()` zwróci `None`.
```rs
let x: i32 = (1..=100).fold(0, |acc, x|acc+x);
println!("{:?}", x);
//output: 5050
```
Metoda `.fold(0, |acc, x| acc + x)` zaczyna z wartością początkową `0` i dla każdego elementu z zakresu `1..=100` dodaje go do akumulatora `acc`. Dzięki temu zawsze zwraca wynik, nawet jeśli iterator jest pusty, w przeciwieństwie do `.reduce()`, które mogłoby zwrócić `None`.
```rs
let x= (16..=100).find(|n| n%6==0 && n%15 ==0);
println!("{:?}", x);
//output: Some(30)
```
Metoda `.find(|n| n % 6 == 0 && n % 15 == 0)` przeszukuje zakres `16..=100` i zwraca pierwszą liczbę podzielną zarówno przez 6, jak i 15. Jeśli w zakresie nie byłoby takiej liczby, wynik byłby `None`.
```rs
let x= (16..=100).rfind(|n| n%6==0 && n%15 ==0);
println!("{:?}", x);
//output: Some(90)
```
Metoda `.rfind(...)` działa tak samo jak `.rev().find(...)`, odwraca kolejność iteracji, dzięki czemu znajduje największą liczbę spełniającą warunek zamiast pierwszej. W tym przypadku zwróci `Some(90)`, ponieważ jest to największa liczba w zakresie `16..=100`, podzielna zarówno przez 6, jak i 15.
```rs
let v: Option<_> = (1..=100).find(|n| n % 10 != 5);
println!("{:?}", v);
//output: Some(1)
```
Podkreślenie (`_`) w `Option<_>` oznacza, że kompilator sam wywnioskuje typ wartości wewnątrz `Option`. W tym przypadku `.find(...)` zwraca `Option<i32>`, ale ponieważ `v` nie jest explicite zadeklarowane jako `Option<i32>`, kompilator automatycznie dobiera właściwy typ (`i32`). Jest to przydatne, gdy nie chcemy ręcznie określać typu lub gdy może on się zmieniać w zależności od kontekstu.
```rs
let v: Vec<_> = (1..=100).filter(|n| n % 10 != 5).collect();
println!("{:?}", v);
//output: [1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19, 20, 21, 22, 23, 24, 26, 27, 28, 29, 30, 31, 32, 33, 34, 36, 37, 38, 39, 40, 41, 42, 43, 44, 46, 47, 48, 49, 50, 51, 52, 53, 54, 56, 57, 58, 59, 60, 61, 62, 63, 64, 66, 67, 68, 69, 70, 71, 72, 73, 74, 76, 77, 78, 79, 80, 81, 82, 83, 84, 86, 87, 88, 89, 90, 91, 92, 93, 94, 96, 97, 98, 99, 100]
```
Metoda `.filter(|n| n % 10 != 5)` odrzuca wszystkie liczby kończące się na `5`, a `.collect()` zbiera pozostałe do wektora `Vec<_>`, gdzie podkreślenie `_` pozwala kompilatorowi samodzielnie wywnioskować typ elementów (`i32`). W wyniku otrzymujemy wektor liczb od `1` do `100`, ale bez tych, które kończą się na `5`.
```rs
let v: Vec<_> = (1..=100).take_while(|n| n % 10 != 5).collect();
println!("{:?}", v);
//output: [1, 2, 3, 4]
```
Metoda `.take_while(|n| n % 10 != 5)` zbiera liczby z zakresu `1..=100`, dopóki nie natrafi na liczbę, która kończy się na `5`. Po napotkaniu liczby kończącej się na `5` (czyli `5`), zbieranie zostaje zakończone.
```rs
let v: Vec<_> = (1..=100).skip_while(|n| n % 10 != 5).collect();
println!("{:?}", v);
//output: [5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100]
```
Metoda `.skip_while(|n| n % 10 != 5)` pomija wszystkie liczby w zakresie `1..=100`, które nie kończą się na `5`, aż napotka pierwszą liczbę, która kończy się na `5`. Po tym, jak napotka liczbę kończącą się na `5`, zbiera pozostałe liczby, w tym tę, która spełnia warunek.
```rs
let x = (1..).map(|x| 1.0 / (x as f64)).find(|x| x < &0.03);
println!("{:?}", x);
//output: Some(0.029411764705882353)
```
Metoda `.map(|x| 1.0 / (x as f64))` przekształca liczby z zakresu `1..` na odwrotności tych liczb, konwertując je na typ `f64`. Następnie `.find(|x| x < &0.03)` szuka pierwszego elementu w tej sekwencji, który jest mniejszy niż `0.03`.
```rs
let x = (1..).map(|x| 1.0 / (x as f64)).enumerate().find(|x| x.1 < 0.03);
println!("{:?}", x);
//output: Some((33, 0.029411764705882353))
```
Metoda `.map(|x| 1.0 / (x as f64))` przekształca liczby z zakresu `1..` na odwrotności liczb, konwertując je na typ `f64`. Następnie `.enumerate()` dodaje do każdego elementu numer indeksu (pozycję w iteracji). Metoda `.find(|x| x.1 < 0.03)` szuka pierwszego elementu, którego wartość (zwracana przez `x.1`, czyli odwrotność) jest mniejsza niż `0.03`.
```rs
let x = (10..20).all(|x| x > 0);
println!("{:?}", x);
//output: true
let x = (10..20).any(|x| x > 15);
println!("{:?}", x);
//output: true
let x = (10..).all(|x| x < 20);
println!("{:?}", x);
//output: false
let x = (10..).any(|x| x < 20);
println!("{:?}", x);
//output: true
```
Metoda `.all()` sprawdza, czy wszystkie elementy w danym iteratorze spełniają określony warunek. Zwraca wartość logiczną (`true` lub `false`), zależnie od tego, czy warunek jest spełniony dla każdego elementu w kolekcji.\
Metoda `.any()` sprawdza, czy przynajmniej jeden element w danym iteratorze spełnia określony warunek. Zwraca wartość logiczną (`true` lub `false`), zależnie od tego, czy istnieje przynajmniej jeden element spełniający warunek.
```rs
(1..10).for_each(|x| print!("{:?} ", x));
//output: 1 2 3 4 5 6 7 8 9 
```
Metoda `.for_each()` służy do iterowania po wszystkich elementach iteratora i wykonania na nich podanej operacji.

# Wykład 7

Plain Old Data (POD) to pojęcie wywodzące się z języka C++ i oznacza strukturę danych, która ma bardzo prostą, "niezaskakującą" reprezentację w pamięci — czyli taką, która:
- nie zawiera konstruktorów ani destruktorów,
- nie zawiera wirtualnych funkcji ani dziedziczenia,
- składa się wyłącznie z prostych typów (np. `int`, `float`, `char`, innych POD),
- może być bezpiecznie kopiowana przez `memcpy` lub zrzucana do pliku jako binarka i później odczytywana.

#### POD a Rust
Rust nie ma dokładnie takiej klasyfikacji jak C++ (POD, trivial, standard-layout itd.), ale w praktyce wiele typów w Rust można uznać za "POD-owate". Tzn. też mają przewidywalny układ w pamięci i nie mają specjalnych zachowań przy kopiowaniu czy destrukcji.

#### `Punkt3D` – struktura nazwanych pól
```rs
#[derive(PartialEq, Debug, Clone, Default)] 
struct Punkt3D {
    x:f64,
    y:f64,
    z:f64,
}
```
Co to oznacza?
- To klasyczna struktura z nazwanymi polami.
- Każde pole ma nazwę (`x`, `y`, `z`) i typ (`f64` – liczby zmiennoprzecinkowe).
- Jest to bardzo czytelna forma, dobra do pracy, gdy chcesz wiedzieć, co oznacza każde pole.

`#[derive(...)]` – automatyczne implementacje\
Rust używa tej składni, by automatycznie zaimplementować pewne cechy (traits):
- `PartialEq` – pozwala porównywać dwie struktury za pomocą `==` i `!=`.
- `Debug` – umożliwia debug-printowanie struktury, np. z `println!("{:?}", punkt)`.
- `Clone` – pozwala na klonowanie, np. `let b = a.clone()`;.
- `Default` – pozwala stworzyć "domyślną" wartość, np. `Punkt3D::default()` zwróci `Punkt3D { x: 0.0, y: 0.0, z: 0.0 }`.


#### `Punkt3D_2` – struktura krotek (tuple struct)
```rs
#[derive(PartialEq, Debug, Clone, Default)]
struct Punkt3D_2 (f64,f64,f64);
```
Co to oznacza?
- To tzw. **tuple struct** – struktura, która wygląda jak krotka, ale ma własną nazwę typu.
- Pola nie mają nazw – są dostępne przez indeksy: `.0`, `.1`, `.2`.
- Funkcjonalnie jest prawie taka sama jak `Punkt3D`, ale mniej czytelna w kontekście semantycznym.

Kiedy używać której?

Cechy	|`Punkt3D` (nazwane pola)	|`Punkt3D_2` (tuple struct)
--|--|--
Czytelność	|✅ lepsza (`x`, `y`, `z`)	                |❌ mniej czytelna (`.0`, `.1`, `.2`)
Semantyka	|✅ jasna (wiadomo, co robi każde pole)	    |🤷 raczej do tymczasowych danych
Wygoda	    |✅ lepsza przy dokumentowaniu, testowaniu	|✅ krótsza w pisaniu


```rs
impl Punkt3D_2
{
    fn new(x: f64, y:f64, z:f64) -> Self
    {
        Self(x, y, z)
    }
}
```
🔍 Co oznacza `impl Punkt3D_2 { ... }`?\
To blok implementacji metod dla typu `Punkt3D_2`. W jego wnętrzu definiujesz funkcje (tzw. metody), które są związane z tą strukturą.

🛠 Co robi `fn new(...) -> Self`?
- `Self` to alias na aktualny typ (`Punkt3D_2`).
- Funkcja `new` przyjmuje trzy argumenty typu f64 i tworzy nową instancję struktury.
- `Self(x, y, z)` to skrócony zapis dla `Punkt3D_2(x, y, z)`.

```rs
impl Punkt3D
{
    fn new(x: f64, y:f64, z:f64) -> Punkt3D
    {
        Punkt3D {
            x: x,
            y: y,
            z: z,
        }
        //Punkt3D {
        //    x,
        //    y,
        //    z,
        //}
    }

    fn srodek_uw() -> Self   //zamiast Punkt3D mozna pisać Self dużą literą
    {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
        //Self::default()
    }
    fn norma(&self) -> f64
    {
        (self.x*self.x + self.y*self.y +self.z*self.z).sqrt()
    }

}
```
`fn new(x: f64, y: f64, z: f64) -> Punkt3D`
- **Cel: Konstruktor** – tworzy nową instancję struktury `Punkt3D` z podanymi współrzędnymi.
- `x: x` przypisuje wartość parametru `x` do pola `x` w strukturze – Rust umożliwia skrót: `x` zamiast `x: x`, jeśli nazwy się zgadzają.
- Zwraca: `Punkt3D { x, y, z }`.

`fn srodek_uw() -> Self`
- **Cel:** Zwraca punkt w centrum układu współrzędnych (0.0, 0.0, 0.0).
- `Self` to alias na typ `Punkt3D`, używany w metodach typu.
- **Alternatywa:** `Self::default()` robi to samo, ponieważ mamy `#[derive(Default)]` i domyślne wartości pól to zera.

`fn norma(&self) -> f64`
- **Cel:** Oblicza długość wektora od punktu (0,0,0) do `self`, czyli jego normę euklidesową.
- `&self` oznacza, że metoda działa na referencji do konkretnego obiektu.
- Obliczenie: $ \sqrt{x^2+y^2+z^2} $
- Zwraca wartość typu `f64`.

#### 1. Tworzenie struktur:
```rs
let mut p1k = Punkt3D_2(3.5, -12.2, 7.6);
```
- Tworzy tuple struct (`Punkt3D_2`) z trzema wartościami `f64`.
- `mut` pozwala później modyfikować `p1k` 
```rs
let mut p1 = Punkt3D {
    x: 3.5,
    y: -12.2,
    z: 7.6,
};
```
- Tworzy klasyczną strukturę Punkt3D z nazwanymi polami.
- Też oznaczony jako mut.
#### 2. Modyfikacja pola `z`:
```rs
p1.z = 3.9;
```
- Zmienia wartość pola `z` w `p1` z `7.6` na `3.9`.
#### 3. Tworzenie drugiego punktu:
```rs
let mut p2 = Punkt3D {
    x: 3.5,
    y: 2.1,
    z: 7.6,
};
```
- Nowa struktura `p2`, różni się od `p1` polem `y`
#### 4. Porównanie struktur:
```rs
println!("{}", p1 == p2); 
// output: false
```
- Dzięki `#[derive(PartialEq)]` można porównywać `==`.
- `false`, bo `p1.y = -12.2`, a `p2.y = 2.1`

#### 5. Debug print:
```rs
println!("{:?}", p1);
// output: Punkt3D { x: 3.5, y: -12.2, z: 3.9 }
```
- Dzięki `#[derive(Debug)]` wypisuje strukturę w formacie debug
#### 6. Tworzenie punktów przez metody:
```rs
let p3 = Punkt3D::new(2.3, 1.0, -0.1);
let p4 = Punkt3D::srodek_uw();
```
- `p3` tworzony przez `new()`, `p4` to punkt zerowy

#### 7. Klonowanie:
```rs
let p5 = p3.clone();
```
- Tworzy kopię `p3`. Dzięki `#[derive(Clone)]`.
#### 8. Wypisywanie punktów:
```rs
println!("{:?}", p3);
println!("{:?}", p4);
// output: Punkt3D { x: 2.3, y: 1.0, z: -0.1 }
//         Punkt3D { x: 0.0, y: 0.0, z: 0.0 }
```
#### 9. Norma (długość wektora):
```rs
println!("{}", p2.norma());
println!("{}", Punkt3D::norma(&p2));
//output: 8.62670273047588
//        8.62670273047588
```
- Dwa sposoby wywołania tej samej metody.
#### 10. Tworzenie wektora `v` z wartościami `Option<Punkt3D>`
```rs
let v = vec![
    None,
    Some(p1.clone()),
    None,
    Some(p2.clone()),
    Some(p3.clone()),
];
```
- Tworzy wektor `v` z wartościami `Option<Punkt3D>`.
- `Some(...)` zawiera sklonowane punkty, a `None` oznacza brak wartości.
- `vec![]` tworzy dynamiczny wektor.
#### 11. Debug-print całego wektora
```rs
println!("{v:?}");
// output: [None, Some(Punkt3D { x: 3.5, y: -12.2, z: 3.9 }), None, Some(Punkt3D { x: 3.5, y: 2.1, z: 7.6 }), Some(Punkt3D { x: 2.3, y: 1.0, z: -0.1 })]
```
#### 12. Iteracja po referencjach do elementów `v`
```rs
for p in &v {
    println!("{:?}", p.clone().unwrap_or(Punkt3D::srodek_uw()));
    println!("{:?}", p.clone().unwrap_or_default());
}
//output:   Punkt3D { x: 0.0, y: 0.0, z: 0.0 }
//          Punkt3D { x: 0.0, y: 0.0, z: 0.0 }
//          Punkt3D { x: 3.5, y: -12.2, z: 3.9 }
//          Punkt3D { x: 3.5, y: -12.2, z: 3.9 }
//          Punkt3D { x: 0.0, y: 0.0, z: 0.0 }
//          Punkt3D { x: 0.0, y: 0.0, z: 0.0 }
//          Punkt3D { x: 3.5, y: 2.1, z: 7.6 }
//          Punkt3D { x: 3.5, y: 2.1, z: 7.6 }
//          Punkt3D { x: 2.3, y: 1.0, z: -0.1 }
//          Punkt3D { x: 2.3, y: 1.0, z: -0.1 }
```
- Przechodzi przez każdy element `v`, używając referencji `&v`.
- `p.clone()` tworzy kopię `Option<Punkt3D>`, by można było ją odpakować przy pomocy `unwrap_or(...)`.
- `unwrap_or_default()` zwraca zawartość `Some(...)` lub wartość domyślną, jeśli `None`.
- W obu przypadkach, gdy `p` to `None`, zwracany jest punkt `(0.0, 0.0, 0.0)`.
#### 13. Tworzenie nowego punktu `p5` na podstawie istniejącego `p1`
```rs
let p5 = Punkt3D {
    y: -98.2,
    ..p1
};
```
- Tworzymy nowy obiekt `Punkt3D`.
- Pole `y` ustawiamy ręcznie na `-98.2`.
- Pozostałe pola (`x` i `z`) automatycznie kopiujemy z istniejącego obiektu `p1`.
- Operator `..p1` oznacza: **"wypełnij resztę pól wartościami z p1"**.
#### 14. Modyfikacja tuple struct `p5k`
```rs
let mut p5k = p1k.clone();
p5k.1 = -98.2;
```
- Tworzy kopię `p1k` i modyfikuje drugie pole (indeks 1).
- Pola w tuple struct są dostępne jako `.0`, `.1`, `.2`.
#### 15. Tworzenie `p6` z `default()` + zmiana `y`
```rs
let p6 = Punkt3D {
    y: -98.2,
    ..Punkt3D::default()
};
```
- Tworzy punkt, w którym tylko `y = -98.2`, a `x` i `z` są domyślne (`0.0`).
- Przykład użycia `..default()` z nadpisaniem pojedynczego pola.


### [Całość wyżej opisywanego kodu](./kody_do_wykladu/w7_1.rs)

### 1. Dlaczego `Eq` nie jest zdefiniowane dla `f64`?
- `f64` (liczby zmiennoprzecinkowe) nie spełniają ścisłego równości (`Eq`), bo mają specjalną wartość **NaN** ("Not a Number").
- W Rust (i matematycznie) zachodzi: \
$NaN==𝑥$ jest zawsze false
nawet gdy 
$x=NaN$.
- Dlatego `f64` implementuje tylko `PartialEq`, a nie `Eq`.

### 2. Przykładowe cechy (`traits`):
- **`Debug`**
    - Pozwala formatować strukturę jako tekst przy pomocy `{:?}`.\
    (np. w `println!("{:?}", zmienna)`).

- **`PartialEq`**
    - Umożliwia porównywanie (`==`, `!=`), ale nie gwarantuje, że każda wartość jest równa samej sobie (bo np. **NaN != NaN**).

- **`Clone`**
    - Pozwala tworzyć kopię zmiennej ręcznie (`clone()`).

- **`Default`**
    - Pozwala stworzyć **domyślną wartość** (np. wszystkie liczby = 0.0).

- **`Hash`**
    - Pozwala na tworzenie hasha wartości (np. do użycia w `HashMap`).

- **`Eq`** (dziedziczy po `PartialEq`)
    - Gwarantuje, że dla każdej wartości x, będzie:\
    $x==x (zawsze true)$\
    `f64` tego nie spełnia (przez NaN).

- **`Copy`** (dziedziczy po `Clone`)
    - Pozwala automatycznie kopiować zmienną bez wywoływania `clone()`, przy zwykłym przypisaniu.\
    (`let b = a;` – `a` nadal istnieje).

- **`PartialOrd`** (dziedziczy po `PartialEq`)
    - Pozwala na częściowe porównania (`<`, `>`, `<=`, `>=`), ale np. NaN nie da się sensownie porównać.

- **`Ord`** (dziedziczy po `PartialOrd` i `Eq`)
    - Umożliwia **pełne porządkowanie** wszystkich wartości — każda wartość musi być "większa", "mniejsza" lub "równa" innej.\
    (np. potrzebne w sortowaniu).


```rs
#[derive(Hash)]
struct Unitarna;

fn main() {
    let u = Unitarna;
}
```

# Wykład 8

📘 Typy Iloczynowe i Sumowe w Rust\
🔹 struct – Typ Iloczynowy (Product Type)
```rs
struct S {
    a: bool,
    b: u8,
}
```
📌 Analiza:
- `bool` ma 2 możliwe wartości: `true` lub `false`
- `u8` ma 256 możliwych wartości: od `0` do `255`
- Struktura `S` zawiera oba pola, więc liczba możliwych kombinacji wynosi:

> **2 × 256 = 512**

🧠 Wniosek:
- `struct` w Rust tworzy **typ iloczynowy** — zbiór wartości to iloczyn kartezjański zbiorów pól składowych.
- Każde pole musi istnieć i mieć jakąś wartość.
- Przykład możliwej wartości:
    ```rs
    S { a: true, b: 42 }
    ```

🔸 `enum` – Typ Sumowy (Sum/Union Type)
```rs
enum E {
    A(bool),
    B(u8),
}
```
📌 Analiza:
- Wariant `A(bool)` ma **2** wartości
- Wariant `B(u8)` ma **256** wartości
- W sumie enum `E` może przyjmować:
> **2 + 256 = 258 różnych wartości**

🧠 Wniosek:
- `enum` w Rust tworzy **typ sumowy (unijny)** — wartość może być jednym z wariantów, a nie wszystkimi naraz.
- W danym momencie enum ma dokładnie jeden wariant.
- Przykłady wartości:
    ```rs
    E::A(false)
    E::B(128)
    ```
🔄 Porównanie
Cechy	|`struct`	|`enum`
--|--|--
Forma	|Typ iloczynowy (product)	|Typ sumowy (union/sum)
Ilość danych	|Wszystkie pola obecne	|Tylko jeden wariant na raz
Liczba wartości	|Iloczyn liczności pól	|Suma liczności wariantów
Przykład wartości	|`{ a: true, b: 10 }`	|`A(false) lub B(42)`

## 🧮 Kalkulator ONP (RPN) w Rust
[Zobacz cały kod](./kody_do_wykladu/w8_1.rs)

📦 Struktury i aliasy
```rs
struct Element {
    rodzaj: char, // '+', '-', '*', '/', 'L'
    wartosc: f64
}

type Stos = Vec<f64>;
```
🔹 `Element`
- Reprezentuje pojedynczy token w wyrażeniu ONP.
- `rodzaj`: znak oznaczający rodzaj — literał (`'L'`) lub operator (`+`, `-`, `*`, `/`).
- `wartosc`: używana tylko, jeśli `rodzaj == 'L'`.

🔹 `Stos`
- Alias na stos (wektor f64), używany do przechowywania tymczasowych wartości podczas obliczeń.

⚙️ Funkcja wykonaj_dzialanie
```rs
fn wykonaj_dzialanie(rodzaj: char, a: f64, b: f64) -> f64 {
    if rodzaj == '+' {
        a + b
    } else if rodzaj == '-' {
        a - b
    } else if rodzaj == '*' {
        a * b
    } else if rodzaj == '/' {
        a / b
    } else {
        panic!("nieznany znak działania")
    }
}
```
- Wykonuje podstawowe działanie matematyczne na dwóch liczbach.
- Obsługuje operatory: `+`, `-`, `*`, `/`.
- W razie nieznanego operatora – program panikuje.
🧠 Funkcja `oblicz_onp`
```rs
fn oblicz_onp(kolejka: &Vec<Element>) -> Option<f64> {
    let mut stos = Stos::new();
    for e in kolejka {
        if e.rodzaj == 'L' {
            stos.push(e.wartosc);
        } else {
            let b = stos.pop()?;
            let a = stos.pop()?;
            let wynik = wykonaj_dzialanie(e.rodzaj, a, b);
            stos.push(wynik);
        }
    }
    let wynik_koncowy = stos.pop();
    if !stos.is_empty() {
        return None;
    }
    return wynik_koncowy;
}
```
🔄 Przebieg działania:
1. Tworzy pusty stos (`Vec<f64>`).
2. Iteruje po kolejce elementów:
    - Jeśli `rodzaj == 'L'`: wrzuca `wartosc` na stos.
    - W przeciwnym razie (operator): pobiera dwie liczby ze stosu (`a`, `b`), wykonuje działanie i wynik wrzuca z powrotem na stos.
3. Po przetworzeniu:
    - Zwraca `Some(wynik)` jeśli stos zawiera dokładnie jedną wartość (prawidłowe wyrażenie ONP).
    - W przeciwnym razie zwraca `None` (np. za mało operandów lub nadmiar).

❗ Bezpieczne operacje:
- Użycie `pop()?` powoduje automatyczne zakończenie (`None`) gdy stos jest zbyt płytki (zbyt mało operandów).

🧪 Przykłady w `main`
```rs
let mut k = Vec::new();
k.push(Element{rodzaj:'L', wartosc:34.5});
println!("{:?}", oblicz_onp(&k));
```
✔️ Wyrażenie: `34.5` → wynik: `Some(34.5)`
```rs
k.push(Element{rodzaj:'L', wartosc:1.0});
k.push(Element{rodzaj:'L', wartosc:0.5});
k.push(Element{rodzaj:'+', wartosc:0.0});
k.push(Element{rodzaj:'*', wartosc:0.0});
println!("{:?}", oblicz_onp(&k));
```
✔️ Wyrażenie: `34.5 1.0 0.5 + *` → wynik: `Some(51.75)`
```rs
let k1 = Vec::new();
println!("{:?}", oblicz_onp(&k1));
```
❌ Puste wyrażenie — brak wyniku → `None`
```rs
let mut k2 = Vec::new();
k2.push(Element{rodzaj:'L', wartosc:1.0});
k2.push(Element{rodzaj:'L', wartosc:0.5});
println!("{:?}", oblicz_onp(&k2));
```
❌ Zbyt dużo operandów, brak operatora → `None`
```rs
let mut k3 = Vec::new();
k3.push(Element{rodzaj:'L', wartosc:1.0});
k3.push(Element{rodzaj:'L', wartosc:0.5});
k3.push(Element{rodzaj:'+', wartosc:0.0});
k3.push(Element{rodzaj:'+', wartosc:0.0});
k3.push(Element{rodzaj:'+', wartosc:0.0});
println!("{:?}", oblicz_onp(&k3));
```
❌ Za dużo operatorów — zbyt mało operandów do działania → None

## 🔄 Zmiany w kalkulatorze ONP — wersja z `enum Rodzaj`
[Zobacz cały kod](./kody_do_wykladu/w8_2.rs)

🆚 Co się zmieniło?
Element kodu	|Wcześniej	|Teraz
--|--|--
Reprezentacja operatorów	|`char` (`'+', '-', '*', '/'`)	|`enum Rodzaj` z wariantami
Sprawdzanie rodzaju operacji	|porównania `char` (`== '+'`)	|porównania `Rodzaj` (`== Rodzaj::Plus`)
Literał	|`'L'` jako `char`	|osobny wariant `Rodzaj::Liczba`

### 🧱 Nowe definicje
🧾 Enum Rodzaj
```rs
#[derive(PartialEq, Copy, Clone)]
enum Rodzaj {
    Plus,
    Minus,
    Razy,
    Podzielic,
    Liczba
}
```
- Zastępuje nieczytelne literały znakowe (`char`) symbolicznymi nazwami.
- Dzięki `derive(PartialEq, Copy, Clone)` można je porównywać i kopiować.

### 🔧 Struktura Element
```rs
struct Element {
    rodzaj: Rodzaj,
    wartosc: f64
}
```
- Teraz `rodzaj` ma typ `Rodzaj`, a nie `char`.

### 🧠 Jak to teraz działa?
1. Literały (liczby) mają `rodzaj: Rodzaj::Liczba` — są wrzucane na stos.
2. Operatory mają inne warianty enum (`Plus`, `Minus`, `Razy`, `Podzielic`) — zdejmują dwie liczby ze stosu, wykonują operację i wynik wrzucają z powrotem.
3. Funkcja `oblicz_onp`:
- Zwraca `Some(wynik)` jeśli na końcu stos zawiera tylko jedną liczbę.
- Zwraca `None`, jeśli wyrażenie jest niepoprawne (np. za dużo operandów, brak operatorów, pusta kolejka).

### ✅ Zalety nowego podejścia
- Czytelność: `Rodzaj::Plus` jest bardziej opisowy niż `'+'`.
- Bezpieczeństwo typów: enum ogranicza możliwe wartości `rodzaj` do zdefiniowanych opcji.
- Mniejsza szansa na literówki: nie ma ryzyka przypadkowego wpisania złego znaku.
- Lepsze dopasowanie do Rustowego stylu: enumy to preferowany sposób wyrażania dyskretnych wyborów.

### 📊 Przykład użycia:
```rs
k.push(Element{rodzaj:Rodzaj::Liczba, wartosc:1.0});
k.push(Element{rodzaj:Rodzaj::Liczba, wartosc:0.5});
k.push(Element{rodzaj:Rodzaj::Plus, wartosc:0.0});
k.push(Element{rodzaj:Rodzaj::Razy, wartosc:0.0});
```

## 🔄 Zastąpienie `struct Element` przez `enum Element`
[Zobacz cały kod](./kody_do_wykladu/w8_3.rs)

### ✅ Co się zmieniło?
Poprzednia wersja	|Obecna wersja
--|--
`Rodzaj` i `Element` to osobne struktury	|Wszystko zintegrowane w jednym `enum Element`
`Element` miał dwa pola: `rodzaj` i `wartosc`	|`Element` to enum z wariantem Liczba(f64)
Dane liczby i operatora były oddzielne	|Teraz `Liczba` zawiera wartość wewnątrz enum
Sprawdzenie `e.rodzaj == Rodzaj::Liczba`	|Sprawdzenie `if let Element::Liczba(w) = e`
`wartosc: 0.0` dla operatorów (dummy value)	|Usunięte — operator nie potrzebuje liczby

### 🧱 Nowa definicja `Element`
```rs
#[derive(PartialEq, Copy, Clone)]
enum Element {
    Plus,
    Minus,
    Razy,
    Podzielic,
    Liczba(f64)
}
```
- Wariant `Liczba(f64)` przechowuje wartość liczby wewnątrz enumu.
- Pozostałe warianty reprezentują działania i nie potrzebują osobnego pola `wartosc: f64`.
### ✨ Jak to działa teraz?
Główna pętla w `oblicz_onp`
```rs
if let Element::Liczba(wartosc) = *e {
    stos.push(wartosc);
} else {
    let b = stos.pop()?; let a = stos.pop()?;
    let wynik = wykonaj_dzialanie(*e, a, b);
    stos.push(wynik);
}
```
- Sprawdzamy, czy `Element` jest liczbą — jeśli tak, wyciągamy wartosc i wrzucamy na stos.
- W przeciwnym razie traktujemy go jako operator i wykonujemy działanie.
- Warto zwrócić uwagę na ten fragment:
    ```rs
    if let Element::Liczba(wartosc) = *e
    ```
    - to tzw. dopasowanie z destrukturyzacją (ang. pattern matching) w skróconej formie — używamy go, by sprawdzić, czy `e` ma konkretny wariant enumu i jednocześnie wydobyć dane (w tym przypadku wartość `f64`).
    - Krok po kroku:
        1. `e` to referencja (`&Element`) — dlatego używamy `*e`, żeby dostać się do samej wartości `Element`.
        2. `if let Element::Liczba(wartosc) = *e`:
            - sprawdza, czy `*e` to wariant `Liczba`.
            - jeśli tak, wyciąga wartość i przypisuje ją do zmiennej `wartosc`.
            - jeśli nie, kod w bloku `if` jest pomijany — przechodzimy do `else`.

## 🧠 Refaktoryzacja: Zmiana z `if else if` na `match` w `wykonaj_dzialanie`
[Zobacz cały kod](./kody_do_wykladu/w8_4.rs)

[Zobacz cały kod (z dodaniem potęgi)](./kody_do_wykladu/w8_5.rs)

W tej wersji kodu nastąpiła refaktoryzacja funkcji wykonaj_dzialanie, która teraz używa match zamiast serii instrukcji if else if.

Co się zmieniło?
- Było:
```rs
if rodzaj == Element::Plus {
    a + b
} else if rodzaj == Element::Minus {
    ...
}
```
- Jest:
```rs
match rodzaj {
    Element::Plus => a + b,
    Element::Minus => a - b,
    ...
    Element::Liczba(_) => panic!("niespodziewana Liczba (z typu Element)")
}
```
Zalety `match`:
- bardziej przejrzysta i idiomatyczna w Rust,
- wymusza pełną obsługę wszystkich wariantów `enum`,
- łatwiej ją rozszerzać i utrzymywać,
- mniej podatna na pomyłki (np. przypadkowe pominięcie wariantu).
- zabezpiecza program przed nieprawidłowym użyciem wariantu `Element::Liczba` w miejscu, gdzie oczekiwany jest operator (np. `Plus`, `Razy`, `Potega`).

## 🧠 Użycie use `crate::Element::{...}` do skrócenia nazw enumów w Rust
[Zobacz cały kod](./kody_do_wykladu/w8_6.rs)

📌 Cel zmiany\
W kodzie zamiast pisać pełne ścieżki typu `Element::Plus`, `Element::Liczba(...)` itd., zastosowano:
```rs
use crate::Element::{Plus, Minus, Razy, Podzielic, Potega, Liczba};
```
co pozwala używać wariantów enuma `Element` bez prefiksu `Element::`, czyli:
```rs
k.push(Liczba(2.0));
k.push(Potega);
```
zamiast:
```rs
k.push(Element::Liczba(2.0));
k.push(Element::Potega);
```
✅ Zalety takiego podejścia
- Czytelność – kod jest krótszy i mniej zaszumiony.
- Mniej powtórzeń – unika się wielokrotnego pisania `Element::`.
- Bezpieczna kontrola – wybierając konkretne warianty (a nie całe `Element::*`), łatwiej śledzić, co dokładnie jest używane w tym pliku.

⚠️ Dlaczego `use crate::Element::*` jest gorsze?\
Użycie:
```rs
use crate::Element::*;
```
importuje wszystkie warianty `Element` do bieżącego zakresu – co:
- może prowadzić do konfliktów nazw,
- zaciera granice tego, skąd pochodzą symbole,
- utrudnia czytanie i analizowanie kodu, zwłaszcza w większych projektach.

🧭 Dobre praktyki
- ✅ Używaj jawnego importu konkretnych wariantów (`{Plus, Minus, ...}`) – jak w tym przykładzie.
- ❌ Unikaj `use enum::*`, zwłaszcza w większych projektach lub plikach z wieloma zależnościami.


```rs
fn komunikat(a: &Option<char>) {
    if !a.is_none() {
        println!("znak: {}", a.unwrap());
    } else {
        println!("PUSTO");
    }
}


fn main() {
    let x: Option<char> = None;
    let y: Option<char> = Some('y');
    
    komunikat(&x);
    komunikat(&y);
}
```
### 🧠 Cel programu
Program definiuje funkcję `komunikat`, która przyjmuje referencję do zmiennej typu `Option<char>` i wypisuje:
- zawarty znak, jeśli jest dostępny (`Some(char)`),
- komunikat `"PUSTO"`, jeśli wartość to `None`.

### 🔧 Funkcja komunikat
- `a: &Option<char>` – funkcja przyjmuje referencję do `Option<char>`, czyli nie kopiuje ani nie przenosi zawartości.
- `a.is_none()` – metoda sprawdza, czy `a` jest równe `None`.
- `!a.is_none()` – czyli: jeśli `a` nie jest puste, to...
- `a.unwrap()` – rozpakowuje wartość z `Some(char)`. Uwaga: wywołanie `unwrap()` na `None` spowodowałoby panic!, ale tu jest ono bezpieczne, bo wcześniej warunek `!a.is_none()` to sprawdził.

### 🧠 Co się dzieje w `match a { ... }`?
```rs
fn komunikat(a: &Option<char>) {
    match a {
        Some(znak)  => { println!("znak: {}", znak); }
        None        => { println!("PUSTO"); }
    }
}
```
🔹 `match` to konstrukcja dopasowania wzorców:
- Sprawdza możliwe warianty wartości zmiennej `a`, która jest typu `&Option<char>`.

🔸 `Some(znak)` – wzorzec dopasowuje się, gdy w `a` jest `Some(c)`:
- Wyciąga zawarty znak do zmiennej znak.
- Następnie wypisuje go: `znak: <wartość>`.

🔸 `None` – dopasowuje się, gdy `a` jest `None`:
- Wypisuje `"PUSTO"`.

✅ Zalety użycia `match` zamiast `unwrap` + `is_none`:
Cecha	|`match`	|`unwrap` + `is_none`
--|--|--
Bezpieczne	|✅ nie grozi `panic!`	|⚠️ `unwrap()` może się wywalić
Idiomatyczne dla Rusta	|✅ tak	|❌ mniej zalecane
Przejrzystość semantyczna	|✅ dopasowanie do wariantów enum	|❌ bardziej proceduralne

### 🧠 Co robi `if let Some(znak) = a`?
```rs
fn komunikat(a: &Option<char>) {
    if let Some(znak) = a {
        println!("znak: {}", znak);
    } else {
        println!("PUSTO");
    }
}
```
🔹 `if let Some(znak) = a`
- Sprawdza, czy zmienna `a` (referencja do `Option<char>`) zawiera wartość (`Some(znak)`).
- Jeśli tak, wypisuje tę wartość.
- W przeciwnym razie (`else`), wypisuje `"PUSTO"`.

✅ Zalety `if let`
- Mniej kodu, gdy interesuje Cię tylko jeden przypadek (`Some`).
- Bardzo czytelne i naturalne.
- Unikasz użycia `unwrap()`, czyli nie ryzykujesz błędu wykonania (`panic!`).

📌 Kiedy używać `if let`, a kiedy `match`?
Potrzebujesz obsłużyć…	|Użyj
--|--
Tylko jeden przypadek	|`if let`
Więcej niż jeden wariant	|`match`
Wszystkie możliwe warianty	|`match`

Ta sama zasada działania co wyżej ale z użyciem `Result<...>`, zamiast `Option<...>` 
```rs
fn komunikat(a: &Result<char, String>) {
    match a {
        Ok(znak)    => { println!("znak: {}", znak); }
        Err(blad)   => { println!("błąd: {}", blad); }
    }
}


fn main() {
    let x: Result<char, String> = Err("brak znaku".to_string());
    let y: Result<char, String> = Ok('y');
    
    komunikat(&x);
    komunikat(&y);
}
```
W tym kodzie mamy do czynienia z bardziej złożoną wersją typu Option, czyli Option<Option<T>>. To oznacza, że mamy opcję, która może zawierać inną opcję.
```rs
fn komunikat(a: &Option<Option<char>>) {
    match a {
        Some(Some(znak))  => { println!("znak: {:?}", znak); }
        Some(None)        => { println!("PRAWIE PUSTO"); }
        None              => { println!("PUSTO"); }
    }
}


fn main() {
    let x: Option<Option<char>> = None;
    let y: Option<Option<char>> = Some(Some('y'));
    let z: Option<Option<char>> = Some(None);
    
    komunikat(&x);
    komunikat(&y);
    komunikat(&z);
}
```
🧠 Co się dzieje w kodzie?
1. Funkcja komunikat
    - Ta funkcja bierze referencję do `Option<Option<char>>` i sprawdza, jaki ma stan.
    - `Some(Some(znak))`: Jeśli opcja zawiera inną opcję, która zawiera wartość (np. `Some('y')`), wypisuje tę wartość.
    - `Some(None)`: Jeśli opcja zawiera inną opcję, ale ta opcja jest pusta (np. `Some(None)`), wypisuje `"PRAWIE PUSTO"`.
    - `None`: Jeśli zewnętrzna opcja jest pusta (np. None), wypisuje `"PUSTO"`.
2. Przypadki w `main`
    - `x = None`: Zewnętrzna opcja jest pusta, więc wypisuje `"PUSTO"`.
    - `y = Some(Some('y'))`: Zewnętrzna opcja zawiera wewnętrzną opcję, która zawiera znak `'y'`, więc wypisuje `"znak: 'y'"`.
    - `z = Some(None)`: Zewnętrzna opcja zawiera wewnętrzną opcję, ale ta wewnętrzna jest pusta, więc wypisuje `"PRAWIE PUSTO"`.

### 📌 Dlaczego Option<Option<T>>?
Można używać `Option<Option<T>>` w sytuacjach, gdzie chcesz reprezentować:
- **Brak wartości** (`None`),
- **Wartość obecna** (zawierająca inną opcję: `Some(Some(T))`),
- **Pustą wartość** (np. `Some(None)`), gdy masz przypadek, w którym coś mogło zostać wstępnie ustalone, ale ostatecznie jest puste.

Tego typu konstrukcja jest użyteczna w bardziej skomplikowanych sytuacjach, ale trzeba uważać, by nie wprowadzać zbędnej złożoności.

```rs
fn komunikat(a: &Option<Option<char>>) {
    match a {
        Some(Some(znak))  => { println!("znak: {:?}", znak); }
        Some(_)           => { println!("PRAWIE PUSTO"); }
        _                 => { println!("PUSTO"); }
    }
}
```
to uproszczona wersja poprzedniego match, która wykorzystuje symbol podkreślenia (_) jako catch-all, czyli dopasowanie ogólne, gdy inne warianty nie pasują.

### 🔍 Co się tu dzieje:
- `Some(Some(znak))` — jeśli mamy `Some(Some('x'))`, to wypisujemy wartość znaku.
- `Some(_)` — jeśli mamy `Some(None)`, czyli zewnętrzne `Some`, ale wewnętrzna opcja jest `None`, to wypisujemy `"PRAWIE PUSTO"`.
- `_` — wszystko inne, czyli None, wypisuje `"PUSTO"`.

### ✅ Efekt działania jest identyczny jak wcześniej, ale kod jest:
- krótszy,
- bardziej zwięzły,
- mniej precyzyjny (bo `Some(None)` i inne potencjalne nietypowe `Some(_)` traktuje tak samo).

### 🔑 Uwagi:
- Ten styl sprawdza się, jeśli nie chcesz rozróżniać dokładnie każdego przypadku Some, tylko zależy Ci na obsłużeniu najważniejszego (`Some(Some(...))`) i reszty ogólnikowo.
- Jest bardziej czytelny w prostych przypadkach.

```rs
fn komunikat(a: &Result<Result<char, String>, String>) {
    match a {
        Ok(Ok(znak))  => { println!("znak: {:?}", znak); }
        Ok(_)         => { println!("PRAWIE PUSTO"); }
        _             => { println!("PUSTO"); }
    }
}


fn main() {
    let x = Err("?".to_string());
    let y = Ok(Ok('y'));
    let z = Ok(Err("???".to_string()));
    
    komunikat(&x);
    komunikat(&y);
    komunikat(&z);
}
```
W tym przykładzie funkcja `komunikat` operuje na zagnieżdżonym typie: `Result<Result<char, String>, String>`. To oznacza, że mamy wynik w wyniku – czyli coś, co może zakończyć się błędem na dwóch poziomach.
### 🔍 Struktura danych
- Zewnętrzne `Result<_, String>`
    - `Ok(...)` — sukces, więc sprawdzamy wartość wewnętrzną
    - `Err(...)` — błąd zewnętrzny
- Wewnętrzne `Result<char, String>`
    - `Ok('x')` — sukces, mamy znak
    - `Err(...)` — błąd wewnętrzny

🔎 Działanie funkcji
- `Ok(Ok(znak))` — sukces zewnętrzny i wewnętrzny → wypisz znak
- `Ok(_)` — sukces zewnętrzny, ale wewnętrzny to `Err(...)` → "PRAWIE PUSTO"
- `_` — wszystko inne (czyli `Err(...)` na zewnątrz) → "PUSTO"

### 🧠 Co to pokazuje?
To ćwiczenie dobrze ilustruje jak działa **zagnieżdżone dopasowanie** `match` i jak można czytelnie oddzielić różne poziomy sukcesu/błędu. Przykład jest bardzo typowy dla kodu np. z funkcjami, które mogą zwracać błąd przy otwieraniu pliku (`Result`) i dodatkowo mogą zwracać błędne dane (`Result` wewnątrz `Ok`).

# Wykład 9

[Kod - input, całość kodu](./kody_do_wykladu/w9_1.rs)

#### Szczegółowy opis:
#### `main` (funkcja główna)
```rs
let imie = wczytaj_napis("Imię? ");
let wiek = wczytaj_usize("Ile masz lat? ");
let ul = wczytaj_f64("Ulubiona liczba? ");
println!("Cześć, {imie}, lat {wiek}!");
println!("Twoja ulubiona liczba: {ul}...");
```
- `wczytaj_napis` – prosi użytkownika o wpisanie imienia.
- `wczytaj_usize` – prosi użytkownika o wiek i konwertuje go do liczby całkowitej `usize`.
- `wczytaj_f64` – prosi użytkownika o ulubioną liczbę i konwertuje ją do liczby zmiennoprzecinkowej `f64`.
#### Funkcja `wczytaj_napis(prompt: &str) -> String`
```rs
let mut odp = String::new();
print!("{prompt}");
std::io::stdout().flush().expect("???: problem z flush");
std::io::stdin().read_line(&mut odp).expect("???: problem z read_line");
//return odp.trim_end_matches('\n').to_string();
return odp.trim_end().to_string();
```
- Wyświetla prompt (np. "Imię?").
- `flush()` jest konieczny, żeby `print!` natychmiast pokazał tekst.
- `read_line` wczytuje linię z wejścia i zapisuje ją do `odp`.
- `trim_end()` usuwa końcowy znak nowej linii.
    - To usuwa wszystkie białe znaki z końca, czyli:
        - `\n` – znak nowej linii,
        - `\r` – znak powrotu karetki,
        - `\t` – tabulacja,
        - ` ` spacja.
    - ⚠️Na Windowsie wczytanie linii z `stdin.read_line()` kończy się znakiem `\r\n`. Użycie `odp.trim_end_matches('\n')` usunie tylko `\n`, zostawiając `\r` — co może skutkować np. dziwnymi efektami przy porównywaniu ciągów czy parsowaniu.
- Zwraca oczyszczony tekst jako String.

#### Funkcja `wczytaj_usize(prompt: &str) -> usize`
```rs
loop {
    let odp = wczytaj_napis(prompt);
    if let Ok(wynik) = odp.parse() {
        return wynik;
    } else {
        println!("Błąd, podaj ponownie!");
    }
}
```
- Pętla: pyta użytkownika, aż poda poprawną liczbę całkowitą.
- `parse()` próbuje skonwertować `String` na `usize`.
- `if let Ok(...)` sprawdza, czy konwersja się powiodła.
- W razie błędu użytkownik widzi komunikat i jest pytany ponownie.

#### Funkcja `wczytaj_f64(prompt: &str) -> f64`
```rs
loop {
    let odp = wczytaj_napis(prompt);
    if let Ok(wynik) = odp.parse::<f64>() {
        return wynik;
    } else {
        println!("Błąd, podaj ponownie!");
    }
}
```
- Wyświetla `prompt`, np. „Ulubiona liczba?”.
- Wczytuje tekst od użytkownika za pomocą `wczytaj_napis`.
- Próbuje zamienić go na liczbę zmiennoprzecinkową `f64` za pomocą: `odp.parse::<f64>()`
- Jeśli się uda – zwraca tę liczbę.
- Jeśli nie – informuje o błędzie i pyta ponownie.

#### 🔍 Co robi `odp.parse::<f64>()`?
Funkcja `.parse::<f64>()` to uniwersalna metoda konwertująca `String` (lub `&str`) na dowolny typ implementujący trait `FromStr`.
W tym przypadku:
- `::<f64>` to tzw. "turbofish" – jawne podanie typu, na który ma być dokonana konwersja.
- Zwraca `Result<f64, _>` – czyli:
    - `Ok(f)` jeśli udało się sparsować liczbę,
    - `Err(_)` jeśli napotkano błąd (np. nieprawidłowy format liczby).

#### 🔁 Alternatywa bez turbofish:
Można napisać po prostu:
```rs
let wynik: Result<f64, _> = odp.parse();
```
ale wtedy typ musi być jednoznaczny z kontekstu (np. poprzez przypisanie do zmiennej typu f64 lub poprzez sygnaturę funkcji).
#### 📘 Przykład działania:
```rs
Ulubiona liczba? 3.1415
→ OK, zwraca 3.1415 (f64)

Ulubiona liczba? abc
→ Błąd, podaj ponownie!
```

[Kod - implementacja niestandardowych porównań dla struktury, całość kodu](./kody_do_wykladu/w9_2.rs)
#### 🔧 Struktura
```rs
struct S(u8, u8);
```
To tzw. **struktura krotek (tuple struct)**. Przechowuje dwa `u8`, dostępne przez `.0` i `.1`.
#### 🟰 Implementacja PartialEq
```rs
impl PartialEq for S {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}
```
- Określa, kiedy dwa obiekty `S` są sobie równe.
- Porównywane jest tylko drugie pole `self.1`, ignorując `self.0`.
Przykład:
```rs
S(2, 1) == S(3, 1) → true  // bo oba mają `.1 == 1`
```
#### 📊 Implementacja PartialOrd
```rs
impl PartialOrd for S {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.1.partial_cmp(&other.1)
    }
}
```
- Pozwala porównywać (`<`, `>`, `<=`, `>=`) obiekty `S` według drugiego pola (`.1`).
- Używa `partial_cmp`, które zwraca `Some(Ordering)` lub `None`, np. w przypadku NaN (dla typów zmiennoprzecinkowych) — ale tutaj mamy `u8`, więc zawsze zwraca `Some(...)`.
#### ▶️ Funkcja main
```rs
let s1 = S(1, 5);
let s2 = S(2, 1);
let s3 = S(3, 1);
```
Trzy instancje `S`, z różnymi `.0`, ale `s2` i `s3` mają takie samo `.1`.
#### 🧪 Porównania:
```rs
println!("{}", s1 == s2);   // false
println!("{}", s1 == s3);   // false
```
→ bo `5 != 1`
```rs
println!("{}", s3 == s2);   // true
```
→ bo oba mają `.1 == 1`
#### 🔽 Porównania porządkowe (`<`):
```rs
println!("{}", s1 < s2);    // false
println!("{}", s1 < s3);    // false
```
→ `5` nie jest mniejsze niż `1`
```rs
println!("{}", s3 < s2);    // false
```
→ `1 < 1` to fałsz
#### 📌 Podsumowanie:
- `S(2, 1) == S(3, 1)` → `true`, bo porównuje się tylko `.1`
- `S(1, 5) < S(2, 1)` → `false`, bo `5 > 1`
- `.0` (pierwsze pole) nie ma żadnego znaczenia przy porównaniach

[kod -szkic gry w kółko i krzyżyk (Tic-Tac-Toe) (niedokonczony)](./kody_do_wykladu/w9_3.rs)
### 🧩 Elementy gry
#### 🎯 Enum Pionek
```rs
enum Pionek {
    Kolko,
    Krzyzyk,
}
```
- Przedstawia możliwe typy pionków na planszy.
- To, co w klasycznej grze oznaczamy jako `O` i `X`.
#### 📦 Enum Pole
```rs
enum Pole {
    Puste,
    Zajete(Pionek),
}
```
- Jedno pole planszy może być:
    - puste, lub
    - zajęte przez pionek (`Kolko` lub `Krzyzyk`).
- Umożliwia łatwe sprawdzanie stanu planszy.
#### 🧩 Struktura Plansza
```rs
struct Plansza(Vec<Vec<Pole>>);
```
- To dwuwymiarowa siatka pól – najpewniej 3×3 dla klasycznego Tic-Tac-Toe.
- Przechowuje stan każdej komórki.
- Opakowana w strukturę, co pozwala dodać metody np. `wykonaj_ruch`, `czy_koniec_gry`.
#### 🎮 Struktura Gra
```rs
struct Gra {
    plansza: Plansza,
    interfejs: InterfejsTekstowy,
    gracze: Vec<GraczCzlowiek>,
    indeks_biezacego_gracza: usize,
}
```
- Reprezentuje pełen stan gry.
- Zawiera:
    - planszę,
    - interfejs do komunikacji z użytkownikiem,
    - dwóch graczy,
    - informację, który gracz teraz wykonuje ruch.
#### 🧑 Struktura GraczCzlowiek
```rs
struct GraczCzlowiek {
    pionek: Pionek,
    imie: String,
}
```
- Opisuje jednego gracza:
    - jego pionek (`Kolko` lub `Krzyzyk`),
    - imię (np. do wyświetlenia w UI).
#### 🖥️ Struktura InterfejsTekstowy
```rs
struct InterfejsTekstowy;
```
- Prawdopodobnie odpowiada za komunikację tekstową z użytkownikiem (terminal).
- Będzie miał metody typu:
    - `pobierz_ustawienia()`,
    - `wyswietl_plansze()`,
    - `zapytaj_o_ruch()`.
### ▶️ Funkcja main
```rs
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
```
Co tu się dzieje:
- Tworzy się interfejs tekstowy.
- Pobierane są ustawienia (najpewniej imiona graczy, wybór pionka, itp.).
- Tworzona jest nowa gra z dwoma graczami, planszą i interfejsem.
- `gra?.graj()` – uruchamia główną pętlę gry (jeśli `gra` to `Result`, używany jest `?` do propagacji błędów).
- Program kończy się sukcesem (`Ok(())`), jeśli nie wystąpił błąd.

### 📌 Uwagi
- Niekompletność: brakuje:
    - implementacji metod jak `Gra::new`, `Gra::graj`,
    - struktury Ustawienia (która zawiera `utworz_gracza_o()` itd.),
    - metod interfejsu (`pobierz_ustawienia()`).
- Kod będzie uzupełniony na kolenym wykładzie :)

# Wykład 10
[w10_1 - gra w kółko i krzyżyk (Tic-Tac-Toe)](./kody_do_wykladu/w10_1.rs)

Ten kod to pełna implementacja gry w **kółko i krzyżyk** (Tic-Tac-Toe) w języku **Rust**. Został napisany w stylu imperatywno-obiektowym, ale dobrze pokazuje idiomy Rustowe, jak np. `enum`, `match`, `Result`, `Option`, `Vec`, a także użycie **klonowania**, **kopiowania**, i wypisywania na konsolę.

### 🔧 Ogólna struktura programu
Program składa się z kilku głównych komponentów:
1. **Pionek** (`enum Pionek`) – reprezentuje gracza (kółko lub krzyżyk)
2. **Pole** (`enum Pole`) – jedno pole na planszy: puste lub zajęte
3. **Plansza** – 2D `Vec` pól, z metodami do ruchów i sprawdzania końca gry
4. **Gracze** – dwie instancje struktury `GraczCzlowiek`
5. **Interfejs** – wejście/wyjście tekstowe przez terminal
6. **Gra** – zarządza przebiegiem partii
7. `main()` – inicjuje i uruchamia grę

## 📌 Kluczowe elementy i idiomy warte uwagi
### 1. `#[derive(Clone, PartialEq, Copy)]`
Użyte np. przy `enum Pionek` i `Pole`.
- `Copy` oznacza, że typ może być kopiowany bitowo (jak `i32`, `char`, etc.)
- `Clone` jest potrzebne, gdy chcemy wyraźnie klonować wartości (np. `x.clone()`)
- `PartialEq` pozwala porównywać wartości (`a == b`)

**Warto zapamiętać:** Rust nie klonuje automatycznie danych; domyślnie wszystko jest przenoszone (`move`), a `Copy` to wyjątek.
### 2. `repr` jako metoda pomocnicza
```rs
impl Pionek {
    fn repr(&self) -> char {
        match self {
            Self::Kolko => 'o',
            Self::Krzyzyk => 'x',
        }
    }
}
```
To częsty idiom w Rust – `repr` (lub `to_char`, `to_string`) jako sposób prezentacji wewnętrznej wartości do wyświetlania.
### 3. Użycie `match` + `enum`
```rs
impl Wynik {
    fn from_pole(pole: Pole) -> Self {
        match pole {
            Pole::Zajete(Pionek::Krzyzyk) => Self::Krzyzyk,
            Pole::Zajete(Pionek::Kolko) => Self::Kolko,
            Pole::Puste => panic!("to nie powinno się zdarzyć! [wygrana pustego pola?]"),
        }
    }
}
```
Silny typ wyliczeniowy + dopasowanie `match` pozwala pokryć każdy przypadek i wymusza dokładność.
### 4. Bezpieczne dostępy do tablicy
Plansza to:
```rs
struct Plansza {
    zaw: Vec<Vec<Pole>>,
    wynik: Option<Wynik>,
}
```
Dostęp do konkretnego pola odbywa się przez:
```rs
fn pole(&self, wsp: Wsp) -> Pole {
    self.zaw[wsp.0][wsp.1]
}
```
Nie ma tu sprawdzania `panic`, bo wcześniej metoda `czy_ruch_poprawny` sprawdza poprawność indeksów.
### 5. Idiom `Option` + `expect`
```rs
fn wynik_partii(&self) -> Wynik {
    self.wynik.expect("to nie powinno się zdarzyć! [sprawdzenie wyniku przed końcem partii?]")
}
```
`Option<T>` to bezpieczna alternatywa dla `null`. `expect` jest wygodne, ale w prawdziwej aplikacji warto użyć `match` lub `.unwrap_or(...)`.
### 6. Zmiana gracza: `1 - self.indeks_biezacego_gracza`
Bardzo zwięzły sposób na przełączanie między dwoma indeksami (0 ↔ 1).
```rs
fn zmien_gracza(&mut self) {
    self.indeks_biezacego_gracza = 1 - self.indeks_biezacego_gracza;
}
```
### 7. Wzorce `if let`, `while let`, `matches!`
```rs
if let Wynik::GraTrwa = self.wynik {
    // ...
}
```
Pozwala sprawdzić, czy wartość ma okreśony wariant `enum` i — opcjonalnie — wyciągnąć dane z wnętrza tego wariantu.
Zalety:
- Skrócona forma `match` dla pojedynczego przypadku.
- Czytelność: dobra do warunkowego wykonania kodu.
Alternatywa:
```rs
match self.wynik {
    Wynik::GraTrwa => { ... },
    _ => {},
}
```
Nie występuje w tym konkretnym kodzie, ale warto go znać. Przykład:
```rs
while let Some(ruch) = kolejka.pop() {
    // dopóki mamy coś w kolejce, wykonuj ruch
}
```
Łączy sprawdzanie dopasowania i pętlę `while`. Używany tam, gdzie chcemy iterować tak długo, jak długo zmienna ma konkretny wariant (np. `Option::Some`).

**Zastosowanie:** często przy ręcznej obsłudze iteratorów, kolejki, stosów.
```rs
fn zajete(&self) -> bool {
    matches!(self, Self::Zajete(_))
}
```
Makro, które zwraca `true` jeśli wartość pasuje do danego wzorca. To szybki sposób na sprawdzenie wariantu bez wyciągania zawartości.
Zalety:
- Zwrotnie daje bool, więc idealne do warunków, np. if, assert!.
- Zwięzłość.
### 8. Wczytywanie danych z terminala
```rs
fn wczytaj_napis(prompt: &str) -> String {
    let mut odp = String::new();
    print!("{}", prompt);
    std::io::stdout().flush().expect("fatalny problem ze standardowym wyjściem");
    std::io::stdin().read_line(&mut odp).expect("fatalny problem ze standardowym wejściem");
    // odp.trim_end_matches('\n').to_string()
    odp.trim_end().to_string()
}
```
Opis działania:
- `prompt: &str` — Tekst, który zostanie wyświetlony jako zachęta dla użytkownika (np. „Podaj imię: ”).
- `print!("{}", prompt);` + `stdout().flush()` — Wyświetla prompt bez nowej linii i wymusza jego wypisanie na ekranie przed oczekiwaniem na dane.
- `read_line(&mut odp)` — Czeka na wpisanie tekstu przez użytkownika i zapisuje go do zmiennej `odp`.
- `trim_end()` — Usuwa końcowe znaki białe (np. `\n`, `\r\n`) z końca wpisanego tekstu.
- `to_string()` — Zwraca gotowy napis jako `String`.
### 9. Formatowanie stringów
```rs
fn opis(&self) -> String {
    format!("{} ({})", self.imie, self.pionek.repr())
}
```
`format!("{} ({})", ...)` - To makro (nie funkcja), które działa podobnie do `printf` w C — ale zwraca `String`.
W `{}` trafia `self.imie` i `self.pionek.repr()` — czyli w efekcie dostajemy nowy tekst.
### 10. `Result<(), String>` w `main`
```rs
fn main() -> Result<(), String> { ... }
```
Pozwala zwracać błędy w stylu funkcyjnym – alternatywa dla panikowania. W `main()` to całkowicie legalne i eleganckie.
### 🎯 Dodatkowe uwagi
#### Sprawdzenie końca gry
Kod zakłada planszę 3x3 – nie jest ogólny, ale czytelny. Sprawdza zwycięstwo po ruchu na:
- przekątnych
- rzędach
- kolumnach
Zrobione przez:
```rs
// działa tylko dla tradycyjnego KiK (3x3)
if self.zaw[0][0].zajete()
    && (self.zaw[0][0] == self.zaw[0][1] && self.zaw[0][1] == self.zaw[0][2]
    || self.zaw[0][0] == self.zaw[1][1] && self.zaw[1][1] == self.zaw[2][2]
    || self.zaw[0][0] == self.zaw[1][0] && self.zaw[1][0] == self.zaw[2][0])
{
    self.wynik = Some(Wynik::from_pole(self.zaw[0][0]));
    return true;
}
```
#### Separacja odpowiedzialności
- `Plansza` nie wie nic o użytkowniku
- `InterfejsTekstowy` nie wie nic o logice gry
- `Gra` jest koordynatorem

#### 🧠 Czego można się nauczyć z tego kodu?
1. Jak działają `enum`, `match`, `Option`, `Result`, `Copy`, `Clone`, `PartialEq`
2. Jak pisać własne typy i metody (`impl`)
3. Jak zarządzać IO i użytkownikiem
4. Jak oddzielać logikę gry od interfejsu
5. Jak implementować proste reguły gry i ich sprawdzanie

#### 🧪 Możliwe ulepszenia
- Rozszerzenie planszy na rozmiar dynamiczny (np. 4x4)
- AI zamiast drugiego gracza
- Obsługa błędów bez `panic!`
- Użycie `enum` dla `Gracz` (człowiek vs AI)
- Refaktoryzacja sprawdzania końca gry (np. z użyciem iteratorów)

## Podział projektu na wiele plików

[w10_2 - gra w kółko i krzyżyk (Tic-Tac-Toe), podzielony na kilka plików](./kody_do_wykladu/w10_2)

```rs
pub mod gra;
mod gracz_czlowiek;
pub mod interfejs_tekstowy;
mod pionek;
mod plansza;
mod pole;
mod ruch;
mod ustawienia;
mod wynik;
```
To zawartość pliku `lib.rs` – czyli głównego punktu wejścia dla biblioteki w projekcie Rust.
### 🧩 `lib.rs` – co to?
To specjalny plik w Rustcie, który:
- jest głównym modułem biblioteki (`crate`),
- definiuje, co będzie dostępne na zewnątrz (publiczne API),
- musi się tak nazywać, jeśli tworzysz crate typu biblioteka (w przeciwieństwie do `main.rs` dla aplikacji).

📌 Uwaga: Nie musi istnieć zarazem `main.rs` i `lib.rs`, ale można mieć oba – wtedy projekt może być używany zarówno jako aplikacja, jak i biblioteka.

### 🔧 Co robią `mod` i `pub mod`?
✅ `mod nazwa;`
- Włącza moduł z pliku `nazwa.rs` (albo z folderu `nazwa/mod.rs`).
- Jest prywatny domyślnie – tylko bieżący moduł może go używać.

Przykład:
```rs
mod pionek;
```
→ Plik `pionek.rs` jest kompilowany i dostępny tylko wewnątrz `lib.rs`

✅ `pub mod nazwa;`
- Robi to samo co `mod`, ale udostępnia moduł na zewnątrz – innym modułom / crate’om.
- Tworzy część publicznego API biblioteki.

Przykład:
```rs
pub mod interfejs_tekstowy;
```
→ Ten moduł można potem użyć np. w `main.rs` tak:
```rs
use twoja_biblioteka::interfejs_tekstowy::wczytaj_napis;
```
### 📄 Gdzie znaleźć nazwę biblioteki ?
```toml
[package]
name = "twoja_biblioteka"
version = "0.1.0"
edition = "2021"
```
Nazwa `twoja_biblioteka` znajduje się w pliku `Cargo.toml`, jest to nazwa paczki i domyślnie także nazwa biblioteki, jeśli masz plik `src/lib.rs`.
#### 📚 Skąd się bierze use twoja_biblioteka::...?
Kiedy tworzysz plik `lib.rs`, to kompilator Rust traktuje jego zawartość jako kod biblioteczny. Inne binarki (np. `main.rs` w `src/` lub `src/bin/`) mogą go używać tak:
```rs
use twoja_biblioteka::interfejs_tekstowy::wczytaj_napis;
```
#### 🔧 Możesz też ją nadpisać
Jeśli chcesz, możesz jawnie określić nazwę biblioteki inaczej niż nazwa paczki:
```toml
[lib]
name = "inna_nazwa"
path = "src/lib.rs"
```
Wtedy musisz używać:
```rs
use inna_nazwa::interfejs_tekstowy::wczytaj_napis;
```
### 🔧 Po co `src/bin/`?
Rust pozwala, by jeden projekt (czyli jeden `Cargo.toml`) zawierał:
- jedną bibliotekę (z `lib.rs`),
- jeden główny program (z `main.rs`),
- opcjonalnie wiele programów binarnych – w `src/bin/`.

Każdy plik `.rs` w `src/bin/` to osobny program (`main`), który można osobno uruchomić.
#### 📁 Przykład struktury projektu
```less
my_project/
├── Cargo.toml
└── src/
    ├── lib.rs          // (opcjonalnie) biblioteka
    ├── main.rs         // główny program binarny: `cargo run`
    └── bin/
        ├── testuj_ai.rs      // `cargo run --bin testuj_ai`
        ├── eksperyment.rs    // `cargo run --bin eksperyment`
        └── pokaz.rs          // `cargo run --bin pokaz`
```
#### 🔨 Jak to działa?
Jeśli masz np. `src/bin/eksperyment.rs` z:
```rs
fn main() {
    println!("Eksperymentuję!");
}
```
To uruchamiasz go tak:
```sh
cargo run --bin eksperyment
```
Każdy taki plik musi zawierać `fn main()`, bo kompiluje się jako osobny program.
#### 🧠 Kiedy to się przydaje?
- Masz jeden silnik gry (np. w `lib.rs`), ale chcesz różne fronty: np. tekstowy, graficzny, testujący.
- Chcesz testować różne tryby działania, osobne symulacje, narzędzia.
- Dzielisz kod logiczny (`lib.rs`) i aplikacyjny (`bin/`), by móc lepiej testować i zarządzać.

### ▶️Priorytety uruchomiania przy użyciu `cargo run`
#### ✅ Jeśli nie ma `main.rs`, a jest jeden plik w `src/bin/`:
```
cargo run
```
➡️ uruchomi automatycznie ten jeden plik z `src/bin/`.\
Rust wie, że skoro nie ma `src/main.rs`, ale jest tylko jeden plik binarny w `src/bin/`, to pewnie chcesz go uruchomić.
#### ✅ Jeśli jest `main.rs`, to:
```
cargo run
```
➡️ uruchomi `src/main.rs` — to domyślny punkt wejścia binarki.
#### ❌ Jeśli jest więcej niż jeden plik w `src/bin/`, a nie ma `main.rs`:
```
cargo run
```
➡️ zakończy się błędem:
```
error: `cargo run` could not determine which binary to run. Use the `--bin` option to specify a binary, or the `default-run` manifest key.
```
Musisz wtedy jawnie wskazać który plik chcesz uruchomić, np.:
```
cargo run --bin nazwa_pliku
```
gdzie `nazwa_pliku.rs` to np. `src/bin/nazwa_pliku.rs`.

##### Podsumowanie:
Sytuacja|	cargo run robi...
--|--
Tylko `main.rs`|	Uruchamia `main.rs`
Tylko jeden plik w `src/bin/`|	Uruchamia ten plik
`main.rs` i pliki w `src/bin/`|	Uruchamia `main.rs`
Więcej niż jeden plik w `src/bin/`, bez `main.rs`|	Wyrzuca błąd – trzeba wskazać binarkę

## Podział projektu na wiele plików wersja druga

[w10_3 - gra w kółko i krzyżyk (Tic-Tac-Toe), podzielony na kilka plików, z hermetyzacją gracza](./kody_do_wykladu/w10_3)
### 🧱 1. Hermetyzacja pól (`pionek`, `imie`)
✅ W drugiej wersji:
```rs
pub struct GraczCzlowiek {
    pionek: Pionek,
    imie: String,
}
```
- Pola są prywatne (brak `pub`).
- Udostępniane są tylko przez gettery:
    ```rs
    pub fn imie(&self) -> &String
    pub fn pionek(&self) -> Pionek
    ```
❌ W pierwszej wersji:
```rs
pub struct GraczCzlowiek {
    pub pionek: Pionek,
    pub imie: String,
}
```
- Pola są **publiczne** – każdy moduł może je zmienić bez kontroli.
- Brak ochrony przed niepożądaną modyfikacją danych.
### 🧱 2. Sposób tworzenia gracza
✅ Druga wersja:
```rs
GraczCzlowiek::new(imie, pionek)
```
- Tworzenie gracza odbywa się przez dedykowaną metodę `new`.
- Pozwala to na:
    - centralizację logiki tworzenia (np. walidacja danych),
    - możliwość późniejszego dodania dodatkowych parametrów bez łamania kodu klienta.

❌ Pierwsza wersja:
```rs
GraczCzlowiek {
    imie,
    pionek,
}
```
- Struktura tworzona bezpośrednio, wymaga jawnego podania wszystkich pól.
- Nie daje elastyczności — każde pole musi być znane i dostępne z zewnątrz.
### 🧱 3. Lepsza separacja odpowiedzialności
W drugiej wersji:
- `GraczCzlowiek` sam zarządza swoimi danymi (nikt z zewnątrz nie zmienia mu `pionka` czy `imienia`).
- Dzięki prywatnym polom i getterom zachowujesz kontrolę nad dostępem do danych.
- To ułatwia przyszłą rozbudowę np. o historię ruchów, liczenie punktów itp.

### 🧠 Dlaczego druga wersja jest lepsza?
Cecha	|Druga wersja	|Pierwsza wersja
--|--|--
Hermetyzacja|	✅ Tak — pola prywatne	|❌ Nie — pola publiczne
Tworzenie obiektów|	✅ Przez `new()`	|❌ Bezpośrednio
Elastyczność i bezpieczeństwo|	✅ Można dodać logikę, walidację	|❌ Brak kontroli
Możliwość refaktoryzacji|	✅ Duża	|❌ Niska
Zgodność z zasadami OOP (np. SOLID)|	✅ Tak	|❌ Raczej nie

### ✨ Podsumowanie
Druga wersja stosuje dobre praktyki programowania:
- Enkapsulacja danych (ukrycie szczegółów implementacyjnych),
- Kontrolowany dostęp przez metody (`gettery`, `new`),
- Możliwość rozwoju kodu bez łamania istniejącego API.

Takie podejście jest bardziej skalowalne i bezpieczne w większych projektach.

# Wykład 11

zagadnienia na kolokwium
- wszystko z pierwszego kolokwium
- tworzenie własnych typów 
    - struct, enum, 
    - implementacja metod dla tych typów
    - derywacja i implementacja cech bibliotecznych
- posługiwanie się vectorem i użycie iteratorów bibliotecznych
- lambdy

Nie będzie:
- wczytywania z konsoli
- modułów

### [w11_1 - gra w kółko i krzyżyk (Tic-Tac-Toe), dodany gracz komputer](./kody_do_wykladu/w11_1)


### Wskaźniki
W Rust, typy:
- `&Typ`
- `Box<Typ>`
- `Rc<Typ>`
- `Arc<Typ>`

to różne rodzaje wskaźników (czyli "inteligentnych wskaźników" lub "smart pointers") używane do zarządzania danymi na różne sposoby — od prostych referencji po zliczanie referencji w środowisku wielowątkowym.

Poniżej znajdziesz dokładne porównanie i opis każdego z nich, z przykładami i zastosowaniem:

### `&Typ` — Referencja
Opis:
- Najprostszy wskaźnik w Rust.
- Jest **nieposiadający** (ang. non-owning): nie przejmuje własności danych.
- Wskazuje na dane, które żyją gdzie indziej.
- Domyślnie **niemutowalny** (`&T`) — mutowalne tylko przez `&mut T`.
- Czas życia (lifetime) musi być znany.

Użycie:
```rs
fn show(value: &i32) {
    println!("{}", value);
}

let x = 10;
show(&x); // przekazanie referencji
```
Kiedy używać:
- Gdy nie chcesz przenosić własności.
- Do przekazywania danych do funkcji bez ich kopiowania.

### `Box<Typ>` — Wskaźnik na stertę (heap)
Opis:
- Przechowuje dane na stercie (heap), a nie na stosie.
- **Posiadający wskaźnik** (ma pełne prawo własności).
- Nie współdzielony (nie można go kopiować ani klonować bez `Clone`).
- Przydatny, gdy typ ma **nieznany rozmiar w czasie kompilacji** lub jest bardzo duży.

Użycie:
```rs
let b = Box::new(42);
println!("{}", b);
```
Kiedy używać:
- Gdy chcesz przechować duży obiekt na stercie.
- Do pracy z typami rekurencyjnymi:
    ```rs
    enum List {
    Cons(i32, Box<List>),
    Nil,
    }
    ```
### `Rc<Typ>` — Reference Counted Pointer (pojedynczy wątek)
Opis:
- Przechowuje dane na stercie i liczy liczbę referencji (wątkowo niebezpieczne).
- Służy do dzielenia własności w jednym wątku.
- Dane są automatycznie zwalniane, gdy licznik referencji osiągnie 0.
- Tylko do użycia w pojedynczym wątku.

Użycie:
```rs
use std::rc::Rc;

let a = Rc::new(10);
let b = Rc::clone(&a); // zwiększa licznik
println!("a = {}, b = {}", a, b);
```
Kiedy używać:
- Gdy wiele struktur w jednym wątku potrzebuje współdzielić dostęp do danych.
- Klasyczny przykład: drzewo, gdzie wiele dzieci wskazuje na tego samego rodzica.
### `Arc<Typ>` — Atomic Reference Counted Pointer (wielowątkowy)
Opis:
- Jak `Rc<T>`, ale bezpieczny wątkowo.
- Licznik referencji jest atomowy (kosztowna operacja, ale bezpieczna).
- Wymagany przy dzieleniu danych między wątkami.

Użycie:
```rs
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3]);
let data2 = Arc::clone(&data);

let handle = thread::spawn(move || {
    println!("{:?}", data2);
});

handle.join().unwrap();
```
Kiedy używać:
- Gdy chcesz dzielić dane między wątkami (np. wielowątkowy serwer, worker pool).
- Połączenie z `Mutex<T>` często spotykane: `Arc<Mutex<T>>`.

| Typ      | Posiada własność? | Można współdzielić?  | Wątki? | Umiejscowienie danych | Użycie główne                       |
| -------- | ----------------- | -------------------- | ------ | --------------------- | ----------------------------------- |
| `&T`     | ❌                 | ✅ (przez referencję) | ✅      | stos / inne           | Dostęp tymczasowy bez przenoszenia  |
| `Box<T>` | ✅                 | ❌                    | ✅      | sterta                | Przeniesienie na stertę, rekurencja |
| `Rc<T>`  | ✅                 | ✅                    | ❌      | sterta                | Dzielenie danych w jednym wątku     |
| `Arc<T>` | ✅                 | ✅                    | ✅      | sterta                | Dzielenie danych między wątkami     |


### [Kod z mechanizmem pożyczania z czasami życia](/kody_do_wykladu/w11_2.rs)
### 🔍 Co robi ten kod?
#### 1. Struktura `Osoba`
```rs
struct Osoba {
    imie: String,
}
```
- Prosta struktura z jednym polem imie, które jest typu String (czyli dynamicznie alokowany tekst na stercie).
- Posiada pełną własność swojego pola String.
#### 2. Struktura `Samochod<'a>`
```rs
struct Samochod<'a> {
    opis: String,
    wlasciciel: &'a Osoba,
}
```
- Struktura z dwoma polami:
    - `opis: String` — pełna własność opisu samochodu.
    - `wlasciciel: &'a Osoba` — referencja do właściciela (typu `Osoba`), nie przejmuje własności.
- `'a` **to czas życia (lifetime)** — oznacza, że `Samochod` może istnieć tylko tak długo, jak długo żyje jego właściciel (`Osoba`).

>Rust wymaga jawnych adnotacji lifetime'ów, gdy struktura przechowuje referencje — aby zapewnić bezpieczeństwo pamięci i uniknąć wiszących wskaźników (dangling pointers).
#### 3. Funkcja main()
```rs
fn main() { 
    let o1 = Osoba {imie: "Edek".to_string()};
```
- Tworzymy zmienną `o1`, która posiada strukturę `Osoba` z imieniem `"Edek"`.
```rs
    let s1 = Samochod {opis: "zielony opel".to_string(), wlasciciel: &o1};
    let s2 = Samochod {opis: "żółty fiat".to_string(), wlasciciel: &o1};
```
- Tworzymy dwa samochody (`s1` i `s2`) z różnymi opisami, ale tym samym właścicielem — referencja do `o1`.
- Rust sprawdza, czy referencje są ważne tak długo, jak potrzebują (czyli `o1` musi żyć co najmniej tak długo jak `s1` i `s2`).
```rs
    {
        let s3 = Samochod {opis: "czarny ford".to_string(), wlasciciel: &o1};
    }
}
```
- Tworzymy trzeci samochód `s3` w zasięgu blokowym (czyli jego życie kończy się po `}`).
- `s3` ma ten sam właściciel `&o1`, ale żyje krócej (to bezpieczne — `o1` nadal istnieje w tym czasie).

### 🧠 Co robi Rust pod spodem?
Rust kompiluje ten kod bez problemu, ponieważ:
- `o1` żyje przez cały czas funkcji `main`.
- Wszystkie samochody (`s1`, `s2`, `s3`) mają referencje do `o1`, więc ich lifetime `'a` jest zgodny z życiem `o1`.
- Nie ma żadnego konfliktu o mutowalność (wszystkie referencje są niemutowalne).
- Żaden `Samochod` nie próbuje przejąć własności `Osoba`, tylko ją pożycza.

### 🔒 Dlaczego potrzebny jest `'a`?
Rust nie może sam zgadnąć, jak długo `wlasciciel` może żyć względem `Samochod`. Gdybyś nie podał `'a`, to Rust by zgłaszał błąd, że nie może ustalić długości życia referencji.

Adnotacja `'a` mówi:
> „`Samochod` nie może żyć dłużej niż osoba, na którą wskazuje `wlasciciel`”.

### 🔚 Co się stanie na końcu?
Po zakończeniu funkcji `main()`:
- `s3` zostaje zniszczony natychmiast po wyjściu z bloku `{}`.
- `s1`, `s2`, `o1` zostają zniszczeni pod koniec main.
- Rust automatycznie zarządza pamięcią (brak `free()` lub `delete`).
### 📌 Podsumowanie
| Element    | Typ            | Własność | Czas życia         | Uwagi                            |
| ---------- | -------------- | -------- | ------------------ | -------------------------------- |
| `o1`       | `Osoba`        | ✅        | całe `main()`      | Właściciel danych `Osoba`        |
| `s1`, `s2` | `Samochod<'a>` | ✅        | całe `main()`      | Pożyczają `o1` przez referencję  |
| `s3`       | `Samochod<'a>` | ✅        | tylko w bloku `{}` | Bezpieczne, bo `o1` żyje dłużej  |
| `&o1`      | `&Osoba`       | ❌        | zależy od `'a`     | Pożyczka bez przejęcia własności |


# Wykład 12

### [w12_1](/kody_do_wykladu/w12_1.rs)
### 🔧 Struktury
`struct Gracz`
```rs
struct Gracz {
    imie: String,
}
```
- Przechowuje imię gracza.
- Posiada dane (`String` → pełna własność).

`struct Gra`
```rs
struct Gra {
    nazwa: String,
}
```
- Reprezentuje grę (np. "Chess", "Counter-Strike").
- Również ma własność pola nazwa.

`struct Ranking<'a>`
```rs
struct Ranking<'a> {
    gracz: &'a Gracz,
    gra: &'a Gra,
    punkty: i32,
}
```
- Przechowuje wynik (`punkty`) konkretnego gracza w konkretnej grze.
- Nie przejmuje własności `Gracz` ani `Gra` — używa referencji (`&`) z adnotacją lifetime `'a`.
- `'a` mówi: `Ranking` nie może żyć dłużej niż `gracz` i `gra`, na które wskazuje.

🧠 Po co `'a`?
Ponieważ `Ranking` zawiera referencje, Rust musi wiedzieć, jak długo dane (`gracz`, `gra`) będą żyły. Lifetime `'a` gwarantuje, że `Ranking` nie przechowuje odniesień do już zniszczonych obiektów.

### 🧾 Podsumowanie
| Struktura | Posiada dane? | Typ przechowywania     | Uwagi                               |
| --------- | ------------- | ---------------------- | ----------------------------------- |
| `Gracz`   | ✅             | `String`               | Pełna własność                      |
| `Gra`     | ✅             | `String`               | Pełna własność                      |
| `Ranking` | ❌             | `&'a Gracz`, `&'a Gra` | Pożyczone dane z okresem życia `'a` |

### [w12_2 - dodano main](/kody_do_wykladu/w12_2.rs)
### 🔍 Co się dzieje w `main()`
Kod tworzy dane graczy, gier i ich wyników w konkretnej grze. Oto krok po kroku:

1. Tworzenie wektora graczy
```rs
let gracze = vec![
    Gracz { imie: "Edek".to_string() },
    Gracz { imie: "Felek".to_string() },
];
```
- Tworzony jest wektor `gracze`, zawierający dwóch graczy (`Gracz`), każdy z własnym imieniem (`String`).
- Wartości są na stercie i posiadane przez `Vec`.
2. Tworzenie wektora gier
```rs
let gry = vec![
    Gra { nazwa: "Kółko i krzyżyk".to_string() },
    Gra { nazwa: "Szachy".to_string() },
];
```
- Analogicznie tworzony jest wektor `gry`, zawierający dwie gry (`Gra`), każda z nazwą (`String`).
- `Vec` posiada te obiekty — są trzymane na stercie.
3. Tworzenie wektora wyników (`Ranking`)
```rs
let wyniki = vec![
    Ranking {
        gracz: &gracze[0],
        gra: &gry[0],
        punkty: 7,
    },
    Ranking {
        gracz: &gracze[1],
        gra: &gry[0],
        punkty: 17,
    },
];
```
- Tworzymy wektor `Ranking`ów, które przechowują referencje do elementów `gracze` i `gry`.
- `Ranking` nie przejmuje własności — pożycza gracza i grę (`&gracze[0]`, `&gry[0]` itd.).
- `punkty` to zwykła liczba całkowita (`i32`).

>Każdy `Ranking` zawiera:
>- referencję do konkretnego gracza,
>- referencję do konkretnej gry,
>- wynik punktowy tego gracza w tej grze.

### 🧠 Bezpieczeństwo lifetimów
Rust kompiluje to, ponieważ:
- `gracze` i `gry` żyją wystarczająco długo — przez cały `main`.
- `Ranking`i przechowują tylko referencje (`&`) do elementów wektorów, które nie znikają za wcześnie.
- Referencje nie są mutowane — nie ma konfliktu dostępu.

### 📦 W pamięci
| Nazwa    | Typ            | Dane                                   |
| -------- | -------------- | -------------------------------------- |
| `gracze` | `Vec<Gracz>`   | 2 graczy: Edek, Felek                  |
| `gry`    | `Vec<Gra>`     | 2 gry: "Kółko i krzyżyk", "Szachy"     |
| `wyniki` | `Vec<Ranking>` | 2 rekordy: Edek i Felek grają w 1. grę |

### [w12_3 - zawartość main przeniesiono do funkcji](/kody_do_wykladu/w12_3.rs)

W tej wersji kodu najważniejsza zmiana to przeniesienie logiki tworzenia graczy, gier i rankingów do osobnej funkcji `f1()`.

### 📌 Kluczowa różnica
W poprzednim kodzie:
- Wszystko działo się w `main()`, więc dane (`gracze`, `gry`, `Ranking`) żyły do końca `main()` — długość życia zmiennych była długa.

W aktualnym kodzie:
- Wszystko dzieje się w funkcji `f1()`, a ta jest wywoływana z `main()`.
- Dane (`gracze`, `gry`, `wyniki`) są lokalne dla `f1()` — znikają po jej zakończeniu.

🧠 Co to oznacza dla lifetimów?
```rs
let wyniki = vec![
    Ranking {
        gracz: &gracze[0],
        gra: &gry[0],
        punkty: 7,
    },
];
```
- `Ranking` zawiera referencje do lokalnych zmiennych (`gracze`, `gry`).
- Te referencje są ważne tylko w czasie działania `f1()`.
- Nie ma problemu kompilacyjnego, bo `Ranking` też jest lokalny — nie próbujemy go zwrócić poza `f1()`.

> Gdybyś próbował zwrócić wyniki z `f1()` do `main()`, Rust **nie pozwoliłby** na to, ponieważ **zwracałbyś referencje do już zniszczonych danych** (dangling references).

### ✅ Dlaczego to działa?
Bo cały `vec![]` z Rankingami żyje tylko w `f1()`, a wszystkie referencje wskazują na dane też z `f1()`. Ich lifetimes są krótkie, ale zgodne — wszystko kończy życie razem.

### 🔄 Co się zmieniło?
| Aspekt                         | Wcześniej (`main()`) | Teraz (`f1()`)                                 |
| ------------------------------ | -------------------- | ---------------------------------------------- |
| Zakres życia zmiennych         | Przez całe `main()`  | Tylko wewnątrz `f1()`                          |
| `Ranking` żyje tak długo jak:  | `gracze`, `gry`      | `gracze`, `gry` (ale krócej, bo w `f1`)        |
| Można użyć wyników w `main()`? | Tak                  | ❌ Nie, chyba że zmienisz lifetime i własność   |
| Błąd kompilacji?               | ❌ Nie                | ❌ Nie (dopóki nie próbujesz zwracać `Ranking`) |

## 🚨 Uwaga
### 🧨 Przypadek 1 – zwracanie `Vec<Ranking>`
```rs
fn f1() -> Vec<Ranking> {
    let gracze = vec![ /* ... */ ];
    let gry = vec![ /* ... */ ];

    let wyniki = vec![
        Ranking { gracz: &gracze[0], gra: &gry[0], punkty: 7 },
        // ...
    ];

    wyniki
}
```
#### ❌ Dlaczego to nie działa?
- `Ranking` zawiera referencje (`&Gracz`, `&Gra`) do danych z `gracze` i `gry`.
- Ale `gracze` i `gry` są lokalne w `f1()` → znikają, gdy `f1()` się kończy.
- Rust **nie pozwala ci zwrócić tych referencji, bo to byłoby niebezpieczne** — odwoływałbyś się do nieistniejącej pamięci.
- Kompilator nie pozwala utworzyć `Vec<Ranking>` z odwołaniami do krótkowiecznych danych.

#### 💬 Błąd kompilatora (w uproszczeniu):
> `gracz` does not live long enough\
> borrowed value does not live long enough

### 🧨 Przypadek 2 – zwracanie `(Vec<Gracz>, Vec<Gra>, Vec<Ranking>)`
```rs
fn f1() -> (Vec<Gracz>, Vec<Gra>, Vec<Ranking>)
```
#### ❌ Dlaczego to też nie działa?
Na pierwszy rzut oka może się wydawać, że skoro zwracasz również `gracze` i `gry`, to powinno działać. Ale **nie działa**, ponieważ:
- `Ranking` zawiera referencje do `gracze` i `gry`.
- `Ranking` powstaje zanim te wektory zostaną przeniesione (zwrócone).
- Rust nie potrafi zagwarantować, że referencje w `Ranking`ach będą spójne z tymi konkretnymi `gracze` i `gry` po przeniesieniu.
- To problem tzw. **self-referential** struct — struktury, które zawierają referencje do innych pól w tej samej wartości (tu: wyniki odnoszą się do gracze/gry, które wracają razem).

#### 💬 Błąd kompilatora (w uproszczeniu):
>borrowed value does not live long enough\
>`gracze` does not live long enough


### [w12_4 - rozdzielenie tworzenia danych i tworzenia rankingów na dwie funkcje (f1 i f2), a także korzystanie z referencji i lifetimów, by zachować bezpieczeństwo pamięci](/kody_do_wykladu/w12_4.rs)

### 🔹 Funkcja `f1`
```rs
fn f1() -> (Vec<Gracz>, Vec<Gra>)
```
- Tworzy dwóch graczy (`Edek`, `Felek`) i dwie gry (`Kółko i krzyżyk`, `Szachy`).
- Zwraca je jako dwie oddzielne kolekcje: `Vec<Gracz>` i `Vec<Gra>`.

➡️ Dane są **zwracane na własność** — nie używamy tu żadnych referencji.
### 🔹 Funkcja `f2`
```rs
fn f2<'a>(gracze: &'a Vec<Gracz>, gry: &'a Vec<Gra>) -> Vec<Ranking<'a>>
```
- Przyjmuje referencje do wektorów graczy i gier, z lifetime `'a`.
- Tworzy ranking na podstawie tych danych: dwie pozycje (gracz, gra, punkty).
- Zwraca `Vec<Ranking>`, gdzie każdy Ranking zawiera referencje do oryginalnych `Gracz` i `Gra`.

➡️ Referencje w rankingach są bezpieczne, bo:
- `Ranking` nie próbuje żyć dłużej niż `gracze` i `gry`,
- te dane są przekazane z `main()` i żyją wystarczająco długo.
### 🔹 Funkcja `main`
```rs
fn main() {
    let (gracze, gry) = f1();         // f1 zwraca dane na własność
    let wyniki = f2(&gracze, &gry);   // przekazujemy referencje do f2
}
```
- W `main()` otrzymujesz dane z `f1()` i przekazujesz ich referencje do `f2()`.
- `f2()` tworzy ranking i zwraca `Vec<Ranking>`, który jest bezpieczny, bo wszystkie dane nadal żyją w `main()`.
### ✅ Dlaczego to działa?
- Referencje w `Ranking`ach wskazują na dane (`gracze`, `gry`), które nadal istnieją w `main()`, więc nie ma zagrożenia dangling reference.
- Kompilator Rust potrafi sprawdzić, że lifetime `'a` jest wystarczająco długi.
### 🧠 Podsumowanie
| Element        | Opis                                                                |
| -------------- | ------------------------------------------------------------------- |
| `f1()`         | Tworzy dane i je zwraca – własność (`Vec<Gracz>`, `Vec<Gra>`)       |
| `f2()`         | Tworzy rankingi z referencji do danych z `f1()`                     |
| `Ranking<'a>`  | Struktura przechowuje **referencje**, więc potrzebne są lifetimes   |
| Bezpieczeństwo | Wszystkie referencje są ważne tak długo, jak dane źródłowe w `main` |

### [w12_5 - rozbicie 2 funkcji na 3](/kody_do_wykladu/w12_5.rs)

W porównaniu do poprzedniego wariantu kodu, w którym dane (`gracze`, `gry`) były tworzone i zwracane razem przez jedną funkcję `f1()`, w tym kodzie podzielono to na trzy oddzielne funkcje: `f0()`, `f1()` i `f2()`.
### 🔄 Zmiany względem poprzedniej wersji
| Poprzedni kod                                               | Obecny kod                                                                          |
| ----------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| Funkcja `f1()` tworzyła **graczy i gry**, a `f2()` ranking. | Funkcja `f0()` tworzy **graczy**, `f1()` tworzy **gry**, `f2()` tworzy **ranking**. |
| `f1()` zwracała `(Vec<Gracz>, Vec<Gra>)`                    | `f0()` → `Vec<Gracz>`, `f1()` → `Vec<Gra>`                                          |
| W `main()`: `let (gracze, gry) = f1();`                     | W `main()`: `let gracze = f0();`, `let gry = f1();`                                 |

### 🧠 Co to zmienia?
**1. Struktura kodu jest bardziej modularna i czytelna:**
- Funkcje mają jeden odpowiedzialny cel (Single Responsibility Principle).
- Łatwiej testować `f0()` i `f1()` osobno, np. dodać nowych graczy albo gry niezależnie.

**2. Z punktu widzenia kompilatora i lifetimów nic się nie zmienia:**
- Nadal w `f2()` przekazujesz referencje do danych z `f0()` i `f1()`, a `Ranking<'a>` poprawnie używa tych referencji.
- Lifetime `'a` działa, bo dane (`gracze`, `gry`) **są tworzone w** `main()` **i żyją długo**, więc referencje w `Ranking` są bezpieczne.

**3. Zalety praktyczne:**
- Taka forma lepiej się skaluje — można w przyszłości mieć różne źródła danych (np. z pliku, API), po jednym dla graczy i dla gier.

📦 Podsumowanie
- ✅ **Funkcjonalnie**: program robi dokładnie to samo — tworzy dane i generuje ranking.
- ✅ **Bezpiecznie**: wszystkie referencje mają ważny lifetime `'a`, więc kod się kompiluje.
- ✅ **Lepsza organizacja kodu**: podział na `f0`, `f1`, `f2` poprawia modularność

### [w12_6 - mała zmiana w main](/kody_do_wykladu/w12_6.rs)
```rs
fn main() {
    let gracze = f0();                      // żyje do końca main
    {
        let gry = f1();                     // żyje do końca tego bloku
        let wyniki = f2(&gracze, &gry);     // używa &gry
    }                                       // gry i wyniki znikają tutaj
}
```
### ✅ Dlaczego ten kod działa
- Funkcja `f2(&gracze, &gry)` tworzy i zwraca `Vec<Ranking>`, który zawiera referencje do `gracze` i `gry`.
- Jednak `wyniki` jest używane tylko w tym samym bloku, w którym żyją `gry`.
- **Rust potrafi sprawdzić**, że:
    - `&gry` nie wycieka poza ten blok,
    - `Ranking` żyje dokładnie tak długo jak `gry`,
    - więc **czas życia referencji pasuje** – i wszystko jest bezpieczne.

>📌 W skrócie: Rust pozwala na tworzenie struktur z referencjami do lokalnych danych, **jeśli te struktury nie opuszczają zasięgu danych, do których się odnoszą**.

### [w12_7 - kolejna mała zmiana w main](/kody_do_wykladu/w12_7.rs)

### 🔍 Kod (istotny fragment)
```rs
let gry;
{
    gry = f1(); // przypisanie w bloku
}
let wyniki = f2(&gracze, &gry);
```
### ✅ Dlaczego to działa?
**1. Zmienna `gry` jest zadeklarowana przed blokiem**
```rs
let gry;
```
- To oznacza, że zmienna `gry` żyje w całej funkcji `main` — czyli aż do końca `main`.
- To, że wartość `gry` jest przypisana wewnątrz bloku, nie zmienia faktu, że **dane żyją tak długo jak zmienna**.

**2. `f1()` zwraca `Vec<Gra>`, czyli dane przechodzą na własność (`ownership`) do `gry`.**
- Rust nie trzyma się tu żadnych referencji — to **pełne wartości** typu `Vec<Gra>`, a nie dane tymczasowe.
- To znaczy, że po przypisaniu `gry = f1();`, wektor `gry` należy do `main`, a nie do bloku, w którym przypisano wartość.

**3. Referencje w `f2(&gracze, &gry)` są ważne**
- Teraz `gry` i `gracze` są **pełnoprawnymi zmiennymi, które żyją dłużej niż** `wyniki`.
- Można więc przekazać je jako referencje, by zbudować `Vec<Ranking<'a>>`, który przechowuje referencje do `gracze` i `gry`.

### 🧠 Kluczowy mechanizm: Czas życia zmiennej ≠ miejsce przypisania
- To, że wartość przypisujesz w bloku, nie skraca życia zmiennej.
- **Życie zmiennej** (`gry`) **zależy od miejsca deklaracji**, a nie miejsca przypisania.

Czyli ten kod:
```rs
let gry;
{
    gry = f1();     // przypisanie tutaj...
}                   // ...ale 'gry' żyje nadal
```
jest równoważny z tym:
```rs
let gry = f1();     // przypisanie bezpośrednie
```
...pod względem czasu życia danych.

### 📦 Dlatego to działa
Dzięki temu, że `gry` zostało zadeklarowane poza blokiem, **przypisanie wewnątrz bloku nie ogranicza jego życia** – więc wszystkie referencje, które tworzysz do tych danych, są bezpieczne z punktu widzenia borrow checkera.

### 📌 Podsumowanie
✅ Ten kod działa, ponieważ:
- `gry` żyje tak długo jak `main`, mimo że przypisanie jest w bloku,
- `f1()` przenosi własność danych do `gry`,
- `f2` tworzy `Ranking` zawierający referencje do `gracze` i `gry`, które są nadal żywe i ważne.

### [w12_8 - implementacja cech dla struktury](/kody_do_wykladu/w12_8.rs)

Ten kod w języku Rust definiuje generyczną strukturę `V2d<T>`, reprezentującą dwuwymiarowy wektor, oraz implementuje dla niej kilka cech i operacji. Zobaczmy dokładnie co się dzieje, linia po linii — wraz z odpowiedzią na pytanie, czemu użyto referencji przy `dbg!(&w4)`.

### 📦 Struktura `V2d<T>`
```rs
#[derive(Debug, Clone, PartialEq, Default, Eq, Copy)]
struct V2d<T> {
    x: T,
    y: T,
}
```
Atrybuty `#[derive(...)]`:
- `Debug` – umożliwia wypisanie instancji za pomocą `dbg!` lub `println!("{:?}", ...)`.
- `Clone` – pozwala tworzyć kopie struktury (np. `v.clone()`).
- `Copy` – umożliwia kopiowanie struktury **bit po bicie**, bez przenoszenia własności (jak `let b = a;` bez `move`).
- `PartialEq`, `Eq` – porównywanie wektorów (`==`, `!=`).
- `Default` – daje domyślny `V2d::default()`, czyli np. `(0, 0)` jeśli `T: Default`.
> 🔸 Wymaga, by typ `T` również implementował te cechy (np. `i32`, `f64`, `String` tak – `Box<dyn Trait>` nie zawsze).

### 🧮 Implementacja `new`
```rs
impl<T> V2d<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
```
Tworzy nowy `V2d<T>` z podanymi wartościami `x` i `y`.
### ➕ Implementacja operatora `+`
```rs
use std::ops::Add;

impl<T: Add<Output = T> + Copy> Add for V2d<T> {
    type Output = V2d<T>;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}
```
Ta część umożliwia robienie np. `v1 + v2`, jeśli typ `T` spełnia:
- `Add<Output = T>` – potrafi się dodawać (`a + b`)
- `Copy` – można go bezpiecznie kopiować bez przenoszenia

### 🧪 Funkcja `main`
```rs
let w0 = V2d::new(71, -13);                                 // V2d<i32>
let w1 = V2d::new(1, 3);                                    // V2d<i32>
let w2 = V2d::new("kot", "pies");                           // V2d<&str>
let w3 = V2d::new(1.1, 3.0);                                // V2d<f64>
let w4 = V2d::new("kot".to_string(), "pies".to_string());   // V2d<String>
```
Tworzysz różne wektory z różnymi typami `T`.
### 🧾 Wywołania dbg!
```rs
dbg!(w0);   // ok: V2d<i32>: Debug + Copy
dbg!(w1);   // ok
dbg!(w2);   // ok: &str implementuje Debug
dbg!(w3);   // ok: f64 też
dbg!(&w4);  // 🟨 tu trzeba referencji
```
### ❓ Dlaczego `dbg!(&w4)` a nie `dbg!(w4)`?
Ponieważ `w4: V2d<String>` nie implementuje `Copy`, więc wyrażenie `dbg!(w4)` by przeniosło własność (`w4` już by nie istniało po tej linii).

Ale później masz:
```rs
dbg!(w4 == w4);     // używasz w4 ponownie!
```
A to byłoby błędem, gdybyś wcześniej przeniósł `w4`. Dlatego zapis:
```rs
dbg!(&w4);          // przekazujesz tylko referencję, nie ruszając własności
```
jest bezpieczny i kompiluje się.

### ✅ Porównania
```rs
dbg!(w0 == w1); // false
dbg!(w2 == w2); // true
dbg!(w3 == w3); // true
dbg!(w4 == w4); // true
```
Działa, bo `PartialEq`/`Eq` są dostępne dzięki `#[derive(...)]` – pod warunkiem, że typy `T` też to wspierają.

### ➕
```rs
dbg!(w0 + w1);      // V2d<i32> + V2d<i32>
dbg!(w1 + w1);      // ok
dbg!(w3 + w3);      // V2d<f64> + V2d<f64>
// dbg!(w2 + w2);   // ❌ błąd: nie można dodać &str + &str
```
Nie działa dla `w2` (`&str`), bo `&str + &str` nie jest legalne w Rust — tylko `String + &str` działa, ale nie odwrotnie.

### 🧠 Podsumowanie
- `V2d<T>` to generyczny typ 2D wektora.
- Wspiera `+`, `==`, `dbg!`, `clone()` itp.
- Działa dla dowolnych `T`, pod warunkiem że `T` ma odpowiednie cechy (`Add`, `Copy`, `PartialEq`, ...).
- `dbg!(&w4)` jest konieczne, bo `w4` nie implementuje `Copy` i chcemy zachować jego własność.

# Wykład 13

### [w13_1 - HashSet](/kody_do_wykladu/w13_1.rs)

W języku Rust, `HashSet` to kolekcja, która:
- Przechowuje unikalne wartości (bez duplikatów),
- Nie zachowuje kolejności dodanych elementów,
- Jest implementowana jako struktura oparta o tablicę haszującą (ang. hash table),
- Wymaga, by elementy miały cechę `Eq` (porównywalność) i `Hash` (możliwość haszowania).
- Rustowy `HashSet` znajduje się w bibliotece standardowej: `std::collections::HashSet`.
- Główne operacje:
    - `insert` – dodaje element (jeśli go nie ma),
    - `contains` – sprawdza, czy element istnieje,
    - `is_subset` / `is_superset` – operacje teorii zbiorów,
    - `==` – porównanie zbiorów niezależnie od kolejności.

### [w13_2 - BTreeSet](/kody_do_wykladu/w13_2.rs)

`BTreeSet` to:
- zbiór oparty o **drzewo B-drzewiaste (B-tree)**, należący do `std::collections::BTreeSet`.
- zbiór unikalnych, posortowanych elementów,
- użyteczny, gdy chcesz utrzymać kolejność rosnącą (np. do wyświetlania, iteracji w porządku logicznym),
- wolniejszy niż `HashSet` przy wstawianiu i wyszukiwaniu, ale bardziej przewidywalny i bezpieczny przy porównywaniu porządkowym.

>Jeśli zależy Ci na szybkości – użyj `HashSet`.\
>Jeśli zależy Ci na kolejności i porządku – użyj `BTreeSet`.

### [w13_3 - HashMap](/kody_do_wykladu/w13_3.rs)

#### 🔧 Import i inicjalizacja
```rs
use std::collections::HashMap;
```
Importuje strukturę `HashMap` ze standardowej biblioteki.

```rs
let mut s1: HashMap<char, i32> = HashMap::new();
```
Tworzy nową, pustą mapę haszującą (`HashMap`) przechowującą:
- klucze typu `char` (`'a'`, `'b'`, ...),
- wartości typu `i32` (np. `3`, `33`).
#### ➕ Wstawianie danych
```rs
s1.insert('a', 3);
s1.insert('b', 33);
s1.insert('c', 23);
```
Dodaje trzy pary klucz-wartość:
- `'a' → 3`
- `'b' → 33`
- `'c' → 23`

```rs
println!("{:?}", s1);
```
- Wypisuje zawartość mapy (kolejność jest nieokreślona).
#### 🔁 Nadpisywanie wartości
```rs
s1.insert('a', 723);
```
- Nadpisuje istniejący klucz `'a'`, nową wartością `723`.
```rs
println!("{:?}", s1);
```
Wyświetli zaktualizowaną mapę (np. `'a': 723, 'b': 33, 'c': 23`).
#### 🔄 Iteracja przez mapę
```rs
for (k, v) in &s1 {
    println!("{k}: {v}");
}
```
- Iteruje przez wszystkie pary `(klucz, wartość)` i wypisuje je.
#### 🔍 Dostęp przez `entry()`
```rs
println!("{:?}", s1.entry('a'));
println!("{:?}", s1.entry('x'));
```
- `entry('a')` → zwraca `OccupiedEntry` – klucz `'a'` już istnieje.
- `entry('x')` → zwraca `VacantEntry` – klucz `'x'` nie istnieje.
#### 📥 Dodawanie wartości tylko jeśli nie istnieją
```rs
s1.entry('b').or_insert(145);
s1.entry('y').or_insert(745);
```
- `'b'` już istnieje → nie zmienia nic.
- `'y'` nie istnieje → wstawia `'y' → 745`.
#### 🛠️ Modyfikowanie wartości istniejących
```rs
s1.entry('a').and_modify(|a| (*a)*=1000);
```
- `'a'` istnieje → wartość `723 * 1000 = 723000`.
```rs
s1.entry('z').and_modify(|a| (*a)*=-1000);
```
- `'z'` nie istnieje → nic się nie dzieje.
#### ✅ Podsumowanie funkcji użytych:
| Funkcja / metoda     | Opis                                                         |
| -------------------- | ------------------------------------------------------------ |
| `insert(k, v)`       | Wstawia lub nadpisuje wartość dla klucza `k`                 |
| `entry(k)`           | Uzyskuje dostęp do wpisu (`OccupiedEntry` lub `VacantEntry`) |
| `or_insert(v)`       | Wstawia wartość, jeśli klucz nie istnieje                    |
| `and_modify(f)`      | Modyfikuje wartość tylko jeśli klucz istnieje                |
| `for (k, v) in &map` | Iteruje przez wszystkie pary klucz-wartość                   |

### [w13_4 - BTreeMap](/kody_do_wykladu/w13_4.rs)

🧩 Co to jest `BTreeMap`?
- `BTreeMap<K, V>` to uporządkowana mapa klucz-wartość.
- Klucze są przechowywane w rosnącej kolejności, a struktura oparta jest na B-drzewie.
- Zajmuje się automatycznym sortowaniem przy każdym wstawieniu.
- Należy do biblioteki standardowej: `std::collections`.
#### 🔧 Import i inicjalizacja
```rs
use std::collections::BTreeMap;
```
- Importuje strukturę `BTreeMap`.
```rs
let mut s1: BTreeMap<char, i32> = BTreeMap::new();
```
- Tworzy pustą mapę:
    - klucze typu `char` (`'a'`, `'b'`, ...),
    - wartości typu `i32`.

#### ➕ Wstawianie danych
```rs
s1.insert('a', 3);
s1.insert('b', 33);
s1.insert('c', 23);
```
Dodaje trzy pary klucz-wartość:
- `'a' → 3`
- `'b' → 33`
- `'c' → 23`

```rs
println!("{:?}", s1);
```
- Wypisuje zawartość mapy (kolejność jest nieokreślona).
#### 🔁 Nadpisywanie wartości
```rs
s1.insert('a', 723);
```
- Nadpisuje istniejący klucz `'a'`, nową wartością `723`.
```rs
println!("{:?}", s1);
```
Wyświetli zaktualizowaną mapę (np. `'a': 723, 'b': 33, 'c': 23`).
#### 🔄 Iteracja przez mapę
```rs
for (k, v) in &s1 {
    println!("{k}: {v}");
}
```
- Iteruje po parach `(klucz, wartość)` w kolejności rosnącej kluczy.
#### 🔍 Dostęp przez `entry()`
```rs
println!("{:?}", s1.entry('a'));
println!("{:?}", s1.entry('x'));
```
- `entry('a')` → zwraca `OccupiedEntry` – klucz `'a'` już istnieje.
- `entry('x')` → zwraca `VacantEntry` – klucz `'x'` nie istnieje.

#### 📥 Wstawianie tylko jeśli brak klucza
```rs
s1.entry('b').or_insert(145);
s1.entry('y').or_insert(745);
```
- `'b'` już istnieje → nie zmienia nic.
- `'y'` nie istnieje → wstawia `'y' → 745`.
#### 🛠️ Modyfikacja istniejących wpisów
```rs
s1.entry('a').and_modify(|a| (*a)*=1000);
```
- `'a'` istnieje → wartość `723 * 1000 = 723000`.
```rs
s1.entry('z').and_modify(|a| (*a)*=-1000);
```
- `'z'` nie istnieje → nic się nie dzieje.

#### ✅ Podsumowanie funkcji i operacji:
| Funkcja / metoda     | Opis                                              |
| -------------------- | ------------------------------------------------- |
| `insert(k, v)`       | Wstawia lub nadpisuje wartość dla klucza `k`      |
| `entry(k)`           | Uzyskuje wpis (`OccupiedEntry` lub `VacantEntry`) |
| `or_insert(v)`       | Wstawia `v` jeśli klucz `k` nie istnieje          |
| `and_modify(f)`      | Modyfikuje istniejącą wartość przy kluczu `k`     |
| `for (k, v) in &map` | Iteracja po posortowanej mapie                    |

#### 🔁 Porównanie BTreeMap vs HashMap
| Cecha                  | `BTreeMap`                          | `HashMap`                    |
| ---------------------- | ----------------------------------- | ---------------------------- |
| Kolejność kluczy       | ✅ Tak, **rosnąca**                  | ❌ Nie                        |
| Wydajność wstawiania   | `O(log n)`                          | `O(1)` średnio               |
| Wymagania typów        | `Ord` (porządek)                    | `Hash + Eq`                  |
| Typowa sytuacja użycia | Gdy ważna jest **kolejność kluczy** | Gdy ważna jest **wydajność** |



### [w13_5 - HashMap](/kody_do_wykladu/w13_5.rs) 
### [w13_6 - BTreeMap](/kody_do_wykladu/w13_6.rs)
Oba kody pokazują bardzo podobne operacje, ale z różnymi strukturami danych: `HashMap` vs `BTreeMap`. Skoro znasz już ich podstawy, poniżej przedstawiam porównanie funkcjonalne i edukacyjne tych dwóch programów, skupiając się na różnicach i ważnych wnioskach, które z nich płyną.

#### **🔍 1. Porządek przechowywania danych**
- `HashMap`: Dane są wyświetlane w losowej kolejności (brak gwarancji porządku).
- `BTreeMap`: Dane są posortowane po kluczach – czyli `"krowa"`, `"małpa"`, `"rekin"` pojawią się w porządku alfabetycznym.

📚 Wniosek: Jeśli zależy Ci na uporządkowanej prezentacji danych (np. alfabetycznie lub rosnąco numerycznie) — użyj `BTreeMap`. W przeciwnym razie `HashMap` może być szybszy.
#### **🧪 2. Porównanie kluczy i wartości**
```rs
println!("{}", mapa.contains_key("kot"));
println!("{:?}", mapa.get("pies"));
```
- Działa identycznie w obu mapach.
- Zwraca `true`/`false` lub `Some(&wartość)` / `None`.

📚 Wniosek: Operacje `contains_key`, `get`, `remove` działają analogicznie — API jest spójne, więc można łatwo zamienić `HashMap` na `BTreeMap`, jeśli zmienią się potrzeby projektu.
#### **🔁 3. Modyfikacja wartości przez referencję**
```rs
for dana in &mut mapa {
    *dana.1 *= 100;
}
```
- Oba kody pokazują, że **iteracja mutowalna** pozwala na bezpośrednią modyfikację wartości (np. przemnożenie).
- Zwraca się do wartości przez `dana.1` (czyli `(&klucz, &mut wartość)`).

📚 Wniosek: W obu strukturach można zmieniać dane bezpośrednio w miejscu – ważna cecha dla optymalizacji.
#### **🧬 4. Użycie `.entry()` i operacje `and_modify`, `or_insert`**
```rs
let e1 = mapa.entry("krowa");
e1.and_modify(|x| *x *= 100);
```
- Obie mapy używają API `Entry` do:
    - modyfikacji istniejącego wpisu (`and_modify`),
    - dodania wartości, jeśli nie istnieje (`or_insert`).

📚 Wniosek: Mechanizm `entry` działa identycznie dla `HashMap` i `BTreeMap`. To pokazuje siłę i spójność API Rustowych kolekcji.
#### **📊 5. Porównanie wartości liczbowych**
```rs
let inna_mapa = ... // f64 jako wartości
```
- Pokazuje, że mapy mogą przechowywać dowolne typy jako wartości (np. `i32`, `f64`), jeśli typy spełniają odpowiednie ograniczenia (`Ord`, `Hash`, `Eq`).

📚 Wniosek: Elastyczność struktur — możesz mieć różne typy danych w mapach, ale wybór `HashMap` lub `BTreeMap` zależy od cech klucza:
- `HashMap` wymaga `Hash + Eq`,
- `BTreeMap` wymaga `Ord`.
#### **🚀 6. Efektywność i wybór odpowiedniej mapy**
| Cecha                         | `HashMap`                      | `BTreeMap`                       |
| ----------------------------- | ------------------------------ | -------------------------------- |
| Kolejność kluczy              | ❌ brak                         | ✅ posortowana                    |
| Wydajność wstawiania/szukania | ✅ szybsza (`O(1)` średnio)     | 🔁 wolniejsza (`O(log n)`)       |
| Stabilność iteracji           | ❌ niegwarantowana              | ✅ deterministyczna i posortowana |
| Obsługa dużych danych         | ✅ lepsza dla szybkiego dostępu | ✅ lepsza dla danych do raportów  |
📚 Wniosek: Kod ilustruje, że oba typy map oferują ten sam zestaw metod, ale ich wydajność i zachowanie przy iteracji różnią się — wybór zależy od potrzeb.
#### **✨ Co te kody uczą łącznie?**
- API Rustowych kolekcji jest **spójne** — możesz łatwo przenieść logikę z `HashMap` do `BTreeMap`.
- Pokazują różne sposoby modyfikacji danych:
    - przez iterację z `&mut`,
    - przez` entry().and_modify()`,
    - przez `entry().or_insert()`.
- Podkreślają różnice między **kolejnością logiczną** (sortowanie) a **kolejnością wydajnościową** (hashowanie).
- Uczą też pracy z **danymi dynamicznymi** — wstawianie, nadpisywanie, usuwanie i iteracja są naturalnie zintegrowane z językiem.

### [w13_7 - Uzdrowiciel i różne metody na rozwiązanie problemu leczenia samego siebie](./kody_do_wykladu/w13_7/src/)
### [w13_7 - główna struktura i testy do niej](./kody_do_wykladu/w13_7/src/uzdrowiciel.rs)

### 🧪 Jak działają testy w Rust – na podstawie powyższego kodu
#### 1. `#[cfg(test)]` – czyli testy tylko w czasie testowania
```rs
#[cfg(test)]
mod tests {
    ...
}
```
- `#[cfg(test)]` oznacza, że ten moduł będzie kompilowany tylko podczas uruchamiania testów (`cargo test`).
- Nie wpływa na normalne działanie programu (np. przy `cargo run` lub `cargo build`).
- Dzięki temu możesz dołączać kod pomocniczy do testów bez zaśmiecania finalnej aplikacji/biblioteki.
#### 2. `#[test]` – oznaczenie funkcji testowej
Każda funkcja oznaczona `#[test]` to osobny test jednostkowy:
```rs
#[test]
fn test1() {
    ...
}
```
- Kompilator traktuje to jako pojedynczy przypadek testowy.
- Testy są automatycznie uruchamiane przez `cargo test`.
- Funkcja nie przyjmuje argumentów i nic nie zwraca – ważne są asercje w środku.
#### 3. `assert_eq!`, `assert!` – sprawdzanie warunków
To są **makra testujące**, które przerywają test w razie niespełnienia warunku:
```rs
assert_eq!(a, b);   // sprawdza a == b
assert!(warunek);   // sprawdza czy warunek == true
```
- Jeśli asercja się nie powiedzie, test zostaje uznany za niezaliczony (fail).
- Przy niepowodzeniu wyświetlany jest pełny komunikat o błędzie, pokazujący wartości oczekiwane i rzeczywiste.
- To pozwala szybko zdiagnozować problem.
#### 4. `dbg!()` – pomocnicze debugowanie
```rs
dbg!(zmienna);
```
- Użycie `dbg!` drukuje do konsoli debugową reprezentację wartości.
- Działa jak `println!("{:?}", ...)`, ale automatycznie dodaje też plik i linię, z której pochodzi.
- Nie wpływa na wynik testu, ale jest pomocne przy analizie działania kodu.
- Nie powinno się zostawiać `dbg!` w kodzie produkcyjnym, ale w testach jest jak najbardziej dopuszczalne.
#### 5. Wiele testów uruchamianych niezależnie
- Każdy `#[test]` działa **niezależnie – testy nie widzą siebie nawzajem**, co oznacza, że nie współdzielą żadnego stanu.
- Przykład: `test1`, `test2`, `test5` – wszystkie tworzą własne obiekty `Uzdrowiciel` od zera, nie opierając się na wcześniejszych testach.
- To fundamentalna zasada dobrych testów: **niezależność** i **powtarzalność**.
#### 6. `cargo test` – jak to uruchamiasz?
```bash
cargo test
```
- Uruchamia wszystkie funkcje oznaczone jako `#[test]` w Twoim projekcie.
- Domyślnie uruchamia je **równolegle**, co pozwala na szybkie sprawdzanie kodu.
- Można testować tylko konkretny test:
    ```bash
    cargo test test2a
    ```

#### 7. Testy jako dokumentacja funkcjonalności
- W Rust testy często pełnią też rolę przykładów użycia struktur, takich jak Twoje:
- Pokazują jak tworzyć, modyfikować i używać struktur w praktyce.
- Dobrze napisane testy można czytać jak dokumentację.
- Np. test5 pokazuje, jak pracować z kolekcją struktur (Vec<Uzdrowiciel>).

#### ✨ Podsumowanie: Co uczą testy do tego kodu?
Te testy pokazują, jak:
- ✅ Korzystać z `#[cfg(test)]` i `#[test]`
- ✅ Pisać **modularne**, **izolowane** testy jednostkowe
- ✅ Weryfikować poprawność kodu przy użyciu `assert_eq!` i `assert!`
- ✅ Tymczasowo wspomagać się `dbg!()` przy debugowaniu
- ✅ Sprawdzać metody modyfikujące dane (`mut`), zwracające `Option`, `bool` itp.
- ✅ Testować działanie struktury również w kontekście kolekcji (np. `Vec`)

### [w13_7 - najprostrze leczenie](./kody_do_wykladu/w13_7/src/u01_najprostsze.rs)
### 🧠 Co robi metoda ulecz
```rs
impl Uzdrowiciel {
    pub fn ulecz(&mut self, cel: &mut Uzdrowiciel, przywracane_zdrowie: u32, koszt: u32) {
        if self.wydaj_mane(koszt).is_some() {
            cel.zmien_zdrowie(przywracane_zdrowie as i32);
        }
    }
}
```
Metoda `ulecz`:
- Sprawdza, czy leczniczy uzdrowiciel (`self`) ma wystarczająco dużo many (`wydaj_mane(koszt)`).
- Jeśli ma, zużywa ją i leczy wskazany cel (`cel`), dodając `przywracane_zdrowie` do jego zdrowia aktualnego.
- Jeśli nie ma wystarczająco many, nie robi nic.
### 🧪 Omówienie testów
#### `test3` – leczenie innego uzdrowiciela
```rs
#[test]
fn test3() {
    let mut edek = Uzdrowiciel::new("Edek", 10, 5);
    let mut felek = Uzdrowiciel::new("Felek", 20, 15);
    edek.zmien_zdrowie(-3);                // Edek ma 7 HP
    felek.ulecz(&mut edek, 10, 7);         // Felek leczy Edka za 10 HP, koszt 7 many
    dbg!(edek);                            // Edek powinien mieć 10 HP (maks)
    dbg!(felek);                           // Felek powinien mieć 8 many (15 - 7)
}
```
➡️ Działa poprawnie: jeden leczy drugiego.
#### `test4` (zakomentowany) – leczenie samego siebie
```rs
// let mut edek = Uzdrowiciel::new("Edek", 10, 5);
// edek.zmien_zdrowie(-3);                 // Edek ma 7 HP
// edek.ulecz(&mut edek, 10, 7);           // Edek leczy sam siebie, koszt 7 many
```
⚠️ Ten kod się nie kompiluje. Dlaczego?

**❌ Rust nie pozwala pożyczyć `&mut self` i jednocześnie `&mut self` jako argument**

Metoda `ulecz` przyjmuje:
```rs
&mut self, cel: &mut Uzdrowiciel
```
Ale próba `edek.ulecz(&mut edek, ...)` powoduje konflikt:
- `self` jest już pożyczony mutowalnie
- próbujesz pożyczyć go ponownie mutowalnie jako `cel`

Rust broni się przed **podwójnym mutowaniem tego samego obiektu w tej samej funkcji**. Nie da się tak zrobić bez obejścia.
#### `test7` (zakomentowany) – leczenie wektora, klasyczna wersja
```rs
let mut herosi = vec![...];
herosi[0].zmien_zdrowie(-3);
herosi[1].ulecz(&mut herosi[0], 10, 7);
```
**⚠️ To również się nie skompiluje**. Rust nie pozwala pożyczyć `&mut` dwóch elementów z tego samego `Vec` jednocześnie w prosty sposób.
#### ✅ `test7a` – poprawna wersja leczenia między członkami `Vec`, z użyciem `split_at_mut`
```rs
let (h0, h1) = herosi.split_at_mut(1);
h0[0].zmien_zdrowie(-3);             // Edek osłabiony
h1[0].ulecz(&mut h0[0], 10, 7);      // Felek leczy Edka
```
🟢 To działa poprawnie. Dlaczego?
- `split_at_mut` dzieli wektor na dwie oddzielne mutowalne części.
- `h0` zawiera Edka, `h1` zawiera Felka i Jolę.
- Dzięki temu mamy **dwa niezależne mutowalne odniesienia** – zgodnie z zasadami borrow checkera.
#### ✅ Czy uzdrowiciel może uleczyć samego siebie w tej wersji kodu?
***NIE***, bo Rust nie pozwala na podwójne `&mut self` w tej formie.

### [w13_7 - metoda: ulecz sie](./kody_do_wykladu/w13_7/src/u02_ulecz_sie.rs)
Co zmieniło się w tym kodzie?
- Metoda `ulecz` nadal służy do leczenia innego `Uzdrowiciela`.
- Metoda `ulecz_sie` umożliwia, by `Uzdrowiciel` uleczył siebie samego – to jest teraz poprawne, bez konieczności pożyczania obiektu dwukrotnie mutowalnie.
- Test `test4a` pokazuje przykład użycia `ulecz_sie`.
- Pozostałe `testy` pozostają bez zmian.
#### Dlaczego tak?
Rust wymaga, by w danym momencie było tylko jedno mutowalne odniesienie do danego obiektu. Gdybyś próbował wywołać:
```rs
edek.ulecz(&mut edek, 10, 7);
```
to próbujesz:
- Mutowalnie pożyczyć `edek` jako `self`,
- I jednocześnie mutowalnie pożyczyć `edek` jako `cel`.

To powoduje konflikt, którego Rust nie pozwala.
#### Jak to działa teraz?
- `ulecz_sie(&mut self, ...)` bierze tylko **jedno mutowalne odniesienie do siebie** – nie ma konfliktu.
- `ulecz(&mut self, cel: &mut Uzdrowiciel, ...)` pozwala mutowalnie pożyczyć inny obiekt.

#### Przykład z testu `test4a`
```rs
let mut edek = Uzdrowiciel::new("Edek", 10, 8);
edek.zmien_zdrowie(-3);     // edek ma 7 HP
edek.ulecz_sie(10, 7);      // edek leczy siebie za 10 HP i koszt 7 many
dbg!(edek);                 // edek powinien mieć 10 HP i mniej many
```
#### Podsumowanie
- Metoda `ulecz_sie` to idiomatyczne i bezpieczne rozwiązanie problemu samoleczenia w Rust.
- Dzięki niej masz jasny i czytelny interfejs API.
- Rust wymaga takich rozwiązań, aby zapobiegać błędom związanym z mutowalnym aliasowaniem.

### [w13_7 - surowe wskaźniki](./kody_do_wykladu/w13_7/src/u03_z_surowymi.rs)
#### 🔍 Co robi ten kod?
Metoda: 
```rs
pub fn ulecz(&mut self, cel: *mut Uzdrowiciel, przywracane_zdrowie: u32, koszt: u32)
```
- Zamiast bezpiecznego `&mut Uzdrowiciel`, przekazujesz `*mut Uzdrowiciel` – surowy wskaźnik mutowalny.
- `cel.as_mut().unwrap()` wewnątrz `unsafe` zamienia go z powrotem na `&mut`.

Dzięki temu możesz przekazać ten sam obiekt jako `self` i jako `cel` — czego Rust zabrania w typowym `&mut` API.

#### 📌 Dlaczego to działa?
Rust rozdziela:
- **Bezpieczeństwo typów i pożyczania (borrow checking)** – na poziomie kompilatora dla bezpiecznego kodu.
- **Bezpieczeństwo wykonania** – za które musisz zadbać sam, jeśli używasz unsafe.

W `unsafe` możesz:
- Dereferencjonować wskaźniki (`*mut T`, `*const T`),
- Omijać borrow checker.

Czyli to działa, bo **kompilator Ci ufa, że wiesz co robisz**.
#### ✅ Test test4 – samoleczenie z wskaźnikiem
```rs
let edek_raw = &mut edek as *mut Uzdrowiciel;
edek.ulecz(edek_raw, 10, 7);
```
- Tworzysz surowy wskaźnik do `edek` (`*mut Uzdrowiciel`).
- Wywołujesz `ulecz`, przekazując wskaźnik do siebie samego.

To działa, ponieważ `self` to `&mut edek`, a `cel` to surowy wskaźnik – kompilator nie narzeka, bo nie analizuje tego konfliktu w `unsafe`.
#### ⚠️ Potencjalne zagrożenie
Chociaż kod działa, **łamiesz zasadę "jednego mutowalnego odniesienia na raz"** – czyli **aliasing** + **mutacja**, co może prowadzić do **niezdefiniowanego zachowania** (UB – undefined behavior) w bardziej złożonym kodzie.

>To działa, ale jest potencjalnie niebezpieczne i niezalecane w bezpiecznych aplikacjach.

#### 🛡️ Bezpieczna alternatywa
Metoda `ulecz_sie` z poprzedniej wersji jest idiomatycznym i bezpiecznym rozwiązaniem:
```rs
pub fn ulecz_sie(&mut self, przywracane_zdrowie: u32, koszt: u32)
```
Nie używa `unsafe`, nie wymaga wskaźników — i kompilator Cię chroni.
#### Podsumowanie
| Cecha                   | `ulecz` z \&mut      | `ulecz_sie` | `ulecz` z \*mut    |
| ----------------------- | -------------------- | ----------- | ------------------ |
| Bezpieczny kod Rust     | ✅                    | ✅           | ❌ (unsafe)         |
| Pozwala na samoleczenie | ❌                    | ✅           | ✅                  |
| Chroni przed UB         | ✅                    | ✅           | ❌                  |
| Wygoda i idiomatyczność | ✅                    | ✅           | ❌                  |
| Użycie w testach        | Ok, z ograniczeniami | Ok          | Ok, ale z ryzykiem |

### [w13_7 - surowe wskaźniki, ale z dopiskiem unsafe w nagłówku funkcji](./kody_do_wykladu/w13_7/src/u04_z_surowymi_i_dopiskiem.rs)
#### ✅ Co się zmieniło?
**Metoda `ulecz` jest teraz oznaczona jako `unsafe`**
```rs
pub unsafe fn ulecz(&mut self, cel: *mut Uzdrowiciel, przywracane_zdrowie: u32, koszt: u32)
```
Oznacza to:
- Każde jej wywołanie musi być wewnątrz bloku `unsafe { ... }`.
- Kompilator nie gwarantuje, że użycie będzie bezpieczne — **Ty (programista) musisz to zapewnić**.

#### Bezpieczniej i bardziej idiomatycznie
Wymuszenie `unsafe` chroni przed przypadkowym błędnym użyciem funkcji — jeśli ktoś ją wywoła, **Rust zmusi go do uważności**:
```rs
unsafe {
    uzdrowiciel.ulecz(ptr, 10, 7);
}
```
To czytelny sygnał: **"uważaj, bo możesz popsuć pamięć"**.

### [w13_7 - RefCell](./kody_do_wykladu/w13_7/src/u05_z_ref_cell.rs)
Ta najnowsza wersja z użyciem `RefCell<Uzdrowiciel>` to bardzo czysta i bezpieczna alternatywa wobec `*mut` i `unsafe`. Oto pełna analiza:
#### ✅ Co sie zmieniło?
Zastąpiłeś użycie surowych wskaźników typem `RefCell<Uzdrowiciel>`, aby skorzystać z **wewnętrznej mutowalności**. Dzięki temu:
- Nie potrzebujesz `unsafe`
- Nie łamiesz zasad aliasowania, bo `RefCell` sam sprawdza w czasie wykonywania, czy nie ma dwóch mutujących pożyczek
- Można używać jednej struktury zarówno do samoleczenia, jak i leczenia innych

#### 🔧 Działanie RefCell
`RefCell` to kontener, który umożliwia mutację nawet z niezmiennych referencji, ale tylko w czasie wykonania (run-time). W odróżnieniu od systemu pożyczania kompilatora:
- `RefCell::borrow()` daje `Ref<T>` — dostęp tylko do odczytu
- `RefCell::borrow_mut()` daje `RefMut<T>` — dostęp do zapisu
- Próba jednoczesnego `borrow_mut()` i `borrow()`/`borrow_mut()` = panic!

### 🧪 Analiza testów
#### `test3` – Leczenie innego
```rs
felek.ulecz(&edek, 10, 7);
```
Działa — bezpieczna mutacja `edek` przez `RefCell`.
#### `test4` – Samoleczenie przez `RefCell`
```rs
edek.borrow_mut().ulecz(&edek, 10, 7);
```
Działa! Rust nie narzeka, bo:
- ulecz przyjmuje `&RefCell<Uzdrowiciel>`
- `self` to `RefMut`, a `cel` to `&RefCell`

**Może dojść do paniki**, jeśli wewnątrz ulecz spróbujesz jeszcze raz pożyczyć self, ale Twój kod robi to tylko raz, więc OK.
#### `test7` – We wspólnym wektorze
```rs
herosi[1].borrow_mut().ulecz(&herosi[0], 10, 7);
```
Działa – brak kolizji pożyczania, bo `ulecz` pożycza tylko `herosi[0]` (a `herosi[1]` jest `self`).
### ⚠️ Możliwe problemy
#### 1. Panika w czasie wykonywania
Jeśli przypadkowo spróbujesz:
```rs
let a = rc.borrow_mut();
let b = rc.borrow_mut(); // PANIKA!
```
To program się wywali — ale tylko przy błędzie logicznym.
#### 2. Uwaga na `RefCell` w środowiskach wielowątkowych
`RefCell` działa tylko w pojedynczym wątku. Jeśli planujesz wątki, musisz użyć `Mutex`.
#### ✅ Rekomendacja
To podejście jest obecnie:
- **Najbardziej idiomatyczne**
- **Całkowicie bezpieczne**
- **Uniwersalne** (samoleczenie i leczenie innych)
### ✅ Podsumowanie
| Podejście         | Bezpieczeństwo   | Ergonomia      | Wielowątkowość        | Uwagi                      |
| ----------------- | ---------------- | -------------- | --------------------- | -------------------------- |
| `&mut`            | ✅ Tak            | ✅ Tak          | ✅ Tak                 | Nie pozwala na aliasowanie |
| `*mut` + `unsafe` | ❌ Twoja rola     | ❌ Nieco trudne | ✅ Tak                 | Potencjalne UB             |
| `RefCell`         | ✅ Tak (run-time) | ✅ Bardzo dobre | ❌ Tylko single-thread | Panika przy złym użyciu    |
