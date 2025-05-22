struct RandGen {
    seed: i64
}

impl RandGen
{
    fn new(seed: i64) -> Self
    {
        Self{
            seed
        }
    }

    fn gen_range(&mut self, min_rand: i64, max_rand: i64) -> i64
    {
        let a =1664525;
        let c =1013904223;
        let m:i64=(2 as i64).pow(32);
        self.seed =(a* self.seed+c)%m;

        self.seed%(max_rand-min_rand+1)+min_rand

    }
}

fn main() {
    let mut generator1 = RandGen::new(123);
    let a = generator1.gen_range(3, 15);
    let b = generator1.gen_range(3, 15);
    let c = generator1.gen_range(3, 15);

    let mut generator2 = RandGen::new(123);
    let a2 = generator2.gen_range(3, 15);
    let b2 = generator2.gen_range(3, 15);
    let c2 = generator2.gen_range(3, 15);

    println!("{}", a == a2);
    println!("{}", b == b2);
    println!("{}", c == c2);

    println!("{}", a >= 3);
    println!("{}", b >= 3);
    println!("{}", c >= 3);

    println!("{}", a <= 15);
    println!("{}", b <= 15);
    println!("{}", c <= 15);
}