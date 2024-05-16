#[cfg(test)]
mod tests {
    mod short {
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
    }

    mod random {
        use rs_abbreviation_number::time::*;

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
        fn random_month_test() {
            let month = Time::from_month(45);
            assert_eq!(31557600, month.get_second());
            assert_eq!(525_960, month.get_minute());
            assert_eq!(8_766, month.get_hour());
            assert_eq!(365, month.get_day());
            assert_eq!(12, month.get_month());
            assert_eq!(1, month.get_year());

            let month = Time::from_year(89);
            assert_eq!(315576000, month.get_second());
            assert_eq!(5_259_600, month.get_minute());
            assert_eq!(87660, month.get_hour());
            assert_eq!(3652, month.get_day());
            assert_eq!(120, month.get_month());
            assert_eq!(10, month.get_year());
        }

        #[test]
        fn random_day_test() {
            let day = Time::from_day(45);
            assert_eq!(31557600, day.get_second());
            assert_eq!(525_960, day.get_minute());
            assert_eq!(8_766, day.get_hour());
            assert_eq!(365, day.get_day());
            assert_eq!(12, day.get_month());
            assert_eq!(1, day.get_year());

            let day = Time::from_year(89);
            assert_eq!(315576000, day.get_second());
            assert_eq!(5_259_600, day.get_minute());
            assert_eq!(87660, day.get_hour());
            assert_eq!(3652, day.get_day());
            assert_eq!(120, day.get_month());
            assert_eq!(10, day.get_year());
        }

        #[test]
        fn random_hour_test() {
            let hour = Time::from_hour(111);
            assert_eq!(399600, hour.get_second());
            assert_eq!(6660, hour.get_minute());
            assert_eq!(111, hour.get_hour());
            assert_eq!(4, hour.get_day());
            assert_eq!(0, hour.get_month());
            assert_eq!(0, hour.get_year());

            let hour = Time::from_hour(74638951);
            assert_eq!(268_700_223_600, hour.get_second());
            assert_eq!(4_478_337_060, hour.get_minute());
            assert_eq!(74638951, hour.get_hour());
            assert_eq!(3109956, hour.get_day());
            assert_eq!(103665, hour.get_month());
            assert_eq!(8638, hour.get_year());
        }

        #[test]
        fn random_minut_test() {
            let min = Time::from_minute(111);
            assert_eq!(6660, min.get_second());
            assert_eq!(111, min.get_minute());
            assert_eq!(1, min.get_hour());
            assert_eq!(0, min.get_day());
            assert_eq!(0, min.get_month());
            assert_eq!(0, min.get_year());

            let min = Time::from_minute(46897);
            assert_eq!(2813820, min.get_second());
            assert_eq!(46897, min.get_minute());
            assert_eq!(781, min.get_hour());
            assert_eq!(32, min.get_day());
            assert_eq!(1, min.get_month());
            assert_eq!(0, min.get_year());
        }

        #[test]
        fn random_second_test() {
            let sec = Time::from_second(1000_993_342);
            assert_eq!(1000_993_342, sec.get_second());
            assert_eq!(16683222, sec.get_minute());
            assert_eq!(278053, sec.get_hour());
            assert_eq!(11585, sec.get_day());
            assert_eq!(380, sec.get_month());
            assert_eq!(31, sec.get_year());

            let sec = Time::from_second(468);
            assert_eq!(468, sec.get_second());
            assert_eq!(7, sec.get_minute());
            assert_eq!(0, sec.get_hour());
            assert_eq!(0, sec.get_day());
            assert_eq!(0, sec.get_month());
            assert_eq!(0, sec.get_year());
        }
    }

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
