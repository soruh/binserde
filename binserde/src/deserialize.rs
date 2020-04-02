pub trait Deserialize<R>
where
    R: std::io::Read,
    Self: Sized,
{
    fn deserialize_ne(reader: &mut R) -> std::io::Result<Self>;
    #[inline]
    fn deserialize_be(reader: &mut R) -> std::io::Result<Self> {
        Self::deserialize_ne(reader)
    }
    #[inline]
    fn deserialize_le(reader: &mut R) -> std::io::Result<Self> {
        Self::deserialize_ne(reader)
    }
}

macro_rules! impl_builtin {
    ($builtin: ty) => {
        impl<R> Deserialize<R> for $builtin
        where
            R: std::io::Read,
        {
            #[inline]
            fn deserialize_ne(reader: &mut R) -> std::io::Result<$builtin> {
                let mut buffer = [0u8; std::mem::size_of::<$builtin>()];
                reader.read_exact(&mut buffer)?;
                Ok(<$builtin>::from_ne_bytes(buffer))
            }
            #[inline]
            fn deserialize_le(reader: &mut R) -> std::io::Result<$builtin> {
                let mut buffer = [0u8; std::mem::size_of::<$builtin>()];
                reader.read_exact(&mut buffer)?;
                Ok(<$builtin>::from_le_bytes(buffer))
            }
            #[inline]
            fn deserialize_be(reader: &mut R) -> std::io::Result<$builtin> {
                let mut buffer = [0u8; std::mem::size_of::<$builtin>()];
                reader.read_exact(&mut buffer)?;
                Ok(<$builtin>::from_be_bytes(buffer))
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

impl<R> Deserialize<R> for bool
where
    R: std::io::Read,
{
    #[inline]
    fn deserialize_ne(reader: &mut R) -> std::io::Result<bool> {
        let res = u8::deserialize_ne(reader)?;
        Ok(if res == 0 { false } else { true })
    }
}

impl<R> Deserialize<R> for std::net::Ipv4Addr
where
    R: std::io::Read,
{
    fn deserialize_ne(reader: &mut R) -> std::io::Result<Self> {
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;

        Ok(buffer.into())
    }
}

impl<R> Deserialize<R> for std::net::Ipv6Addr
where
    R: std::io::Read,
{
    fn deserialize_ne(reader: &mut R) -> std::io::Result<Self> {
        let mut buffer = [0u8; 16];
        reader.read_exact(&mut buffer)?;

        Ok(buffer.into())
    }
}

impl<R, T> Deserialize<R> for Option<T>
where
    R: std::io::Read,
    T: Deserialize<R> + Default + Eq,
{
    #[inline]
    fn deserialize_ne(reader: &mut R) -> std::io::Result<Option<T>> {
        let res = <T>::deserialize_ne(reader)?;
        Ok(if res == <T>::default() {
            None
        } else {
            Some(res)
        })
    }

    #[inline]
    fn deserialize_le(reader: &mut R) -> std::io::Result<Option<T>> {
        let res = <T>::deserialize_le(reader)?;
        Ok(if res == <T>::default() {
            None
        } else {
            Some(res)
        })
    }

    #[inline]
    fn deserialize_be(reader: &mut R) -> std::io::Result<Option<T>> {
        let res = <T>::deserialize_be(reader)?;
        Ok(if res == <T>::default() {
            None
        } else {
            Some(res)
        })
    }
}

#[cfg(feature = "unstable")]
impl<R, T, const N: usize> Deserialize<R> for [T; N]
where
    R: std::io::Read,
    T: Deserialize<R>,
{
    #[inline]
    fn deserialize_ne(reader: &mut R) -> std::io::Result<Self> {
        use std::mem::MaybeUninit;

        // Safety: this is sound as seen in MaybeUninit docs
        let mut res: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        for i in 0..N {
            res[i].write(<T>::deserialize_ne(reader)?);
        }

        // Safetly: every element has been initialized
        Ok(unsafe { (&res as *const _ as *const [T; N]).read() })
    }
    #[inline]
    fn deserialize_be(reader: &mut R) -> std::io::Result<Self> {
        use std::mem::MaybeUninit;

        // Safety: this is sound as seen in MaybeUninit docs
        let mut res: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        for i in 0..N {
            res[i].write(<T>::deserialize_be(reader)?);
        }

        // Safetly: every element has been initialized
        Ok(unsafe { (&res as *const _ as *const [T; N]).read() })
    }
    #[inline]
    fn deserialize_le(reader: &mut R) -> std::io::Result<Self> {
        use std::mem::MaybeUninit;

        // Safety: this is sound as seen in MaybeUninit docs
        let mut res: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        for i in 0..N {
            res[i].write(<T>::deserialize_le(reader)?);
        }

        // Safetly: every element has been initialized
        Ok(unsafe { (&res as *const _ as *const [T; N]).read() })
    }
}
