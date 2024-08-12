extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn small(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = item.to_string();

    let output = format!(
        "#[test]\n\
        #[cfg(any(test_small))]\n\
        {}",
        input
    );

    output.parse().unwrap()
}

#[proc_macro_attribute]
pub fn medium(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = item.to_string();

    let output = format!(
        "#[test]\n\
        #[cfg(any(test_medium))]\n\
        {}",
        input
    );

    output.parse().unwrap()
}

#[proc_macro_attribute]
pub fn large(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = item.to_string();

    let output = format!(
        "#[test]\n\
        #[cfg(any(test_medium))]\n\
        {}",
        input
    );

    output.parse().unwrap()
}
