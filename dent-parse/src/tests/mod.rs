mod parser;
mod tokenizer;

use super::*;

#[test]
fn access() {
    let parser = Dent::new(HashMap::new());

    assert_eq!(parser.parse("{ foo: 1 }").unwrap()["foo"], Value::Int(1));
}

#[test]
fn access_nested() {
    let parser = Dent::new(HashMap::new());

    assert_eq!(
        parser.parse("{ foo: { bar: 1 } }").unwrap()["foo"]["bar"],
        Value::Int(1)
    );
}

#[test]
fn index() {
    let parser = Dent::new(HashMap::new());

    assert_eq!(parser.parse("[ 1 2 3 ]").unwrap()[1], Value::Int(2));
}

#[test]
fn index_nested() {
    let parser = Dent::new(HashMap::new());

    assert_eq!(parser.parse("[ [ 1 2 3 ] ]").unwrap()[0][1], Value::Int(2));
}

#[test]
fn index_access_mixed() {
    let parser = Dent::new(HashMap::new());

    assert_eq!(
        parser.parse("{ foo: [ 1 2 3 ] }").unwrap()["foo"][1],
        Value::Int(2)
    );
}

#[test]
fn import() {
    let parser = Dent::default();

    assert_eq!(
        parser.parse("@import \"examples/dent/dict.dent\""),
        Ok(Value::Dict(
            vec![
                ("name", Value::Str("Mario")),
                (
                    "skills",
                    Value::List(vec![Value::Str("jumps"), Value::Str("grows")])
                ),
                ("age", Value::Int(35)),
                ("alive", Value::Bool(true)),
            ]
            .into_iter()
            .collect()
        ))
    );
}

#[test]
fn import_mutability() {
    let parser = Dent::default();

    let mut v = parser.parse("@import \"examples/dent/dict.dent\"").unwrap();

    assert_eq!(
        v,
        Value::Dict(
            vec![
                ("name", Value::Str("Mario")),
                (
                    "skills",
                    Value::List(vec![Value::Str("jumps"), Value::Str("grows")])
                ),
                ("age", Value::Int(35)),
                ("alive", Value::Bool(true)),
            ]
            .into_iter()
            .collect()
        )
    );

    v["name"] = Value::Str("Luigi");

    assert_eq!(
        parser.parse("@import \"examples/dent/dict.dent\""),
        Ok(Value::Dict(
            vec![
                ("name", Value::Str("Mario")),
                (
                    "skills",
                    Value::List(vec![Value::Str("jumps"), Value::Str("grows")])
                ),
                ("age", Value::Int(35)),
                ("alive", Value::Bool(true)),
            ]
            .into_iter()
            .collect()
        ))
    );
}

#[test]
fn import_nested() {
    let parser = Dent::default();

    assert_eq!(
        parser.parse("{ characters: [ @import \"examples/dent/dict.dent\" ] }"),
        Ok(Value::Dict(
            vec![(
                "characters",
                Value::List(vec![Value::Dict(
                    vec![
                        ("name", Value::Str("Mario")),
                        (
                            "skills",
                            Value::List(vec![Value::Str("jumps"), Value::Str("grows")])
                        ),
                        ("age", Value::Int(35)),
                        ("alive", Value::Bool(true)),
                    ]
                    .into_iter()
                    .collect()
                )])
            )]
            .into_iter()
            .collect()
        ))
    );
}

#[test]
fn merge_dicts() {
    let parser = Dent::default();

    assert_eq!(
        parser.parse("@merge [ { a: 1 b: 2 } { b: 3 c: 4 } ]"),
        Ok(Value::Dict(
            vec![
                ("a", Value::Int(1)),
                ("b", Value::Int(3)),
                ("c", Value::Int(4)),
            ]
            .into_iter()
            .collect()
        ))
    );
}

#[test]
fn merge_lists() {
    let parser = Dent::default();

    assert_eq!(
        parser.parse("@merge [ [ 1 2 3 ] [ 4 5 6 ] ]"),
        Ok(Value::List(vec![
            Value::Int(1),
            Value::Int(2),
            Value::Int(3),
            Value::Int(4),
            Value::Int(5),
            Value::Int(6),
        ]))
    );
}

#[test]
fn recursive() {
    let parser = Dent::default();

    assert_eq!(
        parser.parse_file("examples/dent/recursive.dent"),
        Ok(Value::Dict(
            vec![("self", Value::None)].into_iter().collect()
        ))
    );
}
