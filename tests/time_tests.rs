#[cfg(test)]
mod tests {
    use rs_abbreviation_number::time::*;

    #[test]
    fn year_test() {
        let year = Time::from_year(1);
        assert_eq!(31557600, year.get_second());
        assert_eq!(525_960, year.get_minute());
        assert_eq!(8_766, year.get_hour());
        assert_eq!(365, year.get_day());
        assert_eq!(12, year.get_month());
        assert_eq!(1, year.get_year());

        let year = Time::from_year(10);
        assert_eq!(315576000, year.get_second());
        assert_eq!(5_259_600, year.get_minute());
        assert_eq!(87660, year.get_hour());
        assert_eq!(3652, year.get_day());
        assert_eq!(120, year.get_month());
        assert_eq!(10, year.get_year());
    }

    #[test]
    fn random_year_test() {
        let year = Time::from_year(7);
        assert_eq!(31557600, year.get_second());
        assert_eq!(525_960, year.get_minute());
        assert_eq!(8_766, year.get_hour());
        assert_eq!(365, year.get_day());
        assert_eq!(12, year.get_month());
        assert_eq!(1, year.get_year());

        let year = Time::from_year(137);
        assert_eq!(315576000, year.get_second());
        assert_eq!(5_259_600, year.get_minute());
        assert_eq!(87660, year.get_hour());
        assert_eq!(3652, year.get_day());
        assert_eq!(120, year.get_month());
        assert_eq!(10, year.get_year());
    }

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
}
