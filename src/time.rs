const MINUTE_AS_SECOND: u64 = 60;
const HOUR_AS_SECOND: u64 = MINUTE_AS_SECOND * 60;
const DAY_AS_SECOND: u64 = HOUR_AS_SECOND * 24;
const MONTH_AS_SECOND: u64 = 2_629_747;
const YEAR_AS_SECOND: u64 = 31_557_600;

pub struct Second(u64);
pub struct Minute(u64);
pub struct Hour(u64);
pub struct Day(u64);
pub struct Month(u64);
pub struct Year(u64);
pub struct Time;

pub trait TimeCalculator {
    fn get_year(&self) -> u64;
    fn get_month(&self) -> u64;
    fn get_day(&self) -> u64;
    fn get_hour(&self) -> u64;
    fn get_minute(&self) -> u64;
    fn get_second(&self) -> u64;
}

impl TimeCalculator for Second {
    fn get_year(&self) -> u64 {
        &self.0 / YEAR_AS_SECOND
    }
    fn get_month(&self) -> u64 {
        &self.0 / MONTH_AS_SECOND
    }
    fn get_day(&self) -> u64 {
        &self.0 / DAY_AS_SECOND
    }
    fn get_hour(&self) -> u64 {
        &self.0 / HOUR_AS_SECOND
    }
    fn get_minute(&self) -> u64 {
        &self.0 / MINUTE_AS_SECOND
    }
    fn get_second(&self) -> u64 {
        self.0
    }
}

impl TimeCalculator for Minute {
    fn get_year(&self) -> u64 {
        &self.0 / YEAR_AS_SECOND
    }
    fn get_month(&self) -> u64 {
        &self.0 / MONTH_AS_SECOND
    }
    fn get_day(&self) -> u64 {
        &self.0 / DAY_AS_SECOND
    }
    fn get_hour(&self) -> u64 {
        &self.0 / HOUR_AS_SECOND
    }
    fn get_minute(&self) -> u64 {
        &self.0 / MINUTE_AS_SECOND
    }
    fn get_second(&self) -> u64 {
        self.0
    }
}

impl TimeCalculator for Hour {
    fn get_year(&self) -> u64 {
        &self.0 / YEAR_AS_SECOND
    }
    fn get_month(&self) -> u64 {
        &self.0 / MONTH_AS_SECOND
    }
    fn get_day(&self) -> u64 {
        &self.0 / DAY_AS_SECOND
    }
    fn get_hour(&self) -> u64 {
        &self.0 / HOUR_AS_SECOND
    }
    fn get_minute(&self) -> u64 {
        &self.0 / MINUTE_AS_SECOND
    }
    fn get_second(&self) -> u64 {
        self.0
    }
}

impl TimeCalculator for Day {
    fn get_year(&self) -> u64 {
        &self.0 / YEAR_AS_SECOND
    }
    fn get_month(&self) -> u64 {
        &self.0 / MONTH_AS_SECOND
    }
    fn get_day(&self) -> u64 {
        &self.0 / DAY_AS_SECOND
    }
    fn get_hour(&self) -> u64 {
        &self.0 / HOUR_AS_SECOND
    }
    fn get_minute(&self) -> u64 {
        &self.0 / MINUTE_AS_SECOND
    }
    fn get_second(&self) -> u64 {
        self.0
    }
}

impl TimeCalculator for Month {
    fn get_year(&self) -> u64 {
        &self.0 / YEAR_AS_SECOND
    }
    fn get_month(&self) -> u64 {
        &self.0 / MONTH_AS_SECOND
    }
    fn get_day(&self) -> u64 {
        &self.0 / DAY_AS_SECOND
    }
    fn get_hour(&self) -> u64 {
        &self.0 / HOUR_AS_SECOND
    }
    fn get_minute(&self) -> u64 {
        &self.0 / MINUTE_AS_SECOND
    }
    fn get_second(&self) -> u64 {
        self.0
    }
}

impl TimeCalculator for Year {
    fn get_year(&self) -> u64 {
        self.0 / YEAR_AS_SECOND
    }
    fn get_month(&self) -> u64 {
        const MONTH_COUNT: u64 = 12;
        self.0 / YEAR_AS_SECOND * MONTH_COUNT
    }
    fn get_day(&self) -> u64 {
        &self.0 / DAY_AS_SECOND
    }
    fn get_hour(&self) -> u64 {
        &self.0 / HOUR_AS_SECOND
    }
    fn get_minute(&self) -> u64 {
        &self.0 / MINUTE_AS_SECOND
    }
    fn get_second(&self) -> u64 {
        
        self.0
    }
}

impl Time {
    pub fn from_second(sec: u64) -> Second {
        Second(sec)
    }
    pub fn from_minute(minute: u64) -> Minute {
        Minute(minute * MINUTE_AS_SECOND)
    }

    pub fn from_hour(hour: u64) -> Hour {
        Hour(hour * HOUR_AS_SECOND)
    }

    pub fn from_day(day: u64) -> Day {
        Day(day * DAY_AS_SECOND)
    }

    pub fn from_month(month: u64) -> Month {
        Month(month * MONTH_AS_SECOND)
    }

    pub fn from_year(year: u64) -> Year {
        Year(year * YEAR_AS_SECOND)
    }
}
