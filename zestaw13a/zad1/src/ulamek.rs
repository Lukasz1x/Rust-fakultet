use std::fmt;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Copy, Clone, Debug)]
pub struct Ulamek
{
    licznik: i32,
    mianownik: i32,
}

impl Ulamek
{
    pub fn new(licznik: i32, mianownik: i32) -> Self
    {
        if mianownik == 0
        {
            panic!("Mianownik ma wartość 0!!!");
        }
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

    pub fn from_str(napis: &str) -> Option<Self>
    {
        let a:Vec<&str>= napis.split('/').collect();
        //println!("{a:?}");
        let licznik;
        let mianownik;
        if a.len() == 2
        {
            licznik = a[0].parse::<i32>();
            mianownik = a[1].parse::<i32>();
        }else if a.len() == 1
        {
            licznik = a[0].parse::<i32>();
            mianownik = "1".parse::<i32>();
        }else {
            return None;
        }

        if !(licznik.is_ok() && mianownik.is_ok())
        {
            return None;
        }
        Some(Ulamek::new(licznik.unwrap(), mianownik.unwrap()))
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

impl Default for Ulamek {
    fn default() -> Self
    {
        Ulamek::new(1,1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_f64() {
        let u1 = Ulamek::new(3, 4);
        assert_eq!(u1.as_f64(), 0.75);
    }

    #[test]
    fn test_add() {
        let u1 = Ulamek::new(1, 3);
        let u2 = Ulamek::new(1, 2);
        assert_eq!(u1 + u2, Ulamek::new(5, 6));
    }

    #[test]
    #[should_panic]
    fn test_zerowy_mianownik() {
        let _ = Ulamek::new(1, 0);
    }

    #[test]
    fn test_rozne_zapisy_tego_samego_ulamka() {
        assert_eq!(Ulamek::new(1, -3), Ulamek::new(-2, 6));
    }

    #[test]
    fn test_z_napisu_1() {
        let u1 = Ulamek::from_str("1/-3").unwrap();
        let u2 = Ulamek::from_str("-2/6").unwrap();
        assert_eq!(u1, u2);
        assert_eq!(u1, Ulamek::new(-1, 3));
    }

    #[test]
    fn test_z_napisu_2() {
        let u1 = Ulamek::from_str("13").unwrap();
        let u2 = Ulamek::from_str("-26/-2").unwrap();
        assert_eq!(u1, u2);
        assert_eq!(u1, Ulamek::new(13, 1));
    }

    #[test]
    #[should_panic]
    fn test_z_blednego_napisu_1() {
        let _ = Ulamek::from_str("x/-3").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_z_blednego_napisu_2() {
        let _ = Ulamek::from_str("1/3/5").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_z_blednego_napisu_3() {
        let _ = Ulamek::from_str("/5").unwrap();
    }
}