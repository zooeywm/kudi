use std::ops::Deref;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    Fields, GenericParam, ItemStruct, Token,
    parse::{Parse, ParseStream},
    punctuated::{Pair, Punctuated},
    spanned::Spanned,
};

#[derive(Debug)]
pub struct ItemUnitStruct(ItemStruct);

impl Parse for ItemUnitStruct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item: ItemStruct = input.parse()?;
        if !matches!(item.fields, Fields::Unit) {
            return Err(syn::Error::new_spanned(item, "expected unit struct"));
        }
        Ok(Self(item))
    }
}

impl Deref for ItemUnitStruct {
    type Target = ItemStruct;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct PartialGenerics<'a>(pub &'a Punctuated<GenericParam, Token![,]>);

impl<'a> PartialGenerics<'a> {
    pub fn split_for_impl(&self) -> (PartialImplGenerics<'a>, PartialTypeGenerics<'a>) {
        (PartialImplGenerics(self.0), PartialTypeGenerics(self.0))
    }
}

#[derive(Debug)]
pub struct PartialImplGenerics<'a>(&'a Punctuated<GenericParam, Token![,]>);

impl ToTokens for PartialImplGenerics<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
        if !self.0.empty_or_trailing() {
            Token![,](self.0.span()).to_tokens(tokens);
        }
    }
}

#[derive(Debug)]
pub struct PartialTypeGenerics<'a>(&'a Punctuated<GenericParam, Token![,]>);

impl ToTokens for PartialTypeGenerics<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for (param, comma) in self.0.pairs().map(Pair::into_tuple) {
            match param {
                GenericParam::Lifetime(param) => {
                    param.lifetime.to_tokens(tokens);
                }
                GenericParam::Type(param) => {
                    param.ident.to_tokens(tokens);
                }
                GenericParam::Const(param) => {
                    param.ident.to_tokens(tokens);
                }
            }
            let comma = if let Some(comma) = comma {
                comma
            } else {
                &Token![,](param.span())
            };
            comma.to_tokens(tokens);
        }
    }
}
