//! Python-shaped JSON values: insertion-ordered dicts, integers of any size distinct from floats, and Python's `==`.

use num_bigint::BigInt;
use num_traits::{ToPrimitive, Zero};
use std::collections::HashMap;
use std::hash::Hash;

/// An insertion-ordered map with Python dict semantics: assigning an existing key keeps its position and replaces
/// its value; removing a key closes the gap.
#[derive(Clone, Debug)]
pub struct OMap<K: Clone + Eq + Hash, V> {
    keys: Vec<K>,
    vals: Vec<V>,
    index: HashMap<K, usize>,
}

impl<K: Clone + Eq + Hash, V> Default for OMap<K, V> {
    fn default() -> Self {
        OMap { keys: Vec::new(), vals: Vec::new(), index: HashMap::new() }
    }
}

impl<K: Clone + Eq + Hash, V> OMap<K, V> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn len(&self) -> usize {
        self.keys.len()
    }
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }
    pub fn contains_key(&self, key: &K) -> bool {
        self.index.contains_key(key)
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        self.index.get(key).map(|&at| &self.vals[at])
    }
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match self.index.get(key) {
            Some(&at) => Some(&mut self.vals[at]),
            None => None,
        }
    }
    /// `d[key] = value`.
    pub fn insert(&mut self, key: K, value: V) {
        if let Some(&at) = self.index.get(&key) {
            self.vals[at] = value;
        } else {
            self.index.insert(key.clone(), self.keys.len());
            self.keys.push(key);
            self.vals.push(value);
        }
    }
    /// `d.setdefault(key, make())`, returning the stored value.
    pub fn entry_or_insert_with(&mut self, key: K, make: impl FnOnce() -> V) -> &mut V {
        if !self.index.contains_key(&key) {
            self.insert(key.clone(), make());
        }
        let at = self.index[&key];
        &mut self.vals[at]
    }
    /// `d.pop(key)`.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let at = self.index.remove(key)?;
        self.keys.remove(at);
        let value = self.vals.remove(at);
        for (offset, k) in self.keys.iter().enumerate().skip(at) {
            self.index.insert(k.clone(), offset);
        }
        Some(value)
    }
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.keys.iter()
    }
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.vals.iter()
    }
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.vals.iter_mut()
    }
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.keys.iter().zip(self.vals.iter())
    }
    pub fn first(&self) -> Option<(&K, &V)> {
        self.keys.first().map(|k| (k, &self.vals[0]))
    }
}

pub type Dict = OMap<String, Value>;

/// A JSON value as Python holds it after `json.loads`, or as the importer builds it.
#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Bool(bool),
    Int(BigInt),
    Float(f64),
    Str(String),
    List(Vec<Value>),
    Dict(Dict),
    /// The sidecar parser's `_BAD` sentinel; never part of a result.
    #[doc(hidden)]
    Bad,
}

impl Value {
    pub fn str(text: impl Into<String>) -> Value {
        Value::Str(text.into())
    }
    pub fn int(value: i64) -> Value {
        Value::Int(BigInt::from(value))
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Str(text) => Some(text),
            _ => None,
        }
    }
    pub fn as_dict(&self) -> Option<&Dict> {
        match self {
            Value::Dict(dict) => Some(dict),
            _ => None,
        }
    }
    pub fn as_dict_mut(&mut self) -> Option<&mut Dict> {
        match self {
            Value::Dict(dict) => Some(dict),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&Vec<Value>> {
        match self {
            Value::List(items) => Some(items),
            _ => None,
        }
    }
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Float(value) => Some(*value),
            _ => None,
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(value) => Some(*value),
            _ => None,
        }
    }
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }
    pub fn is_bad(&self) -> bool {
        matches!(self, Value::Bad)
    }
    /// `d.get(key)` on a dict, None otherwise (or when absent).
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.as_dict().and_then(|dict| dict.get(&key.to_string()))
    }
    /// `d.get(key)` with Python's None for an absent key.
    pub fn get_or_null(&self, key: &str) -> &Value {
        static NULL: Value = Value::Null;
        self.get(key).unwrap_or(&NULL)
    }
    /// `key in d` for a dict.
    pub fn has(&self, key: &str) -> bool {
        self.as_dict().map(|dict| dict.contains_key(&key.to_string())).unwrap_or(false)
    }
    /// Python's `type(value).__name__`.
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Null => "NoneType",
            Value::Bool(_) => "bool",
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::Str(_) => "str",
            Value::List(_) => "list",
            Value::Dict(_) => "dict",
            Value::Bad => "object",
        }
    }
    /// Python truthiness.
    pub fn truthy(&self) -> bool {
        match self {
            Value::Null | Value::Bad => false,
            Value::Bool(value) => *value,
            Value::Int(value) => !value.is_zero(),
            Value::Float(value) => *value != 0.0,
            Value::Str(text) => !text.is_empty(),
            Value::List(items) => !items.is_empty(),
            Value::Dict(dict) => !dict.is_empty(),
        }
    }
}

/// A number of Python's numeric tower (bool, int, float) as its exact value, for cross-type `==`.
enum Numeric<'a> {
    Int(BigInt),
    IntRef(&'a BigInt),
    Float(f64),
}

