Orginalny plik zawiera kolory, których nie widać na podglądzie na Githubie, więc warto go pobrać i otworzyć w czymś lepszym.
# Spis treści:
- [Wykład 1](#Wykład-1)
- [Wykład 2](#Wykład-2)
- [Wykład 3](#Wykład-3)
- [Wykład 4](#Wykład-4)
- [Wykład 5](#wykład-5)
- [Wykład 6](#wykład-6)

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