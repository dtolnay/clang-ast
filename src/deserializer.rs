use crate::kind::{AnyKind, Kind, SometimesBorrowedStrDeserializer};
use crate::Node;
use serde::de::value::BorrowedStrDeserializer;
use serde::de::{
    Deserialize, DeserializeSeed, Deserializer, EnumAccess, Error, Expected, IgnoredAny, MapAccess,
    Unexpected, VariantAccess, Visitor,
};
use serde::forward_to_deserialize_any;
use std::error::Error as StdError;
use std::fmt::{self, Display};
use std::marker::PhantomData;

pub(crate) struct NodeDeserializer<'de, 'a, T, M> {
    kind: &'a AnyKind<'de>,
    inner: &'a mut Vec<Node<T>>,
    map: M,
    has_kind: bool,
}

impl<'de, 'a, T, M> NodeDeserializer<'de, 'a, T, M> {
    pub(crate) fn new(kind: &'a AnyKind<'de>, inner: &'a mut Vec<Node<T>>, map: M) -> Self {
        let has_kind = match kind {
            AnyKind::Kind(Kind::null) => false,
            _ => true,
        };
        NodeDeserializer {
            kind,
            inner,
            map,
            has_kind,
        }
    }
}

impl<'de, 'a, T, M> Deserializer<'de> for NodeDeserializer<'de, 'a, T, M>
where
    T: Deserialize<'de>,
    M: MapAccess<'de>,
{
    type Error = M::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let _ = name;
        let expected = self.kind.as_str();
        let mut expects_the_unexpected = None;
        for &variant in variants {
            if variant == expected {
                return visitor.visit_enum(self);
            } else if variant == "Unknown" || variant == "Other" {
                expects_the_unexpected = Some(variant);
            }
        }
        if let Some(unexpected) = expects_the_unexpected {
            visitor.visit_enum(UnknownNode {
                name: unexpected,
                node: self,
            })
        } else {
            visitor.visit_enum(self)
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let _ = name;
        self.deserialize_unit(visitor)
    }

    fn deserialize_unit<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.ignore()?;
        visitor.visit_unit()
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option newtype_struct seq tuple tuple_struct map struct
        identifier ignored_any
    }
}

impl<'de, 'a, T, M> EnumAccess<'de> for NodeDeserializer<'de, 'a, T, M>
where
    T: Deserialize<'de>,
    M: MapAccess<'de>,
{
    type Error = M::Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let deserializer = match &self.kind {
            AnyKind::Kind(kind) => SometimesBorrowedStrDeserializer::borrowed(kind.as_str()),
            AnyKind::Borrowed(kind) => SometimesBorrowedStrDeserializer::borrowed(kind),
            AnyKind::Owned(kind) => SometimesBorrowedStrDeserializer::transient(kind),
        };
        let value = seed.deserialize(deserializer)?;
        Ok((value, self))
    }
}

impl<'de, 'a, T, M> VariantAccess<'de> for NodeDeserializer<'de, 'a, T, M>
where
    T: Deserialize<'de>,
    M: MapAccess<'de>,
{
    type Error = M::Error;

    fn unit_variant(mut self) -> Result<(), Self::Error> {
        self.ignore()?;
        Ok(())
    }

    fn newtype_variant_seed<V>(self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(NodeFieldsDeserializer { node: self })
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let _ = len;
        let _ = visitor;
        let kind = self.kind.as_str();
        let expected = ExpectedTupleVariant { kind };
        Err(Error::invalid_type(Unexpected::StructVariant, &expected))
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let _ = fields;
        let kind = self.kind;
        match visitor.visit_map(NodeFieldsDeserializer { node: self }) {
            Ok(value) => Ok(value),
            Err(error) => Err(error.with_kind(kind)),
        }
    }
}

