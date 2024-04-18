use rs_abbreviation_number::*;

#[cfg(test)]
mod unabbreviate_tests {

    use super::*;

    #[test]
    fn unabbreviate_number_less_then_thousand_test() {
        assert_eq!(1.0, unabbreviate_number("1.0"));
        assert_eq!(111.02, unabbreviate_number("111.02"));
        assert_eq!(453.1, unabbreviate_number("453.1"));

        assert_eq!(453.1, unabbreviate_number("453.1000"));
        assert_eq!(453.0001, unabbreviate_number("453.0001"));
        assert_eq!(453.0, unabbreviate_number("453.00"));
    }

    #[test]
    fn unabbreviate_number_ceil_test() {
        assert_eq!(1000.0, unabbreviate_number("1K"));
        assert_eq!(22_000_000.0, unabbreviate_number("22M"));
        assert_eq!(33_000_000_000.0, unabbreviate_number("33G"));
        assert_eq!(406_000_000_000_000.0, unabbreviate_number("406T"));
        assert_eq!(45_000_000_000_000_000.0, unabbreviate_number("45P"));
    }

    #[test]
    fn unabbreviate_number_randoms_floating_test() {
        assert_eq!(5_100.0, unabbreviate_number("5.1K"));
        assert_eq!(5_010.0, unabbreviate_number("5.01K"));
        assert_eq!(5_000.0, unabbreviate_number("5.0K"));

        assert_eq!(5_100_000.0, unabbreviate_number("5.1M"));
        assert_eq!(5_010_000.0, unabbreviate_number("5.01M"));
        assert_eq!(5_000_000.0, unabbreviate_number("5.0M"));

        assert_eq!(5_100_000_000.0, unabbreviate_number("5.1G"));
        assert_eq!(5_010_000_000.0, unabbreviate_number("5.01G"));
        assert_eq!(5_000_000_000.0, unabbreviate_number("5.0G"));

        assert_eq!(5_100_000_000_000.0, unabbreviate_number("5.1T"));
        assert_eq!(5_010_000_000_000.0, unabbreviate_number("5.01T"));
        assert_eq!(5_000_000_000_000.0, unabbreviate_number("5.0T"));

        assert_eq!(5_100_000_000_000_000.0, unabbreviate_number("5.1P"));
        assert_eq!(5_010_000_000_000_000.0, unabbreviate_number("5.01P"));
        assert_eq!(5_000_000_000_000_000.0, unabbreviate_number("5.0P"));

        assert_eq!(5_100_000_000_000_000_000.0, unabbreviate_number("5.1E"));
        assert_eq!(5_010_000_000_000_000_000.0, unabbreviate_number("5.01E"));
        assert_eq!(5_000_000_000_000_000_000.0, unabbreviate_number("5.0E"));
    }

    #[test]
    fn unabbreviate_number_negative_test() {
        assert_eq!(
            -999_500_000_000_000_000_000.0,
            unabbreviate_number("-999.5E")
        );
        assert_eq!(-101.0, unabbreviate_number("-101"));
        assert_eq!(-1000.0, unabbreviate_number("-1K"));
        assert_eq!(-1_010_000.0, unabbreviate_number("-1.01M"));

        assert_eq!(-101.0, "-101".unabbreviate_number());
        assert_eq!(-1000.0, "-1K".unabbreviate_number());
        assert_eq!(-1_010_000.0, "-1.01M".unabbreviate_number());
    }

    #[test]
    fn unabbreviate_number_exception_test() {
        assert_eq!(1000_000_000_000_000_000.0, unabbreviate_number("001E"));
        assert_eq!(1.0, unabbreviate_number("001x"));
        assert_eq!(1.0, unabbreviate_number("001"));
        assert_eq!(0.0, unabbreviate_number("incalid chars"));
        assert_eq!(1_010.0, unabbreviate_number("001.01K"));
    }

    #[test]
    fn unabbreviate_big_number_test() {
        assert_eq!(
            999_000_000_000_000_000_000_000_000_000_000.0,
            unabbreviate_number("999Q")
        );
        assert_eq!(
            999_500_000_000_000_000_000_000_000_000_000.0,
            unabbreviate_number("999.5Q")
        );
    }
}
