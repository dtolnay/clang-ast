use crate::{LastLocation, Node, NodeVisitor};
use serde::de::{Deserialize, DeserializeSeed, Deserializer, SeqAccess, Visitor};
use std::fmt;
use std::marker::PhantomData;

pub(crate) struct Inner<'a, T> {
    marker: PhantomData<fn() -> T>,
    last_loc: &'a mut LastLocation,
}

impl<'a, T> Inner<'a, T> {
    pub(crate) fn new(last_loc: &'a mut LastLocation) -> Self {
        Inner {
            marker: PhantomData,
            last_loc,
        }
    }
}

impl<'de, 'a, T> DeserializeSeed<'de> for Inner<'a, T>
where
    T: Deserialize<'de>,
{
    type Value = Vec<Node<T>>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(self)
    }
}

impl<'de, 'a, T> Visitor<'de> for Inner<'a, T>
where
    T: Deserialize<'de>,
{
    type Value = Vec<Node<T>>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an array")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut inner = Vec::new();

        while let Some(node) = seq.next_element_seed(NodeVisitor::new(self.last_loc))? {
            inner.push(node);
        }

        Ok(inner)
    }
}
