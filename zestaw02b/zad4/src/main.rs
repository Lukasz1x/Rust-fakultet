fn czy_doskonala(n:i64) -> bool
{
    let mut suma=0;
    for i in 1..n
    {
        if n%i==0
        {
            suma+=i;
        }
    }
    suma==n
}

fn main()
{
    let n =33550336;
    println!("Liczba {}, {} liczbą doskonałą", n, if czy_doskonala(n) {"jest"}else {"nie jest"});
}
