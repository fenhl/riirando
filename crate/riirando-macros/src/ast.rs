use {
    std::collections::HashMap,
    convert_case::{
        Case,
        Casing as _,
    },
    if_chain::if_chain,
    itertools::Itertools as _,
    proc_macro2::Span,
    quote::quote,
    syn::{
        *,
        parse::{
            Parse,
            ParseStream,
        },
    },
    riirando_common::{
        *,
        Item,
    },
};

pub(crate) struct LogicFile {
    pub(crate) helpers: HashMap<String, Expr>,
    pub(crate) regions: HashMap<String, RegionInfo>,
}

impl Parse for LogicFile {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut helpers = HashMap::default();
        let mut regions = HashMap::default();
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(Token![fn]) {
                input.parse::<Token![fn]>()?;
                let name = input.parse::<Ident>()?;
                let content;
                parenthesized!(content in input);
                if !content.is_empty() {
                    return Err(input.error(format!("logic helper {name} takes parameters but helpers with parameters aren't implemented yet")))
                }
                let content;
                braced!(content in input);
                if helpers.insert(name.to_string(), content.parse()?).is_some() {
                    return Err(input.error(format!("logic file defines multiple helpers named {name}")))
                }
                if !content.is_empty() {
                    return Err(input.error(format!("logic helper {name} continues after end of access expression")))
                }
            } else if lookahead.peek(LitStr) {
                let name = input.parse::<LitStr>()?;
                if regions.insert(name.value(), input.parse()?).is_some() {
                    return Err(input.error(format!("logic file defines multiple regions named {:?}", name.value())))
                }
            } else {
                return Err(lookahead.error())
            }
        }
        Ok(Self { helpers, regions })
    }
}

pub(crate) struct RegionInfo {
    pub(crate) savewarp: Option<Savewarp>,
    pub(crate) time_of_day: TimeOfDayBehavior,
    pub(crate) events: HashMap<String, Expr>,
    pub(crate) items: HashMap<Item, Vec<Expr>>,
    pub(crate) exits: HashMap<String, Expr>,
}

impl Parse for RegionInfo {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut savewarp = None;
        let mut time_of_day = None;
        let mut events = None;
        let mut items = None;
        let mut exits = None;
        let content;
        braced!(content in input);
        let fields = content.parse_terminated(RegionInfoField::parse, Token![,])?;
        for field in fields {
            match field {
                RegionInfoField::Savewarp(new_savewarp) => if savewarp.replace(new_savewarp).is_some() { return Err(input.error("savewarp specified multiple times")) },
                RegionInfoField::TimeOfDay(new_time_of_day) => if time_of_day.replace(new_time_of_day).is_some() { return Err(input.error("time_of_day specified multiple times")) },
                RegionInfoField::Events(new_events) => if events.replace(new_events).is_some() { return Err(input.error("events specified multiple times")) },
                RegionInfoField::Items(new_items) => if items.replace(new_items).is_some() { return Err(input.error("items specified multiple times")) },
                RegionInfoField::Exits(new_exits) => if exits.replace(new_exits).is_some() { return Err(input.error("exits specified multiple times")) },
            }
        }
        Ok(Self {
            time_of_day: time_of_day.ok_or_else(|| input.error("missing time_of_day field in region info"))?,
            events: events.unwrap_or_default(),
            items: items.unwrap_or_default(),
            exits: exits.unwrap_or_default(),
            savewarp,
        })
    }
}

enum RegionInfoField {
    Savewarp(Savewarp),
    TimeOfDay(TimeOfDayBehavior),
    Events(HashMap<String, Expr>),
    Items(HashMap<Item, Vec<Expr>>),
    Exits(HashMap<String, Expr>),
}

impl Parse for RegionInfoField {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let field_name = input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;
        Ok(match &*field_name.to_string() {
            "savewarp" => Self::Savewarp(input.parse()?),
            "time_of_day" => Self::TimeOfDay(input.parse()?),
            "events" => {
                let mut events_map = HashMap::default();
                let content;
                braced!(content in input);
                let events = content.parse_terminated(NameAccess::parse, Token![,])?;
                for NameAccess { name, access } in events {
                    if events_map.insert(name.clone(), access).is_some() {
                        return Err(input.error(format!("region defines event {name:?} multiple times")))
                    }
                }
                Self::Events(events_map)
            }
            "items" => {
                let mut items_map = HashMap::<_, Vec<_>>::default();
                let content;
                braced!(content in input);
                let items = content.parse_terminated(ItemLocation::parse, Token![,])?;
                for ItemLocation { item, access } in items {
                    items_map.entry(item).or_default().push(access);
                }
                Self::Items(items_map)
            }
            "exits" => {
                let mut exits_map = HashMap::default();
                let content;
                braced!(content in input);
                let exits = content.parse_terminated(NameAccess::parse, Token![,])?;
                for NameAccess { name, access } in exits {
                    if exits_map.insert(name.clone(), access).is_some() {
                        return Err(input.error(format!("region defines multiple exits to {name:?}")))
                    }
                }
                Self::Exits(exits_map)
            }
            field_name => return Err(input.error(format!("unexpected region info field: {field_name}"))),
        })
    }
}