impl<'de, 'a, T, M> MapAccess<'de> for NodeDeserializer<'de, 'a, T, M>
where
    T: Deserialize<'de>,
    M: MapAccess<'de>,
{
    type Error = M::Error;

    fn next_key_seed<K>(&mut self, mut seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.has_kind {
            let deserializer = BorrowedStrDeserializer::new("kind");
            seed.deserialize(deserializer).map(Some)
        } else {
            loop {
                seed = match self.map.next_key_seed(NodeFieldSeed {
                    kind: self.kind,
                    seed,
                })? {
                    None => return Ok(None),
                    Some(NodeField::Inner(seed)) => {
                        *self.inner = self.map.next_value()?;
                        seed
                    }
                    Some(NodeField::Delegate(value)) => return Ok(Some(value)),
                };
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        if self.has_kind {
            let deserializer = match &self.kind {
                AnyKind::Kind(kind) => SometimesBorrowedStrDeserializer::borrowed(kind.as_str()),
                AnyKind::Borrowed(kind) => SometimesBorrowedStrDeserializer::borrowed(kind),
                AnyKind::Owned(kind) => SometimesBorrowedStrDeserializer::transient(kind),
            };
            let value = seed.deserialize(deserializer);
            self.has_kind = false;
            value
        } else {
            self.map.next_value_seed(seed)
        }
    }
}

impl<'de, 'a, T, M> NodeDeserializer<'de, 'a, T, M>
where
    T: Deserialize<'de>,
    M: MapAccess<'de>,
{
    fn ignore(&mut self) -> Result<(), M::Error> {
        while let Some(node_field) = self.map.next_key_seed(NodeFieldSeed {
            kind: self.kind,
            seed: PhantomData::<IgnoredAny>,
        })? {
            match node_field {
                NodeField::Inner(PhantomData) => {
                    *self.inner = self.map.next_value()?;
                }
                NodeField::Delegate(IgnoredAny) => {
                    let _: IgnoredAny = self.map.next_value()?;
                }
            }
        }
        Ok(())
    }
}

struct UnknownNode<'de, 'a, T, M> {
    name: &'static str,
    node: NodeDeserializer<'de, 'a, T, M>,
}

impl<'de, 'a, T, M> EnumAccess<'de> for UnknownNode<'de, 'a, T, M>
where
    T: Deserialize<'de>,
    M: MapAccess<'de>,
{
    type Error = M::Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let deserializer = BorrowedStrDeserializer::new(self.name);
        let value = seed.deserialize(deserializer)?;
        Ok((value, self))
    }
}

impl<'de, 'a, T, M> VariantAccess<'de> for UnknownNode<'de, 'a, T, M>
where
    T: Deserialize<'de>,
    M: MapAccess<'de>,
{
    type Error = M::Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        self.node.unit_variant()
    }

    fn newtype_variant_seed<V>(self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(self.node)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.node.tuple_variant(len, visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let _ = fields;
        visitor.visit_map(self.node)
    }
}

struct NodeFieldsDeserializer<'de, 'a, T, M> {
    node: NodeDeserializer<'de, 'a, T, M>,
}

impl<'de, 'a, T, M> Deserializer<'de> for NodeFieldsDeserializer<'de, 'a, T, M>
where
    T: Deserialize<'de>,
    M: MapAccess<'de>,
{
    type Error = M::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let kind = self.node.kind;
        match visitor.visit_map(self) {
            Ok(value) => Ok(value),
            Err(error) => Err(error.with_kind(kind)),
        }
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let _ = variants;
        visitor.visit_enum(NodeEnumDeserializer {
            name,
            node: self.node,
        })
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let _ = name;
        self.deserialize_unit(visitor)
    }

    fn deserialize_unit<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.node.ignore()?;
        visitor.visit_unit()
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option newtype_struct seq tuple tuple_struct map struct
        identifier ignored_any
    }
}

