fn f(x: f64) -> f64
{
    x*x-3.0*x+2.0
}

fn fp(x: f64) -> f64
{
    2.0*x-3.0
}

fn met_newt(mut x0: f64, eps: f64, n: u128) -> f64
{
    let mut x0prim:f64 = x0-(f(x0)/fp(x0));
    let mut i:u128=1;
    while (i < n) && ((x0prim - x0).abs() >= eps)
    {
        i+=1;
        x0=x0prim;
        x0prim = x0-(f(x0)/fp(x0));
    }

    x0
}

fn main() {
    println!("{}", met_newt(100.0, 0.01, 200));
}
