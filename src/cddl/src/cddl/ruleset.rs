use crate::cddl::error::Error;
use crate::cddl::{parse, parser};
use pest::iterators::{Pair, Pairs};
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Assignment {
    Define,
    Additional,
}

#[derive(Debug, Clone)]
pub enum Rule {
    /// An alias to another type.
    Typename {
        name: String,
        param: Vec<String>,
        tt: Type,
        assignment: Assignment,
    },
    Groupname {
        param: Vec<String>,
    },
}

/// A Value.
#[derive(Debug, Clone)]
pub enum Value {
    /// An integer. CBOR (and CDDL) can represent numbers up in the range of `[-2^64..2^64-1]`
    /// which is twice as much as i64. We use an i128 for this reason.
    Integer(i128),
    Float(f64),
    Text(String),
    Bytes(Vec<u8>),
}

/// The repesentation of the `occur` rule, with Once added as the default.
#[derive(Debug, Clone)]
pub enum Occurence {
    /// The default if not specified. The group can appear once.
    Required,

    /// A range from a number to another, like `1*5`.
    Range(u64, u64),

    /// Representing any number of occurences, including 0.
    Any,

    /// Representing `+`.
    OnceOrMore,

    /// Representing `?`.
    Optional,
}

#[derive(Debug, Clone)]
pub enum GroupEntry {
    MemberKey(Occurence, Type, Option<Type>),
    // GenericArg(Occurence, Type),
    // Group(Occurence, Vec<GroupChoice>),
}

/// The representation of the `grpchoice` rule.
#[derive(Debug, Clone)]
pub struct GroupChoice(pub Vec<GroupEntry>);

/// The representation of the `type`, `type1` and `type2` rules.
#[derive(Debug, Clone)]
pub enum Type {
    /// A simple type, ie. a type reference.
    Reference(String),

    /// A reference that uses a generic param.
    GenericReference(String, Vec<Type>),

    /// A bare word.
    Id(String),

    /// A constant value.
    Value(Value),

    MapExpression(Vec<GroupChoice>),
    // ArrayExpression(Vec<GroupChoice>),
    //
    // /// Unwrapping a type means stripping its name and merging it with the parent type.
    // /// This is not done during parsing, but during normalization.
    // Unwrap(Box<Type>),
    //
    /// A range operation, where both ends are included.
    InclusiveRange(Box<Type>, Box<Type>),
    /// A range operation, where the end is exclusive (start is always inclusive).
    EndExclusiveRange(Box<Type>, Box<Type>),

    /// A controlled op type, from the rule `ctlop`.
    Control(Box<Type>, String, Box<Type>),

    /// A union type, that can be any of the subtypes defined.
    UnionType(Vec<Type>),

    /// A tagged field in CBOR.
    TaggedType(u64, Box<Type>),

    /// A custom MajorType.
    MajorType(u8, Option<u64>),

    /// Any type, any value. This is equivalent to a single `#`.
    Any(),
}

fn typename_from_id(p: &Pair<parser::Rule>) -> String {
    if p.as_rule() == parser::Rule::id {
        p.as_str().to_string()
    } else {
        unreachable!("Unexpected rule `{:?}`...", p.as_rule());
    }
}

fn id_from_genericparm(p: Pair<parser::Rule>) -> (String, Vec<String>) {
    match p.as_rule() {
        parser::Rule::id => (p.as_str().to_string(), vec![]),
        parser::Rule::id_genericparm | parser::Rule::id_genericarg => {
            let mut it = p.into_inner();

            let first = it.next().unwrap(); // Always have an ID.
            let second = it.next();

            (
                first.as_str().to_string(),
                second.map_or(vec![], |p| {
                    p.into_inner()
                        .filter(|p| p.as_rule() != parser::Rule::id)
                        .map(|p| p.as_str().to_string())
                        .collect()
                }),
            )
        }
        r => unreachable!("Asked to get id() of non-id rule. Rule: `{:?}`", r),
    }
}

fn create_type_from_value(pair: Pair<parser::Rule>) -> Type {
    let v = pair.into_inner().next().unwrap();
    match v.as_rule() {
        parser::Rule::number => {
            let value = match v.as_str().parse::<i128>() {
                Ok(i) => Value::Integer(i),
                Err(_) => match v.as_str().parse::<f64>() {
                    Ok(f) => Value::Float(f),
                    Err(e) => unreachable!("Could not parse value: {:?}", e),
                },
            };
            Type::Value(value)
        }
        parser::Rule::text => {
            Type::Value(Value::Text(v.into_inner().concat().as_str().to_string()))
        }
        parser::Rule::bytes => {
            println!("bytes: {}", v.as_str());
            Type::Value(Value::Bytes(vec![]))
        }
        _ => unreachable!(),
    }
}

