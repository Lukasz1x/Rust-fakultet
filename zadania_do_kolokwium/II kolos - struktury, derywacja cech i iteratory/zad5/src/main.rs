use std::fmt;
use std::ops::{Add, Sub};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Wektor3D {
    x: f64,
    y: f64,
    z: f64,
}

impl fmt::Display for Wektor3D
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add for Wektor3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Wektor3D {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl PartialOrd for Wektor3D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.dlugosc().partial_cmp(&other.dlugosc())
    }
}

impl Wektor3D
{
    fn new(x:f64, y:f64, z:f64) -> Self
    {
        Self{
            x,
            y,
            z
        }
    }

    fn dlugosc(&self) -> f64
    {
        (self.x.powi(2)+self.y.powi(2)+self.z.powi(2)).sqrt()
    }

    fn norma(&self) -> Wektor3D
    {
        Wektor3D
        {
            x: self.x/self.dlugosc(),
            y: self.y/self.dlugosc(),
            z: self.z/self.dlugosc(),
        }
    }

    fn iloczyn_skalarny(&self, other: &Self) -> f64
    {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    fn iloczyn_wektorowy(&self, other: &Self) -> Self
    {
        Self
        {
            x: self.y*other.z - self.z*other.y,
            y: self.z*other.x - self.x*other.z,
            z: self.x*other.y - self.y*other.x,
        }
    }
}

fn posortuj_po_dlugosci(mut wektory: Vec<Wektor3D>) -> Vec<Wektor3D>
{
    wektory.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    wektory
}

fn main() {
    let punkt = Wektor3D::new(1.2, 34.4, 4.4);
    let punkt2 = Wektor3D::new(3.2, 23.4, 2.4);
    println!("Punkt: {}", punkt);
    println!("Punkt+Punkt2: {}", punkt+punkt2);
    println!("Punkt-Punkt2: {}", punkt-punkt2);
    println!("Punkt>Punkt2: {}", punkt>punkt2);
    println!("Norma Punkt1: {}, dlugosc normy: {}", punkt.norma(), punkt.norma().dlugosc());
    println!("Iloczyn skalarny(Punkt, Punkt2): {}", punkt.iloczyn_skalarny(&punkt2));
    println!("Iloczyn wektorowy(Punkt, Punkt2): {}", punkt.iloczyn_wektorowy(&punkt2));

    let punkty = vec![
        Wektor3D::new(-12.4,  7.8,   3.1),
        Wektor3D::new( 15.9, -18.2,  0.0),
        Wektor3D::new( -7.5,  4.2, -19.9),
        Wektor3D::new(  3.3, -2.1,  11.7),
        Wektor3D::new( 19.6,  1.1,  -8.8),
        Wektor3D::new( -5.5, -16.4,  6.2),
        Wektor3D::new(  8.8,  9.9, -14.3),
        Wektor3D::new(-17.0, -3.3,  17.4),
        Wektor3D::new(  0.0, 12.5,  -6.6),
        Wektor3D::new( 13.2, -7.7,  4.4),
    ];

    posortuj_po_dlugosci(punkty).iter().for_each(|x| println!("W3D: {}, dl: {}", x, x.dlugosc()));

}
