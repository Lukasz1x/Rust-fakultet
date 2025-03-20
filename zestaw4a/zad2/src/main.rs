fn zamien(znak: char) -> i32
{
    if znak == 'M'
    {
        return 1000;
    }else if znak == 'D'
    {
        return 500;
    }
    else if znak == 'C'
    {
        return 100;
    }
    else if znak == 'L'
    {
        return 50;
    }
    else if znak == 'X'
    {
        return 10;
    }else if znak == 'V'
    {
        return 5;
    }
    else if znak == 'I'
    {
        return 1;
    }
    0
}

fn rzymskie(napis: &str) -> i32
{
    let mut liczba :i32 =0;
    let w1 :Vec<char> = napis.chars().collect();
    for i in 0..w1.len()-1
    {
        if zamien(w1[i]) < zamien(w1[i+1])
        {
            liczba -= zamien(w1[i]);
        }else {
            liczba += zamien(w1[i]);
        }
    }
    liczba + zamien(w1[w1.len()-1])
}

fn main() {
    println!("{}", rzymskie("III"));
    println!("{}", rzymskie("IX"));
    println!("{}", rzymskie("XIX"));
    println!("{}", rzymskie("MCMX"));
    println!("{}", rzymskie("IV"));
}
