fn wizytowka(imie: &str, nazwisko :&str) -> String
{
    let mut wynik :String=String::new();
    wynik.push(imie.chars().nth(0).unwrap());
    wynik.push_str(". ");
    wynik.push(nazwisko.chars().nth(0).unwrap());
    wynik=wynik.to_uppercase();
    let n :Vec<char> = nazwisko.to_lowercase().chars().collect();
    for k in 1..n.len()
    {
        wynik.push(n[k]);
    }

    wynik
}


fn main() {
    println!("{}", wizytowka("jan", "KOWALSKI"));
}
