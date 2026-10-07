//! Standalone prepared-result statement checks; never proof of producer origin.
use open_pipe_stress_units::{canonical_unit, convert_for_dimension, unit_by_symbol, Dimension};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

pub const CONTRACT_ID: &str = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1";
pub const PROFILE: &str = "product_preview_retained_w1a_v2";
pub const DEFINITION_ID: &str = "RP-PREPARED-ORDINARY-DUAL-v1";
pub const DEFINITION_HASH: &str =
    "a7ed7ca0bf0bba6e8b821ca4befa00a0fa9541a83694be8b28ac63e39b1d0349";
pub const METHOD: &str = "contribution_preserving_multiprecision_v1";
const MAX_BITS: u64 = 0x7fef_ffff_ffff_ffff;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub gate: &'static str,
    /// The bare failure code compared for first-failure parity.
    pub code: String,
    /// Optional detail carried separately from the code (G7 base detail).
    pub detail: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccuracyClass {
    RelativeVerified,
    AbsoluteVerified { bound_bits: u64 },
    InputDerived,
    NonQuantity,
    NotCovered,
}
#[derive(Debug, Clone, PartialEq)]
pub struct RowClassification {
    pub result_id: String,
    pub basis_ref: Value,
    pub normalized_bits: u64,
    pub scale_bits: Option<u64>,
    pub class: AccuracyClass,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Validation {
    pub invocation_bound: bool,
    pub numerical_eligible: bool,
    pub publication_sha256: String,
    pub classifications: Vec<RowClassification>,
}

fn parts(value: f64) -> Result<(u64, i32), &'static str> {
    if !value.is_finite() || value < 0.0 {
        return Err("nonnegative finite operand required");
    }
    let bits = value.to_bits() & !(1 << 63);
    let e = (bits >> 52) as i32;
    let m = bits & ((1 << 52) - 1);
    Ok(if e == 0 {
        (m, -1074)
    } else {
        (m | (1 << 52), e - 1075)
    })
}
fn encode_quantum(mut q: u64, mut e: i32) -> Result<f64, &'static str> {
    if q == 0 {
        return Ok(0.0);
    }
    if q == 1 << 53 {
        q >>= 1;
        e += 1;
    }
    if q >= 1 << 53 {
        return Err("rounding invariant");
    }
    let top = 63 - q.leading_zeros() as i32 + e;
    if top > 1023 {
        return Err("binary64 upper bound overflows");
    }
    let bits = if top < -1022 {
        q.checked_shl((e + 1074) as u32).ok_or("shift range")?
    } else {
        (((top + 1023) as u64) << 52) | ((q << (53 - (64 - q.leading_zeros()))) - (1 << 52))
    };
    if bits > MAX_BITS {
        return Err("binary64 upper bound overflows");
    }
    Ok(f64::from_bits(bits))
}
/// Only the product of two finite nonnegative binary64 operands: <=106 bits.
pub fn upward_product(a: f64, b: f64) -> Result<f64, &'static str> {
    let (ma, ea) = parts(a)?;
    let (mb, eb) = parts(b)?;
    let m = u128::from(ma)
        .checked_mul(u128::from(mb))
        .ok_or("product range")?;
    if m == 0 {
        return Ok(0.0);
    }
    let e = ea + eb;
    let length = 128 - m.leading_zeros() as i32;
    let quantum = (e + length - 53).max(-1074);
    let shift = quantum - e;
    let q = if shift >= length {
        1
    } else if shift > 0 {
        (m >> shift) + u128::from(m & ((1u128 << shift) - 1) != 0)
    } else {
        m.checked_shl((-shift) as u32).ok_or("product shift")?
    };
    encode_quantum(u64::try_from(q).map_err(|_| "product quantum")?, quantum)
}
/// Exact three-term positive sum in minimum-subnormal units. No heap arithmetic.
pub fn upward_small_sum(b0: f64, rounding: f64) -> Result<f64, &'static str> {
    let mut limbs = [0u64; 33];
    limbs[0] = 1;
    for value in [b0, rounding] {
        let (m, e) = parts(value)?;
        let shift = (e + 1074) as usize;
        let at = shift / 64;
        let offset = shift % 64;
        let mut carry = u128::from(m) << offset;
        let mut i = at;
        while carry != 0 {
            let cell = limbs.get_mut(i).ok_or("sum capacity")?;
            let s = u128::from(*cell) + (carry & u128::from(u64::MAX));
            *cell = s as u64;
            carry = (carry >> 64) + (s >> 64);
            i += 1;
        }
    }
    let last = limbs.iter().rposition(|&x| x != 0).ok_or("sum invariant")?;
    let top = last * 64 + (63 - limbs[last].leading_zeros() as usize);
    let discard = top.saturating_sub(52);
    let word = discard / 64;
    let offset = discard % 64;
    let mut q = limbs[word] >> offset;
    if offset != 0 && word + 1 < 33 {
        q |= limbs[word + 1] << (64 - offset);
    }
    let tail = limbs[..word].iter().any(|&x| x != 0)
        || (offset != 0 && limbs[word] & ((1u64 << offset) - 1) != 0);
    if tail {
        q = q.checked_add(1).ok_or("sum quantum")?;
    }
    encode_quantum(q, discard as i32 - 1074)
}
fn scaled_component(x: f64, power: i32) -> Result<f64, &'static str> {
    if !x.is_finite() || x < 0.0 || !matches!(power, 53 | 64) {
        return Err("scaled component input");
    }
    if x == 0.0 {
        return Ok(0.0);
    }
    let d = f64::from_bits(((1023 + power) as u64) << 52);
    let nearest = x / d;
    let back = nearest * d;
    Ok(if back < x {
        f64::from_bits(nearest.to_bits() + 1)
    } else {
        nearest
    })
}
/// Signed zero magnitude inputs are accepted and produce canonical positive zero.
/// Negative-zero wire scales remain forbidden by G2.
pub fn absolute_bound(value: f64, scale: f64) -> Result<f64, &'static str> {
    if !value.is_finite() || !scale.is_finite() || scale < 0.0 {
        return Err("bound input");
    }
    let b0 = scaled_component(scale, 64)?;
    if scale > 0.0 && scale < f64::from_bits(0x0230_0000_0000_0000) {
        upward_small_sum(b0, scaled_component(value.abs(), 53)?)
    } else {
        Ok(b0)
    }
}

/// Item 6a's coupling in binary64, mirroring FK/verify.rs `e_hat`:
/// ê_fo = max(E_fo, fl(E_mo/L)), ê_mo = max(E_mo, fl(L·E_fo)); L = 0 keeps E.
pub fn e_hat(e: [f64; 2], extent: f64) -> [f64; 2] {
    if extent == 0.0 {
        return e;
    }
    let [fo, mo] = e;
    [fo.max(mo / extent), mo.max(extent * fo)]
}
/// The native p512 floor Φ = fl↑(2^-438·ê), mirroring FK/verify.rs `phi_512`:
/// the nearest product, then the next binary64 up when it is below the exact
/// product (decided exactly by scaling back with 2^438).
pub fn phi_512(e_hat: f64) -> f64 {
    let scale = f64::from_bits(0x2490_0000_0000_0000); // 2^-438
    let back = f64::from_bits(0x5B50_0000_0000_0000); // 2^438
    let nearest = e_hat * scale;
    if nearest * back < e_hat {
        if nearest == 0.0 {
            f64::from_bits(1)
        } else {
            f64::from_bits(nearest.to_bits() + 1)
        }
    } else {
        nearest
    }
}

type VResult<T = ()> = Result<T, ValidationError>;
const SAFE: u64 = (1u64 << 53) - 1;
const NAMES: [&str; 4] = ["translation", "rotation", "force", "moment"];
const DOFS: [&str; 6] = ["UX", "UY", "UZ", "RX", "RY", "RZ"];
const SLOTS: [&str; 7] = ["s128", "s256", "s512", "s1024", "v256", "v512", "v1024"];
fn error(gate: &'static str, suffix: &str) -> ValidationError {
    ValidationError {
        gate,
        detail: None,
        code: if suffix.starts_with("SOURCE_") {
            suffix.into()
        } else {
            format!("RETAINED_PRECISION_{suffix}")
        },
    }
}
fn need(ok: bool, gate: &'static str, suffix: &str) -> VResult {
    if ok {
        Ok(())
    } else {
        Err(error(gate, suffix))
    }
}
fn list(v: &Value) -> &[Value] {
    v.as_array().map(Vec::as_slice).unwrap_or(&[])
}
fn text(v: &Value) -> &str {
    v.as_str().unwrap_or("")
}
fn uint(v: &Value) -> Option<u64> {
    let n = v.as_f64()?;
    if n.is_finite()
        && n >= 0.0
        && n <= SAFE as f64
        && n.fract() == 0.0
        && !(n == 0.0 && n.is_sign_negative())
    {
        Some(n as u64)
    } else {
        None
    }
}
fn u(v: &Value) -> u64 {
    uint(v).unwrap_or(u64::MAX)
}
/// D32: after G2 every receipt number is a U or I32 by value, so a number
/// written as an integral float (`17.0`) is rewritten once as the integer it
/// denotes. Later equality and reference checks then see one encoding per
/// value. Canonical hashing renders both forms identically, and `-0` or a
/// non-integral value is left as written.
fn integral_receipt(source: &Value) -> std::borrow::Cow<'_, Value> {
    fn floats(v: &Value) -> bool {
        match v {
            Value::Number(n) => n.is_f64(),
            Value::Array(a) => a.iter().any(floats),
            Value::Object(o) => o.values().any(floats),
            _ => false,
        }
    }
    fn normalize(v: &mut Value) {
        let integer = match v {
            Value::Number(n) if n.is_f64() => n.as_f64().filter(|x| {
                x.is_finite()
                    && x.fract() == 0.0
                    && x.abs() <= SAFE as f64
                    && !(*x == 0.0 && x.is_sign_negative())
            }),
            Value::Array(a) => {
                a.iter_mut().for_each(normalize);
                None
            }
            Value::Object(o) => {
                o.values_mut().for_each(normalize);
                None
            }
            _ => None,
        };
        if let Some(x) = integer {
            *v = if x < 0.0 { json!(x as i64) } else { json!(x as u64) };
        }
    }
    if !floats(&source["retained_precision"]) {
        return std::borrow::Cow::Borrowed(source);
    }
    let mut owned = source.clone();
    if let Some(r) = owned.get_mut("retained_precision") {
        normalize(r);
    }
    std::borrow::Cow::Owned(owned)
}
fn at<'a>(a: &'a Value, i: &Value, gate: &'static str, code: &str) -> VResult<&'a Value> {
    uint(i)
        .and_then(|i| usize::try_from(i).ok())
        .and_then(|i| a.as_array()?.get(i))
        .ok_or_else(|| error(gate, code))
}
fn raw_bits(v: &Value) -> Option<u64> {
    let t = v.as_str()?;
    if t.len() != 16
        || !t
            .bytes()
            .all(|x| x.is_ascii_digit() || (b'a'..=b'f').contains(&x))
    {
        return None;
    }
    let b = u64::from_str_radix(t, 16).ok()?;
    f64::from_bits(b).is_finite().then_some(b)
}
fn f(v: &Value) -> f64 {
    raw_bits(v).map(f64::from_bits).unwrap_or(f64::NAN)
}
fn bits(v: f64) -> Value {
    json!(format!("{:016x}", v.to_bits()))
}
fn sum(values: impl IntoIterator<Item = u64>) -> VResult<u64> {
    values.into_iter().try_fold(0u64, |a, b| {
        a.checked_add(b)
            .filter(|n| *n <= SAFE)
            .ok_or_else(|| error("G5", "WORK_MISMATCH"))
    })
}
fn hash(domain: &str, v: &Value, gate: &'static str, code: &str) -> VResult<String> {
    crate::source_blocks::domain_hash(domain, v).map_err(|_| error(gate, code))
}
fn schema() -> &'static Value {
    static S: OnceLock<Value> = OnceLock::new();
    S.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../../../../schemas/retained_precision_mp_v2.schema.json"
        ))
        .expect("packaged retained schema")
    })
}
fn definition() -> &'static Value {
    static S: OnceLock<Value> = OnceLock::new();
    S.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../../../../fixtures/results/retained_precision_prepared_ordinary_v1.json"
        ))
        .expect("packaged retained definition")
    })
}
fn table() -> &'static Value {
    static S: OnceLock<Value> = OnceLock::new();
    S.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../../../../fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json"
        ))
        .expect("packaged retained table")
    })
}
fn equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => a.as_f64() == b.as_f64(),
        _ => a == b,
    }
}
// This intentionally separates syntactic shape from canonical numeric encoding.
// Applying a general schema validator here would move G2 errors to G1.
fn shape(v: &Value, s: &Value) -> bool {
    if let Some(r) = s["$ref"].as_str() {
        return schema()
            .pointer(r.trim_start_matches('#'))
            .is_some_and(|s| shape(v, s));
    }
    if let Some(bs) = s["oneOf"].as_array() {
        return bs.iter().filter(|s| shape(v, s)).count() == 1;
    }
    if let Some(c) = s.get("const") {
        if !equal(v, c) {
            return false;
        }
    }
    if let Some(es) = s["enum"].as_array() {
        if !es.iter().any(|e| equal(v, e)) {
            return false;
        }
    }
    match s["type"].as_str() {
        Some("object") => v.as_object().is_some_and(|o| {
            list(&s["required"]).iter().all(|k| o.contains_key(text(k)))
                && o.iter()
                    .all(|(k, v)| s["properties"].get(k).is_some_and(|s| shape(v, s)))
        }),
        Some("array") => v.as_array().is_some_and(|a| {
            a.len() >= s["minItems"].as_u64().unwrap_or(0) as usize
                && a.len() as u64 <= s["maxItems"].as_u64().unwrap_or(SAFE)
                && a.iter().all(|v| shape(v, &s["items"]))
        }),
        Some("string") => v
            .as_str()
            .is_some_and(|v| v.chars().count() >= s["minLength"].as_u64().unwrap_or(0) as usize),
        Some("number" | "integer") => v.is_number(),
        Some("boolean") => v.is_boolean(),
        Some("null") => v.is_null(),
        None => true,
        _ => false,
    }
}
fn encoding(v: &Value, s: &Value) -> VResult {
    if let Some(r) = s["$ref"].as_str() {
        return encoding(
            v,
            schema()
                .pointer(r.trim_start_matches('#'))
                .ok_or_else(|| error("G2", "ENCODING_MISMATCH"))?,
        );
    }
    if let Some(bs) = s["oneOf"].as_array() {
        return encoding(
            v,
            bs.iter()
                .find(|s| shape(v, s))
                .ok_or_else(|| error("G2", "ENCODING_MISMATCH"))?,
        );
    }
    let valid = match text(&s["x-rp-encoding"]) {
        "uint" => uint(v).is_some(),
        "i32" => v.as_f64().is_some_and(|n| {
            n.fract() == 0.0
                && n >= i32::MIN as f64
                && n <= i32::MAX as f64
                && !(n == 0.0 && n.is_sign_negative())
        }),
        "bits" => raw_bits(v).is_some(),
        "nonnegative_bits" => raw_bits(v).is_some_and(|b| b >> 63 == 0),
        "hash" => v.as_str().is_some_and(|t| {
            t.len() == 64
                && t.bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        }),
        _ => true,
    };
    need(valid, "G2", "ENCODING_MISMATCH")?;
    if let Some(o) = v.as_object() {
        for (k, v) in o {
            if let Some(p) = s["properties"].get(k) {
                encoding(v, p)?;
            }
        }
    }
    if s["type"] == "array" {
        for v in list(v) {
            encoding(v, &s["items"])?;
        }
    }
    Ok(())
}
/// G2: the receipt's wire encodings, then D34 (C1 §4 "canonical +0, never
/// negative zero"; C1's G2 row): a JSON number equal to -0 anywhere in the
/// receipt is a non-canonical spelling, including the integer fields the
/// schema writes as enum or const values (`G5aError.quantity_kind`,
/// `source_decline.constructor_counts.directional_springs`), which the
/// value-based shape check admits.
fn g2(source: &Value) -> VResult {
    fn negative_zero(v: &Value) -> bool {
        match v {
            Value::Number(n) => n.as_f64().is_some_and(|x| x == 0.0 && x.is_sign_negative()),
            Value::Array(a) => a.iter().any(negative_zero),
            Value::Object(o) => o.values().any(negative_zero),
            _ => false,
        }
    }
    let r = &source["retained_precision"];
    encoding(r, schema())?;
    need(!negative_zero(r), "G2", "ENCODING_MISMATCH")
}
fn source_hash(s: &Value) -> VResult<String> {
    let mut x = s.clone();
    x.as_object_mut()
        .ok_or_else(|| error("G1", "RECEIPT_MISMATCH"))?
        .remove("index");
    hash(
        "retained_precision_source_mp_v2",
        &x,
        "G1",
        "RECEIPT_MISMATCH",
    )
}
fn preparation_payload(a: &Value) -> VResult<Value> {
    let mut members = Vec::new();
    for m in list(&a["preparation"]["members"]) {
        need(m["result"]["kind"] == "prepared", "G1", "RECEIPT_MISMATCH")?;
        members.push(json!({"member":m["member"],"old_source":m["old_source"],"old_facts":m["old_facts"],"section":m["result"]["section"]}));
    }
    Ok(
        json!({"definition_id":a["definition_id"],"definition_sha256":DEFINITION_HASH,"owner_ref":a["owner_ref"],"ordinary_attempt_ref":a["ordinary_attempt_ref"],"material_basis_ref":a["material_basis_ref"],"members":members}),
    )
}
/// The bound inherited preview table and this successor table, as bytes.
const TABLE_BYTES: &[u8] = include_bytes!(
    "../../../../fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json"
);
const INHERITED_TABLE_BYTES: &[u8] =
    include_bytes!("../../../../fixtures/results/semantic_contract_v0_3_preview_physics_1.json");
