fn collatz (mut n:u32) -> u32
{

    let mut w:u32 = 0;
    while n != 1
    {
        w+=1;
        if n%2==0
        {
            n/=2;
        }else
        {
            n=(3*n)+1;
        }
    }

    w
}

fn main()
{
    let n= 567;
    println!("dla {} wynik wynosi {}", n, collatz(n));
}
