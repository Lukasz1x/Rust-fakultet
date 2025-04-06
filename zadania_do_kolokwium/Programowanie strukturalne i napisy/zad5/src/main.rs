fn sort(x: &mut u32, y: &mut u32, z: &mut u32)
{
    if *z < *y
    {
        let pom = *z;
        *z=*y;
        *y=pom;
    }
    if *z < *x
    {
        let pom = *z;
        *z=*x;
        *x=pom;
    }
    if *y < *x
    {
        let pom = *y;
        *y=*x;
        *x=pom;
    }
}
fn main()
{
    let mut a = 5;
    let mut b = 1;
    let mut c =13;

    sort(&mut a, &mut b, &mut c);

    println!("{a} {b} {c}");
}
