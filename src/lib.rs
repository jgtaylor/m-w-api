#[macro_use]
extern crate serde;

pub mod entry;

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
                let value = value.ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                Ok($name(value))
            }
        }

        impl From<$name> for String {
            fn from(tagged_string: $name) -> Self {
                tagged_string.0
            }
        }

        impl<'a> From<&'a $name> for &'a str {
            fn from(tagged_string: &'a $name) -> Self {
                &tagged_string.0
            }
        }
    };
}


#[macro_export]
macro_rules! tagged_vec {
    ($name:ident, $tag:ty, $inner:ty) => {
        #[derive(Debug, Clone, Serialize)]
        pub struct $name(pub Vec<$inner>);

        impl std::ops::Deref for $name {
            type Target = Vec<$inner>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'de> serde::de::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                deserializer.deserialize_seq(TagVisitor::<$tag, $inner, $name> {
                    marker: std::marker::PhantomData,
                })
            }
        }

        struct TagVisitor<T, U, V> {
            marker: std::marker::PhantomData<(T, U, V)>,
        }

        impl<'de, T, U, V> serde::de::Visitor<'de> for TagVisitor<T, U, V>
        where
            T: serde::de::Deserialize<'de>,
            U: serde::de::Deserialize<'de>,
            V: From<Vec<U>>,
        {
            type Value = V;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("an array with a tag and an array of inner elements")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<V, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                // Skip the tag element.
                let _tag: Option<T> = seq.next_element()?;
                // Deserialize the inner vector.
                let v: Vec<U> = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
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

#[macro_export]
macro_rules! tagged_object {
    ($name:ident, $tag:ty, $inner:ty) => {
        #[derive(Debug, Clone, Serialize)]
        pub struct $name {
            tag: Option<$tag>,
            object: $inner,
        }

        impl<'de> serde::de::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                deserializer.deserialize_struct(stringify!($inner), &[], TaggedObjectVisitor::<$tag, $inner>)
            }
        }

        struct TaggedObjectVisitor<T, U> {
            marker: std::marker::PhantomData<(T, U)>, // T is the tag type and U is the inner object type
        }

        impl<'de, 'a, T, U> serde::de::Visitor<'de> for TaggedObjectVisitor<T, U>
        where
            T: serde::de::Deserialize<'de>,
            U: serde::de::Deserialize<'de>,
        {
            type Value = $name;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an object with an optional tag and a main object")
            }

            fn visit_map<A>(self, mut map: A) -> Result<$name, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut tag = None;
                let mut value = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "tag" => {
                            if value.is_some() {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            tag = Some(map.next_value()?);
                        }
                        _ => {
                            if value.is_none() {
                                value = Some(map.next_value()?);
                            } else {
                                return Err(serde::de::Error::unknown_field(&key, &[]));
                            }
                        }
                    }
                }

                Ok($name { tag, object: value.unwrap_or_default() })
            }
        }

        impl From<$inner> for $name {
            fn from(object: $inner) -> Self {
                $name {
                    tag: None,
                    object,
                }
            }
        }

        // Deref implementation to access inner fields directly
        impl std::ops::Deref for $name {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.object
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.object
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
