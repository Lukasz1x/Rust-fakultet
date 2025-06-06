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

struct Urna<T>
{
    generator: RandGen,
    u :Vec<T>
}

impl<T: Copy> Urna<T>
{
    fn new(generator: RandGen) -> Self
    {
        Self {
            generator,
            u: Vec::new()
        }
    }

    fn losuj_z_us(&mut self) -> Option<T>
    {
        if self.rozmiar() == 0
        {
            return None;
        }
        let i = self.generator.gen_range(0, self.rozmiar() as i64) as usize;
        let z = self.u[i];
        self.u.swap_remove(i);


        Some(z)
    }

    fn losuj_bez_us(&mut self) -> Option<T>
    {
        if self.rozmiar() == 0
        {
            return None;
        }
        Some(self.u[self.generator.gen_range(0, self.rozmiar() as i64) as usize])
    }

    fn doloz(&mut self, znak: T)
    {
        self.u.push(znak);
    }

    fn rozmiar(&self) -> usize
    {
        self.u.len()
    }
}

fn main() {
    let mut urna = Urna::<char>::new(RandGen::new(123));

    let a: Option<char> = urna.losuj_z_us();
    println!("{:?}", a.is_none());
    let a: Option<char> = urna.losuj_bez_us();
    println!("{:?}", a.is_none());


    urna.doloz('a');
    urna.doloz('b');
    urna.doloz('c');
    urna.doloz('d');

    println!("{:?}", urna.rozmiar() == 4);
    let y: Option<char> = urna.losuj_z_us();
    println!("{:?}", y.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    let x: Option<char> = urna.losuj_bez_us();
    println!("{:?}", x.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    println!("{:?}", x != y);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 2);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 1);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 0);
    let z: Option<char> = urna.losuj_z_us();
    println!("{:?}", z.is_none());
    println!("{:?}", urna.rozmiar() == 0);
}