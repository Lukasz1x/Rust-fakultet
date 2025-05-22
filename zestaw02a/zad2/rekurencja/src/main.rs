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
    let x0prim:f64 = x0-(f(x0)/fp(x0));
    if n==0 || ((x0prim - x0).abs() < eps)
    {
        return x0
    }else
    {
        return met_newt(x0prim, eps, n-1);
    }
}

fn main() {
    println!("{}", met_newt(-100.0, 0.01, 200));
}