fn create_type_from_type2_tag(pair: Pair<parser::Rule>) -> Type {
    let pairs: Vec<Pair<parser::Rule>> = pair.into_inner().collect();
    let tag = pairs[0]
        .as_str()
        .parse::<u64>()
        .expect("Invalid tag value.");

    let tagged_type = pairs[1].clone().into_inner().next().unwrap();
    Type::TaggedType(tag as u64, Box::new(create_type(tagged_type)))
}

fn create_occurence(pair: Pair<parser::Rule>) -> Occurence {
    match pair.as_str() {
        "+" => Occurence::OnceOrMore,
        "?" => Occurence::Optional,
        str => {
            // This is a `uint? ~ "*" ~ uint?`.
            let vec: Vec<Pair<parser::Rule>> = pair.into_inner().collect();
            match vec.len() {
                0 => Occurence::Any,
                1 => {
                    let nb = vec[0].as_str().parse::<u64>().unwrap();
                    if str.starts_with("*") {
                        // This is `*uint`.
                        Occurence::Range(0, nb)
                    } else {
                        Occurence::Range(nb, u64::max_value())
                    }
                }
                2 => Occurence::Range(
                    vec[0].as_str().parse::<u64>().unwrap(),
                    vec[1].as_str().parse::<u64>().unwrap(),
                ),
                _ => unreachable!("Unexpected number of occur pairs; {:?}", vec),
            }
        }
    }
}

fn create_memberkey_type(pair: Pair<parser::Rule>) -> Type {
    // Just need to read the first rule.
    let item = pair.into_inner().next().unwrap();
    match item.as_rule() {
        parser::Rule::type1 => create_type(item),
        parser::Rule::bareword => Type::Id(item.as_str().to_string()),
        parser::Rule::id => Type::Id(item.as_str().to_string()),
        parser::Rule::value => create_type_from_value(item),
        x => unreachable!("Expected a memberkey rule, got `{:?}", x),
    }
}

fn create_type_from_type2_map(pair: Pair<parser::Rule>) -> Type {
    let group = pair.into_inner().next().unwrap();
    let grpents: Vec<GroupEntry> = group
        .into_inner()
        .map(|grpent| match grpent.as_rule() {
            parser::Rule::grpent_memberkey => {
                // The rule is { occur? ~ memberkey? ~ type_ }
                let mut it = grpent.into_inner();
                let first = it.next().unwrap();
                let second = it.next();
                let third = it.next();
                if third.is_none() && second.is_none() {
                    GroupEntry::MemberKey(Occurence::Required, create_type(first), None)
                } else if third.is_none() {
                    if first.as_rule() == parser::Rule::occur {
                        GroupEntry::MemberKey(
                            create_occurence(first),
                            create_type(second.unwrap()),
                            None,
                        )
                    } else {
                        GroupEntry::MemberKey(
                            Occurence::Any,
                            create_memberkey_type(first),
                            Some(create_type(second.unwrap())),
                        )
                    }
                } else {
                    GroupEntry::MemberKey(
                        create_occurence(first),
                        create_memberkey_type(second.unwrap()),
                        Some(create_type(third.unwrap())),
                    )
                }
            }
            x => unreachable!("Invalid grpent rule `{:?}`", x),
        })
        .collect();

    Type::MapExpression(vec![GroupChoice(grpents)])
}

fn create_type_from_type2_typeref(pair: Pair<parser::Rule>) -> Type {
    let mut it = pair.into_inner();

    let first = it.next().unwrap(); // Always have an ID.
    let second = it.next();

    if let Some(ref genericargs) = second {
        Type::GenericReference(
            first.as_str().to_string(),
            genericargs.clone().into_inner().map(create_type).collect(),
        )
    } else {
        Type::Reference(first.as_str().to_string())
    }
}

fn create_type_from_type2_major(pair: Pair<parser::Rule>) -> Type {
    let mut it = pair.into_inner();
    let digit = it.next().unwrap().as_str().parse::<u8>().unwrap();
    let ai = it.next().map(|p| p.as_str().parse::<u64>().unwrap());

    Type::MajorType(digit, ai)
}

