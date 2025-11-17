use crate::obfuscator::Obfuscator;
use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::{parse_macro_input, Item};

mod obfuscator;

#[proc_macro_attribute]
pub fn obfnstr(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_item = parse_macro_input!(item as Item);
    let mut visitor = Obfuscator;

    match &mut input_item {
        Item::Fn(item_fn) => {
            visitor.visit_item_fn_mut(item_fn);
        }
        Item::Impl(item_impl) => {
            visitor.visit_item_impl_mut(item_impl);
        }
        _ => {
            let err_span = input_item.span();
            return syn::Error::new(
                err_span,
                "this attribute can only be used on functions or impl blocks",
            )
            .to_compile_error()
            .into();
        }
    }

    TokenStream::from(quote! {
        #input_item
    })
}
