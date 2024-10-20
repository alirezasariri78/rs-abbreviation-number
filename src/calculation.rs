use crate::AbbreviationOptions;

const SYMBOLS: [&str; 21] = [
    "q", "r", "y", "z", "a", "f", "p", "n", "μ", "m", "", "K", "M", "G", "T", "P", "E", "Z", "Y",
    "R", "Q",
];
const FRACTION_COUNT: usize = 10;

#[inline]
pub fn abbreviate_number(num: f64, options: &AbbreviationOptions) -> String {
    let base: f64 = 1000.0;
    let mut number = num;
    let max_legal_index = SYMBOLS.len() - 1;
    let mut index_of_symbol = num.abs().log(base).floor() as usize + FRACTION_COUNT;
    if index_of_symbol > max_legal_index {
        index_of_symbol = max_legal_index
    }
    let symbol = *SYMBOLS.get(index_of_symbol).unwrap();
    let pw: f64 = base.powf((index_of_symbol - FRACTION_COUNT) as f64);
    number /= pw;
    get_in_abbr_format(remove_floating_zero(number), options, symbol)
}

pub fn unabbreviate_number(number: &str) -> f64 {
    let chars = number.chars();
    let last_char = chars.clone().last();
    match last_char {
        Some(symbol) => {
            if symbol.is_numeric() {
                let number_string: String = chars.into_iter().collect();
                return number_string.parse().unwrap_or(0.0);
            }
            let symbol_index = SYMBOLS
                .into_iter()
                .position(|c| c == symbol.to_string())
                .unwrap_or(FRACTION_COUNT);

            let num: String = chars.clone().take(chars.count() - 1).collect();
            let parsed_num = num.parse().unwrap_or(0.0);
            let base: f64 = 1000.0;
            match symbol_index {
                0..=FRACTION_COUNT => {
                    let z: f64 = base.powf((FRACTION_COUNT - symbol_index) as f64);
                    parsed_num / z
                }
                _ => {
                    let z: f64 = base.powf((symbol_index - FRACTION_COUNT) as f64);
                    parsed_num * z
                }
            }
        }
        None => number.parse::<f64>().unwrap_or(0.0),
    }
}

#[inline]
pub fn abbreviate_fraction_number(num: f64, options: &AbbreviationOptions) -> String {
    let mut number = num.abs();
    let mut ten_th_counter = 0;
    const THOUSAND: f64 = 1000.0;
    while number < 1.0 {
        number *= 10.0;
        ten_th_counter += 1;
    }

    let thousand_raise: f64 = (ten_th_counter as f64 / 3.0).ceil();
    let symbol_index = FRACTION_COUNT as f64 - thousand_raise;
    if symbol_index < 0.0 {
        return String::from("0");
    }
    let symbol = *SYMBOLS.get(symbol_index as usize).unwrap_or(&"");
    let result = (num * THOUSAND.powf(thousand_raise)) as i128;
    get_in_abbr_format(result.to_string(), options, symbol)
}

fn get_in_abbr_format(number: String, options: &AbbreviationOptions, symbol: &str) -> String {
    format!(
        "{}{}{}{}",
        number,
        options.separator,
        " ".repeat(options.padding),
        symbol
    )
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
