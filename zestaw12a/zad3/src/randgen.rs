pub struct RandGen {
    seed: i64
}

impl RandGen
{
    pub fn new(seed: i64) -> Self
    {
        Self{
            seed
        }
    }

    pub fn gen_range(&mut self, min_rand: i64, max_rand: i64) -> i64
    {
        let a =1664525;
        let c =1013904223;
        let m:i64=(2 as i64).pow(32);
        self.seed =(a* self.seed+c)%m;

        self.seed%(max_rand-min_rand+1)+min_rand

    }
}
