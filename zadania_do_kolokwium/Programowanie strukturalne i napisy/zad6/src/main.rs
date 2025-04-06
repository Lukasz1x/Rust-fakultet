fn swap_range(tab: &mut [u32], (a1, a2):(usize, usize), (b1, b2):(usize, usize))
{
    let x = (a1..a2).zip(b1..b2);
    for i in x
    {
        let pom = tab[i.0];
        tab[i.0] = tab[i.1];
        tab[i.1] = pom;
    }
}

fn main()
{
    let mut v = Vec::new();
    for i in 0..50
    {
        v.push(i);
    }
    println!("{:?}", v);
    swap_range(&mut v, (10, 30), (30, 40));
    println!("{:?}", v);
}
