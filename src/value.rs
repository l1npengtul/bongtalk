use lending_iterator::LendingIterator;
use rhai::{Blob, Dynamic, Map};
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

macro_rules! from_into_cast {
    (
        $(
        (
        $src:tt,
        $dest:ty,
        $( $val:ty ),*
        )
        ),+
    ) => {
        $(
        $(
        #[inline]
        impl From<$val> for Value {
            fn from(val: $val) -> Value {
                Value::$src(
                    (val)
                )
            }
        }

        #[inline]
        impl From<Value> for Option<$val> {
            fn from(val: Value) -> Option<$val> {
                if let Value::$src(v) = val {
                    return Some(<$val>::from(v));
                }
                None
            }
        }

        #[inline]
        impl From<&Value> for Option<&$val> {
            fn from(val: &Value) -> Option<&$val> {
                if let Value::$src(v) = val {
                    return Some(<$val>::from(v));
                }
                None
            }
        }

        #[inline]
        impl From<&Value> for Option<$val> {
            fn from(val: &Value) -> Option<$val> {
                if let Value::$src(v) = val {
                    return Some(<$val>::from(v.clone()));
                }
                None
            }
        }

        )*

        #[inline]
        impl From<$dest> for Value {
            fn from(val: $dest) -> Value {
                Value::$src(
                    val
                )
            }
        }

        #[inline]
        impl From<&$dest> for Value {
            fn from(val: &$dest) -> Value {
                Value::$src(
                    val.clone()
                )
            }
        }

        #[inline]
        impl From<Value> for Option<$dest> {
            fn from(val: Value) -> Option<$dest> {
                if let Value::$src(v) = val {
                    return Some(v);
                }
                None
            }
        }

        #[inline]
        impl From<&Value> for Option<&$dest> {
            fn from(val: &Value) -> Option<&$dest> {
                if let Value::$src(v) = val {
                    return Some(v);
                }
                None
            }
        }

        #[inline]
        impl From<&Value> for Option<$dest> {
            fn from(val: &Value) -> Option<$dest> {
                if let Value::$src(v) = val {
                    return Some(v.clone());
                }
                None
            }
        }

        )+
    };
}

macro_rules! as_type_impl_copy {
    ( $( ($name:ident, $dest:ty) ),* ) => {
        $(
        paste::paste! {
            impl Value {
                pub fn [<as_ $name>](&self) -> Option<$dest> {
                    $dest::from(&self)
                }
            }
        }
        )*
    };
}

macro_rules! as_type_impl_ref {
    ( $(($name:ident, $dest:ty)),* ) => {
        $(
        paste::paste! {
            impl Value {
                pub fn [<as_ $name>](&self) -> Option<&$dest> {
                    $dest::from(&self)
                }
            }
        }
        )*
    };
}

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Eq)]
pub enum Value {
    Null,
    Bool(bool),
    Float(f64),
    Int(i64),
    String(String),
    Array(Vec<Value>),
    Blob(Vec<u8>),
    Map(BTreeMap<String, Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Value::Null => {
                write!(f, "Null")
            }
            Value::Bool(b) => {
                write!(f, "{b}")
            }
            Value::Float(f) => {
                write!(f, "{f}")
            }
            Value::Int(i) => {
                write!(f, "{i}")
            }
            Value::String(s) => {
                write!(f, "{s}")
            }
            Value::Array(arr) => {
                write!(f, "{arr:?}")
            }
            Value::Map(map) => {
                write!(f, "{map:?}")
            }
            Value::Blob(b) => {
                write!(f, "{b:#x?}")
            }
        }
    }
}

impl Value {}

impl From<Value> for Dynamic {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Dynamic::UNIT,
            Value::Bool(b) => Dynamic::from_bool(b),
            Value::Float(f) => Dynamic::from_float(f),
            Value::Int(i) => Dynamic::from_int(i),
            Value::String(s) => Dynamic::from_str(&s),
            Value::Array(a) => a.into(),
            Value::Blob(b) => Dynamic::from_blob(Blob::from(b)),
            Value::Map(m) => m.into(),
        }
    }
}
impl From<&Value> for Dynamic {
    fn from(value: &Value) -> Self {
        match value {
            Value::Null => Dynamic::UNIT,
            Value::Bool(b) => Dynamic::from_bool(*b),
            Value::Float(f) => Dynamic::from_float(*f),
            Value::Int(i) => Dynamic::from_int(*i),
            Value::String(s) => Dynamic::from_str(s),
            Value::Array(a) => a.into(),
            Value::Blob(b) => Dynamic::from_blob(Blob::from(b.as_slice())),
            Value::Map(m) => m.into(),
        }
    }
}

impl From<Dynamic> for Value {
    fn from(value: Dynamic) -> Self {
        Value::from(&value)
    }
}
impl From<&Dynamic> for Value {
    fn from(value: &Dynamic) -> Self {
        if value.is_unit() {
            Value::Null
        } else if value.is_int() {
            Value::Int(value.as_int().unwrap())
        } else if value.is_float() {
            Value::Float(value.as_float().unwrap())
        } else if value.is_bool() {
            Value::Bool(value.as_bool().unwrap())
        } else if value.is_char() {
            Value::String(value.as_char().unwrap().to_string())
        } else if value.is_string() {
            Value::String(value.into_immutable_string().unwrap().to_string())
        } else if value.is_array() {
            Value::Array(
                value
                    .into_array()
                    .map(|x| {
                        x.into_iter()
                            .map(|v| Value::from(v))
                            .collect::<Vec<Value>>()
                    })
                    .unwrap(),
            )
        } else if value.is_blob() {
            Value::Blob(value.into_blob().unwrap())
        } else if value.is_map() {
            let map = value.read_lock::<Map>().unwrap();
            Value::Map(
                map.into_iter()
                    .map(|(k, v)| (k.to_string(), Value::from(v)))
                    .collect(),
            )
        }
    }
}

from_into_cast!(
    (Bool, bool,),
    (Float, f64, f32),
    (Int, i64, i8, u8, i16, u16, i32, u32, u64, i128, u128, isize, usize),
    (String, String, &str, Cow<'_, str>),
    (Array, Vec<Value>, &[Value], Cow<'_, [Value]>),
    (
        Map,
        BTreeMap<String, Value>,
        &[(String, Value)],
        HashMap<String, Value, _>
    )
);

as_type_impl_copy!((bool, bool), (float, f64), (int, i64));

as_type_impl_ref!(
    (string, String),
    (array, Vec<Value>),
    (map, BTreeMap<String, Value>)
);
