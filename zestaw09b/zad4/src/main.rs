use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
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
    year: u16,
    time: Option<Time>
}

#[derive(Debug, Clone)]
struct Time
{
    hour: u8,
    min: u8,
    sec: u8
}

impl PartialOrd for Date
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date
{
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.day, self.month.to_u8(), self.year).cmp(&(other.day, other.month.to_u8(), other.year))
        {
            Ordering::Equal =>
            match (&self.time, &other.time)
            {
                (Some(t1), Some(t2)) => t1.cmp(t2),
                (None, None) => Ordering::Equal,
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less
            },
            ord => ord
        }
    }
}

impl PartialEq for Date
{
    fn eq(&self, other: &Self) -> bool {
        self.day == other.day && self.month == other.month && self.year == other.year && self.time == other.time
    }
}

impl Eq for Date {}

impl PartialOrd for Time
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Time
{
    fn cmp(&self, other: &Self) -> Ordering {
        (self.hour, self.min, self.sec).cmp(&(other.hour, other.min, other.sec))
    }
}

impl PartialEq for Time
{
    fn eq(&self, other: &Self) -> bool {
        self.hour == other.hour && self.min == other.min && self.sec == other.sec
    }
}

impl Eq for Time {}


impl Date
{
    fn to_string(&self) -> String
    {
        let data = format!("{:02}.{:02}.{}", self.day, self.month.to_u8(), self.year);
        match &self.time {
            Some(t) => format!("{} {}", data, t.to_string()),
            None => data
        }
    }

    fn from_3(day: u8, month: Month, year: u16) -> Date
    {
        Self {
            day,
            month,
            year,
            time: None,
        }
    }

    fn from_3_with_time(day: u8, month: Month, year: u16, time: Time) -> Date
    {
        Self {
            day,
            month,
            year,
            time: Some(time),
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
            time: None
        }
    }

    fn from_string_with_time(date_str: &str, date_delim: char, time_str: &str, time_delim: char) -> Date
    {
        let d: Vec<&str> = date_str.split(date_delim).collect();
        let day = d[0].parse::<u8>().unwrap_or(0);
        let month = d[1].parse::<u8>().unwrap_or(0);
        let year = d[2].parse::<u16>().unwrap_or(0);
        Date {
            day,
            month: Month::from_u8(month).unwrap_or(Month::January),
            year,
            time: Some(Time::from_string(time_str, time_delim))
        }
    }

    fn has_time(&self) -> bool
    {
        return self.time.is_some();
    }
    fn set_time(&mut self, time: Time)
    {
        self.time = Some(time);
    }
    fn clear_time(&mut self)
    {
        self.time = None
    }

}

impl Time
{
    fn to_string(&self) -> String
    {
        format!("{:02}:{:02}:{:02}", self.hour, self.min, self.sec)
    }

    fn from_3(hour: u8, min: u8, sec: u8) -> Time
    {
        Self {
            hour,
            min,
            sec
        }
    }

    fn from_string(string: &str, delim: char) -> Time
    {
        let d: Vec<&str> = string.split(delim).collect();
        let hour = d[0].parse::<u8>().unwrap_or(0);
        let min = d[1].parse::<u8>().unwrap_or(0);
        let sec = d[2].parse::<u8>().unwrap_or(0);
        Time {
            hour,
            min,
            sec,
        }
    }
}

fn main() {
    let mut d = Date::from_string("14.05.2025", '.');
    let t = Time::from_3(12, 30, 45);
    let data = Date::from_3_with_time(14,Month::from_u8(5).unwrap(),2025,t.clone());
    println!("{}", d == data);
    println!("{}", d >= data);
    println!("{}", d <= data);
    println!("{}", d != data);
    d.set_time(t);
    println!("--------------");
    println!("{}", d == data);
    println!("{}", d >= data);
    println!("{}", d <= data);
    println!("{}", d != data);
    let data2 = Date::from_string("15.05.2025", '.');
    println!("--------------");
    println!("{}", d == data2);
    println!("{}", d >= data2);
    println!("{}", d <= data2);
    println!("{}", d != data2);
    println!("--------------");
    println!("{}", data == data2);
    println!("{}", data >= data2);
    println!("{}", data <= data2);
    println!("{}", data != data2);
}
