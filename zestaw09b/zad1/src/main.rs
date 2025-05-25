#[derive(Debug)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December
}

impl Month
{
    fn from_u8(v :u8) -> Option<Month>
    {
        match v {
            1 => Some(Month::January),
            2 => Some(Month::February),
            3 => Some(Month::March),
            4 => Some(Month::April),
            5 => Some(Month::May),
            6 => Some(Month::June),
            7 => Some(Month::July),
            8 => Some(Month::August),
            9 => Some(Month::September),
            10 => Some(Month::October),
            11 => Some(Month::November),
            12 => Some(Month::December),
            _ => None,
        }
    }
    fn to_u8(&self) -> u8
    {
        match self {
            Month::January => 1,
            Month::February => 2,
            Month::March => 3,
            Month::April => 4,
            Month::May => 5,
            Month::June => 6,
            Month::July => 7,
            Month::August => 8,
            Month::September => 9,
            Month::October => 10,
            Month::November => 11,
            Month::December => 12,
        }
    }
}


#[derive(Debug)]
struct Date
{
    day: u8,
    month: Month,
    year: u16
}

impl Date
{
    fn to_string(&self) -> String
    {
        format!("{:02}.{:02}.{}", self.day, self.month.to_u8(), self.year)
    }

    fn from_3(day: u8, month: Month, year: u16) -> Date
    {
        Self {
            day,
            month,
            year
        }
    }

    fn from_string(string: &str, delim: char) -> Date
    {
        let d: Vec<&str> = string.split(delim).collect();
        let day = d[0].parse::<u8>().unwrap_or(0);
        let month = d[1].parse::<u8>().unwrap_or(0);
        let year = d[2].parse::<u16>().unwrap_or(0);
        Date {
            day,
            month: Month::from_u8(month).unwrap_or(Month::January),
            year,
        }
    }
}

fn main() {
    let date = Date::from_3(5,Month::May,2025);
    println!("{}",date.to_string());
    let date1 = Date::from_3(14, Month::May, 2025);
    println!("{}", date1.to_string());
    let date2 = Date::from_string("14-05-2025", '-');
    println!("{:?}", date2);
}
