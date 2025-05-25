# Zestaw 9b

1. Stwórz strukturę `Date` oraz zaimplementuj dla niej następujące metody:
- `fn to_string(&self) -> String`
- `fn from_3(day: u8, month: Month, year: u16) -> Date`
- `fn from_string(string: &str, delim: char) -> Date`
2. Stwórz strukturę `Time` oraz zaimplementuj dla niej metody analogiczne jak dla struktury `Date`.

3. Zmodyfikuj strukturę `Date` tak, aby mogła **opcjonalnie** przechowywać również czas. Dodaj metody:
- `fn has_time(&self) -> bool`
- `fn set_time(&mut self, time: ...)`
- `fn clear_time(&mut self)`
4. W zgodzie ze zdroworozsądkowym poczuciem czasu zaimplementuj cechy: `PartialOrd`, `Ord` , `PartialEq`, `Eq` dla zmodyfikowanej struktury `Date`. Zastanów się nad pięknem ludzkiego postrzegania czasu oraz czasem-samym-w-sobie ;)

5. Stwórz strukturę `Task`. Powinna zawierać następujące pola:

- `name: String`
- `description: String`
- `priority: Priority // Low, Medium, High`
- `due: Date`
6. Zaimplementuj sensownie cechy `PartialOrd`, `PartialEq` dla struktury `Task`.