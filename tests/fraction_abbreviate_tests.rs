#[cfg(test)]
mod tests {
    use rs_abbreviation_number::NumericAbbreviate;

    #[test]
    fn small_number_test() {
        assert_eq!("10n", (0.000_000_010).abbreviate_number());
        assert_eq!("643n", (0.000_000_643).abbreviate_number());
    }
    #[test]
    fn small_negative_number_test() {
        assert_eq!("-10n", (-0.000_000_010).abbreviate_number());
        assert_eq!("-643n", (-0.000_000_643).abbreviate_number());
    }

    #[test]
    fn deep_number_test() {
        assert_eq!(
            "100q",
            (0.000_000_000_000_000_000_000_000_000_1).abbreviate_number()
        );

        assert_eq!(
            "0",
            0.000_000_000_000_000_000_000_000_000_000_1.abbreviate_number()
        );
    }

    #[test]
    fn deep_negative_number_test() {
        assert_eq!(
            "-100q",
            (-0.000_000_000_000_000_000_000_000_000_1).abbreviate_number()
        );

        assert_eq!(
            "0",
            (-0.000_000_000_000_000_000_000_000_000_000_1).abbreviate_number()
        );
    }
}
