# Zestaw 7b
1. Zdefiniuj typ strukturalny (wraz z implementacją metod) przeznaczony do przechowywania macierzy prostokątnej wartości typu `f64`. Weź pod uwagę nastepujące możliwości:
- konstruktor `new(wysokość, szerokość, wypełniacz) -> Macierz`
- konstruktor `zerowa(wysokość, szerokość) -> Macierz`
- konstruktor `jednostkowa(wysokość) -> Macierz`
- akcesor `element(indeks_wiersz, indeks_kolumny) -> f64`
- mutator `zmien_element(indeks_wiersza, indeks_kolumny, nowa_wartość)`
- konstruktor `suma(macierz1, macierz2) -> Option<Macierz>`
- metodę `wyświetl`

Napisz testy.