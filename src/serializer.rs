use serde::ser::{
    Error, Impossible, Serialize, SerializeMap, SerializeStruct, SerializeStructVariant, Serializer,
};

pub(crate) struct NodeSerializer<'a, M> {
    map: &'a mut M,
}

impl<'a, M> NodeSerializer<'a, M> {
    pub(crate) fn new(map: &'a mut M) -> Self {
        NodeSerializer { map }
    }
}

impl<'a, M> Serializer for NodeSerializer<'a, M>
where
    M: SerializeMap,
{
    type Ok = ();
    type Error = M::Error;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        Err(Error::custom("unsupported \"kind\""))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        Err(Error::custom("unsupported \"kind\""))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        Err(Error::custom("unsupported \"kind\""))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        Err(Error::custom("unsupported \"kind\""))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        Err(Error::custom("unsupported \"kind\""))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        Err(Error::custom("unsupported \"kind\""))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        Err(Error::custom("unsupported \"kind\""))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        Err(Error::custom("unsupported \"kind\""))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        Err(Error::custom("unsupported \"kind\""))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        Err(Error::custom("unsupported \"kind\""))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        Err(Error::custom("unsupported \"kind\""))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        Err(Error::custom("unsupported \"kind\""))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        Err(Error::custom("unsupported \"kind\""))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        Err(Error::custom("unsupported \"kind\""))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        T::serialize(value, self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        let _ = name;
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        let _ = name;
        let _ = variant_index;
        self.map.serialize_entry("kind", variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let _ = name;
        T::serialize(value, self)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let _ = name;
        let _ = variant_index;
        self.map.serialize_entry("kind", variant)?;
        T::serialize(value, self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let _ = len;
        Err(Error::custom("unsupported \"kind\""))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        let _ = len;
        Err(Error::custom("unsupported \"kind\""))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        let _ = name;
        let _ = len;
        Err(Error::custom("unsupported \"kind\""))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let _ = name;
        let _ = variant_index;
        let _ = variant;
        let _ = len;
        Err(Error::custom("unsupported \"kind\""))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let _ = len;
        Ok(self)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let _ = name;
        let _ = len;
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let _ = name;
        let _ = variant_index;
        let _ = len;
        self.map.serialize_entry("kind", variant)?;
        Ok(self)
    }
}

impl<'a, M> SerializeMap for NodeSerializer<'a, M>
where
    M: SerializeMap,
{
    type Ok = ();
    type Error = M::Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.map.serialize_key(key)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.map.serialize_value(value)
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: ?Sized + Serialize,
        V: ?Sized + Serialize,
    {
        self.map.serialize_entry(key, value)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<'a, M> SerializeStruct for NodeSerializer<'a, M>
where
    M: SerializeMap,
{
    type Ok = ();
    type Error = M::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.map.serialize_entry(key, value)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<'a, M> SerializeStructVariant for NodeSerializer<'a, M>
where
    M: SerializeMap,
{
    type Ok = ();
    type Error = M::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.map.serialize_entry(key, value)
    }

    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}
