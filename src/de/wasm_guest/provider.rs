use core::fmt;

wit_bindgen_guest_rust::generate!({ world: "serde-deserializer-provider", no_std });
export_serde_deserializer_provider!(GuestsideDeserializerProvider);

use crate::{
    any::Any,
    intern::{intern_str_list, intern_string},
};

pub struct GuestsideDeserializerProvider {
    _private: (),
}

impl deserializer::Deserializer for GuestsideDeserializerProvider {
    fn test(
        x: serde_types::S128,
        y: serde_types::Usize,
    ) -> Result<(serde_types::U128, serde_types::Usize), serde_de::Unexpected> {
        deserialize::test(x, y)
    }
}

struct DeValue {
    _private: (),
}

struct DeError {
    inner: DeErrorVariants,
}

enum DeErrorVariants {
    Error {
        err: Any,
        display: String,
        debug: String,
    },
    Custom(String),
    InvalidType {
        unexp: serde_de::Unexpected,
        exp: String,
    },
    InvalidValue {
        unexp: serde_de::Unexpected,
        exp: String,
    },
    InvalidLength {
        len: usize,
        exp: String,
    },
    UnknownVariant {
        variant: String,
        expected: &'static [&'static str],
    },
    UnknownField {
        field: String,
        expected: &'static [&'static str],
    },
    MissingField {
        field: &'static str,
    },
    DuplicateField {
        field: &'static str,
    },
}

fn translate_serde_de_unexpected(unexp: &serde_de::Unexpected) -> serde::de::Unexpected {
    match unexp {
        serde_de::Unexpected::Bool(v) => serde::de::Unexpected::Bool(*v),
        serde_de::Unexpected::Unsigned(v) => serde::de::Unexpected::Unsigned(*v),
        serde_de::Unexpected::Signed(v) => serde::de::Unexpected::Signed(*v),
        serde_de::Unexpected::Float(v) => serde::de::Unexpected::Float(*v),
        serde_de::Unexpected::Char(v) => serde::de::Unexpected::Char(*v),
        serde_de::Unexpected::Str(v) => serde::de::Unexpected::Str(v),
        serde_de::Unexpected::Bytes(v) => serde::de::Unexpected::Bytes(v),
        serde_de::Unexpected::Unit => serde::de::Unexpected::Unit,
        serde_de::Unexpected::Option => serde::de::Unexpected::Option,
        serde_de::Unexpected::NewtypeStruct => serde::de::Unexpected::NewtypeStruct,
        serde_de::Unexpected::Seq => serde::de::Unexpected::Seq,
        serde_de::Unexpected::Map => serde::de::Unexpected::Map,
        serde_de::Unexpected::Enum => serde::de::Unexpected::Enum,
        serde_de::Unexpected::UnitVariant => serde::de::Unexpected::UnitVariant,
        serde_de::Unexpected::NewtypeVariant => serde::de::Unexpected::NewtypeVariant,
        serde_de::Unexpected::TupleVariant => serde::de::Unexpected::TupleVariant,
        serde_de::Unexpected::StructVariant => serde::de::Unexpected::StructVariant,
        serde_de::Unexpected::Other(v) => serde::de::Unexpected::Other(v),
    }
}

impl DeError {
    fn wrap<T: serde::de::Error>(err: T) -> Self {
        let display = format!("{err}");
        let debug = format!("{err:?}");

        // Safety: TODO
        Self {
            inner: DeErrorVariants::Error {
                err: unsafe { Any::new(err) },
                display,
                debug,
            },
        }
    }

    fn take<T: serde::de::Error>(self) -> Option<T> {
        match self.inner {
            // Safety: TODO
            DeErrorVariants::Error { err, .. } => unsafe { err.take() },
            DeErrorVariants::Custom(msg) => Some(serde::de::Error::custom(msg)),
            DeErrorVariants::InvalidType { unexp, exp } => Some(serde::de::Error::invalid_type(
                translate_serde_de_unexpected(&unexp),
                &&*exp,
            )),
            DeErrorVariants::InvalidValue { unexp, exp } => Some(serde::de::Error::invalid_value(
                translate_serde_de_unexpected(&unexp),
                &&*exp,
            )),
            DeErrorVariants::InvalidLength { len, exp } => {
                Some(serde::de::Error::invalid_length(len, &&*exp))
            }
            DeErrorVariants::UnknownVariant { variant, expected } => {
                Some(serde::de::Error::unknown_variant(&variant, expected))
            }
            DeErrorVariants::UnknownField { field, expected } => {
                Some(serde::de::Error::unknown_field(&field, expected))
            }
            DeErrorVariants::MissingField { field } => Some(serde::de::Error::missing_field(field)),
            DeErrorVariants::DuplicateField { field } => {
                Some(serde::de::Error::duplicate_field(field))
            }
        }
    }

