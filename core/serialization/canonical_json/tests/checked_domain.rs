//! T4-S6: `checked_v1_domain_canonical` writes or refuses exactly as the former
//! path `canonical_json_checked_v1_text(&serde_json::to_string(&json!({"domain", "payload"})))`,
//! over borrowed values. The former path lives only here (and in the callers' tests).
use open_pipe_stress_canonical_json::{
    canonical_json_checked_v1_text, checked_v1_domain_canonical, ByteSink, DomainPayload, MAX_SAFE_INTEGER,
};
use serde_json::{json, Map, Number, Value};

/// The former path: copy the payload (less `omit`) into a wrapper, serialize, checked-parse, canonicalize.
fn former(domain: &str, payload: &Value, omit: Option<&str>) -> Result<String, String> {
    let mut payload = payload.clone();
    if let Some(omit) = omit {
        payload.as_object_mut().expect("omit applies to an object").remove(omit);
    }
    canonical_json_checked_v1_text(&serde_json::to_string(&json!({"domain": domain, "payload": payload})).unwrap())
}

fn view<'a>(payload: &'a Value, omit: Option<&'a str>) -> DomainPayload<'a> {
    match omit {
        Some(omit) => DomainPayload::ObjectWithout(payload.as_object().unwrap(), omit),
        None => DomainPayload::Value(payload),
    }
}

/// The new path into both sinks; the byte sink must receive exactly the text.
fn streamed(domain: &str, payload: &Value, omit: Option<&str>) -> Result<String, String> {
    let mut text = String::new();
    let result = checked_v1_domain_canonical(domain, view(payload, omit), &mut text);
    let mut bytes = Vec::new();
    let byte_result = checked_v1_domain_canonical(domain, view(payload, omit), &mut ByteSink(|b: &[u8]| bytes.extend_from_slice(b)));
    assert_eq!(result, byte_result, "both sinks refuse alike");
    match result {
        Ok(()) => {
            assert_eq!(bytes, text.as_bytes(), "the byte sink receives the text");
            Ok(text)
        }
        Err(e) => {
            assert!(text.is_empty() && bytes.is_empty(), "nothing is written on refusal");
            Err(e)
        }
    }
}

fn same(domain: &str, payload: &Value, omit: Option<&str>) -> Result<String, String> {
    let expected = former(domain, payload, omit);
    assert_eq!(streamed(domain, payload, omit), expected, "domain {domain:?} omit {omit:?} payload {payload}");
    expected
}

fn nest(depth: usize, leaf: Value, object: bool) -> Value {
    (0..depth).fold(leaf, |inner, i| if object && i % 2 == 1 { json!({ "k\"\u{1}é": inner }) } else { json!([inner]) })
}

/// Payload container nesting d sits at levels 2..=d+1 under the wrapper: the
/// parser accepts d <= 126 and refuses d >= 127 (the 128th container).
#[test]
fn recursion_limit_at_minus_one_at_and_past_the_boundary() {
    for object in [false, true] {
        for depth in 120..=131 {
            let payload = nest(depth, json!("x"), object);
            let result = same("d", &payload, None);
            assert_eq!(result.is_ok(), depth <= 126, "depth {depth} object {object}: {result:?}");
            if depth > 126 {
                assert!(result.unwrap_err().starts_with("CHECKED-JSON-INVALID: recursion limit exceeded at line 1 column "));
            }
        }
    }
    // Empty containers count as containers at their level.
    for depth in [125, 126, 127] {
        same("d", &nest(depth, json!([]), false), None).ok();
        same("d", &nest(depth, json!({}), true), None).ok();
    }
}

