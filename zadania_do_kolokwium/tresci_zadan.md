## Programowanie strukturalne i napisy

1. Napisz funkcję `d2((x, y), (x, y)) -> f32`, która obliczy dystans
   pomiędzy punktami w przestrzeni *R^2*.
2. Napisz funkcję `d3((x, y, z), (x, y, z)) -> f32`, która obliczy dystans
   pomiędzy punktami w przestrzeni *R^3*.
3. Stwórz tablicę *N* elementów, którą wypełnisz resztami z dzielenia liczby
   `100` przez kolejne liczby naturalne. Następnie wyświetl kolejne wartości
   tablicy od końca.
4. Napisz funkcję `avg(&[u32]) -> f32`, która obliczy średnią arytmetyczną
   liczb z tablicy.
5. Napisz funkcję `sort(... u32, ... u32, ... u32)`, która rosnąco posortuje
   przekazane jej argumenty.
6. Napisz funkcję `swap_range(... [u32], (a1, a2), (b1, b2))`, która zamieni
   miejscami elementy, leżące w podanych przedziałach; jeśli przedziały mają
   różną długość, ogranicz się do długości krótszego z nich.
7. Napisz funkcję, która na podstawie napisu tworzy napis, zawierający co drugi znak napisu podanego w argumencie.

## Option<> i Result<>

1. Napisz funkcję `fraction(numerator: i32, denominator: i32) -> Option<f32>`, która wykona odpowiednie dzielenie lub zwróci `None`, jeżeli to niemożliwe.
2. Napisz funkcję `position(element: i32, array: &[i32] -> Option<usize>)`. Funkcja powinna zwrócić indeks elementu w tablicy lub `None`, jeżeli element nie jest w tablicy.
3. Napisz funkcję `divisors(number: Option<u32>) -> usize`, która zwróci liczbę dzielników parametru number lub zakończy działanie programu, jeśli number ma wartość `None`.
4. Napisz funkcję `wizytowka(imie: Option<String>, nazwisko: Option<String>) -> String`, która stworzy wizytówkę, w której w przypadku braku imienia zostanie użyte imię Jan, a w przypadku braku nazwiska -- Kowalski.
5. Napisz funkcję `miejsce_zerowe(a: f32, b: f32, c: f32) -> (Option<f32>, Option<f32>)`, która oblicza rzeczywiste miejsca zerowe funkcji kwadratowej.
6. Napisz funkcję `oceny(punkty: &[u32], &mut[Result<u8, u32>])`, która przyporządkuje oceny studentom według pewnego klucza. Jeśli ktoś ma więcej niż 100 punktów, należy na tej pozycji umieścić wartość, informującą o błędzie i podać liczbę punktów ponad progiem.
7. Napisz funkcję `rozne_liczby(arr: &[&str], out: &mut [Result<u32, u32>])`, która przyjmie tablicę liczb zapisanych w postaci napisów w systemach dziesiętnym i szesnastkowym. Funkcja powinna rozpoznać system, w którym zapisana jest liczba i przekonwertować ją do zmiennej typu `u32`. Przyjmij, że liczby szesnastkowe oznaczone są prefiksem `0x`. Nie wszystkie napisy muszą być poprawne; zadbaj o obsługę błędów.

## Pętle i iteratory

1. Napisz funkcję o nagłówku `fn powtorki(t: ...) -> ...` która z danego
   wektora utworzy nowy z tymi samymi wartościami, ale tylko tymi, które się po
   sobie powtarzają. Na przykład:
   `powtorki(&vec![1, 3, 4, 3, 3, 3, 3, 4, 1, 1, 6]) == vec![3, 3, 3, 3, 1, 1]`
2. Napisz funkcję o nagłówku `fn unikalne(t: ...) -> ...` która utworzy i
   zwróci nowy wektor, ale tylko z wartościami, które w danym wektorze się nie
   powtarzają (w zachowanej kolejności). Na przykład:
`unikalne(&vec![1, 3, 4, 3, 3, 5, 3, 4, 1, 1, 6]) == vec![5, 6]`
3. Napisz funkcję o nagłówku `fn zlicz_wiele(s1: ..., s2: ...) -> ...` która
   zwróci liczbę elementów (z powtórzeniami) wektora s1 zawartych w s2 (lub
           odwrotnie — wynik będzie ten sam).

```
zlicz_wiele(&vec![1, 2, 1, 3], &vec![1, 2]) == 3
zlicz_wiele(&vec![1, 2, 1, 3], &vec![12]) == 0
zlicz_wiele(&vec![1, 2, 1, 3], &vec![1, 2, 2]) == 4
zlicz_wiele(&vec![1, 2, 1, 3], &vec![1, 2, 2, 1]) == 6
```

## II kolos - struktury, derywacja cech i iteratory

