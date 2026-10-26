//! Values between the suite's vocabulary ([`Node`]) and the generated crate's JSON
//! ([`agency::json::Value`]).
//!
//! Both sides keep a number exactly: the generated reader holds a number in the spelling it
//! arrived in, and a [`Node`] number is exact for every Integer and Decimal. An Integer crosses as
//! its `i64` ([`Number::from`]), never through an `f64`; a spelling that is neither an `i64` nor a
//! decimal literal is refused rather than rounded.

use std::collections::BTreeMap;

use agency::json::Value;
use ess_primitives::facts::Number;
use ess_primitives::node::Node;

/// `node` as the JSON the generated surface reads.
pub fn to_json(node: &Node) -> Value {
    match node {
        Node::Null => Value::Null,
        Node::Bool(value) => Value::Bool(*value),
        Node::Number(number) => Value::Number(match number.as_i64() {
            Some(integer) => integer.to_string(),
            None => number.exact_text(),
        }),
        Node::Text(text) => Value::Text(text.clone()),
        Node::Seq(items) => Value::Array(items.iter().map(to_json).collect()),
        Node::Map(members) => object(members),
    }
}

/// A map of named values as one JSON object, in name order.
pub fn object(members: &BTreeMap<String, Node>) -> Value {
    Value::Object(
        members
            .iter()
            .map(|(name, value)| (name.clone(), to_json(value)))
            .collect(),
    )
}

/// `value`, which the generated surface wrote, as a suite value; `Err` names what cannot be
/// carried exactly.
pub fn to_node(value: &Value) -> Result<Node, String> {
    Ok(match value {
        Value::Null => Node::Null,
        Value::Bool(value) => Node::Bool(*value),
        Value::Number(text) => Node::Number(number(text)?),
        Value::Text(text) => Node::Text(text.clone()),
        Value::Array(items) => Node::Seq(items.iter().map(to_node).collect::<Result<_, _>>()?),
        Value::Object(_) => Node::Map(members(value)?),
    })
}

/// The members of a JSON object, by name; `Err` for anything but an object, or a name written
/// twice.
pub fn members(value: &Value) -> Result<BTreeMap<String, Node>, String> {
    let Value::Object(pairs) = value else {
        return Err(format!("expected an object, found {}", value.describes()));
    };
    let mut out = BTreeMap::new();
    for (name, member) in pairs {
        if out.insert(name.clone(), to_node(member)?).is_some() {
            return Err(format!("the member `{name}` is written twice"));
        }
    }
    Ok(out)
}

fn number(text: &str) -> Result<Number, String> {
    if let Ok(integer) = text.parse::<i64>() {
        return Ok(Number::from(integer));
    }
    Number::decimal_literal(text)
        .ok_or_else(|| format!("the number `{text}` is neither an Integer nor an exact Decimal"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_integer_beyond_binary64_crosses_exactly() {
        let big = i64::MAX;
        let node = Node::Number(Number::from(big));
        let json = to_json(&node);
        assert_eq!(json, Value::Number(big.to_string()));
        let Node::Number(back) = to_node(&json).expect("an i64 is carried") else {
            panic!("an integer reads back as a number");
        };
        assert_eq!(back.as_i64(), Some(big));
    }

    #[test]
    fn a_duplicate_member_is_refused() {
        let value = Value::Object(vec![
            ("a".to_owned(), Value::Null),
            ("a".to_owned(), Value::Null),
        ]);
        assert!(members(&value).is_err());
    }

    #[test]
    fn a_number_with_an_exponent_is_refused() {
        assert!(to_node(&Value::Number("1e3".to_owned())).is_err());
    }
}
