struct Macierz
{
    w: Vec<Vec<f64>>
}

impl Macierz
{
    fn new(wysokosc: usize, szerokosc:usize, wypelniacz: f64) -> Self
    {
        Self
        {
            w: vec![vec![wypelniacz; szerokosc]; wysokosc]
        }
    }

    fn zerowa(wysokosc: usize, szerokosc:usize) -> Self
    {
        Self::new(wysokosc, szerokosc, 0.0)
    }

    fn jednostkowa(wysokosc: usize) -> Self
    {
        let mut w = vec![vec![0.0; wysokosc]; wysokosc];
        for i in 0..wysokosc
        {
            w[i][i] = 1.0;
        }
        Self {w: w.clone()}
    }

    fn element(&self, indeks_wiersz:usize, indeks_kolumny:usize) -> f64
    {
        self.w[indeks_wiersz][indeks_kolumny]
    }

    fn zmien_element(&mut self, indeks_wiersza:usize, indeks_kolumny:usize, nowa_wartosc:f64)
    {
        self.w[indeks_wiersza][indeks_kolumny] = nowa_wartosc;
    }

    fn suma(macierz1:Macierz, macierz2:Macierz) -> Option<Macierz>
    {
        if macierz1.w.len() != macierz2.w.len() || macierz1.w[0].len() != macierz2.w[0].len()
        {
            return None;
        }
        let mut w = vec![vec![0.0; macierz1.w[0].len()]; macierz1.w.len()];

        for i in 0..macierz1.w.len()
        {
            for j in 0..macierz1.w[0].len()
            {
                w[i][j] = macierz1.w[i][j] + macierz2.w[i][j];
            }
        }
        Some(Self {w: w.clone()})
    }

    fn wyswietl(&self)
    {
        for i in &self.w
        {
            for j in i
            {
                print!("{j} ");
            }
            println!();
        }
    }
}

fn main() {
    let m = Macierz::jednostkowa(5);
    m.wyswietl();

    let m2 = Macierz::suma(Macierz::jednostkowa(5), Macierz::jednostkowa(5));
    m2.unwrap_or(Macierz::zerowa(1,1)).wyswietl();
}
