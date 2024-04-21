use crate::LastLocation;
use serde::de::value::{BorrowedStrDeserializer, StrDeserializer, UsizeDeserializer};
use serde::de::{DeserializeSeed, Deserializer, EnumAccess, Error, MapAccess, SeqAccess, Visitor};
use std::fmt;

pub(crate) struct NodeFieldValueAdapter<'a, T> {
    pub delegate: T,
    pub field: NodeField,
    pub last_loc: &'a mut LastLocation,
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum NodeField {
    Kind,
    Loc,
    Range,
    Offset,
    File,
    Line,
    LastFile,
    LastLine,
    Other,
}

impl<'a, T> NodeFieldValueAdapter<'a, T> {
    fn recognize_str(&mut self, string: &str) {
        if let NodeField::File = self.field {
            string.clone_into(&mut self.last_loc.file);
        }
    }

    fn recognize_int(&mut self, int: usize) {
        if let NodeField::Line = self.field {
            self.last_loc.line = int;
        }
    }
}

impl<'de, 'a, T> DeserializeSeed<'de> for NodeFieldValueAdapter<'a, T>
where
    T: DeserializeSeed<'de>,
{
    type Value = T::Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.delegate.deserialize(NodeFieldValueAdapter {
            delegate: deserializer,
            field: self.field,
            last_loc: self.last_loc,
        })
    }
}

