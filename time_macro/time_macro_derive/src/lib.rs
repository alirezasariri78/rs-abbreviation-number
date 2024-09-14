extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

// Define your macro function with the `#[proc_macro]` attribute
#[proc_macro_derive(TimeCalculator)]
pub fn impl_time_calculator(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl TimeCalculator for #name {
            fn get_year(&self) -> u64 {
                 &self.0 / YEAR_AS_SECOND
            }
            fn get_month(&self) -> u64 {
                &self.get_year() * MONTH_IN_YEAR
            }
            fn get_day(&self) -> u64 {
                 &self.get_month() * DAYS_IN_MONTH
            }
            fn get_hour(&self) -> u64 {
                 &self.get_day() * HOUR_IN_DAY
            }
            fn get_minute(&self) -> u64 {
                 &self.get_hour() * 60
            }
            fn get_second(&self) -> u64 {
                 &self.get_minute() * 60
            }
        }
    };
    gen.into()
}
