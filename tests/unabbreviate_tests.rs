use rs_abbreviation_number::*;

#[cfg(test)]
mod unabbreviate_tests {

    use super::*;

    #[test]
    fn unabbreviate_number_less_then_thousand_test() {
        assert_eq!(1.0, "1.0".unabbreviate_number());
        assert_eq!(111.02, "111.02".unabbreviate_number());
        assert_eq!(453.1, "453.1".unabbreviate_number());

        assert_eq!(453.1, "453.1000".unabbreviate_number());
        assert_eq!(453.0001, "453.0001".unabbreviate_number());
        assert_eq!(453.0, "453.00".unabbreviate_number());
    }

    #[test]
    fn unabbreviate_number_ceil_test() {
        assert_eq!(1000.0, "1K".unabbreviate_number());
        assert_eq!(22_000_000.0, "22M".unabbreviate_number());
        assert_eq!(33_000_000_000.0, "33G".unabbreviate_number());
        assert_eq!(406_000_000_000_000.0, "406T".unabbreviate_number());
        assert_eq!(45_000_000_000_000_000.0, "45P".unabbreviate_number());
    }

    #[test]
    fn unabbreviate_number_randoms_floating_test() {
        assert_eq!(5_100.0, "5.1K".unabbreviate_number());
        assert_eq!(5_010.0, "5.01K".unabbreviate_number());
        assert_eq!(5_000.0, "5.0K".unabbreviate_number());

        assert_eq!(5_100_000.0, "5.1M".unabbreviate_number());
        assert_eq!(5_010_000.0, "5.01M".unabbreviate_number());
        assert_eq!(5_000_000.0, "5.0M".unabbreviate_number());

        assert_eq!(5_100_000_000.0, "5.1G".unabbreviate_number());
        assert_eq!(5_010_000_000.0, "5.01G".unabbreviate_number());
        assert_eq!(5_000_000_000.0, "5.0G".unabbreviate_number());

        assert_eq!(5_100_000_000_000.0, "5.1T".unabbreviate_number());
        assert_eq!(5_010_000_000_000.0, "5.01T".unabbreviate_number());
        assert_eq!(5_000_000_000_000.0, "5.0T".unabbreviate_number());

        assert_eq!(5_100_000_000_000_000.0, "5.1P".unabbreviate_number());
        assert_eq!(5_010_000_000_000_000.0, "5.01P".unabbreviate_number());
        assert_eq!(5_000_000_000_000_000.0, "5.0P".unabbreviate_number());

        assert_eq!(5_100_000_000_000_000_000.0, "5.1E".unabbreviate_number());
        assert_eq!(5_010_000_000_000_000_000.0, "5.01E".unabbreviate_number());
        assert_eq!(5_000_000_000_000_000_000.0, "5.0E".unabbreviate_number());
    }

    #[test]
    fn unabbreviate_number_negative_test() {
        assert_eq!(
            -999_500_000_000_000_000_000.0,
            "-999.5E".unabbreviate_number()
        );
        assert_eq!(-101.0, "-101".unabbreviate_number());
        assert_eq!(-1000.0, "-1K".unabbreviate_number());
        assert_eq!(-1_010_000.0, "-1.01M".unabbreviate_number());

        assert_eq!(-101.0, "-101".unabbreviate_number());
        assert_eq!(-1000.0, "-1K".unabbreviate_number());
        assert_eq!(-1_010_000.0, "-1.01M".unabbreviate_number());
    }

    #[test]
    fn unabbreviate_number_exception_test() {
        assert_eq!(1000_000_000_000_000_000.0, "001E".unabbreviate_number());
        assert_eq!(1.0, "001x".unabbreviate_number());
        assert_eq!(1.0, "001".unabbreviate_number());
        assert_eq!(0.0, "incalid chars".unabbreviate_number());
        assert_eq!(1_010.0, "001.01K".unabbreviate_number());
    }

    #[test]
    fn unabbreviate_big_number_test() {
        assert_eq!(
            999_000_000_000_000_000_000_000_000_000_000.0,
            "999Q".unabbreviate_number()
        );
        assert_eq!(
            999_500_000_000_000_000_000_000_000_000_000.0,
            "999.5Q".unabbreviate_number()
        );
    }
}
