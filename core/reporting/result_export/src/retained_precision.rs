//! Standalone prepared-result statement checks; never proof of producer origin.
use open_pipe_stress_units::{canonical_unit, convert_for_dimension, unit_by_symbol, Dimension};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

pub const CONTRACT_ID: &str = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1";
pub const PROFILE: &str = "product_preview_retained_w1a_v2";
pub const DEFINITION_ID: &str = "RP-PREPARED-ORDINARY-DUAL-v1";
pub const DEFINITION_HASH: &str =
    "64ec69b842d6278323aa2ab2e140e999db52eb28e4a9965a567a90b04e228b41";
pub const METHOD: &str = "contribution_preserving_multiprecision_v1";
/// B2-C §3 and §8 (REVISION_02): DEF-C's id and H(`retained_precision_formation_v1`, DEF-C).
pub const COMBINATION_DEFINITION_ID: &str = "RP-PREPARED-COMBINATION-DUAL-v1";
pub const COMBINATION_DEFINITION_HASH: &str =
    "a9219849d4dfecfc4e9529c6b648eb21cdf2e9872f2d681cc0c26cdfd2bdf6f1";
/// B3b (B3-D §6 with REVISION_01): the exact successor `<physics-retained>`,
/// read on its own route (`EXACT`), whose base is pressure-2 (B7b D4 (a)).
pub const EXACT_CONTRACT_ID: &str = "openpipestress.result_semantics/0.3.0/physics-retained-1";
pub const EXACT_PROFILE: &str = "exact_straight_retained_w1a_v2";
pub const EXACT_DEFINITION_ID: &str = "RP-PREPARED-EXACT-DUAL-v1";
/// H(`retained_precision_formation_v1`, DEF-E) (REVISION_01 §1.2). T4-U6: DEF-E
/// states zero mill tolerance and lists the reduced-basis rows and both headlines.
pub const EXACT_DEFINITION_HASH: &str =
    "bd3c7de76f4ba2d1f7a72894c173e9c449e27f318359de648f912a192f7cb031";
const MAX_BITS: u64 = 0x7fef_ffff_ffff_ffff;
/// B4/D1 W1 extension (b7-plan "Failed-ordinary successor rulings" (4)): the requested-mode
/// basis row of a case whose ordinary attempt genuinely failed. It replaces
/// `linear_solver_mode_basis` there; the two never coexist in one case.
pub const REQUESTED_MODE_KIND: &str = "retained_precision_requested_mode_basis";
const REQUESTED_MODE_ID: &str = "result:retained-precision:requested-mode-basis";
const REQUESTED_MODE_COMPONENT: &str = "requested_solver_mode";
const REQUESTED_MODE_COORDINATE: &str = "captured_invocation";
const REQUESTED_MODE_SIGN: &str = "mode_code 1=sparse_interactive, 2=dense_scrutiny; the captured requested solver mode after a failed ordinary attempt, not an ordinary solve observation";

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
/// `hash` of the object `v` without its member `omit`, from the borrowed object (no clone; T4-S6).
fn hash_without(domain: &str, v: &Value, omit: &str, gate: &'static str, code: &str) -> VResult<String> {
    let object = v.as_object().ok_or_else(|| error(gate, code))?;
    crate::source_blocks::domain_hash_without(domain, object, omit).map_err(|_| error(gate, code))
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
/// DEF-C, packaged as DEF-O is (B2-C §8 row 4).
fn combination_definition() -> &'static Value {
    static S: OnceLock<Value> = OnceLock::new();
    S.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../../../../fixtures/results/retained_precision_prepared_combination_v1.json"
        ))
        .expect("packaged combination retained definition")
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
fn exact_definition() -> &'static Value {
    static S: OnceLock<Value> = OnceLock::new();
    S.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../../../../fixtures/results/retained_precision_prepared_exact_v1.json"
        ))
        .expect("packaged exact retained definition")
    })
}
fn exact_table() -> &'static Value {
    static S: OnceLock<Value> = OnceLock::new();
    S.get_or_init(|| {
        serde_json::from_slice(EXACT_TABLE_BYTES).expect("packaged exact retained table")
    })
}
/// B3-D §6.1: the route a statement is read on, decided once from
/// `producer.semantic_contract_id` (the exact identity reads exactly; every
/// other statement reads on the preview route, whose G0 refuses a foreign
/// identity). Each route's G0 table read, G1 preparation hash, G5b evidence
/// cross-check, G7 projection and G8 namespace and material checks sit behind
/// their own call edges (ruling 1).
#[derive(Debug)]
struct Route {
    exact: bool,
    /// The preparation payload's `definition_sha256` (S-1): the H of the
    /// definition the route's table binds.
    definition_hash: &'static str,
    /// G7's projection: the base identity and profile.
    base_id: &'static str,
    base_profile: &'static str,
    /// G8: each member's `section_terms[].geometry.route`.
    geometry: &'static str,
}
const PREVIEW: Route = Route {
    exact: false,
    definition_hash: DEFINITION_HASH,
    base_id: "openpipestress.result_semantics/0.3.0/preview-physics-2",
    base_profile: "product_preview_mechanics_v1",
    geometry: "preview",
};
const EXACT: Route = Route {
    exact: true,
    definition_hash: EXACT_DEFINITION_HASH,
    base_id: "openpipestress.result_semantics/0.3.0/pressure-2",
    base_profile: "exact_pressure_v3",
    geometry: "exact",
};
fn route(source: &Value) -> &'static Route {
    if source["producer"]["semantic_contract_id"] == EXACT_CONTRACT_ID {
        &EXACT
    } else {
        &PREVIEW
    }
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
        "safe_int" => v.as_f64().is_some_and(|n| n.is_finite() && n.fract()==0.0 && n.abs()<=SAFE as f64 && !(n==0.0 && n.is_sign_negative())),
        "limb64" => v.as_str().is_some_and(|s| s.len()==16 && s.bytes().all(|b|b.is_ascii_digit()||(b'a'..=b'f').contains(&b))),
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
/// S-1 (REVISION_01 §2): `definition_sha256` is the route's table-bound
/// definition H, never a module constant.
fn preparation_payload(a: &Value, definition_hash: &str) -> VResult<Value> {
    let mut members = Vec::new();
    for m in list(&a["preparation"]["members"]) {
        need(m["result"]["kind"] == "prepared", "G1", "RECEIPT_MISMATCH")?;
        members.push(json!({"member":m["member"],"old_source":m["old_source"],"old_facts":m["old_facts"],"section":m["result"]["section"]}));
    }
    Ok(
        json!({"definition_id":a["definition_id"],"definition_sha256":definition_hash,"owner_ref":a["owner_ref"],"ordinary_attempt_ref":a["ordinary_attempt_ref"],"material_basis_ref":a["material_basis_ref"],"members":members}),
    )
}
/// The bound inherited preview table and this successor table, as bytes.
const TABLE_BYTES: &[u8] = include_bytes!(
    "../../../../fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json"
);
const INHERITED_TABLE_BYTES: &[u8] =
    include_bytes!("../../../../fixtures/results/semantic_contract_v0_3_preview_physics_2.json");
const TABLE_HASH: &str = "d95e6acfe79f0e08d84fe7a6a71acbd18b38ff70809614227640ee0a0c16ff60";
/// The exact successor's table (XTABLE) and its inherited pressure-2 table.
const EXACT_TABLE_BYTES: &[u8] = include_bytes!(
    "../../../../fixtures/results/semantic_contract_v0_3_physics_retained_1.json"
);
const PRESSURE_TABLE_BYTES: &[u8] =
    include_bytes!("../../../../fixtures/results/semantic_contract_v0_3_pressure_2.json");