impl<'de, 'a, T, M> MapAccess<'de> for NodeFieldsDeserializer<'de, 'a, T, M>
where
    T: Deserialize<'de>,
    M: MapAccess<'de>,
{
    type Error = FieldOfKindError<M::Error>;

    fn next_key_seed<K>(&mut self, mut seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        loop {
            seed = match self
                .node
                .map
                .next_key_seed(NodeFieldSeed {
                    kind: self.node.kind,
                    seed,
                })
                .map_err(FieldOfKindError::Other)?
            {
                None => return Ok(None),
                Some(NodeField::Inner(seed)) => {
                    *self.node.inner = self
                        .node
                        .map
                        .next_value()
                        .map_err(FieldOfKindError::Other)?;
                    seed
                }
                Some(NodeField::Delegate(value)) => return Ok(Some(value)),
            };
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        self.node
            .map
            .next_value_seed(seed)
            .map_err(FieldOfKindError::Other)
    }
}

struct NodeEnumDeserializer<'de, 'a, T, M> {
    name: &'static str,
    node: NodeDeserializer<'de, 'a, T, M>,
}

impl<'de, 'a, T, M> EnumAccess<'de> for NodeEnumDeserializer<'de, 'a, T, M>
where
    T: Deserialize<'de>,
    M: MapAccess<'de>,
{
    type Error = M::Error;
    type Variant = Self;

    fn variant_seed<V>(mut self, mut seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        loop {
            seed = match self.node.map.next_key_seed(NodeFieldSeed {
                kind: self.node.kind,
                seed,
            })? {
                None => {
                    let expected = ExpectedEnum { name: self.name };
                    return Err(Error::invalid_type(Unexpected::Map, &expected));
                }
                Some(NodeField::Inner(seed)) => {
                    *self.node.inner = self.node.map.next_value()?;
                    seed
                }
                Some(NodeField::Delegate(value)) => return Ok((value, self)),
            }
        }
    }
}

impl<'de, 'a, T, M> VariantAccess<'de> for NodeEnumDeserializer<'de, 'a, T, M>
where
    T: Deserialize<'de>,
    M: MapAccess<'de>,
{
    type Error = M::Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        let expected = "unit variant";
        Err(Error::invalid_type(Unexpected::NewtypeVariant, &expected))
    }

    fn newtype_variant_seed<V>(mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let value = self.node.map.next_value_seed(seed)?;
        loop {
            match self.node.map.next_key_seed(NodeFieldSeed {
                kind: self.node.kind,
                seed: PhantomData::<UnexpectedField>,
            })? {
                None => return Ok(value),
                Some(NodeField::Inner(PhantomData)) => {
                    *self.node.inner = self.node.map.next_value()?;
                }
                Some(NodeField::Delegate(unexpected)) => match unexpected {},
            }
        }
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let _ = len;
        let _ = visitor;
        let expected = "tuple variant";
        Err(Error::invalid_type(Unexpected::NewtypeVariant, &expected))
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let _ = fields;
        let _ = visitor;
        let expected = "struct variant";
        Err(Error::invalid_type(Unexpected::NewtypeVariant, &expected))
    }
}

struct NodeFieldSeed<'a, K> {
    kind: &'a AnyKind<'a>,
    seed: K,
}

enum NodeField<K, X> {
    Inner(K),
    Delegate(X),
}

impl<'de, 'a, K> DeserializeSeed<'de> for NodeFieldSeed<'a, K>
where
    K: DeserializeSeed<'de>,
{
    type Value = NodeField<K, K::Value>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(self)
    }
}

impl<'de, 'a, K> Visitor<'de> for NodeFieldSeed<'a, K>
where
    K: DeserializeSeed<'de>,
{
    type Value = NodeField<K, K::Value>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("field of syntax tree node")
    }

    fn visit_str<E>(self, identifier: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match identifier {
            "inner" => Ok(NodeField::Inner(self.seed)),
            other => match self.seed.deserialize(FieldOfKindDeserializer {
                field: other,
                error: PhantomData,
            }) {
                Ok(field) => Ok(NodeField::Delegate(field)),
                Err(error) => Err(error.with_kind(self.kind)),
            },
        }
    }
}

struct FieldOfKindDeserializer<'a, E> {
    field: &'a str,
    error: PhantomData<E>,
}

impl<'de, 'a, E> Deserializer<'de> for FieldOfKindDeserializer<'a, E>
where
    E: Error,
{
    type Error = FieldOfKindError<E>;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.field)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

#[derive(Debug)]
enum FieldOfKindError<E> {
    UnknownField {
        field: Box<str>,
        expected: &'static [&'static str],
    },
    MissingField {
        field: &'static str,
    },
    Other(E),
}