impl<'de, 'a, T> Deserializer<'de> for NodeFieldValueAdapter<'a, T>
where
    T: Deserializer<'de>,
{
    type Error = T::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_any(NodeFieldValueAdapter {
            delegate: visitor,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_bool(NodeFieldValueAdapter {
            delegate: visitor,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_i8(NodeFieldValueAdapter {
            delegate: visitor,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_i16(NodeFieldValueAdapter {
            delegate: visitor,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_i32(NodeFieldValueAdapter {
            delegate: visitor,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_i64(NodeFieldValueAdapter {
            delegate: visitor,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_i128(NodeFieldValueAdapter {
            delegate: visitor,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_u8(NodeFieldValueAdapter {
            delegate: visitor,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_u16(NodeFieldValueAdapter {
            delegate: visitor,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_u32(NodeFieldValueAdapter {
            delegate: visitor,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_u64(NodeFieldValueAdapter {
            delegate: visitor,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_u128(NodeFieldValueAdapter {
            delegate: visitor,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_f32(NodeFieldValueAdapter {
            delegate: visitor,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_f64(NodeFieldValueAdapter {
            delegate: visitor,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_char(NodeFieldValueAdapter {
            delegate: visitor,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_str(NodeFieldValueAdapter {
            delegate: visitor,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_string(NodeFieldValueAdapter {
            delegate: visitor,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_bytes(NodeFieldValueAdapter {
            delegate: visitor,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_byte_buf(NodeFieldValueAdapter {
            delegate: visitor,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_option(NodeFieldValueAdapter {
            delegate: visitor,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_unit(NodeFieldValueAdapter {
            delegate: visitor,
            field: self.field,
            last_loc: self.last_loc,
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
        self.delegate.deserialize_unit_struct(
            name,
            NodeFieldValueAdapter {
                delegate: visitor,
                field: self.field,
                last_loc: self.last_loc,
            },
        )
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_newtype_struct(
            name,
            NodeFieldValueAdapter {
                delegate: visitor,
                field: self.field,
                last_loc: self.last_loc,
            },
        )
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_seq(NodeFieldValueAdapter {
            delegate: visitor,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_tuple(
            len,
            NodeFieldValueAdapter {
                delegate: visitor,
                field: self.field,
                last_loc: self.last_loc,
            },
        )
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_tuple_struct(
            name,
            len,
            NodeFieldValueAdapter {
                delegate: visitor,
                field: self.field,
                last_loc: self.last_loc,
            },
        )
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_map(NodeFieldValueAdapter {
            delegate: visitor,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_struct(
            name,
            fields,
            NodeFieldValueAdapter {
                delegate: visitor,
                field: self.field,
                last_loc: self.last_loc,
            },
        )
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
        self.delegate.deserialize_enum(
            name,
            variants,
            NodeFieldValueAdapter {
                delegate: visitor,
                field: self.field,
                last_loc: self.last_loc,
            },
        )
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.delegate.deserialize_identifier(NodeFieldValueAdapter {
            delegate: visitor,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.field {
            NodeField::Loc | NodeField::Range => {
                self.delegate.deserialize_map(NodeFieldValueAdapter {
                    delegate: visitor,
                    field: self.field,
                    last_loc: self.last_loc,
                })
            }
            NodeField::File => self.delegate.deserialize_str(NodeFieldValueAdapter {
                delegate: visitor,
                field: self.field,
                last_loc: self.last_loc,
            }),
            NodeField::Line => self.delegate.deserialize_u64(NodeFieldValueAdapter {
                delegate: visitor,
                field: self.field,
                last_loc: self.last_loc,
            }),
            NodeField::Kind
            | NodeField::Offset
            | NodeField::LastFile
            | NodeField::LastLine
            | NodeField::Other => self
                .delegate
                .deserialize_ignored_any(NodeFieldValueAdapter {
                    delegate: visitor,
                    field: self.field,
                    last_loc: self.last_loc,
                }),
        }
    }
}

impl<'de, 'a, T> Visitor<'de> for NodeFieldValueAdapter<'a, T>
where
    T: Visitor<'de>,
{
    type Value = T::Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.delegate.expecting(formatter)
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.delegate.visit_bool(v)
    }

    fn visit_i8<E>(mut self, v: i8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.recognize_int(v as usize);
        self.delegate.visit_i8(v)
    }

    fn visit_i16<E>(mut self, v: i16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.recognize_int(v as usize);
        self.delegate.visit_i16(v)
    }

    fn visit_i32<E>(mut self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.recognize_int(v as usize);
        self.delegate.visit_i32(v)
    }

    fn visit_i64<E>(mut self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.recognize_int(v as usize);
        self.delegate.visit_i64(v)
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.delegate.visit_i128(v)
    }

    fn visit_u8<E>(mut self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.recognize_int(v as usize);
        self.delegate.visit_u8(v)
    }

    fn visit_u16<E>(mut self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.recognize_int(v as usize);
        self.delegate.visit_u16(v)
    }

    fn visit_u32<E>(mut self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.recognize_int(v as usize);
        self.delegate.visit_u32(v)
    }

    fn visit_u64<E>(mut self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.recognize_int(v as usize);
        self.delegate.visit_u64(v)
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.delegate.visit_u128(v)
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.delegate.visit_f32(v)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.delegate.visit_f64(v)
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.delegate.visit_char(v)
    }

    fn visit_str<E>(mut self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.recognize_str(v);
        self.delegate.visit_str(v)
    }

    fn visit_borrowed_str<E>(mut self, v: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.recognize_str(v);
        self.delegate.visit_borrowed_str(v)
    }

    fn visit_string<E>(mut self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.recognize_str(&v);
        self.delegate.visit_string(v)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.delegate.visit_bytes(v)
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.delegate.visit_borrowed_bytes(v)
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.delegate.visit_byte_buf(v)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.delegate.visit_none()
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.delegate.visit_some(NodeFieldValueAdapter {
            delegate: deserializer,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.delegate.visit_unit()
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.delegate.visit_newtype_struct(NodeFieldValueAdapter {
            delegate: deserializer,
            field: self.field,
            last_loc: self.last_loc,
        })
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        self.delegate.visit_seq(NodeFieldValueAdapter {
            delegate: seq,
            field: NodeField::Other,
            last_loc: self.last_loc,
        })
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        self.delegate
            .visit_map(MapAccessAdapter::new(self.field, map, self.last_loc))
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: EnumAccess<'de>,
    {
        self.delegate.visit_enum(data)
    }
}

impl<'de, 'a, S> SeqAccess<'de> for NodeFieldValueAdapter<'a, S>
where
    S: SeqAccess<'de>,
{
    type Error = S::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        self.delegate.next_element_seed(NodeFieldValueAdapter {
            delegate: seed,
            field: NodeField::Other,
            last_loc: self.last_loc,
        })
    }
}

struct MapAccessAdapter<'a, M> {
    delegate: M,
    outer_field: NodeField,
    next_field: NodeField,
    has_offset: bool,
    has_file: bool,
    has_line: bool,
    last_loc: &'a mut LastLocation,
}

impl<'a, M> MapAccessAdapter<'a, M> {
    fn new(field: NodeField, delegate: M, last_loc: &'a mut LastLocation) -> Self {
        MapAccessAdapter {
            delegate,
            outer_field: field,
            next_field: NodeField::Other,
            has_offset: false,
            has_file: false,
            has_line: false,
            last_loc,
        }
    }
}

impl<'de, 'a, M> MapAccess<'de> for MapAccessAdapter<'a, M>
where
    M: MapAccess<'de>,
{
    type Error = M::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        let mut seed = Some(seed);
        if let Some((next_field, value)) = self.delegate.next_key_seed(NodeFieldSeed {
            seed: &mut seed,
            field: self.outer_field,
        })? {
            self.next_field = next_field;
            match next_field {
                NodeField::Offset => self.has_offset = true,
                NodeField::File => self.has_file = true,
                NodeField::Line => self.has_line = true,
                _ => {}
            }
            return Ok(Some(value));
        }

        let seed = seed.take().unwrap();
        if self.has_offset && !self.has_file {
            self.next_field = NodeField::LastFile;
            self.has_file = true;
            seed.deserialize(BorrowedStrDeserializer::new("file"))
                .map(Some)
        } else if self.has_offset && !self.has_line {
            self.next_field = NodeField::LastLine;
            self.has_line = true;
            seed.deserialize(BorrowedStrDeserializer::new("line"))
                .map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self.next_field {
            NodeField::LastFile => seed.deserialize(StrDeserializer::new(&self.last_loc.file)),
            NodeField::LastLine => seed.deserialize(UsizeDeserializer::new(self.last_loc.line)),
            _other => self.delegate.next_value_seed(NodeFieldValueAdapter {
                delegate: seed,
                field: self.next_field,
                last_loc: self.last_loc,
            }),
        }
    }
}

struct NodeFieldSeed<'a, T> {
    seed: &'a mut Option<T>,
    field: NodeField,
}

impl<'de, 'a, T> DeserializeSeed<'de> for NodeFieldSeed<'a, T>
where
    T: DeserializeSeed<'de>,
{
    type Value = (NodeField, T::Value);

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(self)
    }
}

impl<'de, 'a, T> Visitor<'de> for NodeFieldSeed<'a, T>
where
    T: DeserializeSeed<'de>,
{
    type Value = (NodeField, T::Value);

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("field of syntax tree node")
    }

    fn visit_str<E>(self, identifier: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let field = match (self.field, identifier) {
            (NodeField::Loc, "spellingLoc" | "expansionLoc") => NodeField::Loc,
            (NodeField::Loc, "offset") => NodeField::Offset,
            (NodeField::Loc, "file") => NodeField::File,
            (NodeField::Loc, "line") => NodeField::Line,
            (NodeField::Range, "begin" | "end") => NodeField::Loc,
            (NodeField::Other, "loc") => NodeField::Loc,
            (NodeField::Other, "range") => NodeField::Range,
            _other => NodeField::Other,
        };
        let value = self
            .seed
            .take()
            .unwrap()
            .deserialize(StrDeserializer::new(identifier))?;
        Ok((field, value))
    }
}
