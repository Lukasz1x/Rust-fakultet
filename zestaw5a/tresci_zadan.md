# Zestaw 5a
1. Napisz funkcję o nagłówku\
`fn zamien_syst8_na_syst2(z: &str) -> Option<String>`\
zamieniającą zapis liczby całkowitej bez znaku w systemie ósemkowym na zapis w systemie dwójkowym. Wynik ma być najkrótszy możliwy, niepusty. Wynik None ma oznaczać wystąpienie w parametrze z niedozwolonego znaku (spoza cyfr ósemkowych) lub pusty napis w parametrze.

2. Napisz funkcję o nagłówku\
`fn wartosc_syst2(z: &str) -> Option<u8>`\
obliczającą wartość całkowitą bez znaku zapisaną w systemie dwójkowym — pod warunkiem, że mieści się na ośmiu bitach. Jeśli nie (lub w zapisie występuje znak inny niż cyfra dwójkowa lub parametr jest pusty), to wynikiem jest None.

3. Napisz funkcję o nagłówku\
`fn wartosc_syst8(z: &str) -> Option<u8>`\
obliczającą wartość całkowitą bez znaku zapisaną w systemie ósemkowym — pod warunkiem, że mieści się na ośmiu bitach. Jeśli nie (lub w zapisie występuje znak inny niż cyfra ósemkowa lub parametr jest pusty), to wynikiem jest None.\
***Uwaga 1*** Funkcję tę należy zbudować z funkcji z zadań poprzednich.\
***Uwaga 2*** Zrób dwie wersje tej funkcji — pierwszą bez znaku zapytania, a drugą ze znakiem zapytania.