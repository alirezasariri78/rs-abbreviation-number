#[cfg(test)]
mod abbreviate_tests {

    use rs_abbreviation_number::*;

    #[test]
    fn mix_floating_test() {
        assert_eq!("1.43K", 1435.549.abbreviate_number());
    }

    #[test]
    fn mix_negative_floating_test() {
        assert_eq!("-1.43K", (-1435.549).abbreviate_number());
    }

    #[test]
    fn abbreviate_number_less_then_thousand_test() {
        assert_eq!("1", 1.abbreviate_number());
        assert_eq!("0", 0.abbreviate_number());
        assert_eq!("123", 123.0.abbreviate_number());
    }

    #[test]
    fn abbreviate_number_ceil_test() {
        assert_eq!("1K", 1_000.0.abbreviate_number());
        assert_eq!("1M", 1_000_000.0.abbreviate_number());
        assert_eq!("16G", 16_000_000_000.0.abbreviate_number());
        assert_eq!("1T", (1_000_000_000_000.0).abbreviate_number());
        assert_eq!("54P", (54_000_000_000_000_000.0).abbreviate_number());
        assert_eq!("123E", 123_000_000_000_000_000_000.0.abbreviate_number());
    }

    #[test]
    fn abbreviate_number_randoms_test() {
        assert_eq!("5K", 5_000.0.abbreviate_number());
        assert_eq!("12M", 12_000_430.0.abbreviate_number());
        assert_eq!("1G", 1_000_345_000.0.abbreviate_number());
        assert_eq!("133T", 133_000_000_055_000.0.abbreviate_number());
        assert_eq!("166P", 166_000_643_300_000_000.0.abbreviate_number());
        assert_eq!("999E", 999_000_123_123_000_000_123.0.abbreviate_number());
    }

    #[test]
    fn abbreviate_number_randoms_floating_test() {
        assert_eq!("5.1K", 5_100.0.abbreviate_number());
        assert_eq!("5.09K", 5_090.0.abbreviate_number());
        assert_eq!("12.01M", 12_010_430.0.abbreviate_number());
        assert_eq!("1.1G", 1_100_345_000.0.abbreviate_number());
        assert_eq!("133.09T", 133_090_000_055_000.0.abbreviate_number());
        assert_eq!("166.05P", 166_050_643_300_000_000.0.abbreviate_number());
        assert_eq!("999.5E", 999_500_123_123_000_000_123.0.abbreviate_number());
    }

    #[test]
    fn abbreviate_number_negative_test() {
        assert_eq!("-5.1K", (-5_100.0).abbreviate_number());
        assert_eq!("-5.09K", (-5_090.0).abbreviate_number());
        assert_eq!("-12.01M", (-12_010_430.0).abbreviate_number());
        assert_eq!("-1.1G", (-1_100_345_000.0).abbreviate_number());
        assert_eq!("-133.09T", (-133_090_000_055_000.0).abbreviate_number());
        assert_eq!("-166.05P", (-166_050_643_300_000_000.0).abbreviate_number());
        assert_eq!(
            "-999.5E",
            (-999_500_123_123_000_000_123.0).abbreviate_number()
        );
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

    use rs_abbreviation_number::*;
    #[test]
    fn abbreviate_inumber_less_then_thousand_test() {
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
        assert_eq!("5K", 5_000.abbreviate_number());
        assert_eq!("12M", 12_000_430.abbreviate_number());
        assert_eq!("1G", 1_000_345_000.abbreviate_number());
    }

    #[test]
    fn abbreviate_number_negative_test() {
        assert_eq!("-5.1K", (-5_100).abbreviate_number());
        assert_eq!("-5.09K", (-5_090).abbreviate_number());
        assert_eq!("-12.01M", (-12_010_430).abbreviate_number());
        assert_eq!("-1.1G", (-1_100_345_000).abbreviate_number());
    }
}
