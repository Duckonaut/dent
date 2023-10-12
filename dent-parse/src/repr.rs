use std::{collections::HashMap, fmt::Display};

/// Value type returned by Dent.
///
/// Represents a value in Dent, which can be a string, integer, float, boolean,
/// list or dictionary.
///
/// Values are stored as references to the original string, so they are only
/// valid as long as the original string is valid. For values returned by parsing
/// files, whether by `Dent::parse_file` or the `@import` function, they should
/// be valid for the lifetime of the parser object.
///
/// # Accessing values
///
/// Values can be accessed using the `[]` operator, which takes either a string
/// or an integer. If the value is a dictionary, the string will be used as a
/// key. If the value is a list, the integer will be used as an index.
///
/// If a dictionary key or list index is not found, the value `Value::None` will
/// be returned during immutable access.
///
/// During mutable access, if a dictionary key is not found, a new entry will be
/// created with the value `Value::None`. If a list index is not found, the program
/// will panic.
///
/// # Examples
/// ```
/// use dent_parse::{Dent, Value};
///
/// let parser = Dent::default();
/// let value = parser.parse("{ foo: 1 }").unwrap();
/// assert_eq!(value["foo"], Value::Int(1));
/// ```
/// ```
/// use dent_parse::{Dent, Value};
///
/// let parser = Dent::default();
/// let value = parser.parse("[ 1 2 3 ]").unwrap();
/// assert_eq!(value[1], Value::Int(2));
/// ```
#[derive(Debug, PartialEq, Clone)]
pub enum Value<'s> {
    None,
    Str(&'s str),
    Int(i64),
    Float(f64),
    Bool(bool),
    List(Vec<Value<'s>>),
    Dict(HashMap<&'s str, Value<'s>>),
}

impl<'s> Value<'s> {
    /// Returns the underlying string value, if it is one
    pub fn as_str(&self) -> Option<&'s str> {
        match self {
            Value::Str(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the underlying integer value, if it is one
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            _ => None,
        }
    }

    /// Returns the underlying float value, if it is one
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// Returns the underlying boolean value, if it is one
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns the underlying list value, if it is one
    pub fn as_list(&self) -> Option<&Vec<Value<'s>>> {
        match self {
            Value::List(l) => Some(l),
            _ => None,
        }
    }

    /// Returns the underlying dictionary value, if it is one
    pub fn as_dict(&self) -> Option<&HashMap<&'s str, Value<'s>>> {
        match self {
            Value::Dict(d) => Some(d),
            _ => None,
        }
    }

    /// Returns true if the value is None
    pub fn is_none(&self) -> bool {
        matches!(self, Value::None)
    }

    /// Returns true if the value is a string
    pub fn is_str(&self) -> bool {
        matches!(self, Value::Str(_))
    }

    /// Returns true if the value is an integer
    pub fn is_int(&self) -> bool {
        matches!(self, Value::Int(_))
    }

    /// Returns true if the value is a float
    pub fn is_float(&self) -> bool {
        matches!(self, Value::Float(_))
    }

    /// Returns true if the value is a boolean
    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    /// Returns true if the value is a list
    pub fn is_list(&self) -> bool {
        matches!(self, Value::List(_))
    }

    /// Returns true if the value is a dictionary
    pub fn is_dict(&self) -> bool {
        matches!(self, Value::Dict(_))
    }

    /// Returns the length of the value, if it is a list or dictionary
    pub fn len(&self) -> Option<usize> {
        match self {
            Value::List(l) => Some(l.len()),
            Value::Dict(d) => Some(d.len()),
            _ => None,
        }
    }

    /// Returns true if the value is empty, if it is a list or dictionary
    pub fn is_empty(&self) -> bool {
        match self {
            Value::List(l) => l.is_empty(),
            Value::Dict(d) => d.is_empty(),
            _ => false,
        }
    }
}

impl<'i, 's> std::ops::Index<&'i str> for Value<'s> {
    type Output = Value<'s>;

    fn index(&self, key: &'i str) -> &Self::Output {
        match self {
            Value::Dict(d) => d.get(key).unwrap_or(&Value::None),
            _ => &Value::None,
        }
    }
}

impl<'s> std::ops::IndexMut<&'s str> for Value<'s> {
    fn index_mut(&mut self, key: &'s str) -> &mut Self::Output {
        match self {
            Value::Dict(d) => d.entry(key).or_insert(Value::None),
            _ => panic!("Cannot index non-dict value"),
        }
    }
}

impl<'s> std::ops::Index<usize> for Value<'s> {
    type Output = Value<'s>;

    fn index(&self, key: usize) -> &Self::Output {
        match self {
            Value::List(l) => l.get(key).unwrap_or(&Value::None),
            _ => &Value::None,
        }
    }
}

impl<'s> std::ops::IndexMut<usize> for Value<'s> {
    fn index_mut(&mut self, key: usize) -> &mut Self::Output {
        match self {
            Value::List(l) => match l.get_mut(key) {
                Some(v) => v,
                None => panic!("Index out of bounds"),
            },
            _ => panic!("Cannot index non-list value"),
        }
    }
}

impl<'s> Display for Value<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::None => write!(f, "none"),
            Value::Str(s) => write!(f, "{}", s),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Bool(b) => write!(f, "{}", b),
            Value::List(l) => {
                write!(f, "[")?;
                for v in l.iter() {
                    write!(f, " {}", v)?;
                }
                write!(f, " ]")
            }
            Value::Dict(d) => {
                write!(f, "{{")?;
                for (k, v) in d.iter() {
                    write!(f, " {}: {}", k, v)?;
                }
                write!(f, " }}")
            }
        }
    }
}
