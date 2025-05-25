#[derive(Debug)]
struct Time
{
    hour: u8,
    min: u8,
    sec: u8
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
    let t1 = Time::from_3(5,43,34);
    println!("{}",t1.to_string());
    let t2 = Time::from_3(17, 36, 25);
    println!("{}", t2.to_string());
    let t3 = Time::from_string("14:05:24", ':');
    println!("{:?}", t3);
}
