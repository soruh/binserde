extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Serialize)]
pub fn serialize(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;

    let (ne_writes, le_writes, be_writes) = create_serialize_impls(&input);

    let expanded = quote! {
        impl<W> Serialize<W> for #ident
        where W: std::io::Write
        {
            fn serialize_ne(&self, writer: &mut W) -> std::io::Result<()> {
                #ne_writes
                Ok(())
            }
            fn serialize_le(&self, writer: &mut W) -> std::io::Result<()> {
                #le_writes
                Ok(())
            }
            fn serialize_be(&self, writer: &mut W) -> std::io::Result<()> {
                #be_writes
                Ok(())
            }


        }
    };

    // panic!("{}", expanded.to_string());

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

fn create_serialize_impls(
    input: &DeriveInput,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    match &input.data {
        syn::Data::Struct(struct_data) => {
            let fields: Vec<(syn::Ident, syn::Type)> = match &struct_data.fields {
                syn::Fields::Named(fields_named) => fields_named
                    .named
                    .iter()
                    .cloned()
                    .map(|field| (field.ident.as_ref().unwrap().clone(), field.ty.clone()))
                    .collect(),
                syn::Fields::Unnamed(fields_unnamed) => fields_unnamed
                    .unnamed
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(|(i, field)| {
                        (
                            syn::Ident::new(&format!("{}", i), proc_macro2::Span::call_site()),
                            field.ty.clone(),
                        )
                    })
                    .collect(),
                syn::Fields::Unit => unimplemented!(),
            };

            let mut ne_writes = Vec::with_capacity(fields.len());
            let mut le_writes = Vec::with_capacity(fields.len());
            let mut be_writes = Vec::with_capacity(fields.len());

            for field in fields.iter() {
                let (ident, _) = field;

                ne_writes.push(quote! { self.#ident.serialize_ne(writer)?; });
                le_writes.push(quote! { self.#ident.serialize_le(writer)?; });
                be_writes.push(quote! { self.#ident.serialize_be(writer)?; });
            }

            (
                quote! { #(#ne_writes)* },
                quote! { #(#le_writes)* },
                quote! { #(#be_writes)* },
            )
        }
        syn::Data::Enum(_enum_data) => panic!("enums are not yet supported"),
        syn::Data::Union(_union_data) => panic!("unions are not yet supported"),
    }
}