impl<E> FieldOfKindError<E>
where
    E: Error,
{
    fn with_kind(self, kind: &AnyKind) -> E {
        match self {
            FieldOfKindError::UnknownField { field, expected } => {
                if let AnyKind::Kind(Kind::null) = kind {
                    E::unknown_field(&field, expected)
                } else if expected.is_empty() {
                    E::custom(format_args!(
                        "unknown field `{}` in {}, there are no fields",
                        field, kind,
                    ))
                } else {
                    E::custom(format_args!(
                        "unknown field `{}` in {}, expected {}",
                        field,
                        kind,
                        OneOf { names: expected },
                    ))
                }
            }
            FieldOfKindError::MissingField { field } => {
                if let AnyKind::Kind(Kind::null) = kind {
                    E::missing_field(field)
                } else {
                    E::custom(format_args!("missing field `{}` in {}", field, kind))
                }
            }
            FieldOfKindError::Other(error) => error,
        }
    }
}

impl<E> Error for FieldOfKindError<E>
where
    E: Error,
{
    fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self {
        FieldOfKindError::UnknownField {
            field: Box::from(field),
            expected,
        }
    }

    fn missing_field(field: &'static str) -> Self {
        FieldOfKindError::MissingField { field }
    }

    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        FieldOfKindError::Other(E::custom(msg))
    }

    fn invalid_type(unexp: Unexpected, exp: &dyn Expected) -> Self {
        FieldOfKindError::Other(E::invalid_type(unexp, exp))
    }

    fn invalid_value(unexp: Unexpected, exp: &dyn Expected) -> Self {
        FieldOfKindError::Other(E::invalid_value(unexp, exp))
    }

    fn invalid_length(len: usize, exp: &dyn Expected) -> Self {
        FieldOfKindError::Other(E::invalid_length(len, exp))
    }

    fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self {
        FieldOfKindError::Other(E::unknown_variant(variant, expected))
    }

    fn duplicate_field(field: &'static str) -> Self {
        FieldOfKindError::Other(E::duplicate_field(field))
    }
}

impl<E> StdError for FieldOfKindError<E>
where
    E: StdError,
{
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            FieldOfKindError::UnknownField { .. } | FieldOfKindError::MissingField { .. } => None,
            FieldOfKindError::Other(error) => error.source(),
        }
    }
}

impl<E> Display for FieldOfKindError<E>
where
    E: Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FieldOfKindError::UnknownField { field, expected } => {
                if expected.is_empty() {
                    write!(formatter, "unknown field `{}`, there are no fields", field)
                } else {
                    write!(
                        formatter,
                        "unknown field `{}`, expected {}",
                        field,
                        OneOf { names: expected },
                    )
                }
            }
            FieldOfKindError::MissingField { field } => {
                write!(formatter, "missing field `{}`", field)
            }
            FieldOfKindError::Other(error) => Display::fmt(error, formatter),
        }
    }
}

struct OneOf {
    names: &'static [&'static str],
}

impl Display for OneOf {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.names.len() {
            0 => unreachable!(), // special case elsewhere
            1 => write!(formatter, "`{}`", self.names[0]),
            2 => write!(formatter, "`{}` or `{}`", self.names[0], self.names[1]),
            _ => {
                formatter.write_str("one of ")?;
                for (i, alt) in self.names.iter().enumerate() {
                    if i > 0 {
                        formatter.write_str(", ")?;
                    }
                    write!(formatter, "`{}`", alt)?;
                }
                Ok(())
            }
        }
    }
}

enum UnexpectedField {}

impl<'de> Deserialize<'de> for UnexpectedField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(UnexpectedFieldVisitor)
    }
}

struct UnexpectedFieldVisitor;

impl<'de> Visitor<'de> for UnexpectedFieldVisitor {
    type Value = UnexpectedField;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("no more fields")
    }

    fn visit_str<E>(self, string: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::unknown_field(string, &[]))
    }
}

struct ExpectedTupleVariant<'a> {
    kind: &'a str,
}

impl<'a> Expected for ExpectedTupleVariant<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "tuple variant of type `{}`", self.kind)
    }
}

struct ExpectedEnum {
    name: &'static str,
}

impl Expected for ExpectedEnum {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "enum `{}`", self.name)
    }
}
