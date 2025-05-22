# Zestaw 9a
1. Tradycyjne karty do gry (brydż, poker itd.) podzielone są na kolory o nazwach:
- pik,
- kier,
- karo,
- trefl.

Zaprojektuj typ wyliczeniowy, który będzie mógł reprezentować dane o kolorze z dodatkowym warunkiem, że są one uporządkowane jak w brydżu (zgodnie z wypunktowaniem powyżej, gdzie są podane malejąco). Przetestuj wszystko.

Wskazówka: Derywuj odpowiednią cechę i ułóż warianty w odpowiedniej kolejności.

2. Zaprojektuj typ do przechwoywania informacji o następujących możliwych błędach:
- brak błędu;
- zły format pliku;
- plik nie istnieje (z tym błędem musi być związana wartość nazwy pliku);
- plik zbyt duży (z tym błędem muszą być związane dwie wartości — rozmiar aktualny i rozmiar maksymalny).

Dodatkowo zaimplementuj dla tego typu metodę o nazwie `pokaz_komunikat`, która wyświetli pełny komunikat o podanym błędzie (wraz z jego danymi).