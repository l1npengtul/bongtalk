use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter};

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
                    $dest::from(val)
                )
            }
        }

        #[inline]
        impl From<Value> for Option<$val> {
            fn from(val: Value) -> Option<$val> {
                if let Value::$src(v) = val {
                    return Some($val::from(v));
                }
                None
            }
        }

        #[inline]
        impl From<&Value> for Option<&$val> {
            fn from(val: &Value) -> Option<&$val> {
                if let Value::$src(v) = val {
                    return Some($val::from(v));
                }
                None
            }
        }

        #[inline]
        impl From<&Value> for Option<$val> {
            fn from(val: &Value) -> Option<$val> {
                if let Value::$src(v) = val {
                    return Some($val::from(v.clone()));
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
        }
    }
}

impl Value {}

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
