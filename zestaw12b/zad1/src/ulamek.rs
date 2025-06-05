use std::fmt;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Copy, Clone)]
pub struct Ulamek
{
    licznik: i32,
    mianownik: i32,
}

impl Ulamek
{
    pub fn new(licznik: i32, mianownik: i32) -> Self
    {
        fn nwd(a: i32, b: i32) -> i32 {
            let mut a = a.abs();
            let mut b = b.abs();
            while b != 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            a
        }

        let d = nwd(licznik, mianownik);
        let mut licznik = licznik / d;
        let mut mianownik = mianownik / d;

        if mianownik < 0 {
            licznik = -licznik;
            mianownik = -mianownik;
        }

        Self
        {
            licznik,
            mianownik,
        }
    }

    pub fn as_f64(&self) -> f64
    {
        self.licznik as f64 / self.mianownik as f64
    }

    pub fn licznik(&self) -> i32
    {
        self.licznik
    }

    pub fn mianownik(&self) -> i32
    {
        self.mianownik
    }
}

impl fmt::Display for Ulamek {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.licznik, self.mianownik)
    }
}

impl Add for Ulamek {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Ulamek::new(self.licznik * other.mianownik + other.licznik * self.mianownik, self.mianownik * other.mianownik)
    }
}

impl AddAssign for Ulamek {
    fn add_assign(&mut self, other: Self) {
        *self = Ulamek::new(self.licznik * other.mianownik + other.licznik * self.mianownik, self.mianownik * other.mianownik)
    }
}

impl Sub for Ulamek {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Ulamek::new(self.licznik * other.mianownik - other.licznik * self.mianownik, self.mianownik * other.mianownik)
    }
}

impl SubAssign for Ulamek {
    fn sub_assign(&mut self, other: Self) {
        *self = Ulamek::new(self.licznik * other.mianownik - other.licznik * self.mianownik, self.mianownik * other.mianownik)
    }
}

impl Mul for Ulamek {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(self.licznik * other.licznik, self.mianownik * other.mianownik)
    }
}

impl MulAssign<Ulamek> for Ulamek {
    fn mul_assign(&mut self, other: Self) {
        *self = Self::new(self.licznik * other.licznik, self.mianownik * other.mianownik)
    }
}

impl Div for Ulamek {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        if other.licznik == 0 {
            panic!("Cannot divide by zero-valued `Rational`!");
        }
        Self::new(self.licznik * other.mianownik, self.mianownik * other.licznik)
    }
}

impl DivAssign<Ulamek> for Ulamek {
    fn div_assign(&mut self, other: Self) {
        if other.licznik == 0 {
            panic!("Cannot divide by zero-valued `Rational`!");
        }
        *self = Self::new(self.licznik * other.mianownik, self.mianownik * other.licznik)
    }
}

impl PartialEq for Ulamek {
    fn eq(&self, other: &Self) -> bool {
        self.licznik == other.licznik && self.mianownik == other.mianownik
    }
}
