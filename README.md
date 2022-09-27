# rs-lib-strategy-serde-json

This library depends on:
- `serde_json`
- `rs-lib-strategy-serde`

## Usage

```rust
fn main() {
    use serde::{Serialize, Deserialize};
    use rs_lib_strategy_serde::StrategyInjector;
    use rs_lib_strategy_serde_json::JsonStrategy;

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct Test {string: String}
 
    let input = r#"{"string":"test"}"#;
    let ser_de = StrategyInjector::new(JsonStrategy);
    let expected = Test{ string: "test".to_string()};
    
    let output: Test = ser_de.deserialize::<Test>(i).unwrap();
    assert_eq!(expected, output)
}
```