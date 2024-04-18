use rs_abbreviation_number::*;

#[cfg(test)]
mod abbreviate_tests {

    use super::*;
    #[test]
    fn abbreviate_number_less_then_thousand_test() {
        assert_eq!("1", 1.abbreviate_number());
        assert_eq!("0", 0.abbreviate_number());
        assert_eq!("123", 123.0.abbreviate_number());
    }

    #[test]
    fn abbreviate_number_ceil_test() {
        assert_eq!("1K", abbreviate_number(1_000.0));
        assert_eq!("1M", abbreviate_number(1_000_000.0));
        assert_eq!("16G", abbreviate_number(16_000_000_000.0));
        assert_eq!("1T", abbreviate_number(1_000_000_000_000.0));
        assert_eq!("54P", abbreviate_number(54_000_000_000_000_000.0));
        assert_eq!("123E", abbreviate_number(123_000_000_000_000_000_000.0));
    }

    #[test]
    fn abbreviate_number_randoms_test() {
        assert_eq!("5K", abbreviate_number(5_000.0));
        assert_eq!("12M", abbreviate_number(12_000_430.0));
        assert_eq!("1G", abbreviate_number(1_000_345_000.0));
        assert_eq!("133T", abbreviate_number(133_000_000_055_000.0));
        assert_eq!("166P", abbreviate_number(166_000_643_300_000_000.0));
        assert_eq!("999E", abbreviate_number(999_000_123_123_000_000_123.0));
    }

    #[test]
    fn abbreviate_number_randoms_floating_test() {
        assert_eq!("5.1K", abbreviate_number(5_100.0));
        assert_eq!("5.09K", abbreviate_number(5_090.0));
        assert_eq!("12.01M", abbreviate_number(12_010_430.0));
        assert_eq!("1.1G", abbreviate_number(1_100_345_000.0));
        assert_eq!("133.09T", abbreviate_number(133_090_000_055_000.0));
        assert_eq!("166.05P", abbreviate_number(166_050_643_300_000_000.0));
        assert_eq!("999.5E", abbreviate_number(999_500_123_123_000_000_123.0));
    }

    #[test]
    fn abbreviate_number_negative_test() {
        assert_eq!("-5.1K", abbreviate_number(-5_100.0));
        assert_eq!("-5.09K", abbreviate_number(-5_090.0));
        assert_eq!("-12.01M", abbreviate_number(-12_010_430.0));
        assert_eq!("-1.1G", abbreviate_number(-1_100_345_000.0));
        assert_eq!("-133.09T", abbreviate_number(-133_090_000_055_000.0));
        assert_eq!("-166.05P", abbreviate_number(-166_050_643_300_000_000.0));
        assert_eq!("-999.5E", abbreviate_number(-999_500_123_123_000_000_123.0));
    }

    #[test]
    fn abbreviate_number_big_negative_number_test() {
        assert_eq!(
            "-999.5Q",
            (-999_509_999_999_999_000_123_123_000_000_123.0).abbreviate_number()
        );

        let big_num: i128 = -999_509_999_999_999_000_123_123_000_000_123;
        assert_eq!("-999.5Q", big_num.abbreviate_number());
    }

    #[test]
    fn abbreviate_number_big_number_test() {
        let big_num: i128 = 999_509_999_999_999_000_123_123_000_000_123;

        assert_eq!("999.5Q", big_num.abbreviate_number());
    }
}

mod integer_abbreviate_tests {

    use super::*;
    #[test]
    fn abbreviate_inumber_less_then_thousand_test() {
        assert_eq!("1", abbreviate_number(1 as f64));
        assert_eq!("0", abbreviate_number(0 as f64));
        assert_eq!("123", abbreviate_number(123 as f64));

        assert_eq!("1", 1.abbreviate_number());
        assert_eq!("0", 0.abbreviate_number());
        assert_eq!("123", 123.abbreviate_number());
    }

    #[test]
    fn abbreviate_number_ceil_test() {
        assert_eq!("1K", 1_000.abbreviate_number());
        assert_eq!("1M", 1_000_000.abbreviate_number());
    }

    #[test]
    fn abbreviate_number_randoms_test() {
        assert_eq!("5K", abbreviate_number(5_000 as f64));
        assert_eq!("12M", abbreviate_number(12_000_430 as f64));
        assert_eq!("1G", abbreviate_number(1_000_345_000 as f64));
    }

    #[test]
    fn abbreviate_number_randoms_floating_test() {
        assert_eq!("5.1K", abbreviate_number(5_100 as f64));
        assert_eq!("5.09K", abbreviate_number(5_090 as f64));
        assert_eq!("12.01M", abbreviate_number(12_010_430 as f64));
        assert_eq!("1.1G", abbreviate_number(1_100_345_000 as f64));
    }

    #[test]
    fn abbreviate_number_negative_test() {
        assert_eq!("-5.1K", abbreviate_number(-5_100 as f64));
        assert_eq!("-5.09K", abbreviate_number(-5_090 as f64));
        assert_eq!("-12.01M", abbreviate_number(-12_010_430 as f64));
        assert_eq!("-1.1G", abbreviate_number(-1_100_345_000 as f64));
    }
}
