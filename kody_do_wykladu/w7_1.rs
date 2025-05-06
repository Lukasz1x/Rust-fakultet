#[derive(PartialEq, Debug, Clone, Default)]
struct Punkt3D {
    x:f64,
    y:f64,
    z:f64,
}

#[derive(PartialEq, Debug, Clone, Default)]
struct Punkt3D_2 (f64,f64,f64);

impl Punkt3D_2
{
    fn new(x: f64, y:f64, z:f64) -> Self
    {
        Self(x, y, z)
    }
}

impl Punkt3D
{
    fn new(x: f64, y:f64, z:f64) -> Punkt3D
    {
        Punkt3D {
            x: x,
            y: y,
            z: z,
        }
        //Punkt3D {
        //    x,
        //    y,
        //    z,
        //}
    }

    fn srodek_uw() -> Self   //zamiast Punkt3D mozna pisać Self dużą literą
    {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
        //Self::default()
    }
    fn norma(&self) -> f64
    {
        (self.x*self.x + self.y*self.y +self.z*self.z).sqrt()
    }

}

fn main() {
    let mut p1k = Punkt3D_2(3.5, -12.2, 7.6);
    let mut p1 = Punkt3D {
        x: 3.5,
        y: -12.2,
        z: 7.6,
    };
    p1.z = 3.9;
    println!("{}", p1.x); // 3.5
    let mut p2 = Punkt3D {
        x: 3.5,
        y: 2.1,
        z: 7.6,
    };
    println!("{}", p1 == p2); // false
    println!("{:?}", p1);
    let p3 = Punkt3D::new(2.3, 1.0, -0.1);
    let p4 = Punkt3D::srodek_uw();
    let p5 = p3.clone();
    println!("{:?}", p3);
    println!("{:?}", p4);
    println!("{}", p2.norma());
    println!("{}", Punkt3D::norma(&p2));
    println!("{:?}", p2);
    let v = vec![
        None,
        Some(p1.clone()),
        None,
        Some(p2.clone()),
        Some(p3.clone()),
    ];
    println!("{v:?}");
    for p in &v{
        println!("{:?}", p.clone().unwrap_or(Punkt3D::srodek_uw()));
        println!("{:?}", p.clone().unwrap_or_default());
    }
    let p5 = Punkt3D {
        y: -98.2,
        ..p1
    };
    let mut p5k = p1k.clone();
    p5k.1 = -98.2;
    println!("{:?}", p5);
    println!("{:?}", p1);
    let p6 = Punkt3D {
        y: -98.2,
        ..Punkt3D::default()
    };
    println!("{:?}", p6);
}