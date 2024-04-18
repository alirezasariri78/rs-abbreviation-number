#[cfg(test)]
mod tests {
    use rs_abbreviation_number::NumericUnAbbreviate;

    #[test]
    fn small_number_test() {
        assert_eq!(0.1, ("100m").unabbreviate_number());
    }

    #[test]
    fn small_negative_number_test() {
        assert_eq!(-0.1, ("-100m").unabbreviate_number());
    }
    #[test]
    fn deep_number_test() {
        assert_eq!(0.000_000_000_1, "100p".unabbreviate_number());
        assert_eq!(0.000_000_000_000_1, "100f".unabbreviate_number());
        assert_eq!(0.000_000_000_000_000_1, "100a".unabbreviate_number());
        assert_eq!(0.000_000_000_000_000_000_1, "100z".unabbreviate_number());
        assert_eq!(
            0.000_000_000_000_000_000_000_1,
            "100y".unabbreviate_number()
        );
        assert_eq!(
            0.000_000_000_000_000_000_000_000_1,
            "100r".unabbreviate_number()
        );
        assert_eq!(
            0.000_000_000_000_000_000_000_000_000_1,
            "100q".unabbreviate_number()
        );
    }

    #[test]
    fn deep_negative_number_test() {
        assert_eq!(-0.000_000_000_1, "-100p".unabbreviate_number());
        assert_eq!(-0.000_000_000_000_1, "-100f".unabbreviate_number());
        assert_eq!(-0.000_000_000_000_000_1, "-100a".unabbreviate_number());
        assert_eq!(-0.000_000_000_000_000_000_1, "-100z".unabbreviate_number());
        assert_eq!(
            -0.000_000_000_000_000_000_000_1,
            "-100y".unabbreviate_number()
        );
        assert_eq!(
            -0.000_000_000_000_000_000_000_000_1,
            "-100r".unabbreviate_number()
        );
        assert_eq!(
            0.000_000_000_000_000_000_000_000_000_1,
            "100q".unabbreviate_number()
        );
    }

    #[test]
    fn deep_random_number_test() {
        assert_eq!(0.000_000_000_193, "193p".unabbreviate_number());
        assert_eq!(0.000_000_000_000_102, "102f".unabbreviate_number());
        assert_eq!(0.000_000_000_000_000_100, "100a".unabbreviate_number());
        assert_eq!(0.000_000_000_000_000_000_001, "1z".unabbreviate_number());
        assert_eq!(0.000_000_000_000_000_000_111, "111z".unabbreviate_number());
    }

    #[test]
    fn deep_negative_random_number_test() {
        assert_eq!(-0.000_000_000_193, "-193p".unabbreviate_number());
        assert_eq!(-0.000_000_000_000_102, "-102f".unabbreviate_number());
        assert_eq!(-0.000_000_000_000_000_100, "-100a".unabbreviate_number());
        assert_eq!(-0.000_000_000_000_000_000_001, "-1z".unabbreviate_number());
        assert_eq!(
            -0.000_000_000_000_000_000_111,
            "-111z".unabbreviate_number()
        );
    }
}
