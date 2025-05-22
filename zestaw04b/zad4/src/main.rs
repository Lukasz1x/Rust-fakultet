fn na_rzymskie(mut liczba: u32) -> String
{
    let mut result:String = String::from("");
    let t=[("M", 1000_u32), ("CM", 900), ("D", 500), ("CD", 400), ("C", 100), ("XC", 90),  ("L", 50), ("XL", 40), ("X", 10), ("IX", 9), ("V", 5), ("IV", 4), ("I", 1)];
    let mut i=0;
    while liczba > 0
    {
        if t[i].1 <= liczba
        {
            liczba-=t[i].1;
            result.push_str(t[i].0);
        }else
        {
            i+=1;
        }
    }


    result
}

fn main() {
    println!("{}", na_rzymskie(1) == "I");
    println!("{}", na_rzymskie(3) == "III");
    println!("{}", na_rzymskie(9) == "IX");
    println!("{}", na_rzymskie(19) == "XIX");
    println!("{}", na_rzymskie(1164) == "MCLXIV");
    println!("{}", na_rzymskie(1910) == "MCMX");
    println!("{}", na_rzymskie(3888) == "MMMDCCCLXXXVIII");
}

