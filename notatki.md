# Komendy cargo
```
cargo new [nazwa]           tworzy projekt w nowym katalogu o podanej nazwie
cargo init                  tworzy projekt w aktualnym katalogu
    --vcs none              po dodaniu projekt jest tworzony bez repozytorium git

cargo build                 kompilacja
cargo run                   uruchamianie (automatyczna kompilacja, jeśli plik był zmieniony od poprzedniej kompilacji)
./target/debug/program      uruchamianie bez kompilacji
```


# Elementy składni kodu
### Komentarze
```rs
// To jest komentarz
/* To 
jest 
komentarz 
wieloliniowy */
```

### Tworzenie zmiennych/stałych
```rs
let [mut] nazwa [:typ][=wartosc];
```
- mut - dodanie go pozwala na zmianę wartości
- :typ - określa typ, musi być jeśli wartość nie jest przypisana
- =wartość - przypisuje wartość, jeśli występuje to nie wymaga podania typu
przykłady:
```rs
let a :i32;
let b = 5;
let mut c :f32 = 6.0;
 ```

Typy proste:
- liczby całkowite ze znakiem: `i8`, `i16`, `i32`, `i64`, `i128`
- liczby całkowite bez znaku: `u8`, `u16`, `u32`, `u64`, `u128`
- liczby zmiennoprzecinkowe: `f32`, `f64`
- pojedyńcze znaki Unicode: `char`
- wartości logiczne: `bool`

### Instrukcje I/O

Wypisanie tekstu z nową linią
```rs
println!("napis");
```
Wypisanie tesktu bez nowej lini na końcu
```rs
print!("napis");
```
Wypisywanie stałych/zmiennych
```rs
let a = 5;
let mut b = 6;
println!("liczba {a} jest mniejsza od liczby {b}");
println!("liczba {} jest mniejsza od liczby {}", a, b);
```
### Instrukcje warunkowe
w przeciwieństwie do cpp w rust wymagane są nawisy `{}` a `()` mogą być pominięte tzn `if warunek { instrukcje }` .

Łączenie warunków (działa tak samo jak w cpp):
- `&&` AND
- `||` OR
- `!`  negacja
- Nawiasy `()` mogą być użyte przy kilku warunkach aby wymusić odpowiednią kolejność

`if else`
```rs
let x = 5;
let y = 10;

if x > 3 && y < 15 {
    println!("Oba warunki są prawdziwe: x > 3 oraz y < 15");
} else {
    println!("Przynajmniej jeden warunek jest fałszywy");
}
```
`else if`
```rs
let x = 5;

if x > 10 {
    println!("x jest większe niż 10");
} else if x == 5 {
    println!("x jest równe 5");
} else {
    println!("x jest mniejsze niż 5");
}
```
### Pętle
`pętla while`

Podobnie jak w instrukcjach warunkowych w pętli while wymagane są nawisy `{}` a `()` mogą być pominięte tzn `while warunek { instrukcje }` .

Za pomocą instrukcji `break` pętlę można przerwać w dowolnym momencie.
```rs
let mut counter = 0;

while counter < 10 {
    if counter == 5 {
        println!("Zatrzymuję pętlę w momencie, gdy counter osiąga 5");
        break; // Kończymy pętlę, gdy counter osiągnie 5
    }
    println!("Counter: {}", counter);
    counter += 1;
}
```
Przy pomocy instrukcji `continue` możemy przeskoczyć do następnej iteracji pętli, pomijając kod, który znajduje się poniżej `continue` w bieżącej iteracji.
```rs
let mut counter = 0;

while counter < 10 {
    counter += 1;
    if counter % 2 == 0 {
        continue; // Pomija liczby parzyste
    }
    println!("Nieparzysta liczba: {}", counter);
}
```

