fn main() {
    let rok = 2024;
    let miesiac = 2;
    let przestepny :bool;
    if rok%400==0
    {
        przestepny=true;
    }else if rok%100==0
    {
        przestepny=false;
    }else if rok%4==0
    {
        przestepny=true;
    }else
    {
        przestepny=false;
    }
    if miesiac==2
    {
        if przestepny
        {
            println!("Miesiąc {miesiac} w roku {rok} ma 29 dni");
        }else
        {
            println!("Miesiąc {miesiac} w roku {rok} ma 28 dni");
        }
    } else if (miesiac%2==0 && miesiac<=7) || (miesiac%2==1 && miesiac>7)
    {
        println!("Miesiąc {miesiac} w roku {rok} ma 30 dni");
    }else
    {
        println!("Miesiąc {miesiac} w roku {rok} ma 31 dni");
    }
}