use serde_json::Value;

pub fn combine_answer(before: Value, change: Value) -> Value {
    let mut before = before.as_object().unwrap().clone();
    let change = change.as_object().unwrap();
    for (key, value) in change {
        before.insert(key.clone(), value.clone());
    }
    Value::Object(before)
}