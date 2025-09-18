extern crate proc_macro;

use proc_macro2::{TokenStream, Span};
use quote::quote;
use syn::{Data, DeriveInput};
use itertools::Itertools;

#[proc_macro_derive(Channels)]
pub fn channels(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let toks = channels_inner(&ast)
        .unwrap_or_else(|err| err.to_compile_error());
    toks.into()
}

fn channels_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match &ast.data {
        Data::Struct(v) => &v.fields,
        _ => return Err(non_struct_error()),
    };

    let fields = fields.iter().map(|f| (
        f.clone().ident.expect("struct fields have names"), 
        f.ty.clone(),
    ));
    let channel_impls = fields.clone().map(|(field, ty)| {
        quote! {
            impl #impl_generics ::components::__private::ChannelGet<#ty> for #name #ty_generics #where_clause {
                fn get(&self) -> #ty {
                    self.#field
                }
            }
            impl #impl_generics ::components::__private::ChannelSet<#ty> for #name #ty_generics #where_clause {
                fn set(&self, value: #ty) -> Self {
                    let mut clone = *self;
                    clone.#field = value;
                    clone
                }
            }
        }
    });
    Ok(quote! {
        #(#channel_impls)*
    })
}

fn non_struct_error() -> syn::Error {
    syn::Error::new(Span::call_site(), "This macro only supports structs.")
}