/// The column is the deep container's byte position in the serialized text:
/// exercise prefixes of escapes, multibyte text, numbers, map order and an
/// omitted member.
#[test]
fn recursion_refusal_column_tracks_the_serialized_prefix() {
    let deep = nest(127, json!(1), false);
    let ok = nest(126, json!(1), false);
    let prefixes = [
        json!(null), json!(true), json!(false), json!(-0.0), json!(1e300), json!(5e-324), json!(u64::MAX), json!(i64::MIN),
        json!("\u{0}\u{1f}\"\\/\u{7f}\u{2028}😀ｚ"), json!([[], {}, [1.5, "a"]]), json!({"z": 1, "a": [2.0]}),
    ];
    for (i, prefix) in prefixes.iter().enumerate() {
        let domain = format!("dom\u{8}ain-{i}-é");
        same(&domain, &json!({"b": prefix, "c": deep, "a": prefix}), None).unwrap_err();
        same(&domain, &json!([prefix, prefix, deep, prefix]), None).unwrap_err();
        same(&domain, &json!({"b": prefix, "c": ok, "a": deep}), None).unwrap_err();
        // Within an object less one member, the next level down is one shallower. An accepted depth
        // still refuses an unsafe prefix number.
        let prefix_ok = same("p", prefix, None).is_ok();
        let inner = nest(126, json!(1), false);
        assert_eq!(same(&domain, &json!({"b": prefix, "r": deep, "a": prefix}), Some("r")).is_ok(), prefix_ok);
        same(&domain, &json!({"b": prefix, "r": prefix, "c": inner}), Some("r")).unwrap_err();
        assert_eq!(same(&domain, &json!({"b": prefix, "r": prefix, "c": nest(125, json!(1), false)}), Some("r")).is_ok(), prefix_ok);
        same(&domain, &json!({"r": deep, "c": inner, "a": prefix}), Some("r")).unwrap_err();
    }
}

/// The depth refusal precedes every validation refusal, wherever it sits.
#[test]
fn recursion_refusal_precedes_validation() {
    let unsafe_int = json!(MAX_SAFE_INTEGER + 1);
    let deep = nest(127, json!(1), false);
    let e = same("d", &json!({"a": unsafe_int, "z": deep}), None).unwrap_err();
    assert!(e.contains("recursion limit exceeded"), "{e}");
}

