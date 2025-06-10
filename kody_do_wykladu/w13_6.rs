use std::collections::BTreeMap;

fn main() {
    let mut mapa = BTreeMap::new();
    let inna_mapa = BTreeMap::from([(12, 8.4), (1, -1.2), (10, 8.1)]);

    mapa.insert("kot", 7);
    mapa.insert("kot", 777);
    mapa.insert("rekin", 187);
    mapa.insert("krowa", 17);

    println!("{}", mapa.contains_key("kot"));
    println!("{}", mapa.contains_key("pies"));

    println!("{:?}", mapa.get("pies"));
    println!("{:?}", mapa.get("kot"));

    mapa.remove("kot");
    println!("{:?}", mapa.get("kot"));

    for dana in &mapa {
        println!("{:?}", dana);
    }
    for dana in &mut mapa {
        *dana.1 *= 100;
    }
    for dana in &mapa {
        println!("{:?}", dana);
    }

    {
        let e1 = mapa.entry("krowa");
        println!("{:?}", e1);
        e1.and_modify(|x| *x *= 100);
    }
    {
        let e1 = mapa.entry("krowa");
        println!("{:?}", e1);
        e1.or_insert(1234);
    }
    {
        let e2 = mapa.entry("małpa");
        println!("{:?}", e2);
        e2.and_modify(|x| *x *= 100);
    }
    {
        let e2 = mapa.entry("małpa");
        println!("{:?}", e2);
        e2.or_insert(1234);
    }
    for dana in &mapa {
        println!("{:?}", dana);
    }
    for dana in &inna_mapa {
        println!("{:?}", dana);
    }
}