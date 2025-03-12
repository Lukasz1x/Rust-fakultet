fn kolejna_pierwsza(mut n:i32) -> i32
{
    let mut pierwsza=false;
    while !pierwsza {
        n+=1;
        pierwsza=true;
        for i in 2..(n as f64).sqrt() as i32
        {
            if n%i==0
            {
                pierwsza=false;
            }
        }
    }

    n
}

fn rozklad(mut n:i32)
{
    println!("Rozklad liczby {n} na czynniki pierwsze: ");
    let mut p=2;
    while n>1
    {
        while n%p==0
        {
            n/=p;
            print!("{p} ");
        }
        p=kolejna_pierwsza(p);
    }
}

fn main()
{
    rozklad(738);
}
