use std::fmt::format;

const SYMBOLS: [&str; 7] = ["", "K", "M", "G", "T", "P", "E"];

fn abbreviate_number(num: f64) -> String {
    let base: f64 = 1000.0;
    let mut number = num;
    let index_of_symbol = num.abs().log(base).floor() as usize;
    let symbol = *SYMBOLS.get(index_of_symbol).unwrap();
    let pw: f64 = base.powf(index_of_symbol as f64);
    number /= pw;
    format!("{}{}", fix_format(number), symbol)
}

// example:
// *1.1
// 1.01
// 1
fn fix_format(number: f64) -> String {
    let num_str: String = number.to_string();
    let number_chars = num_str.chars();
    let dot_index = number_chars.clone().position(|c| c == '.').unwrap_or(0);
    if dot_index == 0 {
        return num_str;
    }
    let chars: Vec<char> = number_chars.take(dot_index + 3).collect();

    let size = chars.len();
    // from right to left
    let first_float = *chars.get(size - 1).unwrap();
    let second_float = *chars.get(size - 2).unwrap();

    if first_float == '0' {
        if second_float == '0' {
            return chars.into_iter().take(size - 3).collect();
        } else {
            return chars.into_iter().take(size - 1).collect();
        }
    }
    chars.into_iter().collect()
}

mod tests {

    use super::*;

    #[test]
    fn abbreviate_number_less_then_thousand_test() {
        assert_eq!("1", abbreviate_number(1.0));
        assert_eq!("0", abbreviate_number(0.0));
        assert_eq!("123", abbreviate_number(123.0));
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
}
