use serde_json::json;

use super::canonical_json;

/// Requirement validation: Receipt JSON uses deterministic UTF-16 key ordering.
#[test]
fn receipt_canonicalization_orders_object_keys_success() {
    let value = json!({"z": 1, "a": {"b": true, "a": false}});
    assert_eq!(
        canonical_json(&value).expect("test setup should succeed"),
        r#"{"a":{"a":false,"b":true},"z":1}"#
    );
}
