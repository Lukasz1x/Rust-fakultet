
fn ilosc_cyfr(mut n:i32) -> u32
{
    let mut w =0;
    while n > 0
    {
        n/=10;
        w+=1;
    }

    w
}

fn czy_liczba_armstronga(n:i32) -> bool
{
    let mut kopia=n;
    let mut suma=0;
    let cyfry = ilosc_cyfr(n);

    for _i in 0..cyfry
    {
        suma+=(kopia%10).pow(cyfry);
        kopia/=10;
    }

    suma==n
}

fn main()
{
    let n=4210818;
    println!("Liczba {}, {} liczbą armstronga", n, if czy_liczba_armstronga(n) {"jest"}else {"nie jest"});
}
