use encode_unicode::CharExt;
use serde::Serialize;

use crate::Error;

pub struct Serializer<'a> {
    buf: &'a mut [u8],
    ofs: usize,
}

impl<'a> Serializer<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Serializer { buf, ofs: 0 }
    }

    #[inline(always)]
    fn assert_space(&self, space: usize) -> Result<(), Error> {
        if self.ofs + space > self.buf.len() {
            Err(Error::OutOfSpace)
        } else {
            Ok(())
        }
    }

    #[inline(always)]
    fn write_u8(&mut self, v: u8) -> Result<(), Error> {
        self.assert_space(1)?;
        unsafe {
            *self.buf.get_unchecked_mut(self.ofs) = v;
        }
        self.ofs += 1;
        Ok(())
    }

    #[inline(always)]
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        let bytes = s.as_bytes();
        let len = bytes.len();
        self.assert_space(len + 2)?;

        // write length
        unsafe {
            *self.buf.get_unchecked_mut(self.ofs) = (len & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 1) = (len >> 8 & 0xFF) as u8;
        }

        let start = self.ofs + 2;
        let end = start + len;
        self.buf[start..end].clone_from_slice(bytes);

        self.ofs += len + 2;
        Ok(())
    }

    #[inline(always)]
    fn write_u16(&mut self, v: u16) -> Result<(), Error> {
        self.assert_space(2)?;
        unsafe {
            *self.buf.get_unchecked_mut(self.ofs) = (v & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 1) = (v >> 8 & 0xFF) as u8;
        }
        self.ofs += 2;
        Ok(())
    }

    #[inline(always)]
    fn write_u32(&mut self, v: u32) -> Result<(), Error> {
        self.assert_space(4)?;
        unsafe {
            *self.buf.get_unchecked_mut(self.ofs) = (v & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 1) = (v >> 8 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 2) = (v >> 16 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 3) = (v >> 24 & 0xFF) as u8;
        }
        self.ofs += 4;
        Ok(())
    }

    #[inline(always)]
    fn write_u64(&mut self, v: u64) -> Result<(), Error> {
        self.assert_space(8)?;
        unsafe {
            *self.buf.get_unchecked_mut(self.ofs) = (v & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 1) = (v >> 8 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 2) = (v >> 16 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 3) = (v >> 24 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 4) = (v >> 32 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 5) = (v >> 40 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 6) = (v >> 48 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 7) = (v >> 56 & 0xFF) as u8;
        }
        self.ofs += 8;
        Ok(())
    }

    #[inline(always)]
    fn write_u128(&mut self, v: u128) -> Result<(), Error> {
        self.assert_space(16)?;
        unsafe {
            *self.buf.get_unchecked_mut(self.ofs) = (v & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 1) = (v >> 8 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 2) = (v >> 16 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 3) = (v >> 24 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 4) = (v >> 32 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 5) = (v >> 40 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 6) = (v >> 48 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 7) = (v >> 56 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 8) = (v >> 64 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 9) = (v >> 72 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 10) = (v >> 80 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 11) = (v >> 88 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 12) = (v >> 96 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 13) =
                (v >> 104 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 14) =
                (v >> 112 & 0xFF) as u8;
            *self.buf.get_unchecked_mut(self.ofs + 15) =
                (v >> 120 & 0xFF) as u8;
        }
        self.ofs += 16;
        Ok(())
    }
}

impl<'b, 'a> serde::ser::Serializer for &'a mut Serializer<'b> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = serde::ser::Impossible<(), Error>;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.write_u8(if v { 1 } else { 0 })
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.write_u8(v as u8)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.write_u16(v as u16)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.write_u32(v as u32)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.write_u64(v as u64)
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.write_u128(v as u128)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.write_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.write_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.write_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.write_u64(v)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.write_u128(v)
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::NotSupported)
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::NotSupported)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let (arr, sz) = v.to_utf8_array();
        self.assert_space(sz)?;
        for (i, c) in arr[..sz].iter().enumerate() {
            unsafe {
                *self.buf.get_unchecked_mut(self.ofs + i) = *c;
            }
        }
        self.ofs += sz;
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.write_str(v)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        // Placeholder for specialization?
        unimplemented!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.write_u8(0)
    }

    fn serialize_some<T: ?Sized>(
        self,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.write_u8(1)?;
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(
        self,
        _name: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        if variant_index > 255 {
            return Err(Error::TooManyVariants);
        }
        self.write_u8(variant_index as u8)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        if variant_index > 255 {
            return Err(Error::TooManyVariants);
        }
        self.write_u8(variant_index as u8)?;
        value.serialize(self)
    }

    fn serialize_seq(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeSeq, Self::Error> {
        let len = len.unwrap_or(0);
        if len > core::u16::MAX as usize {
            Err(Error::LengthExceeded)
        } else {
            self.write_u16(len as u16)?;
            Ok(self)
        }
    }

    fn serialize_tuple(
        self,
        _len: usize,
    ) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        if variant_index > 255 {
            return Err(Error::TooManyVariants);
        }
        self.write_u8(variant_index as u8)?;
        Ok(self)
    }

    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::NotSupported)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        if variant_index > 255 {
            return Err(Error::TooManyVariants);
        }
        self.write_u8(variant_index as u8)?;
        Ok(self)
    }

    fn collect_str<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: core::fmt::Display,
    {
        Err(Error::NotSupported)
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        false
    }
}

impl<'b, 'a: 'b> serde::ser::SerializeStructVariant for &'b mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'b, 'a: 'b> serde::ser::SerializeStruct for &'b mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'b, 'a: 'b> serde::ser::SerializeTupleVariant for &'b mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'b, 'a: 'b> serde::ser::SerializeTupleStruct for &'b mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'b, 'a: 'b> serde::ser::SerializeTuple for &'b mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'b, 'a: 'b> serde::ser::SerializeSeq for &'b mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