#[test]
fn numbers_at_the_edges() {
    let safe = MAX_SAFE_INTEGER;
    let values = [
        json!(0), json!(-1), json!(1), json!(safe), json!(-safe), json!(safe + 1), json!(-safe - 1),
        json!(i64::MAX), json!(i64::MIN), json!(u64::MAX), json!(i64::MAX as u64 + 1),
        json!(0.0), json!(-0.0), json!(1.0), json!(-1.0), json!(0.1), json!(1e-7), json!(1e-6), json!(1e21), json!(1e20),
        json!(5e-324), json!(-5e-324), json!(2.2250738585072014e-308), json!(f64::MAX), json!(f64::MIN),
        json!(9007199254740991.0), json!(9007199254740992.0), json!(9007199254740993.5), json!(4503599627370495.5),
        json!(1.7976931348623157e308), json!(123456789.987654), json!(333333333.3333332),
    ];
    for v in &values {
        same("n", v, None).ok();
        same("n", &json!([v]), None).ok();
        same("n", &json!({"v": v, "r": v}), Some("r")).ok();
    }
    assert_eq!(same("n", &json!(-0.0), None).unwrap(), r#"{"domain":"n","payload":0}"#);
    assert!(same("n", &json!(f64::MAX), None).unwrap_err().starts_with("CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT"));
    assert!(same("n", &json!(u64::MAX), None).unwrap_err().starts_with("CHECKED-JSON-UNSAFE-INTEGER"));
    assert!(same("n", &json!(i64::MIN), None).unwrap_err().starts_with("CHECKED-JSON-UNSAFE-INTEGER"));
    // The first refusal in map order is the one reported.
    let e = same("n", &json!({"b": safe + 1, "a": [1e300], "c": -safe - 2}), None).unwrap_err();
    assert_eq!(e, format!("CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT: {}", 1e300f64));
    // An omitted member's refusal is not reached.
    same("n", &json!({"a": 1, "r": safe + 1}), Some("r")).unwrap();
}

/// Deterministic xorshift over raw bit patterns: every finite double, in a
/// value, an array and a member, refuses or renders alike.
#[test]
fn random_doubles_round_trip_alike() {
    let mut x: u64 = 0x9e37_79b9_7f4a_7c15;
    let mut count = 0;
    while count < 20_000 {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        let v = f64::from_bits(x);
        if let Some(n) = Number::from_f64(v) {
            let number = Value::Number(n);
            same("f", &json!([number, {"k": number}]), None).ok();
            count += 1;
        }
    }
}

#[test]
fn strings_keys_and_ordering() {
    let mut all_controls = String::new();
    for c in 0u32..0x80 {
        all_controls.push(char::from_u32(c).unwrap());
    }
    let strings = [
        "", "a", "\"", "\\", "/", "\u{0}", "\u{1f}", "\u{7f}", "\u{80}", "\u{2028}\u{2029}", "\u{fffd}", "\u{ffff}",
        "😀", "\u{10ffff}", "ｚ", "e\u{301}", &all_controls,
    ];
    let mut map = Map::new();
    for (i, s) in strings.iter().enumerate() {
        map.insert(s.to_string(), json!([s, i, {}, []]));
    }
    let payload = Value::Object(map);
    let text = same("s", &payload, None).unwrap();
    // UTF-16 order puts the surrogate pairs (U+1F600, U+10FFFF) before U+FF5A and U+FFFD.
    assert!(text.find("\"😀\"").unwrap() < text.find("\"ｚ\"").unwrap());
    for omit in strings {
        same("s", &payload, Some(omit)).unwrap();
    }
    same("s", &payload, Some("absent member")).unwrap();
    for domain in strings {
        same(domain, &payload, None).unwrap();
    }
}

#[test]
fn empty_containers_and_scalars() {
    for v in [json!(null), json!(true), json!(false), json!(""), json!([]), json!({}), json!([[]]), json!({"": {}}), json!([{}, [], ""])] {
        same("", &v, None).unwrap();
    }
    same("e", &json!({"r": 1}), Some("r")).unwrap();
    assert_eq!(same("e", &json!({"r": 1}), Some("r")).unwrap(), r#"{"domain":"e","payload":{}}"#);
}

/// Random trees of every kind of value, shallow and deep, with and without an omitted member.
#[test]
fn random_trees_alike() {
    let mut x: u64 = 0x2545_f491_4f6c_dd1d;
    let mut next = move || {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        x
    };
    fn tree(next: &mut dyn FnMut() -> u64, depth: usize, max: usize) -> Value {
        let r = next();
        let kind = if depth >= max { r % 6 } else { r % 8 };
        match kind {
            0 => Value::Null,
            1 => json!(r & 1 == 0),
            2 => json!((next() as i64) >> (next() % 64)),
            3 => Number::from_f64(f64::from_bits(next())).map(Value::Number).unwrap_or(json!(0.5)),
            4 => json!(next() >> (next() % 64)),
            5 => {
                let len = (next() % 6) as usize;
                Value::String((0..len).map(|_| char::from_u32((next() % 0x11_0000) as u32).unwrap_or('\u{1}')).collect())
            }
            6 => Value::Array((0..next() % 4).map(|_| tree(next, depth + 1, max)).collect()),
            _ => Value::Object(
                (0..next() % 4).map(|i| (format!("{}{}", ["a", "é", "😀", "\u{1}"][(next() % 4) as usize], i), tree(next, depth + 1, max))).collect(),
            ),
        }
    }
    for i in 0..3000 {
        let max = 6;
        let t = if i % 10 == 0 {
            // A random leaf under a spine near the parse depth, between random siblings.
            let leaf = tree(&mut next, 1, max);
            let spine = nest(118 + (i / 10) % 14, leaf, i % 20 == 0);
            json!([tree(&mut next, 1, max), spine, tree(&mut next, 1, max)])
        } else {
            tree(&mut next, 1, max)
        };
        same("t", &t, None).ok();
        let wrapped = json!({"r": t, "p": tree(&mut next, 1, max)});
        same("t", &wrapped, Some("r")).ok();
        same("t", &wrapped, Some("p")).ok();
    }
}
