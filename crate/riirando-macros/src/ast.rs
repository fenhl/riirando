use {
    std::collections::HashMap,
    proc_macro2::Span,
    syn::{
        *,
        parse::{
            Parse,
            ParseStream,
        },
    },
    riirando_common::*,
};

pub(crate) struct LogicFile {
    pub(crate) regions: HashMap<String, RegionInfo>,
}

impl Parse for LogicFile {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut regions = HashMap::default();
        while !input.is_empty() {
            let name = input.parse::<LitStr>()?;
            if regions.insert(name.value(), input.parse()?).is_some() {
                return Err(input.error(format!("logic file defines multiple regions named {:?}", name.value())))
            }
        }
        Ok(Self { regions })
    }
}

pub(crate) struct RegionInfo {
    pub(crate) time_of_day: TimeOfDayBehavior,
    pub(crate) exits: HashMap<String, Access>,
}

impl Parse for RegionInfo {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut time_of_day = None;
        let mut exits = None;
        let content;
        braced!(content in input);
        let fields = content.parse_terminated(RegionInfoField::parse, Token![,])?;
        for field in fields {
            match field {
                RegionInfoField::TimeOfDay(new_time_of_day) => if time_of_day.replace(new_time_of_day).is_some() { return Err(input.error("time_of_day specified multiple times")) },
                RegionInfoField::Exits(new_exits) => if exits.replace(new_exits).is_some() { return Err(input.error("exits specified multiple times")) },
            }
        }
        Ok(Self {
            time_of_day: time_of_day.ok_or_else(|| input.error("missing time_of_day field in region info"))?,
            exits: exits.unwrap_or_default(),
        })
    }
}

enum RegionInfoField {
    TimeOfDay(TimeOfDayBehavior),
    Exits(HashMap<String, Access>),
}

impl Parse for RegionInfoField {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let field_name = input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;
        Ok(match &*field_name.to_string() {
            "time_of_day" => Self::TimeOfDay(input.parse()?),
            "exits" => {
                let mut exits_map = HashMap::default();
                let content;
                braced!(content in input);
                let exits = content.parse_terminated(Exit::parse, Token![,])?;
                for Exit { name, access } in exits {
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

struct Exit {
    name: String,
    access: Access,
}

impl Parse for Exit {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let name = input.parse::<LitStr>()?.value();
        input.parse::<Token![:]>()?;
        let access = input.parse()?;
        Ok(Self { name, access })
    }
}

pub(crate) struct Access(pub(crate) Expr);

impl Access {
    fn from_expr(expr: Expr) -> Result<Self> {
        Ok(Self(match expr {
            Expr::Binary(ExprBinary { attrs, left, op: BinOp::Or(_), right }) if attrs.is_empty() => {
                let Self(left) = Self::from_expr(*left)?;
                let Self(right) = Self::from_expr(*right)?;
                parse_quote!(#left || #right)
            }
            Expr::Lit(ExprLit { attrs, lit: Lit::Bool(value) }) if attrs.is_empty() => parse_quote!(#value),
            Expr::Path(ExprPath { attrs, qself, path }) if attrs.is_empty() && qself.is_none() && path.is_ident("at_dampe_time") => parse_quote!(state.time_of_day == TimeOfDay::Dampe),
            Expr::Path(ExprPath { attrs, qself, path }) if attrs.is_empty() && qself.is_none() && path.is_ident("at_day") => parse_quote!(state.time_of_day.is_day()),
            Expr::Path(ExprPath { attrs, qself, path }) if attrs.is_empty() && qself.is_none() && path.is_ident("at_night") => parse_quote!(state.time_of_day.is_night()),
            Expr::Path(ExprPath { attrs, qself, path }) if attrs.is_empty() && qself.is_none() && path.is_ident("is_adult") => parse_quote!(state.age == Age::Adult),
            Expr::Path(ExprPath { attrs, qself, path }) if attrs.is_empty() && qself.is_none() && path.is_ident("is_child") => parse_quote!(state.age == Age::Child),
            expr => return Err(Error::new(Span::call_site(), format!("unexpected access expression: {expr:#?}"))),
        }))
    }
}

impl Parse for Access {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Self::from_expr(input.parse()?)
    }
}
