fn rand(seed: &mut i64, min_rand: i64, max_rand: i64) -> i64
{
    let a =1664525;
    let c =1013904223;
    let m:i64=(2 as i64).pow(32);
    *seed =(a* *seed+c)%m;

    *seed%(max_rand-min_rand+1)+min_rand
}

fn main()
{
    let mut seed=10;
    for i in 0..10
    {
        println!("{}", rand(&mut seed, 0, 100));
    }
}
