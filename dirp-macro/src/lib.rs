use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{ItemFn, LitBool, LitInt, Meta, Token};

#[proc_macro_attribute]
pub fn dp(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as ItemFn);
    let args = syn::parse_macro_input!(attr as DpArgs);

    let fn_name = &input.sig.ident;
    let fn_name_str = fn_name.to_string();

    let doc_lines: Vec<String> = input
        .attrs
        .iter()
        .filter_map(|attr| {
            if let Meta::NameValue(nv) = &attr.meta {
                if nv.path.is_ident("doc") {
                    if let syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(s),
                        ..
                    }) = &nv.value
                    {
                        return Some(s.value().trim().to_string());
                    }
                }
            }
            None
        })
        .collect();
    let description = doc_lines.join("\n");

    let id = args.id;
    let after = &args.after;
    let lite = args.lite;
    let deprecated = match args.deprecated {
        Some(v) => quote! { Some(#v) },
        None => quote! { None },
    };

    let expanded = quote! {
        #input

        inventory::submit! {
            crate::Predicate {
                id: #id,
                name: #fn_name_str,
                description: #description,
                after: &[#(#after),*],
                lite: #lite,
                deprecated: #deprecated,
                check_fn: #fn_name,
            }
        }
    };

    expanded.into()
}

struct DpArgs {
    id: u32,
    after: Vec<u32>,
    lite: bool,
    deprecated: Option<u32>,
}

impl Parse for DpArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut id: Option<u32> = None;
        let mut after = Vec::new();
        let mut lite = false;
        let mut deprecated: Option<u32> = None;

        let fields = Punctuated::<DpField, Token![,]>::parse_terminated(input)?;
        for field in fields {
            match field {
                DpField::Id(v) => id = Some(v),
                DpField::After(v) => after = v,
                DpField::Lite(v) => lite = v,
                DpField::Deprecated(v) => deprecated = Some(v),
            }
        }

        let id = id.ok_or_else(|| input.error("missing required attribute: id"))?;

        Ok(DpArgs {
            id,
            after,
            lite,
            deprecated,
        })
    }
}

enum DpField {
    Id(u32),
    After(Vec<u32>),
    Lite(bool),
    Deprecated(u32),
}

impl Parse for DpField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;
        input.parse::<Token![=]>()?;

        match ident.to_string().as_str() {
            "id" => {
                let lit: LitInt = input.parse()?;
                Ok(DpField::Id(lit.base10_parse()?))
            }
            "after" => {
                let content;
                syn::bracketed!(content in input);
                let ids = Punctuated::<LitInt, Token![,]>::parse_terminated(&content)?;
                Ok(DpField::After(
                    ids.iter().map(|lit| lit.base10_parse().unwrap()).collect(),
                ))
            }
            "lite" => {
                let lit: LitBool = input.parse()?;
                Ok(DpField::Lite(lit.value))
            }
            "deprecated" => {
                let lit: LitInt = input.parse()?;
                Ok(DpField::Deprecated(lit.base10_parse()?))
            }
            other => Err(syn::Error::new(
                ident.span(),
                format!("unknown dp attribute: {other}"),
            )),
        }
    }
}
