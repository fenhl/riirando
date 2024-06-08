use {
    std::fs,
    convert_case::{
        Case,
        Casing,
    },
    proc_macro2::{
        Span,
        TokenStream,
    },
    quote::quote,
    syn::{
        Arm,
        Ident,
        Variant,
        parse_quote,
    },
    crate::ast::*,
};

mod ast;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)] Io(#[from] std::io::Error),
    #[error(transparent)] Syn(#[from] syn::Error),
}

impl Error {
    fn into_compile_error(self) -> TokenStream {
        match self {
            Self::Io(e) => {
                let msg = e.to_string();
                quote! {
                    compile_error!(#msg);
                }
            }
            Self::Syn(e) => e.into_compile_error(),
        }
    }
}

fn regions_inner() -> Result<TokenStream, Error> {
    let mut variants = Vec::<Variant>::default();
    let mut info_arms = Vec::<Arm>::default();
    for res in fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/logic"))? {
        let LogicFile { regions } = syn::parse_str(&fs::read_to_string(res?.path())?)?;
        for (name, RegionInfo { time_of_day, exits }) in regions {
            let variant_name = name.to_case(Case::Pascal);
            let variant_ident = Ident::new(&variant_name, Span::call_site());
            variants.push(parse_quote!(#variant_ident));
            let exits = exits.into_iter()
                .map(|(target_region, Access(access))| {
                    let target_variant = target_region.to_case(Case::Pascal);
                    let target_ident = Ident::new(&target_variant, Span::call_site());
                    quote!(Self::#target_ident => (|state| #access) as Access)
                });
            info_arms.push(parse_quote! {
                Self::#variant_ident => RegionInfo {
                    time_of_day: #time_of_day,
                    exits: collect![
                        #(#exits,)*
                    ],
                },
            });
        }
    }
    Ok(quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub(crate) enum Region {
            #(#variants,)*
        }

        impl Region {
            pub(crate) fn info(&self) -> RegionInfo {
                match self {
                    #(#info_arms)*
                }
            }
        }
    })
}

#[proc_macro]
pub fn regions(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if !input.is_empty() {
        return quote! {
            compile_error!("riirando_macros::impl_region does not take parameters");
        }.into()
    }
    match regions_inner() {
        Ok(output) => output.into(),
        Err(e) => e.into_compile_error().into(),
    }
}