const EXACT_TABLE_HASH: &str = "92a364559e116f4e4a903ff114e1a530916b9761486bd2441aeb9c3711f290bb";
/// RV78-N1's policies (B3D-8): the receipt values XTABLE's `receipt_bindings`
/// binds from version 1, keyed by the receipt body's own paths.
fn receipt_bindings() -> Value {
    json!({"canonicalization":"openpipestress_jcs_ijson_v1","method":METHOD,
        "projection_policy":"RP-LOGICAL-ATTEMPTS-v1",
        "work":{"case_limit":20_000_000_000u64,"invocation_limit":60_000_000_000u64},
        "work_policy":"W1-LME-20B-60B-v1"})
}
/// B3-D §2.4 step 6: the table's `receipt_bindings`, `receipt_policy` and
/// `accuracy_classification.policy` each equal the reader's constants, so
/// neither the table nor a constant can drift alone.
fn table_binds_reader_constants(table: &Value) -> bool {
    table["receipt_bindings"] == receipt_bindings()
        && table["receipt_policy"] == "M03-INTEGRITY-MP-v2"
        && table["accuracy_classification"]["policy"] == "RP-FACADE-SI-v2"
}
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
    g0_with(source, table())
}
/// G0 on the preview route (B2-C §8, rows 1-9, in order), with the table as a
/// parameter: production passes the packaged PTABLE, and the reader-local unit
/// tests a test-only copy. Every row is SOURCE_PRODUCER_CONTRACT_UNSUPPORTED
/// except row 4, RETAINED_PRECISION_FORMATION_MISMATCH.
fn g0_with(source: &Value, t: &Value) -> VResult {
    let unsupported = |ok| need(ok, "G0", "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED");
    // 1-2. Identity and profile; producer component and schema versions.
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
    // 3. The table's identity and profile.
    unsupported(
        t["semantic_contract_id"] == CONTRACT_ID && t["formulation_profile_id"] == PROFILE,
    )?;
    // 4. The table's definitions are [DEF-O, DEF-C] in that order, and each
    // packaged definition's H equals its constant.
    need(
        hash(
            "retained_precision_formation_v1",
            definition(),
            "G0",
            "FORMATION_MISMATCH",
        )? == DEFINITION_HASH
            && hash(
                "retained_precision_formation_v1",
                combination_definition(),
                "G0",
                "FORMATION_MISMATCH",
            )? == COMBINATION_DEFINITION_HASH
            && t["product_formation_definitions"]
                == json!([{"id":DEFINITION_ID,"sha256":DEFINITION_HASH},
                    {"id":COMBINATION_DEFINITION_ID,"sha256":COMBINATION_DEFINITION_HASH}]),
        "G0",
        "FORMATION_MISMATCH",
    )?;
    // 5. The table's bytes and its inherited hash.
    unsupported(
        sha256_hex(TABLE_BYTES) == TABLE_HASH
            && t["inherited_semantic_contract_sha256"] == sha256_hex(INHERITED_TABLE_BYTES),
    )?;
    // 6. N-12's cross-check: the table against the reader's constants.
    unsupported(table_binds_reader_constants(t))?;
    let b = &source["retained_precision"]["body"];
    // 7. D32: integers by value (finite, integral, in range, not -0), never by
    // the JSON number's host type: `1.0` is the receipt version 1.
    unsupported(uint(&b["receipt_version"]) == Some(1))?;
    // 8. The bound receipt members against the table (row 6 binds the table
    // to the constants, so each is one check).
    let rb = &t["receipt_bindings"];
    for (key, want) in [
        ("policy", &t["receipt_policy"]),
        ("projection_policy", &rb["projection_policy"]),
        ("work_policy", &rb["work_policy"]),
        ("facade_policy", &t["accuracy_classification"]["policy"]),
        ("canonicalization", &rb["canonicalization"]),
    ] {
        unsupported(want.is_string() && b[key] == *want)?;
    }
    for key in ["case_limit", "invocation_limit"] {
        unsupported(
            rb["work"][key].as_u64().is_some() && uint(&b["work"][key]) == rb["work"][key].as_u64(),
        )?;
    }
    // 9. Every product attempt's and operand preparation's definition is one
    // of the table's (which owner carries which is G1's).
    let ids: Vec<&Value> = list(&t["product_formation_definitions"])
        .iter()
        .map(|d| &d["id"])
        .collect();
    for a in list(&b["product_attempts"])
        .iter()
        .chain(list(&b["operand_preparations"]))
    {
        if a.is_object() {
            unsupported(ids.contains(&&a["definition_id"]))?;
        }
    }
    Ok(())
}
/// G0 on the exact route (B3-D §2.4, steps 1-9, in order; REVISION_01 §4.3
/// entries 1-10): every step is SOURCE_PRODUCER_CONTRACT_UNSUPPORTED except
/// step 4, RETAINED_PRECISION_FORMATION_MISMATCH.
fn g0_exact(source: &Value) -> VResult {
    let unsupported = |ok| need(ok, "G0", "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED");
    let t = exact_table();
    // 1. Identity and profile.
    unsupported(
        source.is_object()
            && source["producer"]["semantic_contract_id"] == EXACT_CONTRACT_ID
            && source["formulation_basis"]["profile_id"] == EXACT_PROFILE,
    )?;
    // 2. Producer component and version; envelope schema 0.2.0.
    unsupported(
        source["schema_version"] == "0.2.0"
            && source["producer"]["component_name"] == "open_pipe_stress_product_physics"
            && source["producer"]["component_version"] == "0.2.0",
    )?;
    // 3. XTABLE's identity and profile.
    unsupported(
        t["semantic_contract_id"] == EXACT_CONTRACT_ID && t["formulation_profile_id"] == EXACT_PROFILE,
    )?;
    // 4. H(DEF-E) and the table's one bound definition.
    need(
        hash(
            "retained_precision_formation_v1",
            exact_definition(),
            "G0",
            "FORMATION_MISMATCH",
        )? == EXACT_DEFINITION_HASH
            && t["product_formation_definitions"]
                == json!([{"id":EXACT_DEFINITION_ID,"sha256":EXACT_DEFINITION_HASH}]),
        "G0",
        "FORMATION_MISMATCH",
    )?;
    // 5. XTABLE's bytes and its inherited pressure-2 hash.
    unsupported(
        sha256_hex(EXACT_TABLE_BYTES) == EXACT_TABLE_HASH
            && t["inherited_semantic_contract_sha256"] == sha256_hex(PRESSURE_TABLE_BYTES),
    )?;
    // 6. The table against the reader's constants.
    unsupported(table_binds_reader_constants(t))?;
    let b = &source["retained_precision"]["body"];
    // 7. receipt_version 1 (D32: by value).
    unsupported(uint(&b["receipt_version"]) == Some(1))?;
    // 8. The body's policies and limits equal the table's.
    let rb = &t["receipt_bindings"];
    for (key, want) in [
        ("policy", &t["receipt_policy"]),
        ("facade_policy", &t["accuracy_classification"]["policy"]),
        ("projection_policy", &rb["projection_policy"]),
        ("work_policy", &rb["work_policy"]),
        ("canonicalization", &rb["canonicalization"]),
    ] {
        unsupported(want.is_string() && b[key] == *want)?;
    }
    for key in ["case_limit", "invocation_limit"] {
        unsupported(rb["work"][key].as_u64().is_some() && uint(&b["work"][key]) == rb["work"][key].as_u64())?;
    }
    // 9. Every attempt names the exact definition.
    for a in list(&b["product_attempts"]) {
        if a.is_object() {
            unsupported(a["definition_id"] == EXACT_DEFINITION_ID)?;
        }
    }
    Ok(())
}
fn g1(source: &Value, raw: bool, route: &Route) -> VResult {
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
        need(
            hash_without(
                "retained_precision_publication_mp_v2",
                source,
                "retained_precision",
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
                            &preparation_payload(a, route.definition_hash)?,
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
    g1_combinations(&r["body"])
}
/// B2-C §10.1's G1 row (C3a-5; §2.7 "Hashes"), after the case hashes: each
/// operand-prepared CaseSource's preparation hash over its prepared record;
/// each CombinationSource operand's identity of the CaseSource it names; each
/// `retained_selected` combination's identity of its CombinationSource. A
/// reference that does not resolve (or an operand naming no CaseSource) is left
/// to G3/G5.
fn g1_combinations(body: &Value) -> VResult {
    let sources = list(&body["sources"]);
    let resolve = |v: &Value| uint(v).and_then(|i| sources.get(i as usize));
    for s in sources {
        if let Some(p) = uint(&s["preparation"]["operand_preparation_ref"])
            .and_then(|i| list(&body["operand_preparations"]).get(i as usize))
        {
            if p["result"]["kind"] == "prepared"
                && list(&p["preparation"]["members"])
                    .iter()
                    .all(|m| m["result"]["kind"] == "prepared")
            {
                need(
                    hash(
                        "retained_precision_operand_preparation_v1",
                        &operand_preparation_payload(p)?,
                        "G1",
                        "RECEIPT_MISMATCH",
                    )? == s["preparation"]["sha256"],
                    "G1",
                    "RECEIPT_MISMATCH",
                )?;
            }
        }
    }
    for s in sources.iter().filter(|s| s["owner"]["kind"] == "combination") {
        for o in list(&s["operands"]) {
            // The identity is a CaseSource's (§2.7); an operand naming any other
            // source is an association defect, left to G3/G5.
            if let Some(case_source) =
                resolve(&o["source_ref"]).filter(|x| x["owner"]["kind"] != "combination")
            {
                need(
                    source_hash(case_source)? == o["source_identity_sha256"],
                    "G1",
                    "RECEIPT_MISMATCH",
                )?;
            }
        }
    }
    for c in list(&body["combinations"]) {
        if let Some(h) = c.get("source_identity_sha256") {
            if let Some(s) = resolve(&c["source_ref"]) {
                need(source_hash(s)? == *h, "G1", "RECEIPT_MISMATCH")?;
            }
        }
    }
    Ok(())
}
/// C3a-5: the operand preparation payload, `{definition_id, definition_sha256,
/// owner_ref, ordinary_attempt_ref, material_basis_ref, purpose, members}`, with
/// DEF-O's H (an operand preparation is always DEF-O's; SCHEMA `const`).
fn operand_preparation_payload(p: &Value) -> VResult<Value> {
    let mut payload = preparation_payload(p, DEFINITION_HASH)?;
    payload["purpose"] = p["purpose"].clone();
    Ok(payload)
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
/// Current DEF-O/E/C H: only the four typed MPa stress recipes trace the SI
/// midpoint conversion. All other rows keep the actual raw conversion word.
fn projection_matches_row(row: &Value, outcome: &Value) -> bool {
    let value = match text(&outcome["kind"]) {
        "underflow" => 0.0,
        "normal" | "subnormal" => f(&outcome["value"]),
        _ => return false,
    };
    let stress_mpa = row["unit"] == "MPa" && matches!(text(&row["kind"]),
        "element_local_axial_normal_stress" | "element_local_bending_normal_stress_y"
        | "element_local_bending_normal_stress_z" | "element_local_torsional_shear_stress" | "pipe_elastic_normal_stress_maximum_v2");
    let published = if stress_mpa { value / 1_000_000.0 } else { value };
    published.is_finite() && row["value"].as_f64().is_some_and(|v| {
        v.to_bits() == (if published == 0.0 { 0.0 } else { published }).to_bits()
    })
}
/// T4-U6 (D-5, §2.3/§5): a reduced-section twin row. The exact successor
/// overlays it with its nominal twin's retained value; it binds no receipt
/// quantity (the producer's `reduced_twin`), so it is outside the receipt's row
/// universe. No preview row names a section basis. T4-U4b (§4.1): a
/// derived row (σ⁺/σ⁻, τ_v,max) of either basis is a not-covered row at its
/// own position, never a twin.
fn reduced_twin(row: &Value) -> bool {
    !derived_kind(row)
        && row["metadata"]["basis"].as_str().is_some_and(|b| {
            b.ends_with(&format!("{}{}", crate::section_basis::TOKEN_SEPARATOR, crate::section_basis::REDUCED))
        })
}
/// T4-U4b (DEL-04-07 Design u4b-u5b-implementation.md §4.1): the two
/// straight-member derived kinds a W1 successor carries as recipe-free
/// `not_covered` rows, formed from its own published rows (the base read of
/// G7 recomputes them); never projected, never verified.
fn derived_kind(row: &Value) -> bool {
    matches!(row["kind"].as_str(), Some("element_local_normal_stress_extreme" | "element_local_transverse_shear_stress_maximum"))
}
/// GLOBAL_END_ROWS_NOT_COVERED (W-7, DEL-04-01 Design `global-end-actions.md`): a W1
/// successor carries its members' global end rows as recipe-free `not_covered` rows, each
/// the rotation of its member's own successor end rows (the base read checks them); never
/// projected, never verified.
fn global_end_kind(row: &Value) -> bool {
    row["kind"].as_str().is_some_and(crate::global_end_evidence::is_kind)
}
/// The receipt's row universe of a case or combination entry: its rows, less
/// the reduced-section twins (`rows_all_for` keeps them). O4 (DEL-04-08 Design
/// `o4-source-extension.md` §4.7): a reduced-basis row is dropped only when its
/// member's source section term has no `reduced`; a distinct-basis member's
/// reduced rows are receipt rows, in published order.
fn rows_for<'a>(source: &'a Value, case: &Value) -> Vec<&'a Value> {
    let distinct = distinct_members(source, case);
    rows_all_for(source, case)
        .into_iter()
        .filter(|r| !reduced_twin(r) || distinct.contains(text(&r["entity_ref"])))
        .collect()
}
/// O4 §4.7: the member ids whose section term in the entry's own CaseSource
/// (`source_ref`, bound by G1 and G2) carries `reduced`. Empty for an entry
/// without a source and on every document without distinct bases.
fn distinct_members<'a>(source: &'a Value, case: &Value) -> BTreeSet<&'a str> {
    let Some(i) = uint(&case["source_ref"]) else { return BTreeSet::new() };
    distinct_members_of(&source["retained_precision"]["body"]["sources"][i as usize])
}
/// O4 §4.7: the member ids whose section term in CaseSource `s` carries `reduced`.
fn distinct_members_of(s: &Value) -> BTreeSet<&str> {
    let mut out = BTreeSet::new();
    for st in list(&s["section_terms"]).iter().filter(|st| st.get("reduced").is_some()) {
        if let Some(m) = list(&s["id_maps"]["members"]).iter().find(|m| m["kernel_member"] == st["member"]) {
            out.insert(text(&m["id"]));
        }
    }
    out
}
/// Every published row of a case or combination entry, reduced twins included.
fn rows_all_for<'a>(source: &'a Value, case: &Value) -> Vec<&'a Value> {
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
    // B2-C §10.1 G3 (a), (b), (e) and (f) (REVISION_01 N-3 branch points 1 and 2):
    // the combination entries follow the gate evidence one for one; one id set
    // for cases and combinations; a combination attempt and Run exactly for an
    // entry with a Run, each naming its entry. (a) binds once the receipt has a
    // combination entry or an `operand_preparations` member (PR-B2 ruling 1;
    // REVISION_01 §4.1: today's predicate at z = 0); with neither, gate evidence
    // naming no entry is the base reader's (07n `t_gate_ok`).
    let combos = list(&b["combinations"]);
    let gates = list(&source["contract_evidence"]["combination_gates"]);
    let b2 = !combos.is_empty() || b.get("operand_preparations").is_some();
    fail(!b2 || combos.len() == gates.len())?;
    let mut combination_ids = BTreeSet::new();
    let mut combination_runs = Vec::new();
    for (j, e) in combos.iter().enumerate() {
        let id = text(&e["basis_ref"]["ref_id"]);
        fail(
            e["basis_ref"]["ref_id"] == gates[j]["combination_id"]
                && !ids.contains(id)
                && combination_ids.insert(id)
                && e["product_attempt_ref"].is_null() == e["run"].is_null(),
        )?;
        if !e["product_attempt_ref"].is_null() {
            let ai = u(&e["product_attempt_ref"]);
            fail(refs.insert(ai))?;
            let a = at(
                &b["product_attempts"],
                &e["product_attempt_ref"],
                "G3",
                "COVERAGE_MISMATCH",
            )?;
            fail(u(&a["id"]) == ai && a["owner_ref"] == json!({"kind":"combination","index":j}))?;
        }
        if let Some(r) = e.get("run").filter(|r| !r.is_null()) {
            combination_runs.push(u(&r["id"]));
            fail(
                runs.insert(u(&r["id"]), json!({"kind":"combination","index":j}))
                    .is_none(),
            )?;
        }
    }
    // (e) Case attempts first, then combination attempts.
    let attempt_kinds: Vec<bool> = list(&b["product_attempts"])
        .iter()
        .map(|a| a["owner_ref"]["kind"] == "combination")
        .collect();
    fail(attempt_kinds.windows(2).all(|w| w[0] <= w[1]))?;
    // (f) The case Runs, then the combination Runs in Call (authored) order.
    fail(
        combination_runs.windows(2).all(|w| w[0] < w[1])
            && runs
                .iter()
                .filter(|(_, o)| o["kind"] == "case")
                .all(|(id, _)| combination_runs.iter().all(|c| id < c)),
    )?;
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
    // (c), N-3 branch point 5: every row's basis names a case or a combination
    // entry.
    let mut rowids = BTreeSet::new();
    for r in list(&source["results"]) {
        let named = text(&r["basis_ref"]["ref_id"]);
        fail(
            rowids.insert(text(&r["id"]))
                && match text(&r["basis_ref"]["ref_type"]) {
                    "load_case" => ids.contains(named),
                    "combination" => combination_ids.contains(named),
                    _ => false,
                },
        )?;
    }
    // (c): a combination's `result_ids` are exactly its rows, in publication order.
    for e in combos {
        fail(
            list(&e["result_ids"])
                .iter()
                .eq(rows_for(source, e).iter().map(|r| &r["id"])),
        )?;
    }
    // D29 (source.rs `PrimitiveSource::new` NoNodes; I57 s1): every CaseSource has a
    // non-empty body inventory. B1's alignment set, item 1 (f): a source's index is
    // its position and its owner is its own case (the case at `owner.case_index`
    // has `owner.case_id`); a material basis's index is its position and its
    // `case_indices` are unique and name cases. These facts are internal to the
    // receipt, so they are checked bound and unbound; G8 keeps the facts derived
    // from the invocation.
    for (si, s) in list(&b["sources"]).iter().enumerate() {
        let owner = &s["owner"];
        // N-3 branch point 4: a CombinationSource's index is its position and its
        // owner is the combination entry at `owner.combination_index`.
        if owner["kind"] == "combination" {
            fail(
                u(&s["index"]) == si as u64
                    && usize::try_from(u(&owner["combination_index"]))
                        .ok()
                        .and_then(|j| combos.get(j))
                        .is_some_and(|e| e["basis_ref"]["ref_id"] == owner["combination_id"]),
            )?;
            continue;
        }
        fail(
            !list(&s["body_membership"]).is_empty()
                && u(&s["index"]) == si as u64
                && owner["kind"] == "case"
                && usize::try_from(u(&owner["case_index"]))
                    .ok()
                    .and_then(|ci| cs.get(ci))
                    .is_some_and(|c| c["basis_ref"]["ref_id"] == owner["case_id"]),
        )?;
    }
    for (mi, mb) in list(&b["material_bases"]).iter().enumerate() {
        let mut seen = BTreeSet::new();
        fail(
            u(&mb["index"]) == mi as u64
                && list(&mb["case_indices"])
                    .iter()
                    .all(|ci| u(ci) < cs.len() as u64 && seen.insert(u(ci))),
        )?;
    }
    for (i, a) in list(&b["product_attempts"]).iter().enumerate() {
        fail(u(&a["id"]) == i as u64)?;
        // N-3 branch point 3: a combination attempt is owned by its entry and
        // indexes that entry's rows; its summary coverage is checked against
        // operand 0's CaseSource body inventory (a CombinationSource has none).
        // It has no operational or preparation members.
        if a["owner_ref"]["kind"] == "combination" {
            let e = at(&b["combinations"], &a["owner_ref"]["index"], "G3", "COVERAGE_MISMATCH")?;
            let operand0 = at(&b["sources"], &a["source_ref"], "G3", "COVERAGE_MISMATCH")
                .and_then(|s| at(&b["sources"], &s["operands"][0]["source_ref"], "G3", "COVERAGE_MISMATCH"));
            g3_attempt_rows(source, a, e, operand0)?;
            continue;
        }
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
                    .filter(|s| s["owner"]["kind"] != "combination")
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
        let resolved = at(&b["sources"], &a["source_ref"], "G3", "COVERAGE_MISMATCH");
        g3_attempt_rows(source, a, c, resolved)?;
    }
    g3_combinations(b)
}
/// The case ids a mechanics expression's terms name, in authored order (other
/// expressions name none for retention).
fn mechanics_terms(e: &Value) -> Vec<&Value> {
    if e["expression"]["kind"] == "mechanics" {
        list(&e["expression"]["terms"]).iter().map(|t| &t["case_id"]).collect()
    } else {
        Vec::new()
    }
}
/// C3a-1: a retained combination that requests operand preparations (its cause
/// is not `operand_source_unavailable`, which is decided first).
fn requests_preparations(e: &Value) -> bool {
    matches!(text(&e["disposition"]), "retained_selected" | "retained_unavailable")
        && e["reason"]["cause"]["kind"] != "operand_source_unavailable"
}
/// B2-C §10.1 G3 (d), (g), (h) and (i) (C3a-7's G3 row; REVISION_01 N-11 and
/// §4.3): operand preparations, one per `not_required` term case of a
/// requesting combination, in first-need order, each naming exactly its
/// requesting combinations, and a prepared one its own CaseSource; every
/// source named once; sources ordered batch, operand-prepared (record order),
/// then combination. On a receipt with no combination entry and no
/// `operand_preparations` member every predicate here is today's: (d), (h)
/// and (i) are vacuous, and (g) names no batch CaseSource (PR-B2 ruling 1).
fn g3_combinations(b: &Value) -> VResult {
    let fail = |ok| need(ok, "G3", "COVERAGE_MISMATCH");
    let cs = list(&b["cases"]);
    let combos = list(&b["combinations"]);
    let sources = list(&b["sources"]);
    let ops = list(&b["operand_preparations"]);
    fail(b.get("operand_preparations").is_none() || !ops.is_empty())?;
    let b2 = !combos.is_empty() || b.get("operand_preparations").is_some();
    let case_of = |id: &Value| cs.iter().position(|c| c["basis_ref"]["ref_id"] == *id);
    // (d) and (h): the not_required term cases of requesting combinations, in
    // first-need order (combinations, then terms, in authored order).
    let mut need_order = Vec::new();
    for e in combos.iter().filter(|e| requests_preparations(e)) {
        for t in mechanics_terms(e) {
            if let Some(ci) = case_of(t) {
                if cs[ci]["status"] == "not_required" && !need_order.contains(&ci) {
                    need_order.push(ci);
                }
            }
        }
    }
    for (i, p) in ops.iter().enumerate() {
        let ci = usize::try_from(u(&p["owner_ref"]["index"])).ok().filter(|ci| *ci < cs.len());
        fail(u(&p["id"]) == i as u64 && p["owner_ref"]["kind"] == "case" && ci.is_some())?;
        let ci = ci.unwrap_or_default();
        let owner = &cs[ci]["basis_ref"]["ref_id"];
        let requested: Vec<u64> = combos
            .iter()
            .enumerate()
            .filter(|(_, e)| requests_preparations(e) && mechanics_terms(e).contains(&owner))
            .map(|(j, _)| j as u64)
            .collect();
        fail(
            cs[ci]["status"] == "not_required"
                && list(&p["requested_by"]).iter().map(u).eq(requested.iter().copied()),
        )?;
        // A prepared record's source is its own operand-prepared CaseSource; a
        // refused record's `source_ref` is G5's (REVISION_01 §4.3).
        if p["result"]["kind"] == "prepared" {
            let s = at(&b["sources"], &p["source_ref"], "G3", "COVERAGE_MISMATCH")?;
            fail(
                s["owner"]["kind"] == "case"
                    && u(&s["owner"]["case_index"]) == ci as u64
                    && u(&s["preparation"]["operand_preparation_ref"]) == i as u64,
            )?;
        }
    }
    fail(
        ops.iter()
            .map(|p| u(&p["owner_ref"]["index"]))
            .eq(need_order.iter().map(|ci| *ci as u64)),
    )?;
    // (g) A CombinationSource is named by exactly one combination entry, and an
    // operand-prepared CaseSource by exactly one prepared operand preparation
    // and no case, always; a batch CaseSource by exactly one case or one
    // prepared operand preparation once the receipt has a combination entry or
    // an `operand_preparations` member (PR-B2 ruling 1).
    for (si, s) in sources.iter().enumerate() {
        let names = |v: &Value| uint(v) == Some(si as u64);
        let by_cases = cs.iter().filter(|c| names(&c["source_ref"])).count();
        let by_records = ops
            .iter()
            .filter(|p| p["result"]["kind"] == "prepared" && names(&p["source_ref"]))
            .count();
        let by_combinations = combos.iter().filter(|e| names(&e["source_ref"])).count();
        fail(if s["owner"]["kind"] == "combination" {
            by_combinations == 1
        } else if s["preparation"].get("operand_preparation_ref").is_some() {
            by_records == 1 && by_cases == 0
        } else {
            !b2 || by_cases + by_records == 1
        })?;
    }
    // (i) Batch CaseSources, then the operand-prepared ones in record order,
    // then the CombinationSources.
    let class = |s: &Value| {
        if s["owner"]["kind"] == "combination" {
            2
        } else if s["preparation"].get("operand_preparation_ref").is_some() {
            1
        } else {
            0
        }
    };
    let prepared: Vec<u64> = sources
        .iter()
        .filter(|s| class(s) == 1)
        .map(|s| u(&s["preparation"]["operand_preparation_ref"]))
        .collect();
    fail(
        sources.windows(2).all(|w| class(&w[0]) <= class(&w[1]))
            && prepared.windows(2).all(|w| w[0] < w[1]),
    )
}
/// G3's per-attempt row checks, for a case attempt (its own source) and a
/// combination attempt (operand 0's CaseSource).
fn g3_attempt_rows(source: &Value, a: &Value, owner: &Value, inventory_source: VResult<&Value>) -> VResult {
    let fail = |ok| need(ok, "G3", "COVERAGE_MISMATCH");
    let rows = rows_for(source, owner);
    let mut prior = None;
    for o in list(&a["proof"]["projection_outcomes"]) {
        let idx = u(&o["row_index"]);
        // C3 G3 row-index coverage (ROOT ruling): an index must name a
        // hull-projected row of this owner, ascending and unique.
        fail(
            idx < rows.len() as u64
                && prior.is_none_or(|p| idx > p)
                && owner_projected(owner, rows[idx as usize], inventory_source.as_ref().ok().copied(), !route(source).exact),
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
    if let (false, Ok(s)) = (coverage.is_null(), inventory_source) {
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
    Ok(())
}
const SELECTED_MEMBER_LOAD_MESSAGE: &str = "Retained-precision recovery (contribution_preserving_multiprecision_v1) is selected for this load case. Its full-span uniform member loads are represented by declared equivalent nodal loads, each entry within 2^-1022 of its exact consistent value relative to that value; the certificate certifies that declared-load problem. Its published rows carry recovery_method; the retained_precision receipt binds their certified classes, the native attempts and work, and the ordinary-attempt evidence.";
fn g4(source: &Value) -> VResult {
    let cs = list(&source["retained_precision"]["body"]["cases"]);
    let combos = list(&source["retained_precision"]["body"]["combinations"]);
    let ds = list(&source["diagnostics"]);
    let fail = |ok| need(ok, "G4", "DIAGNOSTIC_MISMATCH");
    let mut ids = BTreeSet::new();
    for d in ds {
        fail(
            d["id"].is_string()
                && ids.insert(text(&d["id"]))
                && d["code"] != "SOURCE_BLOCK_RECOVERY_SELECTED",
        )?;
        // N-3 branch point 6: one case or one combination entry.
        if matches!(
            text(&d["code"]),
            "RETAINED_PRECISION_SELECTED" | "RETAINED_PRECISION_UNAVAILABLE"
        ) {
            fail(
                list(&d["affected_refs"]).len() == 1
                    && cs
                        .iter()
                        .chain(combos)
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
            let member_case = uint(&c["source_ref"]).and_then(|i| list(&source["retained_precision"]["body"]["sources"]).get(i as usize))
                .is_some_and(|s| !list(&s["member_load_terms"]).is_empty());
            fail((selected[0]["message"] == SELECTED_MEMBER_LOAD_MESSAGE) == member_case)?;
            fail(!ds.iter().any(|d| {
                d["code"] == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE"
                    && list(&d["affected_refs"]).contains(id)
            }))?;
        }
    }
    // B2-C §10.1 G4: one RETAINED_PRECISION_SELECTED per `retained_selected` and
    // one RETAINED_PRECISION_UNAVAILABLE per `retained_unavailable` combination,
    // none for `ordinary` or `base_withheld`; an unavailable entry's
    // `diagnostic_ref` is its diagnostic's id.
    for e in combos {
        let id = &e["basis_ref"]["ref_id"];
        let named = |code: &str| -> Vec<&Value> {
            ds.iter()
                .filter(|d| d["code"] == code && list(&d["affected_refs"]).contains(id))
                .collect()
        };
        let (selected, unavailable) = (
            named("RETAINED_PRECISION_SELECTED"),
            named("RETAINED_PRECISION_UNAVAILABLE"),
        );
        fail(
            selected.len() == usize::from(e["disposition"] == "retained_selected")
                && unavailable.len() == usize::from(e["disposition"] == "retained_unavailable"),
        )?;
        if e["disposition"] == "retained_unavailable" {
            fail(unavailable[0]["id"] == e["diagnostic_ref"])?;
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
    // N-3 branch point 7: the cases' and the combination entries' Runs, each
    // with its owner reference, indexed by Run id.
    let mut runs: Vec<(Value, &Value, &Value)> = list(&b["cases"])
        .iter()
        .enumerate()
        .filter_map(|(ci, c)| {
            c.get("run")
                .filter(|r| !r.is_null())
                .map(|r| (json!({"kind":"case","index":ci}), c, r))
        })
        .chain(list(&b["combinations"]).iter().enumerate().filter_map(|(j, e)| {
            e.get("run")
                .filter(|r| !r.is_null())
                .map(|r| (json!({"kind":"combination","index":j}), e, r))
        }))
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
        // B2-C §10.1 G5 native class (N-3 branch point 10): a combination Call's
        // Group is a CombinationGroup, whose cache starts as its imports; a
        // batch's is a Group.
        af(g.get("imports").is_some() == (call["kind"] == "mechanics_combination"))?;
        group_caches.insert(
            i as u64,
            list(&g["imports"])
                .iter()
                .map(|x| (text(&x["slot"]).to_owned(), u(&x["build"])))
                .collect(),
        );
    }
    for (i, build) in list(&b["builds"]).iter().enumerate() {
        wf(u(&build["id"]) == i as u64 && u(&build["work"]) == tw(stages_sum(&build["stages"])))?;
    }
    for (call_id, call) in list(&b["calls"]).iter().enumerate() {
        let mechanics = call["kind"] == "mechanics_combination";
        // N-3 branch point 8: a mechanics Call has one owner, and one source and
        // one Run (`runs`) or neither (`pre_source_refusal`).
        af(u(&call["id"]) == call_id as u64
            && list(&call["run_refs"]).len() == list(&call["source_refs"]).len()
            && if mechanics {
                list(&call["owner_refs"]).len() == 1 && list(&call["run_refs"]).len() <= 1
            } else {
                list(&call["run_refs"]).len() == list(&call["owner_refs"]).len()
            })?;
        wf(u(&call["invocation_before"]) == current)?;
        for (position, ri) in list(&call["run_refs"]).iter().enumerate() {
            let (owner, c, r) = runs
                .get(u(ri) as usize)
                .cloned()
                .ok_or_else(|| error("G5", "ATTEMPT_MISMATCH"))?;
            run_order.push(u(ri));
            let si = &call["source_refs"][position];
            let oi = &call["owner_refs"][position];
            // N-3 branch point 9: the Run's origin names its own owner (a case,
            // or a combination entry on a mechanics Call) and its source.
            af(r["origin"]
                == json!({"call":call_id,"position":position,"group":r["origin"]["group"],"source_ref":si,"owner_ref":oi})
                && *oi == owner
                && (owner["kind"] == "combination") == mechanics
                && c["source_ref"] == *si)?;
            // The source's owner id is bound to its case index at G3 (item 1 (f)),
            // a CombinationSource's to its combination index.
            let src = at(&b["sources"], si, "G5", "ATTEMPT_MISMATCH")?;
            af(if mechanics {
                src["owner"]["kind"] == "combination"
                    && src["owner"]["combination_index"] == oi["index"]
            } else {
                src["owner"]["case_index"] == oi["index"]
            })?;
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
            let mut imported = BTreeSet::new();
            let mut cache = if let Some(gid) = gid {
                let g = at(&b["groups"], &json!(gid), "G5", "ATTEMPT_MISMATCH")?;
                af(u(&g["call"]) == call_id as u64 && list(&g["source_refs"]).contains(si))?;
                imported = list(&g["imports"])
                    .iter()
                    .map(|x| (text(&x["slot"]).to_owned(), u(&x["build"])))
                    .collect();
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
            // A combination Run's `cache_before` is exactly its Group's imports.
            if mechanics {
                af(r["cache_before"] == snapshot(&imported.iter().cloned().collect()))?;
            }
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
                    // A combination Run's layout is its representative's (operand 0's
                    // CaseSource: the kernel source is that operand's, KD I5).
                    let mut layout_source = at(&b["sources"], si, "G5", "ATTEMPT_MISMATCH")?;
                    if mechanics {
                        layout_source = at(&b["sources"], &layout_source["representative_source_ref"], "G5", "ATTEMPT_MISMATCH")?;
                    }
                    let layout = list(&layout_source["layout"]);
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
                    // An imported build belongs to its operand's group.
                    wf((build["group"] == r["origin"]["group"]
                        || (!built && imported.contains(&(slot.clone(), bi))))
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
                if c["status"] == "selected" || c["disposition"] == "retained_selected" {
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
    g5_native_combinations(b)?;
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
                .cloned()
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
/// The source an operand term names (B2-C §2.5): a `selected` or `unavailable`
/// case's own CaseSource, or a `not_required` case's operand-preparation source.
fn operand_source<'a>(b: &'a Value, case: &'a Value, ci: usize) -> &'a Value {
    if case["status"] == "not_required" {
        list(&b["operand_preparations"])
            .iter()
            .find(|p| u(&p["owner_ref"]["index"]) == ci as u64)
            .map_or(&Value::Null, |p| &p["source_ref"])
    } else {
        &case["source_ref"]
    }
}
/// B2-C §10.1 G5 native class, ATTEMPT (after today's checks): `calls[0]` is the
/// case batch, then one mechanics Call per entry with a `call_ref`, in authored
/// order, naming its entry; `requested_operands` are the expression's terms
/// with their sources (§2.5); `runs` has one source and one Run, which are the
/// entry's, `pre_source_refusal` neither (REVISION_01 §4.1 row 8: the Call's
/// arrays; a refused entry's null members are G5 products' reason table); the
/// CombinationSource's operands, representative, stiffness and ledger; the
/// Group's imports from `selected` operands only, the first occupied slot in
/// authored order, from each operand's selected Run (`cache_after`). At z = 0
/// nothing is added.
fn g5_native_combinations(b: &Value) -> VResult {
    let af = |ok| need(ok, "G5", "ATTEMPT_MISMATCH");
    let calls = list(&b["calls"]);
    let cs = list(&b["cases"]);
    let combos = list(&b["combinations"]);
    if combos.is_empty() {
        return Ok(());
    }
    let with_calls: Vec<(usize, &Value)> =
        combos.iter().enumerate().filter(|(_, e)| !e["call_ref"].is_null()).collect();
    af(calls.first().is_some_and(|c| c["kind"] == "case_batch")
        && calls.len() == with_calls.len() + 1
        && calls[1..].iter().all(|c| c["kind"] == "mechanics_combination"))?;
    for (k, (j, e)) in with_calls.into_iter().enumerate() {
        let call = &calls[k + 1];
        af(u(&e["call_ref"]) == (k + 1) as u64
            && call["owner_refs"] == json!([{"kind":"combination","index":j}]))?;
        let terms = list(&e["expression"]["terms"]);
        let requested = list(&call["requested_operands"]);
        af(e["expression"]["kind"] == "mechanics" && requested.len() == terms.len())?;
        let mut term_cases = Vec::new();
        for (t, o) in terms.iter().zip(requested) {
            let ci = cs
                .iter()
                .position(|c| c["basis_ref"]["ref_id"] == t["case_id"])
                .ok_or_else(|| error("G5", "ATTEMPT_MISMATCH"))?;
            af(o["factor"] == t["factor"] && o["source_ref"] == *operand_source(b, &cs[ci], ci))?;
            term_cases.push(ci);
        }
        let runs = call["result"]["kind"] == "runs";
        af(runs == (list(&call["run_refs"]).len() == 1 && list(&call["source_refs"]).len() == 1)
            && (call["result"]["kind"] == "pre_source_refusal") == list(&call["run_refs"]).is_empty()
            && (call["result"]["kind"] == "pre_source_refusal") == list(&call["source_refs"]).is_empty())?;
        if !runs {
            continue;
        }
        af(e["run"]["id"] == call["run_refs"][0] && e["source_ref"] == call["source_refs"][0])?;
        let s = at(&b["sources"], &e["source_ref"], "G5", "ATTEMPT_MISMATCH")?;
        let operands = list(&s["operands"]);
        af(s["owner"]["kind"] == "combination"
            && operands.len() == terms.len()
            && s["representative_source_ref"] == operands[0]["source_ref"])?;
        // C2 §3: each operand's case and factor are its term's (as PY and TS
        // check them), and its factor and source are its requested operand's.
        for (((o, r), ci), t) in operands.iter().zip(requested).zip(&term_cases).zip(terms) {
            let case_source = at(&b["sources"], &o["source_ref"], "G5", "ATTEMPT_MISMATCH")?;
            af(u(&o["case_index"]) == *ci as u64
                && o["factor"] == t["factor"]
                && o["factor"] == r["factor"]
                && o["source_ref"] == r["source_ref"]
                && case_source["owner"]["kind"] == "case"
                && u(&case_source["owner"]["case_index"]) == *ci as u64
                && case_source["stiffness_sha256"] == s["stiffness_sha256"])?;
        }
        if e["disposition"] == "retained_selected" {
            af(s["ledger_sha256"] == e["selection"]["ledger_sha256"])?;
        }
        // The Group's imports: per slot, in fixed slot order, the first operand
        // in authored order whose case is `selected` and whose selected Run's
        // `cache_after` holds the slot.
        let g = at(&b["groups"], &e["run"]["origin"]["group"], "G5", "ATTEMPT_MISMATCH")?;
        let mut expected = Vec::new();
        for slot in SLOTS {
            for (oi, ci) in term_cases.iter().enumerate() {
                let case = &cs[*ci];
                if case["status"] != "selected" {
                    continue;
                }
                if let Some(x) = list(&case["run"]["cache_after"]).iter().find(|x| x["slot"] == slot) {
                    expected.push(json!({"operand_index":oi,"selected_run":case["run"]["id"],"slot":slot,"build":x["build"]}));
                    break;
                }
            }
        }
        af(g["imports"] == json!(expected))?;
        for x in &expected {
            let build = at(&b["builds"], &x["build"], "G5", "ATTEMPT_MISMATCH")?;
            af(u(&build["origin"]["run"]) < u(&e["run"]["id"]))?;
        }
    }
    Ok(())
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
    /// Reader-owned D1–D5/K4LED only; grants no validation or standing.
    pub fn native_source_digest(source:&Value)->Result<String,ValidationError> {
        use sha2::{Digest,Sha256};
        Ok(format!("{:x}",Sha256::digest(super::native_bytes(source,true)?)))
    }
    pub fn invocation_binding(source:&Value,invocation:&Value)->Result<(),ValidationError> {
        super::g8(source,invocation,super::route(source))
    }
    pub const fn declared_layout_pairs() -> [(usize,usize);5] {
        let [integer,dyadic,declared]=crate::declared_member_load::layout_pairs();
        [integer,dyadic,declared,
         (std::mem::size_of::<super::MemberExpected>(),std::mem::align_of::<super::MemberExpected>()),
         (std::mem::size_of::<crate::declared_member_load::Recomputed>(),std::mem::align_of::<crate::declared_member_load::Recomputed>())]
    }
    pub const fn declared_layouts() -> [usize;5] {
        let [integer,dyadic,declared]=crate::declared_member_load::layouts();
        [integer,dyadic,declared,std::mem::size_of::<super::MemberExpected>(),std::mem::size_of::<crate::declared_member_load::Recomputed>()]
    }
    pub fn declared_problem(source: &Value) -> Result<(String, [bool;192]), ValidationError> {
        let result = crate::declared_member_load::recompute(source).map_err(|_| error("G8", "PREPARATION_MISMATCH"))?;
        Ok((result.digest, result.nonzero))
    }
    /// Complete new-domain source encoding/hash and member G8 staging path.
    /// Its borrowed pre-existing G8 maps are covered by the existing G8 owner.
    pub fn declared_workspace(source:&Value, case:&Value, pipes:&[Value], digest:&Value)->Result<(),ValidationError> {
        use sha2::{Digest,Sha256};
        for include_loads in [true,false] { let _=Sha256::digest(super::native_bytes(source,include_loads)?); }
        super::verify_member_terms(source,case,pipes,Some(digest),false)
    }

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
    /// B2-C §4's reason table for one `retained_unavailable` combination entry
    /// and its CombinationAttempt.
    pub fn combination_reason_table(entry: &Value, attempt: &Value) -> Result<(), ValidationError> {
        super::combination_reason_table(entry, attempt, &|ok| need(ok, "G5", "PRODUCT_ATTEMPT_MISMATCH"))
    }
    /// B2-C §10.1 G5 products: each `retained_unavailable` entry's cause, on a body.
    pub fn combination_causes(body: &Value) -> Result<(), ValidationError> {
        super::combination_causes(body, &|ok| need(ok, "G5", "PRODUCT_ATTEMPT_MISMATCH"))
    }
    /// R-COMB-1 (R): the classes appended after G5c, on a statement.
    pub fn r_comb_1(source: &Value) -> Vec<RowClassification> {
        super::r_comb_1(source)
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
    let owner = if (p(0), p(1), p(3)) == (Some("preparation"), Some("members"), Some("reduced")) {
        // O4 §4.6: a fault inside a member's reduced record is owned by that record's work.
        index(2).map(|i| &a["preparation"]["members"][i]["reduced"]["work"])
    } else if (p(0), p(1)) == (Some("preparation"), Some("members")) {
        index(2).map(|i| &a["preparation"]["members"][i]["work"])
    } else if (p(0), p(1), p(2)) == (Some("result"), Some("error"), Some("section")) {
        list(&a["preparation"]["members"]).last().map(|m| &m["work"])
    } else if (p(0), p(1)) == (Some("proof"), Some("lanes")) {
        index(2).map(|i| &proof["lanes"][i]["work"])
    } else if (p(0), p(1), p(2)) == (Some("result"), Some("error"), Some("cause"))
        && a["result"]["error"]["kind"] == "values"
    {
        Some(if proof["completion"]["kind"]=="merged" {proof}else{&proof["completion"]})
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
        // O4 §4.6: the same rule for a member's reduced record and its own work.
        for record in [m, &m["reduced"]] {
            if record.is_object() && record["result"]["kind"] != "prepared" && record["result"]["error"]["kind"] == "accounting" {
                r4 &= statuses(&record["work"]) != 0;
            }
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
/// C3's preparation-member checks for a product attempt or an operand
/// preparation (C3a-7's G5 rows): member conversions in property order, a
/// successful prefix then at most one refused member, and new operational
/// members only for prepared ones; conversion counts are WORK.
fn member_checks(a: &Value, pf: &dyn Fn(bool) -> VResult, wf: &mut dyn FnMut(bool) -> VResult) -> VResult {
    let pm = list(&a["preparation"]["members"]);
    let new = list(&a["operational"]["new"]);
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
        // O4 (DEL-04-08 Design `o4-source-extension.md` §4.6): a distinct-basis member's
        // reduced record is a preparation record like the member's own, read by the same
        // rules: present only beside a prepared member; conversions in property order,
        // each of an admitted kind; a prepared result's nine normal, positive conversions
        // equal to its section; a refused reduced annulus ends the preparation (the last
        // member); its conversion count equation is WORK.
        if let Some(r) = m.get("reduced") {
            pf(prepared)?;
            let reduced_prepared = r["result"]["kind"] == "prepared";
            if !reduced_prepared {
                pf(r["result"]["kind"] == "refused" && j + 1 == pm.len())?;
            }
            let rc = list(&r["conversions"]);
            pf(rc.len() <= 9)?;
            for (k, v) in rc.iter().enumerate() {
                pf(v["property"] == props[k].0
                    && v["endpoint"] == props[k].1
                    && conversion_kind_ok(&v["outcome"]))?;
                if reduced_prepared {
                    let idx = if k < 8 { k / 2 } else { 4 };
                    pf(v["outcome"]["kind"] == "normal"
                        && v["outcome"]["value"] == r["result"]["section"][idx]
                        && f(&v["outcome"]["value"]) >= f64::MIN_POSITIVE)?;
                }
            }
            if reduced_prepared {
                pf(rc.len() == 9)?;
            }
            if let Some(n) = exact_count(&r["work"]["conversions"]) {
                wf(n == rc.len() as u64)?;
            }
        }
    }
    for (j, op) in new.iter().enumerate() {
        pf(pm.get(j).is_some_and(|m| m["result"]["kind"] == "prepared" && op["member"] == m["member"]))?;
    }
    Ok(())
}
/// C3's proof-stage checks for one product attempt and its owner (a case, or
/// a combination entry: B2-C §10.1 G5 products, "as C3 without a preparation
/// stage"): stage progression, lane and check records, projection outcomes over
/// the owner's rows and, for a Ready attempt, the frozen row values.
/// The correction calls a completed or failed lane may record. DEF-O's lanes.congruence lets a
/// preview-route case's annular_source lane (index 1) take one further correction; every other
/// lane, the exact route (DEF-E) and combinations (DEF-C) keep at most one.
fn lane_correction_limit(source: &Value, a: &Value, lane: usize) -> u64 {
    if !route(source).exact && a["owner_ref"]["kind"] == "case" && lane == 1 {
        2
    } else {
        1
    }
}
fn proof_checks(
    source: &Value,
    a: &Value,
    c: &Value,
    pf: &dyn Fn(bool) -> VResult,
    wf: &mut dyn FnMut(bool) -> VResult,
) -> VResult {
    let st = &a["stages"];
    let proof = &a["proof"];
    loaded_maximum_diagnostic(source,a,c)?;
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
                wf(n <= lane_correction_limit(source, a, i))?;
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
            .filter(|(_, r)| owner_projected(c, r, uint(&a["source_ref"]).and_then(|i|list(&source["retained_precision"]["body"]["sources"]).get(i as usize)), !route(source).exact))
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
                .filter(|(_, r)| owner_projected(c, r, uint(&a["source_ref"]).and_then(|i|list(&source["retained_precision"]["body"]["sources"]).get(i as usize)), !route(source).exact))
                .map(|(i, _)| i as u64)
                .collect();
            pf(outcomes.iter().map(|v| u(&v["row_index"])).eq(expected))?;
            for v in outcomes {
                let row = rows[u(&v["row_index"]) as usize];
                let o = &v["outcome"];
                pf(o["kind"] != "overflow")?;
                pf(projection_matches_row(row, o))?;
            }
        }
    }
    Ok(())
}
/// Compare normalized binary1024 endpoints by leading exponent then limbs;
/// no exponent-dependent shift or heap integer can be requested by the wire.
fn diagnostic_endpoint(v:&Value)->VResult<(i64,[u64;16])> {
    let fail=|ok|need(ok,"G5","PRODUCT_ATTEMPT_MISMATCH");
    let e=v["exponent"].as_f64().filter(|n|n.is_finite()&&n.fract()==0.0&&n.abs()<=SAFE as f64).ok_or_else(||error("G5","PRODUCT_ATTEMPT_MISMATCH"))? as i64;
    let mut limbs=[0u64;16];fail(list(&v["limbs"]).len()==16)?;
    for (to,from) in limbs.iter_mut().zip(list(&v["limbs"])) {
        let word=text(from);fail(word.len()==16 && word.bytes().all(|b|b.is_ascii_digit()||(b'a'..=b'f').contains(&b)))?;
        *to=u64::from_str_radix(word,16).map_err(|_|error("G5","PRODUCT_ATTEMPT_MISMATCH"))?;
    }
    let zero=limbs.iter().all(|&x|x==0);
    fail(v["negative"]==false && if zero {e==0} else {limbs[15]>>63==1})?;
    Ok((e,limbs))
}
fn loaded_maximum_diagnostic(document:&Value,a:&Value,owner:&Value)->VResult {
    let fail=|ok|need(ok,"G5","PRODUCT_ATTEMPT_MISMATCH");
    let failure=&a["result"]["error"];
    let named=failure["kind"]=="proof" && failure["cause"]["kind"]=="association" && failure["cause"]["detail"]=="loaded maximum search exhausted";
    let diagnostic=a["proof"].get("loaded_maximum_diagnostic");
    fail(named==diagnostic.is_some())?;
    let Some(d)=diagnostic else{return Ok(());};
    fail(!route(document).exact && a["owner_ref"]["kind"]=="case" && owner["status"]=="unavailable" && a["result"]["kind"]=="unavailable"
        && a["stages"]["proof_start"]=="completed" && a["stages"]["projection"]=="failed"
        && ["maxima","values","aliases","certificate","observables","g5a"].iter().all(|k|a["stages"][*k]=="not_entered"))?;
    let s=at(&document["retained_precision"]["body"]["sources"],&a["source_ref"],"G5","PRODUCT_ATTEMPT_MISMATCH")?;
    let member=u(&d["member"]);fail(exact_active_member(s,member))?;
    let rows=rows_for(document,owner);
    let row=uint(&d["row"]).and_then(|i|rows.get(i as usize)).ok_or_else(||error("G5","PRODUCT_ATTEMPT_MISMATCH"))?;
    fail(row["kind"]=="pipe_elastic_normal_stress_maximum_v2" && list(&s["id_maps"]["members"]).iter().any(|m|u(&m["kernel_member"])==member && m["id"]==row["entity_ref"]))?;
    let next=rows.iter().enumerate().filter(|(_,r)|owner_projected(owner,r,Some(s),true)).nth(list(&a["proof"]["projection_outcomes"]).len()).map(|(i,_)|i as u64);
    fail(next==uint(&d["row"]))?;
    let (le,l)=diagnostic_endpoint(&d["lower"])?;let (ue,u)=diagnostic_endpoint(&d["upper"])?;
    let lzero=l.iter().all(|&x|x==0);let uzero=u.iter().all(|&x|x==0);
    fail(lzero || (!uzero && (le<ue || (le==ue && l.iter().rev().cmp(u.iter().rev())!=std::cmp::Ordering::Greater))))
}
/// B2-C §10.1 G5 products for one CombinationAttempt (CONTRACT §4; REVISION_01
/// N-3 branch point 11 and N-9): owned by its entry, on its Run and its
/// CombinationSource; the proof stages as C3 without a preparation stage;
/// native completed iff the Run is selected; Ready iff `retained_selected`;
/// an unavailable attempt is its entry's cause and takes §4's reason table.
fn combination_attempt(
    source: &Value,
    b: &Value,
    ai: usize,
    a: &Value,
    pf: &dyn Fn(bool) -> VResult,
    wf: &mut dyn FnMut(bool) -> VResult,
) -> VResult {
    let e = at(&b["combinations"], &a["owner_ref"]["index"], "G5", "PRODUCT_ATTEMPT_MISMATCH")?;
    let st = &a["stages"];
    let run = &e["run"];
    pf(e["product_attempt_ref"] == a["id"]
        && !run.is_null()
        && run["id"] == a["run_ref"]
        && a["source_ref"] == e["source_ref"])?;
    let s = at(&b["sources"], &a["source_ref"], "G5", "PRODUCT_ATTEMPT_MISMATCH")?;
    pf(s["owner"]["kind"] == "combination"
        && s["owner"]["combination_index"] == a["owner_ref"]["index"]
        && st.get("preparation").is_none())?;
    proof_checks(source, a, e, pf, wf)?;
    let selected = run["kernel_terminal"]["kind"] == "selected";
    pf(st["native"] != "not_entered" && (st["native"] == "completed") == selected)?;
    let proof = &a["proof"];
    if a["result"]["kind"] == "ready" {
        pf(!proof.is_null()
            && st
                .as_object()
                .is_some_and(|o| o.values().all(|v| v == "completed")))?;
        wf(exact_work(proof)
            && a["adapter"]["fault"].is_null()
            && exact_work(&a["overlay_work"])
            && exact_work(&a["g5a_work"]))?;
    }
    pf((a["result"]["kind"] == "ready") == (e["disposition"] == "retained_selected"))?;
    if a["result"]["kind"] == "unavailable" {
        pf(e["disposition"] == "retained_unavailable"
            && e["reason"]["cause"]["kind"] == "prepared_product_failure"
            && u(&e["reason"]["cause"]["product_attempt_ref"]) == ai as u64)?;
    }
    g5_coverage(a, e, Some(s))?;
    g5_stages_in(a, pf, &STAGE8[1..], false,false)?;
    for ok in accounting_rules(a) {
        wf(ok)?;
    }
    // N-5 (REVISION_01 §3.3): a combination's origin refusal is never a
    // capture error.
    pf(!matches!(text(&a["result"]["error"]["kind"]), "capture" | "observable" | "abandoned")
        || a["result"]["error"]["cause"]["kind"] != "origin")?;
    if a["result"]["kind"] == "unavailable" {
        combination_reason_table(e, a, pf)?;
    }
    Ok(())
}
/// CONTRACT §4's reason table, by the combination attempt's own error: native,
/// or capture with a non-selected Run, gives (`combination_unresolved`,
/// `kernel`); any other error (`facade_certificate`, `facade`) with the Run
/// selected; a preparation error is refused.
fn combination_reason_table(e: &Value, a: &Value, pf: &dyn Fn(bool) -> VResult) -> VResult {
    let run = &e["run"];
    let err = &a["result"]["error"];
    let selected = run["kernel_terminal"]["kind"] == "selected";
    let expected = match text(&err["kind"]) {
        "preparation" => return pf(false),
        "native" => {
            pf(!selected && err["run_ref"] == run["id"])?;
            ("combination_unresolved", "kernel")
        }
        "capture" if !selected => ("combination_unresolved", "kernel"),
        _ => {
            pf(selected)?;
            ("facade_certificate", "facade")
        }
    };
    pf(e["reason"]["code"] == expected.0 && e["reason"]["phase"] == expected.1)?;
    if matches!(text(&err["kind"]), "observable" | "g5a") {
        let key = if err["kind"] == "observable" { "observables" } else { "g5a" };
        pf(a["proof"]["checks"][key]["kind"] == "failed" && a["proof"]["checks"][key]["error"] == *err)?;
    }
    Ok(())
}
/// B2-C §10.1 G5 products: each `retained_unavailable` entry's cause. The
/// no-Call causes have no Call, Run, source or attempt (C3a rule 4 as amended
/// by C-4); `operand_source_unavailable` names the first term whose case is
/// `unavailable` with no CaseSource, or with a Run refused `ledger_unavailable`;
/// `operand_preparation_failure` names the refused record of the first
/// `not_required` term with a refused record, no term qualifying for the
/// former; a `CombinationReason` is its Call's pre-source refusal; with a Run,
/// a `prepared_product_failure` naming its attempt (§4's table, above).
fn combination_causes(b: &Value, pf: &dyn Fn(bool) -> VResult) -> VResult {
    let cs = list(&b["cases"]);
    let ops = list(&b["operand_preparations"]);
    for e in list(&b["combinations"]) {
        if e["disposition"] == "retained_selected" {
            pf(!e["product_attempt_ref"].is_null())?;
        }
        if e["disposition"] != "retained_unavailable" {
            continue;
        }
        let cause = &e["reason"]["cause"];
        let none = ["call_ref", "run", "source_ref", "product_attempt_ref"]
            .iter()
            .all(|k| e[*k].is_null());
        let cases: Vec<Option<(usize, &Value)>> = mechanics_terms(e)
            .into_iter()
            .map(|t| cs.iter().enumerate().find(|(_, c)| c["basis_ref"]["ref_id"] == *t))
            .collect();
        let unsourced = cases.iter().position(|c| {
            c.is_some_and(|(_, c)| {
                c["status"] == "unavailable"
                    && (c["source_ref"].is_null()
                        || (c["run"]["kernel_terminal"]["kind"] == "refused"
                            && c["run"]["kernel_terminal"]["reason"]["tag"] == "ledger_unavailable"))
            })
        });
        let preparation_cause = e["reason"]["code"] == "combination_unresolved"
            && e["reason"]["phase"] == "preparation";
        match (text(&cause["kind"]), text(&cause["space"])) {
            ("operand_source_unavailable", _) => {
                pf(none && preparation_cause && unsourced == uint(&cause["operand_index"]).map(|i| i as usize))?;
            }
            ("operand_preparation_failure", _) => {
                let refused = |ci: usize| {
                    ops.iter().position(|p| {
                        u(&p["owner_ref"]["index"]) == ci as u64 && p["result"]["kind"] == "refused"
                    })
                };
                let first = cases.iter().find_map(|c| {
                    c.filter(|(_, c)| c["status"] == "not_required")
                        .and_then(|(ci, _)| refused(ci))
                });
                pf(none
                    && preparation_cause
                    && unsourced.is_none()
                    && first.is_some_and(|r| uint(&cause["operand_preparation_ref"]) == Some(r as u64)))?;
            }
            (_, "combination") => {
                let call = uint(&e["call_ref"]).and_then(|i| list(&b["calls"]).get(i as usize));
                pf(preparation_cause
                    && ["run", "source_ref", "product_attempt_ref"].iter().all(|k| e[*k].is_null())
                    && call.is_some_and(|c| {
                        c["result"]["kind"] == "pre_source_refusal" && c["result"]["reason"] == *cause
                    }))?;
            }
            ("prepared_product_failure", _) => {
                pf(!e["run"].is_null() && e["product_attempt_ref"] == cause["product_attempt_ref"])?;
            }
            _ => pf(false)?,
        }
    }
    Ok(())
}
/// C3a-7's G5 rows for each operand preparation: its ordinary attempt and
/// material basis are its owner case's (PR-B2 ruling 3, as a case attempt's
/// D4b); `stage == completed` iff `prepared` iff a source; a refused record's
/// error is the preparation branch, never an origin capture error (N-5); C3's
/// member-prefix rules, a prepared record complete as a case attempt whose
/// preparation completed (complete old coverage, every new member ready, its
/// CaseSource on its material basis); then the work equations (WORK).
fn operand_preparations(b: &Value, pf: &dyn Fn(bool) -> VResult, wf: &mut dyn FnMut(bool) -> VResult) -> VResult {
    for p in list(&b["operand_preparations"]) {
        let owner = at(&b["cases"], &p["owner_ref"]["index"], "G5", "PRODUCT_ATTEMPT_MISMATCH")?;
        let ordinary = at(&b["ordinary_attempts"], &p["ordinary_attempt_ref"], "G5", "PRODUCT_ATTEMPT_MISMATCH")?;
        pf(p["ordinary_attempt_ref"] == owner["ordinary"]["attempt_ref"]
            && p["material_basis_ref"] == ordinary["material_basis_ref"])?;
        let prepared = p["result"]["kind"] == "prepared";
        pf((p["stage"] == "completed") == prepared && prepared == !p["source_ref"].is_null())?;
        if !prepared {
            pf(p["result"]["error"]["kind"] == "preparation"
                && p["result"]["error"]["capture"]["kind"] != "origin")?;
        }
        let pm = list(&p["preparation"]["members"]);
        let old = list(&p["operational"]["old"]);
        let new = list(&p["operational"]["new"]);
        pf(new.len() <= pm.len()
            && pm.len() <= old.len()
            && pm.iter().map(|m| u(&m["member"])).eq(0..pm.len() as u64))?;
        member_checks(p, pf, wf)?;
        if prepared {
            let s = at(&b["sources"], &p["source_ref"], "G5", "PRODUCT_ATTEMPT_MISMATCH")?;
            pf(pm.len() == old.len()
                && new.len() == old.len()
                && pm.iter().all(|m| m["result"]["kind"] == "prepared")
                && p["operational"]["old_coverage"] == "complete"
                && new.iter().all(|o| o["result"]["kind"] == "ready")
                && s["material_basis_ref"] == p["material_basis_ref"])?;
            // O4 §4.6: a member's reduced record's work is held to the same exactness
            // (absent, `Null` is exact).
            wf(pm.iter().all(|m| exact_work(&m["work"]) && exact_work(&m["reduced"]["work"]))
                && old.iter().chain(new).all(|o| exact_work(&o["work"]))
                && p["adapter"]["fault"].is_null())?;
        }
        for ok in accounting_rules(p) {
            wf(ok)?;
        }
    }
    Ok(())
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
        // N-3 branch point 11: a combination attempt has no preparation stage
        // and takes CONTRACT §4's reason table.
        if a["owner_ref"]["kind"] == "combination" {
            combination_attempt(source, b, ai, a, &pf, &mut wf)?;
            continue;
        }
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
        member_checks(a, &pf, &mut wf)?;
        proof_checks(source, a, c, &pf, &mut wf)?;
        let proof = &a["proof"];
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
            // O4 §4.6: a member's reduced record's work is held to the same exactness
            // (absent, `Null` is exact).
            wf(exact_work(proof)
                && pm.iter().all(|m| exact_work(&m["work"]) && exact_work(&m["reduced"]["work"]))
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
        let member_values=!route(source).exact && s.is_some_and(|s|list(&s["id_maps"]["members"]).iter()
            .any(|m|exact_active_member(s,u(&m["kernel_member"]))));
        g5_stages(a, &pf,member_values)?;
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
    combination_causes(b, &pf)?;
    operand_preparations(b, &pf, &mut wf)?;
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
fn g5_stages(a: &Value, pf: &dyn Fn(bool) -> VResult,member_values:bool) -> VResult {
    g5_stages_in(a, pf, &STAGE8, true,member_values)
}
/// `g5_stages` over a stage list: a case attempt's eight pipeline stages, or a
/// combination attempt's seven (no preparation stage, so no source rule).
fn g5_stages_in(a: &Value, pf: &dyn Fn(bool) -> VResult, stages: &[&str], preparation: bool,member_values:bool) -> VResult {
    let st = &a["stages"];
    let mut seen_end = false;
    for k in stages {
        if seen_end {
            pf(st[*k] == "not_entered")?;
        } else if st[*k] != "completed" {
            seen_end = true;
        }
    }
    pf((st["observables"] == "not_entered") == (st["g5a"] == "not_entered"))?;
    if preparation {
        pf((st["preparation"] == "completed") == !a["source_ref"].is_null())?;
    }
    let proof = &a["proof"];
    if !proof.is_null() {
        let completion = text(&proof["completion"]["kind"]);
        if st["values"] == "failed" {
            // Only the new active-member completion owns arithmetic to merge.
            // Reserved nodal and combination failures keep their separate owner.
            pf(completion == if member_values {"merged"}else{"separate_failure"})?;
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
        | REQUESTED_MODE_KIND
        | "modulus_basis_record"
        | "combination_modulus_basis_record" => "non_quantity",
        "pipe_lame_hoop_stress_v2"
        | "pipe_lame_radial_stress_v2"
        | "pipe_section_pressure_hoop_stress"
        | "pipe_section_pressure_longitudinal_stress"
        | "constant_effort_support_applied_load"
        | "component_user_stress_multiplier_review"
        | "constant_effort_user_input_review"
        | "spring_hanger_user_input_review" => "input_derived",
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
        // T4-U4b §4.1 (REQ-001 "every other row kind"): explicitly not covered.
        "element_local_normal_stress_extreme" | "element_local_transverse_shear_stress_maximum" => "not_covered",
        // W-7: GLOBAL_END_ROWS_NOT_COVERED.
        "element_global_end_force_v1" | "element_global_end_moment_v1" => "not_covered",
        _ => "not_covered",
    }
}
/// The rows an owner's projection hull-projects: a combination's
/// `displacement_magnitude` rows are formed in the projection's second pass
/// (DEF-C r2 `rows.displacement_magnitude`; REVISION_02 §2.2), not hull-projected.
fn exact_active_member(s:&Value, member:u64)->bool {
    s["owner"]["kind"] != "combination" && list(&s["member_load_terms"]).iter().any(|t|
        uint(&t["member"])==Some(member) && list(&t["intensity"]).iter().any(|v|raw_bits(v).is_some_and(|b|b&0x7fff_ffff_ffff_ffff!=0)))
}
fn loaded_maximum_row(owner:&Value,r:&Value,s:Option<&Value>,route_l:bool)->bool {
    route_l && owner["basis_ref"]["ref_type"]!="combination" && r["kind"]=="pipe_elastic_normal_stress_maximum_v2"
        && s.is_some_and(|s|list(&s["id_maps"]["members"]).iter().any(|m|
            m["id"]==r["entity_ref"] && exact_active_member(s,u(&m["kernel_member"]))))
}
fn owner_projected(owner: &Value, r: &Value, s:Option<&Value>,route_l:bool) -> bool {
    (hull_projected(r) || loaded_maximum_row(owner,r,s,route_l))
        && !(owner["basis_ref"]["ref_type"] == "combination" && r["kind"] == "displacement_magnitude")
}
fn hull_projected(r: &Value) -> bool {
    row_kind(r) != "non_quantity"
        && !derived_kind(r)
        && !global_end_kind(r)
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
    /// A selected case, or a `retained_selected` combination entry.
    case: &'a Value,
    /// The case's CaseSource, or the combination's representative (operand 0's)
    /// CaseSource (REVISION_01 N-9).
    source: &'a Value,
    /// The CaseSources whose nodal terms are the owner's own contributions: the
    /// case's, or each operand's with a nonzero factor.
    data_sources: Vec<&'a Value>,
    /// The proof-owned summary coverage of this selected case's product attempt.
    coverage: &'a Value,
    /// The receipt's row universe (`rows_for`).
    rows: Vec<&'a Value>,
    /// Every published row, reduced twins included (`rows_all_for`).
    all_rows: Vec<&'a Value>,
    extents: Vec<f64>,
    raw: Vec<[f64; 4]>,
    final_scales: Vec<[f64; 4]>,
    prescribed: BTreeSet<(&'a str, &'a str)>,
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
        out.push(numeric_owner(source, c, s, vec![s], coverage)?);
    }
    // B2-C §10.1 G5a-G5c: a `retained_selected` combination is one more numeric
    // owner, on its own rows and Selection, with the representative source's
    // maps and sections and its prescribed DOFs (REVISION_01 N-9).
    for e in list(&b["combinations"])
        .iter()
        .filter(|e| e["disposition"] == "retained_selected")
    {
        let (s, data) = combination_sources(b, e, "G5a", "SCALE_MISMATCH")?;
        let coverage = &at(
            &b["product_attempts"],
            &e["product_attempt_ref"],
            "G5a",
            "SCALE_MISMATCH",
        )?["proof"]["summary_coverage"];
        out.push(numeric_owner(source, e, s, data, coverage)?);
    }
    Ok(out)
}
/// A combination entry's representative CaseSource (operand 0's) and the
/// CaseSources of its operands with a nonzero factor.
fn combination_sources<'a>(
    b: &'a Value,
    e: &Value,
    gate: &'static str,
    code: &str,
) -> VResult<(&'a Value, Vec<&'a Value>)> {
    let cs = at(&b["sources"], &e["source_ref"], gate, code)?;
    let operands = list(&cs["operands"]);
    let rep = at(&b["sources"], &cs["representative_source_ref"], gate, code)?;
    let mut data = Vec::new();
    for o in operands {
        if f(&o["factor"]) != 0.0 {
            data.push(at(&b["sources"], &o["source_ref"], gate, code)?);
        }
    }
    Ok((rep, data))
}
fn numeric_owner<'a>(
    source: &'a Value,
    c: &'a Value,
    s: &'a Value,
    data_sources: Vec<&'a Value>,
    coverage: &'a Value,
) -> VResult<NumericCase<'a>> {
    {
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
            prescribed.insert((text(&n["id"]), text(&d["dof"]["component"])));
        }
        for (bi, body) in list(&s["body_membership"]).iter().enumerate() {
            let extent = body_extent(s, bi, body)?;
            extents.push(extent);
            let mut maxima = [0.0f64; 4];
            for r in &rows {
                let n = finite_check(normalized(r), "G5a")?;
                let input = component(r).is_some_and(|c| {
                    prescribed.contains(&(text(&r["entity_ref"]), c))
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
        Ok(NumericCase {
            case: c,
            source: s,
            data_sources,
            coverage,
            rows,
            all_rows: rows_all_for(source, c),
            extents,
            final_scales: raw.clone(),
            raw,
            prescribed,
        })
    }
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
    data_sources: &[&Value],
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
    let member_data = if list(&source["member_load_terms"]).is_empty() { None } else {
        Some(crate::declared_member_load::recompute(source).map_err(|_| error("G5a", "SCALE_MISMATCH"))?)
    };
    for bi in 0..nb {
        let nodes: Vec<u64> = list(&source["body_membership"][bi]["nodes"])
            .iter()
            .map(u)
            .collect();
        let free = |n: u64, d: &str| nodes.contains(&n) && !fixed.contains(&(n, d));
        if !nodes.iter().any(|n| DOFS.iter().any(|d| free(*n, *d))) {
            fail(!has_data[bi])?;
        }
        if data_sources.iter().flat_map(|s| list(&s["nodal_terms"])).any(|t| {
            free(u(&t["dof"]["node"]), text(&t["dof"]["component"])) && f(&t["value"]) != 0.0
        }) {
            fail(has_data[bi])?;
        }
        if member_data.as_ref().is_some_and(|data| nodes.iter().any(|&node| DOFS.iter().enumerate().any(|(d, name)|
            free(node, name) && data.nonzero.get(node as usize * 6 + d) == Some(&true)))) {
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
            coverage_g5a(c, s, &[s], coverage, None, &extents)?;
        }
    }
    // The combination entries, in authored order, as the cases.
    for e in list(&body["combinations"]) {
        if e["disposition"] == "retained_selected" {
            g5a_selected(
                selected
                    .next()
                    .ok_or_else(|| error("G5a", "SCALE_MISMATCH"))?,
            )?;
        } else if e["disposition"] == "retained_unavailable" && !e["product_attempt_ref"].is_null() {
            let a = at(
                &body["product_attempts"],
                &e["product_attempt_ref"],
                "G5a",
                "SCALE_MISMATCH",
            )?;
            let coverage = &a["proof"]["summary_coverage"];
            if coverage.is_null() {
                continue;
            }
            let (s, data) = combination_sources(body, e, "G5a", "SCALE_MISMATCH")?;
            let mut extents = Vec::new();
            for (bi, b) in list(&s["body_membership"]).iter().enumerate() {
                extents.push(body_extent(s, bi, b)?);
            }
            coverage_g5a(e, s, &data, coverage, None, &extents)?;
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
        coverage_g5a(c.case, c.source, &c.data_sources, c.coverage, Some(sel), &c.extents)?;
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
fn g5b(cases: &mut [NumericCase<'_>], exact: bool) -> VResult {
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
            // O4 §4.6-§4.7: the Timoshenko leaf and the reduced section bind source and
            // selection to each other under one presence rule; route L has one section and
            // no shear model (H-1), so either key there is a mismatch.
            for (l, r) in [(left, right)] {
                section(l.get("shear_area").is_some() == r.get("shear_area").is_some()
                    && l.get("reduced").is_some() == r.get("reduced").is_some())?;
                section(exact || (r.get("shear_area").is_none() && r.get("reduced").is_none()))?;
                if let Some(a) = r.get("shear_area") {
                    section(l["shear_area"] == *a && f(a) > 0.0)?;
                }
                if let Some(rr) = r.get("reduced") {
                    section(l["reduced"] == json!({"area":rr["area"],"section_modulus":rr["section_modulus"]})
                        && f(&rr["area"]) > 0.0 && f(&rr["section_modulus"]) > 0.0)?;
                }
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
/// G5b on the exact route (D2 §4.9.3 G5b; B3-D §6.2 and §1.4), after the
/// shared checks: for each selected case, its one `contract_evidence.exact_cases`
/// entry (matched by the case's `load_case_id`) states the receipt's section
/// bit for bit: for each member, its one `pipe_sections` entry (matched by
/// `pipe_id`) has `outside_diameter_m` = `normalized_od` and `ro_m` =
/// `actual_radius`, and on both section bases (T4-U6, D-5: the one receipt
/// section binds the nominal and the reduced section, which coincide on this
/// route) `As_m2` = `area`, `Z_m3` = `section_modulus`, `I_m4` =
/// `actual_second_moment`, `J_m4` = `actual_polar_moment` and `wall_m` =
/// `effective_wall`. Each reduced stress row of the case is its nominal twin's
/// retained value bit for bit (the successor overlays the twin). A missing or
/// repeated entry, or a value that is not a JSON number, is a mismatch.
fn g5b_exact_evidence(source: &Value, cases: &[NumericCase<'_>]) -> VResult {
    let section = |ok| need(ok, "G5b", "SECTION_MISMATCH");
    let number_bits = |v: &Value| v.as_f64().map(bits);
    let evidence = list(&source["contract_evidence"]["exact_cases"]);
    for c in cases {
        let entries: Vec<_> = evidence
            .iter()
            .filter(|e| e["load_case_id"] == c.case["basis_ref"]["ref_id"])
            .collect();
        section(entries.len() == 1)?;
        let sections = list(&entries[0]["pipe_sections"]);
        for st in list(&c.source["section_terms"]) {
            let member = list(&c.source["id_maps"]["members"])
                .iter()
                .find(|m| m["kernel_member"] == st["member"])
                .ok_or_else(|| error("G5b", "SECTION_MISMATCH"))?;
            let matched: Vec<_> = sections
                .iter()
                .filter(|p| p["pipe_id"] == member["id"])
                .collect();
            section(matched.len() == 1)?;
            let (p, g) = (matched[0], &st["geometry"]);
            for (evidence_key, receipt) in [
                ("outside_diameter_m", &g["normalized_od"]),
                ("ro_m", &g["actual_radius"]),
            ] {
                section(number_bits(&p[evidence_key]).is_some_and(|b| b == *receipt))?;
            }
            // O4 §4.7: without `reduced` the one term binds both bases (which then
            // agree in As, Z, I, J and wall: the overlay rule); with it, the nominal term
            // binds `nominal_section`, `reduced` binds `reduced_section`, and the two
            // evidence sections must differ in that set.
            let reduced = st.get("reduced");
            let terms = [(&st["area"], &st["section_modulus"], g), match reduced {
                None => (&st["area"], &st["section_modulus"], g),
                Some(r) => (&r["area"], &r["section_modulus"], &r["geometry"]),
            }];
            for (basis, (area, modulus, geo)) in crate::section_basis::BASES.into_iter().zip(terms) {
                for (evidence_key, receipt) in [
                    ("As_m2", area),
                    ("Z_m3", modulus),
                    ("I_m4", &geo["actual_second_moment"]),
                    ("J_m4", &geo["actual_polar_moment"]),
                    ("wall_m", &geo["effective_wall"]),
                ] {
                    section(number_bits(&p[basis][evidence_key]).is_some_and(|b| b == *receipt))?;
                }
            }
            if let Some(r) = reduced {
                let rg = &r["geometry"];
                section(rg["route"] == g["route"] && rg["normalized_od"] == g["normalized_od"] && rg["actual_radius"] == g["actual_radius"])?;
                let differ = ["As_m2", "Z_m3", "I_m4", "J_m4", "wall_m"].into_iter().any(|k| {
                    number_bits(&p["nominal_section"][k]) != number_bits(&p["reduced_section"][k])
                });
                section(differ)?;
            }
            // O4 §4.2: a Timoshenko member's prepared shear leaf is the evidence's
            // `beam_theory.shear_area_m2` (the selection itself is G7's and G8's).
            if let Some(a) = st.get("shear_area") {
                section(p["beam_theory"]["model"] == crate::section_basis::TIMOSHENKO
                    && number_bits(&p["beam_theory"]["shear_area_m2"]).is_some_and(|b| b == *a))?;
            }
        }
        // The successor overlays each reduced twin, outside the receipt's row
        // universe, with the retained value of its nominal twin, the bound row
        // immediately before it (§2.5 row order).
        let all = rows_all_for(source, c.case);
        let distinct = distinct_members(source, c.case);
        for (i, row) in all.iter().enumerate() {
            // T4-U4b §4.1: a derived row of either basis is its own not-covered
            // row, formed from its basis's rows (G7's base read), never a twin.
            let reduced = !derived_kind(row)
                && (reduced_twin(row)
                    || crate::section_basis::id_basis(text(&row["id"])) == Some(crate::section_basis::REDUCED));
            // O4 §4.7: a distinct-basis member's reduced rows are receipt rows
            // (certified on the reduced section), not overlays.
            if !reduced || distinct.contains(text(&row["entity_ref"])) {
                continue;
            }
            let nominal = i.checked_sub(1).map(|j| all[j]);
            section(reduced_twin(row) && nominal.is_some_and(|t| {
                !reduced_twin(t)
                    && crate::section_basis::twin_id(text(&row["id"])).is_some_and(|id| t["id"] == id.as_str())
                    && row["value"].as_f64().is_some()
                    && t["value"].as_f64().map(bits) == row["value"].as_f64().map(bits)
            }))?;
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
    let term = list(&c.source["section_terms"])
        .iter()
        .find(|s| s["member"] == member["kernel_member"])
        .ok_or_else(|| error("G5b", "SECTION_MISMATCH"))?;
    // O4 §4.7: a reduced row in the universe (its member's term carries
    // `reduced`) is scaled by its own section.
    let section = match term.get("reduced") {
        Some(r) if reduced_twin(row) => r,
        _ => term,
    };
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
                    text(&d["node_id"]),
                    text(&d["component"]),
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
        let first = classes.len();
        for row in &c.rows {
            let n = normalized(row);
            let k = row_kind(row);
            let input = component(row).is_some_and(|d| {
                c.prescribed
                    .contains(&(text(&row["entity_ref"]), d))
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
        // T4-U6 (D-5): a reduced twin carries its nominal twin's retained value,
        // so it takes that row's class (a twin without one would bind unchecked).
        // The classes then follow the published row order.
        if c.all_rows.len() != c.rows.len() {
            let bound: Vec<RowClassification> = classes.drain(first..).collect();
            let mut next = bound.into_iter();
            let mut last: Option<RowClassification> = None;
            // O4 §4.7: a distinct-basis member's reduced rows are bound rows with
            // their own classes.
            let distinct = distinct_members_of(c.source);
            for row in &c.all_rows {
                let class = if reduced_twin(row) && !distinct.contains(text(&row["entity_ref"])) {
                    let nominal = last.clone().ok_or_else(|| error("G5c", "CLASSIFICATION_MISMATCH"))?;
                    RowClassification {
                        result_id: text(&row["id"]).into(),
                        basis_ref: row["basis_ref"].clone(),
                        normalized_bits: normalized(row).to_bits(),
                        ..nominal
                    }
                } else {
                    next.next().ok_or_else(|| error("G5c", "CLASSIFICATION_MISMATCH"))?
                };
                last = Some(class.clone());
                classes.push(class);
            }
        }
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
/// G8 step 4 on the exact route (B3D-7; REVISION_01 §4.2): the base common
/// E/nu only. Every authored quantity is unit-checked first, as on the preview
/// route; E is the normalized base modulus and nu the authored `poisson_ratio`
/// (unit "1", -1 < nu < 1/2); G-hat = RN64(E / (2 RN64(1 + nu))), a positive
/// normal. Returns (E, G-hat, nu).
fn exact_material(raw: &Value) -> VResult<(f64, f64, f64)> {
    let fail = |ok| need(ok, "G8", "PREPARATION_MISMATCH");
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
    let e = unit_value(&raw["elastic_modulus"], Dimension::Stress)?;
    let poisson = &raw["poisson_ratio"];
    fail(poisson["unit"] == "1")?;
    let nu = poisson["value"].as_f64().unwrap_or(f64::NAN);
    fail(e > 0.0 && nu.is_finite() && -1.0 < nu && nu < 0.5)?;
    let g = e / (2.0 * (1.0 + nu));
    fail(g.is_normal() && g > 0.0)?;
    Ok((e, g, nu))
}
/// D1.3 on the exact route: the current document's three-member v3 contract
/// with either shear model (O4, DEL-04-08 Design `o4-source-extension.md`
/// §4.7), and an explicit, finite, length-valued allowance ≥ 0 with
/// c_m + c_a < t on every pipe section (DEF-E G8; the producer's own refusals).
fn exact_namespace(model: &Value) -> bool {
    model["schema_version"].as_str() == Some("0.7.0")
        && model.as_object().is_some_and(|object| !object.contains_key("shear_model"))
        && pressure_contract_is(&model["pressure_contract"], "3.0.0", "exact_pressure_v3")
        && section_allowances(model, true)
}
/// b7-plan "Exact-contract fields" and O4 §4.7: an exact document declares an
/// explicit length-valued, finite `corrosion_allowance` c_a ≥ 0 on every pipe
/// section, with its mill tolerance c_m (absent or null is zero, as the
/// producer reads it) finite and ≥ 0 and c_m + c_a < t (absent, null,
/// malformed, non-length, negative or over-wall allowances are outside the
/// contract the producer admits); a pressure-free document declares none.
fn section_allowances(model: &Value, exact: bool) -> bool {
    list(&model["pipe_segments"]).iter().all(|pipe| {
        let section = &pipe["section"];
        if exact {
            let allowance = unit_value(&section["corrosion_allowance"], Dimension::Length);
            let wall = unit_value(&section["wall_thickness"], Dimension::Length);
            let mill = if section["mill_tolerance"].is_null() {
                Ok(0.0)
            } else {
                unit_value(&section["mill_tolerance"], Dimension::Length)
            };
            matches!((allowance, mill, wall), (Ok(ca), Ok(cm), Ok(t))
                if ca.is_finite() && ca >= 0.0 && cm.is_finite() && cm >= 0.0 && t.is_finite() && cm + ca < t)
        } else {
            section.as_object().is_none_or(|s| !s.contains_key("corrosion_allowance"))
        }
    })
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
/// A current contract with exactly the identity and admitted shear members.
fn pressure_contract_is(v: &Value, version: &str, mode: &str) -> bool {
    v.as_object().is_some_and(|o| {
        o.len() == 3
            && o.get("version").and_then(Value::as_str) == Some(version)
            && o.get("mode").and_then(Value::as_str) == Some(mode)
            && matches!(
                o.get("shear_model").and_then(Value::as_str),
                Some(crate::section_basis::EULER_BERNOULLI | crate::section_basis::TIMOSHENKO)
            )
    })
}
/// D1.3 on the preview branch, type-strict (B3-D REVISION_01 §3, N-4): branch
/// L, a pressure-free document, keyed on the contract's absence (b7-plan
/// "Model format, pressure-free half"): the current model version 0.7.0 with
/// `pressure_contract` absent. Anything else (a 0.7.0 model with
/// any contract, `{}` or `false` included; the retired 0.1.0 and 0.2.0; any
/// 0.3.0 or 0.4.0 model) is outside the namespace. B3a's branch L3 is dropped
/// (the owner retired that contract product-wide); B3D-10's tightenings stay.
fn preview_namespace(model: &Value) -> bool {
    model["schema_version"].as_str() == Some("0.7.0")
        && model.as_object().is_some_and(|object|
            !object.contains_key("pressure_contract") && !object.contains_key("shear_model"))
        && section_allowances(model, false)
}
/// Member-only admission before G8 clones/hashes any invocation or builds
/// shared maps. Borrowed bounded traversal uses no JSON or index staging.
fn guard_member_g8(source:&Value,inv:&Value,exact:bool)->VResult {
    let model=&inv["request"]["model"];let cases=list(&model["load_cases"]);
    // A source member key always guards, including a malformed/forged source.
    // With that key omitted, only the newly admitted preview member family
    // activates this guard. Legacy pressure, exact and nodal inputs keep their
    // established hash/namespace precedence. These checks only borrow Values.
    let member=list(&source["retained_precision"]["body"]["sources"]).iter().any(|s|s.get("member_load_terms").is_some())
        || (!exact && preview_namespace(model)
            && cases.iter().any(|c|list(&c["primitive_loads"]).iter().any(|l|
                l["target"]["type"]=="element" && l["category"]=="distributed_force" && l["dimension"]=="force_per_length")));
    if !member{return Ok(());}
    let fail=|ok|need(ok,"G8","PREPARATION_MISMATCH");
    let materials=if list(&inv["request"]["materials"]).is_empty(){list(&model["materials"])}else{list(&inv["request"]["materials"])};
    fail(cases.len()<=3 && list(&model["nodes"]).len()<=32 && list(&model["pipe_segments"]).len()<=32
        && list(&model["supports"]).len()<=32 && materials.len()<=4 && list(&model["combinations"]).len()<=2
        && cases.len()+list(&model["combinations"]).len()<=3
        && cases.iter().all(|c|list(&c["primitive_loads"]).len()<=128 && text(&c["id"]).len()<=1024)
        && list(&model["pipe_segments"]).iter().all(|p|text(&p["id"]).len()<=1024))?;
    fail(list(&model["supports"]).iter().map(|s|list(&s["restraints"]).len()).sum::<usize>()<=192
        && materials.iter().all(|m|list(&m["temperature_points"]).len()<=16)
        && list(&model["combinations"]).iter().all(|c|list(&c["terms"]).len()<=3 && list(&c["operand_ids"]).len()<=3))?;
    fn bounded(v:&Value,depth:usize,t:&mut [usize;3])->bool {
        if depth>18{return false;}t[0]+=1;if t[0]>16400{return false;}
        match v {
            Value::String(s)=>{t[1]=t[1].saturating_add(s.len());s.len()<=128 && t[1]<=67584},
            Value::Array(a)=>a.iter().all(|v|bounded(v,depth+1,t)),
            Value::Object(o)=>o.iter().all(|(k,v)|{t[2]=t[2].saturating_add(k.len());k.len()<=128&&t[2]<=67584&&bounded(v,depth+1,t)}),
            _=>true,
        }
    }
    fail(bounded(inv,0,&mut [0;3]))
}
fn g8(source: &Value, inv: &Value, route: &Route) -> VResult {
    guard_member_g8(source,inv,route.exact)?;
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
        // B7a: the producer's pressure-free branch is the current version
        // without a contract (`model_format::is_pressure_free`); 0.4.0 stays
        // excluded. D1.3's namespace, type-strict (B3-D REVISION_01 §3, N-4; B3a's L3
        // dropped); B3b: the exact route's own branch.
        if route.exact { exact_namespace(model) } else { preview_namespace(model) }
            // B1's alignment set, item 2 (g), PP's acceptance: no
            // `reference_configurations` member (null included); `components`
            // absent or []; `combinations` absent or an array (N-3 branch point
            // 12: B2-C admits model combinations, bound below).
            && model.get("components").is_none_or(|v| v.as_array().is_some_and(Vec::is_empty))
            && model.get("combinations").is_none_or(Value::is_array)
            && !model
                .as_object()
                .is_some_and(|m| m.contains_key("reference_configurations")),
        "G8",
        "INVOCATION_MISMATCH",
    )?;
    // B2-C §10.1 G8 invocation: the entries are the model's combinations, in
    // order and id, each expression equal to its model combination.
    let authored = list(&model["combinations"]);
    need(
        list(&b["combinations"]).len() == authored.len()
            && list(&b["combinations"])
                .iter()
                .zip(authored)
                .all(|(e, m)| expression_matches(e, m)),
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
        // Exact route, G8 step 3: the base common E/nu only (D1.5).
        fail(!route.exact || value == json!({"kind":"base"}))?;
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
        let code = if mode == "sparse_interactive" { 1.0 } else { 2.0 };
        // B4/D1 ruling (4): a failed ordinary case's requested-mode basis (G5 bound its shape and
        // owner) binds the actual invocation's mode, and it replaces the ordinary mode row.
        let requested: Vec<_> = rows
            .iter()
            .filter(|r| r["kind"] == REQUESTED_MODE_KIND)
            .collect();
        if !requested.is_empty() {
            // Defence in depth: unreachable while G5 binds the row to the receipt's mode first.
            need(
                requested.len() == 1 && modes.is_empty() && requested[0]["value"].as_f64() == Some(code),
                "G8",
                "REQUESTED_MODE_MISMATCH",
            )?;
            continue;
        }
        // P1: exactly one mode row, valued with the mode code (1 sparse, 2 dense).
        fail(modes.len() == 1 && modes[0]["value"].as_f64() == Some(code))?;
        // E14: the DEC-050/053 parity observation is retired; no table carries its
        // kind, so a row of it has no signature and is refused before G8.
    }
    fail(list(&b["material_bases"]).len() == expected_selectors.len())?;
    for (mi, mb) in list(&b["material_bases"]).iter().enumerate() {
        // A basis's index is G3's (item 1 (f)); its selector and cases come from the invocation.
        fail(
            mb["selector"] == expected_selectors[mi]
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
            if route.exact {
                // Exact route, G8 step 4: E, G-hat and the derived shear origin.
                let (e, g, nu) = exact_material(raw)?;
                fail(
                    m["id"] == raw["id"]
                        && m["selection"] == json!({"kind":"base"})
                        && m["shear_origin"]
                            == json!({"kind":"derived_e_nu","poisson_ratio":bits(nu),
                                "constitutive_basis":"homogeneous_isotropic_E_nu_v1"})
                        && m["elastic_modulus"] == bits(e)
                        && m["shear_modulus"] == bits(g),
                )?;
                continue;
            }
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
    if route.exact {
        let evidence = list(&source["contract_evidence"]["exact_cases"]);
        let case_of = |entry: &Value| {
            cases
                .iter()
                .position(|c| c["id"] == entry["load_case_id"])
                .ok_or_else(|| error("G8", "PREPARATION_MISMATCH"))
        };
        // G8 step 5, S-C (B3D-12): the source reader's actual-material binding over
        // every `exact_cases` entry; its own code is detail only (B3D-13).
        for entry in evidence {
            let ci = case_of(entry)?;
            crate::physics_source::actual_materials(inv, &cases[ci], entry).map_err(|detail| {
                ValidationError {
                    gate: "G8",
                    code: "RETAINED_PRECISION_PREPARATION_MISMATCH".into(),
                    detail: Some(detail),
                }
            })?;
        }
        // G8 step 6, N-6 (REVISION_01 §5): each entry's published `G_pa` is the
        // receipt's G-hat for that material, bit for bit.
        for entry in evidence {
            let mb = &b["material_bases"][case_bases[case_of(entry)?]];
            for pm in list(&entry["pipe_materials"]) {
                let mat = list(&mb["materials"])
                    .iter()
                    .find(|m| m["id"] == pm["material_id"])
                    .ok_or_else(|| error("G8", "PREPARATION_MISMATCH"))?;
                fail(pm["G_pa"].as_f64().map(bits).as_ref() == Some(&mat["shear_modulus"]))?;
            }
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
    for s in list(&b["sources"])
        .iter()
        .filter(|s| s["owner"]["kind"] != "combination")
    {
        let ci = u(&s["owner"]["case_index"]) as usize;
        let case = cases
            .get(ci)
            .ok_or_else(|| error("G8", "PREPARATION_MISMATCH"))?;
        let maps = &s["id_maps"];
        // A source's index and owner kind are G3's (item 1 (f)).
        fail(
            s["owner"]["case_id"] == case["id"]
                && u(&s["material_basis_ref"]) == case_bases[ci] as u64,
        )?;
        // G8's sourced-case check, both routes (ROOT's ruling on I100's B3 addendum 01; D1.5
        // read through C1's "no 0.4 extension"): `pressure_regions` present and [] on the
        // exact route (step 8, D1.5-exact), absent, null or [] on the preview route (B3D-11's
        // leniency), type-strict; `equivalent_static` absent or null; no `analysis_state`
        // member, null included. A key PP's typed load case lacks, such as a case-level
        // `pressure`, is not read.
        fail(
            if route.exact {
                case.get("pressure_regions").is_some_and(|r| r.as_array().is_some_and(Vec::is_empty))
            } else {
                case.get("pressure_regions")
                    .is_none_or(|r| r.is_null() || r.as_array().is_some_and(Vec::is_empty))
            } && case["equivalent_static"].is_null()
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
            // D-5 (O4 §4.7): route E's stiffness section is the nominal wall t; route L
            // keeps its one t - c_m section (H-1).
            let t = if route.exact { wall } else { wall - tol };
            let geo = &section["geometry"];
            fail(
                d > 0.0
                    && t > 0.0
                    && t < d * 0.5
                    && geo["route"] == route.geometry
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
            // O4 §4.7: the D-5 reduced section and the Timoshenko leaf, rederived from the
            // captured invocation. A reduced term is present exactly when t_r = (t - c_m) - c_a
            // differs from t (then the wall, and with it the annulus, differs), on that wall;
            // `shear_area` exactly on a timoshenko_annulus_energy_v1 document, equal to
            // RN(kappa_hat * A_prep) with kappa_hat by the fixed order from the nominal radii
            // (D/2 - t, D/2) and the member's nu, and A_prep the term's prepared area.
            if route.exact {
                let allowance = unit_value(&p["section"]["corrosion_allowance"], Dimension::Length)?;
                let tr = crate::section_basis::reduced_wall(wall, tol, allowance);
                match section.get("reduced") {
                    None => fail(tr.to_bits() == t.to_bits())?,
                    Some(r) => {
                        let rg = &r["geometry"];
                        fail(
                            tr.to_bits() != t.to_bits()
                                && tr > 0.0
                                && rg["route"] == route.geometry
                                && rg["normalized_od"] == bits(d)
                                && rg["effective_wall"] == bits(tr)
                                && rg["actual_radius"] == bits(d * 0.5)
                                && [&r["area"], &r["section_modulus"], &rg["actual_second_moment"], &rg["actual_polar_moment"]]
                                    .into_iter()
                                    .all(|v| f(v) >= f64::MIN_POSITIVE),
                        )?;
                    }
                }
                let timoshenko = model["pressure_contract"]["shear_model"] == crate::section_basis::TIMOSHENKO;
                match section.get("shear_area") {
                    None => fail(!timoshenko)?,
                    Some(a) => {
                        fail(timoshenko)?;
                        let raw = materials
                            .iter()
                            .find(|x| x["id"] == p["material"])
                            .ok_or_else(|| error("G8", "PREPARATION_MISMATCH"))?;
                        let (_, _, nu) = exact_material(raw)?;
                        let kappa = crate::section_basis::timoshenko_shear_factor(d * 0.5 - t, d * 0.5, nu);
                        fail(kappa.is_finite() && kappa > 0.0 && *a == bits(kappa * f(&section["area"])))?;
                    }
                }
            } else {
                fail(section.get("reduced").is_none() && section.get("shear_area").is_none())?;
            }
        }
        let mut terms = Vec::new();
        for (i, l) in list(&case["primitive_loads"]).iter().enumerate() {
            if l["target"]["type"] == "element" { continue; }
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
        verify_member_terms(s, case, pipes, if b["cases"][ci]["status"] == "selected" { Some(&b["cases"][ci]["selection"]["ledger_sha256"]) } else { None }, route.exact)?;
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
    // C3a-7's G8 row: each operand preparation binds as a case attempt does.
    for a in list(&b["product_attempts"])
        .iter()
        .filter(|a| a["owner_ref"]["kind"] != "combination")
        .chain(list(&b["operand_preparations"]))
    {
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
            // D-5 (O4 §4.7): route E's facts carry the nominal wall t; route L t - c_m.
            fail(facts[0] == bits(d) && facts[1] == bits(if route.exact { wall } else { wall - tol }))?;
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
                    // O4 §4.6: the reduced preparation record is the reduced term's
                    // prepared section, as the member's own record is the nominal one's.
                    match (pm.get("reduced"), st.get("reduced")) {
                        (None, None) => {}
                        (Some(pr), Some(r)) => fail(
                            pr["result"]["section"]
                                == json!([
                                    r["area"],
                                    r["geometry"]["actual_second_moment"],
                                    r["geometry"]["actual_polar_moment"],
                                    r["section_modulus"],
                                    r["geometry"]["actual_radius"]
                                ]),
                        )?,
                        _ => fail(false)?,
                    }
                }
                if op["result"]["kind"] == "ready" {
                    verify_new_operational(op)?;
                }
            }
        }
        let _ = case;
    }
    g8_combinations(b, &case_bases)
}
/// B2-C §10.1 G8 invocation: one entry's expression against its model
/// combination (§2.7): basis to kind; mechanics terms' `load_case` and factor
/// bits (the JSON number parsed to binary64) in authored order, repeats kept;
/// subtraction minuend and subtrahend; range ids sorted in UTF-8 byte order;
/// mode.
fn expression_matches(e: &Value, m: &Value) -> bool {
    let x = &e["expression"];
    e["basis_ref"]["ref_id"] == m["id"]
        && match m["basis"].as_str() {
            Some("mechanics") => {
                x["kind"] == "mechanics"
                    && list(&x["terms"]).len() == list(&m["terms"]).len()
                    && list(&x["terms"]).iter().zip(list(&m["terms"])).all(|(t, a)| {
                        a["load_case"].is_string()
                            && t["case_id"] == a["load_case"]
                            && a["factor"].as_f64().is_some_and(|v| t["factor"] == bits(v))
                    })
            }
            Some("result_state_subtraction") => {
                x["kind"] == "result_state_subtraction"
                    && m["minuend_id"].is_string()
                    && m["subtrahend_id"].is_string()
                    && x["minuend_id"] == m["minuend_id"]
                    && x["subtrahend_id"] == m["subtrahend_id"]
            }
            Some("range_envelope") => {
                let mut ids: Vec<&str> = list(&m["operand_ids"]).iter().filter_map(Value::as_str).collect();
                ids.sort_unstable();
                x["kind"] == "range_envelope"
                    && ids.len() == list(&m["operand_ids"]).len()
                    && list(&x["operand_ids"]).iter().map(text).eq(ids)
                    && m["mode"].is_string()
                    && x["mode"] == m["mode"]
            }
            _ => false,
        }
}
/// B2-C §10.1 G8 preparation, after the case and operand-preparation bindings:
/// each CombinationSource's K4CMB recomputed from its operands' recomputed
/// K4SRC bytes and factor bits (`kernel_source_sha256`; K4LED is attested,
/// C-12); R-8's operand equality: every operand CaseSource has the
/// representative's material basis and `section_terms`, and the combination
/// attempt's material basis is every operand case's.
fn g8_combinations(b: &Value, case_bases: &[usize]) -> VResult {
    use sha2::{Digest, Sha256};
    let fail = |ok| need(ok, "G8", "PREPARATION_MISMATCH");
    for entry in list(&b["combinations"]) {
        if entry["reason"]["cause"]["tag"] == "operand_member_loads" {
            fail(mechanics_terms(entry).iter().any(|id| list(&b["sources"]).iter().any(|source|
                source["owner"]["kind"] == "case" && source["owner"]["case_id"] == **id && !list(&source["member_load_terms"]).is_empty())))?;
        }
    }
    for s in list(&b["sources"])
        .iter()
        .filter(|s| s["owner"]["kind"] == "combination")
    {
        let operands = list(&s["operands"]);
        let rep = at(&b["sources"], &s["representative_source_ref"], "G8", "PREPARATION_MISMATCH")?;
        let mut k4cmb = b"K4CMB\x01".to_vec();
        k4cmb.extend(
            u32::try_from(operands.len())
                .map_err(|_| error("G8", "PREPARATION_MISMATCH"))?
                .to_le_bytes(),
        );
        for o in operands {
            let case_source = at(&b["sources"], &o["source_ref"], "G8", "PREPARATION_MISMATCH")?;
            fail(list(&case_source["member_load_terms"]).is_empty())?;
            let k4src = native_bytes(case_source, true)?;
            k4cmb.extend(
                raw_bits(&o["factor"])
                    .ok_or_else(|| error("G8", "PREPARATION_MISMATCH"))?
                    .to_le_bytes(),
            );
            k4cmb.extend(
                u32::try_from(k4src.len())
                    .map_err(|_| error("G8", "PREPARATION_MISMATCH"))?
                    .to_le_bytes(),
            );
            k4cmb.extend(k4src);
            let ci = usize::try_from(u(&case_source["owner"]["case_index"])).ok();
            fail(
                case_source["owner"]["kind"] == "case"
                    && case_source["material_basis_ref"] == rep["material_basis_ref"]
                    && case_source["section_terms"] == rep["section_terms"]
                    && ci.and_then(|ci| case_bases.get(ci)).is_some_and(|mb| u(&case_source["material_basis_ref"]) == *mb as u64),
            )?;
        }
        fail(s["kernel_source_sha256"] == format!("{:x}", Sha256::digest(&k4cmb)))?;
    }
    for a in list(&b["product_attempts"])
        .iter()
        .filter(|a| a["owner_ref"]["kind"] == "combination")
    {
        let s = at(&b["sources"], &a["source_ref"], "G8", "PREPARATION_MISMATCH")?;
        for o in list(&s["operands"]) {
            let ci = usize::try_from(u(&o["case_index"])).ok();
            fail(ci.and_then(|ci| case_bases.get(ci)).is_some_and(|mb| u(&a["material_basis_ref"]) == *mb as u64))?;
        }
    }
    // The combination's Selection section terms are the representative's,
    // projected to the Selection members.
    for e in list(&b["combinations"])
        .iter()
        .filter(|e| e["disposition"] == "retained_selected")
    {
        let cs = at(&b["sources"], &e["source_ref"], "G8", "PREPARATION_MISMATCH")?;
        let rep = at(&b["sources"], &cs["representative_source_ref"], "G8", "PREPARATION_MISMATCH")?;
        let terms = list(&rep["section_terms"]);
        fail(
            list(&e["selection"]["section_terms"]).len() == terms.len()
                && list(&e["selection"]["section_terms"]).iter().zip(terms).all(|(l, r)| {
                    ["area", "section_modulus", "length", "axial_stiffness", "torsional_stiffness"]
                        .iter()
                        .all(|k| l[*k] == r[*k])
                }),
        )?;
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
/// New-domain-only staging is scoped to this call. The original nodal term
/// builder and the existing G8 maps/layout retain their prior owners.
struct MemberExpected { ordinal:usize, member:usize, intensity:[u64;3] }
fn verify_member_terms(s:&Value, case:&Value, pipes:&[Value], selected_digest:Option<&Value>, exact:bool)->VResult {
    let fail=|ok| need(ok,"G8","PREPARATION_MISMATCH");
    let authored=list(&case["primitive_loads"]);
    if !authored.iter().any(|l|l["target"]["type"]=="element") {
        return fail(s.get("member_load_terms").is_none());
    }
    fail(!exact && authored.len()<=128 && pipes.len()<=32 && text(&case["id"]).len()<=1024)?;
    // Fixed records borrow authored identities; no member JSON/id cloning.
    let mut expected=Vec::with_capacity(authored.len());
    for (ordinal,l) in authored.iter().enumerate().filter(|(_,l)| l["target"]["type"]=="element") {
        let member=pipes.iter().position(|p|p["id"]==l["target"]["pipe"]).ok_or_else(||error("G8","PREPARATION_MISMATCH"))?;
        fail(text(&pipes[member]["id"]).len()<=1024 && text(&l["magnitude"]["unit"]).len()<=1024)?;
        let generated=format!("load:self-weight:{}:{}:{}:{}",text(&case["id"]).len(),text(&case["id"]),text(&pipes[member]["id"]).len(),text(&pipes[member]["id"]));
        fail(l["category"]=="distributed_force" && l["dimension"]=="force_per_length" && l.get("extent").is_none() && l["id"]!=generated)?;
        let axis=match text(&l["direction"]) {"global_x"=>0,"global_y"=>1,"global_z"=>2,_=>return Err(error("G8","PREPARATION_MISMATCH"))};
        let mut intensity=[0u64;3];intensity[axis]=unit_value(&l["magnitude"],Dimension::ForcePerLength)?.to_bits();
        expected.push(MemberExpected{ordinal,member,intensity});
    }
    expected.sort_unstable_by(|a,z| (a.member,text(&authored[a.ordinal]["id"]).as_bytes(),a.intensity,a.ordinal)
        .cmp(&(z.member,text(&authored[z.ordinal]["id"]).as_bytes(),z.intensity,z.ordinal)));
    let emitted=list(&s["member_load_terms"]);
    fail(emitted.len()==expected.len() && emitted.iter().zip(&expected).all(|(a,e)|
        u(&a["constructor_ordinal"])==e.ordinal as u64 && u(&a["primitive_load_index"])==e.ordinal as u64
        && a["source_id"]==authored[e.ordinal]["id"] && u(&a["member"])==e.member as u64
        && list(&a["intensity"]).len()==3 && list(&a["intensity"]).iter().zip(e.intensity).all(|(v,b)|raw_bits(v)==Some(b))))?;
    if let Some(digest)=selected_digest {
        let computed=crate::declared_member_load::recompute(s).map_err(|_|error("G8","PREPARATION_MISMATCH"))?;
        fail(*digest==computed.digest)?;
    }
    Ok(())
}
fn verify_native_source_hashes(s: &Value) -> VResult {
    use sha2::{Digest, Sha256};
    for (source, expected) in [(true, "kernel_source_sha256"), (false, "stiffness_sha256")] {
        // Encoding and hash staging end before member G8 arithmetic starts.
        let matches = {
            let encoded = native_bytes(s, source)?;
            let digest = Sha256::digest(&encoded);
            s[expected] == format!("{:x}", digest)
        };
        need(matches, "G8", "PREPARATION_MISMATCH")?;
    }
    Ok(())
}
/// A CaseSource's native K4SRC (`source`) or K4STF bytes, rebuilt from its maps.
fn native_bytes(s: &Value, source: bool) -> VResult<Vec<u8>> {
    if source && s.get("member_load_terms").is_some() {
        // Bound the complete new-domain encoding before any buffer grows.
        // These are conservative supersets of D1 (text up to1024 bytes,
        // nodal/member counts separately128); nodal-only behavior is untouched.
        let fail=|ok|need(ok,"G8","PREPARATION_MISMATCH");
        let maps=&s["id_maps"];
        fail(list(&maps["nodes"]).len()<=32 && list(&maps["members"]).len()<=32
            && list(&maps["springs"]).len()<=192 && list(&s["constraints"]).len()<=192
            && list(&s["nodal_terms"]).len()<=128 && list(&s["stations"]).len()<=96
            && list(&s["supports"]).len()<=32 && s["member_load_terms"].as_array().is_some_and(|v|!v.is_empty()&&v.len()<=128))?;
        fail(list(&maps["nodes"]).iter().all(|n|list(&n["coordinates"]).len()==3)
            && list(&maps["members"]).iter().all(|m|list(&m["y_reference"]).len()==3)
            && list(&s["supports"]).iter().all(|g|list(&g["restrained"]).len()==6 && list(&g["springs"]).len()<=192)
            && list(&s["nodal_terms"]).iter().chain(list(&s["member_load_terms"])).all(|t|text(&t["source_id"]).len()<=1024)
            && list(&s["member_load_terms"]).iter().all(|t|list(&t["intensity"]).len()==3))?;
    }
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
    {
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
        // O4 §4.2 (FK `TimoshenkoLeafSet::encode_block`): a non-empty Timoshenko
        // sidecar makes the encoding version 2 and appends, after the stiffness part,
        // the framed block `TSH\x01`, the count, then each leaf's member (u32) and
        // shear-area bits (u64), sorted by member. An Euler-Bernoulli source keeps
        // version 1 and today's bytes.
        let mut leaves: Vec<(u64, &Value)> = list(&s["section_terms"])
            .iter()
            .filter_map(|st| st.get("shear_area").map(|a| (u(&st["member"]), a)))
            .collect();
        if !leaves.is_empty() {
            leaves.sort_by_key(|l| l.0);
            out[5] = 2;
            out.extend_from_slice(b"TSH\x01");
            put(&mut out, leaves.len() as u64)?;
            for (member, shear_area) in leaves {
                put(&mut out, member)?;
                val(&mut out, shear_area)?;
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
            let members = list(&s["member_load_terms"]);
            if !members.is_empty() {
                put(&mut out, members.len() as u64)?;
                for term in members {
                    put(&mut out, u(&term["member"]))?;
                    for value in list(&term["intensity"]) { val(&mut out, value)?; }
                    let id = text(&term["source_id"]);
                    put(&mut out, id.len() as u64)?;
                    out.extend(id.as_bytes());
                }
            }
        }
        Ok(out)
    }
}
/// B4/D1 W1 extension: whether a case's ordinary attempt genuinely failed: a structural failure
/// outside DN §4.3's exclusions (mechanism, asymmetric, invalid input) that W2 did not publish.
fn failed_ordinary(o: &Value) -> bool {
    o["initial"]["kind"] == "structural_failure"
        && !matches!(
            text(&o["initial"]["error"]["tag"]),
            "mechanism" | "asymmetric" | "invalid_input"
        )
        && matches!(text(&o["w2"]["kind"]), "not_triggered" | "failed")
}
/// The requested-mode basis text (b7-plan "Failed-ordinary successor rulings" (4)): the captured
/// mode token, the failed ordinary attempt and the selected retained owner (the receipt's case
/// `case_index`) with its certificate (that case's `selection`).
pub fn requested_mode_basis(mode: &str, case_index: usize) -> String {
    format!("captured_requested_mode; solver_mode={mode}; ordinary_attempt=failed_without_observation; owner_ref=case:{case_index}; certificate=retained_precision.body.cases[{case_index}].selection")
}
/// The integrity code product_physics `integrity_failure_code` publishes for an ordinary attempt's
/// structural failure, over the receipt's encoding of that error. Never a success code.
fn initial_failure_code(error: &Value) -> &'static str {
    let detail = text(&error["detail"]);
    match text(&error["tag"]) {
        "mechanism" => "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM",
        "negative_energy" => "NUMERICAL_INTEGRITY_NEGATIVE_ENERGY",
        "asymmetric" | "invalid_input" => "NUMERICAL_INTEGRITY_FAILED",
        "numerically_unresolved" if detail.contains("contribution") || detail.contains("assembly") => {
            "NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED"
        }
        _ => "NUMERICAL_INTEGRITY_UNRESOLVED",
    }
}
/// The case quality tokens (structural_status, solve_quality, model_matrix_fidelity,
/// accuracy_evidence) product_physics `assessed_numerical_quality` assigns to a failure code.
fn failure_quality(code: &str) -> [&'static str; 4] {
    match code {
        "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM" => ["physical_mechanism_witnessed", "failed", "not_assessed", "not_claimed"],
        "NUMERICAL_INTEGRITY_NEGATIVE_ENERGY" => ["negative_energy_witnessed", "failed", "not_assessed", "not_claimed"],
        "NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED" => ["numerically_unresolved", "unresolved", "assembly_loss_detected", "unresolved"],
        "NUMERICAL_INTEGRITY_FAILED" => ["numerically_unresolved", "failed", "assembly_uncertainty", "unresolved"],
        _ => ["numerically_unresolved", "unresolved", "assembly_uncertainty", "unresolved"],
    }
}
/// The fixed tail of product_physics' integrity and W2-refusal diagnostic messages.
const INTEGRITY_MESSAGE_TAIL: &str =
    "; no structural rejection is bypassed by generic LU or output quantization";
/// B4/D1 rulings (2)-(4) (G5, before G8's invocation binding): a requested-mode basis row exists
/// only on a selected case whose ordinary attempt genuinely failed, and such a case has exactly
/// one; it never coexists with an ordinary mode or parity observation; its value is the code of
/// the receipt's requested mode and its metadata names the mode, the case's retained owner and
/// its certificate. That case's quality entry keeps the failed attempt's outcome (never a
/// success token) and names the certificate's diagnostic and the basis row. A blocking
/// diagnostic is admitted only as such a case's own failure diagnostic (ruling (3)).
fn g5_requested_mode(source: &Value) -> VResult {
    let b = &source["retained_precision"]["body"];
    let rm = |ok| need(ok, "G5", "REQUESTED_MODE_MISMATCH");
    let mut bound: Vec<(&Value, &Value)> = Vec::new();
    for (i, c) in list(&b["cases"]).iter().enumerate() {
        let o = &b["ordinary_attempts"][i];
        let cid = &c["basis_ref"]["ref_id"];
        let rows = rows_for(source, c);
        let requested: Vec<&Value> = rows
            .iter()
            .copied()
            .filter(|r| r["kind"] == REQUESTED_MODE_KIND)
            .collect();
        let observed = rows
            .iter()
            .filter(|r| {
                text(&r["kind"]) == "linear_solver_mode_basis"
            })
            .count();
        let failed = failed_ordinary(o);
        if requested.is_empty() {
            rm(!(failed && c["status"] == "selected"))?;
            continue;
        }
        rm(failed && c["status"] == "selected" && requested.len() == 1 && observed == 0)?;
        let row = requested[0];
        let mode = text(&o["requested_mode"]);
        let code: f64 = match mode {
            "sparse_interactive" => 1.0,
            "dense_scrutiny" => 2.0,
            _ => return rm(false),
        };
        let m = &row["metadata"];
        rm(row["id"] == REQUESTED_MODE_ID
            && row["unit"] == "mode_code"
            && row["entity_ref"] == "solver:linear_static_preview"
            && row["value"].as_f64().is_some_and(|v| v.to_bits() == code.to_bits())
            && list(&row["source_result_refs"]).is_empty()
            && m["component"] == REQUESTED_MODE_COMPONENT
            && m["coordinate_system"] == REQUESTED_MODE_COORDINATE
            && m["location"] == *cid
            && m["sign_convention"] == REQUESTED_MODE_SIGN
            && m["basis"] == requested_mode_basis(mode, i))?;
        let q = &source["numerical_quality"]["cases"][i];
        let refs = list(&q["evidence_refs"]);
        let selected = json!(format!("diagnostic:retained-precision:{}:selected", text(cid)));
        rm(q["basis_ref"] == c["basis_ref"]
            && matches!(text(&q["solve_quality"]), "unresolved" | "failed")
            && q["structural_status"] != "passive_model_basis"
            && q["accuracy_evidence"] != "reference_verified"
            && refs.contains(&json!(REQUESTED_MODE_ID))
            && refs.contains(&selected))?;
        // Ruling (3), reverse binding: each failure diagnostic the attempt names is still that
        // failure's diagnostic: blocking, naming this case alone, with the non-success code the
        // recorded failure determines (a failed W2 is ruling (5)'s named refusal). The receipt
        // records no message, so the message is bound to the producer's template for this case.
        // Ruling (2): the case's quality keeps that code's outcome tokens and names the diagnostic.
        let fd = |ok| need(ok, "G5", "FAILURE_DIAGNOSTIC_MISMATCH");
        let fq = |ok| need(ok, "G5", "FAILURE_QUALITY_MISMATCH");
        let mut expected: Vec<(&Value, &str, bool)> = Vec::new();
        if !o["initial"]["diagnostic_ref"].is_null() {
            let code = initial_failure_code(&o["initial"]["error"]);
            expected.push((&o["initial"]["diagnostic_ref"], code, false));
        }
        if !o["w2"]["diagnostic_ref"].is_null() {
            fd(o["w2"]["kind"] == "failed")?;
            expected.push((&o["w2"]["diagnostic_ref"], "NUMERICAL_INTEGRITY_UNRESOLVED", true));
        }
        fd(!expected.is_empty())?;
        let lead = format!("Load case {}: ", text(cid));
        for (id, code, w2) in expected {
            let Some(d) = list(&source["diagnostics"]).iter().find(|d| d["id"] == *id) else {
                return fd(false);
            };
            let message = text(&d["message"]);
            let body = message.strip_prefix(lead.as_str()).unwrap_or("");
            fd(d["severity"] == "blocking"
                && d["affected_refs"] == json!([cid])
                && d["code"] == code
                && message.ends_with(INTEGRITY_MESSAGE_TAIL)
                && if w2 {
                    body.contains("; range_scaling: attempted; step1_trigger=")
                } else {
                    body.starts_with("structural integrity: ")
                })?;
            let [structural, solve, fidelity, accuracy] = failure_quality(code);
            fq(refs.contains(id)
                && q["structural_status"] == structural
                && q["solve_quality"] == solve
                && q["model_matrix_fidelity"] == fidelity
                && q["accuracy_evidence"] == accuracy)?;
            bound.push((id, cid));
        }
        // F2: the envelope status ranks at least this case's failed outcome (product_physics'
        // maximum), so it is never a success token beside a failed-ordinary case.
        let status = text(&source["numerical_quality"]["status"]);
        fq(status == "failed" || (status == "unresolved" && q["solve_quality"] == "unresolved"))?;
    }
    // Ruling (5): a failed W2 attempt is only the closed named refusal, whose step-1 trigger, when
    // recorded, is the outer range trigger.
    for o in list(&b["ordinary_attempts"]) {
        if o["w2"]["kind"] == "failed" {
            let refusal = &o["w2"]["failure"]["refusal"];
            need(
                o["w2"]["failure"]["tag"] == "refused"
                    && (refusal["trigger"].is_null() || refusal["trigger"] == o["w2"]["trigger"]),
                "G5",
                "W2_FAILURE_MISMATCH",
            )?;
        }
    }
    for d in list(&source["diagnostics"]) {
        if d["severity"] == "blocking" {
            need(
                bound
                    .iter()
                    .any(|(id, cid)| d["id"] == **id && d["affected_refs"] == json!([cid])),
                "G5",
                "BLOCKING_DIAGNOSTIC_UNBOUND",
            )?;
        }
    }
    Ok(())
}
fn g5_ordinary(source: &Value) -> VResult {
    let b = &source["retained_precision"]["body"];
    let fail = |ok| need(ok, "G5", "ATTEMPT_MISMATCH");
    let ds = list(&source["diagnostics"]);
    for (i, c) in list(&b["cases"]).iter().enumerate() {
        let o = &b["ordinary_attempts"][i];
        let cid = &c["basis_ref"]["ref_id"];
        let quality = &source["numerical_quality"]["cases"][i];
        let diag = |id: &Value| -> VResult<&Value> {
            ds.iter()
                .find(|d| d["id"] == *id && list(&d["affected_refs"]).contains(cid))
                .ok_or_else(|| error("G5", "ATTEMPT_MISMATCH"))
        };
        // B1's alignment set, item 1: the ordinary attempt's basis reference
        // resolves to a material basis that lists this case (the ordinary class's
        // reference rule, as D4b's for a product attempt). G8 binds the basis to
        // the invocation's selector.
        fail(
            at(
                &b["material_bases"],
                &o["material_basis_ref"],
                "G5",
                "ATTEMPT_MISMATCH",
            )
            .is_ok_and(|mb| list(&mb["case_indices"]).iter().any(|x| u(x) == i as u64)),
        )?;
        // B7a (b), stage 1: a formation failure names the basis whose stiffness
        // failed to form, which is the failing case's own basis: its basis_index
        // resolves and equals the attempt's material_basis_ref. PY `_g5_ordinary`
        // applies the same rule.
        if o["initial"]["kind"] == "formation_failure" {
            let index = &o["initial"]["basis_index"];
            fail(
                at(&b["material_bases"], index, "G5", "ATTEMPT_MISMATCH").is_ok()
                    && uint(index) == uint(&o["material_basis_ref"]),
            )?;
        }
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
        // B1's alignment set, item 3: C2's cause table (C2 §2: each cause has its
        // own phase and codes, one-to-one), for every unavailable case whose cause
        // is not a prepared product failure (that one is D4d's, in the product
        // class). TS's form, with an `unavailable_precondition` code keyed by its
        // precondition; `receipt_failure` codes stay a set (C2 keys none to `check`).
        if c["status"] == "unavailable"
            && c["reason"]["cause"]["kind"] != "prepared_product_failure"
        {
            let (reason, run) = (&c["reason"], &c["run"]);
            let cause = &reason["cause"];
            let (phase, code) = (text(&reason["phase"]), text(&reason["code"]));
            fail(match text(&cause["kind"]) {
                "source_error" => {
                    phase == "preparation"
                        && code == "source_unavailable"
                        && run.is_null()
                        && !decline.is_null()
                        && decline["error"] == cause["error"]
                }
                "unavailable_precondition" => {
                    matches!(phase, "routing" | "preparation")
                        && run.is_null()
                        && match text(&cause["precondition"]) {
                            "caller" => code == "caller_not_qualified",
                            "resource_admission" => code == "resource_admission_not_available",
                            "upstream_no_wrap" => code == "upstream_no_wrap_not_established",
                            "capture" | "source_family" => code == "source_unavailable",
                            _ => false,
                        }
                }
                "receipt_failure" => {
                    phase == "receipt"
                        && matches!(
                            code,
                            "receipt_encoding"
                                | "publication_hash_range"
                                | "invocation_not_representable"
                        )
                }
                "facade_failure" => {
                    phase == "facade"
                        && code == "facade_certificate"
                        && run["kernel_terminal"]["kind"] == "selected"
                        && cause["owner_ref"] == json!({"kind": "case", "index": i})
                }
                // A kernel reason: the case's own Run, its terminal kind in the
                // code, and the cause equal to the terminal's reason.
                _ => {
                    phase == "kernel"
                        && !run.is_null()
                        && code == format!("kernel_{}", text(&run["kernel_terminal"]["kind"]))
                        && *cause == run["kernel_terminal"]["reason"]
                }
            })?;
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
    // B2-C §10.1 G5 ordinary class: the disposition rule (decision 5, C-1) and
    // D6a's diagnostic rule for each combination entry. D6b stays case-only
    // (REVISION_01 §4.5).
    let gates = list(&source["contract_evidence"]["combination_gates"]);
    let status = |id: &Value| {
        list(&b["cases"])
            .iter()
            .find(|c| c["basis_ref"]["ref_id"] == *id)
            .map(|c| &c["status"])
    };
    for (j, e) in list(&b["combinations"]).iter().enumerate() {
        let id = &e["basis_ref"]["ref_id"];
        let gate = gates.get(j).unwrap_or(&Value::Null);
        let terms = mechanics_terms(e);
        let distinct = terms.iter().enumerate().all(|(i, t)| !terms[..i].contains(t));
        let retained = e["expression"]["kind"] == "mechanics"
            && distinct
            && terms
                .iter()
                .any(|t| status(t).is_some_and(|s| s == "selected"));
        fail(match text(&e["disposition"]) {
            "base_withheld" => {
                gate["withheld"] == true
                    && e["reason"] == gate["reason"]
                    && e["expression"]["kind"] == "mechanics"
            }
            "retained_selected" | "retained_unavailable" => gate["withheld"] != true && retained,
            "ordinary" => gate["withheld"] != true && !retained,
            _ => false,
        })?;
        let exact: Vec<&Value> = ds
            .iter()
            .filter(|d| {
                list(&d["affected_refs"]).contains(id)
                    && !text(&d["code"]).starts_with("RETAINED_PRECISION_")
            })
            .map(|d| &d["id"])
            .collect();
        fail(list(&e["diagnostic_refs"]).iter().collect::<Vec<_>>() == exact)?;
    }
    Ok(())
}
/// R-COMB-1 (R) (B2-C §5 with REVISION_01 §2 and REVISION_02 §4.1): a
/// derivation, not a check. After the G5c classes, for each combination in
/// authored order that is `retained_unavailable`, or `ordinary` with an
/// expression naming a case that is not `not_required`, one class per row in
/// publication order: `not_covered` for a quantity row, `non_quantity` for a
/// record row; `normalized_bits` from the row's value, `scale_bits` null.
fn r_comb_1(source: &Value) -> Vec<RowClassification> {
    let b = &source["retained_precision"]["body"];
    let status = |id: &Value| {
        list(&b["cases"])
            .iter()
            .find(|c| c["basis_ref"]["ref_id"] == *id)
            .map(|c| text(&c["status"]))
    };
    let mut out = Vec::new();
    for e in list(&b["combinations"]) {
        let x = &e["expression"];
        let named: Vec<&Value> = match text(&x["kind"]) {
            "mechanics" => mechanics_terms(e),
            "result_state_subtraction" => vec![&x["minuend_id"], &x["subtrahend_id"]],
            "range_envelope" => list(&x["operand_ids"]).iter().collect(),
            _ => Vec::new(),
        };
        let withheld = match text(&e["disposition"]) {
            "retained_unavailable" => true,
            "ordinary" => named.iter().any(|id| status(id) != Some("not_required")),
            _ => false,
        };
        if !withheld {
            continue;
        }
        // Every published row of the entry, reduced twins included (T4-U6).
        for row in rows_all_for(source, e) {
            out.push(RowClassification {
                result_id: text(&row["id"]).into(),
                basis_ref: row["basis_ref"].clone(),
                normalized_bits: normalized(row).to_bits(),
                scale_bits: None,
                class: if row_kind(row) == "non_quantity" {
                    AccuracyClass::NonQuantity
                } else {
                    AccuracyClass::NotCovered
                },
            });
        }
    }
    out
}
/// G7's projection onto the route's base: remove the receipt and the row
/// tokens; set the base identity and profile (preview-physics-2 and
/// `product_preview_mechanics_v1`; pressure-2 and `exact_pressure_v3`).
fn project(source: &Value, raw: bool, route: &Route) -> VResult<Value> {
    let mut projected = source.clone();
    projected
        .as_object_mut()
        .ok_or_else(|| error("G7", "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"))?
        .remove("retained_precision");
    projected["producer"]["semantic_contract_id"] = json!(route.base_id);
    projected["formulation_basis"]["profile_id"] = json!(route.base_profile);
    if raw {
        for r in projected["results"].as_array_mut().ok_or_else(|| {
            error(
                "G7",
                if route.exact {
                    "SOURCE_PHYSICS_ARRAY_INVALID"
                } else {
                    "SOURCE_PREVIEW_PHYSICS_ARRAY_INVALID"
                },
            )
        })?
        {
            if let Some(o) = r.as_object_mut() {
                o.remove("recovery_method");
            }
        }
        // B4/D1: the requested-mode basis is the successor's own record (G5, G8), not a base row.
        projected["results"]
            .as_array_mut()
            .into_iter()
            .for_each(|rows| rows.retain(|r| r["kind"] != REQUESTED_MODE_KIND));
    }
    Ok(projected)
}
// Eligibility is on (U7, D-U7-5): an invocation-bound statement of a solved
// model whose cases are selected or not_required is eligible; every gate runs.
const IMPLEMENTATION_COMPLETE: bool = true;
/// Validate a raw successor statement against the original request/mode.
/// Hashes bind the supplied statements; they do not establish producer origin.
pub fn validate(source: &Value, actual_invocation: Option<&Value>) -> VResult<Validation> {
    let route = route(source);
    if route.exact {
        g0_exact(source)?;
    } else {
        g0(source)?;
    }
    gates_after_g0(source, actual_invocation, route)
}
/// G1-G8, in order, of a statement whose G0 (identity, packaged tables, definitions and bound
/// receipt values) has passed. `validate` is its only production caller; reader-local tests
/// also reach the later gates through it directly.
fn gates_after_g0(source: &Value, actual_invocation: Option<&Value>, route: &Route) -> VResult<Validation> {
    g1(source, true, route)?;
    g2(source)?;
    let normalized = integral_receipt(source);
    let source: &Value = &normalized;
    g3(source, actual_invocation)?;
    g4(source)?;
    let body = &source["retained_precision"]["body"];
    g5_native(body)?;
    g5_ordinary(source)?;
    g5_requested_mode(source)?;
    g5_products(source)?;
    let mut cases = numeric_cases(source)?;
    g5a(body, &cases)?;
    g5b(&mut cases, route.exact)?;
    if route.exact {
        g5b_exact_evidence(source, &cases)?;
    }
    let mut classifications = g5c(&cases)?;
    classifications.extend(r_comb_1(source));
    // G6 binds every published row, reduced twins included (T4-U6).
    for c in list(&body["cases"]) {
        for row in rows_all_for(source, c) {
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
    // B2-C §10.1 G6: `recovery_method` exactly on a `retained_selected`
    // combination's rows.
    for e in list(&body["combinations"]) {
        for row in rows_all_for(source, e) {
            need(
                if e["disposition"] == "retained_selected" {
                    row["recovery_method"] == METHOD
                } else {
                    row.get("recovery_method").is_none()
                },
                "G6",
                "ROW_METHOD_MISMATCH",
            )?;
        }
    }
    let projected = project(source, true, route)?;
    // C1 G7 "existing base failure codes" (06b settlement): each language
    // reports its own unchanged base code, here the Rust base's
    // SOURCE_PREVIEW_PHYSICS_<CODE> (preview) or SOURCE_PHYSICS_<CODE>
    // (exact: pressure-2's `validate_pressure_evidence`); any text
    // after the code is detail.
    crate::semantic_contract::for_source(&projected).map_err(base_error)?;
    // T4-U4b (§4.1, §5.1; S4 round 2): on route L the base read above is
    // model-free, so with the invocation the projection's τ rows are
    // recomputed bit for bit with c_b from the captured model (the
    // preview-physics-2 reader's G2 model checks), reported at G7 with the
    // base's code. Route E's base read already forms c_b from its evidence.
    if let (Some(inv), false) = (actual_invocation, route.exact) {
        crate::u4b_evidence::preview_model_checks(&projected, &inv["request"]["model"]).map_err(base_error)?;
    }
    if let Some(inv) = actual_invocation {
        g8(source, inv, route)?;
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
/// The preview-physics-2 transport metadata check on the reader's projection (B1's
/// alignment set, item 4): the closed statement shape and its internal
/// consistency, with no row read. It is TS's `validatePreviewPhysicsTransportMetadata`,
/// check for check, in TS's order and with TS's detail texts: no foreign namespace;
/// the formulation basis exactly the profile and the table's limitations; the closed
/// `contract_evidence`; then each preview case (its shape, unique id, maximum
/// coverage, support attribution, extrema and intensified measures) and each
/// combination gate. Since RR "I4 made at `30f3d1b24a`; …" ruling 2, TS's check and
/// this one share PY's extrema-number demand (`preview_physics_evidence`'s "extrema
/// numbers"): each extremum's `global_upper_bound_pa` and `certified_gap_pa` is a JSON
/// number. TS's check with that demand is the ruled shared form; this comment claims
/// no other agreement with PY's check. The Rust base reader has no metadata-only
/// entry: `validate_preview_physics_evidence` joins rows throughout. A JSON number
/// read here is always finite, so TS's finite-tree walk has nothing to refuse.
fn preview_physics_transport_metadata(p: &Value) -> Result<(), String> {
    fn demand(ok: bool, detail: &str) -> Result<(), String> {
        if ok {
            Ok(())
        } else {
            Err(format!("SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: {detail}"))
        }
    }
    fn shape(v: &Value, keys: &[&str]) -> bool {
        v.as_object()
            .is_some_and(|o| o.len() == keys.len() && keys.iter().all(|k| o.contains_key(*k)))
    }
    let nonempty = |v: &Value| v.as_str().is_some_and(|t| !t.is_empty());
    let strings = |v: &Value| {
        v.as_array().is_some_and(|a| {
            let mut seen = BTreeSet::new();
            a.iter().all(|x| nonempty(x) && seen.insert(text(x)))
        })
    };
    let number = |v: &Value| v.as_f64().filter(|_| v.is_number());
    let safe_integer = |v: &Value| number(v).filter(|n| n.fract() == 0.0 && n.abs() <= SAFE as f64);
    let table = crate::semantic_contract::preview_physics_contract();
    demand(
        p.get("source_block_recovery").is_none() && p.get("carrier_evidence").is_none(),
        "unsupported source namespace",
    )?;
    let f = &p["formulation_basis"];
    demand(
        shape(f, &["profile_id", "limitations"]),
        "formulation basis",
    )?;
    demand(
        f["profile_id"] == "product_preview_mechanics_v1"
            && f["limitations"].is_array()
            && f["limitations"] == table["supported_profile_limitations"],
        "formulation profile or limitations",
    )?;
    let evidence = &p["contract_evidence"];
    demand(
        shape(evidence, &["preview_cases", "combination_gates"]),
        "transport evidence shape",
    )?;
    demand(
        evidence["preview_cases"].is_array() && evidence["combination_gates"].is_array(),
        "transport evidence shape",
    )?;
    const CASE: [&str; 5] = [
        "load_case_id",
        "pipe_stress_extrema",
        "stress_maximum_coverage",
        "support_attribution",
        "intensified_measures",
    ];
    const EXTREMA: [&str; 13] = [
        "pipe_id",
        "result_id",
        "station_fraction",
        "span_index",
        "local_fraction",
        "value_lower_pa",
        "value_upper_pa",
        "global_upper_bound_pa",
        "certified_gap_pa",
        "subdivisions",
        "approximation",
        "coefficient_basis",
        "enclosure_scope",
    ];
    const MEASURE: [&str; 10] = [
        "result_id",
        "component_id",
        "pipe_id",
        "location",
        "factor_role",
        "sif",
        "sif_source_reference",
        "section_modulus_m3",
        "bending_moment_y_n_m",
        "bending_moment_z_n_m",
    ];
    const WITHHELD: [&str; 2] = [
        "SUPPORT_ACTION_ATTRIBUTION_WITHHELD",
        "CONSTANT_EFFORT_NOT_CONSUMED",
    ];
    const GATES: [&str; 3] = [
        "NONLINEAR_COMBINATION_REQUIRES_SOLVE",
        "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE",
        "COMBINATION_MODULUS_BASIS_MIXED",
    ];
    // MAX_SUBDIVISIONS of core/loads/stress_recovery/src/elastic_extrema.rs.
    const MAX_SUBDIVISIONS: f64 = 131072.0;
    let mut ids = BTreeSet::new();
    let mut attribution_sets = BTreeSet::new();
    let (mut extrema_ids, mut measure_ids) = (Vec::new(), Vec::new());
    for c in list(&evidence["preview_cases"]) {
        demand(shape(c, &CASE), "preview case shape")?;
        demand(
            nonempty(&c["load_case_id"]) && ids.insert(text(&c["load_case_id"])),
            "preview case identity",
        )?;
        demand(
            c["pipe_stress_extrema"].is_array() && c["intensified_measures"].is_array(),
            "preview case lists",
        )?;
        let coverage = &c["stress_maximum_coverage"];
        demand(
            shape(
                coverage,
                &[
                    "complete",
                    "unavailable_pipe_ids",
                    "outside_domain_pipe_ids",
                ],
            ),
            "maximum coverage shape",
        )?;
        let (unavailable, outside) = (
            &coverage["unavailable_pipe_ids"],
            &coverage["outside_domain_pipe_ids"],
        );
        demand(
            coverage["complete"].is_boolean() && strings(unavailable) && strings(outside),
            "maximum coverage values",
        )?;
        demand(
            !list(unavailable).iter().any(|x| list(outside).contains(x)),
            "maximum coverage overlap",
        )?;
        // T4-U4b (§2.1, review item 6; Codex round 2): arcs publish their maximum,
        // so no member is outside its domain, as the raw reader and PY transport
        // require.
        demand(list(outside).is_empty(), "maximum coverage outside the domain")?;
        demand(
            coverage["complete"].as_bool()
                == Some(list(unavailable).is_empty() && list(outside).is_empty()),
            "maximum coverage completeness",
        )?;
        let attribution = &c["support_attribution"];
        demand(
            shape(attribution, &["attributed_support_ids", "withheld"]),
            "support attribution shape",
        )?;
        let attributed = &attribution["attributed_support_ids"];
        demand(
            strings(attributed) && attribution["withheld"].is_array(),
            "support attribution values",
        )?;
        let mut withheld = Vec::new();
        for record in list(&attribution["withheld"]) {
            demand(
                shape(record, &["support_id", "reason"]),
                "withheld support shape",
            )?;
            demand(
                nonempty(&record["support_id"]) && WITHHELD.contains(&text(&record["reason"])),
                "withheld support values",
            )?;
            withheld.push((text(&record["support_id"]), text(&record["reason"])));
        }
        demand(
            !list(attributed)
                .iter()
                .any(|x| withheld.iter().any(|(id, _)| text(x) == *id)),
            "support both attributed and withheld",
        )?;
        for x in list(&c["pipe_stress_extrema"]) {
            demand(shape(x, &EXTREMA), "extrema shape")?;
            // T4-U4b (§2.3): the straight token triple or the arc record's (the raw
            // reader binds an arc's record to its rows; transport has no rows).
            let tokens = (x["approximation"].as_str(), x["coefficient_basis"].as_str(), x["enclosure_scope"].as_str());
            demand(
                nonempty(&x["pipe_id"])
                    && nonempty(&x["result_id"])
                    && (tokens
                        == (
                            Some("piecewise_quadratic_straight_section_statics"),
                            Some("j_side_section_equilibrium_binary64_enclosures"),
                            Some("supplied_binary64_polynomial_coefficient_enclosures; solution error is separate"),
                        )
                        || tokens
                            == (
                                Some(crate::u4b_evidence::ARC_APPROXIMATION),
                                Some(crate::u4b_evidence::ARC_COEFFICIENT_BASIS),
                                Some(crate::u4b_evidence::ARC_ENCLOSURE_SCOPE),
                            )),
                "extrema identity or basis",
            )?;
            // Ruling 2 (RR "I4 made at `30f3d1b24a`; …"): PY's extrema-number demand, at
            // PY's place and with PY's detail; the other four numbers are typed below.
            // PR-B2 N-1: the same demand refuses a negative value (-0 and +0 pass).
            demand(
                ["global_upper_bound_pa", "certified_gap_pa"]
                    .iter()
                    .all(|k| number(&x[*k]).is_some_and(|n| n >= 0.0)),
                "extrema numbers",
            )?;
            demand(
                ["station_fraction", "local_fraction"]
                    .iter()
                    .all(|k| number(&x[*k]).is_some_and(|n| (0.0..=1.0).contains(&n))),
                "extrema fractions",
            )?;
            demand(
                safe_integer(&x["span_index"]).is_some_and(|n| n >= 0.0)
                    && safe_integer(&x["subdivisions"])
                        .is_some_and(|n| (0.0..=MAX_SUBDIVISIONS).contains(&n)),
                "extrema integers",
            )?;
            let (lower, upper) = (number(&x["value_lower_pa"]), number(&x["value_upper_pa"]));
            demand(
                lower.zip(upper).is_some_and(|(l, h)| l >= 0.0 && l <= h)
                    && upper == number(&x["global_upper_bound_pa"]),
                "extrema bounds",
            )?;
            extrema_ids.push(text(&x["result_id"]));
        }
        let pipes: Vec<&str> = list(&c["pipe_stress_extrema"])
            .iter()
            .map(|x| text(&x["pipe_id"]))
            .collect();
        demand(
            pipes.iter().collect::<BTreeSet<_>>().len() == pipes.len()
                && !pipes.iter().any(|p| {
                    list(unavailable)
                        .iter()
                        .chain(list(outside))
                        .any(|x| text(x) == *p)
                }),
            "extrema member partition",
        )?;
        for m in list(&c["intensified_measures"]) {
            // T4-U4b (§2.6): a directional record is told apart by its own key set
            // (the raw reader binds it to its row); its result id stays bound once.
            // Codex round 2: it is one of the schema's two closed directional
            // shapes with their typed fields, or refused with the detail PY's
            // schema check of the same evidence gives.
            if !shape(m, &MEASURE) && m.get("i_in_plane").is_some() && m.get("i_out_of_plane").is_some() {
                let positive = |k: &str| number(&m[k]).is_some_and(|n| n > 0.0);
                let fraction = |k: &str| number(&m[k]).is_some_and(|n| (0.0..=1.0).contains(&n));
                let factors = nonempty(&m["pipe_id"])
                    && nonempty(&m["result_id"])
                    && positive("i_in_plane")
                    && positive("i_out_of_plane");
                let station = shape(m, &["pipe_id", "result_id", "location", "i_in_plane", "i_out_of_plane"])
                    && matches!(m["location"].as_str(), Some("end_i" | "end_j" | "quarter_1" | "midspan" | "quarter_3"));
                let maximum = shape(m, &[&EXTREMA[..], &["i_in_plane", "i_out_of_plane"]].concat())
                    && fraction("station_fraction")
                    && fraction("local_fraction")
                    && number(&m["span_index"]) == Some(0.0)
                    && ["value_lower_pa", "value_upper_pa", "global_upper_bound_pa", "certified_gap_pa"]
                        .iter()
                        .all(|k| number(&m[*k]).is_some())
                    && number(&m["subdivisions"])
                        .is_some_and(|n| n.fract() == 0.0 && (0.0..=MAX_SUBDIVISIONS).contains(&n))
                    && m["approximation"] == crate::u4b_evidence::ARC_DIRECTIONAL_APPROXIMATION
                    && m["coefficient_basis"] == crate::u4b_evidence::ARC_COEFFICIENT_BASIS
                    && m["enclosure_scope"] == crate::u4b_evidence::ARC_ENCLOSURE_SCOPE;
                demand(factors && (station || maximum), "transport evidence shape")?;
                measure_ids.push(text(&m["result_id"]));
                continue;
            }
            demand(shape(m, &MEASURE), "intensified measure shape")?;
            demand(
                nonempty(&m["result_id"])
                    && nonempty(&m["component_id"])
                    && nonempty(&m["pipe_id"])
                    && matches!(text(&m["location"]), "end_i" | "end_j")
                    && matches!(
                        text(&m["factor_role"]),
                        "bend" | "branch_header" | "branch_branch"
                    )
                    && nonempty(&m["sif_source_reference"]),
                "intensified measure identity",
            )?;
            demand(
                [
                    "sif",
                    "section_modulus_m3",
                    "bending_moment_y_n_m",
                    "bending_moment_z_n_m",
                ]
                .iter()
                .all(|k| number(&m[*k]).is_some())
                    && number(&m["sif"]).is_some_and(|n| n > 0.0)
                    && number(&m["section_modulus_m3"]).is_some_and(|n| n > 0.0),
                "intensified measure inputs",
            )?;
            measure_ids.push(text(&m["result_id"]));
        }
        let mut sorted_attributed: Vec<&str> = list(attributed).iter().map(text).collect();
        sorted_attributed.sort_unstable();
        withheld.sort_unstable();
        attribution_sets.insert((sorted_attributed, withheld));
    }
    // The attributed and withheld sets are the same in every case.
    demand(
        attribution_sets.len() <= 1,
        "attribution sets differ between cases",
    )?;
    demand(
        extrema_ids.iter().collect::<BTreeSet<_>>().len() == extrema_ids.len()
            && measure_ids.iter().collect::<BTreeSet<_>>().len() == measure_ids.len(),
        "duplicate evidence result binding",
    )?;
    let mut gates = BTreeSet::new();
    for gate in list(&evidence["combination_gates"]) {
        demand(
            shape(gate, &["combination_id", "withheld", "reason"]),
            "combination gate shape",
        )?;
        demand(
            nonempty(&gate["combination_id"]) && gates.insert(text(&gate["combination_id"])),
            "combination gate identity",
        )?;
        demand(
            match gate["withheld"].as_bool() {
                Some(true) => GATES.contains(&text(&gate["reason"])),
                Some(false) => gate["reason"].is_null(),
                None => false,
            },
            "combination gate reason",
        )?;
    }
    Ok(())
}
/// Metadata-only transport checks cannot reconstitute or verify omitted raw rows.
/// The publication digest is retained as a statement, never authenticated here.
pub fn validate_transport_metadata(source: &Value) -> VResult<Validation> {
    let route = route(source);
    if route.exact {
        g0_exact(source)?;
    } else {
        g0(source)?;
    }
    g1(source, false, route)?;
    g2(source)?;
    let normalized = integral_receipt(source);
    let source: &Value = &normalized;
    let projected = project(source, false, route)?;
    crate::semantic_contract::for_source_metadata(&projected)
        .map_err(|code| ValidationError {
            gate: "G2",
            code,
            detail: None,
        })?;
    // B1's alignment set, item 4: the header check stays at G2; the base's
    // transport metadata check runs at G7, as PY and TS run it: preview-physics-2's
    // on the preview route, the full current exact-retained pressure transport
    // check on the exact route (B7b D4). No current check is retired with v2.
    if route.exact {
        crate::physics_evidence::validate_transport_metadata(&projected).map_err(base_error)?;
    } else {
        preview_physics_transport_metadata(&projected).map_err(base_error)?;
    }
    Ok(Validation {
        invocation_bound: false,
        numerical_eligible: false,
        publication_sha256: text(&source["retained_precision"]["body"]["publication_sha256"])
            .into(),
        classifications: Vec::new(),
    })
}

/// B3b (B3-D §2.4 step 6; REV §2): the table/constant cross-check on a
/// test-only table. The packaged XTABLE binds; a table whose `receipt_bindings`,
/// `receipt_policy` or `accuracy_classification.policy` drifts from the reader's
/// constants does not, and neither does a constant-only change (the bindings
/// compared whole).
#[cfg(test)]
mod b3b_table_binding_tests {
    use super::*;
    #[test]
    fn step_6_table_against_reader_constants() {
        let packaged = exact_table();
        assert!(table_binds_reader_constants(packaged));
        // B3D-8: PTABLE spells the member identically (read by B2's preview G0, not here).
        assert!(table_binds_reader_constants(table()));
        let mut drifted = Vec::new();
        for (path, value) in [
            ("/receipt_bindings/canonicalization", json!("openpipestress_jcs_ijson_v2")),
            ("/receipt_bindings/method", json!("contribution_preserving_multiprecision_v2")),
            ("/receipt_bindings/projection_policy", json!("RP-LOGICAL-ATTEMPTS-v2")),
            ("/receipt_bindings/work/case_limit", json!(19_999_999_999u64)),
            ("/receipt_bindings/work/invocation_limit", json!(60_000_000_001u64)),
            ("/receipt_bindings/work_policy", json!("W1-LME-20B-60B-v2")),
            ("/receipt_bindings/extra", json!(1)),
            ("/receipt_policy", json!("M03-INTEGRITY-MP-v3")),
            ("/accuracy_classification/policy", json!("RP-FACADE-SI-v3")),
        ] {
            let mut t = packaged.clone();
            let (parent, key) = path.rsplit_once('/').unwrap();
            t.pointer_mut(if parent.is_empty() { "" } else { parent }).unwrap()[key] = value;
            if table_binds_reader_constants(&t) {
                drifted.push(path);
            }
        }
        assert!(drifted.is_empty(), "{drifted:?}");
    }
}

/// B7a (b7-plan "Model format, pressure-free half"): G8's preview namespace keys
/// on the pressure contract's absence on the current model version. Keying it on
/// the version string alone (a 0.7.0 model with a contract would pass) or on the
/// retired 0.1.0/0.2.0 fails this test.
#[cfg(test)]
mod b7a_preview_namespace_tests {
    use super::*;
    #[test]
    fn preview_namespace_keys_on_the_contracts_absence_not_the_version() {
        let model = |version: &str, contract: Option<Value>| {
            let mut m = json!({"schema_version": version, "project": {"id": "p"}});
            if let Some(contract) = contract {
                m["pressure_contract"] = contract;
            }
            m
        };
        assert!(preview_namespace(&model("0.7.0", None)));
        assert!(!preview_namespace(&model("0.7.0", Some(Value::Null))));
        for contract in [json!({"version": "2.0.0", "mode": "exact_straight_pressure_v2"}), json!({}), json!(false)] {
            assert!(!preview_namespace(&model("0.7.0", Some(contract.clone()))), "0.7.0 with {contract}");
        }
        for version in ["0.1.0", "0.2.0", "0.3.0", "0.4.0", "0.5.0", "0.5.1", "0.6.0"] {
            assert!(!preview_namespace(&model(version, None)), "{version}");
        }
        assert!(!preview_namespace(&json!({"project": {"id": "p"}})), "a missing version");
        let exact = json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"});
        assert!(exact_namespace(&model("0.7.0", Some(exact.clone()))));
        for value in [Value::Null, json!("euler_bernoulli"), json!(false), json!(1), json!([]), json!({})] {
            let mut preview = model("0.7.0", None);
            preview["shear_model"] = value.clone();
            assert!(!preview_namespace(&preview), "top-level shear {value}");
            let mut pressure = model("0.7.0", Some(exact.clone()));
            pressure["shear_model"] = value.clone();
            assert!(!exact_namespace(&pressure), "top-level shear {value}");
        }
    }
}

#[cfg(test)]
#[path = "../tests/support/retained_precision_corpus.rs"]
mod retained_precision_corpus_storage;

#[cfg(test)]
fn shared_test_corpus() -> &'static Value {
    static CORPUS: std::sync::OnceLock<Value> = std::sync::OnceLock::new();
    CORPUS.get_or_init(|| retained_precision_corpus_storage::expand(serde_json::from_str(
        include_str!("../../../../fixtures/results/retained_precision_cases.json")
    ).unwrap()))
}

/// I61 U6e (snapshot 07g): reader-local pins for RV79-N1 (D37's expected table is
/// the corpus's, derived from native source, never this reader's) and RV80-N2
/// (`integral_receipt` touches only the receipt).
#[cfg(test)]
mod u6e_reader_round_tests {
    use super::*;
    fn corpus() -> &'static Value {
        shared_test_corpus()
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

/// B2-C §8's reader-local G0 tests (REVISION_01 §4.4): G0's table-dependent
/// checks on a test-only copy of PTABLE, the receipt unchanged (the shared
/// ordinary base). Eight drifts of a bound member (each `receipt_bindings`
/// member, `receipt_policy`, `accuracy_classification.policy`) fail row 6
/// SOURCE_PRODUCER_CONTRACT_UNSUPPORTED; a reordered definition list and a
/// changed DEF-C H fail row 4 RETAINED_PRECISION_FORMATION_MISMATCH; the
/// packaged table passes.
#[cfg(test)]
mod b2_g0_table_tests {
    use super::*;
    fn receipt() -> Value {
        let corpus = shared_test_corpus();
        corpus["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["id"] == "ordinary_prepared_synthetic")
            .unwrap()["source"]
            .clone()
    }
    fn with(pointer: &str, value: Value) -> Result<(), ValidationError> {
        let mut t = table().clone();
        *t.pointer_mut(pointer).unwrap() = value;
        g0_with(&receipt(), &t)
    }
    fn unsupported(r: Result<(), ValidationError>) {
        assert_eq!(r.map_err(|e| (e.gate, e.code)), Err(("G0", "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED".into())));
    }
    fn formation(r: Result<(), ValidationError>) {
        assert_eq!(r.map_err(|e| (e.gate, e.code)), Err(("G0", "RETAINED_PRECISION_FORMATION_MISMATCH".into())));
    }
    #[test]
    fn packaged_table_passes() {
        assert_eq!(g0_with(&receipt(), table()), Ok(()));
        assert_eq!(g0(&receipt()), Ok(()));
    }
    #[test]
    fn bindings_canonicalization_drift() {
        unsupported(with("/receipt_bindings/canonicalization", json!("openpipestress_jcs_ijson_v2")));
    }
    #[test]
    fn bindings_method_drift() {
        unsupported(with("/receipt_bindings/method", json!("contribution_preserving_multiprecision_v2")));
    }
    #[test]
    fn bindings_projection_policy_drift() {
        unsupported(with("/receipt_bindings/projection_policy", json!("RP-LOGICAL-ATTEMPTS-v2")));
    }
    #[test]
    fn bindings_case_limit_drift() {
        unsupported(with("/receipt_bindings/work/case_limit", json!(19_999_999_999u64)));
    }
    #[test]
    fn bindings_invocation_limit_drift() {
        unsupported(with("/receipt_bindings/work/invocation_limit", json!(60_000_000_001u64)));
    }
    #[test]
    fn bindings_work_policy_drift() {
        unsupported(with("/receipt_bindings/work_policy", json!("W1-LME-20B-60B-v2")));
    }
    #[test]
    fn receipt_policy_drift() {
        unsupported(with("/receipt_policy", json!("M03-INTEGRITY-MP-v3")));
    }
    #[test]
    fn facade_policy_drift() {
        unsupported(with("/accuracy_classification/policy", json!("RP-FACADE-SI-v3")));
    }
    #[test]
    fn definitions_reordered() {
        let mut list = table()["product_formation_definitions"].clone();
        list.as_array_mut().unwrap().reverse();
        formation(with("/product_formation_definitions", list));
    }
    #[test]
    fn combination_definition_hash_changed() {
        formation(with(
            "/product_formation_definitions/1/sha256",
            json!("0c43cf427b35d35e291e42382d242bca3762b61344b705744127c7c9b4372b6f"),
        ));
    }
    /// Row 9 reads the given table's ids: a table naming only DEF-O refuses at
    /// row 4 first; DEF-C's id on a case attempt passes row 9 (G1's `const`
    /// refuses it, B2-C §10.3 m8).
    #[test]
    fn row_9_reads_the_table_ids() {
        let mut source = receipt();
        source["retained_precision"]["body"]["product_attempts"][0]["definition_id"] = json!(COMBINATION_DEFINITION_ID);
        assert_eq!(g0_with(&source, table()), Ok(()));
        source["retained_precision"]["body"]["product_attempts"][0]["definition_id"] = json!(EXACT_DEFINITION_ID);
        unsupported(g0_with(&source, table()));
        source["retained_precision"]["body"]["product_attempts"][0]["definition_id"] = json!(DEFINITION_ID);
        source["retained_precision"]["body"]["operand_preparations"] = json!([{"definition_id": "RP-UNKNOWN"}]);
        unsupported(g0_with(&source, table()));
    }
}

/// The reader's outcome as JSON, for callers outside Rust (the desktop's wasm
/// engine). A pass is `{"ok":true,"validation":{…}}`: every `Validation` field,
/// each row's binary64 words as 16 lowercase hex digits, `bound_bits` set only
/// on an `absolute_verified` row. A refusal is
/// `{"ok":false,"error":{"gate","code","detail"}}`, with `detail` null when the
/// reader carries none.
pub fn outcome_json(outcome: &VResult<Validation>) -> Value {
    let hex = |bits: u64| Value::from(format!("{bits:016x}"));
    match outcome {
        Ok(v) => json!({"ok": true, "validation": {
            "invocation_bound": v.invocation_bound,
            "numerical_eligible": v.numerical_eligible,
            "publication_sha256": v.publication_sha256,
            "classifications": v.classifications.iter().map(|row| {
                let (class, bound_bits) = match row.class {
                    AccuracyClass::RelativeVerified => ("relative_verified", Value::Null),
                    AccuracyClass::AbsoluteVerified { bound_bits } => ("absolute_verified", hex(bound_bits)),
                    AccuracyClass::InputDerived => ("input_derived", Value::Null),
                    AccuracyClass::NonQuantity => ("non_quantity", Value::Null),
                    AccuracyClass::NotCovered => ("not_covered", Value::Null),
                };
                json!({"result_id": row.result_id, "basis_ref": row.basis_ref,
                    "normalized_bits": hex(row.normalized_bits),
                    "scale_bits": row.scale_bits.map_or(Value::Null, hex),
                    "bound_bits": bound_bits, "class": class})
            }).collect::<Vec<_>>(),
        }}),
        Err(e) => json!({"ok": false, "error": {"gate": e.gate, "code": e.code, "detail": e.detail}}),
    }
}

/// b7-plan R2: preview-physics-2's transport metadata check (the base check this
/// reader's transport reading runs at G7), for callers that read a preview-physics-2
/// header directly. Unchanged; this only makes it reachable.
pub fn validate_preview_physics_transport_metadata(source: &Value) -> Result<(), String> {
    preview_physics_transport_metadata(source)
}

#[cfg(test)]
mod w1_si_first_publication_tests {
    use super::*;
    #[test]
    fn w1_si_first_stress_publication_binds_the_actual_si_word_and_row() {
        let outcome = json!({"kind":"normal","value":"40f8000000000002"});
        for kind in ["element_local_axial_normal_stress", "element_local_bending_normal_stress_y",
            "element_local_bending_normal_stress_z", "element_local_torsional_shear_stress"] {
            let mut row=json!({"kind":kind,"unit":"MPa","value":f64::from_bits(0x3fb92a737110e456)});
            assert!(projection_matches_row(&row,&outcome));
            // Exact dyadic midpoint divided before RN64 is the obsolete other word.
            row["value"]=json!(f64::from_bits(0x3fb92a737110e455));
            assert!(!projection_matches_row(&row,&outcome));
            row["value"]=json!(f64::from_bits(0x3fb92a737110e456));
            for (field,value) in [("kind",json!("element_local_axial_force")),("unit",json!("Pa"))] {
                let mut wrong=row.clone();wrong[field]=value;
                assert!(!projection_matches_row(&wrong,&outcome));
            }
            let changed=json!({"kind":"normal","value":"40f8000000000000"});
            assert!(!projection_matches_row(&row,&changed));
        }
    }
    #[test]
    fn w1_si_first_other_rows_keep_raw_words_and_stress_zero_is_truthful() {
        for (kind,unit,word) in [("global_nodal_displacement_x","mm",0x4197700000000001),
            ("element_local_torsional_moment","N*m",0x40f8000000000002),
            ("pipe_elastic_normal_stress_maximum_v2","Pa",0x40f8000000000002),
            ("element_local_axial_force","MPa",0x40f8000000000002)] {
            let row=json!({"kind":kind,"unit":unit,"value":f64::from_bits(word)});
            let outcome=json!({"kind":"normal","value":format!("{word:016x}")});
            assert!(projection_matches_row(&row,&outcome));
        }
        let mut row=json!({"kind":"element_local_axial_normal_stress","unit":"MPa","value":0.0});
        for outcome in [json!({"kind":"underflow","negative":true}),
            json!({"kind":"subnormal","value":"0000000000000001","relative_precision":"3ff0000000000000"})] {
            assert!(projection_matches_row(&row,&outcome));
            row["value"]=json!(-0.0);assert!(!projection_matches_row(&row,&outcome));row["value"]=json!(0.0);
        }
        for outcome in [json!({"kind":"overflow","negative":false}),
            json!({"kind":"normal","value":"7ff0000000000000"}),json!({"kind":"unknown"})] {
            assert!(!projection_matches_row(&row,&outcome));
        }
    }
}

#[cfg(test)]
mod b4_failed_ordinary_tests {
    //! B4/D1 W1 extension (b7-plan "Failed-ordinary successor rulings"): the requested-mode basis,
    //! the admitted blocking diagnostic and the closed W2 refusal, read below G0. The producer
    //! asserts that the fixture is its own output. With the table pins on the current reviewed
    //! bytes (ruling (6)), the full reader gives the same validation as the gates below G0.
    use super::*;
    const FIXTURE: &str =
        include_str!("../../../../fixtures/results/retained_precision_failed_ordinary_cxf.json");
    /// N06 under b7-plan's "N06 absorbed-spring certificate extension" (four successors).
    const N06_FIXTURE: &str =
        include_str!("../../../../fixtures/results/retained_precision_failed_ordinary_n06.json");
    fn cases() -> Vec<(Value, Value)> {
        cases_of(FIXTURE)
    }
    fn cases_of(text: &str) -> Vec<(Value, Value)> {
        let fixture: Value = serde_json::from_str(text).unwrap();
        list(&fixture["cases"])
            .iter()
            .map(|c| {
                (c["successor"].clone(), json!({"request": c["request"], "solver_mode": c["solver_mode"]}))
            })
            .collect()
    }
    /// Rehash the publication and the receipt (the mutations touch neither sources nor preparations).
    fn rehash(s: &mut Value) {
        let mut env = s.clone();
        env.as_object_mut().unwrap().remove("retained_precision");
        let publication = hash("retained_precision_publication_mp_v2", &env, "G1", "X").unwrap();
        s["retained_precision"]["body"]["publication_sha256"] = json!(publication);
        let receipt = hash("retained_precision_receipt_mp_v2", &s["retained_precision"]["body"], "G1", "X").unwrap();
        s["retained_precision"]["receipt_sha256"] = json!(receipt);
    }
    fn below_g0(s: &Value, inv: Option<&Value>) -> VResult<Validation> {
        gates_after_g0(s, inv, route(s))
    }
    fn refused(s: &Value, inv: &Value) -> (&'static str, String) {
        match below_g0(s, Some(inv)) {
            Err(e) => (e.gate, e.code),
            Ok(_) => ("accepted", String::new()),
        }
    }
    fn requested_index(s: &Value) -> usize {
        list(&s["results"]).iter().position(|r| r["kind"] == REQUESTED_MODE_KIND).unwrap()
    }
    #[test]
    fn failed_ordinary_successors_pass_g1_to_g8_and_are_eligible() {
        for (s, inv) in cases() {
            let mode = inv["solver_mode"].clone();
            assert_eq!(below_g0(&s, None).map(|v| v.numerical_eligible), Ok(false), "{mode}");
            let v = below_g0(&s, Some(&inv)).unwrap_or_else(|e| panic!("{mode}: {e:?}"));
            assert!(v.invocation_bound && v.numerical_eligible, "{mode}");
            assert!(v.classifications.iter().any(|c| c.result_id == REQUESTED_MODE_ID && c.class == AccuracyClass::NonQuantity), "{mode}");
        }
    }
    /// N06's successors read like CX-F's: G1-G8 pass and the case is eligible with its
    /// invocation; with the table pins on the current bytes, the full reader gives the same.
    #[test]
    fn n06_congruence_successors_pass_g1_to_g8_and_are_eligible() {
        let cases = cases_of(N06_FIXTURE);
        assert_eq!(cases.len(), 4);
        for (s, inv) in cases {
            let mode = inv["solver_mode"].clone();
            assert_eq!(below_g0(&s, None).map(|v| v.numerical_eligible), Ok(false), "{mode}");
            let v = below_g0(&s, Some(&inv)).unwrap_or_else(|e| panic!("{mode}: {e:?}"));
            assert!(v.invocation_bound && v.numerical_eligible, "{mode}");
            assert!(v.classifications.iter().any(|c| c.result_id == REQUESTED_MODE_ID && c.class == AccuracyClass::NonQuantity), "{mode}");
            assert_eq!(validate(&s, Some(&inv)), Ok(v), "{mode}");
        }
    }
    /// DEF-O's lanes.congruence admits one further correction on the annular_source lane only:
    /// three calls there, or two on the admitted_k lane, refuse as work mismatches.
    #[test]
    fn n06_lane_correction_calls_are_bounded_by_lane() {
        let wm = ("G5", "RETAINED_PRECISION_WORK_MISMATCH".to_string());
        for (base, inv) in cases_of(N06_FIXTURE) {
            let mode = text(&inv["solver_mode"]).to_owned();
            let lanes = &base["retained_precision"]["body"]["product_attempts"][0]["proof"]["lanes"];
            assert_eq!(lanes[1]["work"]["correction"]["calls"], json!({"kind":"exact","value":2}), "{mode}");
            assert_eq!(lanes[0]["work"]["correction"]["calls"], json!({"kind":"exact","value":1}), "{mode}");
            for (lane, calls) in [(1, 3), (0, 2)] {
                let mut s = base.clone();
                s["retained_precision"]["body"]["product_attempts"][0]["proof"]["lanes"][lane]["work"]["correction"]["calls"] =
                    json!({"kind":"exact","value":calls});
                rehash(&mut s);
                assert_eq!(refused(&s, &inv), wm, "{mode}: lane {lane} calls {calls}");
            }
        }
    }
    /// The lane-work check alone (G5's `proof_checks`) on an N06 attempt relabelled as the exact
    /// route or as a combination: there the annular_source lane keeps at most one correction, so
    /// its two calls refuse; with one call the same relabelled attempt passes, so nothing else in
    /// the check depends on the relabelling.
    #[test]
    fn exact_route_and_combination_lanes_keep_one_correction() {
        fn lane_work_ok(s: &Value) -> bool {
            let b = &s["retained_precision"]["body"];
            let (a, c) = (&b["product_attempts"][0], &b["cases"][0]);
            let pf = |ok: bool| if ok { Ok(()) } else { Err(error("G5", "PRODUCT_ATTEMPT_MISMATCH")) };
            let mut all = true;
            let mut wf = |ok: bool| -> VResult {
                all &= ok;
                Ok(())
            };
            proof_checks(s, a, c, &pf, &mut wf).unwrap();
            all
        }
        let calls = |s: &mut Value, lane: usize, n: u64| {
            s["retained_precision"]["body"]["product_attempts"][0]["proof"]["lanes"][lane]["work"]["correction"]["calls"] =
                json!({"kind":"exact","value":n});
        };
        for (base, inv) in cases_of(N06_FIXTURE) {
            let mode = text(&inv["solver_mode"]).to_owned();
            assert!(lane_work_ok(&base), "{mode}: preview case annular lane, two calls");
            let mut exact = base.clone();
            exact["producer"]["semantic_contract_id"] = json!(EXACT_CONTRACT_ID);
            assert!(!lane_work_ok(&exact), "{mode}: exact route refuses two calls");
            calls(&mut exact, 1, 1);
            assert!(lane_work_ok(&exact), "{mode}: exact route, one call");
            let mut combination = base.clone();
            combination["retained_precision"]["body"]["product_attempts"][0]["owner_ref"]["kind"] = json!("combination");
            assert!(!lane_work_ok(&combination), "{mode}: combination refuses two calls");
            calls(&mut combination, 1, 1);
            assert!(lane_work_ok(&combination), "{mode}: combination, one call");
            let mut admitted = base.clone();
            calls(&mut admitted, 0, 2);
            assert!(!lane_work_ok(&admitted), "{mode}: admitted_k refuses two calls");
        }
        for (source, a, lane, limit) in [(CONTRACT_ID, "case", 1, 2), (CONTRACT_ID, "case", 0, 1),
            (EXACT_CONTRACT_ID, "case", 1, 1), (CONTRACT_ID, "combination", 1, 1), (EXACT_CONTRACT_ID, "combination", 1, 1)] {
            let s = json!({"producer":{"semantic_contract_id":source}});
            assert_eq!(lane_correction_limit(&s, &json!({"owner_ref":{"kind":a}}), lane), limit, "{source} {a} {lane}");
        }
    }
    /// A published certified quantity moved by one ulp (receipt and publication rehashed) no
    /// longer matches its recorded projection outcome and is refused.
    #[test]
    fn n06_perturbed_certified_value_refuses() {
        for (base, inv) in cases_of(N06_FIXTURE) {
            let mode = text(&inv["solver_mode"]).to_owned();
            for kind in ["element_local_torsional_moment", "global_nodal_rotation_x", "global_nodal_rotation_y"] {
                let Some(at) = list(&base["results"]).iter().position(|r| r["kind"] == kind && r["value"].as_f64().is_some_and(|v| v != 0.0)) else { continue };
                let mut s = base.clone();
                let v = s["results"][at]["value"].as_f64().unwrap();
                s["results"][at]["value"] = json!(f64::from_bits(v.to_bits() + 1));
                rehash(&mut s);
                let (gate, code) = refused(&s, &inv);
                assert_ne!(gate, "accepted", "{mode}: {kind}");
                println!("N06_PERTURBED {mode} {kind} {gate} {code}");
            }
        }
    }
    #[test]
    fn failed_ordinary_successors_pass_g0_with_the_current_table_pins() {
        // Ruling (6): TABLE_HASH and its mirrors follow the reviewed current table bytes, so the
        // full reader (G0 first) gives the same eligible validation as the gates below G0.
        for (s, inv) in cases() {
            let mode = inv["solver_mode"].clone();
            let v = validate(&s, Some(&inv)).unwrap_or_else(|e| panic!("{mode}: {e:?}"));
            assert!(v.invocation_bound && v.numerical_eligible, "{mode}");
            assert_eq!(Ok(v), below_g0(&s, Some(&inv)), "{mode}");
        }
    }
    #[test]
    fn requested_mode_mutations_refuse_by_name() {
        let mm = ("G5", "RETAINED_PRECISION_REQUESTED_MODE_MISMATCH".to_string());
        for (base, inv) in cases() {
            let mode = text(&inv["solver_mode"]).to_owned();
            let at = requested_index(&base);
            let mutate = |f: &dyn Fn(&mut Value)| {
                let mut s = base.clone();
                f(&mut s);
                rehash(&mut s);
                refused(&s, &inv)
            };
            // An ordinary mode observation beside the requested-mode basis (coexistence). Appended,
            // so that the receipt's row indices still resolve (an inserted row is G3's).
            assert_eq!(mutate(&|s| {
                let mut row = s["results"][at].clone();
                row["id"] = json!("result:solver-mode:linear-solve-basis");
                row["kind"] = json!("linear_solver_mode_basis");
                s["results"].as_array_mut().unwrap().push(row);
            }), mm, "{mode}: coexistence");
            // A fabricated ordinary observation in place of the basis.
            assert_eq!(mutate(&|s| {
                s["results"][at]["id"] = json!("result:solver-mode:linear-solve-basis");
                s["results"][at]["kind"] = json!("linear_solver_mode_basis");
            }), mm, "{mode}: fabricated observation");
            // E14: a row of the retired parity kind has no signature and is refused.
            let parity = mutate(&|s| {
                let mut row = s["results"][at].clone();
                row["id"] = json!("result:sparse-live:dense-parity-relative-delta");
                row["kind"] = json!("sparse_live_path_dense_parity_relative_delta");
                s["results"].as_array_mut().unwrap().push(row);
            });
            assert_ne!(parity.0, "accepted", "{mode}: retired parity kind {parity:?}");
            // A mode code other than the captured mode.
            assert_eq!(mutate(&|s| {
                let other = if s["results"][at]["value"] == json!(1.0) { 2.0 } else { 1.0 };
                s["results"][at]["value"] = json!(other);
            }), mm, "{mode}: other mode");
            // The owner or certificate named in the basis is another case's.
            for (from, to) in [("owner_ref=case:0", "owner_ref=case:1"), ("cases[0].selection", "cases[1].selection")] {
                assert_eq!(mutate(&|s| {
                    let basis = text(&s["results"][at]["metadata"]["basis"]).replace(from, to);
                    s["results"][at]["metadata"]["basis"] = json!(basis);
                }), mm, "{mode}: {to}");
            }
            // A success token in the failed case's quality entry, or the basis row unnamed there.
            assert_eq!(mutate(&|s| s["numerical_quality"]["cases"][0]["solve_quality"] = json!("sensitive")), mm, "{mode}: sensitive");
            assert_eq!(mutate(&|s| s["numerical_quality"]["cases"][0]["structural_status"] = json!("passive_model_basis")), mm, "{mode}: passive");
            let mut s = base.clone();
            s["numerical_quality"]["cases"][0]["solve_quality"] = json!("checks_passed");
            rehash(&mut s);
            assert_eq!(refused(&s, &inv).0, "G5", "{mode}: checks_passed");
            assert_eq!(mutate(&|s| {
                s["numerical_quality"]["cases"][0]["evidence_refs"].as_array_mut().unwrap().retain(|r| r != REQUESTED_MODE_ID);
            }), mm, "{mode}: basis unnamed");
            // A requested-mode basis on a case whose ordinary attempt succeeded.
            assert_eq!(mutate(&|s| {
                let o = &mut s["retained_precision"]["body"]["ordinary_attempts"][0];
                o["initial"] = json!({"kind":"report","report_diagnostic_ref":"diagnostic:numerical-integrity:case","outcome":"sensitive"});
                o["w2"] = json!({"kind":"not_triggered"});
                s["numerical_quality"]["cases"][0]["solve_quality"] = json!("sensitive");
            }), mm, "{mode}: successful ordinary attempt");
            // The invocation's mode is not the captured mode: G8.
            let mut other = inv.clone();
            other["solver_mode"] = json!(if mode == "sparse_interactive" { "dense_scrutiny" } else { "sparse_interactive" });
            assert_eq!(refused(&base, &other).0, "G8", "{mode}: invocation mode");
        }
    }
    #[test]
    fn blocking_diagnostics_and_w2_refusal_mutations_refuse_by_name() {
        let unbound = ("G5", "RETAINED_PRECISION_BLOCKING_DIAGNOSTIC_UNBOUND".to_string());
        for (base, inv) in cases() {
            let mode = text(&inv["solver_mode"]).to_owned();
            let mutate = |f: &dyn Fn(&mut Value)| {
                let mut s = base.clone();
                f(&mut s);
                rehash(&mut s);
                refused(&s, &inv)
            };
            // A blocking diagnostic not bound to a failed case's requested-mode basis.
            assert_eq!(mutate(&|s| {
                s["diagnostics"].as_array_mut().unwrap().push(json!({"id":"diagnostic:invented:blocking","code":"SOLVER_SYSTEM_BLOCKED",
                    "severity":"blocking","message":"invented","source":"core/product_physics","affected_refs":["supports"]}));
            }), unbound, "{mode}: unbound blocking diagnostic");
            // The failure diagnostic widened beyond its one case: the reverse binding refuses first.
            assert_eq!(mutate(&|s| {
                for d in s["diagnostics"].as_array_mut().unwrap() {
                    if d["severity"] == "blocking" { d["affected_refs"] = json!(["case", "supports"]); }
                }
            }), ("G5", "RETAINED_PRECISION_FAILURE_DIAGNOSTIC_MISMATCH".to_string()), "{mode}: widened blocking diagnostic");
            // Removing the requested-mode basis shifts the receipt's row indices: G3 refuses it. (In
            // place, a fabricated observation without the basis is G5's: the other test.)
            assert_eq!(mutate(&|s| {
                let at = requested_index(s);
                s["results"].as_array_mut().unwrap().remove(at);
            }), ("G3", "RETAINED_PRECISION_COVERAGE_MISMATCH".to_string()), "{mode}: basis removed");
            // A W2 failure other than the translated named refusal.
            assert_eq!(mutate(&|s| {
                s["retained_precision"]["body"]["ordinary_attempts"][0]["w2"]["failure"] = json!({"tag":"not_engaged"});
            }), ("G5", "RETAINED_PRECISION_W2_FAILURE_MISMATCH".to_string()), "{mode}: untranslated W2 variant");
            // The inner refusal trigger differs from the outer range trigger.
            assert_eq!(mutate(&|s| {
                s["retained_precision"]["body"]["ordinary_attempts"][0]["w2"]["failure"]["refusal"]["trigger"]["error"]["detail"] = json!("another");
            }), ("G5", "RETAINED_PRECISION_W2_FAILURE_MISMATCH".to_string()), "{mode}: inner trigger");
        }
    }
    /// The failed case's bound failure diagnostic, by the W2 attempt's reference.
    fn failure_diagnostic(s: &mut Value) -> &mut Value {
        let id = s["retained_precision"]["body"]["ordinary_attempts"][0]["w2"]["diagnostic_ref"].clone();
        s["diagnostics"].as_array_mut().unwrap().iter_mut().find(|d| d["id"] == id).unwrap()
    }
    #[test]
    fn reclassified_failure_diagnostics_refuse_by_name() {
        // Ruling (3)'s reverse binding (g2 review F1): the failure diagnostic the attempt names stays
        // blocking, names its one case, keeps the code the failure determines and the producer's
        // message template (the receipt records no message, so no message text is bound beyond it).
        let fd = ("G5", "RETAINED_PRECISION_FAILURE_DIAGNOSTIC_MISMATCH".to_string());
        for (base, inv) in cases() {
            let mode = text(&inv["solver_mode"]).to_owned();
            let mutate = |f: &dyn Fn(&mut Value)| {
                let mut s = base.clone();
                f(&mut s);
                rehash(&mut s);
                refused(&s, &inv)
            };
            for severity in ["warning", "info"] {
                assert_eq!(mutate(&|s| failure_diagnostic(s)["severity"] = json!(severity)), fd, "{mode}: {severity}");
            }
            for code in ["NUMERICAL_INTEGRITY_CHECKS_PASSED", "NUMERICAL_INTEGRITY_SENSITIVE", "NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED"] {
                assert_eq!(mutate(&|s| failure_diagnostic(s)["code"] = json!(code)), fd, "{mode}: {code}");
            }
            assert_eq!(mutate(&|s| {
                let d = failure_diagnostic(s);
                d["severity"] = json!("warning");
                d["code"] = json!("NUMERICAL_INTEGRITY_CHECKS_PASSED");
            }), fd, "{mode}: downgraded and recoded");
            for message in [
                "Load case case: numerical integrity checks passed",
                "Load case supports: range: scaled evaluation outside normal range; range_scaling: attempted; step1_trigger=x; no structural rejection is bypassed by generic LU or output quantization",
                "Load case case: range: scaled evaluation outside normal range; no structural rejection is bypassed by generic LU or output quantization",
            ] {
                assert_eq!(mutate(&|s| failure_diagnostic(s)["message"] = json!(message)), fd, "{mode}: message {message}");
            }
            // A failed W2 that names no failure diagnostic is already the receipt's (G1).
            assert_eq!(mutate(&|s| {
                s["retained_precision"]["body"]["ordinary_attempts"][0]["w2"]["diagnostic_ref"] = Value::Null;
            }), ("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH".to_string()), "{mode}: failure diagnostic unnamed");
        }
    }
    #[test]
    fn success_quality_beside_a_failed_ordinary_case_refuses_by_name() {
        // Rulings (2) and (3) (g2 review F2): the case keeps the bound failure code's outcome tokens and
        // names its diagnostic; the envelope status is never a success token beside it.
        let fq = ("G5", "RETAINED_PRECISION_FAILURE_QUALITY_MISMATCH".to_string());
        for (base, inv) in cases() {
            let mode = text(&inv["solver_mode"]).to_owned();
            let mutate = |f: &dyn Fn(&mut Value)| {
                let mut s = base.clone();
                f(&mut s);
                rehash(&mut s);
                refused(&s, &inv)
            };
            for status in ["checks_passed", "sensitive", "not_assessed"] {
                assert_eq!(mutate(&|s| s["numerical_quality"]["status"] = json!(status)), fq, "{mode}: envelope {status}");
            }
            for (field, token) in [("solve_quality", "failed"), ("model_matrix_fidelity", "assembly_loss_detected"),
                ("model_matrix_fidelity", "represented_equations_retained"), ("accuracy_evidence", "not_claimed"),
                ("structural_status", "physical_mechanism_witnessed")] {
                assert_eq!(mutate(&|s| s["numerical_quality"]["cases"][0][field] = json!(token)), fq, "{mode}: {field}={token}");
            }
            assert_eq!(mutate(&|s| {
                s["numerical_quality"]["cases"][0]["evidence_refs"].as_array_mut().unwrap().retain(|r| r != "diagnostic:numerical-integrity:case");
            }), fq, "{mode}: failure diagnostic unnamed in quality");
        }
    }
    #[test]
    fn initial_failure_code_follows_the_producer_mapping() {
        let code = |e: Value| initial_failure_code(&e);
        assert_eq!(code(json!({"tag":"range","detail":"division overflow or underflow"})), "NUMERICAL_INTEGRITY_UNRESOLVED");
        assert_eq!(code(json!({"tag":"numerically_unresolved","detail":"positive diagonal contribution absorbed by assembly"})), "NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED");
        assert_eq!(code(json!({"tag":"numerically_unresolved","detail":"pivot"})), "NUMERICAL_INTEGRITY_UNRESOLVED");
        assert_eq!(code(json!({"tag":"negative_energy"})), "NUMERICAL_INTEGRITY_NEGATIVE_ENERGY");
        assert_eq!(code(json!({"tag":"mechanism"})), "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM");
        assert_eq!(code(json!({"tag":"asymmetric"})), "NUMERICAL_INTEGRITY_FAILED");
        assert_eq!(code(json!({"tag":"invalid_input"})), "NUMERICAL_INTEGRITY_FAILED");
    }
}

#[cfg(test)]
mod c7_prescribed_key_controls {
    use super::*;

    fn fixture(first: &str, second: &str, owned: bool, combination: bool) -> Value {
        let basis = json!({"ref_type":"case","id":"case"});
        let selection = json!({
            "input_derived_dofs":[{"node_id":first,"component":"UX"},{"node_id":second,"component":"RZ"}],
            "absolute_verified":[],"not_covered":[]
        });
        let mut document = json!({
            "results":[
                {"id":"row-x","kind":"global_nodal_displacement_x","entity_ref":first,"value":0.0,"unit":"m","basis_ref":basis},
                {"id":"row-rz","kind":"global_nodal_rotation_z","entity_ref":second,"value":0.0,"unit":"rad","basis_ref":basis}
            ],
            "retained_precision":{"body":{
                "sources":[{
                    "id_maps":{"nodes":[
                        {"id":first,"kernel_node":0,"coordinates":["0000000000000000","0000000000000000","0000000000000000"]},
                        {"id":second,"kernel_node":1,"coordinates":["3ff0000000000000","0000000000000000","0000000000000000"]}
                    ],"members":[],"support_ids":[]},
                    "constraints":[{"dof":{"node":0,"component":"UX"}},{"dof":{"node":1,"component":"RZ"}}],
                    "body_membership":[{"body":0,"nodes":[0,1]}]
                }],
                "cases":[{"status":"selected","source_ref":0,"product_attempt_ref":0,"basis_ref":basis,"selection":selection}],
                "product_attempts":[{"proof":{"summary_coverage":{}}}],"combinations":[]
            }}
        });
        if owned {
            document["retained_precision"]["body"]["sources"][0]["id_maps"]["nodes"][0]["kernel_node"] = json!(0.0);
        }
        if combination {
            let basis = json!({"ref_type":"combination","id":"blend"});
            let rows = document["results"].as_array().unwrap().clone();
            for mut row in rows {
                row["basis_ref"] = basis.clone();
                row["id"] = json!(format!("blend:{}", text(&row["id"])));
                document["results"].as_array_mut().unwrap().push(row);
            }
            document["retained_precision"]["body"]["sources"].as_array_mut().unwrap().push(json!({
                "representative_source_ref":0,"operands":[{"factor":"3ff0000000000000","source_ref":0},{"factor":"0000000000000000","source_ref":999}]
            }));
            document["retained_precision"]["body"]["combinations"].as_array_mut().unwrap().push(json!({
                "disposition":"retained_selected","source_ref":1,"product_attempt_ref":0,"basis_ref":basis,"selection":selection
            }));
        }
        document
    }

    fn outcome(document: &Value, owned: bool, combination: bool) -> Value {
        let normalized = integral_receipt(document);
        assert_eq!(matches!(&normalized, std::borrow::Cow::Owned(_)), owned);
        let root: &Value = &normalized;
        let cases = match numeric_cases(root) {
            Ok(cases) => cases,
            Err(e) => return json!({"gate":e.gate,"code":e.code,"detail":e.detail}),
        };
        assert_eq!(cases.len(), if combination {2} else {1});
        if combination {
            assert!(std::ptr::eq(cases[1].source, &root["retained_precision"]["body"]["sources"][0]));
            assert_eq!(cases[1].data_sources.len(), 1);
            assert!(std::ptr::eq(cases[1].data_sources[0], cases[1].source));
        }
        match g5c(&cases) {
            Ok(rows) => json!({"rows":rows.into_iter().map(|r|json!({
                "id":r.result_id,"basis_ref":r.basis_ref,"normalized_bits":format!("{:016x}",r.normalized_bits),
                "scale_bits":r.scale_bits.map(|x|format!("{x:016x}")),"class":format!("{:?}",r.class)
            })).collect::<Vec<_>>()}),
            Err(e) => json!({"gate":e.gate,"code":e.code,"detail":e.detail}),
        }
    }

    #[test]
    fn c7_equivalence_classification_error_bytes() {
        let ids = [
            ("ascii".to_string(), "other".to_string()),
            ("".to_string(), "empty-peer".to_string()),
            ("é".to_string(), "e\u{301}".to_string()),
            ("节点🦀".to_string(), "节点🦀-other".to_string()),
            ("a".repeat(127), "b".repeat(128)),
            ("é".repeat(64), "é".repeat(64)+"a"),
        ];
        for (id_index, (first, second)) in ids.iter().enumerate() {
            for owned in [false,true] {
                for combination in [false,true] {
                    let document = fixture(first, second, owned, combination);
                    let value = outcome(&document, owned, combination);
                    assert_eq!(value["rows"].as_array().unwrap_or_else(||panic!("fixture failed: {value}")).len(), if combination {4} else {2});
                    println!("C7_CASE ids-{id_index}-{owned}-{combination} {value}");
                }
            }
        }
        for owned in [false,true] {
            for alteration in ["reorder","duplicate","tamper-node","tamper-component","omit","malformed","minus-zero","nonzero","empty-set"] {
                let mut document = fixture("node-a", "node-b", owned, false);
                let selection = &mut document["retained_precision"]["body"]["cases"][0]["selection"]["input_derived_dofs"];
                match alteration {
                    "reorder" => selection.as_array_mut().unwrap().reverse(),
                    "duplicate" => { let duplicate=selection[0].clone(); selection.as_array_mut().unwrap().push(duplicate); },
                    "tamper-node" => selection[0]["node_id"] = json!("other-node"),
                    "tamper-component" => selection[0]["component"] = json!("UY"),
                    "omit" => { selection.as_array_mut().unwrap().pop(); },
                    "malformed" => selection[0]["component"] = Value::Null,
                    "minus-zero" => document["results"][0]["value"] = json!(-0.0),
                    "nonzero" => document["results"][0]["value"] = json!(1.0),
                    "empty-set" => {
                        *selection=json!([]);
                        document["retained_precision"]["body"]["sources"][0]["constraints"]=json!([]);
                        document["results"]=json!([{"id":"mode","kind":"linear_solver_mode_basis","entity_ref":"node-a","value":1,"unit":"1","basis_ref":{"ref_type":"case","id":"case"}}]);
                    },
                    _=>unreachable!(),
                }
                let value = outcome(&document, owned, false);
                if matches!(alteration,"reorder"|"empty-set") {
                    assert!(value.get("rows").is_some());
                } else {
                    assert_eq!(value,json!({"gate":"G5c","code":"RETAINED_PRECISION_INPUT_DOF_MISMATCH","detail":null}));
                }
                println!("C7_CASE {alteration}-{owned} {value}");
            }
        }
    }

    #[test]
    fn c7_equivalence_single_entry_and_unicode_distinction() {
        for owned in [false,true] {
            let mut document=fixture("é","peer",owned,false);
            document["retained_precision"]["body"]["sources"][0]["constraints"].as_array_mut().unwrap().truncate(1);
            document["retained_precision"]["body"]["cases"][0]["selection"]["input_derived_dofs"].as_array_mut().unwrap().truncate(1);
            document["results"].as_array_mut().unwrap().truncate(1);
            let normalized=integral_receipt(&document);
            let cases=numeric_cases(&normalized).unwrap();
            assert_eq!(cases[0].prescribed.len(),1);
            assert!(cases[0].prescribed.contains(&("é".into(),"UX".into())));
            assert!(!cases[0].prescribed.contains(&("e\u{301}".into(),"UX".into())));
            let value=outcome(&document,owned,false);
            assert_eq!(value["rows"].as_array().unwrap().len(),1);
            println!("C7_CASE single-unicode-{owned} {value}");
            drop(cases);
            drop(normalized);
            document["retained_precision"]["body"]["cases"][0]["selection"]["input_derived_dofs"][0]["node_id"]=json!("e\u{301}");
            let value=outcome(&document,owned,false);
            assert_eq!(value,json!({"gate":"G5c","code":"RETAINED_PRECISION_INPUT_DOF_MISMATCH","detail":null}));
            println!("C7_CASE unicode-same-component-tamper-{owned} {value}");
            document["retained_precision"]["body"]["cases"][0]["selection"]["input_derived_dofs"][0]["node_id"]=json!("é");
            document["results"][0]["entity_ref"]=json!("e\u{301}");
            let value=outcome(&document,owned,false);
            assert_eq!(value,json!({"gate":"G5c","code":"RETAINED_PRECISION_CLASSIFICATION_MISMATCH","detail":null}));
            println!("C7_CASE unicode-row-lookup-tamper-{owned} {value}");
        }
    }

    #[test]
    fn c7_borrowed_storage_and_positive_type_atoms() {
        for owned in [false,true] {
            let document=fixture("节点🦀","é",owned,true);
            let normalized=integral_receipt(&document);
            let cases=numeric_cases(&normalized).unwrap();
            for case in &cases {
                for (node,component) in &case.prescribed {
                    let constraint=list(&case.source["constraints"]).iter().find(|d|text(&d["dof"]["component"])==*component).unwrap();
                    let value=at(&case.source["id_maps"]["nodes"],&constraint["dof"]["node"],"G5a","SCALE_MISMATCH").unwrap();
                    assert_eq!(node.as_ptr(),text(&value["id"]).as_ptr());
                    assert_eq!(component.as_ptr(),text(&constraint["dof"]["component"]).as_ptr());
                }
            }
            assert_eq!(std::mem::size_of::<(&str,&str)>(),2*std::mem::size_of::<&str>());
            assert!(std::mem::size_of::<BTreeSet<(&str,&str)>>() > 0);
            println!("C7_ATOMS owned={owned} numeric_case={} set_header={} borrowed_pair={} value_ref={}",
                std::mem::size_of::<NumericCase<'_>>(),std::mem::size_of::<BTreeSet<(&str,&str)>>(),
                std::mem::size_of::<(&str,&str)>(),std::mem::size_of::<&Value>());
        }
    }
}

#[cfg(test)]
mod t4_u6_g8_tests {
    use super::*;

    /// O4 (DEL-04-08 Design `o4-source-extension.md` §4.7): physics-retained-1's
    /// G8 admits either shear model and any finite, nonnegative mill tolerance and
    /// explicit allowance with c_m + c_a < t; the mill-tolerance decline is retired.
    /// An unknown shear model, a negative, non-length, absent or over-wall allowance,
    /// or a negative mill tolerance stays outside the namespace.
    #[test]
    fn g8_namespace_admits_either_shear_model_and_nonzero_allowances() {
        let request: Value = serde_json::from_str(include_str!(
            "../../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json"
        ))
        .unwrap();
        let model = &request["model"];
        assert!(exact_namespace(model));
        let wall = unit_value(&model["pipe_segments"][0]["section"]["wall_thickness"], Dimension::Length).unwrap();
        for (key, value, admitted) in [
            ("mill_tolerance", json!({"value": 0.5, "unit": "mm"}), true),
            ("mill_tolerance", json!({"value": 0, "unit": "mm"}), true),
            ("mill_tolerance", Value::Null, true),
            ("mill_tolerance", json!({"value": -0.5, "unit": "mm"}), false),
            ("mill_tolerance", json!({"value": 1, "unit": "N"}), false),
            ("corrosion_allowance", json!({"value": 1.0, "unit": "mm"}), true),
            ("corrosion_allowance", json!({"value": -1.0, "unit": "mm"}), false),
            ("corrosion_allowance", json!({"value": 1, "unit": "N"}), false),
            ("corrosion_allowance", Value::Null, false),
            ("corrosion_allowance", json!({"value": wall, "unit": "m"}), false),
        ] {
            let mut changed = model.clone();
            changed["pipe_segments"][0]["section"][key] = value.clone();
            assert_eq!(exact_namespace(&changed), admitted, "{key} {value}");
        }
        for (shear, admitted) in [("timoshenko_annulus_energy_v1", true), ("euler_bernoulli", true), ("cowper", false)] {
            let mut changed = model.clone();
            changed["pressure_contract"]["shear_model"] = json!(shear);
            assert_eq!(exact_namespace(&changed), admitted, "{shear}");
        }
    }
}

#[cfg(test)]
mod w1_member_reader_gate_tests {
    use super::*;
    #[test]
    fn member_early_guard_preserves_old_namespace_and_bounds_source_forgeries() {
        let source=json!({"retained_precision":{"body":{"sources":[]}}});
        let mut loads=vec![json!({"target":{"type":"node"}});128];
        loads.push(json!({"target":{"type":"element"},"category":"distributed_force","dimension":"force_per_length"}));
        let inv=json!({"request":{"model":{"schema_version":"0.7.0","load_cases":[{"id":"c","primitive_loads":loads}]}}});
        let refuse=|s:&Value,i:&Value,exact| {
            let e=guard_member_g8(s,i,exact).unwrap_err();
            assert_eq!((e.gate,e.code.as_str()),("G8","RETAINED_PRECISION_PREPARATION_MISMATCH"));
        };
        refuse(&source,&inv,false); // source member terms omitted, valid new family
        let mut malformed=inv.clone();
        malformed["request"]["model"]["load_cases"][0]["primitive_loads"][128]["direction"]=json!("ux");
        malformed["request"]["model"]["load_cases"][0]["primitive_loads"][128]["extent"]=json!([0,1]);
        malformed["request"]["model"]["load_cases"][0]["primitive_loads"][128]["magnitude"]=json!({"value":1,"unit":"wrong-unit"});
        refuse(&source,&malformed,false); // later declines cannot evade the bound
        let mut legacy=inv.clone();legacy["request"]["model"]["schema_version"]=json!("0.3.0");
        legacy["request"]["model"]["pressure_contract"]=json!({"version":"1.0.0","mode":"legacy_pressure_v1"});
        guard_member_g8(&source,&legacy,false).unwrap();
        guard_member_g8(&source,&inv,true).unwrap(); // source-key-absent exact route
        let mut pressure=inv.clone();
        pressure["request"]["model"]["load_cases"][0]["primitive_loads"][128]["category"]=json!("pressure");
        pressure["request"]["model"]["load_cases"][0]["primitive_loads"][128]["dimension"]=json!("pressure");
        guard_member_g8(&source,&pressure,false).unwrap();
        let mut nodal=inv.clone();nodal["request"]["model"]["load_cases"][0]["primitive_loads"][128]["target"]["type"]=json!("node");
        guard_member_g8(&source,&nodal,false).unwrap();
        for marker in [Value::Null,json!([])] {
            let forged=json!({"retained_precision":{"body":{"sources":[{"member_load_terms":marker}]}}});
            refuse(&forged,&legacy,false);
            refuse(&forged,&inv,true);
            refuse(&forged,&nodal,false);
        }
    }
    fn fixture()->(Value,Value,Vec<Value>) {
        let source=json!({"id_maps":{"nodes":[{"coordinates":(["0000000000000000";3])},{"coordinates":["4000000000000000","0000000000000000","0000000000000000"]}],"members":[{"node_i":0,"node_j":1}]},"nodal_terms":[],"member_load_terms":[{"constructor_ordinal":0,"primitive_load_index":0,"source_id":"l","member":0,"intensity":["0000000000000000","4008000000000000","0000000000000000"]}]});
        let case=json!({"id":"c","primitive_loads":[{"id":"l","target":{"type":"element","pipe":"p"},"category":"distributed_force","dimension":"force_per_length","direction":"global_y","magnitude":{"value":3,"unit":"N/m"}}]});
        (source,case,vec![json!({"id":"p"})])
    }
    #[test]
    fn selected_member_case_binds_invocation_declared_problem_and_k4led() {
        let (source,case,pipes)=fixture();
        let digest=json!("5708a650839d856692fd6b9f50a2f43b66445a4886304becf9d1b1b21cc6e57e");
        verify_member_terms(&source,&case,&pipes,Some(&digest),false).unwrap();
        let error=verify_member_terms(&source,&case,&pipes,Some(&json!("0".repeat(64))),false).unwrap_err();
        assert_eq!((error.gate,error.code.as_str()),("G8","RETAINED_PRECISION_PREPARATION_MISMATCH"));
        let mut wrong_case=case.clone();wrong_case["primitive_loads"][0]["magnitude"]["value"]=json!(-3);
        assert!(verify_member_terms(&source,&wrong_case,&pipes,Some(&digest),false).is_err());
        for mutant in ["duplicate","omit","order","ordinal","intensity"] {
            let mut altered=source.clone();
            match mutant {
                "duplicate"=>{let t=altered["member_load_terms"][0].clone();altered["member_load_terms"].as_array_mut().unwrap().push(t);}
                "omit"=>{altered.as_object_mut().unwrap().remove("member_load_terms");}
                "order"|"ordinal"=>altered["member_load_terms"][0]["constructor_ordinal"]=json!(1),
                _=>altered["member_load_terms"][0]["intensity"][0]=json!("3ff0000000000000"),
            }
            assert!(verify_member_terms(&altered,&case,&pipes,Some(&digest),false).is_err(),"{mutant}");
        }
    }
    #[test]
    fn member_gates_keep_exact_route_extent_and_generated_load_declines() {
        let (source,case,pipes)=fixture();
        assert!(verify_member_terms(&source,&case,&pipes,None,true).is_err());
        let mut partial=case.clone();partial["primitive_loads"][0]["extent"]=json!([0,1]);
        assert!(verify_member_terms(&source,&partial,&pipes,None,false).is_err());
        let mut generated=case.clone();generated["primitive_loads"][0]["id"]=json!("load:self-weight:1:c:1:p");
        assert!(verify_member_terms(&source,&generated,&pipes,None,false).is_err());
    }
    #[test]
    fn operand_member_loads_reason_requires_an_actual_operand_even_if_zero() {
        let (mut source,_,_)=fixture();source["owner"]=json!({"kind":"case","case_id":"c"});
        source["member_load_terms"][0]["intensity"]=json!((["0000000000000000";3]));
        let mut body=json!({"sources":[source],"product_attempts":[],"combinations":[{"disposition":"retained_unavailable","expression":{"kind":"mechanics","terms":[{"case_id":"c"}]},"reason":{"cause":{"space":"combination","tag":"operand_member_loads"}}}]});
        g8_combinations(&body,&[]).unwrap();
        body["sources"][0].as_object_mut().unwrap().remove("member_load_terms");
        assert!(g8_combinations(&body,&[]).is_err());
    }
    #[test]
    fn selected_member_disclosure_is_required_exactly_on_member_cases() {
        let (source,_,_)=fixture();
        let mut document=json!({"retained_precision":{"body":{"sources":[source],"cases":[{"basis_ref":{"ref_id":"c"},"status":"selected","source_ref":0}],"combinations":[]}},"diagnostics":[{"id":"d","affected_refs":["c"],"code":"RETAINED_PRECISION_SELECTED","message":SELECTED_MEMBER_LOAD_MESSAGE}]});
        g4(&document).unwrap();
        document["diagnostics"][0]["message"]=json!("ordinary selected message");assert!(g4(&document).is_err());
        document["retained_precision"]["body"]["sources"][0].as_object_mut().unwrap().remove("member_load_terms");g4(&document).unwrap();
        document["diagnostics"][0]["message"]=json!(SELECTED_MEMBER_LOAD_MESSAGE);assert!(g4(&document).is_err());
    }
    #[test]
    fn loaded_maximum_projection_is_source_aware_and_publishes_si_stress() {
        let (mut s,_,_)=fixture();s["id_maps"]["members"][0]["id"]=json!("p");s["id_maps"]["members"][0]["kernel_member"]=json!(0);
        let owner=json!({"basis_ref":{"ref_type":"load_case","ref_id":"c"}});
        let row=json!({"kind":"pipe_elastic_normal_stress_maximum_v2","entity_ref":"p","unit":"MPa","value":2.0});
        assert!(owner_projected(&owner,&row,Some(&s),true));
        assert!(projection_matches_row(&row,&json!({"kind":"normal","value":bits(2_000_000.)})));
        let opposite=s["member_load_terms"][0].clone();s["member_load_terms"].as_array_mut().unwrap().push(opposite);
        s["member_load_terms"][1]["intensity"][1]=json!("c008000000000000");
        assert!(owner_projected(&owner,&row,Some(&s),true));
        assert!(!owner_projected(&owner,&row,Some(&s),false));
        let combination=json!({"basis_ref":{"ref_type":"combination","ref_id":"c"}});
        assert!(!owner_projected(&combination,&row,Some(&s),true));
        for load in s["member_load_terms"].as_array_mut().unwrap(){load["intensity"]=json!(["8000000000000000","0000000000000000","0000000000000000"]);}
        assert!(!owner_projected(&owner,&row,Some(&s),true));
        assert!(!owner_projected(&owner,&row,None,true));
    }
    fn exhausted_fixture()->(Value,Value,Value) {
        let (mut s,_,_)=fixture();s["owner"]=json!({"kind":"case"});s["id_maps"]["members"][0]["id"]=json!("p");s["id_maps"]["members"][0]["kernel_member"]=json!(0);
        let owner=json!({"basis_ref":{"ref_type":"load_case","ref_id":"c"},"status":"unavailable"});
        let mut upper=json!({"negative":false,"exponent":0,"limbs":vec!["0000000000000000";16]});upper["limbs"][15]=json!("8000000000000000");
        let lower=json!({"negative":false,"exponent":0,"limbs":vec!["0000000000000000";16]});
        let a=json!({"source_ref":0,"owner_ref":{"kind":"case"},"stages":{"proof_start":"completed","projection":"failed","maxima":"not_entered","values":"not_entered","aliases":"not_entered","certificate":"not_entered","observables":"not_entered","g5a":"not_entered"},"result":{"kind":"unavailable","error":{"kind":"proof","cause":{"kind":"association","detail":"loaded maximum search exhausted"}}},"proof":{"loaded_maximum_diagnostic":{"row":0,"member":0,"lower":lower,"upper":upper}}});
        let doc=json!({"producer":{"semantic_contract_id":CONTRACT_ID},"retained_precision":{"body":{"sources":[s]}},"results":[{"basis_ref":owner["basis_ref"],"entity_ref":"p","kind":"pipe_elastic_normal_stress_maximum_v2","unit":"MPa","value":1.0}]});
        (doc,a,owner)
    }
    #[test]
    fn exhausted_loaded_maximum_diagnostic_is_required_and_associated() {
        let (doc,a,owner)=exhausted_fixture();loaded_maximum_diagnostic(&doc,&a,&owner).unwrap();
        for fault in ["missing","cause","ready","member","row","later","route","nodal","combination"] {
            let(mut d,mut b)=(doc.clone(),a.clone());
            match fault {
                "missing"=>{b["proof"].as_object_mut().unwrap().remove("loaded_maximum_diagnostic");},
                "cause"=>b["result"]["error"]["cause"]["detail"]=json!("other failure"),
                "ready"=>b["result"]["kind"]=json!("ready"),
                "member"=>b["proof"]["loaded_maximum_diagnostic"]["member"]=json!(1),
                "row"=>b["proof"]["loaded_maximum_diagnostic"]["row"]=json!(1),
                "later"=>b["stages"]["maxima"]=json!("completed"),
                "route"=>d["producer"]["semantic_contract_id"]=json!(EXACT_CONTRACT_ID),
                "nodal"=>{d["retained_precision"]["body"]["sources"][0].as_object_mut().unwrap().remove("member_load_terms");},
                _=>b["owner_ref"]["kind"]=json!("combination"),
            }
            assert!(loaded_maximum_diagnostic(&d,&b,&owner).is_err(),"{fault}");
        }
    }
    #[test]
    fn diagnostic_endpoint_encoding_and_canonical_order_are_separate_gates() {
        let (doc,a,owner)=exhausted_fixture();let original=a["proof"]["loaded_maximum_diagnostic"]["upper"].clone();
        let spec=&schema()["$defs"]["LoadedMaximumEndpoint"];
        for value in [json!(0.5),json!(9_007_199_254_740_992u64),json!(-0.0)] {
            let mut v=original.clone();v["exponent"]=value;assert!(encoding(&v,spec).is_err());
        }
        for word in ["abcdef","FFFFFFFFFFFFFFFF","000000000000000g"] {
            let mut v=original.clone();v["limbs"][0]=json!(word);assert!(encoding(&v,spec).is_err());
        }
        for fault in ["negative","zero_exp","unnormalized","reversed"] {
            let mut b=a.clone();let d=&mut b["proof"]["loaded_maximum_diagnostic"];
            match fault {
                "negative"=>d["lower"]["negative"]=json!(true),
                "zero_exp"=>d["lower"]["exponent"]=json!(1),
                "unnormalized"=>d["upper"]["limbs"][15]=json!("4000000000000000"),
                _=>{d["lower"]=original.clone();d["lower"]["exponent"]=json!(1);},
            }
            assert!(loaded_maximum_diagnostic(&doc,&b,&owner).is_err(),"{fault}");
        }
        let mut b=a.clone();b["proof"]["loaded_maximum_diagnostic"]["upper"]["exponent"]=json!(SAFE);
        loaded_maximum_diagnostic(&doc,&b,&owner).unwrap(); // comparison is bounded even at safe-integer extremes
    }

}
