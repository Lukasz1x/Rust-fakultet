fn swap(x: &mut i32, y: &mut i32)
{
    let pom = *x;
    *x=*y;
    *y=pom;
}

fn main()
{
    let mut a=10;
    let mut b=20;
    swap(&mut a, &mut b);
    dbg!(a);
    dbg!(b);
}