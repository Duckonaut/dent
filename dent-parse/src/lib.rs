mod error;
mod repr;
mod tokenizer;
pub use error::*;
pub use repr::*;
use tokenizer::{Token, Tokenizer};

#[cfg(test)]
mod tests;

use std::{
    collections::HashMap,
    io::Read,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

/// Alias for a trait object that represents a function that can be called from
/// Dent. The function takes a reference to a value and returns a value.
///
/// The function can be called from Dent using the `@` operator, after
/// being registered with `Dent::add_function`.
///
/// A Dent function can only take a single argument, for simplicity.
/// If you need to pass multiple arguments, you can use a list or dictionary.
///
/// # Examples
/// ```
/// use dent_parse::{Dent, Value, Function};
/// use std::collections::HashMap;
///
/// let mut functions: HashMap<String, Box<Function>> = HashMap::new();
/// functions.insert(
///     "sum".to_string(),
///     Box::new(move |value: &Value| -> Value {
///         let mut sum = 0;
///         if let Value::List(values) = value {
///             for value in values.iter() {
///                 if let Value::Int(i) = value {
///                     sum += i;
///                 }
///             }
///             Value::Int(sum)
///         } else if let Value::Int(i) = value {
///             Value::Int(*i)
///         } else {
///             Value::None
///         }
///     }),
/// );
/// let parser = Dent::new(functions);
///
/// assert_eq!(parser.parse("@sum {}"), Ok(Value::None));
/// assert_eq!(parser.parse("@sum 0"), Ok(Value::Int(0)));
/// assert_eq!(parser.parse("@sum [ 1 2 3 ]"), Ok(Value::Int(6)));
/// ```
pub type Function = dyn for<'a> Fn(&Value<'a>) -> Value<'a> + Send + Sync;

/// Main struct for parsing Dent.
///
/// This struct is used to parse Dent files and strings. It can also be used to
/// register functions that can be called from Dent.
///
/// # Examples
/// ```
/// use dent_parse::{Dent, Value};
/// use std::collections::HashMap;
///
/// let parser = Dent::default();
///
/// assert_eq!(parser.parse("foo"), Ok(Value::Str("foo")));
/// assert_eq!(parser.parse("[ 1 2 3 ]"), Ok(Value::List(vec![
///     Value::Int(1),
///     Value::Int(2),
///     Value::Int(3)
/// ])));
/// ```
pub struct Dent {
    internal: Arc<Mutex<DentInternal>>,
}

struct Import {
    src: &'static str,
    value: Value<'static>,
}

impl Drop for Import {
    fn drop(&mut self) {
        unsafe {
            let b = Box::from_raw(self.src as *const str as *mut str);
            std::mem::drop(b);
        }
    }
}

struct DentInternal {
    functions: HashMap<String, Arc<Function>>,
    import_map: HashMap<PathBuf, Import>,
}

struct ParserState<'s> {
    tokenizer: Tokenizer<'s>,
    token: Token<'s>,
}

impl<'s> ParserState<'s> {
    fn new(mut tokenizer: Tokenizer<'s>) -> Result<Self> {
        let token = tokenizer.next()?;
        Ok(ParserState { tokenizer, token })
    }

    fn next(&mut self) -> Result<()> {
        self.token = self.tokenizer.next()?;
        Ok(())
    }
}

impl Dent {
    /// Creates a new Dent parser with the given functions.
    ///
    /// If you want to use the built-in functions, you can use `Dent::default`,
    /// or call `Dent::add_builtins` after creating the parser.
    pub fn new(functions: HashMap<String, Box<Function>>) -> Dent {
        let functions = functions
            .into_iter()
            .map(|(k, v)| (k, Arc::new(v) as Arc<Function>))
            .collect();

        let internal = DentInternal {
            functions,
            import_map: HashMap::new(),
        };

        Dent {
            internal: Arc::new(Mutex::new(internal)),
        }
    }

