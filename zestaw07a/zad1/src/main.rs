#[derive(PartialEq, Debug)]
struct Rgb
{
    red: u8,
    green: u8,
    blue: u8,
}

impl Rgb {
    fn from_3u8(red:u8, green:u8, blue:u8) -> Self
    {
        Self{
            red,
            green,
            blue,
        }
    }

    fn from_3percent(r:f64, g:f64, b:f64) -> Option<Self>
    {
        if r>100.0 || r<0.0 || g>100.0 || g<0.0 || b>100.0 || b<0.0
        {
            return None;
        }
        Some(Self{
            red: (r*255.0/100.0) as u8,
            green: (g*255.0/100.0) as u8,
            blue: (b*255.0/100.0) as u8,
        })
    }

    fn gray(p:f64) -> Option<Self>
    {
        if p>100.0 || p<0.0
        {
            return None
        }
        Some(Self{
            red: (p*255.0/100.0) as u8,
            green: (p*255.0/100.0) as u8,
            blue: (p*255.0/100.0) as u8,
        })
    }

    fn white() -> Self
    {
        Self{
            red:255,
            green:255,
            blue:255,
        }
    }

    fn black() -> Self
    {
        Self{
            red:0,
            green:0,
            blue:0,
        }
    }

    fn invert(&mut self)
    {
        self.red=255-self.red;
        self.green=255-self.green;
        self.blue=255-self.blue;
    }

    fn intensity(&self) -> f64
    {
        //suma wszystkiego / max
        ((self.red as f64) + (self.green as f64) + (self.blue as f64)) / (255.0 * 3.0)
    }

    fn as_rgb_u8tuple(&self) -> (u8, u8, u8)
    {
        (self.red, self.green, self.blue)
    }

    fn as_cmy_u8tuple(&self) -> (u8, u8, u8)
    {
        (255-self.red, 255-self.green, 255-self.blue)
    }
}

fn main() {
    let szary1 = Rgb::from_3u8(127, 127, 127);
    let szary2 = Rgb::from_3percent(50.0, 50.0, 50.0).unwrap();
    let szary3 = Rgb::gray(50.0).unwrap();
    let fiolet = Rgb::from_3u8(100, 35, 120);
    let bialy1 = Rgb::white();
    let bialy2 = Rgb::from_3u8(255, 255, 255);
    let mut czarny1 = Rgb::black();
    let czarny2 = Rgb::from_3u8(0, 0, 0);
    // println!("{szary1:?}, {szary2:?}, {szary3:?}");
    println!("{} {}", szary1 == szary2, szary1 == szary3);
    println!("{} {}", bialy1 == bialy2, czarny1 == czarny2);
    czarny1.invert();
    println!("{}", bialy1 == czarny1);
    println!("{}", fiolet.intensity() == 1.0/3.0);
    println!("{}", fiolet.as_rgb_u8tuple() == (100, 35, 120));
    println!("{}", fiolet.as_cmy_u8tuple() == (155, 220, 135));
}