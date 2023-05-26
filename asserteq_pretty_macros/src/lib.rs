use std::{str::FromStr, convert::TryFrom};

use proc_macro::{Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, DeriveInput};

extern crate proc_macro;

#[proc_macro]
pub fn impl_tuple(input: TokenStream) -> TokenStream {
    let tokens = input.into_iter().collect::<Vec<_>>();

    // Make sure at least one token is provided.
    if tokens.is_empty() {
        return err(Span::call_site(), "expected integer, found no input");
    }

    // Make sure we don't have too many tokens.
    if tokens.len() > 1 {
        return err(tokens[1].span(), "unexpected second token");
    }

    // Get the number from our token.
    let count = match &tokens[0] {
        TokenTree::Literal(lit) => {
            // Unfortunately, `Literal` doesn't have nice methods right now, so
            // the easiest way for us to get an integer out of it is to convert
            // it into string and parse it again.
            if let Ok(count) = lit.to_string().parse::<usize>() {
                count
            } else {
                let msg = format!("expected unsigned integer, found `{}`", lit);
                return err(lit.span(), msg);
            }
        }
        other => {
            let msg = format!("expected integer literal, found `{}`", other);
            return err(other.span(), msg);
        }
    };
    let types: Vec<_> = (0..count)
        .map(|x| quote::__private::TokenStream::from_str(&format!("T{}", x)).unwrap())
        .collect();
    let nums: Vec<_> = (0..count)
        .map(|x| quote::__private::TokenStream::from_str(&format!("{}", x)).unwrap())
        .collect();
    let fstr = "Differences:".to_owned() + &"{}".repeat(count);
    TokenStream::from(
        quote! { impl <#( #types: PrettyDiff ),*> PrettyDiff for (#( #types ,)*) {
            fn pretty_diff(left: &Self, right: &Self) -> String {
                format!(#fstr,
                    #(
                        if left.#nums != right.#nums {
                            format!("\n\tPosition {}: {}",#nums,PrettyDiff::pretty_diff(&left.#nums,&right.#nums))
                        } else {
                            String::new()
                        }
                    ),*
                )
            }
        }},
    )
}

fn err(span: Span, msg: impl Into<String>) -> TokenStream {
    let msg = msg.into();
    quote_spanned!(span.into()=> {
        compile_error!(#msg);
    })
    .into()
}

#[proc_macro_derive(PrettyDiff)]
pub fn derive_prettydiff(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    prettydiff::NamedStruct::try_from(&ast)
        .map(|ns| ns.emit())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

mod extract;
mod faultmsg;
mod prettydiff;