    /// Adds the built-in functions to the parser.
    ///
    /// This function adds the following functions:
    /// - `import`: Imports a Dent file. Takes a string (file path) as an argument.
    /// - `merge`: Merges a list of lists or a list of dicts into a single list or dict.
    pub fn add_builtins(&mut self) {
        let internal = self.internal.clone();

        let outer_functions = &mut self.internal.lock().unwrap().functions;

        outer_functions.insert(
            "import".to_string(),
            Arc::new(move |value| {
                if let Value::Str(s) = value {
                    let path = Path::new(s);

                    let value = Self::import(internal.clone(), path);

                    match value {
                        Ok(v) => v,
                        Err(_) => Value::None,
                    }
                } else {
                    Value::None
                }
            }),
        );

        outer_functions.insert(
            "merge".to_string(),
            Arc::new(move |value| {
                // we want either a list of dicts or a list of lists
                if let Value::List(values) = value {
                    let mut result = Vec::new();
                    let mut is_dict = None;
                    for value in values.iter() {
                        if let Value::List(values) = value {
                            if is_dict.is_some() && is_dict.unwrap() {
                                return Value::None;
                            }
                            is_dict = Some(false);
                            result.extend(values.clone());
                        } else if let Value::Dict(values) = value {
                            if is_dict.is_some() && !is_dict.unwrap() {
                                return Value::None;
                            }
                            is_dict = Some(true);
                            result.push(Value::Dict(values.clone()));
                        }
                    }

                    match is_dict {
                        Some(true) => Value::Dict(
                            result
                                .into_iter()
                                .flat_map(|v| {
                                    if let Value::Dict(d) = v {
                                        d
                                    } else {
                                        panic!("Expected dict");
                                    }
                                })
                                .collect(),
                        ),
                        Some(false) => Value::List(result),
                        None => Value::None,
                    }
                } else {
                    Value::None
                }
            }),
        );
    }

    /// Adds a function to the parser.
    ///
    /// The function can be called from Dent using the `@` operator.
    /// The function takes a reference to a value and returns a value.
    /// The function can only take a single argument, for simplicity.
    ///
    /// # Examples
    /// ```
    /// use dent_parse::{Dent, Value};
    ///
    /// let mut dent = Dent::default();
    /// dent.add_function("count", Box::new(|value| {
    ///     if let Value::List(values) = value {
    ///         Value::Int(values.len() as i64)
    ///     } else {
    ///         Value::None
    ///     }
    /// }));
    /// assert_eq!(dent.parse("@count [ 1 2 3 ]"), Ok(Value::Int(3)));
    /// ```
    pub fn add_function(&mut self, name: &str, function: Box<Function>) {
        let function = Arc::new(function);

        let outer_functions = &mut self.internal.lock().unwrap().functions;

        outer_functions.insert(name.to_string(), function);
    }

