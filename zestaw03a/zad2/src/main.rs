fn swap(x: &mut i32, y: &mut i32, z: &mut i32)
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
    let mut a=50;
    let mut b=40;
    let mut c=30;

    swap(&mut a, &mut b, &mut c);
    dbg!(a);
    dbg!(b);
    dbg!(c);

}