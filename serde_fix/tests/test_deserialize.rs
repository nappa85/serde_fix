use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
struct NewType<T>(T);

#[test]
fn deserialize_newtype_i32() {
    let result = vec![("field".to_owned(), NewType(11))];

    assert_eq!(serde_fix::from_str("field=11\u{1}"), Ok(result));
}

#[test]
fn deserialize_bytes() {
    let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

    assert_eq!(
        serde_fix::from_bytes("first=23\u{1}last=42\u{1}".as_bytes()),
        Ok(result)
    );
}

#[test]
fn deserialize_str() {
    let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

    assert_eq!(serde_fix::from_str("first=23\u{1}last=42\u{1}"), Ok(result));
}

#[test]
fn deserialize_borrowed_str() {
    let result = vec![("first", 23), ("last", 42)];

    assert_eq!(serde_fix::from_str("first=23\u{1}last=42\u{1}"), Ok(result));
}

#[test]
fn deserialize_reader() {
    let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

    assert_eq!(
        serde_fix::from_reader("first=23\u{1}last=42\u{1}".as_bytes()),
        Ok(result)
    );
}

#[test]
fn deserialize_option() {
    let result = vec![
        ("first".to_owned(), Some(23)),
        ("last".to_owned(), Some(42)),
    ];
    assert_eq!(serde_fix::from_str("first=23\u{1}last=42\u{1}"), Ok(result));
}

#[test]
fn deserialize_unit() {
    assert_eq!(serde_fix::from_str(""), Ok(()));
    assert_eq!(serde_fix::from_str("\u{1}"), Ok(()));
    assert_eq!(serde_fix::from_str("\u{1}\u{1}"), Ok(()));
    assert!(serde_fix::from_str::<()>("first=23\u{1}").is_err());
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
enum X {
    A,
    B,
    C,
}

#[test]
fn deserialize_unit_enum() {
    let result = vec![
        ("one".to_owned(), X::A),
        ("two".to_owned(), X::B),
        ("three".to_owned(), X::C),
    ];

    assert_eq!(
        serde_fix::from_str("one=A\u{1}two=B\u{1}three=C\u{1}"),
        Ok(result)
    );
}

#[test]
fn deserialize_unit_type() {
    assert_eq!(serde_fix::from_str(""), Ok(()));
}