fn create_type(pair: Pair<parser::Rule>) -> Type {
    match pair.as_rule() {
        parser::Rule::type_ => {
            let inners: Vec<Pair<parser::Rule>> = pair
                .into_inner()
                .filter(|p| p.as_rule() == parser::Rule::type1)
                .collect();

            if inners.len() != 1 {
                Type::UnionType(inners.iter().map(|i| create_type(i.clone())).collect())
            } else {
                create_type(inners[0].clone())
            }
        }
        parser::Rule::type1 => {
            let pairs: Vec<Pair<parser::Rule>> = pair.into_inner().collect();

            match pairs.len() {
                0 => unreachable!("`type1` cannot be empty."),
                1 => {
                    // No range.
                    create_type(pairs[0].clone())
                }
                3 => {
                    // Range or control.
                    let p1 = pairs[0].clone();
                    let p2 = pairs[1].clone();
                    let p3 = pairs[2].clone();
                    let first = create_type(p1);
                    let range_or_ctl_op = p2;
                    let second = create_type(p3.into_inner().next().unwrap().clone());

                    match range_or_ctl_op.as_str() {
                        "..." => Type::EndExclusiveRange(Box::new(first), Box::new(second)),
                        ".." => Type::InclusiveRange(Box::new(first), Box::new(second)),
                        _ => {
                            let id_ = range_or_ctl_op.into_inner().next().unwrap();
                            Type::Control(Box::new(first), typename_from_id(&id_), Box::new(second))
                        }
                    }
                }
                x => unreachable!("`type1` cannot be of length {:?}: {:#?}", x, pairs),
            }
        }
        parser::Rule::type2 => create_type(pair.into_inner().next().unwrap()),
        parser::Rule::value => create_type_from_value(pair),
        parser::Rule::type2_typeref => {
            create_type_from_type2_typeref(pair.into_inner().next().unwrap())
        }
        parser::Rule::type2_paren => create_type(pair.into_inner().next().unwrap()),
        parser::Rule::type2_tag => create_type_from_type2_tag(pair),
        parser::Rule::type2_map => create_type_from_type2_map(pair),
        parser::Rule::type2_major => create_type_from_type2_major(pair),
        parser::Rule::type2_any => Type::Any(),
        x => unreachable!("Was not expecting `{:?}` (str is `{:?}`)", x, pair.as_str()),
    }
}

fn create_rule_from_typename(pairs: Pairs<parser::Rule>) -> Rule {
    let pairs: Vec<Pair<parser::Rule>> = pairs.collect();

    let (id, genericparm) = id_from_genericparm(pairs[0].clone());

    let (assignt, type_) = match pairs.len() {
        3 => {
            // Does not have a `genericparm`.
            (pairs[1].as_str().to_string(), create_type(pairs[2].clone()))
        }
        4 => (pairs[2].as_str().to_string(), create_type(pairs[3].clone())),

        _ => unreachable!(),
    };

    Rule::Typename {
        name: id,
        param: genericparm,
        tt: type_,
        assignment: if assignt.eq(&"=".to_string()) {
            Assignment::Define
        } else {
            Assignment::Additional
        },
    }
}

fn create_ruleset_from_cddl(pairs: Pairs<parser::Rule>) -> Ruleset {
    // Get all the `rule_*` rules and create a ruleset out of it.
    Ruleset {
        rules: pairs
            .filter(|p| p.as_rule() != parser::Rule::EOI)
            .map(|pair| match pair.as_rule() {
                parser::Rule::rule_typename => create_rule_from_typename(pair.into_inner()),
                parser::Rule::rule_groupname => {
                    // create_rule_from_typename(pair.into_inner())
                    Rule::Groupname { param: vec![] }
                }
                x => unreachable!("Unexpected rule: `{:?}`.", x),
            })
            .collect(),
    }
}

#[derive(Debug)]
pub struct Ruleset {
    rules: Vec<Rule>,
}

impl Ruleset {
    /// Validates the rules for consistency.
    pub fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl TryFrom<Pairs<'_, parser::Rule>> for Ruleset {
    type Error = crate::cddl::error::Error;

    fn try_from(value: Pairs<parser::Rule>) -> Result<Self, Self::Error> {
        Ok(create_ruleset_from_cddl(value))
    }
}

impl FromStr for Ruleset {
    type Err = crate::cddl::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(create_ruleset_from_cddl(parse(s)?))
    }
}
