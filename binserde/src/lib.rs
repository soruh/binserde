#![cfg_attr(feature = "unstable", feature(const_generics, maybe_uninit_extra))]

pub mod deserialize;
pub mod serialize;

pub use deserialize::Deserialize;
pub use serialize::Serialize;

#[cfg(test)]
mod tests {
    use crate::deserialize::Deserialize;
    use crate::serialize::Serialize;
    use std::io::Cursor;

    macro_rules! test_builtin {
        ($builtin: ty) => {
            test_builtin!($builtin, rand::random::<$builtin>())
        };
        ($builtin: ty, $value: expr) => {
            let expected: $builtin = $value;
            let mut buffer = Vec::with_capacity(std::mem::size_of::<$builtin>() * 3);

            expected.serialize_le(&mut buffer).unwrap();
            expected.serialize_be(&mut buffer).unwrap();
            expected.serialize_ne(&mut buffer).unwrap();

            let mut buffer = Cursor::new(buffer);

            let le = <$builtin>::deserialize_le(&mut buffer).unwrap();
            let be = <$builtin>::deserialize_be(&mut buffer).unwrap();
            let ne = <$builtin>::deserialize_ne(&mut buffer).unwrap();

            assert_eq!(expected, le);
            assert_eq!(le, be);
            assert_eq!(be, ne);
        };
    }

    #[test]
    fn test_u8() {
        test_builtin!(u8);
    }
    #[test]
    fn test_u16() {
        test_builtin!(u16);
    }
    #[test]
    fn test_u32() {
        test_builtin!(u32);
    }
    #[test]
    fn test_u64() {
        test_builtin!(u64);
    }
    #[test]
    fn test_u128() {
        test_builtin!(u128);
    }
    #[test]
    fn test_usize() {
        test_builtin!(usize);
    }
    #[test]
    fn test_i8() {
        test_builtin!(i8);
    }
    #[test]
    fn test_i16() {
        test_builtin!(i16);
    }
    #[test]
    fn test_i32() {
        test_builtin!(i32);
    }
    #[test]
    fn test_i64() {
        test_builtin!(i64);
    }
    #[test]
    fn test_i128() {
        test_builtin!(i128);
    }
    #[test]
    fn test_isize() {
        test_builtin!(isize);
    }
    #[test]
    fn test_f32() {
        test_builtin!(f32);
    }
    #[test]
    fn test_f64() {
        test_builtin!(f64);
    }
    #[test]
    #[cfg(feature = "unstable")]
    fn test_array_unstable() {
        let array: [u16; 4] = [1, 5, 123, 10000];

        let mut buffer = Vec::new();

        array.serialize_le(&mut buffer).unwrap();

        let mut buffer = Cursor::new(buffer);

        let res = <[u16; 4]>::deserialize_le(&mut buffer).unwrap();

        assert_eq!(res, array);
    }
}
