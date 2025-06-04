struct Gracz {
    imie: String,
}

struct Gra {
    nazwa: String,
}

struct Ranking<'a> {
    gracz: &'a Gracz,
    gra: &'a Gra,
    punkty: i32,
}

fn f1() -> (Vec<Gracz>, Vec<Gra>) {
    let gracze = vec![
        Gracz {
            imie: "Edek".to_string(),
        },
        Gracz {
            imie: "Felek".to_string(),
        },
    ];
    let gry = vec![
        Gra {
            nazwa: "Kółko i krzyżyk".to_string(),
        },
        Gra {
            nazwa: "Szachy".to_string(),
        },
    ];
    (gracze, gry)
}

fn f2<'a>(gracze: &'a Vec<Gracz>, gry: &'a Vec<Gra>) -> Vec<Ranking<'a>> {
    let wyniki = vec![
        Ranking {
            gracz: &gracze[0],
            gra: &gry[0],
            punkty: 7,
        },
        Ranking {
            gracz: &gracze[1],
            gra: &gry[0],
            punkty: 17,
        },
    ];
    wyniki
}

fn main() {
    let (gracze, gry) = f1();
    let wyniki = f2(&gracze, &gry);
}