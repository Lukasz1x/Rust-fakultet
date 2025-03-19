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
println!("{}", a)           //output: x
println!("{:?}", a)         //output: 'x'
```

```rs
let a = 'ń';                //typ znakowy (char) ma 4 bajty
println!("{}", a)           //output: ń
println!("{:?}", a)         //output: 'ń'
println!("{}", a as u32)    //outpyt: 324
```
Sposoby zapisywania liczb:
```rs
let a  = 1_000_003;         //1000003
let b = 0xfa;               //250 
let c = 0o721;              //465
let d =0b0011_1010;         //58
let e =b'a';                //97 - pod e zostanie przypisana wartość litery 'a' z tabeli ASCII
```
Pętla nieskończona:
```rs
loop{
    println!("x");
}
```

Pętla `loop` musi się wykonać raz lub do pierwszej instrukcji `break`, dlatego dozwolone jest zwracanie wartości przez `break`,natomiast pętla `while` może wcale się nie wywołać dlatego w niej nie jest dopuszczony `break` ze zwracaniem.

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
        break 999;          //break może zwracać wartość tylko w przypadku pętli loop
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
    x+1                     //można pisać return x+1;
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
- na własnośc (przez wartość)
    - przez kopiowanie
    - przez przeniesienie
- przez pożyczkę/referencję `&`
- przez pożyczkę/referencję mutowalną `&mut`

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
    powitaj_v1("Edek"); //nie zadziała, bo Edek jest (&?)str, a String to struct
    powitaj_v2("Edek"); //nie zadziała, bo Edek jest &str, a funkcja przyjmuje str, jest tu jakiś problem, że str ma rozmiar nieznany podczas kompilacji??
    powitaj_v3("Edek"); //zadziała
}
```
String nie jest typem kopiowalnym, jest przekazywany na własność.
```rs
fn powitaj_v1(imie: &str) //uzywanie &str jest bardziej użytecznie w nagłówku funkcji niż &String 
{ 
    println!("Witaj, {imie}!")
}
fn powitaj_v1(imie: &String) {
    println!("Witaj, {imie}!")
}

fn powitaj_v2(imie: String) {
    println!("Witaj, {imie}!")
}



fn main() {
    powitaj_v1("Edek");
    let imie1 ="Felek".to_string();
    let imie2 =String::from("Balbina");
    powitaj_v2(&imie1);
    powitaj_v3(imie2.clone()); //String jest potencjalnie bardzo dużą wartością, więc nie ma kopiowania stringów, żeby programista zrobił to sam pisząc .clone()
    powitaj_v1(&imie1); //dozwolona jest konwersja z &String na &str
    powitaj_v1(&imie2); //jest nie jawna konwersja typów
}
```

```rs
fn powitaj_v0(tab: [i32; 4]) { //bez & musi być rozmiar
    println!("Witaj, {tab:?}!") 
}
fn powitaj_v1(tab: &[i32]) { 
    println!("Witaj, {tab:?}!")
}
fn powitaj_v1(tab: &Vec<i32>) {
    println!("Witaj, {tab:?}!")
}

fn powitaj_v2(tab: Vec<i32>) {
    println!("Witaj, {tab:?}!")
}



fn main() {
    let tab0 = [1,4,90,34];
    powitaj_v0(tab0);
    powitaj_v1(&[15,3,20]);
    let tab1 = vec![3,5,7,10,3,4,5,6];
    let tab2 = Vec::from([4,10,3,9,87]);
    powitaj_v2(&tab1);
    powitaj_v3(tab2.clone());
    powitaj_v1(&tab1); 
    powitaj_v1(&tab2); 
}
```
```rs
fn wyswietl_jeden(t: &[i32], i: usize){
    println!("{}", t[i]);
}

fn main() {
    let tab0 = [1,4,90,34];
    println!("{}", tab0[2]); 
    wyswietl_jeden(&tab0, 12); //program spanikuje
}
```
```rs
let tab = [0;6]; //stworzenie tablcy o 6 elementach równych 0
```

