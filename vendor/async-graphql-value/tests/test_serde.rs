use async_graphql_value::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Debug;

fn test_value<T: Serialize + DeserializeOwned + Clone + PartialEq + Debug>(value: T) {
    assert_eq!(
        from_value::<T>(to_value(value.clone()).unwrap()).unwrap(),
        value
    )
}

#[test]
fn test_serde() {
    test_value(true);
    test_value(100i32);
    test_value(1.123f64);
    test_value(Some(100i32));
    test_value(ConstValue::Null);
    test_value(vec![0i32, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    #[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
    struct NewType(i32);
    test_value(NewType(100i32));

    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
    enum Enum {
        A,
        B,
    }
    test_value(Enum::A);
    test_value(Enum::B);

    let mut obj = BTreeMap::<Name, ConstValue>::new();
    obj.insert(Name::new("A"), ConstValue::Number(10.into()));
    obj.insert(Name::new("B"), ConstValue::Number(20.into()));
    test_value(obj);

    let mut obj = BTreeMap::<Enum, ConstValue>::new();
    obj.insert(Enum::A, ConstValue::Number(10.into()));
    obj.insert(Enum::B, ConstValue::Number(20.into()));
    test_value(obj);

    #[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
    struct Struct {
        a: i32,
        b: Option<Enum>,
    }
    test_value(Struct {
        a: 100,
        b: Some(Enum::B),
    });
}
