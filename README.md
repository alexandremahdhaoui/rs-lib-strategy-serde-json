# rs-lib-serde-strategy-json

This library depends on:
- `serde_json`
- `rs-lib-serde-strategy`

## Usage

```rust
fn main() {
    use serde::{Serialize, Deserialize};
    use rs_lib_serde_strategy::StrategyInjector;
    use rs_lib_serde_strategy_json::JsonStrategy;

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct Test {string: String}
 
    let input = r#"{"string":"test"}"#;
    let ser_de = StrategyInjector::new(JsonStrategy);
    let expected = Test{ string: "test".to_string()};
    
    let output: Test = ser_de.deserialize::<Test>(i).unwrap();
    assert_eq!(expected, output)
}
```