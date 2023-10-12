use super::*;

#[test]
fn add_function() {
    let mut functions: HashMap<String, Box<Function>> = HashMap::new();
    functions.insert(
        "add".to_string(),
        Box::new(|value| {
            let mut sum = 0;
            if let Value::List(values) = value {
                for value in values.iter() {
                    if let Value::Int(i) = value {
                        sum += i;
                    }
                }
                Value::Int(sum)
            } else if let Value::Int(i) = value {
                Value::Int(*i)
            } else {
                Value::None
            }
        }),
    );
    let parser = Dent::new(functions);

    assert_eq!(parser.parse("@add {}"), Ok(Value::None));
    assert_eq!(parser.parse("@add 0"), Ok(Value::Int(0)));
    assert_eq!(parser.parse("@add [ 1 2 ]"), Ok(Value::Int(3)));
}

#[test]
fn empty() {
    let parser = Dent::new(HashMap::new());

    assert_eq!(parser.parse(""), Ok(Value::None));
}

#[test]
fn string() {
    let parser = Dent::new(HashMap::new());

    assert_eq!(parser.parse("foo"), Ok(Value::Str("foo")));
    assert_eq!(parser.parse("\"foo\""), Ok(Value::Str("foo")));
}

#[test]
fn list() {
    let parser = Dent::new(HashMap::new());

    assert_eq!(
        parser.parse("[ 1 2 3 ]"),
        Ok(Value::List(vec![
            Value::Int(1),
            Value::Int(2),
            Value::Int(3)
        ]))
    );
}

#[test]
fn dict() {
    let parser = Dent::new(HashMap::new());

    assert_eq!(
        parser.parse("{ foo: 1 bar: 2 }"),
        Ok(Value::Dict(
            vec![("foo", Value::Int(1)), ("bar", Value::Int(2))]
                .into_iter()
                .collect()
        ))
    );
}

#[test]
fn comment() {
    let parser = Dent::new(HashMap::new());

    assert_eq!(parser.parse("# foo"), Ok(Value::None));
}

#[test]
fn multithreaded() {
    let parser = Dent::new(HashMap::new());

    let mut threads = Vec::new();

    let parser = Arc::new(parser);

    for _ in 0..100 {
        let parser = parser.clone();
        threads.push(std::thread::spawn(move || {
            assert_eq!(parser.parse("foo"), Ok(Value::Str("foo")));
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    assert_eq!(parser.parse("foo"), Ok(Value::Str("foo")));
}

#[test]
fn file() {
    let parser = Dent::new(HashMap::new());

    assert_eq!(
        parser.parse_file("examples/dent/dict.dent"),
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
