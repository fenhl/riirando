use {
    proc_macro2::TokenStream,
    quote::{
        ToTokens,
        quote,
    },
    syn::{
        Ident,
        parse::{
            Parse,
            ParseStream,
        },
    },
};

pub enum TimeOfDayBehavior {
    /// Cannot alter time of day in this region. Used for dungeons as well as helper regions like Root.
    None,
    /// Time does not pass but can be set to noon or midnight using the Sun's Song. This reloads the scene.
    Static,
    /// Time passes normally and can be set to any value simply by waiting.
    Passes,
    /// Special behavior for Ganon's castle grounds which force time of day to Dampé time.
    OutsideGanonsCastle,
}

impl Parse for TimeOfDayBehavior {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(match &*input.parse::<Ident>()?.to_string() {
            "None" => Self::None,
            "Static" => Self::Static,
            "Passes" => Self::Passes,
            "OutsideGanonsCastle" => Self::OutsideGanonsCastle,
            name => return Err(input.error(format!("expected time-of-day behavior, found ident {name}"))),
        })
    }
}

impl ToTokens for TimeOfDayBehavior {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let stream = match self {
            Self::None => quote!(TimeOfDayBehavior::None),
            Self::Static => quote!(TimeOfDayBehavior::Static),
            Self::Passes => quote!(TimeOfDayBehavior::Passes),
            Self::OutsideGanonsCastle => quote!(TimeOfDayBehavior::OutsideGanonsCastle),
        };
        stream.to_tokens(tokens);
    }
}
