#[macro_use]
extern crate serde;

pub mod entry;

pub use serde::de::{Deserialize, Visitor, SeqAccess, Error};
pub use std::fmt;
pub use std::marker::PhantomData;
pub use std::ops::Deref;

#[macro_export]
macro_rules! tagged_string {
    ($name:ident, $discard_literal:expr) => {
        #[derive(Debug, Clone, Serialize)]
        pub struct $name(String);

        impl<'de> serde::de::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                deserializer.deserialize_seq(TaggedStringVisitor)
            }
        }

        struct TaggedStringVisitor;

        impl<'de> serde::de::Visitor<'de> for TaggedStringVisitor {
            type Value = $name;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("an array with a tag and a string")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<$name, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                // Get and validate the tag.
                let tag: Option<String> = seq.next_element()?;
                let tag = tag.ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

                // Optionally, check that the tag matches the expected literal.
                if tag != $discard_literal {
                    return Err(serde::de::Error::custom(format!(
                        "expected tag `{}`, found `{}`",
                        $discard_literal, tag
                    )));
                }

                // Get the next element as the defining string.
                let value: Option<String> = seq.next_element()?;
                let value = value.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok($name(value))
            }
        }
    };
}


#[macro_export]
macro_rules! tagged_vec {
    ($name:ident, $tag:ty, $inner:ty) => {
        #[derive(Debug, Clone, Serialize)]
        pub struct $name(pub Vec<$inner>);

        impl Deref for $name {
            type Target = Vec<$inner>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_seq(TagVisitor::<$tag, $inner, $name> {
                    marker: PhantomData,
                })
            }
        }

        struct TagVisitor<T, U, V> {
            marker: PhantomData<(T, U, V)>,
        }

        impl<'de, T, U, V> Visitor<'de> for TagVisitor<T, U, V>
        where
            T: Deserialize<'de>,
            U: Deserialize<'de>,
            V: From<Vec<U>>,
        {
            type Value = V;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an array with a tag and an array of inner elements")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<V, A::Error>
            where
                A: SeqAccess<'de>,
            {
                // Skip the tag element.
                let _tag: Option<T> = seq.next_element()?;
                // Deserialize the inner vector.
                let v: Vec<U> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(V::from(v))
            }
        }

        // Implement From<Vec<$inner>> for $name so that the TagVisitor can construct it.
        impl From<Vec<$inner>> for $name {
            fn from(v: Vec<$inner>) -> Self {
                $name(v)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
