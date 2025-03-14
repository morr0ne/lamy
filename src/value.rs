use std::{
    collections::HashMap,
    fmt,
    iter::FromIterator,
    ops::{Index, IndexMut},
};

use serde::Deserialize;

/// Represents any YAML value
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum Value {
    /// Null value (`null`, `~`, or empty)
    Null,

    /// Boolean value (`true`, `false`, `yes`, `no`, etc.)
    Boolean(bool),

    /// Integer value
    Integer(i64),

    /// Float value
    Float(f64),

    /// String value
    String(String),

    /// Sequence (array) of values
    Sequence(Vec<Value>),

    /// Mapping (object) of string keys to values
    Mapping(HashMap<String, Value>),
}

impl Value {
    /// Returns true if this value is null
    pub const fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Returns true if this value is a boolean
    pub const fn is_boolean(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }

    /// Returns true if this value is an integer
    pub const fn is_integer(&self) -> bool {
        matches!(self, Value::Integer(_))
    }

    /// Returns true if this value is a float
    pub const fn is_float(&self) -> bool {
        matches!(self, Value::Float(_))
    }

    /// Returns true if this value is numeric (integer or float)
    pub const fn is_number(&self) -> bool {
        matches!(self, Value::Integer(_) | Value::Float(_))
    }

    /// Returns true if this value is a string
    pub const fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    /// Returns true if this value is a sequence
    pub const fn is_sequence(&self) -> bool {
        matches!(self, Value::Sequence(_))
    }

    /// Returns true if this value is a mapping
    pub const fn is_mapping(&self) -> bool {
        matches!(self, Value::Mapping(_))
    }

    // Value extraction methods

    /// Returns the boolean value if this is a boolean, or None otherwise
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns the i64 value if this is an integer, or None otherwise
    pub const fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Integer(i) => Some(*i),
            _ => None,
        }
    }

    /// Returns the f64 value if this is a float or integer, or None otherwise
    pub const fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            Value::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }

    /// Returns a reference to the string if this is a string, or None otherwise
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns a reference to the sequence if this is a sequence, or None otherwise
    pub const fn as_sequence(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Sequence(seq) => Some(seq),
            _ => None,
        }
    }

    /// Returns a mutable reference to the sequence if this is a sequence, or None otherwise
    pub fn as_sequence_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            Value::Sequence(seq) => Some(seq),
            _ => None,
        }
    }

    /// Returns a reference to the mapping if this is a mapping, or None otherwise
    pub const fn as_mapping(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::Mapping(map) => Some(map),
            _ => None,
        }
    }

    /// Returns a mutable reference to the mapping if this is a mapping, or None otherwise
    pub fn as_mapping_mut(&mut self) -> Option<&mut HashMap<String, Value>> {
        match self {
            Value::Mapping(map) => Some(map),
            _ => None,
        }
    }

    // Value conversion helpers

    /// Convert this value to a string representation
    pub fn to_string_lossy(&self) -> String {
        match self {
            Value::Null => "null".to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => s.clone(),
            Value::Sequence(_) => "[...]".to_string(),
            Value::Mapping(_) => "{...}".to_string(),
        }
    }

    /// Access a value in a sequence by index
    pub fn get_index(&self, index: usize) -> Option<&Value> {
        match self {
            Value::Sequence(seq) => seq.get(index),
            _ => None,
        }
    }

    /// Access a value in a mapping by key
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Mapping(map) => map.get(key),
            _ => None,
        }
    }

    /// Access a mutable value in a mapping by key
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        match self {
            Value::Mapping(map) => map.get_mut(key),
            _ => None,
        }
    }

    /// Insert a value into a mapping with the given key
    pub fn insert(&mut self, key: String, value: Value) -> Option<Value> {
        match self {
            Value::Mapping(map) => map.insert(key, value),
            _ => None,
        }
    }

    /// Remove a value from a mapping by key
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        match self {
            Value::Mapping(map) => map.remove(key),
            _ => None,
        }
    }

    /// Push a value onto a sequence
    pub fn push(&mut self, value: Value) -> bool {
        match self {
            Value::Sequence(seq) => {
                seq.push(value);
                true
            }
            _ => false,
        }
    }

    /// Convert this value to a boolean, with a default for non-booleans
    pub const fn as_bool_or(&self, default: bool) -> bool {
        match self.as_bool() {
            Some(b) => b,
            None => default,
        }
    }

    /// Convert this value to an i64, with a default for non-integers
    pub const fn as_i64_or(&self, default: i64) -> i64 {
        match self.as_i64() {
            Some(i) => i,
            None => default,
        }
    }

    /// Convert this value to an f64, with a default for non-floats
    pub const fn as_f64_or(&self, default: f64) -> f64 {
        match self.as_f64() {
            Some(f) => f,
            None => default,
        }
    }

    /// Convert this value to a string, with a default for non-strings
    pub fn as_str_or<'a>(&'a self, default: &'a str) -> &'a str {
        match self.as_str() {
            Some(s) => s,
            None => default,
        }
    }

    /// Get the length of this value (for strings, sequences, and mappings)
    pub fn len(&self) -> Option<usize> {
        match self {
            Value::String(s) => Some(s.len()),
            Value::Sequence(seq) => Some(seq.len()),
            Value::Mapping(map) => Some(map.len()),
            _ => None,
        }
    }

    /// Check if this value is empty (for strings, sequences, and mappings)
    pub fn is_empty(&self) -> Option<bool> {
        match self.len() {
            Some(len) => Some(len == 0),
            None => None,
        }
    }
}

