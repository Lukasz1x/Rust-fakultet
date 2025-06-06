#[derive(PartialEq, Copy, Clone)]
enum Moneta
{
    Orzel,
    Reszka,
}

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
    println!("Litery");
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

    println!("+++++++++++++++++++++++++++++++++");
    println!("Liczby");

    let mut urna = Urna::<i32>::new(RandGen::new(123));

    let a: Option<i32> = urna.losuj_z_us();
    println!("{:?}", a.is_none());
    let a: Option<i32> = urna.losuj_bez_us();
    println!("{:?}", a.is_none());


    urna.doloz(1);
    urna.doloz(2);
    urna.doloz(3);
    urna.doloz(4);

    println!("{:?}", urna.rozmiar() == 4);
    let y: Option<i32> = urna.losuj_z_us();
    println!("{:?}", y.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    let x: Option<i32> = urna.losuj_bez_us();
    println!("{:?}", x.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    println!("{:?}", x != y);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 2);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 1);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 0);
    let z: Option<i32> = urna.losuj_z_us();
    println!("{:?}", z.is_none());
    println!("{:?}", urna.rozmiar() == 0);

    println!("+++++++++++++++++++++++++++++++++");
    println!("Boole");

    let mut urna = Urna::<bool>::new(RandGen::new(123));

    let a: Option<bool> = urna.losuj_z_us();
    println!("{:?}", a.is_none());
    let a: Option<bool> = urna.losuj_bez_us();
    println!("{:?}", a.is_none());


    urna.doloz(true);
    urna.doloz(false);
    urna.doloz(false);
    urna.doloz(true);

    println!("{:?}", urna.rozmiar() == 4);
    let y: Option<bool> = urna.losuj_z_us();
    println!("{:?}", y.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    let x: Option<bool> = urna.losuj_bez_us();
    println!("{:?}", x.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    println!("{:?}", x != y);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 2);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 1);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 0);
    let z: Option<bool> = urna.losuj_z_us();
    println!("{:?}", z.is_none());
    println!("{:?}", urna.rozmiar() == 0);

    println!("+++++++++++++++++++++++++++++++++");
    println!("Napisy");

    let mut urna = Urna::<&str>::new(RandGen::new(123));

    let a: Option<&str> = urna.losuj_z_us();
    println!("{:?}", a.is_none());
    let a: Option<&str> = urna.losuj_bez_us();
    println!("{:?}", a.is_none());


    urna.doloz(&"Napis1");
    urna.doloz(&"Napis2");
    urna.doloz(&"Napis3");
    urna.doloz(&"Napis4");

    println!("{:?}", urna.rozmiar() == 4);
    let y: Option<&str> = urna.losuj_z_us();
    println!("{:?}", y.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    let x: Option<&str> = urna.losuj_bez_us();
    println!("{:?}", x.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    println!("{:?}", x != y);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 2);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 1);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 0);
    let z: Option<&str> = urna.losuj_z_us();
    println!("{:?}", z.is_none());
    println!("{:?}", urna.rozmiar() == 0);

    println!("+++++++++++++++++++++++++++++++++");
    println!("Monety");

    let mut urna = Urna::<Moneta>::new(RandGen::new(123));

    let a: Option<Moneta> = urna.losuj_z_us();
    println!("{:?}", a.is_none());
    let a: Option<Moneta> = urna.losuj_bez_us();
    println!("{:?}", a.is_none());


    urna.doloz(Moneta::Orzel);
    urna.doloz(Moneta::Reszka);
    urna.doloz(Moneta::Orzel);
    urna.doloz(Moneta::Reszka);

    println!("{:?}", urna.rozmiar() == 4);
    let y: Option<Moneta> = urna.losuj_z_us();
    println!("{:?}", y.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    let x: Option<Moneta> = urna.losuj_bez_us();
    println!("{:?}", x.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    println!("{:?}", x != y);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 2);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 1);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 0);
    let z: Option<Moneta> = urna.losuj_z_us();
    println!("{:?}", z.is_none());
    println!("{:?}", urna.rozmiar() == 0);
}