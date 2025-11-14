use crate::obfuscator::Obfuscator;
use proc_macro::TokenStream;
use quote::quote;
use syn::visit_mut::VisitMut;
use syn::{parse_macro_input, ItemFn};

mod obfuscator;

#[proc_macro_attribute]
pub fn obfnstr(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_fn = parse_macro_input!(item as ItemFn);
    let mut visitor = Obfuscator;
    visitor.visit_item_fn_mut(&mut item_fn);
    TokenStream::from(quote! {
        #item_fn
    })
}
