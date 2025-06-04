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

fn main() {}
