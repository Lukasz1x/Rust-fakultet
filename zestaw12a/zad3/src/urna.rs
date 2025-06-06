use crate::randgen::RandGen;

pub struct Urna<T>
{
    generator: RandGen,
    u :Vec<T>
}

impl<T: Copy> Urna<T>
{
    pub fn new(generator: RandGen) -> Self
    {
        Self {
            generator,
            u: Vec::new()
        }
    }

    pub fn losuj_z_us(&mut self) -> Option<T>
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

    pub fn losuj_bez_us(&mut self) -> Option<T>
    {
        if self.rozmiar() == 0
        {
            return None;
        }
        Some(self.u[self.generator.gen_range(0, self.rozmiar() as i64) as usize])
    }

    pub fn doloz(&mut self, znak: T)
    {
        self.u.push(znak);
    }

    pub fn rozmiar(&self) -> usize
    {
        self.u.len()
    }
}