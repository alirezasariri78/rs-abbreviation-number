const SYMBOLS: [&str; 7] = ["", "K", "M", "G", "T", "P", "E"];

fn unabbreviate_fnumber(number: &str) -> f64 {
    let trimed_num = number.trim_start_matches('0');
    println!("{trimed_num}");
    let chars = trimed_num.chars();
    let last_char = chars.clone().last();
    match last_char {
        Some(symbol) => {
            let symbol_index = SYMBOLS
                .into_iter()
                .position(|c| c.to_lowercase() == symbol.to_lowercase().to_string())
                .unwrap_or(0);

            if symbol_index == 0 {
                let number_string: String = chars.into_iter().collect();
                return number_string.parse().unwrap_or(0.0);
            }

            let num: String = chars.clone().into_iter().take(chars.count() - 1).collect();
            let parsed_num = num.parse().unwrap_or(0.0);
            let base: f64 = 1000.0;
            let z: f64 = base.powf(symbol_index as f64);
            return parsed_num * z as f64;
        }
        None => return trimed_num.parse::<f64>().unwrap_or(0.0),
    }
}

fn main() {
    let x = unabbreviate_fnumber("453.1");
    println!("{x}");
}
