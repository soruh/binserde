use binserde::Serialize;

#[derive(binserde_derive::Serialize)]
struct MyStruct {
    a: u8,
    b: i32,
    c: u64,
}

#[test]
fn test_struct() {
    let res = MyStruct {
        a: 2,
        b: -123,
        c: 51,
    };

    let mut buffer: Vec<u8> = Vec::new();
    res.serialize_le(&mut buffer).unwrap();

    dbg!(&buffer);

    assert_eq!(buffer, vec![2, 133, 255, 255, 255, 51, 0, 0, 0, 0, 0, 0, 0]);
}
