use std::num::ParseIntError;

#[derive(Debug)]
struct BladLiczbaNieparzysta;

#[derive(Debug)]
struct TylkoParzyste(i32);

#[derive(Debug)]
enum MojBlad {
    BladParsowania(ParseIntError),
    BladParzystosci,
}

impl From<BladLiczbaNieparzysta> for MojBlad {
    fn from(e: BladLiczbaNieparzysta) -> MojBlad {
        MojBlad::BladParzystosci
    }
}

impl From<ParseIntError> for MojBlad {
    fn from(e: ParseIntError) -> MojBlad {
        MojBlad::BladParsowania(e)
    }
}

impl TylkoParzyste {
    fn from_i32(n: i32) -> Result<TylkoParzyste, BladLiczbaNieparzysta> {
        if n % 2 == 0 {
            Ok(TylkoParzyste(n))
        } else {
            Err(BladLiczbaNieparzysta)
        }
    }

    fn from_str(x: &str) -> Option<TylkoParzyste> {
        let n: i32 = x.parse().ok()?;
        let wynik = TylkoParzyste::from_i32(n).ok()?;
        Some(wynik)
    }

    fn from_str_res(x: &str) -> Result<TylkoParzyste, MojBlad> {
        let n = x.parse::<i32>();
        match n {
            Ok(_) => {}
            Err(p) => { return Err(MojBlad::BladParsowania(p)); }
        }
        let wynik = TylkoParzyste::from_i32(n.unwrap());
        match wynik {
            Ok(_) => {}
            Err(p) => { return Err(MojBlad::BladParzystosci); }
        }
        Ok(wynik.unwrap())
    }

    fn from_str_res_1(x: &str) -> Result<TylkoParzyste, MojBlad> {
        let n: i32 = x.parse()?;
        let wynik = TylkoParzyste::from_i32(n)?;
        Ok(wynik)
    }
}


fn main() {
    println!("===================");
    println!("{:?}", "".parse::<i32>());
    println!("{:?}", TylkoParzyste::from_str(""));
    println!("{:?}", TylkoParzyste::from_str_res(""));
    println!("{:?}", TylkoParzyste::from_str_res_1(""));
    println!("===================");
    println!("{:?}", "x".parse::<i32>());
    println!("{:?}", TylkoParzyste::from_str("x"));
    println!("{:?}", TylkoParzyste::from_str_res("x"));
    println!("{:?}", TylkoParzyste::from_str_res_1("x"));
    println!("===================");
    println!("{:?}", TylkoParzyste::from_str("33"));
    println!("{:?}", TylkoParzyste::from_str_res("33"));
    println!("{:?}", TylkoParzyste::from_str_res_1("33"));
    println!("===================");
    println!("{:?}", TylkoParzyste::from_str("44"));
    println!("{:?}", TylkoParzyste::from_str_res("44"));
    println!("{:?}", TylkoParzyste::from_str_res_1("44"));
    println!("===================");
}