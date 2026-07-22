use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

#[proc_macro]
pub fn time_this(input: TokenStream) -> TokenStream {
    let timing_code = "let start = std::time::Instant::now();";
    let elapsed_code = "dbg!(start.elapsed());";
    let timing_tokens = timing_code.parse::<TokenStream>().unwrap();
    let elapsed_tokens = elapsed_code.parse::<TokenStream>().unwrap();
    timing_tokens.into_iter().chain(input).chain(elapsed_tokens).collect()
}

#[proc_macro]
pub fn time_this_too(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);

    quote! {
        let runtime = std::time::Instant::now();
        #input
        dbg!(runtime.elapsed());
    }.into()
}

#[proc_macro]
pub fn to_do(input: TokenStream) -> TokenStream {
    let start = "";
    let criticality = "dbg!(criticality);";
    let message = "dbg!(message);";
    let start_tokens = start.parse::<TokenStream>().unwrap();
    let criticality_tokens = criticality.parse::<TokenStream>().unwrap();
    let message_tokens = message.parse::<TokenStream>().unwrap();
    start_tokens.into_iter().chain(input).chain(criticality_tokens).chain(message_tokens).collect()
}
