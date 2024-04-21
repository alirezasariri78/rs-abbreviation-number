#[cfg(test)]
mod tests {
    use rs_abbreviation_number::*;

    #[test]
    fn small_number_test() {
        assert_eq!(
            "10n",
            (0.000_000_010).abbreviate_number(&Default::default())
        );
        assert_eq!(
            "643n",
            (0.000_000_643).abbreviate_number(&Default::default())
        );
    }
    #[test]
    fn small_negative_number_test() {
        assert_eq!(
            "-10n",
            (-0.000_000_010).abbreviate_number(&Default::default())
        );
        assert_eq!(
            "-643n",
            (-0.000_000_643).abbreviate_number(&Default::default())
        );
    }

    #[test]
    fn deep_number_test() {
        assert_eq!(
            "100q",
            (0.000_000_000_000_000_000_000_000_000_1).abbreviate_number(&Default::default())
        );

        assert_eq!(
            "0",
            0.000_000_000_000_000_000_000_000_000_000_1.abbreviate_number(&Default::default())
        );
    }

    #[test]
    fn seperator_number_test() {
        assert_eq!(
            "100-q",
            (0.000_000_000_000_000_000_000_000_000_1).abbreviate_number(&AbbreviationOptions {
                padding: 0,
                separator: "-".to_string()
            })
        );
    }

    #[test]
    fn deep_negative_number_test() {
        assert_eq!(
            "-100q",
            (-0.000_000_000_000_000_000_000_000_000_1).abbreviate_number(&Default::default())
        );

        assert_eq!(
            "0",
            (-0.000_000_000_000_000_000_000_000_000_000_1).abbreviate_number(&Default::default())
        );
    }

    #[test]
    fn padding_abbr() {
        assert_eq!(
            "-100,   q",
            (-0.000_000_000_000_000_000_000_000_000_1).abbreviate_number(&AbbreviationOptions {
                separator: String::from(","),
                padding: 3
            })
        );
    }

    #[test]
    fn negative_number_seperator_test() {
        assert_eq!(
            "-100 q",
            (-0.000_000_000_000_000_000_000_000_000_1).abbreviate_number(&AbbreviationOptions {
                separator: String::from(" "),
                ..Default::default()
            })
        );

        assert_eq!(
            "-100-q",
            (-0.000_000_000_000_000_000_000_000_000_1).abbreviate_number(&AbbreviationOptions {
                separator: String::from("-"),
                ..Default::default()
            })
        );
    }
}
