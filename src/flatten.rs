use crate::errors;
use serde_json::{json, Map, Value};

pub fn flatten(value: &Value) -> Result<Map<String, Value>, errors::Error> {
    let mut flattened_json = Map::<String, Value>::new();

    match value {
        Value::Object(map) => {
            if map.is_empty() {
                return Ok(flattened_json);
            }
            flatten_object(&mut flattened_json, None, map)?;
        }
        _ => return Err(errors::Error::NotAnObject),
    }

    Ok(flattened_json)
}

fn flatten_object(
    result: &mut Map<String, Value>,
    property: Option<&str>,
    nested_json: &Map<String, Value>,
) -> Result<(), errors::Error> {
    for (prop, value) in nested_json {
        let flattened_prop = property.map_or_else(
            || prop.clone(),
            |parent_key| format!("{}.{}", parent_key, prop),
        );

        match value {
            Value::Array(array) => flatten_array(result, &flattened_prop, array),
            Value::Object(sub_json) => flatten_object(result, Some(&flattened_prop), sub_json),
            _ => flatten_value(result, &flattened_prop, value.clone()),
        }?
    }

    Ok(())
}

fn flatten_array(
    result: &mut Map<String, Value>,
    property: &str,
    array: &[Value],
) -> Result<(), errors::Error> {
    for (i, value) in array.iter().enumerate() {
        let flattened_prop = format!("{}[{}]", property, i);

        match value {
            Value::Object(sub_json) => flatten_object(result, Some(&flattened_prop), sub_json),
            Value::Array(sub_array) => flatten_array(result, &flattened_prop, sub_array),
            _ => flatten_value(result, &flattened_prop, value.clone()),
        }?
    }

    Ok(())
}

fn flatten_value(
    result: &mut Map<String, Value>,
    property: &str,
    val: Value,
) -> Result<(), errors::Error> {
    if val.is_object() || val.is_array() {
        return Err(errors::Error::NotAValue);
    }

    if let Some(v) = result.get_mut(property) {
        if let Some(existing_array) = v.as_array_mut() {
            existing_array.push(val);
        } else {
            let v = v.take();
            result[property] = json!([v, val]);
        }
    } else {
        result.insert(property.to_string(), json!(val));
    }

    Ok(())
}