fn numeric(value: &Value) -> Option<Numeric<'_>> {
    match value {
        Value::Bool(flag) => Some(Numeric::Int(BigInt::from(*flag as u8))),
        Value::Int(number) => Some(Numeric::IntRef(number)),
        Value::Float(number) => Some(Numeric::Float(*number)),
        _ => None,
    }
}

fn int_eq_float(int: &BigInt, float: f64) -> bool {
    if !float.is_finite() || float.fract() != 0.0 {
        return false;
    }
    match crate::rational::Q::from_f64(float) {
        Some(exact) => exact.is_integer() && &exact.numer() == int,
        None => false,
    }
}

/// Python's `a == b` for JSON-shaped values: bool, int and float compare by value (True == 1 == 1.0, -0.0 == 0.0,
/// NaN != NaN); dicts by key set and values, in any order; lists item by item.
pub fn py_eq(a: &Value, b: &Value) -> bool {
    if let (Some(x), Some(y)) = (numeric(a), numeric(b)) {
        return match (x, y) {
            (Numeric::Float(p), Numeric::Float(q)) => p == q,
            (Numeric::Float(p), Numeric::Int(q)) | (Numeric::Int(q), Numeric::Float(p)) => int_eq_float(&q, p),
            (Numeric::Float(p), Numeric::IntRef(q)) | (Numeric::IntRef(q), Numeric::Float(p)) => int_eq_float(q, p),
            (Numeric::Int(p), Numeric::Int(q)) => p == q,
            (Numeric::Int(p), Numeric::IntRef(q)) | (Numeric::IntRef(q), Numeric::Int(p)) => &p == q,
            (Numeric::IntRef(p), Numeric::IntRef(q)) => p == q,
        };
    }
    match (a, b) {
        (Value::Null, Value::Null) | (Value::Bad, Value::Bad) => true,
        (Value::Str(x), Value::Str(y)) => x == y,
        (Value::List(x), Value::List(y)) => x.len() == y.len() && x.iter().zip(y).all(|(p, q)| py_eq(p, q)),
        (Value::Dict(x), Value::Dict(y)) => {
            x.len() == y.len() && x.iter().all(|(key, value)| y.get(key).map(|other| py_eq(value, other)).unwrap_or(false))
        }
        _ => false,
    }
}

/// `value in (allowed strings...)` for a tuple of strings: only an equal string is a member.
pub fn str_in(value: &Value, allowed: &[&str]) -> bool {
    matches!(value, Value::Str(text) if allowed.contains(&text.as_str()))
}

/// An i64 view of a Python int, when it fits.
pub fn small_int(value: &Value) -> Option<i64> {
    match value {
        Value::Int(number) => number.to_i64(),
        _ => None,
    }
}

/// The deepest container nesting a public entry point takes as a [`Value`] (a value inside one list or dict is at
/// depth 1). The port's recursive walks (equality, formatting, cloning) stay within any build's stack below it, the
/// wasm build's included; the sidecar JSON parser has the same bound ([`crate::MAX_SIDECAR_NESTING`]).
pub const MAX_VALUE_NESTING: usize = 64;

/// What [`screen`] finds wrong with a value given to a public entry point, found without recursion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unrepresentable {
    /// Containers nest deeper than [`MAX_VALUE_NESTING`].
    TooDeep,
    /// The sidecar parser's internal sentinel, which no Python value is.
    Sentinel,
}

/// Screens `value` iteratively, so a hostile depth cannot overflow the stack before it is refused.
pub fn screen(value: &Value) -> Result<(), Unrepresentable> {
    let mut stack: Vec<(&Value, usize)> = vec![(value, 0)];
    while let Some((item, depth)) = stack.pop() {
        match item {
            Value::Bad => return Err(Unrepresentable::Sentinel),
            Value::List(items) => {
                if depth >= MAX_VALUE_NESTING {
                    return Err(Unrepresentable::TooDeep);
                }
                stack.extend(items.iter().map(|child| (child, depth + 1)));
            }
            Value::Dict(dict) => {
                if depth >= MAX_VALUE_NESTING {
                    return Err(Unrepresentable::TooDeep);
                }
                stack.extend(dict.values().map(|child| (child, depth + 1)));
            }
            _ => {}
        }
    }
    Ok(())
}

/// The `Unsupported` reason for an unrepresentable value given as `name`.
pub fn unrepresentable(problem: Unrepresentable, name: &str) -> String {
    match problem {
        Unrepresentable::Sentinel => format!("the {name} holds a value that is not a JSON value"),
        Unrepresentable::TooDeep => {
            format!("the {name} nests containers deeper than {MAX_VALUE_NESTING}, which this port does not follow")
        }
    }
}

/// Build a dict in insertion order: `dict![("a", v), ...]`.
#[macro_export]
#[doc(hidden)]
macro_rules! dict {
    ($(($key:expr, $value:expr)),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut map = $crate::value::Dict::new();
        $( map.insert(String::from($key), $value); )*
        $crate::value::Value::Dict(map)
    }};
}
