#[derive(Eq, PartialEq, Clone, Debug)]
enum Gatunek
{
    Powiesc,
    Romans,
    Horror,
    Komedia,
}

#[derive(Clone, Debug)]
struct Ksiazka
{
    tytul: String,
    autor: String,
    liczba_stron: usize,
    gatunek: Gatunek,
}

impl Ksiazka {
    fn new(tytul: String, autor: String, liczba_stron: usize, gatunek: Gatunek) -> Self
    {
        Self{
            tytul,
            autor,
            liczba_stron,
            gatunek,
        }
    }
}

fn filtruj_po_gatunku(ksiazki: &Vec<Ksiazka>, gatunek: Gatunek) -> Vec<Ksiazka>
{
    let mut k = Vec::new();
    for i in ksiazki
    {
        if (*i).gatunek == gatunek && (*i).liczba_stron >= 300
        {
            k.push((*i).clone());
        }
    }
    k
}


fn main() {
    let mut ksiazki = Vec::new();
    ksiazki.push(Ksiazka::new("Zbrodnia i kara".to_string(), "Fiodor Dostojewski".to_string(), 670, Gatunek::Powiesc));
    ksiazki.push(Ksiazka::new("Duma i uprzedzenie".to_string(), "Jane Austen".to_string(), 432, Gatunek::Romans));
    ksiazki.push(Ksiazka::new("Lśnienie".to_string(), "Stephen King".to_string(), 512, Gatunek::Horror));
    ksiazki.push(Ksiazka::new("Mikołajek".to_string(), "René Goscinny".to_string(), 150, Gatunek::Komedia));
    ksiazki.push(Ksiazka::new("Mały książę".to_string(), "Antoine de Saint-Exupéry".to_string(), 96, Gatunek::Powiesc));
    ksiazki.push(Ksiazka::new("Folwark zwierzęcy".to_string(), "George Orwell".to_string(), 144, Gatunek::Powiesc));
    ksiazki.push(Ksiazka::new("Stary człowiek i morze".to_string(), "Ernest Hemingway".to_string(), 127, Gatunek::Powiesc));
    ksiazki.push(Ksiazka::new("Cierpienia młodego Wertera".to_string(), "Johann Wolfgang von Goethe".to_string(), 224, Gatunek::Romans));
    println!("{:?}", ksiazki);
    println!();
    println!("{:?}", filtruj_po_gatunku(&ksiazki, Gatunek::Powiesc));
}
