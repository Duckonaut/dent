use std::{io::Read, path::PathBuf};

use dent_parse::{Dent, Value};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = "dentq",
    about = "A simple command line tool for querying dent files."
)]
struct Cli {
    #[clap(help = "The dent file to query.")]
    file: PathBuf,
    #[clap(help = "The query to run. For example: .foo.bar[0].baz")]
    query: String,
}

fn main() {
    let args = Cli::parse();
    let dent = Dent::default();

    if args.file == PathBuf::from("-") {
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        let mut buffer = String::new();
        handle.read_to_string(&mut buffer).unwrap();

        let v = dent.parse(&buffer).unwrap();

        let result = query(&v, &args.query);
        println!("{}", result);
    } else {
        if !args.file.exists() {
            eprintln!("File does not exist: {:?}", args.file);
            std::process::exit(1);
        }

        let v = dent.parse_file(&args.file).unwrap();

        let result = query(&v, &args.query);
        println!("{}", result);
    }
}

enum QueryPart {
    Key(String),
    Index(usize),
}

fn query(value: &Value, query: &str) -> String {
    let parts = query.split('.');
    let mut query_parts = Vec::new();

    for part in parts.filter(|p| !p.is_empty()) {
        if part.contains('[') {
            let mut part_parts = part.split('[');
            let key = part_parts.next().unwrap();
            let index = part_parts.next().unwrap().replace(']', "");
            let index = index.parse::<usize>().unwrap();
            query_parts.push(QueryPart::Key(key.to_string()));
            query_parts.push(QueryPart::Index(index));
        } else {
            query_parts.push(QueryPart::Key(part.to_string()));
        }
    }

    let mut result = value.clone();

    for part in query_parts {
        match part {
            QueryPart::Key(key) => {
                result = result[key.as_str()].clone();
            }
            QueryPart::Index(index) => {
                result = result[index].clone();
            }
        }
    }

    result.to_string()
}
