// Mopdified from derive-getters crate

//! PrettyDiff internals
use std::convert::TryFrom;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, Error, FieldsNamed, Ident, Result};

use crate::{
    extract::{named_fields, named_struct},
    faultmsg::Problem,
};

pub struct Field {
    name: Ident,
}

impl Field {
    fn from_field(field: &syn::Field) -> Result<Option<Self>> {
        let name: Ident = field
            .ident
            .clone()
            .ok_or(Error::new(Span::call_site(), Problem::UnnamedField))?;

        Ok(Some(Field { name: name.clone() }))
    }

    fn from_fields_named(fields_named: &FieldsNamed) -> Result<Vec<Self>> {
        fields_named
            .named
            .iter()
            .try_fold(Vec::new(), |mut fields, field| {
                if let Some(field) = Field::from_field(field)? {
                    fields.push(field);
                }

                Ok(fields)
            })
    }
}

pub struct NamedStruct<'a> {
    original: &'a DeriveInput,
    name: Ident,
    fields: Vec<Field>,
}

impl<'a> NamedStruct<'a> {
    pub fn emit(&self) -> TokenStream {
        let (impl_generics, struct_generics, where_clause) =
            self.original.generics.split_for_impl();
        let struct_name = &self.name;
        let fields: Vec<TokenStream> = self
            .fields
            .iter()
            .map(|field| {
                let f = field.name.clone();
                quote! {
                    if a.#f != b.#f {
                        res += &format!("\n\tField `{}` differs: ",stringify!(#f));
                        res += &asserteq_pretty::PrettyDiff::pretty_diff(&a.#f,&b.#f)
                    }
                }
            })
            .collect();

        quote!(
            impl #impl_generics asserteq_pretty::PrettyDiff for #struct_name #struct_generics
                #where_clause
            {
                fn pretty_diff(a: &#struct_name #struct_generics, b: &#struct_name #struct_generics) -> String {
                    let mut res = format!("{}:",stringify!(#struct_name));
                    #(#fields);*
                    res
                }
            }
        )
    }
}

impl<'a> TryFrom<&'a DeriveInput> for NamedStruct<'a> {
    type Error = Error;

    fn try_from(node: &'a DeriveInput) -> Result<Self> {
        let struct_data = named_struct(node)?;
        let named_fields = named_fields(struct_data)?;
        let fields = Field::from_fields_named(named_fields)?;

        Ok(NamedStruct {
            original: node,
            name: node.ident.clone(),
            fields,
        })
    }
}