// Display implementation for values
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "\"{}\"", s.replace('"', "\\\"")),
            Value::Sequence(_) => write!(f, "[...]"),
            Value::Mapping(_) => write!(f, "{{...}}"),
        }
    }
}

// Index implementation for accessing sequence elements
impl Index<usize> for Value {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Value::Sequence(seq) => &seq[index],
            _ => panic!("Expected a sequence to index into"),
        }
    }
}

// Mutable index implementation for accessing sequence elements
impl IndexMut<usize> for Value {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            Value::Sequence(seq) => &mut seq[index],
            _ => panic!("Expected a sequence to index into"),
        }
    }
}

// Index implementation for accessing mapping elements
impl Index<&str> for Value {
    type Output = Value;

    fn index(&self, key: &str) -> &Self::Output {
        match self {
            Value::Mapping(map) => map.get(key).expect("No entry found for key"),
            _ => panic!("Expected a mapping to index into"),
        }
    }
}

// Mutable index implementation for accessing mapping elements
impl IndexMut<&str> for Value {
    fn index_mut(&mut self, key: &str) -> &mut Self::Output {
        match self {
            Value::Mapping(map) => map.get_mut(key).expect("No entry found for key"),
            _ => panic!("Expected a mapping to index into"),
        }
    }
}

// Conversion from various types to Value
impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Boolean(b)
    }
}

impl From<i8> for Value {
    fn from(i: i8) -> Self {
        Value::Integer(i64::from(i))
    }
}

impl From<i16> for Value {
    fn from(i: i16) -> Self {
        Value::Integer(i64::from(i))
    }
}

impl From<i32> for Value {
    fn from(i: i32) -> Self {
        Value::Integer(i64::from(i))
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Integer(i)
    }
}

impl From<u8> for Value {
    fn from(i: u8) -> Self {
        Value::Integer(i64::from(i))
    }
}

impl From<u16> for Value {
    fn from(i: u16) -> Self {
        Value::Integer(i64::from(i))
    }
}

impl From<u32> for Value {
    fn from(i: u32) -> Self {
        Value::Integer(i64::from(i))
    }
}

impl From<u64> for Value {
    fn from(i: u64) -> Self {
        if i <= i64::MAX as u64 {
            Value::Integer(i as i64)
        } else {
            Value::Float(i as f64)
        }
    }
}

impl From<f32> for Value {
    fn from(f: f32) -> Self {
        Value::Float(f64::from(f))
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Float(f)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_owned())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl<T> From<Vec<T>> for Value
where
    T: Into<Value>,
{
    fn from(v: Vec<T>) -> Self {
        Value::Sequence(v.into_iter().map(Into::into).collect())
    }
}

impl<K, V> From<HashMap<K, V>> for Value
where
    K: ToString,
    V: Into<Value>,
{
    fn from(m: HashMap<K, V>) -> Self {
        Value::Mapping(
            m.into_iter()
                .map(|(k, v)| (k.to_string(), v.into()))
                .collect(),
        )
    }
}

impl<T> FromIterator<T> for Value
where
    T: Into<Value>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Value::Sequence(iter.into_iter().map(Into::into).collect())
    }
}

// Option conversion
impl<T> From<Option<T>> for Value
where
    T: Into<Value>,
{
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(v) => v.into(),
            None => Value::Null,
        }
    }
}

// Tuple conversions
impl<T> From<(T,)> for Value
where
    T: Into<Value>,
{
    fn from(t: (T,)) -> Self {
        Value::Sequence(vec![t.0.into()])
    }
}

impl<T1, T2> From<(T1, T2)> for Value
where
    T1: Into<Value>,
    T2: Into<Value>,
{
    fn from(t: (T1, T2)) -> Self {
        Value::Sequence(vec![t.0.into(), t.1.into()])
    }
}

impl<T1, T2, T3> From<(T1, T2, T3)> for Value
where
    T1: Into<Value>,
    T2: Into<Value>,
    T3: Into<Value>,
{
    fn from(t: (T1, T2, T3)) -> Self {
        Value::Sequence(vec![t.0.into(), t.1.into(), t.2.into()])
    }
}

// Implement a struct-like constructor for mappings
impl Value {
    /// Create a new mapping
    pub fn mapping() -> Self {
        Value::Mapping(HashMap::new())
    }

    /// Create a new sequence
    pub fn sequence() -> Self {
        Value::Sequence(Vec::new())
    }

    /// Create a null value
    pub const fn null() -> Self {
        Value::Null
    }

    /// Create a mapping with the given key-value pairs
    pub fn from_pairs(pairs: impl IntoIterator<Item = (String, Value)>) -> Self {
        Value::Mapping(HashMap::from_iter(pairs))
    }
}