const TABLE_HASH: &str = "c74742ce6a936384e00986006e6a0b2e6bb11f190451e876eed9ffa11903c6a8";
fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
/// D2 (C1/C3 G0 rows): exactly the producer identity (component and schema
/// versions), the definition and inherited-table hashes over the bound
/// bytes, `receipt_version`, the policy ids, the canonicalization profile and
/// the 20B/60B thresholds. A G0 field that is absent or of the wrong type
/// fails G0; every other shape defect waits for G1.
fn g0(source: &Value) -> VResult {
    let unsupported = |ok| need(ok, "G0", "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED");
    unsupported(
        source.is_object()
            && source["producer"]["semantic_contract_id"] == CONTRACT_ID
            && source["formulation_basis"]["profile_id"] == PROFILE,
    )?;
    unsupported(
        source["schema_version"] == "0.2.0"
            && source["producer"]["component_name"] == "open_pipe_stress_product_physics"
            && source["producer"]["component_version"] == "0.2.0",
    )?;
    unsupported(
        table()["semantic_contract_id"] == CONTRACT_ID
            && table()["formulation_profile_id"] == PROFILE,
    )?;
    need(
        hash(
            "retained_precision_formation_v1",
            definition(),
            "G0",
            "FORMATION_MISMATCH",
        )? == DEFINITION_HASH
            && table()["product_formation_definitions"]
                == json!([{"id":DEFINITION_ID,"sha256":DEFINITION_HASH}]),
        "G0",
        "FORMATION_MISMATCH",
    )?;
    unsupported(
        sha256_hex(TABLE_BYTES) == TABLE_HASH
            && table()["inherited_semantic_contract_sha256"] == sha256_hex(INHERITED_TABLE_BYTES),
    )?;
    let b = &source["retained_precision"]["body"];
    // D32: integers by value (finite, integral, in range, not -0), never by
    // the JSON number's host type: `1.0` is the receipt version 1.
    unsupported(uint(&b["receipt_version"]) == Some(1))?;
    for (key, value) in [
        ("policy", "M03-INTEGRITY-MP-v2"),
        ("projection_policy", "RP-LOGICAL-ATTEMPTS-v1"),
        ("work_policy", "W1-LME-20B-60B-v1"),
        ("facade_policy", "RP-FACADE-SI-v2"),
        ("canonicalization", "openpipestress_jcs_ijson_v1"),
    ] {
        unsupported(b[key] == value)?;
    }
    for (key, want) in [
        ("case_limit", 20_000_000_000u64),
        ("invocation_limit", 60_000_000_000),
    ] {
        unsupported(uint(&b["work"][key]) == Some(want))?;
    }
    for a in list(&b["product_attempts"]) {
        if a.is_object() {
            unsupported(a["definition_id"] == DEFINITION_ID)?;
        }
    }
    Ok(())
}
fn g1(source: &Value, raw: bool) -> VResult {
    let r = &source["retained_precision"];
    need(shape(r, schema()), "G1", "RECEIPT_MISMATCH")?;
    if raw {
        need(
            source["results"].is_array()
                && list(&source["results"])
                    .iter()
                    .all(|r| shape(r, &schema()["$defs"]["RawRow"])),
            "G1",
            "RECEIPT_MISMATCH",
        )?;
    }
    need(
        hash(
            "retained_precision_receipt_mp_v2",
            &r["body"],
            "G1",
            "RECEIPT_MISMATCH",
        )? == r["receipt_sha256"],
        "G1",
        "RECEIPT_MISMATCH",
    )?;
    if raw {
        let mut public = source.clone();
        public
            .as_object_mut()
            .ok_or_else(|| error("G1", "RECEIPT_MISMATCH"))?
            .remove("retained_precision");
        need(
            hash(
                "retained_precision_publication_mp_v2",
                &public,
                "G1",
                "RECEIPT_MISMATCH",
            )? == r["body"]["publication_sha256"],
            "G1",
            "RECEIPT_MISMATCH",
        )?;
    }
    // Defer malformed reference encodings to G2 and owner associations to G3/G5.
    for s in list(&r["body"]["sources"]) {
        if !s["preparation"].is_null() {
            if let Some(a) = uint(&s["preparation"]["attempt_ref"])
                .and_then(|i| list(&r["body"]["product_attempts"]).get(i as usize))
            {
                if list(&a["preparation"]["members"])
                    .iter()
                    .all(|m| m["result"]["kind"] == "prepared")
                {
                    need(
                        hash(
                            "retained_precision_preparation_v1",
                            &preparation_payload(a)?,
                            "G1",
                            "RECEIPT_MISMATCH",
                        )? == s["preparation"]["sha256"],
                        "G1",
                        "RECEIPT_MISMATCH",
                    )?;
                }
            }
        }
    }
    for c in list(&r["body"]["cases"]) {
        if let Some(h) = c.get("source_identity_sha256") {
            if let Some(s) =
                uint(&c["source_ref"]).and_then(|i| list(&r["body"]["sources"]).get(i as usize))
            {
                need(source_hash(s)? == *h, "G1", "RECEIPT_MISMATCH")?;
            }
        }
    }
    Ok(())
}
/// C3 conversion kinds (checklist P5, at G5 PRODUCT_ATTEMPT per snapshot 06a):
/// Normal is a normal binary64 or ±0; Subnormal has nonzero subnormal bits.
/// Underflow/overflow carry no value; their Ready rules are checked with rows.
fn conversion_kind_ok(o: &Value) -> bool {
    let n = f(&o["value"]);
    match text(&o["kind"]) {
        "normal" => n == 0.0 || n.abs() >= f64::MIN_POSITIVE,
        "subnormal" => n != 0.0 && n.abs() < f64::MIN_POSITIVE,
        _ => true,
    }
}
fn rows_for<'a>(source: &'a Value, case: &Value) -> Vec<&'a Value> {
    list(&source["results"])
        .iter()
        .filter(|r| r["basis_ref"] == case["basis_ref"])
        .collect()
}
fn g3(source: &Value, inv: Option<&Value>) -> VResult {
    let b = &source["retained_precision"]["body"];
    let cs = list(&b["cases"]);
    let qs = list(&source["numerical_quality"]["cases"]);
    let fail = |ok| need(ok, "G3", "COVERAGE_MISMATCH");
    fail(!cs.is_empty() && cs.len() == qs.len() && cs.iter().any(|c| c["status"] == "selected"))?;
    let mut ids = BTreeSet::new();
    let mut refs = BTreeSet::new();
    let mut runs = BTreeMap::new();
    for (i, c) in cs.iter().enumerate() {
        fail(
            ids.insert(text(&c["basis_ref"]["ref_id"]))
                && c["basis_ref"] == qs[i]["basis_ref"]
                && c["ordinary"]["quality_binding"] == json!({"kind":"present","index":i})
                && u(&c["ordinary"]["attempt_ref"]) == i as u64,
        )?;
        let o = at(
            &b["ordinary_attempts"],
            &json!(i),
            "G3",
            "COVERAGE_MISMATCH",
        )?;
        fail(u(&o["case_index"]) == i as u64 && o["case_id"] == c["basis_ref"]["ref_id"])?;
        if !c["product_attempt_ref"].is_null() {
            let ai = u(&c["product_attempt_ref"]);
            fail(refs.insert(ai))?;
            let a = at(
                &b["product_attempts"],
                &c["product_attempt_ref"],
                "G3",
                "COVERAGE_MISMATCH",
            )?;
            fail(u(&a["id"]) == ai && a["owner_ref"] == json!({"kind":"case","index":i}))?;
        }
        // D1: a Run's id is its execution-order position; the Run's own
        // origin owner/source are G5 class-1 checks.
        if let Some(r) = c.get("run").filter(|r| !r.is_null()) {
            fail(
                runs.insert(u(&r["id"]), json!({"kind":"case","index":i}))
                    .is_none(),
            )?;
        }
    }
    fail(
        list(&b["ordinary_attempts"]).len() == cs.len()
            && refs
                .into_iter()
                .eq(0..list(&b["product_attempts"]).len() as u64),
    )?;
    fail(
        runs.keys().copied().eq(0..runs.len() as u64)
            && list(&b["work"]["execution_order"]).iter().eq(runs.values()),
    )?;
    if let Some(i) = inv {
        fail(
            list(&i["request"]["model"]["load_cases"])
                .iter()
                .map(|x| &x["id"])
                .eq(cs.iter().map(|x| &x["basis_ref"]["ref_id"])),
        )?;
    }
    let mut rowids = BTreeSet::new();
    for r in list(&source["results"]) {
        fail(
            rowids.insert(text(&r["id"]))
                && r["basis_ref"]["ref_type"] == "load_case"
                && ids.contains(text(&r["basis_ref"]["ref_id"])),
        )?;
    }
    // D29 (source.rs `PrimitiveSource::new` NoNodes; I57 s1): every CaseSource has a
    // non-empty body inventory.
    for s in list(&b["sources"]) {
        fail(!list(&s["body_membership"]).is_empty())?;
    }
    for (i, a) in list(&b["product_attempts"]).iter().enumerate() {
        fail(u(&a["id"]) == i as u64)?;
        let old = list(&a["operational"]["old"]);
        let pm = list(&a["preparation"]["members"]);
        let new = list(&a["operational"]["new"]);
        // D1 (C2:98; F1:78-84, 101, 130; retained_product.rs `ProductCapture::capture_case_source`): old, prepared and new
        // member ids are exactly 0..len-1 in native order, so prepared and new
        // are prefixes of old.
        fail(
            new.len() <= pm.len()
                && pm.len() <= old.len()
                && old.iter().map(|m| u(&m["member"])).eq(0..old.len() as u64)
                && pm.iter().map(|m| u(&m["member"])).eq(0..pm.len() as u64)
                && new.iter().map(|m| u(&m["member"])).eq(0..new.len() as u64),
        )?;
        // D1: a captured prefix has no prepared or new members (G3); its null
        // source/run and unavailable result are C3 references (G5).
        if a["operational"]["old_coverage"] == "captured_prefix" {
            fail(pm.is_empty() && new.is_empty())?;
        }
        // D1 (checkpoint-A ruling): an unsourced complete old inventory has the
        // member count of every CaseSource in the receipt (one model); with no
        // CaseSource, G8 compares it with the invocation. Emptiness alone is
        // not rejected (no native rejection is cited).
        if a["source_ref"].is_null() && a["operational"]["old_coverage"] == "complete" {
            fail(
                list(&b["sources"])
                    .iter()
                    .all(|s| list(&s["id_maps"]["members"]).len() == old.len()),
            )?;
        }
        // D22 (D16; C3:146-148): a dangling attempt source_ref is a C3
        // reference defect (G5 PRODUCT_ATTEMPT); the source-dependent G3
        // checks run only when the reference resolves.
        if let Ok(s) = at(&b["sources"], &a["source_ref"], "G3", "COVERAGE_MISMATCH") {
            fail(
                old.iter()
                    .map(|m| &m["member"])
                    .eq(list(&s["id_maps"]["members"])
                        .iter()
                        .map(|m| &m["kernel_member"])),
            )?;
        }
        let c = at(
            &b["cases"],
            &a["owner_ref"]["index"],
            "G3",
            "COVERAGE_MISMATCH",
        )?;
        let rows = rows_for(source, c);
        let mut prior = None;
        for o in list(&a["proof"]["projection_outcomes"]) {
            let idx = u(&o["row_index"]);
            // C3 G3 row-index coverage (ROOT ruling): an index must name a
            // hull-projected row of this case, ascending and unique.
            fail(
                idx < rows.len() as u64
                    && prior.is_none_or(|p| idx > p)
                    && hull_projected(rows[idx as usize]),
            )?;
            prior = Some(idx);
        }
        // I57 s4 G3: a complete proof-owned coverage vector has exactly one entry
        // per native body of the source associated through this attempt, in
        // ascending body order 0..body_count-1. There is no empty complete vector
        // (a complete source has at least one body). A null source reference is
        // left to the G5 same-source binding; an out-of-range one already fails
        // the existing G3 member association above.
        let coverage = &a["proof"]["summary_coverage"];
        let resolved = at(&b["sources"], &a["source_ref"], "G3", "COVERAGE_MISMATCH");
        if let (false, Ok(s)) = (coverage.is_null(), resolved) {
            let inventory = list(&s["body_membership"]);
            fail(
                !inventory.is_empty()
                    && inventory
                        .iter()
                        .map(|x| u(&x["body"]))
                        .eq(0..inventory.len() as u64)
                    && list(coverage)
                        .iter()
                        .map(|e| u(&e["body"]))
                        .eq(0..inventory.len() as u64),
            )?;
        }
    }
    Ok(())
}
fn g4(source: &Value) -> VResult {
    let cs = list(&source["retained_precision"]["body"]["cases"]);
    let ds = list(&source["diagnostics"]);
    let fail = |ok| need(ok, "G4", "DIAGNOSTIC_MISMATCH");
    let mut ids = BTreeSet::new();
    for d in ds {
        fail(
            d["id"].is_string()
                && ids.insert(text(&d["id"]))
                && d["code"] != "SOURCE_BLOCK_RECOVERY_SELECTED",
        )?;
        if matches!(
            text(&d["code"]),
            "RETAINED_PRECISION_SELECTED" | "RETAINED_PRECISION_UNAVAILABLE"
        ) {
            fail(
                list(&d["affected_refs"]).len() == 1
                    && cs
                        .iter()
                        .any(|c| d["affected_refs"][0] == c["basis_ref"]["ref_id"]),
            )?;
        }
    }
    for c in cs {
        let id = &c["basis_ref"]["ref_id"];
        let selected: Vec<_> = ds
            .iter()
            .filter(|d| {
                d["code"] == "RETAINED_PRECISION_SELECTED" && list(&d["affected_refs"]).contains(id)
            })
            .collect();
        let unavailable: Vec<_> = ds
            .iter()
            .filter(|d| {
                d["code"] == "RETAINED_PRECISION_UNAVAILABLE"
                    && list(&d["affected_refs"]).contains(id)
            })
            .collect();
        fail(
            selected.len() == usize::from(c["status"] == "selected")
                && unavailable.len() == usize::from(c["status"] == "unavailable"),
        )?;
        if c["status"] == "unavailable" {
            fail(unavailable[0]["id"] == c["diagnostic_ref"])?;
        }
        if c["status"] == "selected" {
            fail(!ds.iter().any(|d| {
                d["code"] == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE"
                    && list(&d["affected_refs"]).contains(id)
            }))?;
        }
    }
    Ok(())
}
fn stages_sum(v: &Value) -> VResult<u64> {
    sum(v
        .as_object()
        .ok_or_else(|| error("G5", "WORK_MISMATCH"))?
        .values()
        .map(u))
}
fn g5_native(b: &Value) -> VResult {
    let af = |ok| need(ok, "G5", "ATTEMPT_MISMATCH");
    // D3 (C3:304 convention): native WORK predicates are deferred to the end
    // of class 1, so a class containing an ATTEMPT defect reports ATTEMPT.
    let work_ok = std::cell::Cell::new(true);
    let wf = |ok: bool| -> VResult {
        if !ok {
            work_ok.set(false);
        }
        Ok(())
    };
    let tw = |r: VResult<u64>| -> u64 {
        r.unwrap_or_else(|_| {
            work_ok.set(false);
            u64::MAX
        })
    };
    let mut runs: Vec<_> = list(&b["cases"])
        .iter()
        .enumerate()
        .filter_map(|(ci, c)| c.get("run").filter(|r| !r.is_null()).map(|r| (ci, c, r)))
        .collect();
    runs.sort_by_key(|(_, _, r)| u(&r["id"]));
    // D8 kernel scope (checkpoint A; C1:66-68): a work_accounting stop or reason
    // anywhere in a Run, a build or a group preparation is outside the emitted
    // domain (adaptive.rs `run_schedule_inner`, `finish_terminal`, `solve_cases_projected`).
    for item in runs
        .iter()
        .map(|(_, _, r)| *r)
        .chain(list(&b["builds"]))
        .chain(list(&b["groups"]).iter().map(|g| &g["preparation"]))
    {
        let mut objs = Vec::new();
        objects(item, &mut objs);
        af(!objs
            .iter()
            .any(|o| o.get("tag").is_some_and(|t| t == "work_accounting")))?;
    }
    let mut current = 0;
    let mut run_order = Vec::new();
    let mut builds_seen = BTreeSet::new();
    let mut group_caches: BTreeMap<u64, BTreeMap<String, u64>> = BTreeMap::new();
    for (i, g) in list(&b["groups"]).iter().enumerate() {
        af(u(&g["id"]) == i as u64
            && !list(&g["source_refs"]).is_empty()
            && g["first_source_ref"] == g["source_refs"][0])?;
        let call = at(&b["calls"], &g["call"], "G5", "ATTEMPT_MISMATCH")?;
        let mut sources = BTreeSet::new();
        for si in list(&g["source_refs"]) {
            let s = at(&b["sources"], si, "G5", "ATTEMPT_MISMATCH")?;
            af(sources.insert(u(si))
                && list(&call["source_refs"]).contains(si)
                && s["stiffness_sha256"] == g["stiffness_sha256"])?;
        }
        group_caches.insert(i as u64, BTreeMap::new());
    }
    for (i, build) in list(&b["builds"]).iter().enumerate() {
        wf(u(&build["id"]) == i as u64 && u(&build["work"]) == tw(stages_sum(&build["stages"])))?;
    }
    for (call_id, call) in list(&b["calls"]).iter().enumerate() {
        af(u(&call["id"]) == call_id as u64
            && list(&call["run_refs"]).len() == list(&call["source_refs"]).len()
            && list(&call["run_refs"]).len() == list(&call["owner_refs"]).len())?;
        wf(u(&call["invocation_before"]) == current)?;
        for (position, ri) in list(&call["run_refs"]).iter().enumerate() {
            let (ci, c, r) = runs
                .get(u(ri) as usize)
                .copied()
                .ok_or_else(|| error("G5", "ATTEMPT_MISMATCH"))?;
            run_order.push(u(ri));
            let si = &call["source_refs"][position];
            let oi = &call["owner_refs"][position];
            af(r["origin"]
                == json!({"call":call_id,"position":position,"group":r["origin"]["group"],"source_ref":si,"owner_ref":oi})
                && r["origin"]["owner_ref"]["kind"] == "case"
                && *oi == json!({"kind":"case","index":ci})
                && c["source_ref"] == *si)?;
            let src = at(&b["sources"], si, "G5", "ATTEMPT_MISMATCH")?;
            af(src["owner"]["case_index"] == oi["index"]
                && src["owner"]["case_id"] == c["basis_ref"]["ref_id"])?;
            let records = list(&r["records"]);
            let attempts = list(&r["attempts"]);
            af(records.len() <= 4 && attempts.len() <= 3)?;
            af(records.is_empty() == attempts.is_empty())?;
            g5_schedule(r, Some(b))?;
            if let Some(first) = attempts.first() {
                af(first["precision"] == 128
                    && first["candidate_record"] == 0
                    && first["origin"]["kind"] == "fresh")?;
            }
            wf(u(&r["invocation_before"]) == current)?;
            let gid = uint(&r["origin"]["group"]);
            let mut cache = if let Some(gid) = gid {
                let g = at(&b["groups"], &json!(gid), "G5", "ATTEMPT_MISMATCH")?;
                af(u(&g["call"]) == call_id as u64 && list(&g["source_refs"]).contains(si))?;
                if g["preparation"]["kind"] == "refused" {
                    af(records.is_empty()
                        && r["kernel_terminal"]["kind"] == "refused"
                        && r["kernel_terminal"]["reason"] == g["preparation"]["reason"])?;
                }
                group_caches
                    .get(&gid)
                    .cloned()
                    .ok_or_else(|| error("G5", "ATTEMPT_MISMATCH"))?
            } else {
                // D27: a class-1 ATTEMPT check reads the recorded
                // invocation_before, never the WORK-derived running meter.
                af(records.is_empty()
                    && u(&r["invocation_before"]) >= u(&b["work"]["invocation_limit"])
                    && r["kernel_terminal"]["reason"]["tag"] == "budget"
                    && r["kernel_terminal"]["reason"]["scope"] == "invocation")?;
                BTreeMap::new()
            };
            let snapshot = |cache: &BTreeMap<String, u64>| -> Value {
                json!(SLOTS
                    .iter()
                    .filter_map(|slot| cache.get(*slot).map(|bi| json!({"slot":slot,"build":bi})))
                    .collect::<Vec<_>>())
            };
            wf(r["cache_before"] == snapshot(&cache))?;
            let mut amounts = Vec::new();
            let mut prior_precision = 0;
            for (index, record) in records.iter().enumerate() {
                let p = u(&record["precision"]);
                af(u(&record["index"]) == index as u64
                    && p > prior_precision
                    && matches!(p, 128 | 256 | 512 | 1024)
                    && u(&record["residual_basis"]) == if p == 1024 { 1024 } else { p + 64 })?;
                prior_precision = p;
                af(u(&record["corrections"]) <= 3)?;
                if record["role"] == "candidate" {
                    // D5a (C1:105; adaptive.rs `solve_precision`, `verify_precision`): a candidate record
                    // has no verification, no verification shared build and no
                    // verification-pass work.
                    af(record["verification"].is_null()
                        && record["verification_shared_build_ref"].is_null()
                        && u(&record["work"]["verification_lme"]) == 0
                        && matches!(
                            text(&record["outcome"]["kind"]),
                            "accepted" | "rejected" | "failed"
                        ))?;
                } else if record["role"] == "verification" {
                    af(matches!(
                        text(&record["outcome"]["kind"]),
                        "verified" | "solved" | "failed"
                    ))?;
                    // D5b (adaptive.rs `run_schedule_inner`, `terminal`): an escalating stop on a failed
                    // verification is a solve failure, so the verification pass never
                    // ran: no report and no verification-pass work.
                    // D21: a verification shared build is also pass evidence; natively
                    // it is obtained only inside verify_precision (adaptive.rs `verify_precision`).
                    if record["outcome"]["kind"] == "failed" && escalating(&record["outcome"]["reason"]) {
                        af(record["verification"].is_null()
                            && u(&record["work"]["verification_lme"]) == 0
                            && record["verification_shared_build_ref"].is_null())?;
                    }
                }
                // D5d/D28 (C2:22-24, :54; C1:114; adaptive.rs `rejection_reason`,
                // 4714-4720): every quantity-bearing reason (stop_rule,
                // verification_estimate, charge, publication_enclosure) names a
                // layout row of the Run's source with the same body and kind.
                let reason = &record["outcome"]["reason"];
                if reason["space"] == "attempt"
                    && matches!(
                        text(&reason["tag"]),
                        "stop_rule" | "verification_estimate" | "charge" | "publication_enclosure"
                    )
                {
                    let layout = list(&at(&b["sources"], si, "G5", "ATTEMPT_MISMATCH")?["layout"]);
                    af(layout.iter().any(|row| {
                        row["quantity"] == reason["quantity"]
                            && row["body"] == reason["body"]
                            && row["kind"] == reason["kind"]
                    }))?;
                    // D33 (RV80-N1; verify.rs `verify_state`): the native verification
                    // estimate exists only for Force and Moment rows. `charge`
                    // may name displacement rows and stays unrestricted.
                    af(reason["tag"] != "verification_estimate"
                        || matches!(text(&reason["kind"]), "force" | "moment"))?;
                }
                af(record["storage"]["limbs_per_entry"]
                    == json!(if p <= 256 {
                        4
                    } else if p == 512 {
                        8
                    } else {
                        16
                    }))?;
                let w = &record["work"];
                let own = tw(sum([u(&w["wide_lme"]), u(&w["exact_sum_lme"])]));
                wf(own == u(&w["own_lme"]) && own == tw(stages_sum(&w["own_stages"])))?;
                wf(u(&w["stop_rule_lme"]) == u(&w["own_stages"]["stop_rule"]))?;
                // verify_state owns these five disjoint stage slots; solve and
                // candidate comparison use the other slots (verify.rs `verify_state`).
                wf(u(&w["verification_lme"])
                    == tw(sum(["scale", "estimate", "charge", "bound", "shift"]
                        .iter()
                        .map(|key| u(&w["own_stages"][*key])))))?;
                wf(
                    tw(sum([u(&w["stop_rule_lme"]), u(&w["verification_lme"])])) <= own
                        && tw(stages_sum(&w["shared_stages"]))
                            == tw(sum([u(&w["shared_lme"]), u(&w["verification_shared_lme"])])),
                )?;
                let mut shared_stages: BTreeMap<&str, u64> = BTreeMap::new();
                for (field, phase, built, cost, slot) in [
                    (
                        "shared_build_ref",
                        "shared",
                        w["shared_built_here"] == true,
                        u(&w["shared_lme"]),
                        format!("s{p}"),
                    ),
                    (
                        "verification_shared_build_ref",
                        "verification_shared",
                        w["verification_shared_built_here"] == true,
                        u(&w["verification_shared_lme"]),
                        format!("v{p}"),
                    ),
                ] {
                    if record[field].is_null() {
                        wf(!built && cost == 0)?;
                        continue;
                    }
                    let bi = u(&record[field]);
                    // D16: a dangling build reference is WORK (C1 build
                    // provenance), deferred to the end of class 1 (D3).
                    let Ok(build) = at(&b["builds"], &record[field], "G5", "WORK_MISMATCH") else {
                        work_ok.set(false);
                        continue;
                    };
                    wf(build["group"] == r["origin"]["group"]
                        && build["slot"] == slot
                        && u(&build["work"]) == cost)?;
                    if built {
                        wf(!cache.contains_key(&slot)
                            && build["origin"]
                                == json!({"call":call_id,"run":ri,"physical_record":index,"phase":phase})
                            && builds_seen.insert(bi)
                            && bi == builds_seen.len() as u64 - 1)?;
                        if build["state"] != "budget_failure" {
                            cache.insert(slot, bi);
                        }
                    } else {
                        wf(cache.get(&slot) == Some(&bi) && build["state"] != "budget_failure")?;
                    }
                    // C1 build state/reason (WORK, as Python _g5_cache); every build
                    // is referenced by its building record (builds_seen check).
                    wf((build["state"] == "success") == build["reason"].is_null()
                        && (build["state"] == "budget_failure")
                            == (build["reason"]["tag"] == "budget"))?;
                    // C1: a failed build fails its requesting record with the same stop.
                    if build["state"] != "success" {
                        af(record["outcome"]["kind"] == "failed"
                            && stop_of(&record["outcome"]["reason"]) == Some(&build["reason"]))?;
                    }
                    for (k, v) in build["stages"]
                        .as_object()
                        .map(|o| o.iter().collect::<Vec<_>>())
                        .unwrap_or_else(|| {
                            work_ok.set(false);
                            Vec::new()
                        })
                    {
                        let old = shared_stages.get(k.as_str()).copied().unwrap_or(0);
                        shared_stages.insert(k, tw(sum([old, u(v)])));
                    }
                }
                for (k, v) in w["shared_stages"]
                    .as_object()
                    .map(|o| o.iter().collect::<Vec<_>>())
                    .unwrap_or_else(|| {
                        work_ok.set(false);
                        Vec::new()
                    })
                {
                    wf(u(v) == shared_stages.get(k.as_str()).copied().unwrap_or(0))?;
                }
                amounts.push((
                    tw(sum([own, u(&w["shared_lme"]), u(&w["verification_shared_lme"])])),
                    tw(sum([
                        own,
                        if w["shared_built_here"] == true {
                            u(&w["shared_lme"])
                        } else {
                            0
                        },
                        if w["verification_shared_built_here"] == true {
                            u(&w["verification_shared_lme"])
                        } else {
                            0
                        },
                    ])),
                ));
                if record["role"] == "verification" {
                    wf(u(&w["stop_rule_lme"]) == 0)?;
                }
            }
            wf(r["cache_after"] == snapshot(&cache))?;
            if let Some(gid) = gid {
                group_caches.insert(gid, cache);
            }
            let mut fragments = BTreeSet::new();
            let mut previous_candidate = None;
            for (ai, a) in attempts.iter().enumerate() {
                let ci = u(&a["candidate_record"]);
                let cr = records
                    .get(ci as usize)
                    .ok_or_else(|| error("G5", "ATTEMPT_MISMATCH"))?;
                af(a["precision"] == cr["precision"]
                    && a["outcome"] == cr["outcome"]
                    && matches!(u(&a["precision"]), 128 | 256 | 512)
                    && previous_candidate.is_none_or(|p| ci > p))?;
                previous_candidate = Some(ci);
                if a["origin"]["kind"] == "fresh" {
                    af(cr["role"] == "candidate")?;
                } else {
                    let pi = u(&a["origin"]["attempt"]);
                    af(pi + 1 == ai as u64 && cr["role"] == "verification_then_candidate")?;
                    let old = &attempts[pi as usize];
                    af(old["verification"]["record"] == a["candidate_record"]
                        && old["verification"]["phase"] == "completed"
                        && old["outcome"]["kind"] == "rejected")?;
                }
                let verification = if a["verification"].is_null() {
                    None
                } else {
                    let vi = u(&a["verification"]["record"]);
                    let vr = records
                        .get(vi as usize)
                        .ok_or_else(|| error("G5", "ATTEMPT_MISMATCH"))?;
                    af(vi == ci + 1
                        && u(&vr["precision"]) == 2 * u(&a["precision"])
                        && vr["precision"] == a["verification"]["precision"]
                        && matches!(
                            text(&vr["role"]),
                            "verification" | "verification_then_candidate"
                        ))?;
                    if vr["role"] == "verification_then_candidate" {
                        af(a["verification"]["phase"] == "completed"
                            && a["verification"]["reason"].is_null())?;
                    } else {
                        af((a["verification"]["phase"] == "failed")
                            == (vr["outcome"]["kind"] == "failed"))?;
                        if a["verification"]["phase"] == "failed" {
                            af(a["verification"]["reason"] == vr["outcome"]["reason"])?;
                        } else {
                            af(a["verification"]["reason"].is_null())?;
                        }
                    }
                    Some(vi)
                };
                if a["outcome"]["kind"] == "accepted" {
                    af(ai + 1 == attempts.len() && verification.is_some())?;
                }
                if verification.is_none() {
                    af(a["outcome"]["kind"] == "failed")?;
                }
                if ai + 1 < attempts.len() {
                    let next = u(&attempts[ai + 1]["precision"]);
                    let reason = &a["outcome"]["reason"];
                    if a["outcome"]["kind"] == "failed" {
                        af(verification.is_none()
                            && attempts[ai + 1]["origin"]["kind"] == "fresh"
                            && reason["tag"] == "stop"
                            && matches!(
                                text(&reason["stop"]["tag"]),
                                "pivot" | "condition" | "residual_gate"
                            )
                            && next == 2 * u(&a["precision"]))?;
                    } else if a["verification"]["phase"] == "failed" {
                        let stop = &a["verification"]["reason"]["stop"];
                        let vr = &records
                            [verification.ok_or_else(|| error("G5", "ATTEMPT_MISMATCH"))? as usize];
                        af(a["outcome"]["kind"] == "rejected"
                            && reason["tag"] == "verification_failed"
                            && attempts[ai + 1]["origin"]["kind"] == "fresh"
                            && vr["verification_shared_build_ref"].is_null()
                            && matches!(
                                text(&stop["tag"]),
                                "pivot" | "condition" | "residual_gate"
                            )
                            && next == 4 * u(&a["precision"]))?;
                    } else {
                        af(a["outcome"]["kind"] == "rejected"
                            && attempts[ai + 1]["origin"]
                                == json!({"kind":"reused_verification","attempt":ai})
                            && next == 2 * u(&a["precision"]))?;
                    }
                }
                let mut charges = Vec::new();
                let mut debits = Vec::new();
                for fr in list(&a["charges"]) {
                    let index = u(&fr["record"]);
                    let part = text(&fr["part"]);
                    af(fragments.insert((index, part)))?;
                    let rec = records
                        .get(index as usize)
                        .ok_or_else(|| error("G5", "ATTEMPT_MISMATCH"))?;
                    let stop = u(&rec["work"]["stop_rule_lme"]);
                    if part == "candidate_stop" {
                        af(index == ci)?;
                        charges.push(stop);
                        debits.push(stop);
                    } else {
                        af((index == ci && cr["role"] == "candidate")
                            || Some(index) == verification)?;
                        charges.push(
                            amounts[index as usize]
                                .0
                                .checked_sub(stop)
                                .unwrap_or_else(|| {
                                    work_ok.set(false);
                                    0
                                }),
                        );
                        debits.push(
                            amounts[index as usize]
                                .1
                                .checked_sub(stop)
                                .unwrap_or_else(|| {
                                    work_ok.set(false);
                                    0
                                }),
                        );
                    }
                }
                wf(tw(sum(charges)) == u(&a["case_charge"])
                    && tw(sum(debits)) == u(&a["invocation_increment"]))?;
            }
            let expected: BTreeSet<_> = records
                .iter()
                .enumerate()
                .flat_map(|(i, r)| {
                    if r["role"] == "verification" {
                        vec![(i as u64, "solve_and_verification")]
                    } else {
                        vec![
                            (i as u64, "solve_and_verification"),
                            (i as u64, "candidate_stop"),
                        ]
                    }
                })
                .collect();
            af(fragments == expected)?;
            let charge = tw(sum(amounts.iter().map(|p| p.0)));
            let debit = tw(sum(amounts.iter().map(|p| p.1)));
            wf(charge == u(&r["case_charge"])
                && charge == tw(sum(attempts.iter().map(|a| u(&a["case_charge"]))))
                && debit == u(&r["invocation_increment"])
                && debit == tw(sum(attempts.iter().map(|a| u(&a["invocation_increment"])))))?;
            // N10: a Run entered with the invocation meter exhausted is idle.
            if u(&r["invocation_before"]) >= u(&b["work"]["invocation_limit"]) {
                af(attempts.is_empty()
                    && r["origin"]["group"].is_null()
                    && r["kernel_terminal"]["reason"]
                        == json!({"space":"unresolved","tag":"budget","scope":"invocation"}))?;
            }
            current = tw(sum([current, debit]));
            wf(current == u(&r["invocation_after"]))?;
            if r["kernel_terminal"]["kind"] == "selected" {
                let a = attempts
                    .last()
                    .ok_or_else(|| error("G5", "ATTEMPT_MISMATCH"))?;
                let vr = at(
                    &r["records"],
                    &a["verification"]["record"],
                    "G5",
                    "ATTEMPT_MISMATCH",
                )?;
                let cr = at(
                    &r["records"],
                    &a["candidate_record"],
                    "G5",
                    "ATTEMPT_MISMATCH",
                )?;
                af(r["kernel_terminal"]["reason"].is_null()
                    && a["outcome"]["kind"] == "accepted"
                    && vr["outcome"]["kind"] == "verified"
                    && a["verification"]["phase"] == "completed"
                    && !vr["verification"].is_null())?;
                wf(charge <= u(&b["work"]["case_limit"])
                    && current <= u(&b["work"]["invocation_limit"]))?;
                if c["status"] == "selected" {
                    let s = &c["selection"];
                    af(s["precision"] == a["precision"]
                        && s["verification_precision"] == a["verification"]["precision"])?;
                    for k in ["pivot_margin_min", "rcond", "residual_worst", "corrections"] {
                        af(s[k] == cr[k])?;
                    }
                    af(s["resolution_scale"] == vr["verification"]["resolution"]
                        && s["theta"] == vr["verification"]["theta"]
                        && s["certified_bound"]
                            == json!(list(&vr["verification"]["bound"])
                                .iter()
                                .filter(|x| !x["value"].is_null())
                                .collect::<Vec<_>>()))?;
                }
            } else {
                af(!r["kernel_terminal"]["reason"].is_null()
                    && !attempts.iter().any(|a| a["outcome"]["kind"] == "accepted"))?;
                let reason = &r["kernel_terminal"]["reason"];
                if reason["tag"] == "budget" {
                    let case_over = charge > u(&b["work"]["case_limit"]);
                    let inv_over = current > u(&b["work"]["invocation_limit"]);
                    wf(if reason["scope"] == "case" {
                        case_over
                    } else {
                        !case_over
                            && (inv_over
                                || (records.is_empty()
                                    && u(&r["invocation_before"])
                                        >= u(&b["work"]["invocation_limit"])))
                    })?;
                }
            }
        }
        wf(u(&call["invocation_after"]) == current)?;
    }
    af(run_order.iter().copied().eq(0..runs.len() as u64))?;
    // C5/N11 (C2:143): call-local groups formed at first equality of the full
    // stiffness bytes, in first-seen order; an idle (group-null) Run forms none;
    // a refused group preparation returns before any attempt.
    for (call_id, call) in list(&b["calls"]).iter().enumerate() {
        let call_groups: Vec<&Value> = list(&b["groups"])
            .iter()
            .filter(|g| u(&g["call"]) == call_id as u64)
            .collect();
        let mut order: Vec<(Value, Vec<Value>)> = Vec::new();
        let mut of_run = Vec::new();
        for (ri, si) in list(&call["run_refs"]).iter().zip(list(&call["source_refs"])) {
            let (_, _, run) = runs
                .get(u(ri) as usize)
                .copied()
                .ok_or_else(|| error("G5", "ATTEMPT_MISMATCH"))?;
            if run["origin"]["group"].is_null() {
                continue;
            }
            let key = at(&b["sources"], si, "G5", "ATTEMPT_MISMATCH")?["stiffness_sha256"].clone();
            let gi = match order.iter().position(|(k, _)| *k == key) {
                Some(gi) => gi,
                None => {
                    order.push((key, Vec::new()));
                    order.len() - 1
                }
            };
            order[gi].1.push(si.clone());
            of_run.push((run, gi));
        }
        af(call_groups.len() == order.len()
            && call_groups
                .iter()
                .zip(&order)
                .all(|(g, (k, m))| g["stiffness_sha256"] == *k && list(&g["source_refs"]) == m.as_slice()))?;
        for (run, gi) in of_run {
            let g = call_groups[gi];
            af(run["origin"]["group"] == g["id"])?;
            if g["preparation"]["kind"] == "refused" {
                af(list(&run["attempts"]).is_empty()
                    && run["kernel_terminal"]
                        == json!({"kind":"refused","reason":g["preparation"]["reason"]}))?;
            }
        }
    }
    wf(u(&b["work"]["charged"]) == current && builds_seen.len() == list(&b["builds"]).len())?;
    need(work_ok.get(), "G5", "WORK_MISMATCH")
}
/// A G7 base failure: the leading `[A-Z][A-Z0-9_]*` token is the bare code;
/// the full base text, when it says more, is carried as detail.
fn base_error(text: String) -> ValidationError {
    let end = text
        .char_indices()
        .find(|(i, c)| !(c.is_ascii_uppercase() || (*i > 0 && (c.is_ascii_digit() || *c == '_'))))
        .map_or(text.len(), |(i, _)| i);
    let code = text[..end].to_string();
    ValidationError {
        gate: "G7",
        detail: (code != text).then_some(text),
        code,
    }
}
/// The native Stop inside a Reason: a bare stop, or an attempt-space stop.
fn stop_of(reason: &Value) -> Option<&Value> {
    if reason["space"] == "stop" {
        Some(reason)
    } else if reason["space"] == "attempt" && reason["tag"] == "stop" {
        Some(&reason["stop"])
    } else {
        None
    }
}
fn escalating(reason: &Value) -> bool {
    stop_of(reason).is_some_and(|s| matches!(text(&s["tag"]), "pivot" | "condition" | "residual_gate"))
}
/// Checklist N1-N10 schedule replay of the native ladder (FK/adaptive.rs:
/// 4519-4766): candidates at the 128/256/512 slots; an escalating candidate
/// failure advances one slot, an escalating failed verification solve two, a
/// non-escalating or verification-pass failure ends the ladder, a rejected
/// candidate hands its completed verification on while a slot remains, and
/// leaving the loop is the Ceiling. A pre-schedule run has no records, no
/// charge and a non-selected terminal with a reason.
fn g5_schedule(r: &Value, body: Option<&Value>) -> VResult {
    let af = |ok| need(ok, "G5", "ATTEMPT_MISMATCH");
    let records = list(&r["records"]);
    let attempts = list(&r["attempts"]);
    af(records.iter().all(|x| u(&x["corrections"]) <= 3))?;
    let terminal = &r["kernel_terminal"];
    // A WorkAccounting terminal exists natively only when a work status is not
    // exact (adaptive.rs `run_schedule_inner`, `finish_terminal`, `solve_cases_projected`); C1:66-68 forbids emitting such a
    // run, so it lies outside the emitted terminal domain (C1:148), idle or not.
    af(!(terminal["kind"] == "unresolved"
        && terminal["reason"]["space"] == "unresolved"
        && terminal["reason"]["tag"] == "work_accounting"))?;
    if attempts.is_empty() {
        // Pre-schedule returns (adaptive.rs `solve_cases_projected`): invocation exhaustion
        // (group null, Budget(invocation)); a refused group (checked with C5);
        // a CasePrep failure in a ready group (refused LedgerUnavailable).
        af(records.is_empty()
            && u(&r["case_charge"]) == 0
            && u(&r["invocation_increment"]) == 0
            && terminal["kind"] != "selected"
            && !terminal["reason"].is_null())?;
        if let Some(b) = body {
            if r["origin"]["group"].is_null() {
                af(u(&r["invocation_before"]) >= u(&b["work"]["invocation_limit"])
                    && *terminal
                        == json!({"kind":"unresolved","reason":{"space":"unresolved","tag":"budget","scope":"invocation"}}))?;
            } else {
                let g = at(&b["groups"], &r["origin"]["group"], "G5", "ATTEMPT_MISMATCH")?;
                if g["preparation"]["kind"] == "ready" {
                    af(terminal["kind"] == "refused" && terminal["reason"]["tag"] == "ledger_unavailable")?;
                }
            }
        }
        return Ok(());
    }
    let mut c = 0u32;
    let mut ended = false;
    let mut escalated = false;
    let mut end_stop = Value::Null;
    let mut next_record = 0u64;
    for (ai, a) in attempts.iter().enumerate() {
        af(!ended && c < 3 && u(&a["precision"]) == 128u64 << c)?;
        let last = ai + 1 == attempts.len();
        let v = &a["verification"];
        // Physical record replay: a fresh candidate opens the next record; a
        // reused one is the prior rejected attempt's completed verification.
        let cr_i = u(&a["candidate_record"]);
        let cr = records
            .get(cr_i as usize)
            .ok_or_else(|| error("G5", "ATTEMPT_MISMATCH"))?;
        if a["origin"]["kind"] == "fresh" {
            af(cr_i == next_record && cr["role"] == "candidate")?;
        } else {
            af(ai > 0 && {
                let p = &attempts[ai - 1];
                u(&a["origin"]["attempt"]) == ai as u64 - 1
                    && p["outcome"]["kind"] == "rejected"
                    && !p["verification"].is_null()
                    && p["verification"]["phase"] == "completed"
                    && u(&p["verification"]["record"]) == cr_i
                    && cr_i + 1 == next_record
                    && cr["role"] == "verification_then_candidate"
            })?;
        }
        next_record = next_record.max(cr_i.saturating_add(1));
        let kind = text(&a["outcome"]["kind"]);
        af(matches!(kind, "accepted" | "rejected" | "failed"))?;
        // D5c: rejected(verification_failed) requires the failed verification phase.
        if a["outcome"]["reason"] == json!({"space":"attempt","tag":"verification_failed"}) {
            af(!v.is_null() && v["phase"] == "failed")?;
        }
        if !v.is_null() {
            let vi = u(&v["record"]);
            af(vi == cr_i.saturating_add(1) && vi == next_record)?;
            let vr = records
                .get(vi as usize)
                .ok_or_else(|| error("G5", "ATTEMPT_MISMATCH"))?;
            next_record = vi + 1;
            if v["phase"] == "failed" {
                af(vr["role"] == "verification"
                    && vr["outcome"]["kind"] == "failed"
                    && v["reason"] == vr["outcome"]["reason"]
                    && kind == "rejected"
                    && a["outcome"]["reason"] == json!({"space":"attempt","tag":"verification_failed"}))?;
            } else {
                af(v["phase"] == "completed" && v["reason"].is_null())?;
                if vr["role"] == "verification" {
                    af(vr["outcome"]["kind"] == if kind == "accepted" { "verified" } else { "solved" })?;
                } else {
                    af(vr["role"] == "verification_then_candidate" && kind == "rejected")?;
                }
            }
        } else {
            af(kind == "failed" && stop_of(&a["outcome"]["reason"]).is_some())?;
        }
        escalated = false;
        match kind {
            "accepted" => {
                af(!v.is_null() && v["phase"] == "completed" && last)?;
                ended = true
            }
            _ if v.is_null() => {
                if escalating(&a["outcome"]["reason"]) {
                    c += 1;
                    escalated = true;
                } else {
                    ended = true;
                    end_stop = stop_of(&a["outcome"]["reason"]).cloned().unwrap_or(Value::Null);
                }
            }
            _ if v["phase"] == "failed" => {
                // A failed verification solve with an escalating stop skips two
                // slots; a verification-pass failure is terminal (4576-4611).
                if escalating(&v["reason"]) {
                    c += 2;
                    escalated = true;
                } else {
                    ended = true;
                    end_stop = stop_of(&v["reason"]).cloned().unwrap_or(Value::Null);
                }
            }
            "rejected" => {
                c += 1;
                if c < 3 {
                    af(!last && attempts[ai + 1]["origin"]["kind"] == "reused_verification")?;
                }
            }
            _ => {
                ended = true;
                end_stop = stop_of(&a["outcome"]["reason"]).cloned().unwrap_or(Value::Null);
            }
        }
    }
    af(next_record == records.len() as u64)?;
    let ceiling = json!({"kind":"unresolved","reason":{"space":"unresolved","tag":"ceiling"}});
    if attempts[attempts.len() - 1]["outcome"]["kind"] == "accepted" {
        af(terminal["kind"] == "selected" && terminal["reason"].is_null())
    } else if ended {
        // N5: a terminal stop ends on its exact terminal() translation
        // (adaptive.rs `terminal`).
        af(terminal_of(&end_stop).is_some_and(|t| *terminal == t))
    } else if escalated && c < 3 {
        // N9: an escalating last stop with slots left ends only through a work
        // fault (adaptive.rs `run_schedule_inner`), which is never emitted.
        af(false)
    } else {
        // N8: leaving the loop past the last slot is the Ceiling (4762-4766).
        af(*terminal == ceiling)
    }
}
/// adaptive.rs `terminal` terminal(): the exact kernel terminal of a
/// terminal stop, or None for an escalating stop. The wire carries
/// WorkAccounting as {fault} only (C3:261-263).
fn terminal_of(stop: &Value) -> Option<Value> {
    let tag = text(&stop["tag"]);
    let unresolved = |name: &str, keys: &[&str]| {
        let mut reason = json!({"space":"unresolved","tag":name});
        for k in keys {
            reason[*k] = stop[*k].clone();
        }
        Some(json!({"kind":"unresolved","reason":reason}))
    };
    match tag {
        "negative_energy" => Some(
            json!({"kind":"refused","reason":{"space":"refusal","tag":"negative_energy","i":stop["i"],"j":stop["j"]}}),
        ),
        "structure" => Some(json!({"kind":"refused","reason":{"space":"refusal","tag":"structure"}})),
        "count_range" => unresolved("count_range", &["name"]),
        "work_accounting" => unresolved("work_accounting", &["fault"]),
        "budget" => unresolved("budget", &["scope"]),
        "span" => unresolved("exact_sum_span", &[]),
        "exponent" => unresolved("exponent_range", &[]),
        "zero_diagonal" => unresolved("zero_diagonal", &["global_dof"]),
        "arithmetic" => unresolved("arithmetic", &["error"]),
        "resolution_scale" => unresolved("resolution_scale_unencodable", &["body", "kind"]),
        "publication_certificate" => unresolved("publication_certificate", &["index", "issue"]),
        _ => None,
    }
}
/// Internal, test-only (D14): reader-logic entry points for tests of rules
/// that have no native-faithful shared base yet. They run the same functions
/// `validate` uses, on a partial input, return no Validation and grant nothing.
/// Not a public API.
#[doc(hidden)]
pub mod reader_logic {
    use super::*;
    /// The G5 schedule replay for one Run (checklist N1-N10).
    pub fn schedule(run: &Value) -> Result<(), ValidationError> {
        g5_schedule(run, None)
    }
    /// The G5 ordinary/source_decline pass for a whole statement (O2-O5).
    pub fn ordinary(source: &Value) -> Result<(), ValidationError> {
        g5_ordinary(source)
    }
    /// The schedule replay with the statement body (N10 idle rules).
    pub fn schedule_in(run: &Value, body: &Value) -> Result<(), ValidationError> {
        g5_schedule(run, Some(body))
    }
    /// R1'-R4 for one product attempt.
    pub fn accounting(attempt: &Value) -> [bool; 4] {
        accounting_rules(attempt)
    }
    /// The D4d reason table for one unavailable case and its attempt.
    pub fn reason_table(case: &Value, attempt: &Value) -> Result<(), ValidationError> {
        super::reason_table(case, attempt)
    }
    /// The G7 bare-code/detail split of a base failure text.
    pub fn g7_error(text: &str) -> ValidationError {
        base_error(text.to_string())
    }
}
/// Every JSON object inside `v`, `v` first (depth-first, document order).
fn objects<'a>(v: &'a Value, out: &mut Vec<&'a serde_json::Map<String, Value>>) {
    match v {
        Value::Object(o) => {
            out.push(o);
            for x in o.values() {
                objects(x, out);
            }
        }
        Value::Array(a) => {
            for x in a {
                objects(x, out);
            }
        }
        _ => {}
    }
}
/// A work status as its fault set: overflow = 1, inconsistent = 2, both = 3.
fn status_faults(v: &Value) -> Option<u8> {
    match v.as_str()? {
        "exact" => Some(0),
        "overflow" => Some(1),
        "inconsistent" => Some(2),
        "both" => Some(3),
        _ => None,
    }
}
/// Every JSON object inside `v` with its path (object keys and array indices
/// as text), `v` first, in document order.
fn located<'a>(
    v: &'a Value,
    path: &mut Vec<String>,
    out: &mut Vec<(Vec<String>, &'a serde_json::Map<String, Value>)>,
) {
    match v {
        Value::Object(o) => {
            out.push((path.clone(), o));
            for (k, x) in o {
                path.push(k.clone());
                located(x, path, out);
                path.pop();
            }
        }
        Value::Array(a) => {
            for (i, x) in a.iter().enumerate() {
                path.push(i.to_string());
                located(x, path, out);
                path.pop();
            }
        }
        _ => {}
    }
}
/// The emitted fault set of a work owner: unavailable-Count faults and sticky
/// statuses anywhere inside it ("both" is overflow plus inconsistent).
fn statuses(v: &Value) -> u8 {
    let mut objs = Vec::new();
    objects(v, &mut objs);
    let mut seen = 0u8;
    for o in &objs {
        if o.get("kind").is_some_and(|k| k == "unavailable") {
            if let Some(f) = o.get("fault").and_then(status_faults) {
                seen |= f;
            }
        }
        if let Some(f) = o.get("sticky_status").and_then(status_faults) {
            seen |= f;
        }
    }
    seen
}
/// R3' owner scopes (checkpoint A, D8): a member's PreparationWork, a lane's
/// LaneWork, the values completion, else the whole ProofTrace.
fn fault_owner<'a>(a: &'a Value, path: &[String]) -> Option<&'a Value> {
    let proof = &a["proof"];
    let p = |i: usize| path.get(i).map(String::as_str);
    let index = |i: usize| path.get(i).and_then(|x| x.parse::<usize>().ok());
    let owner = if (p(0), p(1)) == (Some("preparation"), Some("members")) {
        index(2).map(|i| &a["preparation"]["members"][i]["work"])
    } else if (p(0), p(1), p(2)) == (Some("result"), Some("error"), Some("section")) {
        list(&a["preparation"]["members"]).last().map(|m| &m["work"])
    } else if (p(0), p(1)) == (Some("proof"), Some("lanes")) {
        index(2).map(|i| &proof["lanes"][i]["work"])
    } else if (p(0), p(1), p(2)) == (Some("result"), Some("error"), Some("cause"))
        && a["result"]["error"]["kind"] == "values"
    {
        Some(&proof["completion"])
    } else {
        Some(proof)
    };
    owner.filter(|o| !o.is_null())
}
/// The OperationalError causes of an attempt, located as Python does: a
/// non-ready MemberOperational's error, a CaptureError `prepared_arithmetic`
/// cause, and a G5aError `operational`/`arithmetic` cause (result or check).
fn operational_errors(a: &Value) -> Vec<&Value> {
    let mut out = Vec::new();
    for side in ["old", "new"] {
        for m in list(&a["operational"][side]) {
            if m["result"]["kind"] != "ready" {
                out.push(&m["result"]["error"]);
            }
        }
    }
    let null = &Value::Null;
    let error = if a["result"]["kind"] == "unavailable" {
        &a["result"]["error"]
    } else {
        null
    };
    let mut objs = Vec::new();
    located(error, &mut Vec::new(), &mut objs);
    let mut g5a_roots: Vec<&Value> = Vec::new();
    for (_, o) in &objs {
        let cause = o.get("cause").unwrap_or(null);
        if o.get("kind").is_some_and(|k| k == "prepared_arithmetic") {
            out.push(cause);
        }
        if o.get("kind").is_some_and(|k| k == "g5a") {
            g5a_roots.push(cause);
        }
    }
    for check in a["proof"]["checks"].as_object().into_iter().flat_map(|c| c.values()) {
        if check["kind"] == "failed" && check["error"]["kind"] == "g5a" {
            g5a_roots.push(&check["error"]["cause"]);
        }
    }
    for root in g5a_roots {
        let mut inner = Vec::new();
        located(root, &mut Vec::new(), &mut inner);
        for (_, x) in inner {
            if x.get("kind").is_some_and(|k| k == "operational" || k == "arithmetic") {
                if let Some(cause) = x.get("cause").filter(|c| c.is_object()) {
                    out.push(cause);
                }
            }
        }
    }
    out
}
/// Checkpoint A, D8 (C3:232-236; G5 WORK, class 4), for one product attempt:
/// R1': no adapter fault and no CaptureError/G5aError `accounting{event}`
/// (an adapter overflow leaves a count of at least 2^62, retained_product.rs `AdapterWork::enter`).
/// R2': no ScalarTrace is `lost`, and no OperationalError `accounting`
/// (returned only for a lost ScalarWork, retained_product.rs `ScalarWork::check` through `ScalarWork::operation`).
/// R3': every fault-bearing cause, in any spelling (`work_accounting{fault}`,
/// a nested `stop/work_accounting`, a view `work{fault}`), has its fault in
/// its owner's emitted statuses (`fault_owner`).
/// R4: a SectionError `accounting` (on a PreparedMember, or as the preparation
/// error's section, owned by the last member) needs a non-exact status in that
/// member's PreparationWork (FK product_certificate.rs `SectionPreparationWork::check`).
fn accounting_rules(a: &Value) -> [bool; 4] {
    let mut objs = Vec::new();
    located(a, &mut Vec::new(), &mut objs);
    let r1 = a["adapter"]["fault"].is_null()
        && !objs
            .iter()
            .any(|(_, o)| o.get("kind").is_some_and(|k| k == "accounting") && o.contains_key("event"));
    let r2 = !objs.iter().any(|(_, o)| o.get("lost") == Some(&Value::Bool(true)))
        && !operational_errors(a)
            .iter()
            .any(|e| e["kind"] == "accounting" && e.get("event").is_none());
    let r3 = objs.iter().all(|(path, o)| {
        let spelled = (o.get("kind").is_some_and(|k| k == "work_accounting" || k == "work")
            || o.get("tag").is_some_and(|t| t == "work_accounting"))
            && o.contains_key("fault");
        !spelled
            || fault_owner(a, path).is_some_and(|owner| {
                o.get("fault")
                    .and_then(status_faults)
                    .is_some_and(|f| f & !statuses(owner) == 0)
            })
    });
    let mut r4 = true;
    for m in list(&a["preparation"]["members"]) {
        if m["result"]["kind"] != "prepared" && m["result"]["error"]["kind"] == "accounting" {
            r4 &= statuses(&m["work"]) != 0;
        }
    }
    let error = &a["result"]["error"];
    if a["result"]["kind"] == "unavailable"
        && error["kind"] == "preparation"
        && error["section"]["kind"] == "accounting"
    {
        r4 &= list(&a["preparation"]["members"])
            .last()
            .is_some_and(|m| statuses(&m["work"]) != 0);
    }
    [r1, r2, r3, r4]
}
fn exact_count(v: &Value) -> Option<u64> {
    (v["kind"] == "exact").then(|| u(&v["value"]))
}
fn exact_work(v: &Value) -> bool {
    match v {
        Value::Object(o) => {
            !(v["kind"] == "unavailable" && o.contains_key("fault"))
                && (!o.contains_key("sticky_status") || v["sticky_status"] == "exact")
                && v["lost"] != true
                && o.values().all(exact_work)
        }
        Value::Array(a) => a.iter().all(exact_work),
        _ => true,
    }
}
/// I57 s3/s4 G5: proof-owned summary coverage binding and stage implications,
/// checked in the C3 association pass after the existing schedule.
///
/// Null means no complete vector was retained by this proof; it is never zero
/// coverage. Ready, a completed certificate or a passed G5a requires the
/// complete vector; a failed certificate may carry null. A complete vector
/// requires the attempt's own source and Run (same origin source/owner), a
/// selected native Run, both lanes completed in their declared order, completed
/// proof_start/projection/maxima/values/aliases and an entered certificate.
/// Complete coverage never implies certificate success.
fn g5_coverage(a: &Value, c: &Value, s: Option<&Value>) -> VResult {
    let pf = |ok| need(ok, "G5", "PRODUCT_ATTEMPT_MISMATCH");
    let proof = &a["proof"];
    if proof.is_null() {
        return Ok(());
    }
    let coverage = &proof["summary_coverage"];
    let st = &a["stages"];
    let checks = &proof["checks"];
    if a["result"]["kind"] == "ready"
        || st["certificate"] == "completed"
        || checks["certificate"]["kind"] == "passed"
        || st["g5a"] == "completed"
        || checks["g5a"]["kind"] == "passed"
    {
        pf(!coverage.is_null())?;
    }
    if coverage.is_null() {
        return Ok(());
    }
    let run = &c["run"];
    pf(s.is_some()
        && !a["run_ref"].is_null()
        && !run.is_null()
        && run["id"] == a["run_ref"]
        && c["source_ref"] == a["source_ref"]
        && run["origin"]["source_ref"] == a["source_ref"]
        && run["origin"]["owner_ref"] == a["owner_ref"]
        && run["kernel_terminal"]["kind"] == "selected")?;
    let lanes = list(&proof["lanes"]);
    pf(lanes.len() == 2
        && lanes
            .iter()
            .zip(["admitted_k", "annular_source"])
            .all(|(l, law)| l["law"] == law && l["state"] == "completed"))?;
    pf(["proof_start", "projection", "maxima", "values", "aliases"]
        .iter()
        .all(|k| st[*k] == "completed"))?;
    pf(matches!(text(&st["certificate"]), "completed" | "failed")
        && matches!(text(&checks["certificate"]["kind"]), "passed" | "failed"))
}
/// D4d (S06 section 1): the exhaustive reason table for an unavailable case
/// whose prepared_product_failure cause names this attempt, applied by the
/// attempt's own error (D19).
fn reason_table(c: &Value, a: &Value) -> VResult {
    let pf = |ok| need(ok, "G5", "PRODUCT_ATTEMPT_MISMATCH");
    let st = &a["stages"];
    let proof = &a["proof"];
    let e = &a["result"]["error"];
    let run = &c["run"];
    let expected = match text(&e["kind"]) {
        "preparation" => {
            pf(run.is_null() && st["preparation"] == "failed")?;
            ("source_unavailable", "preparation")
        }
        // D4d (S06 section 1): native requires the case's own nonselected
        // Run, and its run_ref is that Run; native with a selected Run is
        // invalid.
        "native" => {
            pf(!run.is_null()
                && run["kernel_terminal"]["kind"] != "selected"
                && e["run_ref"] == run["id"])?;
            if run["kernel_terminal"]["kind"] == "unresolved" {
                ("kernel_unresolved", "kernel")
            } else {
                pf(run["kernel_terminal"]["kind"] == "refused")?;
                ("kernel_refused", "kernel")
            }
        }
        "capture" if run.is_null() => ("source_unavailable", "preparation"),
        "capture" if run["kernel_terminal"]["kind"] != "selected" => {
            if run["kernel_terminal"]["kind"] == "unresolved" {
                ("kernel_unresolved", "kernel")
            } else {
                pf(run["kernel_terminal"]["kind"] == "refused")?;
                ("kernel_refused", "kernel")
            }
        }
        _ => {
            pf(run["kernel_terminal"]["kind"] == "selected")?;
            ("facade_certificate", "facade")
        }
    };
    pf(c["reason"]["code"] == expected.0 && c["reason"]["phase"] == expected.1)?;
    if matches!(text(&e["kind"]), "observable" | "g5a") {
        let key = if e["kind"] == "observable" {
            "observables"
        } else {
            "g5a"
        };
        pf(proof["checks"][key]["kind"] == "failed" && proof["checks"][key]["error"] == *e)?;
    }
    Ok(())
}
/// R-D38 (4b) (DESIGN_v2 §2; RR:8823; C1:103, C3:167), for an attempt whose
/// native stage failed with no Run: a native capture failure before any Run,
/// beside a registered prepared source (C2 §3 registers a CaseSource once it
/// is constructed and its maps validate, whatever the outcome). G5
/// PRODUCT_ATTEMPT. The conjuncts here: an unavailable `capture` result; the
/// stage record done(1, [failed]), preparation completed and every stage after
/// native not_entered; the case unavailable with `prepared_product_failure`
/// naming this attempt and reason (source_unavailable, preparation), D4d's
/// mapping for a capture with no Run; and a non-null source reference equal to
/// the case's own (TS already requires it; [r01: N-6]).
/// The rest of (4b) holds at the call site: `run_ref` is null (the branch),
/// so the case's Run is null and `proof` is null (both refused above
/// otherwise), and a non-null source reference resolves to a CaseSource whose
/// preparation binds this attempt (checked above for every attempt). That no
/// Run, Call or Group `source_refs` entry, Build or `execution_order` entry
/// names this case or its source holds for every receipt: G3 binds
/// `execution_order` to the cases' Runs, and G5's native class binds every
/// Call position to its Run's own case and source, every Group source to its
/// Call, and every Build to its building record.
fn d38_capture_before_run(c: &Value, a: &Value, ai: usize) -> bool {
    let st = &a["stages"];
    a["result"]["kind"] == "unavailable"
        && a["result"]["error"]["kind"] == "capture"
        && st["preparation"] == "completed"
        && STAGE8[2..]
            .iter()
            .chain(&["observables", "g5a"])
            .all(|k| st[*k] == "not_entered")
        && c["status"] == "unavailable"
        && c["reason"]["cause"]["kind"] == "prepared_product_failure"
        && u(&c["reason"]["cause"]["product_attempt_ref"]) == ai as u64
        && c["reason"]["code"] == "source_unavailable"
        && c["reason"]["phase"] == "preparation"
        && !a["source_ref"].is_null()
        && a["source_ref"] == c["source_ref"]
}
fn g5_products(source: &Value) -> VResult {
    let b = &source["retained_precision"]["body"];
    let pf = |ok| need(ok, "G5", "PRODUCT_ATTEMPT_MISMATCH");
    // C3 association/stage/check failures precede C3 accounting failures across
    // the ordered attempt list. Native work was already checked above.
    let mut pending_work = Vec::new();
    let mut wf = |ok| -> VResult {
        pending_work.push(ok);
        Ok(())
    };
    // D4c (S06 section 1; C3:165): a case claiming a prepared_product_failure
    // cause has its own product attempt, the one the cause names.
    // D20 (C3:165; D17 order): a selected case has a C3 product attempt.
    for c in list(&b["cases"]) {
        if c["reason"]["cause"]["kind"] == "prepared_product_failure" {
            pf(!c["product_attempt_ref"].is_null()
                && c["product_attempt_ref"] == c["reason"]["cause"]["product_attempt_ref"])?;
        }
        if c["status"] == "selected" {
            pf(!c["product_attempt_ref"].is_null())?;
        }
    }
    for (ai, a) in list(&b["product_attempts"]).iter().enumerate() {
        let c = at(
            &b["cases"],
            &a["owner_ref"]["index"],
            "G5",
            "PRODUCT_ATTEMPT_MISMATCH",
        )?;
        // D1 (F1:97, 130-131; C3:304): a captured prefix has no source and no
        // Run, and its result is unavailable (C3 references, G5).
        if a["operational"]["old_coverage"] == "captured_prefix" {
            pf(a["source_ref"].is_null()
                && a["run_ref"].is_null()
                && a["result"]["kind"] == "unavailable")?;
        }
        let ordinary = at(
            &b["ordinary_attempts"],
            &a["ordinary_attempt_ref"],
            "G5",
            "PRODUCT_ATTEMPT_MISMATCH",
        )?;
        pf(c["product_attempt_ref"] == a["id"]
            && a["ordinary_attempt_ref"] == c["ordinary"]["attempt_ref"]
            && a["material_basis_ref"] == ordinary["material_basis_ref"])?;
        if a["run_ref"].is_null() {
            pf(c["run"].is_null())?;
        } else {
            pf(c["run"]["id"] == a["run_ref"] && a["source_ref"] == c["source_ref"])?;
        }
        let s = if a["source_ref"].is_null() {
            None
        } else {
            let s = at(
                &b["sources"],
                &a["source_ref"],
                "G5",
                "PRODUCT_ATTEMPT_MISMATCH",
            )?;
            pf(s["owner"]["case_index"] == a["owner_ref"]["index"]
                && s["material_basis_ref"] == a["material_basis_ref"]
                && s["preparation"]["attempt_ref"] == json!(ai))?;
            Some(s)
        };
        let pm = list(&a["preparation"]["members"]);
        let old = list(&a["operational"]["old"]);
        let new = list(&a["operational"]["new"]);
        let st = &a["stages"];
        let props = [
            ("area", "lo"),
            ("area", "hi"),
            ("second_moment", "lo"),
            ("second_moment", "hi"),
            ("polar_moment", "lo"),
            ("polar_moment", "hi"),
            ("section_modulus", "lo"),
            ("section_modulus", "hi"),
            ("radius", "exact"),
        ];
        for (j, m) in pm.iter().enumerate() {
            let prepared = m["result"]["kind"] == "prepared";
            if !prepared {
                pf(j + 1 == pm.len() && j >= new.len())?;
            }
            let cv = list(&m["conversions"]);
            pf(cv.len() <= 9)?;
            for (k, v) in cv.iter().enumerate() {
                pf(v["property"] == props[k].0
                    && v["endpoint"] == props[k].1
                    && conversion_kind_ok(&v["outcome"]))?;
                if prepared {
                    let idx = if k < 8 { k / 2 } else { 4 };
                    pf(v["outcome"]["kind"] == "normal"
                        && v["outcome"]["value"] == m["result"]["section"][idx]
                        && f(&v["outcome"]["value"]) >= f64::MIN_POSITIVE)?;
                }
            }
            if prepared {
                pf(cv.len() == 9)?;
            }
            if let Some(n) = exact_count(&m["work"]["conversions"]) {
                wf(n == cv.len() as u64)?;
            }
        }
        for (j, op) in new.iter().enumerate() {
            pf(pm[j]["result"]["kind"] == "prepared" && op["member"] == pm[j]["member"])?;
        }
        let proof = &a["proof"];
        if proof.is_null() {
            pf(st["proof_start"] == "not_entered"
                && [
                    "projection",
                    "maxima",
                    "values",
                    "aliases",
                    "certificate",
                    "observables",
                    "g5a",
                ]
                .iter()
                .all(|k| st[k] == "not_entered"))?;
        } else {
            pf(!a["run_ref"].is_null()
                && c["run"]["kernel_terminal"]["kind"] == "selected"
                && st["proof_start"] != "not_entered")?;
            let lanes = list(&proof["lanes"]);
            pf(lanes.len() <= 2)?;
            for (i, lane) in lanes.iter().enumerate() {
                pf(lane["law"] == ["admitted_k", "annular_source"][i]
                    && (lane["state"] == "completed") == lane["error"].is_null())?;
                if lane["state"] == "failed" {
                    pf(i + 1 == lanes.len())?;
                }
                if let Some(n) = exact_count(&lane["work"]["correction"]["calls"]) {
                    wf(n <= 1)?;
                }
                wf(lane["work"]["data_capacity"] == lane["work"]["view"]["data_capacity"])?;
            }
            if st["proof_start"] == "completed" {
                pf(lanes.len() == 2 && lanes.iter().all(|l| l["state"] == "completed"))?;
            }
            if st["projection"] != "not_entered" {
                pf(lanes.len() == 2
                    && lanes.iter().all(|l| l["state"] == "completed")
                    && st["proof_start"] == "completed")?;
            }
            for (k, prior) in [
                ("maxima", "projection"),
                ("values", "maxima"),
                ("aliases", "values"),
                ("certificate", "aliases"),
            ] {
                if st[k] != "not_entered" {
                    pf(st[prior] == "completed")?;
                }
            }
            for k in ["certificate", "observables", "g5a"] {
                let check = &proof["checks"][k];
                pf(match text(&st[k]) {
                    "not_entered" => check["kind"] == "not_entered",
                    "completed" => check["kind"] == "passed",
                    "failed" => check["kind"] == "failed",
                    _ => false,
                })?;
                if check["kind"] == "failed" {
                    pf(check["error"]["kind"]
                        == if k == "certificate" {
                            "proof"
                        } else if k == "observables" {
                            "observable"
                        } else {
                            "g5a"
                        })?;
                }
            }
            let outcomes = list(&proof["projection_outcomes"]);
            if let Some(n) = exact_count(&proof["projection_conversions"]) {
                wf(n == outcomes.len() as u64)?;
            }
            if st["projection"] == "not_entered" {
                pf(outcomes.is_empty())?;
            }
            if proof["completion"]["kind"] == "merged" {
                // A failed maxima calculation abandons its already created
                // builder and merges its work before complete_maxima is entered.
                pf(st["projection"] == "completed")?;
            }
            if proof["completion"]["kind"] == "separate_failure" {
                pf(a["result"]["kind"] == "unavailable"
                    && a["result"]["error"]["kind"] == "values"
                    && st["values"] == "failed")?;
            }
            if st["aliases"] != "not_entered" || st["certificate"] != "not_entered" {
                pf(proof["completion"]["kind"] == "merged")?;
            }
            if st["observables"] != "not_entered" || st["g5a"] != "not_entered" {
                pf(st["certificate"] != "not_entered")?;
            }
            let rows = rows_for(source, c);
            let projected: Vec<_> = rows
                .iter()
                .enumerate()
                .filter(|(_, r)| hull_projected(r))
                .map(|(i, _)| i as u64)
                .collect();
            pf(outcomes.len() <= projected.len()
                && outcomes
                    .iter()
                    .map(|v| u(&v["row_index"]))
                    .eq(projected.iter().take(outcomes.len()).copied()))?;
            if st["projection"] == "completed" {
                pf(outcomes.len() == projected.len())?;
            }
            for v in outcomes {
                pf(conversion_kind_ok(&v["outcome"]))?;
            }
            if a["result"]["kind"] == "ready" {
                let rows = rows_for(source, c);
                let expected: Vec<_> = rows
                    .iter()
                    .enumerate()
                    .filter(|(_, r)| hull_projected(r))
                    .map(|(i, _)| i as u64)
                    .collect();
                pf(outcomes.iter().map(|v| u(&v["row_index"])).eq(expected))?;
                for v in outcomes {
                    let row = rows[u(&v["row_index"]) as usize];
                    let o = &v["outcome"];
                    pf(o["kind"] != "overflow")?;
                    let value = if o["kind"] == "underflow" {
                        0.0
                    } else {
                        f(&o["value"])
                    };
                    pf(row["value"].as_f64().is_some_and(|v| {
                        v.to_bits() == (if value == 0.0 { 0.0 } else { value }).to_bits()
                    }))?;
                }
            }
        }
        if st["native"] == "completed" {
            pf(c["run"]["kernel_terminal"]["kind"] == "selected")?;
        }
        if st["native"] != "not_entered" {
            // R-D38 (DESIGN_v2 §2; RR:8823): an entered native stage has a Run
            // (4a), except a native capture failure before any Run beside a
            // registered prepared source (4b).
            if st["native"] == "failed" && a["run_ref"].is_null() {
                pf(d38_capture_before_run(c, a, ai))?;
            } else {
                pf(!a["run_ref"].is_null() && st["preparation"] == "completed")?;
            }
            pf((st["native"] == "completed")
                == (c["run"]["kernel_terminal"]["kind"] == "selected"))?;
        } else {
            pf(a["run_ref"].is_null())?;
        }
        if st["preparation"] == "completed" {
            pf(pm.len() == old.len()
                && new.len() == old.len()
                && pm.iter().all(|m| m["result"]["kind"] == "prepared")
                && s.is_some())?;
        }
        if a["result"]["kind"] == "ready" {
            pf(s.is_some()
                && pm.len() == old.len()
                && new.len() == old.len()
                && a["operational"]["old_coverage"] == "complete"
                && !proof.is_null()
                && st
                    .as_object()
                    .is_some_and(|o| o.values().all(|v| v == "completed"))
                && new.iter().all(|o| o["result"]["kind"] == "ready"))?;
            wf(exact_work(proof)
                && pm.iter().all(|m| exact_work(&m["work"]))
                && old.iter().chain(new).all(|o| exact_work(&o["work"]))
                && a["adapter"]["fault"].is_null()
                && exact_work(&a["overlay_work"])
                && exact_work(&a["g5a_work"]))?;
        }
        if c["status"] == "selected" {
            pf(a["result"]["kind"] == "ready")?;
        }
        // D19 (S06 section 1: C2 causes apply only to outcomes without an
        // actual C3 product attempt). An unavailable attempt's case is
        // unavailable with prepared_product_failure naming this attempt, so the
        // D4d reason table applies by the attempt's own error; a Ready
        // attempt's case is selected, or unavailable with receipt_failure.
        if a["result"]["kind"] == "unavailable" {
            pf(c["status"] == "unavailable"
                && c["reason"]["cause"]["kind"] == "prepared_product_failure"
                && u(&c["reason"]["cause"]["product_attempt_ref"]) == ai as u64)?;
        }
        if a["result"]["kind"] == "ready" {
            pf(c["status"] == "selected"
                || (c["status"] == "unavailable"
                    && c["reason"]["cause"]["kind"] == "receipt_failure"))?;
        }
        g5_coverage(a, c, s)?;
        g5_stages(a, &pf)?;
        // R1'-R4 (checkpoint A, D8; C3:232-236): work-class rules, deferred with
        // the other C3 WORK equations until every attempt's association passed.
        for ok in accounting_rules(a) {
            wf(ok)?;
        }
        if c["status"] == "unavailable"
            && c["reason"]["cause"]["kind"] == "prepared_product_failure"
        {
            pf(u(&c["reason"]["cause"]["product_attempt_ref"]) == ai as u64
                && a["result"]["kind"] == "unavailable")?;
            reason_table(c, a)?;
        }
    }
    // P9 (C3:279-287) and D37 (D35 widened): an unavailable result's error
    // agrees with its stage record in both directions; runs after every
    // attempt's association checks (class 3, typed checks).
    for a in list(&b["product_attempts"]) {
        if a["result"]["kind"] != "unavailable" {
            continue;
        }
        pf(error_stages(a))?;
    }
    for ok in pending_work {
        need(ok, "G5", "WORK_MISMATCH")?;
    }
    Ok(())
}
/// D37 (D35 widened; RV78-S1/S2, RV79-X1): the native transition sequence
/// (retained_product.rs `ProductCapture::prepare_owned_case` through `PreparedCase::solve_native` preparation and native, `PreparedCase::project_candidate` the
/// prepared candidate; retained_receipt.rs `PreparedTrace::enter` through `PreparedTrace::checked` trace states) fixes, for each
/// public error kind, the stage record it leaves:
/// - `preparation`: preparation failed; `native`: native failed (a nonselected
///   native outcome); `values`: values failed;
/// - `capture`: native failed before any Run; or, with no failed stage, the
///   capture checks before proof_start (native completed, proof_start not
///   entered), after a completed certificate (observables and G5a not
///   entered), or at the precharged commit (every stage completed). A failed
///   preparation is always the `preparation` kind, which carries its capture
///   cause;
/// - `proof`: proof_start, projection or certificate failed;
/// - `abandoned`: maxima or aliases failed, or the bound-rows view failed
///   after completed aliases (certificate not entered);
/// - after a completed certificate both checks are entered: `observable`
///   requires observables failed; `g5a` observables passed and G5a failed;
///   `numeric` both passed (every stage completed).
/// The kind must be one the first failed or terminal stage produces, and every
/// stage it presupposes is recorded so (g5_stages fixes the not-entered tail,
/// observables and G5a entered together, and stage <=> check). Row by row this
/// is I62's native table (R/I62/review_repair_07/RETURN_07F.md section 1):
/// capture (a)-(d), proof (a)-(c), abandoned (a)-(c), and one record each for
/// preparation, native, values, numeric, observable and g5a.
fn error_stages(a: &Value) -> bool {
    let st = &a["stages"];
    let is = |k: &str, v: &str| st[k] == v;
    let first_failed = STAGE8.iter().copied().find(|k| is(k, "failed"));
    let certified = first_failed.is_none() && is("certificate", "completed");
    let all_completed = certified && is("observables", "completed") && is("g5a", "completed");
    match text(&a["result"]["error"]["kind"]) {
        "preparation" => first_failed == Some("preparation"),
        "native" => first_failed == Some("native"),
        "values" => first_failed == Some("values"),
        "capture" => {
            first_failed == Some("native")
                || (first_failed.is_none()
                    && is("native", "completed")
                    && is("proof_start", "not_entered"))
                || (certified && is("observables", "not_entered") && is("g5a", "not_entered"))
                || all_completed
        }
        "proof" => matches!(first_failed, Some("proof_start" | "projection" | "certificate")),
        "abandoned" => {
            matches!(first_failed, Some("maxima" | "aliases"))
                || (first_failed.is_none()
                    && is("aliases", "completed")
                    && is("certificate", "not_entered"))
        }
        "observable" => certified && is("observables", "failed") && !is("g5a", "not_entered"),
        "g5a" => certified && is("observables", "completed") && is("g5a", "failed"),
        "numeric" => all_completed,
        _ => false,
    }
}
const STAGE8: [&str; 8] = [
    "preparation",
    "native",
    "proof_start",
    "projection",
    "maxima",
    "values",
    "aliases",
    "certificate",
];
/// P2/P6 (C3:196-201, 253-257; retained_receipt.rs `PreparedTrace::enter` through `PreparedTrace::checked`): stages advance
/// only through returned transitions, so after the first stage that did not
/// complete every later pipeline stage is not_entered; observables and G5a are
/// entered together; a source exists iff preparation completed; and the
/// completion kind follows the stages that entered it.
fn g5_stages(a: &Value, pf: &dyn Fn(bool) -> VResult) -> VResult {
    let st = &a["stages"];
    let mut seen_end = false;
    for k in STAGE8 {
        if seen_end {
            pf(st[k] == "not_entered")?;
        } else if st[k] != "completed" {
            seen_end = true;
        }
    }
    pf((st["observables"] == "not_entered") == (st["g5a"] == "not_entered"))?;
    pf((st["preparation"] == "completed") == !a["source_ref"].is_null())?;
    let proof = &a["proof"];
    if !proof.is_null() {
        let completion = text(&proof["completion"]["kind"]);
        if st["values"] == "failed" {
            pf(completion == "separate_failure")?;
        } else if st["certificate"] != "not_entered"
            || st["maxima"] == "failed"
            || st["aliases"] != "not_entered"
        {
            pf(completion == "merged")?;
        } else if st["projection"] != "completed" {
            pf(completion == "not_entered")?;
        }
    }
    Ok(())
}
fn row_kind(r: &Value) -> &'static str {
    let k = text(&r["kind"]);
    let unit = text(&r["unit"]);
    match k {
        "linear_solver_mode_basis"
        | "sparse_live_path_dense_parity_relative_delta"
        | "modulus_basis_record"
        | "combination_modulus_basis_record" => "non_quantity",
        "pipe_lame_hoop_stress_v2"
        | "pipe_lame_radial_stress_v2"
        | "pipe_section_pressure_hoop_stress"
        | "pipe_section_pressure_longitudinal_stress"
        | "constant_effort_support_applied_load"
        | "component_user_stress_multiplier_review"
        | "component_user_stiffness_macro_element_review"
        | "constant_effort_user_input_review"
        | "spring_hanger_user_input_review"
        | "expansion_joint_pressure_thrust_load_review" => "input_derived",
        "global_nodal_displacement_x"
        | "global_nodal_displacement_y"
        | "global_nodal_displacement_z"
        | "displacement_magnitude"
            if matches!(unit, "m" | "mm") =>
        {
            "translation"
        }
        "global_nodal_rotation_x" | "global_nodal_rotation_y" | "global_nodal_rotation_z"
            if unit == "rad" =>
        {
            "rotation"
        }
        "element_local_axial_force"
        | "element_local_shear_force_y"
        | "element_local_shear_force_z"
        | "pipe_wall_axial_force_v2"
        | "pipe_effective_axial_force_v2"
        | "reaction_resultant"
        | "support_reaction_force_magnitude_v2"
            if matches!(unit, "N" | "kN") =>
        {
            "force"
        }
        "element_local_torsional_moment"
        | "element_local_bending_moment_y"
        | "element_local_bending_moment_z"
        | "support_reaction_moment_magnitude_v2"
            if matches!(unit, "N*m" | "kN*m") =>
        {
            "moment"
        }
        "support_reaction_component_v2" | "pipe_wall_endpoint_action_v2"
            if matches!(unit, "N" | "kN") =>
        {
            "force"
        }
        "support_reaction_component_v2" | "pipe_wall_endpoint_action_v2"
            if matches!(unit, "N*m" | "kN*m") =>
        {
            "moment"
        }
        "element_local_axial_normal_stress"
        | "element_local_bending_normal_stress_y"
        | "element_local_bending_normal_stress_z"
        | "element_local_torsional_shear_stress"
        | "pipe_axial_membrane_stress_v2"
        | "pipe_elastic_normal_stress_maximum_v2"
        | "component_equal_factor_intensified_bending_stress_v1"
        | "open_formula_stress_summary"
            if matches!(unit, "Pa" | "MPa") =>
        {
            "stress"
        }
        _ => "not_covered",
    }
}
fn hull_projected(r: &Value) -> bool {
    row_kind(r) != "non_quantity"
        && !matches!(
            text(&r["kind"]),
            "support_reaction_force_magnitude_v2"
                | "support_reaction_moment_magnitude_v2"
                | "pipe_elastic_normal_stress_maximum_v2"
        )
}
fn normalized(r: &Value) -> f64 {
    let x = r["value"].as_f64().unwrap_or(f64::NAN);
    match text(&r["unit"]) {
        "mm" => x / 1000.0,
        "kN" | "kN*m" => x * 1000.0,
        "MPa" => x * 1_000_000.0,
        _ => x,
    }
}
fn component(r: &Value) -> Option<&'static str> {
    match text(&r["kind"]) {
        "global_nodal_displacement_x" => Some("UX"),
        "global_nodal_displacement_y" => Some("UY"),
        "global_nodal_displacement_z" => Some("UZ"),
        "global_nodal_rotation_x" => Some("RX"),
        "global_nodal_rotation_y" => Some("RY"),
        "global_nodal_rotation_z" => Some("RZ"),
        _ => None,
    }
}
fn row_body<'a>(r: &Value, s: &'a Value) -> (Option<usize>, Option<&'a Value>) {
    let maps = &s["id_maps"];
    let entity = &r["entity_ref"];
    let member = list(&maps["members"]).iter().find(|m| m["id"] == *entity);
    let node = list(&maps["nodes"])
        .iter()
        .find(|n| n["id"] == *entity)
        .map(|n| u(&n["kernel_node"]))
        .or_else(|| member.map(|m| u(&m["node_i"])))
        .or_else(|| {
            list(&maps["support_ids"])
                .iter()
                .find(|s| s["id"] == *entity)
                .map(|s| u(&s["node"]))
        });
    let body = node.and_then(|n| {
        list(&s["body_membership"])
            .iter()
            .position(|b| list(&b["nodes"]).iter().any(|x| u(x) == n))
    });
    (body, member)
}
struct NumericCase<'a> {
    case: &'a Value,
    source: &'a Value,
    /// The proof-owned summary coverage of this selected case's product attempt.
    coverage: &'a Value,
    rows: Vec<&'a Value>,
    extents: Vec<f64>,
    raw: Vec<[f64; 4]>,
    final_scales: Vec<[f64; 4]>,
    prescribed: BTreeSet<(String, String)>,
}
fn finite_check(v: f64, gate: &'static str) -> VResult<f64> {
    need(v.is_finite(), gate, "SCALE_MISMATCH")?;
    Ok(v)
}
/// One body's membership check and native extent L (adaptive::body_extent:
/// per-axis max - min, then ((dx·dx + dy·dy) + dz·dz).sqrt()).
fn body_extent(s: &Value, bi: usize, body: &Value) -> VResult<f64> {
    need(
        u(&body["body"]) == bi as u64 && !list(&body["nodes"]).is_empty(),
        "G5a",
        "SCALE_MISMATCH",
    )?;
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    for ni in list(&body["nodes"]) {
        let n = at(&s["id_maps"]["nodes"], ni, "G5a", "SCALE_MISMATCH")?;
        for j in 0..3 {
            lo[j] = lo[j].min(f(&n["coordinates"][j]));
            hi[j] = hi[j].max(f(&n["coordinates"][j]));
        }
    }
    let mut d = [0.0; 3];
    for j in 0..3 {
        d[j] = finite_check(hi[j] - lo[j], "G5a")?;
    }
    let x = finite_check(d[0] * d[0], "G5a")?;
    let y = finite_check(d[1] * d[1], "G5a")?;
    let xy = finite_check(x + y, "G5a")?;
    let z = finite_check(d[2] * d[2], "G5a")?;
    finite_check(finite_check(xy + z, "G5a")?.sqrt(), "G5a")
}
fn numeric_cases(source: &Value) -> VResult<Vec<NumericCase<'_>>> {
    let b = &source["retained_precision"]["body"];
    let mut out = Vec::new();
    for c in list(&b["cases"])
        .iter()
        .filter(|c| c["status"] == "selected")
    {
        let s = at(&b["sources"], &c["source_ref"], "G5a", "SCALE_MISMATCH")?;
        let coverage = &at(
            &b["product_attempts"],
            &c["product_attempt_ref"],
            "G5a",
            "SCALE_MISMATCH",
        )?["proof"]["summary_coverage"];
        let rows = rows_for(source, c);
        let mut extents = Vec::new();
        let mut raw = Vec::new();
        let mut prescribed = BTreeSet::new();
        for d in list(&s["constraints"]) {
            let n = at(
                &s["id_maps"]["nodes"],
                &d["dof"]["node"],
                "G5a",
                "SCALE_MISMATCH",
            )?;
            prescribed.insert((text(&n["id"]).into(), text(&d["dof"]["component"]).into()));
        }
        for (bi, body) in list(&s["body_membership"]).iter().enumerate() {
            let extent = body_extent(s, bi, body)?;
            extents.push(extent);
            let mut maxima = [0.0f64; 4];
            for r in &rows {
                let n = finite_check(normalized(r), "G5a")?;
                let input = component(r).is_some_and(|c| {
                    prescribed.contains(&(text(&r["entity_ref"]).into(), c.into()))
                });
                if row_body(r, s).0 == Some(bi) && !input {
                    if let Some(k) = NAMES.iter().position(|k| *k == row_kind(r)) {
                        maxima[k] = maxima[k].max(n.abs());
                    }
                }
            }
            let [tr, ro, fo, mo] = maxima;
            let coupled = if extent == 0.0 {
                maxima
            } else {
                [
                    tr.max(finite_check(extent * ro, "G5a")?),
                    ro.max(finite_check(tr / extent, "G5a")?),
                    fo.max(finite_check(mo / extent, "G5a")?),
                    mo.max(finite_check(extent * fo, "G5a")?),
                ]
            };
            raw.push(coupled);
        }
        out.push(NumericCase {
            case: c,
            source: s,
            coverage,
            rows,
            extents,
            final_scales: raw.clone(),
            raw,
            prescribed,
        });
    }
    Ok(out)
}
/// I57 s2/s4 canonical layout rebuilt from the bound source maps in the native
/// recover::layout order. Only a constrained displacement/rotation row is
/// input-derived, and every C3 prescription is exact +0, so D=false; a
/// purported force/moment input-derived row (or any other flag) cannot match.
/// G8 still independently binds these maps to the actual invocation.
fn canonical_layout(s: &Value) -> VResult<Value> {
    let fail = |ok| need(ok, "G5a", "SCALE_MISMATCH");
    let maps = &s["id_maps"];
    let bodies = list(&s["body_membership"]);
    let body_of = |n: u64| -> VResult<usize> {
        bodies
            .iter()
            .position(|b| list(&b["nodes"]).iter().any(|x| u(x) == n))
            .ok_or_else(|| error("G5a", "SCALE_MISMATCH"))
    };
    let component = |v: &Value| -> VResult<usize> {
        DOFS.iter()
            .position(|d| v == *d)
            .ok_or_else(|| error("G5a", "SCALE_MISMATCH"))
    };
    let mut fixed = BTreeSet::new();
    for c in list(&s["constraints"]) {
        let di = component(&c["dof"]["component"])?;
        fail(fixed.insert((u(&c["dof"]["node"]), di)) && c["value"] == "0000000000000000")?;
    }
    let mut layout = Vec::new();
    let mut add = |q: Value, kind: &str, n: u64, input: bool| -> VResult {
        layout.push(json!({"index":layout.len(),"quantity":q,"kind":kind,"body":body_of(n)?,"input_derived":input}));
        Ok(())
    };
    let nodes = list(&maps["nodes"]).len() as u64;
    for n in 0..nodes {
        for (di, d) in DOFS.iter().enumerate() {
            add(
                json!({"tag":"displacement","dof":{"node":n,"component":d}}),
                if di < 3 { "translation" } else { "rotation" },
                n,
                fixed.contains(&(n, di)),
            )?;
        }
    }
    for n in 0..nodes {
        add(
            json!({"tag":"displacement_magnitude","node":n}),
            "translation",
            n,
            false,
        )?;
    }
    for m in list(&maps["members"]) {
        for end in ["i", "j"] {
            for (di, d) in DOFS.iter().enumerate() {
                add(
                    json!({"tag":"end_action","member":m["kernel_member"],"end":end,"component":d}),
                    if di < 3 { "force" } else { "moment" },
                    u(&m["node_i"]),
                    false,
                )?;
            }
        }
    }
    for st in list(&s["stations"]) {
        let m = at(&maps["members"], &st["member"], "G5a", "SCALE_MISMATCH")?;
        for (di, d) in DOFS.iter().enumerate() {
            add(
                json!({"tag":"station_action","station":st["id"],"component":d}),
                if di < 3 { "force" } else { "moment" },
                u(&m["node_i"]),
                false,
            )?;
        }
    }
    for sp in list(&maps["springs"]) {
        let di = component(&sp["component"])?;
        add(
            json!({"tag":"spring_action","spring":sp["kernel_spring"],"component":sp["component"]}),
            if di < 3 { "force" } else { "moment" },
            u(&sp["node"]),
            false,
        )?;
    }
    for c in list(&s["constraints"]) {
        let di = component(&c["dof"]["component"])?;
        add(
            json!({"tag":"reaction","dof":c["dof"]}),
            if di < 3 { "force" } else { "moment" },
            u(&c["dof"]["node"]),
            false,
        )?;
    }
    for g in list(&s["supports"]) {
        for (kind, tag) in [
            ("force", "support_force_magnitude"),
            ("moment", "support_moment_magnitude"),
        ] {
            add(json!({"tag":tag,"support":g["id"]}), kind, u(&g["node"]), false)?;
        }
    }
    Ok(Value::Array(layout))
}
/// G5a for each selected case, in the I57 s4 order: existing summary encodings
/// and ranges with native p/P/floor rules; canonical source layout and extent;
/// compact-flag Boolean feasibility; estimate/charge rederivation; the exact
/// PP validate_summary_shape rosters (items 1-4); directly derivable data facts.
/// The private stop/data facts come only from the proof-owned coverage; final
/// rows never supply a native nonzero or data fact. No Cartesian roster.
/// I57 s2/s4 G5a coverage relations shared by a selected case (`sel` is its
/// Selection) and an unavailable case whose attempt keeps a complete vector
/// (`sel` is None): native p and the 2p verification record; the canonical
/// layout; compact-flag feasibility; for a selected case the estimate/charge
/// rederivation, the exact stop/estimate/charge rosters and the B roster; then
/// the record bound/theta/data_blocks relations and the direct data facts.
/// Without a Selection, p comes from the selected Run's last attempt, E and
/// theta from its verification record, and at p512 floor positivity is
/// Φ = phi_512(ê) > 0 from that record (adaptive.rs `rule`). No Selection
/// roster or selected pass condition is applied to an unavailable attempt.
fn coverage_g5a(
    case: &Value,
    source: &Value,
    coverage: &Value,
    sel: Option<&Value>,
    extents: &[f64],
) -> VResult {
    let fail = |ok| need(ok, "G5a", "SCALE_MISMATCH");
    let nb = list(&source["body_membership"]).len();
    fail(extents.len() == nb)?;
    let run = &case["run"];
    let last = list(&run["attempts"])
        .last()
        .ok_or_else(|| error("G5a", "SCALE_MISMATCH"))?;
    let p = match sel {
        Some(s) => u(&s["precision"]),
        None => u(&last["precision"]),
    };
    fail(matches!(p, 128 | 256 | 512))?;
    let record = at(
        &run["records"],
        &last["verification"]["record"],
        "G5a",
        "SCALE_MISMATCH",
    )?;
    let verification = &record["verification"];
    fail(
        u(&record["precision"]) == 2 * p
            && !verification.is_null()
            && sel.is_none_or(|s| u(&s["verification_precision"]) == 2 * p),
    )?;
    // Canonical source layout (extent L was rederived in adaptive::body_extent
    // order when the case was opened).
    fail(source["layout"] == canonical_layout(source)?)?;
    let coverage = list(coverage);
    fail(
        coverage.len() == nb
            && coverage
                .iter()
                .map(|e| u(&e["body"]))
                .eq(0..nb as u64),
    )?;
    let resolution = match sel {
        Some(s) => &s["resolution_scale"],
        None => &verification["resolution"],
    };
    let theta = &verification["theta"];
    for v in [resolution, theta] {
        fail(list(v).iter().map(|x| u(&x["body"])).eq(0..nb as u64))?;
    }
    let has_data: Vec<bool> = coverage.iter().map(|e| e["has_data"] == true).collect();
    let mut want_stop = Vec::new();
    let mut want_estimate = Vec::new();
    let mut want_charge = Vec::new();
    for (bi, entry) in coverage.iter().enumerate() {
        let stop: [bool; 4] = std::array::from_fn(|k| entry["stop"][k] == true);
        let mut present = [false; 4];
        let mut non_input = [false; 4];
        for m in list(&source["layout"]) {
            if u(&m["body"]) != bi as u64 {
                continue;
            }
            if let Some(k) = NAMES.iter().position(|k| m["kind"] == *k) {
                present[k] = true;
                non_input[k] |= m["input_derived"] == false;
            }
        }
        let l = extents[bi];
        let e = [
            f(&resolution[bi]["force"]),
            f(&resolution[bi]["moment"]),
        ];
        let floor = match sel {
            Some(s) if !s["floor"].is_null() => [
                f(&s["floor"][bi]["force"]) > 0.0,
                f(&s["floor"][bi]["moment"]) > 0.0,
            ],
            None if p == 512 => {
                let h = e_hat(e, l);
                [phi_512(h[0]) > 0.0, phi_512(h[1]) > 0.0]
            }
            _ => [false; 2],
        };
        // Necessary public consistency: some permitted private A (nonzero only
        // where a non-input row exists; D=false) reproduces the attested stop
        // bits under final_case.rs's formula. No A is claimed as the actual one.
        let feasible = (0u8..16).any(|mask| {
            let a: [bool; 4] = std::array::from_fn(|k| mask >> k & 1 == 1);
            if (0..4).any(|k| a[k] && !non_input[k]) {
                return false;
            }
            let mut positive = if l == 0.0 {
                a
            } else {
                [a[0] || a[1], a[0] || a[1], a[2] || a[3], a[2] || a[3]]
            };
            positive[2] |= floor[0];
            positive[3] |= floor[1];
            (0..4).all(|k| (present[k] && (positive[k] || a[k])) == stop[k])
        });
        fail(feasible)?;
        let e = [e[0] > 0.0, e[1] > 0.0];
        let hats = if l == 0.0 { e } else { [e[0] || e[1]; 2] };
        let estimate = [present[2] && hats[0], present[3] && hats[1]];
        // Native p512: every force/moment row is non-input-derived, so the
        // charge flags equal the force/moment stop flags; otherwise estimate.
        let charge = if p == 512 { [stop[2], stop[3]] } else { estimate };
        for k in 0..4 {
            if stop[k] {
                want_stop.push((bi as u64, k));
            }
        }
        for k in 0..2 {
            if estimate[k] {
                want_estimate.push((bi as u64, k + 2));
            }
            if charge[k] {
                want_charge.push((bi as u64, k + 2));
            }
        }
    }
    if let Some(sel) = sel {
        // Items 1 and 2: exactly one entry per true bit, none per false bit.
        let roster = |key: &str| -> Vec<(u64, usize)> {
            list(&sel[key])
                .iter()
                .map(|v| {
                    (
                        u(&v["body"]),
                        NAMES.iter().position(|k| v["kind"] == *k).unwrap_or(4),
                    )
                })
                .collect()
        };
        fail(
            roster("stop_rule") == want_stop
                && roster("verification_estimate") == want_estimate
                && roster("verification_charge") == want_charge,
        )?;
        // Item 4: one finite positive B iff has_data.
        fail(
            list(&sel["certified_bound"])
                .iter()
                .map(|v| u(&v["body"]))
                .eq((0..nb as u64).filter(|b| has_data[*b as usize])),
        )?;
    }
    // The verification record's per-body bound (null for a no-data body); a
    // no-data body has theta=+0; data_blocks=0 iff no body has data and is
    // otherwise at least the true-body count. The record is a second
    // consistency relation, not an independent witness of the data fact.
    let bound = list(&verification["bound"]);
    fail(
        bound.len() == nb
            && bound
                .iter()
                .enumerate()
                .all(|(i, x)| u(&x["body"]) == i as u64 && x["value"].is_null() != has_data[i]),
    )?;
    fail((0..nb).all(|bi| has_data[bi] || theta[bi]["value"] == "0000000000000000"))?;
    let true_count = has_data.iter().filter(|x| **x).count() as u64;
    let blocks = u(&verification["data_blocks"]);
    fail((blocks == 0) == (true_count == 0) && blocks >= true_count)?;
    // Directly derivable data facts from separate original contributions:
    // no free DOF implies false; a nonzero admitted nodal term at a free DOF
    // implies true. Netted loads or zero final rows never imply false.
    let fixed: BTreeSet<(u64, &str)> = list(&source["constraints"])
        .iter()
        .map(|d| (u(&d["dof"]["node"]), text(&d["dof"]["component"])))
        .collect();
    for bi in 0..nb {
        let nodes: Vec<u64> = list(&source["body_membership"][bi]["nodes"])
            .iter()
            .map(u)
            .collect();
        let free = |n: u64, d: &str| nodes.contains(&n) && !fixed.contains(&(n, d));
        if !nodes.iter().any(|n| DOFS.iter().any(|d| free(*n, *d))) {
            fail(!has_data[bi])?;
        }
        if list(&source["nodal_terms"]).iter().any(|t| {
            free(u(&t["dof"]["node"]), text(&t["dof"]["component"])) && f(&t["value"]) != 0.0
        }) {
            fail(has_data[bi])?;
        }
    }
    Ok(())
}
/// G5a in case order: each selected case through its Selection, and each
/// unavailable case whose product attempt keeps a complete coverage vector
/// through the Selection-free source/record/data relations (I57 s4).
fn g5a(body: &Value, cases: &[NumericCase<'_>]) -> VResult {
    let mut selected = cases.iter();
    for c in list(&body["cases"]) {
        if c["status"] == "selected" {
            g5a_selected(
                selected
                    .next()
                    .ok_or_else(|| error("G5a", "SCALE_MISMATCH"))?,
            )?;
        } else if c["status"] == "unavailable" && !c["product_attempt_ref"].is_null() {
            let a = at(
                &body["product_attempts"],
                &c["product_attempt_ref"],
                "G5a",
                "SCALE_MISMATCH",
            )?;
            let coverage = &a["proof"]["summary_coverage"];
            if coverage.is_null() {
                continue;
            }
            let s = at(&body["sources"], &a["source_ref"], "G5a", "SCALE_MISMATCH")?;
            let mut extents = Vec::new();
            for (bi, b) in list(&s["body_membership"]).iter().enumerate() {
                extents.push(body_extent(s, bi, b)?);
            }
            coverage_g5a(c, s, coverage, None, &extents)?;
        }
    }
    Ok(())
}
fn g5a_selected(c: &NumericCase<'_>) -> VResult {
    let fail = |ok| need(ok, "G5a", "SCALE_MISMATCH");
    {
        let sel = &c.case["selection"];
        let nb = c.raw.len();
        let p = u(&sel["precision"]);
        // Existing summary encodings/ranges and native p/P/floor rules.
        fail(
            matches!(p, 128 | 256 | 512)
                && u(&sel["verification_precision"]) == 2 * p
                && sel["floor_ratio"] == "3dd0000000000000"
                && f(&sel["pivot_margin_min"]) > 0.0
                && f(&sel["rcond"]) > 0.0,
        )?;
        for key in ["body_scales", "resolution_scale", "theta"] {
            fail(
                list(&sel[key])
                    .iter()
                    .map(|x| u(&x["body"]))
                    .eq(0..nb as u64),
            )?;
        }
        for (key, limit) in [
            ("stop_rule", 2f64.powi(-64)),
            ("verification_estimate", 0.25),
            ("verification_charge", 1.0),
        ] {
            let mut pairs = BTreeSet::new();
            let mut last = None;
            for v in list(&sel[key]) {
                let bi = u(&v["body"]);
                let ki = NAMES
                    .iter()
                    .position(|k| v["kind"] == *k)
                    .ok_or_else(|| error("G5a", "SCALE_MISMATCH"))?;
                let pair = (bi, ki);
                fail(
                    bi < nb as u64
                        && (key == "stop_rule" || ki >= 2)
                        && pairs.insert(pair)
                        && last.is_none_or(|p| pair > p)
                        && f(&v["value"]) <= limit,
                )?;
                last = Some(pair);
            }
        }
        fail(list(&sel["theta"]).iter().all(|v| f(&v["value"]) <= 0.5))?;
        let mut bs = BTreeSet::new();
        for v in list(&sel["certified_bound"]) {
            fail(u(&v["body"]) < nb as u64 && bs.insert(u(&v["body"])) && f(&v["value"]) > 0.0)?;
        }
        fail(sel["floor"].is_null() == (p != 512))?;
        if !sel["floor"].is_null() {
            fail(
                list(&sel["floor"])
                    .iter()
                    .map(|v| u(&v["body"]))
                    .eq(0..nb as u64),
            )?;
        }
        coverage_g5a(c.case, c.source, c.coverage, Some(sel), &c.extents)?;
        // Item 3: resolution and theta cover every body once (above); resolution
        // keeps its original zero/sanity/lower checks.
        for bi in 0..nb {
            let scale = c.raw[bi];
            let l = c.extents[bi];
            let e = [
                f(&sel["resolution_scale"][bi]["force"]),
                f(&sel["resolution_scale"][bi]["moment"]),
            ];
            let hat = if l == 0.0 {
                e
            } else {
                [
                    e[0].max(finite_check(e[1] / l, "G5a")?),
                    e[1].max(finite_check(l * e[0], "G5a")?),
                ]
            };
            let upper = [
                finite_check(hat[0] * f64::from_bits(0x3ff0000000001000), "G5a")?,
                finite_check(hat[1] * f64::from_bits(0x3ff0000000001000), "G5a")?,
            ];
            fail(upper[0] >= scale[2] && upper[1] >= scale[3])?;
            for r in &c.rows {
                if row_body(r, c.source).0 == Some(bi) {
                    if let Some(k) = NAMES.iter().position(|k| *k == row_kind(r)) {
                        if k >= 2 && e[k - 2] == 0.0 {
                            fail(normalized(r).to_bits() == 0)?;
                        }
                    }
                }
            }
            for member in list(&c.source["id_maps"]["members"]) {
                if !list(&c.source["body_membership"][bi]["nodes"]).contains(&member["node_i"]) {
                    continue;
                }
                let section = list(&c.source["section_terms"])
                    .iter()
                    .find(|s| s["member"] == member["kernel_member"])
                    .ok_or_else(|| error("G5a", "SCALE_MISMATCH"))?;
                for k in 0..2 {
                    let mut endpoints = [0.0; 2];
                    for (end, ni) in [&member["node_i"], &member["node_j"]].iter().enumerate() {
                        let node = at(&c.source["id_maps"]["nodes"], ni, "G5a", "SCALE_MISMATCH")?;
                        let mut xyz = [0.0; 3];
                        for j in 0..3 {
                            let hits: Vec<_> = c
                                .rows
                                .iter()
                                .filter(|r| {
                                    r["entity_ref"] == node["id"]
                                        && component(r) == Some(DOFS[k * 3 + j])
                                })
                                .collect();
                            fail(hits.len() == 1)?;
                            xyz[j] = normalized(hits[0]).abs();
                        }
                        endpoints[end] =
                            finite_check(finite_check(xyz[0] + xyz[1], "G5a")? + xyz[2], "G5a")?;
                    }
                    let total = finite_check(endpoints[0] + endpoints[1], "G5a")?;
                    let threshold = 2f64.powi(-59) * scale[k];
                    let lower = if total <= threshold {
                        0.0
                    } else {
                        let margin = 2f64.powi(-60) * scale[k];
                        finite_check(
                            f(section
                                .get(if k == 0 {
                                    "axial_stiffness"
                                } else {
                                    "torsional_stiffness"
                                })
                                .unwrap_or(&Value::Null))
                                * (total - margin),
                            "G5a",
                        )?
                    };
                    fail(upper[k] >= lower)?;
                }
            }
        }
    }
    Ok(())
}
fn g5b(cases: &mut [NumericCase<'_>]) -> VResult {
    let sf = |ok| need(ok, "G5b", "SCALE_MISMATCH");
    let section = |ok| need(ok, "G5b", "SECTION_MISMATCH");
    for c in cases {
        let s = &c.case["selection"];
        for bi in 0..c.raw.len() {
            let mut sc = c.raw[bi];
            if u(&s["precision"]) == 512 {
                let l = c.extents[bi];
                let e = [
                    f(&s["resolution_scale"][bi]["force"]),
                    f(&s["resolution_scale"][bi]["moment"]),
                ];
                // C1 G5b: the same E/ê/Φ at p512, with exact scale bits.
                let hats = e_hat(e, l);
                for j in 0..2 {
                    let phi = phi_512(hats[j]);
                    sf(s["floor"][bi][NAMES[j + 2]] == bits(phi))?;
                    sc[j + 2] = sc[j + 2].max(phi);
                }
            }
            for j in 0..4 {
                sf(s["body_scales"][bi][NAMES[j]] == bits(sc[j]))?;
            }
            c.final_scales[bi] = sc;
        }
        section(list(&s["section_terms"]).len() == list(&c.source["section_terms"]).len())?;
        for (left, right) in list(&s["section_terms"])
            .iter()
            .zip(list(&c.source["section_terms"]))
        {
            let member = list(&c.source["id_maps"]["members"])
                .iter()
                .find(|m| m["kernel_member"] == right["member"])
                .ok_or_else(|| error("G5b", "SECTION_MISMATCH"))?;
            section(left["member_id"] == member["id"])?;
            for k in [
                "area",
                "section_modulus",
                "length",
                "axial_stiffness",
                "torsional_stiffness",
            ] {
                section(left[k] == right[k] && f(&right[k]) > 0.0)?;
            }
        }
        for row in &c.rows {
            if row_kind(row) == "stress" {
                row_scale(row, c)?;
            }
        }
    }
    Ok(())
}
fn row_scale(row: &Value, c: &NumericCase<'_>) -> VResult<Option<f64>> {
    let kind = row_kind(row);
    let (body, member) = row_body(row, c.source);
    let Some(bi) = body else { return Ok(None) };
    let sc = c
        .final_scales
        .get(bi)
        .ok_or_else(|| error("G5b", "SCALE_MISMATCH"))?;
    if let Some(k) = NAMES.iter().position(|k| *k == kind) {
        return Ok(Some(sc[k]));
    }
    if kind != "stress" {
        return Ok(None);
    }
    let Some(member) = member else {
        return Ok(None);
    };
    let section = list(&c.source["section_terms"])
        .iter()
        .find(|s| s["member"] == member["kernel_member"])
        .ok_or_else(|| error("G5b", "SECTION_MISMATCH"))?;
    let k = match text(&row["kind"]) {
        "pipe_elastic_normal_stress_maximum_v2" => f64::from_bits(0x4006a09e667f3bcd),
        "open_formula_stress_summary" => 4.0,
        "component_equal_factor_intensified_bending_stress_v1" => {
            return Err(error("G5b", "SECTION_MISMATCH"))
        }
        _ => 1.0,
    };
    let axial = finite_check(sc[2] / f(&section["area"]), "G5b")?;
    let bend = finite_check(sc[3] / f(&section["section_modulus"]), "G5b")?;
    let weighted = finite_check(k * bend, "G5b")?;
    Ok(Some(finite_check(axial + weighted, "G5b")?))
}
fn g5c(cases: &[NumericCase<'_>]) -> VResult<Vec<RowClassification>> {
    let mut classes = Vec::new();
    for c in cases {
        let s = &c.case["selection"];
        let actual: BTreeSet<_> = list(&s["input_derived_dofs"])
            .iter()
            .map(|d| {
                (
                    text(&d["node_id"]).to_string(),
                    text(&d["component"]).to_string(),
                )
            })
            .collect();
        need(
            actual == c.prescribed && actual.len() == list(&s["input_derived_dofs"]).len(),
            "G5c",
            "INPUT_DOF_MISMATCH",
        )?;
        let mut absolutes = Vec::new();
        let mut uncovered = Vec::new();
        for row in &c.rows {
            let n = normalized(row);
            let k = row_kind(row);
            let input = component(row).is_some_and(|d| {
                c.prescribed
                    .contains(&(text(&row["entity_ref"]).into(), d.into()))
            });
            let scale = if input || matches!(k, "non_quantity" | "input_derived") {
                None
            } else {
                row_scale(row, c)?
            };
            let class = if k == "non_quantity" {
                AccuracyClass::NonQuantity
            } else if input || k == "input_derived" {
                if input {
                    need(n.to_bits() == 0, "G5c", "INPUT_DOF_MISMATCH")?;
                }
                AccuracyClass::InputDerived
            } else if let Some(sc) = scale {
                if sc >= f64::from_bits(0x0230000000000000) && n.abs() >= 2f64.powi(-34) * sc {
                    AccuracyClass::RelativeVerified
                } else {
                    let bound = absolute_bound(n, sc)
                        .map_err(|_| error("G5c", "CLASSIFICATION_MISMATCH"))?;
                    absolutes.push(json!({"result_id":row["id"],"bound":bits(bound)}));
                    AccuracyClass::AbsoluteVerified {
                        bound_bits: bound.to_bits(),
                    }
                }
            } else {
                uncovered.push(row["id"].clone());
                AccuracyClass::NotCovered
            };
            classes.push(RowClassification {
                result_id: text(&row["id"]).into(),
                basis_ref: row["basis_ref"].clone(),
                normalized_bits: n.to_bits(),
                scale_bits: scale.map(f64::to_bits),
                class,
            });
        }
        need(
            s["absolute_verified"] == json!(absolutes) && s["not_covered"] == json!(uncovered),
            "G5c",
            "CLASSIFICATION_MISMATCH",
        )?;
    }
    Ok(classes)
}
fn unit_value(q: &Value, dimension: Dimension) -> VResult<f64> {
    let value = q["value"]
        .as_f64()
        .ok_or_else(|| error("G8", "PREPARATION_MISMATCH"))?;
    let from = unit_by_symbol(text(&q["unit"]), dimension)
        .map_err(|_| error("G8", "PREPARATION_MISMATCH"))?;
    let to = canonical_unit(dimension).ok_or_else(|| error("G8", "PREPARATION_MISMATCH"))?;
    let n = convert_for_dimension(value, dimension, from, to)
        .map_err(|_| error("G8", "PREPARATION_MISMATCH"))?;
    need(
        n.is_finite() && (dimension != Dimension::Temperature || n >= 0.0),
        "G8",
        "PREPARATION_MISMATCH",
    )?;
    Ok(n)
}
fn selected_material(raw: &Value, case: &Value) -> VResult<([f64; 2], Value)> {
    let fail = |ok| need(ok, "G8", "PREPARATION_MISMATCH");
    // All authored quantities are normalized by the producer before selection.
    let mut points = Vec::new();
    for p in std::iter::once(raw).chain(list(&raw["temperature_points"]).iter()) {
        for (k, d) in [
            ("elastic_modulus", Dimension::Stress),
            ("shear_modulus", Dimension::Stress),
            ("temperature", Dimension::Temperature),
            (
                "thermal_expansion_coefficient",
                Dimension::ThermalExpansionCoefficient,
            ),
        ] {
            if !p[k].is_null() {
                unit_value(&p[k], d)?;
            }
        }
    }
    fail(case["modulus_basis_ref"].is_null() || case["modulus_basis_temperature"].is_null())?;
    let pair = |p: &Value| -> VResult<[f64; 2]> {
        let eg = [
            unit_value(&p["elastic_modulus"], Dimension::Stress)?,
            unit_value(&p["shear_modulus"], Dimension::Stress)?,
        ];
        fail(eg.iter().all(|x| *x > 0.0))?;
        Ok(eg)
    };
    if !case["modulus_basis_ref"].is_null() {
        let p = list(&raw["temperature_points"])
            .iter()
            .find(|p| p["id"] == case["modulus_basis_ref"])
            .ok_or_else(|| error("G8", "PREPARATION_MISMATCH"))?;
        return Ok((pair(p)?, json!({"kind":"named_point","point_id":p["id"]})));
    }
    if case["modulus_basis_temperature"].is_null() {
        return Ok((pair(raw)?, json!({"kind":"base"})));
    }
    let t = unit_value(&case["modulus_basis_temperature"], Dimension::Temperature)?;
    for p in list(&raw["temperature_points"]) {
        if !p["temperature"].is_null() {
            points.push((unit_value(&p["temperature"], Dimension::Temperature)?, p));
        }
    }
    points.sort_by(|a, b| a.0.total_cmp(&b.0));
    fail(!points.windows(2).any(|p| p[0].0 == p[1].0))?;
    let bracket = points
        .windows(2)
        .find(|p| p[0].0 < t && t < p[1].0)
        .ok_or_else(|| error("G8", "PREPARATION_MISMATCH"))?;
    let (lo, hi) = (bracket[0], bracket[1]);
    let l = pair(lo.1)?;
    let h = pair(hi.1)?;
    let la = unit_value(
        &lo.1["thermal_expansion_coefficient"],
        Dimension::ThermalExpansionCoefficient,
    )?;
    let ha = unit_value(
        &hi.1["thermal_expansion_coefficient"],
        Dimension::ThermalExpansionCoefficient,
    )?;
    let fraction = (t - lo.0) / (hi.0 - lo.0);
    let eg = [
        l[0] + fraction * (h[0] - l[0]),
        l[1] + fraction * (h[1] - l[1]),
    ];
    fail(eg.iter().all(|x| x.is_finite() && *x > 0.0) && (la + fraction * (ha - la)).is_finite())?;
    Ok((
        eg,
        json!({"kind":"interpolated","lower_point_id":lo.1["id"],"upper_point_id":hi.1["id"],"target_kelvin":bits(t)}),
    ))
}
fn normalized_nodes(model: &Value) -> VResult<Vec<Value>> {
    let mut out = Vec::new();
    for (i, n) in list(&model["nodes"]).iter().enumerate() {
        let mut xyz = Vec::new();
        for a in ["x", "y", "z"] {
            xyz.push(bits(unit_value(
                &json!({"value":n["position"][a],"unit":model["project"]["units"]["length"]}),
                Dimension::Length,
            )?));
        }
        out.push(json!({"model_index":i,"kernel_node":i,"id":n["id"],"coordinates":xyz}));
    }
    Ok(out)
}
fn selector(case: &Value) -> VResult<Value> {
    need(
        case["modulus_basis_ref"].is_null() || case["modulus_basis_temperature"].is_null(),
        "G8",
        "PREPARATION_MISMATCH",
    )?;
    Ok(if !case["modulus_basis_ref"].is_null() {
        json!({"kind":"named","id":case["modulus_basis_ref"]})
    } else if !case["modulus_basis_temperature"].is_null() {
        json!({"kind":"temperature","kelvin":bits(unit_value(&case["modulus_basis_temperature"],Dimension::Temperature)?)})
    } else {
        json!({"kind":"base"})
    })
}
fn find_node(nodes: &[Value], id: &Value) -> VResult<usize> {
    nodes
        .iter()
        .position(|n| n["id"] == *id)
        .ok_or_else(|| error("G8", "PREPARATION_MISMATCH"))
}
fn dof_index(v: &Value) -> VResult<usize> {
    DOFS.iter()
        .position(|d| v == *d)
        .ok_or_else(|| error("G8", "PREPARATION_MISMATCH"))
}
fn g8(source: &Value, inv: &Value) -> VResult {
    let b = &source["retained_precision"]["body"];
    let fail = |ok| need(ok, "G8", "PREPARATION_MISMATCH");
    need(
        inv.as_object().is_some_and(|o| {
            o.len() == 2 && o.contains_key("request") && o.contains_key("solver_mode")
        }) && hash(
            "source_blocks_invocation_v1",
            inv,
            "G8",
            "INVOCATION_MISMATCH",
        )? == b["invocation"]["value"],
        "G8",
        "INVOCATION_MISMATCH",
    )?;
    let req = &inv["request"];
    let model = &req["model"];
    let mode = text(&inv["solver_mode"]);
    need(
        matches!(mode, "sparse_interactive" | "dense_scrutiny")
            && model["project"]["id"] == source["model_ref"],
        "G8",
        "INVOCATION_MISMATCH",
    )?;
    need(
        // D31: the producer treats model 0.1.0 and 0.2.0 on one branch
        // (pressure_runtime.rs `validate_profile`); 0.4.0 stays excluded.
        matches!(text(&model["schema_version"]), "0.1.0" | "0.2.0" | "0.3.0")
            && model["pressure_contract"].is_null()
            && list(&model["combinations"]).is_empty()
            && list(&model["components"]).is_empty()
            && !model
                .as_object()
                .is_some_and(|m| m.contains_key("reference_configurations")),
        "G8",
        "INVOCATION_MISMATCH",
    )?;
    let nodes = list(&model["nodes"]);
    let pipes = list(&model["pipe_segments"]);
    let supports = list(&model["supports"]);
    let cases = list(&model["load_cases"]);
    let materials = if list(&req["materials"]).is_empty() {
        list(&model["materials"])
    } else {
        list(&req["materials"])
    };
    for objects in [nodes, pipes, supports, materials] {
        let mut seen = BTreeSet::new();
        for o in objects {
            fail(!text(&o["id"]).is_empty() && seen.insert(text(&o["id"])))?;
        }
    }
    fail(
        !nodes.is_empty()
            && nodes
                .len()
                .checked_mul(6)
                .is_some_and(|n| n <= u32::MAX as usize)
            && pipes
                .len()
                .checked_mul(3)
                .is_some_and(|n| n <= u32::MAX as usize),
    )?;
    let node_maps = normalized_nodes(model)?;
    let used: BTreeSet<_> = pipes.iter().map(|p| text(&p["material"])).collect();
    let expected_material_indices: Vec<_> = materials
        .iter()
        .enumerate()
        .filter(|(_, m)| used.contains(text(&m["id"])))
        .map(|(i, _)| i as u64)
        .collect();
    fail(expected_material_indices.len() == used.len())?;
    let mut expected_selectors: Vec<Value> = Vec::new();
    let mut case_bases = Vec::new();
    for (i, case) in cases.iter().enumerate() {
        let value = selector(case)?;
        let index = expected_selectors
            .iter()
            .position(|s| *s == value)
            .unwrap_or_else(|| {
                expected_selectors.push(value.clone());
                expected_selectors.len() - 1
            });
        case_bases.push(index);
        let o = at(
            &b["ordinary_attempts"],
            &json!(i),
            "G8",
            "PREPARATION_MISMATCH",
        )?;
        fail(o["requested_mode"] == mode && u(&o["material_basis_ref"]) == index as u64)?;
        // F-1 text B (DESIGN_v2 §3.2-§3.3, decision 8): for every case, selected,
        // unavailable or not_required, since every case's rows come from the one
        // ordinary run. `o` is the case's own ordinary attempt (G3 binds
        // `ordinary.attempt_ref` to the case index).
        let rows = rows_for(source, &b["cases"][i]);
        let modes: Vec<_> = rows
            .iter()
            .filter(|r| r["kind"] == "linear_solver_mode_basis")
            .collect();
        // P1: exactly one mode row, valued with the mode code (1 sparse, 2 dense).
        fail(
            modes.len() == 1
                && modes[0]["value"].as_f64()
                    == Some(if mode == "sparse_interactive" {
                        1.0
                    } else {
                        2.0
                    }),
        )?;
        let parity = rows
            .iter()
            .filter(|r| r["kind"] == "sparse_live_path_dense_parity_relative_delta")
            .count();
        // P2: at most one parity row. P3: none in sparse_interactive. P4: none
        // when W2 published (b != 0; OQ5). A dense b = 0 case may lack it (a
        // failed observation lane); that deletion is the disclosed limit.
        fail(
            parity <= 1
                && (parity == 0 || mode == "dense_scrutiny")
                && (parity == 0 || o["w2"]["kind"] != "published"),
        )?;
    }
    fail(list(&b["material_bases"]).len() == expected_selectors.len())?;
    for (mi, mb) in list(&b["material_bases"]).iter().enumerate() {
        fail(
            u(&mb["index"]) == mi as u64
                && mb["selector"] == expected_selectors[mi]
                && mb["case_indices"]
                    == json!(case_bases
                        .iter()
                        .enumerate()
                        .filter(|(_, b)| **b == mi)
                        .map(|(i, _)| i)
                        .collect::<Vec<_>>()),
        )?;
        let ci = case_bases
            .iter()
            .position(|x| *x == mi)
            .ok_or_else(|| error("G8", "PREPARATION_MISMATCH"))?;
        fail(
            list(&mb["materials"])
                .iter()
                .map(|m| u(&m["input_index"]))
                .eq(expected_material_indices.iter().copied()),
        )?;
        for m in list(&mb["materials"]) {
            let raw = materials
                .get(u(&m["input_index"]) as usize)
                .ok_or_else(|| error("G8", "PREPARATION_MISMATCH"))?;
            let (eg, selection) = selected_material(raw, &cases[ci])?;
            fail(
                m["id"] == raw["id"]
                    && m["selection"] == selection
                    && m["shear_origin"] == json!({"kind":"explicit_g"})
                    && m["elastic_modulus"] == bits(eg[0])
                    && m["shear_modulus"] == bits(eg[1]),
            )?;
        }
    }
    // Rebuild common topology and the rigid/global-scalar-spring boundary.
    let mut parent: Vec<usize> = (0..nodes.len()).collect();
    fn root(p: &[usize], mut i: usize) -> usize {
        while p[i] != i {
            i = p[i];
        }
        i
    }
    for p in pipes {
        let i = find_node(nodes, &p["from"])?;
        let j = find_node(nodes, &p["to"])?;
        fail(i != j)?;
        let (a, z) = (root(&parent, i), root(&parent, j));
        parent[a.max(z)] = a.min(z);
    }
    let roots: BTreeSet<_> = (0..nodes.len()).map(|i| root(&parent, i)).collect();
    let mut bodies = Vec::new();
    for (bi, rt) in roots.into_iter().enumerate() {
        let ns: Vec<_> = (0..nodes.len())
            .filter(|i| root(&parent, *i) == rt)
            .collect();
        let ms: Vec<_> = pipes
            .iter()
            .enumerate()
            .filter(|(_, p)| find_node(nodes, &p["from"]).is_ok_and(|i| ns.contains(&i)))
            .map(|(i, _)| i)
            .collect();
        bodies.push(json!({"body":bi,"nodes":ns,"members":ms}));
    }
    let mut fixed: BTreeMap<(usize, usize), Vec<usize>> = BTreeMap::new();
    let mut springs = Vec::new();
    let mut support_groups = Vec::new();
    let mut support_ids = Vec::new();
    for (i, raw) in supports.iter().enumerate() {
        fail(
            raw["nonlinear"].is_null()
                && raw["hanger"].is_null()
                && raw["imposed_displacement"].is_null()
                && matches!(
                    text(&raw["family"]),
                    "" | "anchor" | "guide" | "line_stop" | "vertical_support" | "spring"
                ),
        )?;
        let n = find_node(nodes, &raw["node"])?;
        let mut restraint = [false; 6];
        let mut sid = Vec::new();
        let mut seen = BTreeSet::new();
        for d in list(&raw["restraints"]) {
            fail(seen.insert(dof_index(d)?))?;
        }
        if raw["family"] == "spring" {
            let q = &raw["stiffness"];
            let di = dof_index(&q["dof"])?;
            fail(seen == BTreeSet::from([di]))?;
            let k = unit_value(
                &q["value"],
                if di < 3 {
                    Dimension::LinearStiffness
                } else {
                    Dimension::RotationalStiffness
                },
            )?;
            fail(k > 0.0)?;
            let j = springs.len();
            sid.push(j);
            springs.push(json!({"boundary_index":j,"kernel_spring":j,"support_index":i,"node":n,"component":DOFS[di],"stiffness":bits(k)}));
        } else {
            fail(raw["stiffness"].is_null())?;
            for di in seen {
                restraint[di] = true;
                fail(fixed.insert((n, di), vec![i]).is_none())?;
            }
        }
        support_ids.push(json!({"model_index":i,"kernel_support":i,"id":raw["id"],"node":n}));
        support_groups.push(
            json!({"id":i,"node":n,"restrained":restraint,"springs":sid,"directional_springs":[]}),
        );
    }
    let constraints:Vec<_>=fixed.iter().map(|((n,d),ids)|json!({"dof":{"node":n,"component":DOFS[*d]},"value":"0000000000000000","support_indices":ids})).collect();
    let stations:Vec<_>=pipes.iter().enumerate().flat_map(|(i,_)|[("quarter_1",0.25),("midspan",0.5),("quarter_3",0.75)].into_iter().enumerate().map(move |(j,(location,fraction))|json!({"id":3*i+j,"member":i,"location":location,"fraction":bits(fraction)}))).collect();
    for (si, s) in list(&b["sources"]).iter().enumerate() {
        let ci = u(&s["owner"]["case_index"]) as usize;
        let case = cases
            .get(ci)
            .ok_or_else(|| error("G8", "PREPARATION_MISMATCH"))?;
        let maps = &s["id_maps"];
        fail(
            u(&s["index"]) == si as u64
                && s["owner"]["kind"] == "case"
                && s["owner"]["case_id"] == case["id"]
                && u(&s["material_basis_ref"]) == case_bases[ci] as u64,
        )?;
        fail(
            list(&case["pressure_regions"]).is_empty()
                && case["equivalent_static"].is_null()
                && !case
                    .as_object()
                    .is_some_and(|o| o.contains_key("analysis_state")),
        )?;
        fail(
            maps["nodes"] == json!(node_maps)
                && maps["support_ids"] == json!(support_ids)
                && maps["springs"] == json!(springs)
                && s["supports"] == json!(support_groups)
                && s["constraints"] == json!(constraints)
                && s["stations"] == json!(stations)
                && s["body_membership"] == json!(bodies),
        )?;
        fail(
            list(&maps["members"]).len() == pipes.len()
                && list(&s["section_terms"]).len() == pipes.len(),
        )?;
        let mb = &b["material_bases"][case_bases[ci]];
        let mut built = BTreeSet::new();
        for (i, (m, p)) in list(&maps["members"]).iter().zip(pipes).enumerate() {
            let section = &s["section_terms"][i];
            fail(
                u(&m["model_index"]) == i as u64
                    && u(&m["kernel_member"]) == i as u64
                    && u(&section["member"]) == i as u64
                    && m["id"] == p["id"]
                    && u(&m["built_pipe_index"]) < pipes.len() as u64
                    && built.insert(u(&m["built_pipe_index"])),
            )?;
            fail(
                u(&m["node_i"]) == find_node(nodes, &p["from"])? as u64
                    && u(&m["node_j"]) == find_node(nodes, &p["to"])? as u64,
            )?;
            fail(
                m["y_reference"]
                    == json!([
                        bits(p["y_reference"]["x"].as_f64().unwrap_or(f64::NAN)),
                        bits(p["y_reference"]["y"].as_f64().unwrap_or(f64::NAN)),
                        bits(p["y_reference"]["z"].as_f64().unwrap_or(f64::NAN))
                    ]),
            )?;
            let mat = list(&mb["materials"])
                .iter()
                .find(|x| x["input_index"] == m["material_index"])
                .ok_or_else(|| error("G8", "PREPARATION_MISMATCH"))?;
            fail(
                mat["id"] == p["material"]
                    && m["E"] == mat["elastic_modulus"]
                    && m["G"] == mat["shear_modulus"],
            )?;
            let d = unit_value(&p["section"]["outside_diameter"], Dimension::Length)?;
            let wall = unit_value(&p["section"]["wall_thickness"], Dimension::Length)?;
            let tol = if p["section"]["mill_tolerance"].is_null() {
                0.0
            } else {
                unit_value(&p["section"]["mill_tolerance"], Dimension::Length)?
            };
            let t = wall - tol;
            let geo = &section["geometry"];
            fail(
                d > 0.0
                    && t > 0.0
                    && t < d * 0.5
                    && geo["route"] == "preview"
                    && geo["normalized_od"] == bits(d)
                    && geo["effective_wall"] == bits(t)
                    && geo["actual_radius"] == bits(d * 0.5)
                    && section["area"] == m["A_K"]
                    && geo["actual_second_moment"] == m["Iy_K"]
                    && m["Iy_K"] == m["Iz_K"]
                    && geo["actual_polar_moment"] == m["J_K"],
            )?;
            for key in ["E", "G", "A_K", "Iy_K", "Iz_K", "J_K"] {
                fail(f(&m[key]) >= f64::MIN_POSITIVE)?;
            }
        }
        let mut terms = Vec::new();
        for (i, l) in list(&case["primitive_loads"]).iter().enumerate() {
            fail(
                l["target"]["type"] == "node"
                    && l["category"] != "thermal"
                    && matches!(text(&l["dimension"]), "force" | "moment"),
            )?;
            let node = find_node(nodes, &l["target"]["node"])?;
            let di = match text(&l["direction"]) {
                "global_x" | "UX" => 0,
                "global_y" | "UY" => 1,
                "global_z" | "UZ" => 2,
                "rotation_x" | "RX" => 3,
                "rotation_y" | "RY" => 4,
                "rotation_z" | "RZ" => 5,
                _ => return Err(error("G8", "PREPARATION_MISMATCH")),
            };
            fail((di < 3) == (l["dimension"] == "force"))?;
            let value = unit_value(
                &l["magnitude"],
                if di < 3 {
                    Dimension::Force
                } else {
                    Dimension::Moment
                },
            )?;
            terms.push(json!({"constructor_ordinal":i,"source_id":l["id"],"primitive_load_index":i,"dof":{"node":node,"component":DOFS[di]},"value":bits(value)}));
        }
        terms.sort_by(|a, z| {
            (
                u(&a["dof"]["node"]),
                DOFS.iter().position(|d| a["dof"]["component"] == *d),
                text(&a["source_id"]).as_bytes(),
                text(&a["value"]),
                u(&a["constructor_ordinal"]),
            )
                .cmp(&(
                    u(&z["dof"]["node"]),
                    DOFS.iter().position(|d| z["dof"]["component"] == *d),
                    text(&z["source_id"]).as_bytes(),
                    text(&z["value"]),
                    u(&z["constructor_ordinal"]),
                ))
        });
        fail(s["nodal_terms"] == json!(terms))?;
        let body_of = |n: usize| -> VResult<usize> {
            bodies
                .iter()
                .position(|b| list(&b["nodes"]).iter().any(|x| u(x) == n as u64))
                .ok_or_else(|| error("G8", "PREPARATION_MISMATCH"))
        };
        let mut layout = Vec::new();
        let mut add = |q: Value, k: &str, n: usize, input: bool| -> VResult {
            layout.push(json!({"index":layout.len(),"quantity":q,"kind":k,"body":body_of(n)?,"input_derived":input}));
            Ok(())
        };
        for n in 0..nodes.len() {
            for (di, d) in DOFS.iter().enumerate() {
                add(
                    json!({"tag":"displacement","dof":{"node":n,"component":d}}),
                    if di < 3 { "translation" } else { "rotation" },
                    n,
                    fixed.contains_key(&(n, di)),
                )?;
            }
        }
        for n in 0..nodes.len() {
            add(
                json!({"tag":"displacement_magnitude","node":n}),
                "translation",
                n,
                false,
            )?;
        }
        for m in list(&maps["members"]) {
            for end in ["i", "j"] {
                for (di, d) in DOFS.iter().enumerate() {
                    add(
                        json!({"tag":"end_action","member":m["kernel_member"],"end":end,"component":d}),
                        if di < 3 { "force" } else { "moment" },
                        u(&m["node_i"]) as usize,
                        false,
                    )?;
                }
            }
        }
        for st in &stations {
            let m = &maps["members"][u(&st["member"]) as usize];
            for (di, d) in DOFS.iter().enumerate() {
                add(
                    json!({"tag":"station_action","station":st["id"],"component":d}),
                    if di < 3 { "force" } else { "moment" },
                    u(&m["node_i"]) as usize,
                    false,
                )?;
            }
        }
        for sp in &springs {
            let di = dof_index(&sp["component"])?;
            add(
                json!({"tag":"spring_action","spring":sp["kernel_spring"],"component":sp["component"]}),
                if di < 3 { "force" } else { "moment" },
                u(&sp["node"]) as usize,
                false,
            )?;
        }
        for c in &constraints {
            let d = &c["dof"];
            let di = dof_index(&d["component"])?;
            add(
                json!({"tag":"reaction","dof":d}),
                if di < 3 { "force" } else { "moment" },
                u(&d["node"]) as usize,
                false,
            )?;
        }
        for sp in &support_groups {
            for (k, tag) in [
                ("force", "support_force_magnitude"),
                ("moment", "support_moment_magnitude"),
            ] {
                add(
                    json!({"tag":tag,"support":sp["id"]}),
                    k,
                    u(&sp["node"]) as usize,
                    false,
                )?;
            }
        }
        fail(s["layout"] == json!(layout))?;
        verify_native_source_hashes(s)?;
    }
    // Failed prefixes have no new source; their old/helper/new overlap still binds
    // to the independently normalized request, never to invented source entries.
    for a in list(&b["product_attempts"]) {
        let ci = u(&a["owner_ref"]["index"]) as usize;
        let case = cases
            .get(ci)
            .ok_or_else(|| error("G8", "PREPARATION_MISMATCH"))?;
        let mb = at(
            &b["material_bases"],
            &a["material_basis_ref"],
            "G8",
            "PREPARATION_MISMATCH",
        )?;
        fail(a["material_basis_ref"] == json!(case_bases[ci]))?;
        let old = list(&a["operational"]["old"]);
        let new = list(&a["operational"]["new"]);
        for op in old.iter().chain(new) {
            fail(u(&op["member"]) < pipes.len() as u64)?;
        }
        if a["operational"]["old_coverage"] == "complete" {
            fail(
                old.iter()
                    .map(|o| u(&o["member"]))
                    .eq(0..pipes.len() as u64),
            )?;
        }
        // F1:101-106 / C3:155-158 (P7 settlement, 06b): every attempt binds the
        // old tuple of each attached PreparedMember below; unattached old entries
        // (helpers not yet entered) remain producer attestations.
        for (j, pm) in list(&a["preparation"]["members"]).iter().enumerate() {
            let mi = u(&pm["member"]) as usize;
            let p = pipes
                .get(mi)
                .ok_or_else(|| error("G8", "PREPARATION_MISMATCH"))?;
            let mat = list(&mb["materials"])
                .iter()
                .find(|m| m["id"] == p["material"])
                .ok_or_else(|| error("G8", "PREPARATION_MISMATCH"))?;
            let os = list(&pm["old_source"]);
            let facts = list(&pm["old_facts"]);
            fail(
                os[0] == mat["elastic_modulus"]
                    && os[1] == mat["shear_modulus"]
                    && os[2] == facts[2]
                    && os[3] == facts[3]
                    && os[4] == facts[3]
                    && os[5] == facts[4],
            )?;
            let d = unit_value(&p["section"]["outside_diameter"], Dimension::Length)?;
            let wall = unit_value(&p["section"]["wall_thickness"], Dimension::Length)?;
            let tol = if p["section"]["mill_tolerance"].is_null() {
                0.0
            } else {
                unit_value(&p["section"]["mill_tolerance"], Dimension::Length)?
            };
            fail(facts[0] == bits(d) && facts[1] == bits(wall - tol))?;
            let ni = find_node(nodes, &p["from"])?;
            let nj = find_node(nodes, &p["to"])?;
            let mut positions = list(&node_maps[ni]["coordinates"]).to_vec();
            positions.extend_from_slice(list(&node_maps[nj]["coordinates"]));
            let mut old_inputs = positions.clone();
            for i in [0, 1, 2, 5] {
                old_inputs.push(os[i].clone());
            }
            fail(old[j]["inputs"] == json!(old_inputs))?;
            if let Some(op) = new.get(j) {
                let section = list(&pm["result"]["section"]);
                fail(section.len() == 5)?;
                let mut inputs = positions.clone();
                inputs.extend([
                    os[0].clone(),
                    os[1].clone(),
                    section[0].clone(),
                    section[2].clone(),
                ]);
                fail(op["inputs"] == json!(inputs))?;
                if !a["source_ref"].is_null() {
                    let s = at(
                        &b["sources"],
                        &a["source_ref"],
                        "G8",
                        "PREPARATION_MISMATCH",
                    )?;
                    let st = &s["section_terms"][mi];
                    let m = &s["id_maps"]["members"][mi];
                    fail(
                        pm["result"]["section"]
                            == json!([
                                st["area"],
                                st["geometry"]["actual_second_moment"],
                                st["geometry"]["actual_polar_moment"],
                                st["section_modulus"],
                                st["geometry"]["actual_radius"]
                            ])
                            && m["A_K"] == section[0]
                            && m["Iy_K"] == section[1]
                            && m["Iz_K"] == section[1]
                            && m["J_K"] == section[2],
                    )?;
                    if op["result"]["kind"] == "ready" {
                        for k in ["length", "axial_stiffness", "torsional_stiffness"] {
                            fail(op["result"][k] == st[k])?;
                        }
                    }
                }
                if op["result"]["kind"] == "ready" {
                    verify_new_operational(op)?;
                }
            }
        }
        let _ = case;
    }
    Ok(())
}
fn verify_new_operational(op: &Value) -> VResult {
    let fail = |ok| need(ok, "G8", "PREPARATION_MISMATCH");
    let x: Vec<f64> = list(&op["inputs"]).iter().map(f).collect();
    fail(x.len() == 10 && x[6..].iter().all(|x| *x > 0.0))?;
    let d = [x[3] - x[0], x[4] - x[1], x[5] - x[2]];
    fail(d.iter().all(|x| x.is_finite()))?;
    let squares = [d[0] * d[0], d[1] * d[1], d[2] * d[2]];
    fail(squares.iter().all(|x| x.is_finite()))?;
    let xy = squares[0] + squares[1];
    let xyz = xy + squares[2];
    let length = xyz.sqrt();
    fail(xy.is_finite() && xyz.is_finite() && length.is_finite() && length > 1e-12)?;
    let inv = 1.0 / length;
    let normalization = d.map(|d| d * inv);
    let axial_product = x[6] * x[8];
    let torsional_product = x[7] * x[9];
    let axial = axial_product / length;
    let torsion = torsional_product / length;
    fail(
        [axial_product, torsional_product, axial, torsion]
            .iter()
            .all(|v| v.is_normal()),
    )?;
    fail(
        op["result"]
            == json!({"kind":"ready","length":bits(length),"axial_stiffness":bits(axial),"torsional_stiffness":bits(torsion),"normalization":normalization.map(bits)}),
    )
}
fn verify_native_source_hashes(s: &Value) -> VResult {
    use sha2::{Digest, Sha256};
    fn put(out: &mut Vec<u8>, n: u64) -> VResult {
        let n = u32::try_from(n).map_err(|_| error("G8", "PREPARATION_MISMATCH"))?;
        out.extend(n.to_le_bytes());
        Ok(())
    }
    fn val(out: &mut Vec<u8>, v: &Value) -> VResult {
        out.extend(
            raw_bits(v)
                .ok_or_else(|| error("G8", "PREPARATION_MISMATCH"))?
                .to_le_bytes(),
        );
        Ok(())
    }
    fn dof(out: &mut Vec<u8>, d: &Value) -> VResult {
        put(out, u(&d["node"]))?;
        out.push(dof_index(&d["component"])? as u8);
        Ok(())
    }
    let maps = &s["id_maps"];
    for (source, expected) in [(true, "kernel_source_sha256"), (false, "stiffness_sha256")] {
        let mut out = if source {
            b"K4SRC\x01".to_vec()
        } else {
            b"K4STF\x01".to_vec()
        };
        put(&mut out, list(&maps["nodes"]).len() as u64)?;
        for n in list(&maps["nodes"]) {
            for v in list(&n["coordinates"]) {
                val(&mut out, v)?;
            }
        }
        put(&mut out, list(&maps["members"]).len() as u64)?;
        for m in list(&maps["members"]) {
            for k in ["kernel_member", "node_i", "node_j"] {
                put(&mut out, u(&m[k]))?;
            }
            for k in ["E", "G", "A_K", "Iy_K", "Iz_K", "J_K"] {
                val(&mut out, &m[k])?;
            }
            for v in list(&m["y_reference"]) {
                val(&mut out, v)?;
            }
        }
        put(&mut out, list(&maps["springs"]).len() as u64)?;
        for sp in list(&maps["springs"]) {
            put(&mut out, u(&sp["kernel_spring"]))?;
            dof(
                &mut out,
                &json!({"node":sp["node"],"component":sp["component"]}),
            )?;
            val(&mut out, &sp["stiffness"])?;
        }
        put(&mut out, 0)?;
        put(&mut out, list(&s["constraints"]).len() as u64)?;
        for c in list(&s["constraints"]) {
            dof(&mut out, &c["dof"])?;
            if source {
                val(&mut out, &c["value"])?;
            }
        }
        if source {
            put(&mut out, list(&s["nodal_terms"]).len() as u64)?;
            for t in list(&s["nodal_terms"]) {
                dof(&mut out, &t["dof"])?;
                let id = text(&t["source_id"]);
                put(&mut out, id.len() as u64)?;
                out.extend(id.as_bytes());
                val(&mut out, &t["value"])?;
            }
            put(&mut out, list(&s["stations"]).len() as u64)?;
            for st in list(&s["stations"]) {
                put(&mut out, u(&st["id"]))?;
                put(&mut out, u(&st["member"]))?;
                val(&mut out, &st["fraction"])?;
            }
            put(&mut out, list(&s["supports"]).len() as u64)?;
            for g in list(&s["supports"]) {
                put(&mut out, u(&g["id"]))?;
                put(&mut out, u(&g["node"]))?;
                for r in list(&g["restrained"]) {
                    out.push(u8::from(r == true));
                }
                put(&mut out, list(&g["springs"]).len() as u64)?;
                for sp in list(&g["springs"]) {
                    put(&mut out, u(sp))?;
                }
                put(&mut out, 0)?;
            }
        }
        need(
            s[expected] == format!("{:x}", Sha256::digest(&out)),
            "G8",
            "PREPARATION_MISMATCH",
        )?;
    }
    Ok(())
}
fn g5_ordinary(source: &Value) -> VResult {
    let b = &source["retained_precision"]["body"];
    let fail = |ok| need(ok, "G5", "ATTEMPT_MISMATCH");
    let ds = list(&source["diagnostics"]);
    let mut work_refs = BTreeSet::new();
    for (i, c) in list(&b["cases"]).iter().enumerate() {
        let o = &b["ordinary_attempts"][i];
        let cid = &c["basis_ref"]["ref_id"];
        let quality = &source["numerical_quality"]["cases"][i];
        let diag = |id: &Value| -> VResult<&Value> {
            ds.iter()
                .find(|d| d["id"] == *id && list(&d["affected_refs"]).contains(cid))
                .ok_or_else(|| error("G5", "ATTEMPT_MISMATCH"))
        };
        // D6a (checkpoint-A ruling): the untyped diagnostic_refs are unique and
        // resolve. Typed references below must be listed and name the case (C2:166).
        let mut refs = BTreeSet::new();
        for id in list(&o["diagnostic_refs"]) {
            fail(refs.insert(text(id)) && ds.iter().any(|d| d["id"] == *id))?;
        }
        // F5 (D-U6-7; decision 2, A2, RR:8821, amending checkpoint A's D6a,
        // RR:8117): the list is exactly the diagnostics whose affected_refs name
        // the case, once each, in envelope order, excluding RETAINED_PRECISION_*
        // (a T1 (a)-omitted disclosure is absent from the published envelope).
        let exact: Vec<&Value> = ds
            .iter()
            .filter(|d| list(&d["affected_refs"]).contains(cid) && !text(&d["code"]).starts_with("RETAINED_PRECISION_"))
            .map(|d| &d["id"])
            .collect();
        fail(list(&o["diagnostic_refs"]).iter().collect::<Vec<_>>() == exact)?;
        for path in [
            &o["initial"]["diagnostic_ref"],
            &o["initial"]["report_diagnostic_ref"],
            &o["w2"]["diagnostic_ref"],
            &o["w2"]["report_diagnostic_ref"],
            &o["formation"]["load_row_finding"]["diagnostic_ref"],
            &o["formation"]["d5_diagnostic_ref"],
            &o["legacy_source"]["diagnostic_ref"],
        ] {
            if !path.is_null() {
                diag(path)?;
                fail(refs.contains(text(path)))?;
            }
        }
        if matches!(text(&c["status"]), "selected" | "not_required") {
            fail(o["initial"]["kind"] != "not_attempted")?;
        }
        // B1 (DESIGN_v2 §3.3, decision 9; C1:101): a not_required case has no
        // product attempt, an attempted ordinary run (above) and the published
        // verdict checks_passed. Its initial need not be a Passed report, nor
        // its W2 untriggered: a W2-published case with that verdict is
        // not_required (T-4). PY `_g5_ordinary` and TS `ordinaryAttempts`
        // apply the same rule.
        if c["status"] == "not_required" {
            fail(c["product_attempt_ref"].is_null() && quality["solve_quality"] == "checks_passed")?;
        }
        // The report-outcome equality is kept; its W2 guard is equivalent
        // under D6c, since W2 runs only after a failed ordinary attempt.
        if o["initial"]["kind"] == "report" && o["w2"]["kind"] == "not_triggered" {
            fail(o["initial"]["outcome"] == quality["solve_quality"])?;
        }
        if o["w2"]["kind"] != "not_triggered" {
            let trigger = &o["w2"]["trigger"];
            if trigger["tag"] == "formation" {
                fail(
                    o["initial"]["kind"] == "formation_failure"
                        && trigger["error"] == o["initial"]["error"],
                )?;
            } else {
                fail(
                    o["initial"]["kind"] == "structural_failure"
                        && o["initial"]["error"]["tag"] == "range"
                        && trigger["error"] == o["initial"]["error"],
                )?;
            }
            if o["w2"]["kind"] == "published" {
                fail(
                    o["w2"]["force_scale_exponent"]
                        .as_f64()
                        .is_some_and(|n| n != 0.0),
                )?;
            }
        }
        if !o["legacy_source"]["work_ref"].is_null() {
            let wi = u(&o["legacy_source"]["work_ref"]);
            let w = at(
                &b["legacy_source_work"],
                &o["legacy_source"]["work_ref"],
                "G5",
                "ATTEMPT_MISMATCH",
            )?;
            fail(work_refs.insert(wi) && u(&w["case_index"]) == i as u64)?;
        }
        // O5 (C2:115-119; S06:51): a source_decline is an unavailable case with
        // no source or Run, owned by this case and its material basis.
        let decline = &c["source_decline"];
        if !decline.is_null() {
            let owner = &decline["input_owner"];
            let mb = at(
                &b["material_bases"],
                &owner["material_basis_ref"],
                "G5",
                "ATTEMPT_MISMATCH",
            )?;
            fail(
                c["status"] == "unavailable"
                    && c["source_ref"].is_null()
                    && c["run"].is_null()
                    && u(&owner["case_index"]) == i as u64
                    && owner["case_id"] == *cid
                    && list(&mb["case_indices"]).iter().any(|x| u(x) == i as u64),
            )?;
        }
        // D6b (C1:101; C2:164; checkpoint-A ruling): a selected case's ordinary
        // quality routes to retained precision: sensitive, unresolved or failed
        // (not checks_passed, not not_assessed).
        if c["status"] == "selected" {
            fail(matches!(
                text(&quality["solve_quality"]),
                "sensitive" | "unresolved" | "failed"
            ))?;
        }
        if c["status"] == "selected" {
            fail(
                c["selection"]["rcond_label"]
                    == "sensitivity to matrix-entry perturbation, not to authored parameters",
            )?;
        }
    }
    fail(work_refs.len() == list(&b["legacy_source_work"]).len())?;
    Ok(())
}
fn project(source: &Value, raw: bool) -> VResult<Value> {
    let mut projected = source.clone();
    projected
        .as_object_mut()
        .ok_or_else(|| error("G7", "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"))?
        .remove("retained_precision");
    projected["producer"]["semantic_contract_id"] =
        json!("openpipestress.result_semantics/0.3.0/preview-physics-1");
    projected["formulation_basis"]["profile_id"] = json!("product_preview_mechanics_v1");
    if raw {
        for r in projected["results"]
            .as_array_mut()
            .ok_or_else(|| error("G7", "SOURCE_PREVIEW_PHYSICS_ARRAY_INVALID"))?
        {
            if let Some(o) = r.as_object_mut() {
                o.remove("recovery_method");
            }
        }
    }
    Ok(projected)
}
// Eligibility is on (U7, D-U7-5): an invocation-bound statement of a solved
// model whose cases are selected or not_required is eligible; every gate runs.
const IMPLEMENTATION_COMPLETE: bool = true;
/// Validate a raw successor statement against the original request/mode.
/// Hashes bind the supplied statements; they do not establish producer origin.
pub fn validate(source: &Value, actual_invocation: Option<&Value>) -> VResult<Validation> {
    g0(source)?;
    g1(source, true)?;
    g2(source)?;
    let normalized = integral_receipt(source);
    let source: &Value = &normalized;
    g3(source, actual_invocation)?;
    g4(source)?;
    let body = &source["retained_precision"]["body"];
    g5_native(body)?;
    g5_ordinary(source)?;
    g5_products(source)?;
    let mut cases = numeric_cases(source)?;
    g5a(body, &cases)?;
    g5b(&mut cases)?;
    let classifications = g5c(&cases)?;
    for c in list(&body["cases"]) {
        for row in rows_for(source, c) {
            need(
                if c["status"] == "selected" {
                    row["recovery_method"] == METHOD
                } else {
                    row.get("recovery_method").is_none()
                },
                "G6",
                "ROW_METHOD_MISMATCH",
            )?;
        }
    }
    let projected = project(source, true)?;
    // C1 G7 "existing base failure codes" (06b settlement): each language
    // reports its own unchanged base code, here the Rust base's
    // SOURCE_PREVIEW_PHYSICS_<CODE>; any text after the code is detail.
    crate::semantic_contract::for_source(&projected).map_err(base_error)?;
    if let Some(inv) = actual_invocation {
        g8(source, inv)?;
    }
    let eligible = IMPLEMENTATION_COMPLETE
        && actual_invocation.is_some()
        && source["status"]["mechanics"] == "MECHANICS_SOLVED"
        && list(&body["cases"])
            .iter()
            .all(|c| matches!(text(&c["status"]), "selected" | "not_required"));
    Ok(Validation {
        invocation_bound: actual_invocation.is_some(),
        numerical_eligible: eligible,
        publication_sha256: text(&body["publication_sha256"]).into(),
        classifications,
    })
}
/// Metadata-only transport checks cannot reconstitute or verify omitted raw rows.
/// The publication digest is retained as a statement, never authenticated here.
pub fn validate_transport_metadata(source: &Value) -> VResult<Validation> {
    g0(source)?;
    g1(source, false)?;
    g2(source)?;
    let normalized = integral_receipt(source);
    let source: &Value = &normalized;
    let projected = project(source, false)?;
    crate::semantic_contract::for_source_metadata(&projected)
        .map_err(|code| ValidationError {
            gate: "G2",
            code,
            detail: None,
        })?;
    Ok(Validation {
        invocation_bound: false,
        numerical_eligible: false,
        publication_sha256: text(&source["retained_precision"]["body"]["publication_sha256"])
            .into(),
        classifications: Vec::new(),
    })
}

/// I61 U6e (snapshot 07g): reader-local pins for RV79-N1 (D37's expected table is
/// the corpus's, derived from native source, never this reader's) and RV80-N2
/// (`integral_receipt` touches only the receipt).
#[cfg(test)]
mod u6e_reader_round_tests {
    use super::*;
    fn corpus() -> Value {
        serde_json::from_str(include_str!("../../../../fixtures/results/retained_precision_cases.json")).unwrap()
    }

    #[test]
    fn d37_error_stages_matches_the_corpus_table_rv79_n1() {
        let table = corpus()["d37"].clone();
        let order: Vec<String> = table["stage_order"].as_array().unwrap().iter().map(|s| s.as_str().unwrap().to_owned()).collect();
        let pipeline: Vec<&str> = STAGE8.to_vec();
        assert_eq!(order[..8], pipeline[..], "the pipeline order");
        assert_eq!(order[8..], ["observables".to_owned(), "g5a".to_owned()]);
        let records: Vec<String> = table["records"].as_array().unwrap().iter().map(|r| r.as_str().unwrap().to_owned()).collect();
        assert_eq!(records.len(), 25);
        let stages = |r: &str| -> Value {
            Value::Object(order.iter().zip(r.chars()).map(|(k, m)| (k.clone(), table["marks"][m.to_string().as_str()].clone())).collect())
        };
        let mut kinds: Vec<String> = table["kinds"].as_object().unwrap().keys().cloned().collect();
        kinds.extend(table["unknown_kinds"].as_array().unwrap().iter().map(|k| k.as_str().unwrap().to_owned()));
        let mut misses = Vec::new();
        for kind in &kinds {
            let allowed = table["kinds"].get(kind.as_str()).and_then(Value::as_array).cloned().unwrap_or_default();
            for r in &records {
                let attempt = json!({"stages": stages(r), "result": {"kind": "unavailable", "error": {"kind": kind}}});
                if error_stages(&attempt) != allowed.contains(&json!(r)) {
                    misses.push(format!("{kind} {r}"));
                }
            }
        }
        assert!(misses.is_empty(), "{misses:?}");
    }

    #[test]
    fn rv80_n2_integral_receipt_touches_only_the_receipt() {
        let shared = corpus();
        let mut source = shared["cases"][0]["source"].clone();
        let charged = source["retained_precision"]["body"]["work"]["charged"].as_u64().unwrap();
        source["retained_precision"]["body"]["work"]["charged"] = json!(charged as f64);
        source["results"][1]["value"] = json!(17.0);
        source["diagnostics"][0]["probe"] = json!(3.0);
        let normalized = integral_receipt(&source);
        assert!(matches!(normalized, std::borrow::Cow::Owned(_)), "the receipt's integral float is normalized");
        assert!(normalized["retained_precision"]["body"]["work"]["charged"].is_u64());
        assert_eq!(normalized["retained_precision"]["body"]["work"]["charged"], json!(charged));
        for (key, value) in source.as_object().unwrap() {
            if key != "retained_precision" {
                assert_eq!(serde_json::to_string(&normalized[key.as_str()]).unwrap(), serde_json::to_string(value).unwrap(), "{key} is byte-identical");
            }
        }
        assert!(normalized["results"][1]["value"].is_f64(), "a row's 17.0 stays 17.0");
    }
}