struct ItemLocation {
    item: Item,
    access: Expr,
}

impl Parse for ItemLocation {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let item = input.parse()?;
        input.parse::<Token![:]>()?;
        let access = input.parse()?;
        Ok(Self { item, access })
    }
}

struct NameAccess {
    name: String,
    access: Expr,
}

impl Parse for NameAccess {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let name = input.parse::<LitStr>()?.value();
        input.parse::<Token![:]>()?;
        let access = input.parse()?;
        Ok(Self { name, access })
    }
}

pub(crate) fn expand_access_expr(helpers: &HashMap<String, Expr>, expr: &Expr) -> Result<Expr> {
    Ok(match expr {
        Expr::Binary(ExprBinary { attrs, left, op: BinOp::And(_), right }) if attrs.is_empty() => {
            let left = expand_access_expr(helpers, &left)?;
            let right = expand_access_expr(helpers, &right)?;
            parse_quote!((#left && #right))
        }
        Expr::Binary(ExprBinary { attrs, left, op: BinOp::Or(_), right }) if attrs.is_empty() => {
            let left = expand_access_expr(helpers, &left)?;
            let right = expand_access_expr(helpers, &right)?;
            parse_quote!((#left || #right))
        }
        Expr::Call(ExprCall { attrs, func, paren_token: _, args }) if attrs.is_empty() => match **func {
            Expr::Path(ExprPath { ref attrs, qself: None, ref path }) if attrs.is_empty() => if let Some(ident) = path.get_ident() {
                match &*ident.to_string() {
                    "can_pay" => match args.into_iter().exactly_one() {
                        Ok(Expr::Lit(ExprLit { attrs, lit: Lit::Int(lit) })) if attrs.is_empty() => {
                            let price = lit.base10_parse::<u16>()?;
                            match price {
                                0 => parse_quote!(true),
                                1..=99 => parse_quote!(inventory.contains(Item::Wallet)),
                                100..=200 => parse_quote!(inventory.contains((Item::Wallet, 2))),
                                201..=500 => parse_quote!(inventory.contains((Item::Wallet, 3))),
                                501..=999 => parse_quote!(inventory.contains((Item::Wallet, 4))),
                                1000.. => parse_quote!(false),
                            }
                        }
                        Ok(expr) => return Err(Error::new(Span::call_site(), format!("unexpected parameter in can_pay: {expr:#?}"))),
                        Err(e) => return Err(Error::new(Span::call_site(), format!("can_pay takes exactly 1 parameter, got {}", e.len()))),
                    },
                    "here" => match args.into_iter().exactly_one() {
                        Ok(expr) => {
                            let _ = expand_access_expr(helpers, expr)?; //TODO create the event
                            parse_quote!(inventory.contains(AnonymousEvent {})) //TODO
                        }
                        Err(e) => return Err(Error::new(Span::call_site(), format!("here takes exactly 1 parameter, got {}", e.len()))),
                    },
                    _ => return Err(Error::new(Span::call_site(), format!("unexpected function name in access expression: {ident}")))
                }
            } else {
                return Err(Error::new(Span::call_site(), format!("unexpected path in callee in access expression: {path:#?}")))
            },
            ref expr => return Err(Error::new(Span::call_site(), format!("unexpected callee in access expression: {expr:#?}"))),
        }
        Expr::Lit(ExprLit { attrs, lit }) if attrs.is_empty() => match lit {
            Lit::Bool(value) => parse_quote!(#value),
            Lit::Str(lit) => {
                let variant_name = lit.value().to_case(Case::Pascal);
                let variant_ident = Ident::new(&variant_name, Span::call_site());
                parse_quote!(inventory.contains(NamedEvent::#variant_ident))
            }
            lit => return Err(Error::new(Span::call_site(), format!("unexpected literal in access expression: {lit:#?}"))),
        },
        Expr::Paren(ExprParen { attrs, paren_token: _, expr }) if attrs.is_empty() => expand_access_expr(helpers, expr)?,
        Expr::Path(ExprPath { attrs, qself: None, path }) if attrs.is_empty() => if let Some(ident) = path.get_ident() {
            match &*ident.to_string() {
                "at_dampe_time" => parse_quote!((state.time_of_day == TimeOfDay::Dampe)),
                "at_day" => parse_quote!(state.time_of_day.is_day()),
                "at_night" => parse_quote!(state.time_of_day.is_night()),
                "is_adult" => parse_quote!((state.age == Age::Adult)),
                "is_child" => parse_quote!((state.age == Age::Child)),
                _ => if let Some(access) = helpers.get(&ident.to_string()) {
                    expand_access_expr(helpers, access)?
                } else if let Ok(item) = parse2(quote!(#ident)) {
                    match item {
                        Item::BombBag => parse_quote!(inventory.contains(Item::BombBag | Item::Bombs)),
                        Item::DinsFire => parse_quote!(inventory.contains(Item::DinsFire | Item::MagicMeter | Item::MagicRefills)),
                        Item::MagicMeter => parse_quote!(inventory.contains(Item::MagicMeter | Item::MagicRefills)),
                        Item::Slingshot => parse_quote!((state.age == Age::Child && inventory.contains(Item::Slingshot | Item::DekuSeeds))),
                        | Item::Bottle
                        | Item::GoldSkulltulaToken
                        | Item::Ocarina
                        | Item::OcarinaAButton
                        | Item::OcarinaCDownButton
                        | Item::OcarinaCLeftButton
                        | Item::OcarinaCRightButton
                        | Item::OcarinaCUpButton
                        | Item::Scale
                        | Item::StoneOfAgony
                            => parse_quote!(inventory.contains(#item)),
                        | Item::Boomerang
                        | Item::Bugs
                        | Item::DekuShield
                        | Item::DekuSticks
                        | Item::KokiriSword
                        | Item::MagicBean
                            => parse_quote!((state.age == Age::Child && inventory.contains(#item))),
                        | Item::Cojiro
                        | Item::Hookshot
                        | Item::HoverBoots
                        | Item::OddMushroom
                        | Item::OddPotion
                        | Item::PoachersSaw
                            => parse_quote!((state.age == Age::Adult && inventory.contains(#item))),
                        | Item::BoleroOfFire
                        | Item::EponasSong
                        | Item::MinuetOfForest
                        | Item::NocturneOfShadow
                        | Item::PreludeOfLight
                        | Item::RequiemOfSpirit
                        | Item::SariasSong
                        | Item::SerenadeOfWater
                        | Item::SongOfStorms
                        | Item::SongOfTime
                        | Item::SunsSong
                        | Item::ZeldasLullaby
                            => parse_quote!(inventory.contains(Item::Ocarina | #item)), //TODO note buttons
                        | Item::Arrows
                        | Item::Bombs
                        | Item::DekuNuts
                        | Item::DekuSeeds
                        | Item::MagicRefills
                        | Item::PieceOfHeart
                        | Item::RecoveryHearts
                        | Item::Wallet
                            => return Err(Error::new(Span::call_site(), format!("unsupported item in access expression: {item:?}"))),
                    }
                } else {
                    return Err(Error::new(Span::call_site(), format!("unexpected identifier in access expression: {ident}")))
                },
            }
        } else {
            return Err(Error::new(Span::call_site(), format!("unexpected path in access expression: {path:#?}")))
        },
        Expr::Tuple(ExprTuple { attrs, paren_token: _, elems }) if attrs.is_empty() => if_chain! {
            if let Some((Expr::Path(ExprPath { attrs: item_attrs, qself: None, path }), Expr::Lit(ExprLit { attrs: count_attrs, lit: Lit::Int(count) }))) = elems.into_iter().collect_tuple();
            if item_attrs.is_empty();
            if count_attrs.is_empty();
            if let Some(ident) = path.get_ident();
            if let Ok(item) = parse2(quote!(#ident));
            then {
                match item {
                    | Item::MagicBean
                        => parse_quote!((state.age == Age::Child && inventory.contains((#item, #count)))),
                    | Item::Hookshot
                        => parse_quote!((state.age == Age::Adult && inventory.contains((#item, #count)))),
                    _ => return Err(Error::new(Span::call_site(), format!("unsupported item in tuple in access expression: {item:?}"))),
                }
            } else {
                return Err(Error::new(Span::call_site(), format!("unexpected tuple in access expression: {expr:#?}")))
            }
        },
        expr => return Err(Error::new(Span::call_site(), format!("unexpected access expression: {expr:#?}"))),
    })
}
