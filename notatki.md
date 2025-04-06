# Spis treści
0. [Komendy cargo](#komendy-cargo)
1. [Podstawowe elementy składni w Rust](#podstawowe-elementy-składni-w-rust)
2. [Typy danych i zmienne w Rust](#typy-danych-i-zmienne-w-rust)
3. [Operatory w Rust](#operatory-w-rust)
4. [Struktury sterujące w Rust](#struktury-sterujące-w-rust)
5. [Funkcje w Rust](#funkcje-w-rust)


# Komendy cargo
### ***Tworzenie nowego projektu***
`cargo new [nazwa]`\
Tworzy nowy projekt w katalogu o podanej nazwie. Tworzony katalog zawiera plik manifestu `Cargo.toml`, katalog `src/` z plikiem `main.rs` (lub `lib.rs` dla biblioteki) oraz domyślnie inicjalizuje repozytorium Git.

`cargo init`\
Inicjalizuje nowy projekt w aktualnym katalogu, dodając plik `Cargo.toml` oraz strukturę plików zgodną z Rustem.

`--vcs none`\
Wyłącza domyślną inicjalizację repozytorium Git. Użyteczne, gdy nie chcemy, aby Cargo automatycznie tworzyło repozytorium wersjonowania.
### ***Budowanie i uruchamianie projektu***
`cargo build`\
Kompiluje projekt w trybie debug i zapisuje wynikowy plik binarny w katalogu `target/debug/.`

`cargo run`\
Kompiluje i uruchamia program. Jeśli kod źródłowy został zmieniony od ostatniej kompilacji, Cargo automatycznie ponownie skompiluje projekt przed uruchomieniem.

`./target/debug/[nazwa_programu]`\
Uruchamia skompilowany program ręcznie, bez ponownej kompilacji. Użyteczne, gdy chcemy szybko uruchomić program bez angażowania Cargo.

### ***Analiza i formatowanie kodu***
`cargo clippy`\
Uruchamia narzędzie Clippy, które analizuje kod i sugeruje poprawki zgodnie z najlepszymi praktykami Rusta. Pomaga unikać błędów, optymalizować kod i poprawiać jego czytelność.

`cargo fmt`\
Automatycznie formatuje kod zgodnie z oficjalnym stylem Rusta przy użyciu narzędzia rustfmt. Zapewnia spójność formatowania w całym projekcie.

# Podstawowe elementy składni w Rust
### Struktura programu
Każdy program w języku Rust zaczyna się od funkcji `main()`, która jest punktem wejścia do aplikacji.
```rs
fn main() {
    println!("Witaj w Rust!");
}
```
- `fn` oznacza definicję funkcji.
- `main` to nazwa funkcji głównej.
- `{}` oznaczają ciało funkcji.
- `println!()` to makro służące do wyświetlania tekstu na ekranie.
- W Rust makra rozpoznaje się po `!` na końcu ich nazwy.

### Zasady dotyczące wielkości liter

Rust jest językiem case-sensitive, co oznacza, że `zmienna` i `Zmienna` to dwie różne nazwy.

### Komentarze
W Rust można używać dwóch rodzajów komentarzy.
- Jednoliniowe (poprzedzone `//`)
```rs
// To jest komentarz jednoliniowy
println!("Witaj w Rust!"); // Komentarz na końcu linii
```
- Wieloliniowe (poprzedzone /* i zakończone */)
```rs
/*
   To jest komentarz
   wieloliniowy w Rust.
*/
```
Dodatkowo istnieją komentarze dokumentacyjne, używane do generowania dokumentacji
- `///` dla dokumentacji funkcji, struktur itp
- `//!` dla dokumentacji całego modułu
```rs
/// Funkcja wyświetlająca powitanie
fn witaj() {
    println!("Cześć!");
}
```
# Typy danych i zmienne w Rust
Rust jest językiem **statycznie typowanym**, co oznacza, że każdy typ zmiennej musi być znany w czasie kompilacji. Można określać typy jawnie lub pozwolić kompilatorowi na ich inferencję.

### 1 Deklaracja i inicjalizacja zmiennych
W Rust zmienne deklaruje się przy użyciu słowa kluczowego `let`.
```rs
let x = 5;              // Rust domyślnie przypisze typ i32
let y: f64 = 3.14;      // Jawne określenie typu
```
✅ Rust domyślnie przypisuje zmienne jako niemutowalne.\
🚨 Jeśli chcesz zmienić wartość zmiennej, dodaj mut.
```rs
let mut liczba = 10;
liczba = 20;            // OK, bo zmienna jest mutowalna
```
### 2 Typy danych podstawowych
Rust oferuje kilka podstawowych typów danych.

#### Liczby całkowite

Typ	|Rozmiar	|Zakres wartości
--|--|---
`i8`	|8-bit	|-128 do 127
`i16`	|16-bit	|-32,768 do 32,767
`i32`	|32-bit	|-2,147,483,648 do 2,147,483,647
`i64`	|64-bit	|-9,223,372,036,854,775,808 do 9,223,372,036,854,775,807
`i128`	|128-bit	|Jeszcze większy zakres 😃
`isize`	|Architektura zależna	|32-bit na 32-bitowym CPU, 64-bit na 64-bitowym

Liczby bez znaku (`u8`, `u16`, `u32`, `u64`, `u128`, `usize`) mają zakres od `0` do `maksymalnej wartości`.
```rs
let a: i32 = -10;       // liczba całkowita ze znakiem
let b: u32 = 100;       // liczba całkowita bez znaku
```
#### Liczby zmiennoprzecinkowe
Rust obsługuje tylko `f32` (pojedyncza precyzja) i `f64` (podwójna precyzja). Domyślnie Rust używa `f64`.
```rs
let pi: f64 = 3.1415;
let e: f32 = 2.718;
```
#### Typ `bool` (logiczny)
Rust obsługuje wartości `true` i `false`.
```rs
let aktywne: bool = true;
let wynik = 5 > 3; // wynik to true
```
#### Znaki (`char`)
Rust obsługuje 4-bajtowe znaki Unicode.
```rs
let znak: char = 'A';
let emoji = '😃';
```
#### Napisy (`str` i `String`)
- `&str` – niezmienny (statyczny) łańcuch znaków
- `String` – dynamiczny, możliwy do modyfikacji
```rs
let tekst: &str = "To jest tekst";
let mut dynamiczny_tekst = String::from("Witaj");
dynamiczny_tekst.push_str(" w Rust!");
```
### 3 Stałe (`const` i `static`)
Stałe muszą mieć określony typ i nie można ich zmieniać po zadeklarowaniu.
```rs
const PI: f64 = 3.1415;
```
`static` działa podobnie do `const`, ale przechowuje dane w stałym miejscu w pamięci.
```rs
static LICZNIK: i32 = 100;
```
### 4 Zakres zmiennych (scope)
Rust używa zasięgu blokowego, więc zmienne istnieją tylko wewnątrz `{}`.
```rs
fn main() {
    let x = 5;
    {
        let y = 10;         // Zmienna wewnętrzna
        println!("{}", y);  // OK
    }
    // println!("{}", y);   // BŁĄD: y nie istnieje poza blokiem
}
```
### 5 Cieńowanie zmiennych (shadowing)
Rust pozwala na ponowne zadeklarowanie zmiennej o tej samej nazwie.
```rs
fn main() {
    let x = 5;
    let x = x + 2;          // "Zacieniamy" wcześniejsze x
    println!("{}", x);      // 7
}
```
🚨 Cieńowanie jest inne niż `mut`, ponieważ tworzy nową zmienną zamiast modyfikować starą.

# Operatory w Rust

Rust obsługuje różne typy operatorów, w tym arytmetyczne, porównania, logiczne, bitowe, przypisania i inkrementacji/dekrementacji.

### Operatory arytmetyczne
Rust wspiera standardowe operatory matematyczne.
Operator	|Opis	|Przykład	|Wynik
--|--|--|--
`+`|Dodawanie	|`5 + 3`	|`8`
`-`|Odejmowanie	|`10 - 4`	|`6`
`*`|Mnożenie	|`3 * 4`	|`12`
`/`|Dzielenie	|`10 / 2`	|`5`
`%`|Modulo (reszta z dzielenia)	|`10 % 3`	|`1`
```rs
let a = 5;
let b = 2;
println!("Dodawanie: {}", a + b);   // 7
println!("Dzielenie: {}", a / b);   // 2 - dzelenie liczb całkowitych zwraca wynik zaokrąglony w dół
```
### Operatory porównania
Porównania zwracają `true` lub `false`.
Operator	|Opis	|Przykład	|Wynik
--|--|--|--
`==`	|Równość	|`5 == 5`	|`true`
`!=`	|Różne	|`5 != 3`	|`true`
`>`	|Większe	|`7 > 3`	|`true`
`<`	|Mniejsze	|`2 < 8`	|`true`
`>=`	|Większe lub równe	|`5 >= 5`	|`true`
`<=`	|Mniejsze lub równe	|`3 <= 4`	|`true`
```rs
let x = 10;
let y = 20;
println!("Czy x jest większe od y? {}", x > y);     //false
```
### Operatory logiczne
Służą do operacji na wartościach logicznych (`bool`).
Operator	|Opis	|Przykład	|Wynik
--|--|--|--
`&&`	|AND (i)	|`true && false`	|`false`
`\|\|`	|OR (lub)   |`true \|\| false`  |`true`
`!`	|NOT (negacja)	|`!true`	|`false`

### Operatory bitowe
Działają na poziomie bitów liczby.
Operator	|Opis	|Przykład (`a = 0b1100`, `b = 0b1010`)	|Wynik
--|--|--|--
`&`	|AND	|`a & b`	|`0b1000 (8)`
`\|`|OR	| `a \| b` | `0b1110 (14)`
`^`	|XOR	|`a ^ b`	|`0b0110 (6)`
`~`	|NOT	|`!a`	|`(-13, bo U2 na 32 bit)`
`<<`	|Przesunięcie w lewo	|`a << 1`	|`0b11000 (24)`
`>>`	|Przesunięcie w prawo	|`a >> 1`	|`0b0110 (6)`
🚨 Przesunięcie bitowe `<<` i `>>` może powodować utratę bitów!

### Operatory przypisania
Operator	|Opis	|Przykład	|Równoważne
--|--|--|--
`=`	|Przypisanie	|`x = 5`	|`x = 5`
`+=`	|Dodaj i przypisz	|`x += 2`	|`x = x + 2`
`-=`	|Odejmij i przypisz	|`x -= 3`	|`x = x - 3`
`*=`	|Pomnóż i przypisz	|`x *= 4`	|`x = x * 4`
`/=`	|Podziel i przypisz	|`x /= 2`	|`x = x / 2`
`%=`	|Modulo i przypisz	|`x %= 3`	|`x = x % 3`

# Struktury sterujące w Rust
Struktury sterujące pozwalają na kontrolowanie przepływu programu. W Rust mamy:
- Instrukcje warunkowe (`if`, `else if`, `else`, `match`)
- Pętle (`loop`, `while`, `for`)
- Instrukcje sterujące przepływem (`break`, `continue`, `return`)

## 1 Instrukcje warunkowe
### `if`, `else if`, `else`
Instrukcja `if` działa jak w innych językach, ale nie używa nawiasów `()`.
```rs
fn main() {
    let liczba = 10;

    if liczba > 0 {
        println!("Liczba jest dodatnia");
    } else if liczba < 0 {
        println!("Liczba jest ujemna");
    } else {
        println!("Liczba to zero");
    }
}
```
✅ Warunek musi być typu `bool` (np. `if liczba` jak w C nie zadziała).

🚀 `if` jako wyrażenie (może zwracać wartość).
```rs
let x = if true { 10 } else { 20 };         // x = 10
```
🚨 Wszystkie gałęzie `if` muszą zwracać ten sam typ!
```rs
let x = if true { 10 } else { "tekst" };    // BŁĄD ❌
```
### `match` – potężny switch
`match` pozwala na obsługę wielu przypadków.
```rs
fn main() {
    let liczba = 2;

    match liczba {
        1 => println!("Jeden"),
        2 => println!("Dwa"),
        3..=5 => println!("Od trzech do pięciu"),
        _ => println!("Inna liczba"), // `_` to domyślny case
    }
}
```
✅ Można używać zakresów `x..=y`.
✅ `_` oznacza dowolną inną wartość (domyślny przypadek).
✅ `match` działa także na typach `enum` i opcjonalnych wartościach (`Option`, `Result`).
## 2 Pętle
### `loop` – nieskończona pętla
`loop` działa jak `while true`, ale z możliwością wyjścia przez `break`.
```rs
fn main() {
    let mut licznik = 0;

    loop {
        licznik += 1;
        println!("Iteracja: {}", licznik);

        if licznik == 5 {
            break; // Przerwanie pętli
        }
    }
}
```
✅ `loop` można używać do zwracania wartości.
```rs
let wynik = loop {
    break 42; // Przerywa pętlę i zwraca 42
};
println!("Wynik: {}", wynik);
```
### `while` – klasyczna pętla
`while` wykonuje kod dopóki warunek jest `true`.
```rs
fn main() {
    let mut licznik = 3;

    while licznik > 0 {
        println!("Odliczanie: {}", licznik);
        licznik -= 1;
    }
}
```
### `for` – iteracja po kolekcjach
Pętla `for` działa na iteracjach (np. po `range` lub `vector`).
```rs
fn main() {
    for i in 1..5 { // Zakres 1..4 (bez 5)
        println!("i: {}", i);
    }
}
```
✅ Pełny zakres `1..=5` **(z 5)**.
```rs
for i in 1..=5 {
    println!("i: {}", i);
}
```
✅ Iteracja po tablicy.
```rs
let tablica = [10, 20, 30];
for element in tablica {
    println!("Element: {}", element);
}
```
## 3 Instrukcje sterujące przepływem
### `break` – przerwanie pętli
Przykład z `while`
```rs
fn main() {
    let mut licznik = 0;

    while licznik < 10 {
        licznik += 1;
        if licznik == 5 {
            break;  // Wyjście z pętli, gdy licznik = 5
        }
    }
}
```
✅ `break` może zwracać wartość w `loop` (np. do przypisania zmiennej).
### `continue` – pominięcie iteracji
```rs
fn main() {
    for i in 1..=5 {
        if i == 3 {
            continue; // Pomija wartość 3
        }
        println!("{}", i);
    }
}
```
✅ Wydrukuje: 1, 2, 4, 5 (pominie 3).
### `return` – zwracanie wartości z funkcji
`return` kończy funkcję i zwraca wartość.
```rs
fn podwoj(x: i32) -> i32 {
    return x * 2; // Zwraca wartość
}
```
✅ W Rust można pominąć `return`, jeśli ostatnia instrukcja zwraca wartość.
```rs
fn podwoj(x: i32) -> i32 {
    x * 2 // Brak `return`, ale działa
}
```
🚨 Bez `;` (bo inaczej zwraca `()` zamiast wartości).

# Funkcje w Rust

Funkcje w Rust są podstawowymi blokami kodu i mogą przyjmować argumenty, zwracać wartości oraz mieć różne sposoby przekazywania parametrów.

### Deklaracja i definicja funkcji

Funkcję definiuje się za pomocą słowa kluczowego `fn`.

```rs
fn nazwa() {
    println!("To jest funkcja!");
}

fn main() {
    nazwa();    // Wywołanie funkcji
}
```
✅ Rust używa `snake_case` do **nazw funkcji** (np. `moja_funkcja`).

### Argumenty funkcji
Funkcja może przyjmować argumenty i określać ich typy

```rs
fn powitanie(imie: &str) {
    println!("Cześć, {}!", imie);
}

fn main() {
    powitanie("Alice");
}
```
✅ Rust wymaga jawnego określenia typów argumentów.

🚨 Argumenty przekazywane przez wartość są kopiowane (dla typów Copy) lub przenoszone (dla typów bez Copy).

### Zwracanie wartości
Funkcja może zwracać wartość, określając jej typ po `->`.
```rs
fn podwoj(x: i32) -> i32 {
    x * 2       // Brak średnika – ostatnia instrukcja to wartość zwracana
}

fn main() {
    let wynik = podwoj(5);
    println!("Wynik: {}", wynik);
}
```
✅ Brak `;` na końcu sprawia, że Rust traktuje to jako wartość zwracaną.
✅ Można też użyć `return`, ale nie jest to konieczne.
```rs
fn podwoj(x: i32) -> i32 {
    return x * 2;
}
```