    fn display(&self) -> String {
        match &self.inner {
            DeErrorVariants::Error { display, .. } => String::from(display),
            DeErrorVariants::Custom(msg) => String::from(msg),
            DeErrorVariants::InvalidType { unexp, exp } => {
                let unexp = translate_serde_de_unexpected(unexp);
                format!("invalid type: {unexp}, expected {exp}")
            }
            DeErrorVariants::InvalidValue { unexp, exp } => {
                let unexp = translate_serde_de_unexpected(unexp);
                format!("invalid value: {unexp}, expected {exp}")
            }
            DeErrorVariants::InvalidLength { len, exp } => {
                format!("invalid length {len}, expected {exp}")
            }
            DeErrorVariants::UnknownVariant { variant, expected } => {
                format!(
                    "unknown variant `{variant}`, {}",
                    ExpectedOneOf {
                        expected,
                        kinds: "variants"
                    }
                )
            }
            DeErrorVariants::UnknownField { field, expected } => {
                format!(
                    "unknown field `{field}`, {}",
                    ExpectedOneOf {
                        expected,
                        kinds: "fields"
                    }
                )
            }
            DeErrorVariants::MissingField { field } => {
                format!("missing field `{field}`")
            }
            DeErrorVariants::DuplicateField { field } => {
                format!("duplicate field `{field}`")
            }
        }
    }

    fn debug(&self) -> String {
        match &self.inner {
            DeErrorVariants::Error { debug, .. } => {
                format!("serde_wit::de::Error {{ err: {debug} }}")
            }
            DeErrorVariants::Custom(msg) => {
                format!("serde_wit::de::Error {{ err: Custom({msg}) }}")
            }
            DeErrorVariants::InvalidType { unexp, exp } => {
                let unexp = translate_serde_de_unexpected(unexp);
                format!("serde_wit::de::Error {{ err: InvalidType {{ unexp: {unexp:?}, exp: {exp:?} }} }}")
            }
            DeErrorVariants::InvalidValue { unexp, exp } => {
                let unexp = translate_serde_de_unexpected(unexp);
                format!("serde_wit::de::Error {{ err: InvalidValue {{ unexp: {unexp:?}, exp: {exp:?} }} }}")
            }
            DeErrorVariants::InvalidLength { len, exp } => {
                format!(
                    "serde_wit::de::Error {{ err: InvalidLength {{ len: {len}, exp: {exp:?} }} }}"
                )
            }
            DeErrorVariants::UnknownVariant { variant, expected } => {
                format!(
                    "serde_wit::de::Error {{ err: UnknownVariant {{ variant: {variant:?}, expected: {expected:?} }} }}"
                )
            }
            DeErrorVariants::UnknownField { field, expected } => {
                format!(
                    "serde_wit::de::Error {{ err: UnknownField {{ field: {field:?}, expected: {expected:?} }} }}"
                )
            }
            DeErrorVariants::MissingField { field } => {
                format!("serde_wit::de::Error {{ err: MissingField {{ field: {field:?} }} }}")
            }
            DeErrorVariants::DuplicateField { field } => {
                format!("serde_wit::de::Error {{ err: DuplicateField {{ field: {field:?} }} }}")
            }
        }
    }

    fn custom(msg: &str) -> Self {
        Self {
            inner: DeErrorVariants::Custom(String::from(msg)),
        }
    }

    fn invalid_type(unexp: serde_de::Unexpected, exp: &str) -> Self {
        Self {
            inner: DeErrorVariants::InvalidType {
                unexp,
                exp: String::from(exp),
            },
        }
    }

    fn invalid_value(unexp: serde_de::Unexpected, exp: &str) -> Self {
        Self {
            inner: DeErrorVariants::InvalidValue {
                unexp,
                exp: String::from(exp),
            },
        }
    }

    fn invalid_length(len: usize, exp: &str) -> Self {
        Self {
            inner: DeErrorVariants::InvalidLength {
                len,
                exp: String::from(exp),
            },
        }
    }

    fn unknown_variant(variant: &str, expected: &[&str]) -> Self {
        let expected = expected
            .iter()
            .map(|e| intern_string(String::from(*e)))
            .collect();

        Self {
            inner: DeErrorVariants::UnknownVariant {
                variant: String::from(variant),
                expected: intern_str_list(expected),
            },
        }
    }

    fn unknown_field(field: &str, expected: &[&str]) -> Self {
        let expected = expected
            .iter()
            .map(|e| intern_string(String::from(*e)))
            .collect();

        Self {
            inner: DeErrorVariants::UnknownField {
                field: String::from(field),
                expected: intern_str_list(expected),
            },
        }
    }

    fn missing_field(field: &str) -> Self {
        Self {
            inner: DeErrorVariants::MissingField {
                field: intern_string(String::from(field)),
            },
        }
    }

    fn duplicate_field(field: &str) -> Self {
        Self {
            inner: DeErrorVariants::DuplicateField {
                field: intern_string(String::from(field)),
            },
        }
    }
}

struct ExpectedOneOf {
    expected: &'static [&'static str],
    kinds: &'static str,
}

impl fmt::Display for ExpectedOneOf {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.expected {
            [] => write!(fmt, "there are no {}", self.kinds),
            [one] => write!(fmt, "expected `{one}`"),
            [one, two] => write!(fmt, "expected `{one}` or `{two}`"),
            _ => {
                write!(fmt, "expected one of ")?;
                for (i, alt) in self.expected.iter().enumerate() {
                    if i > 0 {
                        write!(fmt, ", ")?;
                    }
                    write!(fmt, "`{alt}`")?;
                }
                Ok(())
            }
        }
    }
}
