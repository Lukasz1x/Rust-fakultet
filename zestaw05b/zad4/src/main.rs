fn wartosc_cyfry_rzymskiej(c: char) -> Result<u16, String>
{

    let rzymskie = [('I', 1),  ('V', 5), ('X', 10), ('L', 50), ('C', 100), ('D', 500), ('M', 1000)];
    for i in rzymskie
    {
        if c == i.0
        {
            return  Ok(i.1);
        }
    }

    Err("Podany znak nie jest cyfrą rzymską!".to_string())
}

fn rzymskie(napis: &str) -> Result<u128, String>
{
    if napis.len() == 0
    {
        return Err("Pusty napis!".to_string());
    }
    let mut liczba :i128 = 0;
    let w :Vec<char> = napis.chars().collect();

    // sprawdzanie poprawności liczby
    if w.len()>3
    {
        for i in 0..w.len() - 3
        {
            // powtórzenia (max 3)
            if w[i] == w[i + 1] && w[i] == w[i + 2] && w[i] == w[i + 3]
            {
                return Err("Niewłaściwa kolejność cyfr!".to_string());
            }
        }
    }
    if w.len()>2 {
        for i in 0..w.len() - 2
        {
            // max jedna mniejsza przed większą
            let poprzednia = wartosc_cyfry_rzymskiej(w[i])? as i128;
            let aktualna = wartosc_cyfry_rzymskiej(w[i + 1])? as i128;
            let nastepna = wartosc_cyfry_rzymskiej(w[i + 2])? as i128;
            if aktualna == poprzednia && nastepna > aktualna
            {
                return Err("Niewłaściwa kolejność cyfr!".to_string());
            }
        }
    }
    for i in 0..w.len()-1
    {
        let mut aktualna = wartosc_cyfry_rzymskiej(w[i])? as i128;
        let mut nastepna = wartosc_cyfry_rzymskiej(w[i+1])? as i128;
        if aktualna < nastepna
        {
            if (nastepna != 5 * aktualna) && (nastepna != 10 * aktualna)
            {
                return Err("Niewłaściwa kolejność cyfr!".to_string());
            }
        }
        // nie można powtarzać V, L, D
        if w[i] == w[i + 1]
        {
            if w[i] == 'V' || w[i] == 'L' || w[i] == 'D'
            {
                return Err("Niewłaściwa kolejność cyfr!".to_string());
            }
        }
    }

    for i in 0..w.len()-1
    {
        let mut aktualna = wartosc_cyfry_rzymskiej(w[i])? as i128;
        let mut nastepna = wartosc_cyfry_rzymskiej(w[i+1])? as i128;
        //println!("aktualna: {}, nastepna: {}", aktualna, nastepna);
        if aktualna < nastepna
            {
            liczba -= aktualna;
        }else {
            liczba += aktualna;
        }
        //println!("liczba: {}", liczba);
    }
    liczba += wartosc_cyfry_rzymskiej(w[w.len()-1])? as i128;

    Ok(liczba as u128)
}
fn main() {
    println!("{:?}", rzymskie("III"));  // 3
    println!("{:?}", rzymskie("IX"));   //9
    println!("{:?}", rzymskie("XIX"));  //19
    println!("{:?}", rzymskie("MCMX")); //1910
    println!("{:?}", rzymskie("IV"));   //4

    println!("{:?}", rzymskie("VV"));
    println!("{:?}", rzymskie("XXXX"));
    println!("{:?}", rzymskie("IIII"));
    println!("{:?}", rzymskie("DDD"));
    println!("{:?}", rzymskie("IIID"));
    println!("{:?}", rzymskie("IM"));
    println!("{:?}", rzymskie(""));
    println!("{:?}", rzymskie("sdfaf"));
}
