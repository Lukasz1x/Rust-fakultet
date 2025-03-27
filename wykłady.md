# Spis treści:
- [Wykład 1](#Wykład-1)
- [Wykład 2](#Wykład-2)
- [Wykład 3](#Wykład-3)
- [Wykład 4](#Wykład-4)
- [Wykład 5](#wykład-5)

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

Problem czytelników i pisarzy.
.|read only|mut(able)|
-- |--|--
private | +| +|
shared | +| ☠️

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
    show(&x, &y);
    show(&x, &x);
    // swap(&mut x, &mut x);

    let mut t = [1, 30, 45];
    show(&t[0], &t[2]);
    // swap(&mut t[0], &mut t[2]);  // tablica to jedna struktura i mimo że to 2 różne elementy to jedna struktura

    add_to(&mut x, &y);
    show(&x, &y);
    // add_to(&mut x, &x); //jeśli jest mutowanie to niże moża więcej pożyczyć tej samej zmiennej
}
```

```rs
fn f1() -> i32 {}

fn f2() -> Option<i32> {}                           // warianty: Some(wynik), None

fn f3() -> Result<i32, BladWejsciaWyjscia> {}       // warianty: Ok(wynik), Err(opis_błędu)

fn f4() -> Result<i32, BladParsowanie> {}           // warianty: Ok(wynik), Err(opis_błędu)

fn main()
{
    let x = Some('x');
    let mut y = None;
    let a :Result<bool, &str> = Err("kot");
    let mut b = Ok(3.5);

    println!("{}", x.unwrap());
    // println!("{}", y.unwrap());
    // println!("{}", a.unwrap());
    println!("{}", b.unwrap());

    println!("{}", x.unwrap_or('a'));
    println!("{}", y.unwrap_or(342));
    println!("{}", a.unwrap_or(true));
    println!("{}", b.unwrap_or(-1.3));

    println!("{}", y.unwrap_or_default());
    println!("{}", a.unwrap_or_default());

    println!("{}", x.is_none());
    println!("{}", y.is_some());
    println!("{}", a.is_ok());
    println!("{}", b.is_err());

    println!("{:?}", a.ok());
    println!("{:?}", a.err());
    println!("{:?}", b.ok());
    println!("{:?}", b.err());

    // println!("{}", y.expect("Spodziewałem się liczby"));

    y = Some(123);
    b = Err(true);

}
```

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

    // Dlaczego nie ma tu implementacji xor ?

}
```

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


