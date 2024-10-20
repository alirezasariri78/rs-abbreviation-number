const MINUTE_AS_SECOND: u64 = 60;
const HOUR_AS_SECOND: u64 = MINUTE_AS_SECOND * 60;
const DAY_AS_SECOND: u64 = HOUR_AS_SECOND * HOUR_IN_DAY;
const MONTH_AS_SECOND: u64 = DAY_AS_SECOND * DAYS_IN_MONTH;
const YEAR_AS_SECOND: u64 = DAYS_IN_YEAR * DAY_AS_SECOND;

const HOUR_IN_DAY: u64 = 24;
const MONTH_IN_YEAR: u64 = 12;
const DAYS_IN_MONTH: u64 = 30;
const DAYS_IN_YEAR: u64 = 365;

pub use time_macro::*;
use time_macro_derive::*;

#[derive(TimeCalculator)]
pub struct Second(u64);

#[derive(TimeCalculator)]
pub struct Minute(u64);

#[derive(TimeCalculator)]
pub struct Hour(u64);

#[derive(TimeCalculator)]
pub struct Day(u64);

#[derive(TimeCalculator)]
pub struct Month(u64);

#[derive(TimeCalculator)]
pub struct Year(u64);

pub struct Time;

impl Time {
    ///  An Extension For Time.
    /// # Example
    /// ```
    /// use rs_abbreviation_number::time::*;
    /// let seconds=Time::from_second(34_536_000);
    /// assert_eq!(1,seconds.get_year());
    /// assert_eq!(12,seconds.get_month());
    /// ```

    pub fn from_second(sec: u64) -> Second {
        Second(sec)
    }

    ///  An Extension For Time.
    /// # Example
    /// ```
    /// use rs_abbreviation_number::time::*;
    /// let minuts=Time::from_minute(575_600);
    /// assert_eq!(1,minuts.get_year());
    /// assert_eq!(12,minuts.get_month());
    /// ```
    pub fn from_minute(minute: u64) -> Minute {
        Minute(minute * MINUTE_AS_SECOND)
    }
    ///  An Extension For Time.
    /// # Example
    /// ```
    /// use rs_abbreviation_number::time::*;
    /// let hours=Time::from_hour(9593);
    /// assert_eq!(1,hours.get_year());
    /// assert_eq!(12,hours.get_month());
    /// assert_eq!(360,hours.get_day());
    /// ```
    pub fn from_hour(hour: u64) -> Hour {
        Hour(hour * HOUR_AS_SECOND)
    }

    ///  An Extension For Time.
    /// # Example
    /// ```
    /// use rs_abbreviation_number::time::*;
    /// let days=Time::from_day(399);
    /// assert_eq!(1,days.get_year());
    /// assert_eq!(12,days.get_month());
    /// assert_eq!(360,days.get_day());
    /// ```
    pub fn from_day(day: u64) -> Day {
        Day(day * DAY_AS_SECOND)
    }

    ///  An Extension For Time.
    /// # Example
    /// ```
    /// use rs_abbreviation_number::time::*;
    /// let month=Time::from_month(15);
    /// assert_eq!(1,month.get_year());
    /// assert_eq!(12,month.get_month());
    /// assert_eq!(360,month.get_day());
    /// ```
    pub fn from_month(month: u64) -> Month {
        Month(month * MONTH_AS_SECOND)
    }
    ///  An Extension For Time.
    /// # Example
    /// ```
    /// use rs_abbreviation_number::time::*;
    /// let years=Time::from_year(8);
    /// assert_eq!(8,years.get_year());
    /// assert_eq!(96,years.get_month());
    /// assert_eq!(2880,years.get_day());
    /// ```
    pub fn from_year(year: u64) -> Year {
        Year(year * YEAR_AS_SECOND)
    }
}
