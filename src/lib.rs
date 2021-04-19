mod deserializer;
mod id;
mod kind;
mod loc;

extern crate serde;

use crate::deserializer::NodeDeserializer;
use crate::kind::AnyKind;
use serde::de::{Deserializer, MapAccess, Visitor};
use serde::Deserialize;
use std::fmt;
use std::marker::PhantomData;

pub use crate::id::Id;
pub use crate::kind::Kind;
pub use crate::loc::{BareSourceLocation, IncludedFrom, SourceLocation, SourceRange};

#[derive(Debug)]
pub struct Node<T> {
    pub id: Id,
    pub kind: T,
    pub inner: Vec<Node<T>>,
}

struct NodeVisitor<T> {
    marker: PhantomData<fn() -> T>,
}

impl<'de, T> Visitor<'de> for NodeVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = Node<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("clang syntax tree node")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum FirstField {
            Id,
            Kind,
            Inner,
        }

        let mut id = None;
        let mut inner = Vec::new();
        let kind = loop {
            match map.next_key()? {
                None => {
                    let kind = AnyKind::Kind(Kind::null);
                    let deserializer = NodeDeserializer::new(kind, &mut inner, map);
                    break T::deserialize(deserializer)?;
                }
                Some(FirstField::Id) => {
                    if id.is_some() {
                        return Err(serde::de::Error::duplicate_field("id"));
                    }
                    id = Some(map.next_value()?);
                }
                Some(FirstField::Kind) => {
                    let kind: AnyKind = map.next_value()?;
                    let deserializer = NodeDeserializer::new(kind, &mut inner, map);
                    break T::deserialize(deserializer)?;
                }
                Some(FirstField::Inner) => {
                    return Err(serde::de::Error::missing_field("kind"));
                }
            }
        };

        let id = id.unwrap_or_default();

        Ok(Node { id, kind, inner })
    }
}

impl<'de, T> Deserialize<'de> for Node<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let marker = PhantomData;
        let visitor = NodeVisitor { marker };
        deserializer.deserialize_map(visitor)
    }
}
