fn wartosc_syst2(z: &str) -> Option<u8>
{
    if z.len() == 0
    {
        return None;
    }
    let mut wynik:u8 =0;
    let mut dwa : u8 = 1;
    let mut bit8 =false;
    for x in z.chars().rev()
    {
        let znak = (x as i8)-(b'0' as i8);
        if znak != 0 && znak != 1
        {
            return None;
        }
        if znak == 1
        {
            if bit8
            {
                return None;
            }
            wynik += dwa;
        }
        if dwa < 128
        {
            dwa *= 2;
        }else {
            bit8 =true;
        }

    }

    Some(wynik)
}

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

fn wartosc_syst8(z: &str) -> Option<u8>
{
    let w1 = zamien_syst8_na_syst2(&z);
    if w1.is_none()
    {
        return None;
    }
    let w1 = w1.unwrap();
    let w2 = wartosc_syst2(&w1);
    if w2.is_none()
    {
        return None;
    }
    w2
}
fn wartosc_syst8_v2(z: &str) -> Option<u8>
{
    Some(wartosc_syst2(&zamien_syst8_na_syst2(&z)?)?)
}

fn main() {
    println!("{:?}", wartosc_syst8("072"));
    println!("{:?}", wartosc_syst8("00"));
    println!("{:?}", wartosc_syst8("01234567"));
    println!("{:?}", wartosc_syst8("jakies inne znaki"));
    println!("{:?}", wartosc_syst8(""));
    println!("{:?}", wartosc_syst8("123"));
    println!("{:?}", wartosc_syst8("23"));
    println!("{:?}", wartosc_syst8("54"));
    println!();
    println!("{:?}", wartosc_syst8_v2("072"));
    println!("{:?}", wartosc_syst8_v2("00"));
    println!("{:?}", wartosc_syst8_v2("01234567"));
    println!("{:?}", wartosc_syst8_v2("jakies inne znaki"));
    println!("{:?}", wartosc_syst8_v2(""));
    println!("{:?}", wartosc_syst8_v2("123"));
    println!("{:?}", wartosc_syst8_v2("23"));
    println!("{:?}", wartosc_syst8_v2("54"));
}
