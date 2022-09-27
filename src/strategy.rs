use serde::{Deserialize, Serialize};
use serde_strategy::Strategy;

pub struct JsonStrategy;
impl Strategy for JsonStrategy {
    fn serialize<T>(&self, value: &T) -> Option<String> where T: ?Sized + Serialize {
        match serde_json::to_string(value) {
            Ok(res) => Some(res),
            _ => None
        }
    }

    fn deserialize<'a, T>(&self, s: &'a str) -> Option<T> where T: Deserialize<'a> {
        match serde_json::from_str::<T>(s) {
            Ok(res) => Some(res),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use serde::{Serialize, Deserialize};
    use serde_strategy::StrategyInjector;
    use crate::JsonStrategy;

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct Test {string: String}

    fn obj() -> Test {
        Test{ string: "test".to_string() }
    }

    fn string() -> &'static str {
        r#"{"string":"test"}"#
    }

    #[test]
    fn serde_de() {
        let i = string();
        let e = obj();

        let ser_de = StrategyInjector::new(JsonStrategy);
        let o: Test = ser_de.deserialize::<Test>(i).unwrap();
        assert_eq!(e, o)
    }

    #[test]
    fn serde_ser() {
        let i = obj();
        let e = string();

        let ser_de = StrategyInjector::new(JsonStrategy);
        let o = ser_de.serialize(&i).unwrap();
        assert_eq!(e, o)
    }
}