    /// Parses a Dent string.
    ///
    /// The returned value is a zero-copy representation of the parsed Dent
    /// string. This means that the returned value borrows from the input string.
    ///
    /// If you want to parse a file, use `Dent::parse_file` instead.
    ///
    /// # Examples
    /// ```
    /// use dent_parse::{Dent, Value};
    ///
    /// let parser = Dent::default();
    ///
    /// assert_eq!(parser.parse("foo"), Ok(Value::Str("foo")));
    /// assert_eq!(parser.parse("2"), Ok(Value::Int(2)));
    /// assert_eq!(parser.parse("2.0"), Ok(Value::Float(2.0)));
    /// assert_eq!(parser.parse("true"), Ok(Value::Bool(true)));
    /// ```
    pub fn parse<'s>(&self, input: &'s str) -> Result<Value<'s>> {
        let tokenizer = Tokenizer::new(input);

        let mut state = ParserState::new(tokenizer)?;

        Self::parse_value(self.internal.clone(), &mut state)
    }

    /// Parses a Dent file.
    ///
    /// The returned value is a zero-copy representation of the parsed Dent. All strings
    /// in the returned value borrow from the input file.
    ///
    /// The file is read and stored in memory for the lifetime of the program.
    ///
    /// # Examples
    /// ```
    /// use dent_parse::{Dent, Value};
    /// use std::collections::HashMap;
    ///
    /// let parser = Dent::default();
    /// let value = parser.parse_file("examples/dent/dict.dent").unwrap();
    /// assert_eq!(value, Value::Dict(
    ///     vec![
    ///         ("name", Value::Str("Mario")),
    ///         (
    ///             "skills",
    ///             Value::List(vec![Value::Str("jumps"), Value::Str("grows")])
    ///         ),
    ///         ("age", Value::Int(35)),
    ///         ("alive", Value::Bool(true)),
    ///     ].into_iter().collect()
    /// ));
    /// ```
    pub fn parse_file<P: AsRef<Path>>(&self, path: P) -> Result<Value<'static>> {
        Self::import(self.internal.clone(), path)
    }

    fn import<P: AsRef<Path>>(
        internal: Arc<Mutex<DentInternal>>,
        path: P,
    ) -> Result<Value<'static>> {
        let path = if let Ok(path) = path.as_ref().canonicalize() {
            path
        } else {
            return Ok(Value::None);
        };

        let mut ilock = internal.lock().unwrap();
        let import_map = &mut ilock.import_map;
        if let Some(value) = import_map.get(&path) {
            return Ok(value.value.clone());
        }

        import_map.insert(
            path.clone(),
            Import {
                src: "",
                value: Value::None,
            },
        );

        drop(ilock);

        let mut file = std::fs::File::open(&path).unwrap();

        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let static_contents = Box::leak(contents.into_boxed_str());

        let tokenizer = Tokenizer::new(static_contents);

        let mut state = ParserState::new(tokenizer).unwrap();

        let value = Self::parse_value(internal.clone(), &mut state);

        let value = match value {
            Ok(v) => v,
            Err(_) => Value::None,
        };

        let mut ilock = internal.lock().unwrap();
        let import_map = &mut ilock.import_map;

        let i = import_map.get_mut(&path).unwrap();
        i.src = static_contents;
        i.value = value.clone();

        Ok(value)
    }

    fn parse_value<'s>(
        internal: Arc<Mutex<DentInternal>>,
        state: &mut ParserState<'s>,
    ) -> Result<Value<'s>> {
        let v = match state.token {
            Token::Eof => Ok(Value::None),
            Token::At => {
                state.next()?;
                if let Token::String(s) = state.token {
                    state.next()?;
                    let function = internal
                        .lock()
                        .unwrap()
                        .functions
                        .get(&s.to_string())
                        .cloned();
                    if let Some(function) = function {
                        let value = Self::parse_value(internal.clone(), state)?;
                        Ok(function(&value))
                    } else {
                        Err(Error::UnknownFunction(s.to_string()))
                    }
                } else {
                    Err(Error::UnexpectedToken(state.token.type_name()))
                }
            }
            Token::String(s) => {
                state.next()?;
                Ok(Value::Str(s))
            }
            Token::OpenBracket => {
                state.next()?;
                let mut values = Vec::new();
                while state.token != Token::CloseBracket {
                    if state.token == Token::Eof {
                        return Err(Error::UnexpectedEof);
                    }
                    values.push(Self::parse_value(internal.clone(), state)?);
                }
                state.next()?;
                Ok(Value::List(values))
            }
            Token::OpenBrace => {
                state.next()?;
                let mut values = HashMap::new();
                while state.token != Token::CloseBrace {
                    if state.token == Token::Eof {
                        return Err(Error::UnexpectedEof);
                    }
                    if let Token::String(s) = state.token {
                        state.next()?;
                        if state.token != Token::Colon {
                            return Err(Error::UnexpectedToken(state.token.type_name()));
                        }
                        state.next()?;
                        values.insert(s, Self::parse_value(internal.clone(), state)?);
                    } else {
                        return Err(Error::UnexpectedToken(state.token.type_name()));
                    }
                }
                state.next()?;
                Ok(Value::Dict(values))
            }
            Token::Number(n) => {
                state.next()?;
                if let Ok(i) = n.parse::<i64>() {
                    Ok(Value::Int(i))
                } else if let Ok(f) = n.parse::<f64>() {
                    Ok(Value::Float(f))
                } else {
                    panic!("Tokenizer returned invalid number: {}", n);
                }
            }
            Token::Bool(b) => {
                state.next()?;
                Ok(Value::Bool(b))
            }
            Token::Comment => {
                state.next()?;
                Self::parse_value(internal, state)
            }
            _ => Err(Error::UnexpectedToken(state.token.type_name())),
        };
        v
    }
}

impl Default for Dent {
    fn default() -> Self {
        let mut s = Self::new(HashMap::new());
        s.add_builtins();
        s
    }
}
