use {
    std::{
        collections::{
            BTreeMap,
            BTreeSet,
            HashMap,
        },
        fs,
    },
    convert_case::{
        Case,
        Casing as _,
    },
    enum_iterator::all,
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
    riirando_common::Savewarp,
    crate::ast::*,
};

mod ast;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)] Io(#[from] std::io::Error),
    #[error(transparent)] Syn(#[from] syn::Error),
    #[error("logic helper {0} is defined multiple times")]
    HelperNameCollision(String),
    #[error("region {0} is defined multiple times")]
    RegionNameCollision(String),
}

impl Error {
    fn into_compile_error(self) -> TokenStream {
        match self {
            Self::Syn(e) => e.into_compile_error(),
            _ => {
                let msg = self.to_string();
                quote! {
                    compile_error!(#msg);
                }
            }
        }
    }
}

fn regions_inner() -> Result<TokenStream, Error> {
    let mut helpers = HashMap::default();
    let mut events = BTreeSet::default();
    let mut regions = BTreeMap::default();
    for res in fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/logic"))? {
        let LogicFile { helpers: new_helpers, regions: new_regions } = syn::parse_str(&fs::read_to_string(res?.path())?)?;
        for (name, helper) in new_helpers {
            if helpers.insert(name.clone(), helper).is_some() {
                return Err(Error::HelperNameCollision(name))
            }
        }
        for (name, region) in new_regions {
            if regions.insert(name.clone(), region).is_some() {
                return Err(Error::RegionNameCollision(name))
            }
        }
    }
    let mut variants = Vec::<Variant>::default();
    let mut info_arms = Vec::<Arm>::default();
    for (name, RegionInfo { savewarp, time_of_day, events: new_events, items, exits }) in &regions {
        let variant_name = name.to_case(Case::Pascal);
        let variant_ident = Ident::new(&variant_name, Span::call_site());
        variants.push(parse_quote!(#variant_ident));
        let savewarp = savewarp.unwrap_or_else(|| Savewarp::Overworld); //TODO automatically assign dungeon savewarps once dungeons are split into individual logic files
        for (name, _ /*TODO collect events in search */) in new_events {
            let variant_name = name.to_case(Case::Pascal);
            let variant_ident = Ident::new(&variant_name, Span::call_site());
            events.insert(variant_ident);
        }
        let items = items.into_iter()
            .map(|(item, accesses)| {
                let accesses = accesses.into_iter()
                    .map(|access| {
                        let access = expand_access_expr(&helpers, &access)?;
                        Ok(quote!((|state, inventory| #access) as Access))
                    })
                    .collect::<Result<Vec<_>, Error>>()?;
                Ok(quote!(#item => vec![#(#accesses),*]))
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let exits = exits.into_iter()
            .filter_map(|(target_region, access)| {
                if !regions.contains_key(target_region) {
                    eprintln!("no region named {target_region}"); //DEBUG skip unknown regions for better follow-up errors
                    return None
                }
                let target_variant = target_region.to_case(Case::Pascal);
                let target_ident = Ident::new(&target_variant, Span::call_site());
                let access = match expand_access_expr(&helpers, &access) {
                    Ok(access) => access,
                    Err(e) => return Some(Err(e.into())),
                };
                Some(Ok(quote!(Self::#target_ident => (|state, inventory| #access) as Access)))
            })
            .chain((name == "Root").then(|| all::<Savewarp>().map(|savewarp| {
                let target_variant = savewarp.to_string();
                let target_ident = Ident::new(&target_variant, Span::call_site());
                Ok(quote!(Self::#target_ident => (|state, inventory| state.savewarp == #savewarp) as Access))
            })).into_iter().flatten())
            .collect::<Result<Vec<_>, Error>>()?;
        info_arms.push(parse_quote! {
            Self::#variant_ident => RegionInfo {
                savewarp: #savewarp,
                time_of_day: #time_of_day,
                items: collect![
                    #(#items,)*
                ],
                exits: collect![
                    #(#exits,)*
                ],
            },
        });
    }
    Ok(quote! {
        #[derive(enumset::EnumSetType)]
        pub(crate) enum NamedEvent {
            #(#events,)*
        }

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
            compile_error!("riirando_macros::regions does not take parameters");
        }.into()
    }
    match regions_inner() {
        Ok(output) => output.into(),
        Err(e) => e.into_compile_error().into(),
    }
}
