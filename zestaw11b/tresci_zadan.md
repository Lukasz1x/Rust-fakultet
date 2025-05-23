# Zestaw 11b
1. Stwórz strukturę `Ksiazka`, która przechowuje tytuł, autora i liczbę stron. Następnie stwórz `enum Gatunek` z kilkoma gatunkami. Przechowuj książki w wektorze i zaimplementuj metodę filtrującą tylko książki podanego gatunku, liczące więcej niż 300 stron.

2. Zaimplementuj prosty system zgłoszeń. Każde zgłoszenie ma unikalny identyfikator, tytuł oraz status (`enum Status { Otwarte, Przetwarzane, Zamkniete(Rezultat) }`, gdzie Rezultat także jest `enum`em). Zgłoszenia są trzymane w wektorze. Dodaj metody do zmiany statusu oraz listowania tylko zgłoszeń o danym statusie.

3. Zdefiniuj `enum PaymentMethod { Cash, Card, Transfer }` oraz strukturę `Transaction` z polem `amount: f64` i `method: PaymentMethod`. Zaimplementuj funkcję, która sumuje wszystkie transakcje danego typu z tablicy/wektora.

4. Zaimplementuj prosty system zarządzania rezerwacjami lotniczymi.

- Każda rezerwacja ma pasażera, numer lotu, klasę podróży (ekonomiczna, biznesowa, pierwsza), oraz status (zarezerwowane, odwołane).
- Struktura `SystemRezerwacji` przechowuje `Vec<Rezerwacja>`.
- Dodaj metody:
    - dodania nowej rezerwacji,
    - anulowania rezerwacji na podstawie nazwiska pasażera,
    - zliczania liczby rezerwacji w danej klasie i statusie,
    - listowania pasażerów w podanej klasie, posortowanych alfabetycznie,

5. Zaprojektuj strukturę `DanePogodowe`, która zawiera:
- lokalizację (`String`),
- dzień kolejny obserwacji (`u32`),
- temperaturę (`f32`),
- warunki pogodowe (słońce, chmury, deszcz, śnieg).

Zaimplementuj `DziennikPogodowy`, który:
- przechowuje dane z wielu dni,
- oferuje metody:
    - średnia temperatura dla danego typu pogody,
    - najczęstszy typ pogody,
    - znalezienie słonecznych dni z temperaturą > 30°C