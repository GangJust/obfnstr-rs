use syn::visit_mut::VisitMut;
use syn::{parse_quote, visit_mut, Expr, Lit};

pub struct Obfuscator;

impl VisitMut for Obfuscator {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        match expr {
            // Check if the expression is a string literal that we can obfuscate.
            Expr::Lit(expr_lit) => {
                if let Lit::Str(lit_str) = &expr_lit.lit {
                    let string = lit_str.value();
                    // This assumes `obfnstr::bytes_xor` is in scope and returns an iterator.
                    let obfuscated_bytes = obfnstr::bytes_xor(string.as_bytes());
                    // Replace the string literal with a call to the runtime deobfuscator.
                    *expr = parse_quote! {
                        obfnstr::obfuscate(&[#( #obfuscated_bytes ),*])
                    };
                }
            }

            _ => {}
        }

        // If it's not a string literal, continue visiting its children.
        visit_mut::visit_expr_mut(self, expr);
    }
}
