#[derive(Debug, Eq, PartialEq)]
enum PaymentMethod
{
    Cash,
    Card,
    Transfer
}

#[derive(Debug)]
struct Transaction
{
    amount: f64,
    method: PaymentMethod
}

impl Transaction
{
    fn new(amount: f64, method: PaymentMethod) -> Self
    {
        Self{
            amount,
            method
        }
    }
}

fn zsumuj_transakcje_danego_typu(platnosci: &Vec<Transaction>, payment_method: PaymentMethod) -> f64
{
    let mut suma = 0.0;
    for p in platnosci
    {
        if (*p).method == payment_method
        {
            suma+= (*p).amount;
        }
    }
    suma
}

fn main() {
    let mut platnosci = Vec::new();
    platnosci.push(Transaction::new(65.87, PaymentMethod::Card));
    platnosci.push(Transaction::new(120.00, PaymentMethod::Cash));
    platnosci.push(Transaction::new(245.50, PaymentMethod::Transfer));
    platnosci.push(Transaction::new(9.99, PaymentMethod::Card));
    platnosci.push(Transaction::new(1000.00, PaymentMethod::Transfer));
    platnosci.push(Transaction::new(15.25, PaymentMethod::Cash));
    platnosci.push(Transaction::new(49.49, PaymentMethod::Card));
    platnosci.push(Transaction::new(500.00, PaymentMethod::Transfer));
    platnosci.push(Transaction::new(200.00, PaymentMethod::Cash));
    platnosci.push(Transaction::new(79.90, PaymentMethod::Card));

    println!("{:?}", platnosci);
    println!("Suma płatności kartą: {}", zsumuj_transakcje_danego_typu(&platnosci, PaymentMethod::Card));
    println!("Suma płatności gotówką: {}", zsumuj_transakcje_danego_typu(&platnosci, PaymentMethod::Cash));
    println!("Suma płatności przelewem: {}", zsumuj_transakcje_danego_typu(&platnosci, PaymentMethod::Transfer));
}
