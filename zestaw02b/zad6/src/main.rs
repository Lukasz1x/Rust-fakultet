fn pow_mod(x: u128, n: u128, p: u128) -> u128
{
    if n == 1
    {
        return x%p;
    }
    if n%2 == 0
    {
        let a :u128 =  pow_mod(x, n/2, p);
        return (a*a)%p;
    }else
    {
        return (pow_mod(x, 1, p)*pow_mod(x, n-1, p))%p;
    }
}

fn main() {
    println!("{}", pow_mod(4564,544, 54));
}
