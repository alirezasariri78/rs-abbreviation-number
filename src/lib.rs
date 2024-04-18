const SYMBOLS: [&str; 7] = ["", "K", "M", "G", "T", "P", "E"];

pub trait NumericAbbreviate {
    fn abbreviate_fnumber(&self) -> String;
}

pub trait NumericUnAbbreviate {
    fn unabbreviate_fnumber(&self) -> f64;
}

impl NumericAbbreviate for f64 {
    fn abbreviate_fnumber(&self) -> String {
        abbreviate_fnumber(*self)
    }
}

impl NumericAbbreviate for f32 {
    fn abbreviate_fnumber(&self) -> String {
        abbreviate_fnumber(*self as f64)
    }
}

impl NumericAbbreviate for i64 {
    fn abbreviate_fnumber(&self) -> String {
        abbreviate_fnumber(*self as f64)
    }
}

impl NumericAbbreviate for i32 {
    fn abbreviate_fnumber(&self) -> String {
        abbreviate_fnumber(*self as f64)
    }
}

impl NumericAbbreviate for i16 {
    fn abbreviate_fnumber(&self) -> String {
        abbreviate_fnumber(*self as f64)
    }
}

impl NumericAbbreviate for i8 {
    fn abbreviate_fnumber(&self) -> String {
        abbreviate_fnumber(*self as f64)
    }
}

impl NumericUnAbbreviate for String {
    fn unabbreviate_fnumber(&self) -> f64 {
        unabbreviate_fnumber(self)
    }
}

impl NumericUnAbbreviate for &str {
    fn unabbreviate_fnumber(&self) -> f64 {
        unabbreviate_fnumber(self)
    }
}

fn abbreviate_fnumber(num: f64) -> String {
    let base: f64 = 1000.0;
    let mut number = num;
    let index_of_symbol = num.abs().log(base).floor() as usize;
    let symbol = *SYMBOLS.get(index_of_symbol).unwrap();
    let pw: f64 = base.powf(index_of_symbol as f64);
    number /= pw;
    format!("{}{}", remove_floating_zero(number), symbol)
}

fn unabbreviate_fnumber(number: &str) -> f64 {
    let trimed_num = number.trim_start_matches('0');
    let chars = trimed_num.chars();
    let last_char = chars.clone().last();
    match last_char {
        Some(symbol) => {
            if symbol.is_numeric() {
                let number_string: String = chars.into_iter().collect();
                return number_string.parse().unwrap_or(0.0);
            }
            let symbol_index = SYMBOLS
                .into_iter()
                .position(|c| c.to_lowercase() == symbol.to_lowercase().to_string())
                .unwrap_or(0);

            let num: String = chars.clone().into_iter().take(chars.count() - 1).collect();
            let parsed_num = num.parse().unwrap_or(0.0);
            let base: f64 = 1000.0;
            let z: f64 = base.powf(symbol_index as f64);
            return parsed_num * z;
        }
        None => return trimed_num.parse::<f64>().unwrap_or(0.0),
    }
}

