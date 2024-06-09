use {
    convert_case::{
        Case,
        Casing as _,
    },
    derive_more::Display,
    enum_iterator::{
        Sequence,
        all,
    },
    enumset::EnumSetType,
    proc_macro2::{
        Span,
        TokenStream,
    },
    quote::{
        ToTokens,
        quote,
    },
    syn::{
        Ident,
        LitStr,
        parse::{
            Parse,
            ParseStream,
        },
    },
};

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Sequence)]
pub enum Savewarp {
    Overworld,
    DekuTree,
    DodongosCavern,
    JabuJabusBelly,
    ForestTemple,
    FireTemple,
    WaterTemple,
    ShadowTemple,
    SpiritTemple,
    IceCavern,
    BottomOfTheWell,
    GerudoTrainingGround,
    InsideGanonsCastle,
    // Currently we assume that the name of the savewarp is equal to the name of its target region.
    // Once boss ER is implemented, this is something that needs to change, so a boss's savewarp can be separate from its vanilla dungeon's.
    /*
    QueenGohmaBossRoom,
    KingDodongoBossRoom,
    BarinadeBossRoom,
    PhantomGanonBossRoom,
    VolvagiaBossRoom,
    MorphaBossRoom,
    BongoBongoBossRoom,
    TwinrovaBossRoom,
    */
    GanonsTower,
    KfLinksHouse,
    ThievesHideout,
}

impl Parse for Savewarp {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let name = input.parse::<LitStr>()?.value().to_case(Case::Pascal);
        for variant in all::<Self>() {
            if name == variant.to_string() {
                return Ok(variant)
            }
        }
        Err(input.error(format!("expected savewarp, found string literal {name:?}")))
    }
}

impl ToTokens for Savewarp {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = Ident::new(&self.to_string(), Span::call_site());
        quote!(Savewarp::#ident).to_tokens(tokens);
    }
}

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

#[derive(Debug, Hash, EnumSetType)]
pub enum Item {
    KokiriSword,
}

impl Parse for Item {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(match &*input.parse::<Ident>()?.to_string() {
            "KokiriSword" => Self::KokiriSword,
            name => return Err(input.error(format!("expected item, found ident {name}"))),
        })
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let stream = match self {
            Self::KokiriSword => quote!(Item::KokiriSword),
        };
        stream.to_tokens(tokens);
    }
}
