use crate::errors;
use serde_json::{json, Map, Value};

pub fn unflatten(data: &Map<String, Value>) -> Result<Value, errors::Error> {
    let mut output = json!({});

    if data.is_empty() {
        return Ok(output);
    }

    let regex = regex::Regex::new(r"\.?([^.\[\]]+)|\[(\d+)\]").unwrap();
    for (p, value) in data {
        let mut cur = &mut output;
        let mut property = "";

        for c in regex.captures_iter(p) {
            let c2 = c.get(2).map(|m| m.as_str());

            let value = if c2.is_some() {
                Value::Array(vec![])
            } else {
                Value::Object(Map::new())
            };

            match cur {
                Value::Array(a) => {
                    let index = property
                        .parse::<usize>()
                        .map_err(|_| errors::Error::InvalidProperty)?;
                    if a.get(index).is_none() {
                        a.push(value);
                    }
                    cur = cur.get_mut(index).ok_or(errors::Error::FormatError)?;
                }
                Value::Object(o) => {
                    if o.get(property).is_none() {
                        o.insert(property.to_owned(), value);
                    } else if c2.is_some() && o.get(property).is_some_and(|f| f.is_object()) {
                        return Err(errors::Error::FormatError);
                    }

                    cur = cur.get_mut(property).ok_or(errors::Error::Unspecified)?;
                }
                _ => return Err(errors::Error::InvalidType),
            };

            if let Some(v2) = c2 {
                property = v2;
            } else if let Some(v1) = c.get(1).map(|m| m.as_str()) {
                property = v1;
            } else {
                return Err(errors::Error::InvalidProperty);
            };
        }

        match cur {
            Value::Array(a) => {
                a.push(value.clone());
            }
            Value::Object(o) => {
                if o.contains_key(property) {
                    return Err(errors::Error::FormatError);
                }
                o.insert(property.to_owned(), value.clone());
            }
            _ => return Err(errors::Error::InvalidType),
        }
    }
    output
        .get("")
        .ok_or(errors::Error::InvalidProperty)
        .cloned()
}