#### 1. Zaprojektuj struktury do reprezentowania studenta i jego ocen:
- `enum Ocena { A, B, C, D, E, F }`
- `struct Student { imie: String, nazwisko: String, oceny: Vec<Ocena> }`\
Zaimplementuj Dziennik, który przechowuje `Vec<Student>` i metody:
- `dodaj_studenta(...)`
- `dodaj_ocene(nazwisko: &str, ocena: Ocena)`
- `srednia_ocen(nazwisko: &str) -> Option<f32>` (przelicz A=5.0 ... F=0.0)
- `najlepsi_studenci(min_avg: f32) -> Vec<String>` (nazwiska studentów)
- `statystyka_ogolna_ocen() -> HashMap<Ocena, usize>`
#### 2. Zaprojektuj system, gdzie użytkownicy oceniają książki:
- `struct Recenzja { uzytkownik: String, ocena: u8, komentarz: String }`
- `struct Ksiazka { tytul: String, autor: String, recenzje: Vec<Recenzja> }`\
Zaimplementuj Biblioteka, która przechowuje Vec<Ksiazka> i metody:
- `dodaj_ksiazke(...)`
- `dodaj_recenzje(tytul: &str, recenzja: Recenzja)`
- `srednia_ocena(tytul: &str) -> Option<f32>`
- `najlepiej_oceniane(liczba: usize) -> Vec<String>` (tytuły)
- `komentarze_dla_autora(autor: &str) -> Vec<String>`
#### 3. Przy użyciu iteratorów wygeneruj wektory zawierające:
- małe litery alfabetu angielskiego ('a'..='z')
- kwadraty liczb 1..=10 → [1, 4, 9, 16, ..., 100]
- 10 pierwszych potęg liczby 2 → [1, 2, 4, 8, 16, ...]
- odwrotności liczb od 1 do 20 → [1.0, 0.5, 0.333, ...]
- liczby z przedziału 1..=100, które są podzielne przez 3, ale nie przez 4
#### 4. Zaimplementuj funkcje, które dla danego `Vec<string>` zwrócą tylko te elementy które są:
- krótsze niż 4 znaki: `fn krotsze_niz_4(wej: Vec<String>) -> Vec<String>`
- nie zawierają literty 'a' i 'A': `fn bez_liter_a(wej: Vec<String>) -> Vec<String>`
- zawierają cyfry: `fn zawierajace_cyfry(wej: Vec<String>) -> Vec<String>`
- są palindromami (np. kajak): `fn palindromy(wej: Vec<String>) -> Vec<String>`
- mają podwojoną literę (np. wanna): `fn z_podwojeniem_litery(wej: Vec<String>) -> Vec<String>`
#### 5. Wektor w przestrzeni 3D
Zaprojektuj strukturę `Wektor3D`, która reprezentuje wektor w przestrzeni trójwymiarowej `(x, y, z: f64)`.
```rs
#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Wektor3D {
    x: f64,
    y: f64,
    z: f64,
}
```
Zaimplementuj ręcznie:
- std::fmt::Display – wypisywanie jako (x, y, z)
- std::ops::Add – dodawanie dwóch wektorów
- std::ops::Sub – odejmowanie dwóch wektorów
- std::cmp::PartialOrd – porównywanie długości wektorów
    >Porównanie wektorów ma opierać się na długości (sqrt(x² + y² + z²))

Dodaj metody:
- `fn dlugosc(&self) -> f64`
- `fn norma(&self) -> Self`
    >Zwraca znormalizowany wektor (o długości 1, w tym samym kierunku)
- `fn iloczyn_skalarny(&self, other: &Self) -> f64`
- `fn iloczyn_wektorowy(&self, other: &Self) -> Self`

Zaimplementuj funkcję:
```rs
fn posortuj_po_dlugosci(mut wektory: Vec<Wektor3D>) -> Vec<Wektor3D>
```
- sortuje wektory rosnąco według długości
- (opcjonalnie) użyj `.sort_by(|a, b| a.partial_cmp(b).unwrap())`
#### 6. Zadanie: System zarządzania projektami zespołowymi

Opis:
- Zaprojektuj system do zarządzania projektami, zespołami i zadaniami. W systemie:
- Każdy projekt ma unikalny identyfikator, nazwę, zespół i listę zadań.
- Każdy członek zespołu ma imię, nazwisko, i specjalizację (enum Specjalizacja).
- Każde zadanie ma nazwę, przypisaną osobę (opcjonalnie), status (enum StatusZadania) i liczbę godzin oszacowanych oraz rzeczywistych.

Struktury:
```rs
enum Specjalizacja {
    Frontend,
    Backend,
    DevOps,
    QA,
    Manager,
}

enum StatusZadania {
    Nowe,
    WTrakcie,
    Zakonczone,
    Odrzucone(String), // powód odrzucenia
}

struct CzlonekZespołu {
    imie: String,
    nazwisko: String,
    specjalizacja: Specjalizacja,
}

struct Zadanie {
    nazwa: String,
    przypisany: Option<String>, // nazwisko osoby
    status: StatusZadania,
    godziny_oszacowane: u32,
    godziny_rzeczywiste: Option<u32>,
}

struct Projekt {
    id: u32,
    nazwa: String,
    zespol: Vec<CzlonekZespołu>,
    zadania: Vec<Zadanie>,
}
```

Wymagania – zaimplementuj metody dla impl Projekt:
- `fn dodaj_czlonka(&mut self, czlonek: CzlonekZespołu)`
- `fn dodaj_zadanie(&mut self, zadanie: Zadanie)`
- `fn przypisz_zadanie(&mut self, nazwa_zadania: &str, nazwisko: &str) -> Result<(), String>`
- `fn zakoncz_zadanie(&mut self, nazwa: &str, godziny: u32) -> Result<(), String>`
- `fn odrzuc_zadanie(&mut self, nazwa: &str, powod: String) -> Result<(), String>`
- `fn godziny_na_osobe(&self) -> HashMap<String, u32>`
(sumuje godziny oszacowane przypisanych zadań)
- `fn zadania_dla_specjalizacji(&self, spec: Specjalizacja) -> Vec<String>`
(nazwy zadań przypisane osobom o danej specjalizacji)
- `fn raport(&self)` – wypisz zadania z nazwiskiem, statusem i godzinami

Dodatkowe wymaganie – zaimplementuj impl Projekt:
- `fn przeciazony_pracownik(&self) -> Option<String>`\
(znajdź pracownika, którego suma godzin oszacowanych przekracza 100)
- `fn statystyki_statusow(&self) -> HashMap<StatusZadania, usize>`\
(StatusZadania powinien implementować Hash + Eq, możesz użyć derive)

