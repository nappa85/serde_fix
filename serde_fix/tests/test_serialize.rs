use serde::Serialize;

#[derive(Serialize)]
struct NewType<T>(T);

#[test]
fn serialize_newtype_i32() {
    let params = &[("field", Some(NewType(11)))];
    assert_eq!(
        serde_fix::to_string(params),
        Ok("field=11\u{1}".to_owned())
    );
}

#[test]
fn serialize_option_map_int() {
    let params = &[("first", Some(23)), ("middle", None), ("last", Some(42))];

    assert_eq!(
        serde_fix::to_string(params),
        Ok("first=23\u{1}last=42\u{1}".to_owned())
    );
}

#[test]
fn serialize_option_map_string() {
    let params = &[
        ("first", Some("hello")),
        ("middle", None),
        ("last", Some("world")),
    ];

    assert_eq!(
        serde_fix::to_string(params),
        Ok("first=hello\u{1}last=world\u{1}".to_owned())
    );
}

#[test]
fn serialize_option_map_bool() {
    let params = &[("one", Some(true)), ("two", Some(false))];

    assert_eq!(
        serde_fix::to_string(params),
        Ok("one=true\u{1}two=false\u{1}".to_owned())
    );
}

#[test]
fn serialize_map_bool() {
    let params = &[("one", true), ("two", false)];

    assert_eq!(
        serde_fix::to_string(params),
        Ok("one=true\u{1}two=false\u{1}".to_owned())
    );
}

#[derive(Serialize)]
enum X {
    A,
    B,
    C,
}

#[test]
fn serialize_unit_enum() {
    let params = &[("one", X::A), ("two", X::B), ("three", X::C)];
    assert_eq!(
        serde_fix::to_string(params),
        Ok("one=A\u{1}two=B\u{1}three=C\u{1}".to_owned())
    );
}

#[derive(Serialize)]
struct Unit;

#[test]
fn serialize_unit_struct() {
    assert_eq!(serde_fix::to_string(Unit), Ok("\u{1}".to_owned()));
}

#[test]
fn serialize_unit_type() {
    assert_eq!(serde_fix::to_string(()), Ok("\u{1}".to_owned()));
}