fn remove_floating_zero(number: f64) -> String {
    let num_str: String = number.to_string();
    let number_chars = num_str.chars();
    let dot_index = number_chars.clone().position(|c| c == '.').unwrap_or(0);
    if dot_index == 0 {
        return num_str;
    }
    let chars: Vec<char> = number_chars.take(dot_index + 3).collect();
    let chars_to_str: String = chars.into_iter().collect();
    chars_to_str
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

mod abbreviate_tests {

    use super::*;
    #[test]
    fn abbreviate_fnumber_less_then_thousand_test() {
        assert_eq!("1", abbreviate_fnumber(1.0));
        assert_eq!("0", abbreviate_fnumber(0.0));
        assert_eq!("123", abbreviate_fnumber(123.0));
    }

    #[test]
    fn abbreviate_fnumber_ceil_test() {
        assert_eq!("1K", abbreviate_fnumber(1_000.0));
        assert_eq!("1M", abbreviate_fnumber(1_000_000.0));
        assert_eq!("16G", abbreviate_fnumber(16_000_000_000.0));
        assert_eq!("1T", abbreviate_fnumber(1_000_000_000_000.0));
        assert_eq!("54P", abbreviate_fnumber(54_000_000_000_000_000.0));
        assert_eq!("123E", abbreviate_fnumber(123_000_000_000_000_000_000.0));
    }

    #[test]
    fn abbreviate_fnumber_randoms_test() {
        assert_eq!("5K", abbreviate_fnumber(5_000.0));
        assert_eq!("12M", abbreviate_fnumber(12_000_430.0));
        assert_eq!("1G", abbreviate_fnumber(1_000_345_000.0));
        assert_eq!("133T", abbreviate_fnumber(133_000_000_055_000.0));
        assert_eq!("166P", abbreviate_fnumber(166_000_643_300_000_000.0));
        assert_eq!("999E", abbreviate_fnumber(999_000_123_123_000_000_123.0));
    }

    #[test]
    fn abbreviate_fnumber_randoms_floating_test() {
        assert_eq!("5.1K", abbreviate_fnumber(5_100.0));
        assert_eq!("5.09K", abbreviate_fnumber(5_090.0));
        assert_eq!("12.01M", abbreviate_fnumber(12_010_430.0));
        assert_eq!("1.1G", abbreviate_fnumber(1_100_345_000.0));
        assert_eq!("133.09T", abbreviate_fnumber(133_090_000_055_000.0));
        assert_eq!("166.05P", abbreviate_fnumber(166_050_643_300_000_000.0));
        assert_eq!("999.5E", abbreviate_fnumber(999_500_123_123_000_000_123.0));
    }

    #[test]
    fn abbreviate_fnumber_negative_test() {
        assert_eq!("-5.1K", abbreviate_fnumber(-5_100.0));
        assert_eq!("-5.09K", abbreviate_fnumber(-5_090.0));
        assert_eq!("-12.01M", abbreviate_fnumber(-12_010_430.0));
        assert_eq!("-1.1G", abbreviate_fnumber(-1_100_345_000.0));
        assert_eq!("-133.09T", abbreviate_fnumber(-133_090_000_055_000.0));
        assert_eq!("-166.05P", abbreviate_fnumber(-166_050_643_300_000_000.0));
        assert_eq!(
            "-999.5E",
            abbreviate_fnumber(-999_500_123_123_000_000_123.0)
        );
    }
}

mod integer_abbreviate_tests {

    use super::*;
    #[test]
    fn abbreviate_inumber_less_then_thousand_test() {
        assert_eq!("1", abbreviate_fnumber(1 as f64));
        assert_eq!("0", abbreviate_fnumber(0 as f64));
        assert_eq!("123", abbreviate_fnumber(123 as f64));

        assert_eq!("1", 1.abbreviate_fnumber());
        assert_eq!("0", 0.abbreviate_fnumber());
        assert_eq!("123", 123.abbreviate_fnumber());
    }

    #[test]
    fn abbreviate_fnumber_ceil_test() {
        assert_eq!("1K", 1_000.abbreviate_fnumber());
        assert_eq!("1M", 1_000_000.abbreviate_fnumber());
    }

    #[test]
    fn abbreviate_fnumber_randoms_test() {
        assert_eq!("5K", abbreviate_fnumber(5_000 as f64));
        assert_eq!("12M", abbreviate_fnumber(12_000_430 as f64));
        assert_eq!("1G", abbreviate_fnumber(1_000_345_000 as f64));
    }

    #[test]
    fn abbreviate_fnumber_randoms_floating_test() {
        assert_eq!("5.1K", abbreviate_fnumber(5_100 as f64));
        assert_eq!("5.09K", abbreviate_fnumber(5_090 as f64));
        assert_eq!("12.01M", abbreviate_fnumber(12_010_430 as f64));
        assert_eq!("1.1G", abbreviate_fnumber(1_100_345_000 as f64));
    }

    #[test]
    fn abbreviate_fnumber_negative_test() {
        assert_eq!("-5.1K", abbreviate_fnumber(-5_100 as f64));
        assert_eq!("-5.09K", abbreviate_fnumber(-5_090 as f64));
        assert_eq!("-12.01M", abbreviate_fnumber(-12_010_430 as f64));
        assert_eq!("-1.1G", abbreviate_fnumber(-1_100_345_000 as f64));
    }
}

mod unabbreviate_tests {

    use super::*;

    #[test]
    fn unabbreviate_fnumber_less_then_thousand_test() {
        assert_eq!(1.0, unabbreviate_fnumber("1.0"));
        assert_eq!(111.02, unabbreviate_fnumber("111.02"));
        assert_eq!(453.1, unabbreviate_fnumber("453.1"));

        assert_eq!(453.1, unabbreviate_fnumber("453.1000"));
        assert_eq!(453.0001, unabbreviate_fnumber("453.0001"));
        assert_eq!(453.0, unabbreviate_fnumber("453.00"));
    }

    #[test]
    fn unabbreviate_fnumber_ceil_test() {
        assert_eq!(1000.0, unabbreviate_fnumber("1K"));
        assert_eq!(22_000_000.0, unabbreviate_fnumber("22M"));
        assert_eq!(33_000_000_000.0, unabbreviate_fnumber("33G"));
        assert_eq!(406_000_000_000_000.0, unabbreviate_fnumber("406T"));
        assert_eq!(45_000_000_000_000_000.0, unabbreviate_fnumber("45P"));
    }

    #[test]
    fn unabbreviate_fnumber_randoms_floating_test() {
        assert_eq!(5_100.0, unabbreviate_fnumber("5.1K"));
        assert_eq!(5_010.0, unabbreviate_fnumber("5.01K"));
        assert_eq!(5_000.0, unabbreviate_fnumber("5.0K"));

        assert_eq!(5_100_000.0, unabbreviate_fnumber("5.1M"));
        assert_eq!(5_010_000.0, unabbreviate_fnumber("5.01M"));
        assert_eq!(5_000_000.0, unabbreviate_fnumber("5.0M"));

        assert_eq!(5_100_000_000.0, unabbreviate_fnumber("5.1G"));
        assert_eq!(5_010_000_000.0, unabbreviate_fnumber("5.01G"));
        assert_eq!(5_000_000_000.0, unabbreviate_fnumber("5.0G"));

        assert_eq!(5_100_000_000_000.0, unabbreviate_fnumber("5.1T"));
        assert_eq!(5_010_000_000_000.0, unabbreviate_fnumber("5.01T"));
        assert_eq!(5_000_000_000_000.0, unabbreviate_fnumber("5.0T"));

        assert_eq!(5_100_000_000_000_000.0, unabbreviate_fnumber("5.1P"));
        assert_eq!(5_010_000_000_000_000.0, unabbreviate_fnumber("5.01P"));
        assert_eq!(5_000_000_000_000_000.0, unabbreviate_fnumber("5.0P"));

        assert_eq!(5_100_000_000_000_000_000.0, unabbreviate_fnumber("5.1E"));
        assert_eq!(5_010_000_000_000_000_000.0, unabbreviate_fnumber("5.01E"));
        assert_eq!(5_000_000_000_000_000_000.0, unabbreviate_fnumber("5.0E"));
    }

    #[test]
    fn unabbreviate_fnumber_negative_test() {
        assert_eq!(
            -999_500_000_000_000_000_000.0,
            unabbreviate_fnumber("-999.5E")
        );
        assert_eq!(-101.0, unabbreviate_fnumber("-101"));
        assert_eq!(-1000.0, unabbreviate_fnumber("-1K"));
        assert_eq!(-1_010_000.0, unabbreviate_fnumber("-1.01M"));

        assert_eq!(-101.0, "-101".unabbreviate_fnumber());
        assert_eq!(-1000.0, "-1k".unabbreviate_fnumber());
        assert_eq!(-1_010_000.0, "-1.01M".unabbreviate_fnumber());
    }

    #[test]
    fn unabbreviate_fnumber_symbol_lower_test() {
        assert_eq!(5_100.0, unabbreviate_fnumber("5.1k"));
        assert_eq!(5_100_000.0, unabbreviate_fnumber("5.1m"));
        assert_eq!(5_100_000_000.0, unabbreviate_fnumber("5.1g"));
        assert_eq!(5_100_000_000_000.0, unabbreviate_fnumber("5.1t"));
        assert_eq!(5_100_000_000_000_000.0, unabbreviate_fnumber("5.1p"));
        assert_eq!(5_000_000_000_000_000_000.0, unabbreviate_fnumber("5.0e"));
    }

    #[test]
    fn unabbreviate_fnumber_exception_test() {
        assert_eq!(1000_000_000_000_000_000.0, unabbreviate_fnumber("001E"));
        assert_eq!(1.0, unabbreviate_fnumber("001x"));
        assert_eq!(1.0, unabbreviate_fnumber("001"));
        assert_eq!(0.0, unabbreviate_fnumber("incalid chars"));
        assert_eq!(1_010.0, unabbreviate_fnumber("001.01K"));
    }
}
