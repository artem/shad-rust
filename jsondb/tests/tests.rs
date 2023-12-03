use jsondb::Database;

use json::{array, object, JsonValue};

////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_simple() {
    let mut db = Database::new();
    assert_eq!(
        db.exec(r#"{"collection": "persons", "select": {}}"#)
            .unwrap(),
        array![],
    );

    assert_eq!(
        db.exec(r#"{"collection": "persons", "insert": {"name": "Vasya", "age": 18}}"#)
            .unwrap(),
        JsonValue::Null,
    );
    assert_eq!(
        db.exec(r#"{"collection": "persons", "select": {}}"#)
            .unwrap(),
        array![object! {
            name: "Vasya",
            age: 18,
        }]
    );

    assert_eq!(
        db.exec(r#"{"collection": "persons", "insert": {"name": "Petya", "age": 15}}"#)
            .unwrap(),
        JsonValue::Null,
    );

    assert_eq!(
        db.exec(r#"{"collection": "persons", "select": {"name": "Vasya"}}"#)
            .unwrap(),
        array![object! {
            name: "Vasya",
            age: 18,
        }]
    );
    assert_eq!(
        db.exec(r#"{"collection": "persons", "select": {"name": "Petya"}}"#)
            .unwrap(),
        array![object! {
            name: "Petya",
            age: 15,
        }]
    );

    assert_eq!(
        db.exec(r#"{"collection": "persons", "select": {"age": {"$gt": 17}}}"#)
            .unwrap(),
        array![object! {
            name: "Vasya",
            age: 18,
        }]
    );

    assert_eq!(
        db.exec(r#"{"collection": "persons", "select": {"age": {"$lt": 20}}}"#)
            .unwrap(),
        array![
            object! {
                name: "Vasya",
                age: 18,
            },
            object! {
                name: "Petya",
                age: 15,
            }
        ]
    );

    assert_eq!(
        db.exec(r#"{"collection": "persons", "select": {"age": {"$in": [15, 17]}}}"#)
            .unwrap(),
        array![object! {
            name: "Petya",
            age: 15,
        }]
    );

    assert_eq!(
        db.exec(r#"{"collection": "persons", "select": {"name": {"$in": ["Vasya", "Boris"]}}}"#)
            .unwrap(),
        array![object! {
            name: "Vasya",
            age: 18,
        }]
    );

    assert_eq!(
        db.exec(
            r#"{"collection": "persons", "select": {"$or": [{"name": "Vasya"}, {"age": 15}]}}"#
        )
        .unwrap(),
        array![
            object! {
                name: "Vasya",
                age: 18,
            },
            object! {
                name: "Petya",
                age: 15,
            }
        ]
    );
}

#[test]
fn test_invalid_insertions() {
    let mut db = Database::new();

    assert!(db.exec(r#"{"collection": "foo", "insert": null}"#).is_err());
    assert!(db
        .exec(r#"{"collection": "foo", "insert": [1, 2, 3]}"#)
        .is_err());
    assert!(db
        .exec(r#"{"collection": "foo", "insert": "test"}"#)
        .is_err());
    assert!(db.exec(r#"{"collection": "foo", "insert": 135}"#).is_err());
    assert!(db
        .exec(r#"{"collection": "foo", "insert": {"$fooooo": 345}}"#)
        .is_err());

    assert_eq!(
        db.exec(r#"{"collection": "persons", "select": {}}"#)
            .unwrap(),
        array![],
    );
}
