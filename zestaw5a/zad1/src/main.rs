fn zamien_syst8_na_syst2(z: &str) -> Option<String>
{
    if z.len() == 0
    {
        return None;
    }
    let mut wynik = String::new();
    let mut zero = false;
    for x in z.chars()
    {
        let mut znak = (x as i8)-(b'0' as i8);
        if znak >8 || znak <0
        {
            return None;
        }
        let mut d =4;
        while d>0
        {
            if znak>=d
            {
                znak-=d;
                wynik.push('1');
                zero=true;
            }else
            {
                if zero
                {
                    wynik.push('0');
                }
            }
            d/=2;
        }
    }
    if !zero
    {
        wynik.push('0');
    }

    Some(wynik)
}

fn main() {
    println!("{:?}", zamien_syst8_na_syst2("0720"));
    println!("{:?}", zamien_syst8_na_syst2("00"));
    println!("{:?}", zamien_syst8_na_syst2("01234567"));
    println!("{:?}", zamien_syst8_na_syst2("jakies inne znaki"));
    println!("{:?}", zamien_syst8_na_syst2(""));
}
