### Programowanie strukturalne i napisy

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

### Option<> i Result<>

1. Napisz funkcję `fraction(numerator: i32, denominator: i32) -> Option<f32>`, która wykona odpowiednie dzielenie lub zwróci `None`, jeżeli to niemożliwe.
2. Napisz funkcję `position(element: i32, array: &[i32] -> Option<usize>)`. Funkcja powinna zwrócić indeks elementu w tablicy lub `None`, jeżeli element nie jest w tablicy.
3. Napisz funkcję `divisors(number: Option<u32>) -> usize`, która zwróci liczbę dzielników parametru number lub zakończy działanie programu, jeśli number ma wartość `None`.
4. Napisz funkcję `wizytowka(imie: Option<String>, nazwisko: Option<String>) -> String`, która stworzy wizytówkę, w której w przypadku braku imienia zostanie użyte imię Jan, a w przypadku braku nazwiska -- Kowalski.
5. Napisz funkcję `miejsce_zerowe(a: f32, b: f32, c: f32) -> (Option<f32>, Option<f32>)`, która oblicza rzeczywiste miejsca zerowe funkcji kwadratowej.
6. Napisz funkcję `oceny(punkty: &[u32], &mut[Result<u8, u32>])`, która przyporządkuje oceny studentom według pewnego klucza. Jeśli ktoś ma więcej niż 100 punktów, należy na tej pozycji umieścić wartość, informującą o błędzie i podać liczbę punktów ponad progiem.
7. Napisz funkcję `rozne_liczby(arr: &[&str], out: &mut [Result<u32, u32>])`, która przyjmie tablicę liczb zapisanych w postaci napisów w systemach dziesiętnym i szesnastkowym. Funkcja powinna rozpoznać system, w którym zapisana jest liczba i przekonwertować ją do zmiennej typu `u32`. Przyjmij, że liczby szesnastkowe oznaczone są prefiksem `0x`. Nie wszystkie napisy muszą być poprawne; zadbaj o obsługę błędów.

### Pętle i iteratory

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