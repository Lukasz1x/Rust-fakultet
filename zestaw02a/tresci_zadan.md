# Zestaw 2a
1. Zadania numer 6 oraz 9 z Zestawu 1 zrób na dwa sposoby (każde) — z użyciem pętli while/loop oraz z użyciem pętli for.
2. Przy założeniu, że mamy zdefiniowane dwie funkcje


fn f(x: f64) -> f64\
fn fp(x: f64) -> f64

(spełniające odpowiednie założenia; druga jest pochodną pierwszej) napisz funkcję


fn met_newt(x0: f64, eps: f64, n: u128) -> f64\
realizującą znajdowanie przybliżonego miejsca zerowego metodą Newtona — w czterech wersjach:
- iteracyjnej z pętlą loop (z ewentualnymi break continue return);
- iteracyjnej z pętlą while (bez żadnych break continue return);
- rekurencyjnej;
- iteracyjnej z pętlą for (z ewentualnymi break continue return).