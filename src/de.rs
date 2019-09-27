use encode_unicode::Utf8Char;
use serde::de::{DeserializeSeed, IntoDeserializer, Visitor};
use serde::Deserialize;

use crate::Error;

pub struct Deserializer<'a> {
    buf: &'a [u8],
    ofs: usize,
}

struct SeqAccess<'a, 'b: 'a> {
    deserializer: &'a mut Deserializer<'b>,
    len: usize,
}

type DeserializeResult<T> = Result<T, Error>;

impl<'a, 'b: 'a> serde::de::SeqAccess<'b> for SeqAccess<'a, 'b> {
    type Error = Error;

    fn next_element_seed<V: DeserializeSeed<'b>>(
        &mut self,
        seed: V,
    ) -> Result<Option<V::Value>, Error> {
        if self.len > 0 {
            self.len -= 1;
            Ok(Some(DeserializeSeed::deserialize(
                seed,
                &mut *self.deserializer,
            )?))
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

impl<'de> Deserializer<'de> {
    pub fn new(buf: &'de [u8]) -> Self {
        Deserializer { buf, ofs: 0 }
    }

    #[inline(always)]
    fn assert_space(&self, space: usize) -> Result<(), Error> {
        if self.ofs + space > self.buf.len() {
            Err(Error::OutOfSpace)
        } else {
            Ok(())
        }
    }

    #[inline]
    fn read_u8(&mut self) -> Result<u8, Error> {
        self.assert_space(1)?;
        let val = unsafe { *self.buf.get_unchecked(self.ofs) };
        self.ofs += 1;
        Ok(val)
    }

    #[inline]
    fn read_u16(&mut self) -> Result<u16, Error> {
        self.assert_space(2)?;
        let mut val;
        unsafe {
            val = *self.buf.get_unchecked(self.ofs) as u16;
            val |= (*self.buf.get_unchecked(self.ofs + 1) as u16) << 8;
        }
        self.ofs += 2;
        Ok(val)
    }

    #[inline]
    fn read_u32(&mut self) -> Result<u32, Error> {
        self.assert_space(4)?;
        let mut val;
        unsafe {
            val = *self.buf.get_unchecked(self.ofs) as u32;
            val |= (*self.buf.get_unchecked(self.ofs + 1) as u32) << 8;
            val |= (*self.buf.get_unchecked(self.ofs + 2) as u32) << 16;
            val |= (*self.buf.get_unchecked(self.ofs + 3) as u32) << 24;
        }
        self.ofs += 4;
        Ok(val)
    }

    #[inline]
    fn read_u64(&mut self) -> Result<u64, Error> {
        self.assert_space(8)?;
        let mut val;
        unsafe {
            val = *self.buf.get_unchecked(self.ofs) as u64;
            val |= (*self.buf.get_unchecked(self.ofs + 1) as u64) << 8;
            val |= (*self.buf.get_unchecked(self.ofs + 2) as u64) << 16;
            val |= (*self.buf.get_unchecked(self.ofs + 3) as u64) << 24;
            val |= (*self.buf.get_unchecked(self.ofs + 4) as u64) << 32;
            val |= (*self.buf.get_unchecked(self.ofs + 5) as u64) << 40;
            val |= (*self.buf.get_unchecked(self.ofs + 6) as u64) << 48;
            val |= (*self.buf.get_unchecked(self.ofs + 7) as u64) << 56;
        }
        self.ofs += 8;
        Ok(val)
    }

    fn read_u128(&mut self) -> Result<u128, Error> {
        self.assert_space(16)?;
        let mut val;
        unsafe {
            val = *self.buf.get_unchecked(self.ofs) as u128;
            val |= (*self.buf.get_unchecked(self.ofs + 1) as u128) << 8;
            val |= (*self.buf.get_unchecked(self.ofs + 2) as u128) << 16;
            val |= (*self.buf.get_unchecked(self.ofs + 3) as u128) << 24;
            val |= (*self.buf.get_unchecked(self.ofs + 4) as u128) << 32;
            val |= (*self.buf.get_unchecked(self.ofs + 5) as u128) << 40;
            val |= (*self.buf.get_unchecked(self.ofs + 6) as u128) << 48;
            val |= (*self.buf.get_unchecked(self.ofs + 7) as u128) << 56;

            val |= (*self.buf.get_unchecked(self.ofs + 8) as u128) << 64;
            val |= (*self.buf.get_unchecked(self.ofs + 9) as u128) << 72;
            val |= (*self.buf.get_unchecked(self.ofs + 10) as u128) << 80;
            val |= (*self.buf.get_unchecked(self.ofs + 11) as u128) << 88;
            val |= (*self.buf.get_unchecked(self.ofs + 12) as u128) << 96;
            val |= (*self.buf.get_unchecked(self.ofs + 13) as u128) << 104;
            val |= (*self.buf.get_unchecked(self.ofs + 14) as u128) << 112;
            val |= (*self.buf.get_unchecked(self.ofs + 15) as u128) << 120;
        }
        self.ofs += 16;
        Ok(val)
    }
}

impl<'de> serde::de::Deserializer<'de> for &mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let value: u8 = Deserialize::deserialize(self)?;
        match value {
            0 => visitor.visit_bool(false),
            1 => visitor.visit_bool(true),
            _ => Err(Error::InvalidRepresentation),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i8(self.read_u8()? as i8)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i16(self.read_u16()? as i16)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i32(self.read_u32()? as i32)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i64(self.read_u64()? as i64)
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i128(self.read_u128()? as i128)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u8(self.read_u8()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u16(self.read_u16()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u32(self.read_u32()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u64(self.read_u64()?)
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u128(self.read_u128()?)
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match Utf8Char::from_slice_start(&self.buf[self.ofs..]) {
            Ok((c, count)) => {
                self.ofs += count;
                visitor.visit_char(c.to_char())
            }
            Err(_) => Err(Error::InvalidRepresentation),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let len = self.read_u16()? as usize;
        let start = self.ofs;
        let end = self.ofs + len;
        self.ofs += len;
        match core::str::from_utf8(&self.buf[start..end]) {
            Ok(string) => Ok(visitor.visit_borrowed_str(string)?),
            Err(_) => Err(Error::InvalidRepresentation),
        }
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let len = self.read_u16()? as usize;
        let start = self.ofs;
        let end = self.ofs + len;
        self.ofs += len;
        // slice checks bounds by itself.
        visitor.visit_borrowed_bytes(&self.buf[start..end])
    }

    fn deserialize_byte_buf<V>(
        self,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let value: u8 = Deserialize::deserialize(&mut *self)?;
        match value {
            0 => visitor.visit_none(),
            1 => visitor.visit_some(self),
            _ => Err(Error::InvalidRepresentation),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let len = Deserialize::deserialize(&mut *self)?;
        visitor.visit_seq(SeqAccess {
            deserializer: self,
            len: len,
        })
    }

    fn deserialize_tuple<V>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(SeqAccess {
            deserializer: self,
            len: len,
        })
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    fn deserialize_identifier<V>(
        self,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_ignored_any<V>(
        self,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        false
    }
}

impl<'b, 'de: 'b> serde::de::EnumAccess<'de> for &'b mut Deserializer<'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V: DeserializeSeed<'de>>(
        self,
        seed: V,
    ) -> DeserializeResult<(V::Value, Self)> {
        let x: u8 = Deserialize::deserialize(&mut *self)?;
        let v =
            DeserializeSeed::deserialize(seed, (x as u32).into_deserializer())?;
        Ok((v, self))
    }
}

impl<'b, 'de: 'b> serde::de::VariantAccess<'de> for &'b mut Deserializer<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Error> {
        Ok(())
    }

    fn newtype_variant_seed<V: DeserializeSeed<'de>>(
        self,
        seed: V,
    ) -> DeserializeResult<V::Value> {
        DeserializeSeed::deserialize(seed, self)
    }

    fn tuple_variant<V: Visitor<'de>>(
        self,
        len: usize,
        visitor: V,
    ) -> DeserializeResult<V::Value> {
        serde::de::Deserializer::deserialize_tuple(self, len, visitor)
    }

    fn struct_variant<V: Visitor<'de>>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error> {
        serde::de::Deserializer::deserialize_tuple(self, fields.len(), visitor)
    }
}
