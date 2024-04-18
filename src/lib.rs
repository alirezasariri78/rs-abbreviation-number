mod calculation;
use calculation::*;
pub trait NumericAbbreviate {
    fn abbreviate_number(&self) -> String;
}

pub trait NumericUnAbbreviate {
    fn unabbreviate_number(&self) -> f64;
}

impl NumericAbbreviate for f64 {
    fn abbreviate_number(&self) -> String {
        handle_abbreviation(*self)
    }
}

impl NumericAbbreviate for f32 {
    fn abbreviate_number(&self) -> String {
        handle_abbreviation(*self as f64)
    }
}

impl NumericAbbreviate for i128 {
    fn abbreviate_number(&self) -> String {
        handle_abbreviation(*self as f64)
    }
}

impl NumericAbbreviate for i64 {
    fn abbreviate_number(&self) -> String {
        handle_abbreviation(*self as f64)
    }
}

impl NumericAbbreviate for i32 {
    fn abbreviate_number(&self) -> String {
        handle_abbreviation(*self as f64)
    }
}

impl NumericAbbreviate for i16 {
    fn abbreviate_number(&self) -> String {
        handle_abbreviation(*self as f64)
    }
}

impl NumericAbbreviate for i8 {
    fn abbreviate_number(&self) -> String {
        handle_abbreviation(*self as f64)
    }
}

impl NumericUnAbbreviate for String {
    fn unabbreviate_number(&self) -> f64 {
        unabbreviate_number(self)
    }
}

impl NumericUnAbbreviate for &str {
    fn unabbreviate_number(&self) -> f64 {
        unabbreviate_number(self)
    }
}

fn handle_abbreviation(number: f64) -> String {
    if number.abs() >= 1.0 || number == 0.0 {
        abbreviate_number(number)
    } else {
        abbreviate_fraction_number(number)
    }
}
