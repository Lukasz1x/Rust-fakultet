#[derive(Debug, Eq, PartialEq, Clone)]
enum Warunki
{
    Slonce,
    Chmury,
    Deszcz,
    Snieg,
}

#[derive(Debug, Clone)]
struct DanePogodowe
{
    lokalizacja: String,
    dzien_kolejny_obserwacji: u32,
    temperatura: f32,
    warunki_pogodowe: Warunki
}

impl DanePogodowe
{
    fn new(lokalizacja: String, dzien_kolejny_obserwacji: u32, temperatura: f32, warunki_pogodowe: Warunki) -> Self
    {
        Self
        {
            lokalizacja,
            dzien_kolejny_obserwacji,
            temperatura,
            warunki_pogodowe
        }
    }
}

#[derive(Debug)]
struct DziennikPogodowy
{
    dane: Vec::<DanePogodowe>
}

impl DziennikPogodowy
{
    fn new() -> Self
    {
        Self
        {
            dane: Vec::new()
        }
    }

    fn dodaj_wpis(&mut self, lokalizacja: String, temperatura: f32, warunki: Warunki)
    {
        let mut max = 0;
        for d in &self.dane
        {
            if d.lokalizacja == lokalizacja
            {
                if max < d.dzien_kolejny_obserwacji
                {
                    max = d.dzien_kolejny_obserwacji;
                }
            }
        }
        self.dane.push(DanePogodowe::new(lokalizacja, max+1, temperatura, warunki));
    }

    fn srednia_dla_pogody(&self, warunki: Warunki) -> f32
    {
        let mut suma = 0.0;
        let mut ile = 0.0;
        for d in &self.dane
        {
            if d.warunki_pogodowe == warunki
            {
                suma+=d.temperatura;
                ile+=1.0;
            }
        }
        suma/ile
    }

    fn najczestrza_pogoda(&self) -> Warunki
    {
        let mut chmury = 0;
        let mut deszcz = 0;
        let mut slonce = 0;
        let mut snieg = 0;
        for d in &self.dane
        {
            match d.warunki_pogodowe
            {
                Warunki::Chmury => chmury += 1,
                Warunki::Deszcz => deszcz += 1,
                Warunki::Slonce => slonce += 1,
                Warunki::Snieg => snieg += 1
            }
        }

        if chmury > deszcz && chmury > slonce && chmury > snieg
        {
            return Warunki::Chmury;
        }else if deszcz > slonce && deszcz > snieg
        {
            return  Warunki::Deszcz;
        }else if slonce > snieg
        {
            return Warunki::Slonce
        }else
        {
            return Warunki::Snieg
        }
    }

    fn sloneczne_powyzej_30(&self) -> Vec<DanePogodowe>
    {
        let mut sloneczne: Vec<DanePogodowe> = Vec::new();
        for d in &self.dane
        {
            if d.warunki_pogodowe == Warunki::Slonce && d.temperatura > 30.0
            {
                sloneczne.push(d.clone());
            }
        }
        sloneczne
    }
}



fn main() {
    let mut dziennik = DziennikPogodowy::new();
    dziennik.dodaj_wpis("Lublin".to_string(), 14.0, Warunki::Chmury);
    dziennik.dodaj_wpis("Lublin".to_string(), 16.5, Warunki::Deszcz);
    dziennik.dodaj_wpis("Lublin".to_string(), 31.2, Warunki::Slonce);
    dziennik.dodaj_wpis("Lublin".to_string(), -2.0, Warunki::Snieg);

    dziennik.dodaj_wpis("Krakow".to_string(), 30.1, Warunki::Slonce);
    dziennik.dodaj_wpis("Krakow".to_string(), 18.3, Warunki::Chmury);
    dziennik.dodaj_wpis("Krakow".to_string(), 12.9, Warunki::Deszcz);
    dziennik.dodaj_wpis("Krakow".to_string(), -1.5, Warunki::Snieg);

    dziennik.dodaj_wpis("Warszawa".to_string(), 29.8, Warunki::Slonce);
    dziennik.dodaj_wpis("Warszawa".to_string(), 32.5, Warunki::Slonce);
    dziennik.dodaj_wpis("Warszawa".to_string(), 20.0, Warunki::Chmury);
    dziennik.dodaj_wpis("Warszawa".to_string(), 15.0, Warunki::Deszcz);

    dziennik.dodaj_wpis("Gdansk".to_string(), 18.3, Warunki::Chmury);
    dziennik.dodaj_wpis("Gdansk".to_string(), 17.0, Warunki::Chmury);
    dziennik.dodaj_wpis("Gdansk".to_string(), 19.5, Warunki::Deszcz);
    dziennik.dodaj_wpis("Gdansk".to_string(), 8.0, Warunki::Snieg);

    println!("Średnia temperatura dla Słońca: {:.1}", dziennik.srednia_dla_pogody(Warunki::Slonce));
    println!("Najczęstsze warunki: {:?}", dziennik.najczestrza_pogoda());
    println!("Słoneczne dni powyżej 30°C:");
    for wpis in dziennik.sloneczne_powyzej_30() {
        println!("{:?}", wpis);
    }
}
