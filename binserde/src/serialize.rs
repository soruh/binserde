pub trait Serialize<W>
where
    W: std::io::Write,
{
    fn serialize_ne(&self, writer: &mut W) -> std::io::Result<()>;
    #[inline]
    fn serialize_be(&self, writer: &mut W) -> std::io::Result<()> {
        self.serialize_ne(writer)
    }
    #[inline]
    fn serialize_le(&self, writer: &mut W) -> std::io::Result<()> {
        self.serialize_ne(writer)
    }
}

macro_rules! impl_builtin {
    ($builtin: ty) => {
        impl<W> Serialize<W> for $builtin
        where
            W: std::io::Write,
        {
            #[inline]
            fn serialize_le(&self, writer: &mut W) -> std::io::Result<()> {
                writer.write_all(&<$builtin>::to_le_bytes(*self))
            }
            #[inline]
            fn serialize_be(&self, writer: &mut W) -> std::io::Result<()> {
                writer.write_all(&<$builtin>::to_be_bytes(*self))
            }
            #[inline]
            fn serialize_ne(&self, writer: &mut W) -> std::io::Result<()> {
                writer.write_all(&<$builtin>::to_ne_bytes(*self))
            }
        }
    };
}

impl_builtin!(u8);
impl_builtin!(u16);
impl_builtin!(u32);
impl_builtin!(u64);
impl_builtin!(u128);
impl_builtin!(usize);

impl_builtin!(i8);
impl_builtin!(i16);
impl_builtin!(i32);
impl_builtin!(i64);
impl_builtin!(i128);
impl_builtin!(isize);

impl_builtin!(f32);
impl_builtin!(f64);

impl<W> Serialize<W> for bool
where
    W: std::io::Write,
{
    #[inline]
    fn serialize_ne(&self, writer: &mut W) -> std::io::Result<()> {
        (*self as u8).serialize_ne(writer)
    }
}

impl<W, T> Serialize<W> for [T]
where
    W: std::io::Write,
    T: Serialize<W>,
{
    #[inline]
    fn serialize_ne(&self, writer: &mut W) -> std::io::Result<()> {
        for item in self {
            item.serialize_ne(writer)?
        }
        Ok(())
    }
    #[inline]
    fn serialize_be(&self, writer: &mut W) -> std::io::Result<()> {
        for item in self {
            item.serialize_be(writer)?
        }
        Ok(())
    }
    #[inline]
    fn serialize_le(&self, writer: &mut W) -> std::io::Result<()> {
        for item in self {
            item.serialize_ne(writer)?
        }
        Ok(())
    }
}

impl<W, T> Serialize<W> for Vec<T>
where
    W: std::io::Write,
    T: Serialize<W>,
{
    #[inline]
    fn serialize_ne(&self, writer: &mut W) -> std::io::Result<()> {
        for item in self {
            item.serialize_ne(writer)?
        }
        Ok(())
    }
    #[inline]
    fn serialize_be(&self, writer: &mut W) -> std::io::Result<()> {
        for item in self {
            item.serialize_be(writer)?
        }
        Ok(())
    }
    #[inline]
    fn serialize_le(&self, writer: &mut W) -> std::io::Result<()> {
        for item in self {
            item.serialize_ne(writer)?
        }
        Ok(())
    }
}

#[cfg(feature = "const_generics")]
impl<W, T, const N: usize> Serialize<W> for [T; N]
where
    W: std::io::Write,
    T: Serialize<W>,
{
    #[inline]
    fn serialize_ne(&self, writer: &mut W) -> std::io::Result<()> {
        for item in self {
            item.serialize_ne(writer)?
        }
        Ok(())
    }
    #[inline]
    fn serialize_be(&self, writer: &mut W) -> std::io::Result<()> {
        for item in self {
            item.serialize_be(writer)?
        }
        Ok(())
    }
    #[inline]
    fn serialize_le(&self, writer: &mut W) -> std::io::Result<()> {
        for item in self {
            item.serialize_ne(writer)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::serialize::Serialize;
    #[test]
    fn test_slice() {
        let array: [u16; 4] = [1, 5, 123, 10000];
        let slice: &[u16] = &array;

        let mut buffer = Vec::new();

        slice.serialize_le(&mut buffer).unwrap();
        slice.serialize_be(&mut buffer).unwrap();

        let expected = vec![
            1, 0, 5, 0, 123, 0, 16, 39, // le
            0, 1, 0, 5, 0, 123, 39, 16, // be
        ];
        assert_eq!(buffer, expected);
    }
    #[test]
    #[cfg(feature = "unstable")]
    fn test_array_unstable() {
        let array: [u16; 4] = [1, 5, 123, 10000];

        let mut buffer = Vec::new();

        array.serialize_le(&mut buffer).unwrap();
        array.serialize_be(&mut buffer).unwrap();

        let expected = vec![
            1, 0, 5, 0, 123, 0, 16, 39, // le
            0, 1, 0, 5, 0, 123, 39, 16, // be
        ];
        assert_eq!(buffer, expected);
    }
}
