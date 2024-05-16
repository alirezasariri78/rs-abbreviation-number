#[cfg(test)]
mod tests {
    mod zero {
        use rs_abbreviation_number::time::*;
        #[test]
        fn zero_year_test() {
            let year = Time::from_year(0);
            assert_eq!(0, year.get_second());
            assert_eq!(0, year.get_minute());
            assert_eq!(0, year.get_hour());
            assert_eq!(0, year.get_day());
            assert_eq!(0, year.get_month());
            assert_eq!(0, year.get_year());
        }

        #[test]
        fn zero_month_test() {
            let month = Time::from_month(0);
            assert_eq!(0, month.get_second());
            assert_eq!(0, month.get_minute());
            assert_eq!(0, month.get_hour());
            assert_eq!(0, month.get_day());
            assert_eq!(0, month.get_month());
            assert_eq!(0, month.get_year());
        }

        #[test]
        fn zero_day_test() {
            let day = Time::from_day(0);
            assert_eq!(0, day.get_second());
            assert_eq!(0, day.get_minute());
            assert_eq!(0, day.get_hour());
            assert_eq!(0, day.get_day());
            assert_eq!(0, day.get_month());
            assert_eq!(0, day.get_year());
        }

        #[test]
        fn zero_hour_test() {
            let hour = Time::from_hour(0);
            assert_eq!(0, hour.get_second());
            assert_eq!(0, hour.get_minute());
            assert_eq!(0, hour.get_hour());
            assert_eq!(0, hour.get_day());
            assert_eq!(0, hour.get_month());
            assert_eq!(0, hour.get_year());
        }

        #[test]
        fn zero_minute_test() {
            let minute = Time::from_minute(0);
            assert_eq!(0, minute.get_second());
            assert_eq!(0, minute.get_minute());
            assert_eq!(0, minute.get_hour());
            assert_eq!(0, minute.get_day());
            assert_eq!(0, minute.get_month());
            assert_eq!(0, minute.get_year());
        }

        #[test]
        fn zero_second_test() {
            let second = Time::from_second(0);
            assert_eq!(0, second.get_second());
            assert_eq!(0, second.get_minute());
            assert_eq!(0, second.get_hour());
            assert_eq!(0, second.get_day());
            assert_eq!(0, second.get_month());
            assert_eq!(0, second.get_year());
        }
    }
}
