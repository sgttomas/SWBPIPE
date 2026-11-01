//! C1 slice 2: the summary tables of a held result (DEL-04-05
//! `c1-native-custody-plan.md`, slice 2), ported from the TypeScript builders
//! that remain their oracle until the webview switches (slice 4):
//!
//! | Table | Builder (apps/desktop/src/features) |
//! |---|---|
//! | `support_load_cases`, `support_load_summaries` | `results/supportLoadSummary.ts` |
//! | `displacement_extremes` | `results/displacementExtremes.ts` (`buildDisplacementExtremes`) |
//! | `stress_ratio` (emitted stress groups) | `results/stressRatioTable.ts` (`buildStressRatioTable` without a rule-check run) |
//! | `component_bodies` | `results/componentBody.ts` (`componentBodies`) |
//! | `key_results` | `results/keyResults.ts` (`buildKeyResults` without a rule-check run; no `standingText`) |
//! | `withheld_support_state_rows` | `results/keyResults.ts` (`withheldSupportStateRows`) |
//! | `overlay_cases`, `colour_quantities` | `viewport/resultColourOverlay.ts` (`resultOverlayCases`, `availableResultColourQuantities` without a ratio table) |
//! | `results_panel` | `results/ResultsPanel.tsx` (`countResultFamilies`, `countResultFrames`), `results/resultsNavigator.ts` (`navigatorCases`) |
//!
//! Each port reads the parsed envelope as the TypeScript reads `JSON.parse`'s
//! output: absent and `null` members are told apart, comparisons are strict,
//! a tie keeps the first row, strings sort by UTF-16 code units, and nothing
//! is computed from a value except comparisons and magnitudes (no new
//! engineering quantity, unit conversion or rounding). Values, units and row
//! ids are copied unchanged. A table's shape is the builder's output shape,
//! so the parity leg (`resultSummaryParity.test.ts`) compares them directly.
//!
//! Not here, because they are session state or user choice rather than rows:
//! the rule-check run's groups and ratios (they look rows up by id: slice 3),
//! the numerical health's presentation text (`standingText`), and the
//! viewport overlays' per-choice drawing, which read the display planes
//! (`result_planes`).
use serde_json::{json, Map, Value};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

pub(crate) const RANGE_ENVELOPE_BASIS: &str = "explicit_user_range_envelope";
pub(crate) const REACTION_COMPONENT_KIND: &str = "support_reaction_component_v2";
pub(crate) const REACTION_FORCE_MAGNITUDE_KIND: &str = "support_reaction_force_magnitude_v2";
pub(crate) const REACTION_MOMENT_MAGNITUDE_KIND: &str = "support_reaction_moment_magnitude_v2";
pub(crate) const SUPPORT_STATE_ROW_KIND: &str = "nonlinear_support_active_set_state_code";
const SUPPORT_LOAD_FRAME: &str = "global";
const MECHANICS_SOLVED: &str = "MECHANICS_SOLVED";

// ---------------------------------------------------------------------------
// JavaScript's reading of a parsed JSON value
// ---------------------------------------------------------------------------

/// `row.metadata?.[key]`: absent unless the metadata is an object holding the key.
pub(crate) fn meta<'a>(row: &'a Value, key: &str) -> Option<&'a Value> {
    row.get("metadata").and_then(|metadata| metadata.get(key))
}

/// `Number.isFinite(value)`: a JSON number (serde_json reads none that is not finite).
pub(crate) fn finite(value: Option<&Value>) -> Option<f64> {
    match value {
        Some(Value::Number(number)) => number.as_f64(),
        _ => None,
    }
}

/// JavaScript truthiness.
pub(crate) fn truthy(value: Option<&Value>) -> bool {
    match value {
        None | Some(Value::Null) => false,
        Some(Value::Bool(flag)) => *flag,
        Some(Value::Number(number)) => number.as_f64().is_some_and(|x| x != 0.0),
        Some(Value::String(text)) => !text.is_empty(),
        Some(_) => true,
    }
}

/// `value === text`.
fn is_str(value: Option<&Value>, text: &str) -> bool {
    matches!(value, Some(Value::String(s)) if s == text)
}

/// `typeof value === "string" ? value : null`.
fn str_of(value: Option<&Value>) -> Option<&str> {
    match value {
        Some(Value::String(text)) => Some(text),
        _ => None,
    }
}

/// `Number.prototype.toString()` (radix 10).
pub(crate) fn js_number(x: f64) -> String {
    if x.is_nan() {
        return "NaN".into();
    }
    if x == 0.0 {
        return "0".into();
    }
    if x.is_infinite() {
        return if x > 0.0 { "Infinity".into() } else { "-Infinity".into() };
    }
    if x < 0.0 {
        return format!("-{}", js_number(-x));
    }
    // Rust's `{:e}` writes the shortest digits that read back to `x`, as the
    // ECMAScript algorithm chooses them.
    let text = format!("{x:e}");
    let (mantissa, exponent) = text.split_once('e').expect("{:e} has an exponent");
    let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
    let k = digits.len() as i32;
    let n = exponent.parse::<i32>().expect("an integer exponent") + 1;
    if k <= n && n <= 21 {
        format!("{digits}{}", "0".repeat((n - k) as usize))
    } else if 0 < n && n <= 21 {
        format!("{}.{}", &digits[..n as usize], &digits[n as usize..])
    } else if -6 < n && n <= 0 {
        format!("0.{}{digits}", "0".repeat((-n) as usize))
    } else {
        let sign = if n - 1 >= 0 { '+' } else { '-' };
        let exp = (n - 1).abs();
        if k == 1 {
            format!("{digits}e{sign}{exp}")
        } else {
            format!("{}.{}e{sign}{exp}", &digits[..1], &digits[1..])
        }
    }
}

/// `String(value)`, as a template literal writes it (`undefined` when absent).
pub(crate) fn js_string(value: Option<&Value>) -> Cow<'_, str> {
    match value {
        None => "undefined".into(),
        Some(Value::Null) => "null".into(),
        Some(Value::Bool(flag)) => flag.to_string().into(),
        Some(Value::Number(number)) => js_number(number.as_f64().unwrap_or(f64::NAN)).into(),
        Some(Value::String(text)) => Cow::Borrowed(text),
        Some(Value::Array(items)) => items
            .iter()
            .map(|item| match item {
                Value::Null => Cow::Borrowed(""),
                other => js_string(Some(other)),
            })
            .collect::<Vec<_>>()
            .join(",")
            .into(),
        Some(Value::Object(_)) => "[object Object]".into(),
    }
}

/// A value as a `Map` or `Set` key, or under `===`: strings by content,
/// numbers by value (0 and -0 alike), objects and arrays by identity.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum JsKey<'a> {
    Undefined,
    Null,
    Bool(bool),
    Number(u64),
    String(&'a str),
    Identity(usize),
}

impl std::hash::Hash for JsKey<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            JsKey::Undefined => state.write_u8(0),
            JsKey::Null => state.write_u8(1),
            JsKey::Bool(flag) => {
                state.write_u8(2);
                flag.hash(state);
            }
            JsKey::Number(bits) => {
                state.write_u8(3);
                bits.hash(state);
            }
            JsKey::String(text) => {
                state.write_u8(4);
                text.hash(state);
            }
            JsKey::Identity(address) => {
                state.write_u8(5);
                address.hash(state);
            }
        }
    }
}

pub(crate) fn js_key(value: Option<&Value>) -> JsKey<'_> {
    match value {
        None => JsKey::Undefined,
        Some(Value::Null) => JsKey::Null,
        Some(Value::Bool(flag)) => JsKey::Bool(*flag),
        Some(Value::Number(number)) => {
            let x = number.as_f64().unwrap_or(f64::NAN);
            JsKey::Number(if x == 0.0 { 0.0f64.to_bits() } else { x.to_bits() })
        }
        Some(Value::String(text)) => JsKey::String(text),
        Some(other) => JsKey::Identity(other as *const Value as usize),
    }
}

/// UTF-16 code unit order: JavaScript's string comparison and default sort.
pub(crate) fn utf16_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    a.encode_utf16().cmp(b.encode_utf16())
}

/// `[...values].sort()` of distinct values: by their string forms in UTF-16
/// order, `undefined` last.
fn js_default_sort<'a>(values: &mut [Option<&'a Value>]) {
    values.sort_by(|a, b| match (a, b) {
        (None, None) => std::cmp::Ordering::Equal,
        (None, _) => std::cmp::Ordering::Greater,
        (_, None) => std::cmp::Ordering::Less,
        (Some(_), Some(_)) => utf16_cmp(&js_string(*a), &js_string(*b)),
    });
}

/// `array.join(", ")`: `null` and `undefined` as empty.
fn js_join(values: &[Option<&Value>]) -> String {
    values
        .iter()
        .map(|value| match value {
            None | Some(Value::Null) => Cow::Borrowed(""),
            other => js_string(*other),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// Distinct values in first-seen order, as a `Set` keeps them.
fn distinct<'a>(values: impl IntoIterator<Item = Option<&'a Value>>) -> Vec<Option<&'a Value>> {
    let mut seen = HashSet::new();
    values.into_iter().filter(|value| seen.insert(js_key(*value))).collect()
}

/// JavaScript's `String.prototype.trim` white space: WhiteSpace and LineTerminator.
pub(crate) fn js_space(c: char) -> bool {
    matches!(
        c,
        '\u{9}' | '\u{a}' | '\u{b}' | '\u{c}' | '\u{d}' | ' ' | '\u{a0}' | '\u{1680}' | '\u{2000}'..='\u{200a}'
            | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}' | '\u{feff}'
    )
}

/// An object from members, leaving out the absent ones (as `JSON.stringify`
/// leaves out `undefined`).
fn object<'a>(members: impl IntoIterator<Item = (&'static str, Option<Value>)>) -> Value {
    let mut map = Map::new();
    for (key, value) in members {
        if let Some(value) = value {
            map.insert(key.into(), value);
        }
    }
    Value::Object(map)
}

fn copied(value: Option<&Value>) -> Option<Value> {
    value.cloned()
}

fn array(value: Option<&Value>) -> &[Value] {
    value.and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[])
}

/// `value || fallback`.
fn or<'a>(value: Option<&'a Value>, fallback: Option<&'a Value>) -> Option<&'a Value> {
    if truthy(value) {
        value
    } else {
        fallback
    }
}

/// `basisKey(basis)`: `ref_type:ref_id` of a truthy basis, else none.
pub(crate) fn basis_key(basis: Option<&Value>) -> Option<String> {
    truthy(basis).then(|| {
        let basis = basis.expect("truthy");
        format!("{}:{}", js_string(basis.get("ref_type")), js_string(basis.get("ref_id")))
    })
}

fn result_solved(source: &Value) -> bool {
    is_str(source.get("status").and_then(|status| status.get("mechanics")), MECHANICS_SOLVED)
}

pub(crate) fn rows(source: &Value) -> &[Value] {
    array(source.get("results"))
}

// ---------------------------------------------------------------------------
// The reader's standing and the numerical health, as the overlays read them
// ---------------------------------------------------------------------------

const HEALTH_STATES: [&str; 5] = ["checks_passed", "sensitive", "quality_unresolved", "solve_failed", "not_assessed"];
const HEALTH_ROUTES: [&str; 4] = ["generic", "pressure_source", "retained", "unsupported"];
const CAUSE_KINDS: [&str; 8] = [
    "conditioning",
    "load_fidelity",
    "formation",
    "formation_recovery",
    "curved_bend_short",
    "recovery_basis_unqualified",
    "mechanism",
    "negative_energy",
];

/// The parts of `numericalHealthView` the tables read: whether rows carry
/// badges (the generic route) and each case's and admitted combination's state.
#[derive(Debug)]
pub(crate) struct HealthView {
    row_badges: bool,
    cases: HashMap<String, &'static str>,
    combinations: HashMap<String, &'static str>,
}

/// `ResultOverlayStanding`: the reader's numerical standing of the result and
/// the health view of it, if any.
#[derive(Debug)]
pub(crate) struct Standing {
    pub(crate) eligible: bool,
    pub(crate) health: Option<HealthView>,
}

/// `numericalHealthView(parseNumericalHealth(value), …)`'s existence and the
/// states it reads: none for anything `parseNumericalHealth` refuses, and for
/// the retained and unsupported routes.
pub(crate) fn health_view(value: &Value) -> Option<HealthView> {
    fn field<'a>(o: &'a Map<String, Value>, key: &str) -> Option<&'a Value> {
        o.get(key)
    }
    fn nonempty(value: Option<&Value>) -> Option<&str> {
        str_of(value).filter(|s| !s.is_empty())
    }
    fn str_or_null(value: Option<&Value>) -> Option<()> {
        match value? {
            Value::Null => Some(()),
            other => nonempty(Some(other)).map(drop),
        }
    }
    fn state(value: Option<&Value>) -> Option<&'static str> {
        let text = str_of(value)?;
        HEALTH_STATES.iter().copied().find(|s| *s == text)
    }
    fn num_or_null(value: Option<&Value>) -> Option<Option<f64>> {
        match value? {
            Value::Null => Some(None),
            Value::Number(n) => n.as_f64().map(Some),
            _ => None,
        }
    }
    fn location(value: &Value) -> Option<()> {
        let o = value.as_object()?;
        match str_of(field(o, "kind"))? {
            "node_dof" => nonempty(field(o, "node")).and(nonempty(field(o, "dof"))).map(drop),
            "ref" => nonempty(field(o, "ref")).map(drop),
            "text" => nonempty(field(o, "text")).map(drop),
            _ => None,
        }
    }
    fn cause(value: &Value) -> Option<()> {
        let o = value.as_object()?;
        let kind = str_of(field(o, "kind"))?;
        CAUSE_KINDS.contains(&kind).then_some(())?;
        str_or_null(field(o, "diagnostic_id"))?;
        field(o, "locations")?.as_array()?.iter().try_for_each(location)
    }
    let o = value.as_object()?;
    let route = str_of(field(o, "route")).filter(|r| HEALTH_ROUTES.contains(r))?;
    let verdict_only = field(o, "verdict_only")?.as_bool()?;
    if verdict_only && route != "generic" {
        return None;
    }
    let mut cases = HashMap::new();
    for case in field(o, "cases")?.as_array()? {
        let c = case.as_object()?;
        let id = nonempty(field(c, "case_id"))?;
        let s = state(field(c, "state"))?;
        str_or_null(field(c, "code"))?;
        str_or_null(field(c, "integrity_diagnostic_id"))?;
        num_or_null(field(c, "rcond"))?;
        if let Some(digits) = num_or_null(field(c, "digits"))? {
            if digits < 0.0 {
                return None;
            }
        }
        field(c, "conditioning_sensitive")?.as_bool()?;
        field(c, "causes")?.as_array()?.iter().try_for_each(cause)?;
        if cases.insert(id.to_string(), s).is_some() {
            return None;
        }
    }
    let mut combinations = HashMap::new();
    let mut ids = HashSet::new();
    for combination in field(o, "combinations")?.as_array()? {
        let m = combination.as_object()?;
        let id = nonempty(field(m, "combination_id"))?;
        let s = match field(m, "state")? {
            Value::Null => None,
            other => Some(state(Some(other))?),
        };
        field(m, "cases")?.as_array()?.iter().try_for_each(|c| nonempty(Some(c)).map(drop))?;
        let withheld = field(m, "withheld")?.as_bool()?;
        if !ids.insert(id.to_string()) {
            return None;
        }
        // A withheld combination, or one with no state, gets no view entry.
        if let (false, Some(s)) = (withheld, s) {
            combinations.insert(id.to_string(), s);
        }
    }
    (route == "generic" || route == "pressure_source").then(|| HealthView { row_badges: route == "generic", cases, combinations })
}

/// `rowHealthState(health, { basis_ref })`.
fn row_health_state(health: &HealthView, basis: Option<&Value>) -> Option<&'static str> {
    if !health.row_badges || !truthy(basis) {
        return None;
    }
    let basis = basis.expect("truthy");
    let id = str_of(basis.get("ref_id"))?;
    match str_of(basis.get("ref_type")) {
        Some("load_case") => health.cases.get(id).copied(),
        Some("combination") => health.combinations.get(id).copied(),
        _ => None,
    }
}

/// `overlayMayShowRow(standing, { basis_ref })`.
pub(crate) fn overlay_may_show(standing: Option<&Standing>, basis: Option<&Value>) -> bool {
    let Some(standing) = standing else { return false };
    let Some(health) = standing.health.as_ref().filter(|_| standing.eligible) else { return false };
    !health.row_badges || row_health_state(health, basis) == Some("checks_passed")
}

pub(crate) const WITHHELD_NO_HEALTH: &str = "Withheld: numerical health is not available for this result.";
pub(crate) const WITHHELD_NOT_ELIGIBLE: &str = "Withheld: the result's numerical standing does not admit it.";
const WITHHELD_NO_STATE: &str = "Withheld: this case has no numerical health state.";

fn state_text(state: &str) -> &'static str {
    match state {
        "checks_passed" => "checks passed",
        "sensitive" => "sensitive",
        "quality_unresolved" => "quality unresolved",
        "solve_failed" => "solve failed",
        _ => "not assessed",
    }
}

/// `caseWithheldReason(standing, basis)`.
pub(crate) fn case_withheld_reason(standing: Option<&Standing>, basis: Option<&Value>) -> String {
    let Some(health) = standing.and_then(|s| s.health.as_ref()) else { return WITHHELD_NO_HEALTH.into() };
    if !standing.expect("has health").eligible {
        return WITHHELD_NOT_ELIGIBLE.into();
    }
    match row_health_state(health, basis) {
        Some(state) => format!("Withheld: numerical health {}.", state_text(state)),
        None => WITHHELD_NO_STATE.into(),
    }
}

// ---------------------------------------------------------------------------
// Cases
// ---------------------------------------------------------------------------

/// `SupportLoadCase`.
#[derive(Clone, Debug)]
pub(crate) struct Case<'a> {
    pub(crate) key: String,
    ref_type: &'static str,
    ref_id: Option<&'a Value>,
    /// The basis as the builders freeze it: `{ ref_type, ref_id }`.
    pub(crate) basis: Value,
    pub(crate) label: Option<Value>,
    pub(crate) envelope: bool,
}

impl Case<'_> {
    fn to_json(&self) -> Value {
        object([
            ("key", Some(json!(self.key))),
            ("basis", Some(self.basis.clone())),
            ("label", self.label.clone()),
            ("envelope", Some(json!(self.envelope))),
        ])
    }
}

fn frozen_basis(ref_type: &str, ref_id: Option<&Value>) -> Value {
    object([("ref_type", Some(json!(ref_type))), ("ref_id", copied(ref_id))])
}

fn case_ref_type(basis: Option<&Value>) -> Option<&'static str> {
    match str_of(basis?.get("ref_type")) {
        Some("load_case") => Some("load_case"),
        Some("combination") => Some("combination"),
        _ => None,
    }
}

/// A `Map` of a model array's items by `id` to one member (the last item wins).
fn by_id<'a>(items: &'a [Value], member: &str) -> HashMap<JsKey<'a>, Option<&'a Value>> {
    items.iter().map(|item| (js_key(item.get("id")), item.get(member))).collect()
}

/// Load cases first, then combinations, each in first-seen order.
fn load_cases_first<T>(cases: Vec<T>, ref_type: impl Fn(&T) -> &'static str) -> Vec<T> {
    let (load, combination): (Vec<T>, Vec<T>) = cases.into_iter().partition(|case| ref_type(case) == "load_case");
    load.into_iter().chain(combination).collect()
}

/// `supportLoadCases(model, result)`.
pub(crate) fn support_load_cases<'a>(model: &'a Value, source: &'a Value) -> Vec<Case<'a>> {
    let case_labels = by_id(array(model.get("load_cases")), "label");
    let combination_labels = by_id(array(model.get("combinations")), "label");
    let mut order: Vec<Case<'a>> = Vec::new();
    let mut seen: HashMap<String, usize> = HashMap::new();
    for row in rows(source) {
        let basis = row.get("basis_ref");
        let Some(ref_type) = truthy(basis).then(|| case_ref_type(basis)).flatten() else { continue };
        if finite(row.get("value")).is_none() {
            continue;
        }
        let key = basis_key(basis).expect("truthy");
        let index = *seen.entry(key.clone()).or_insert_with(|| {
            let ref_id = basis.and_then(|b| b.get("ref_id"));
            order.push(Case { key, ref_type, ref_id, basis: frozen_basis(ref_type, ref_id), label: None, envelope: false });
            order.len() - 1
        });
        if is_str(meta(row, "basis"), RANGE_ENVELOPE_BASIS) {
            order[index].envelope = true;
        }
    }
    for case in &mut order {
        let labels = if case.ref_type == "load_case" { &case_labels } else { &combination_labels };
        let own = labels.get(&js_key(case.ref_id)).copied().flatten();
        case.label = if truthy(own) {
            Some(json!(format!("{} ({})", js_string(own), js_string(case.ref_id))))
        } else {
            copied(case.ref_id)
        };
    }
    load_cases_first(order, |case| case.ref_type)
}

/// `ResultOverlayCase` of `resultOverlayCases(model, result)`.
pub(crate) fn overlay_cases(model: &Value, source: &Value) -> Vec<Value> {
    let labels = by_id(array(model.get("load_cases")), "label");
    let mut seen = HashSet::new();
    let mut cases = Vec::new();
    for row in rows(source) {
        if !drawable(row) {
            continue;
        }
        let basis = row.get("basis_ref");
        let Some(ref_type) = case_ref_type(basis) else { continue };
        let key = basis_key(basis).expect("drawable");
        if !seen.insert(key.clone()) {
            continue;
        }
        let ref_id = basis.and_then(|b| b.get("ref_id"));
        let own = (ref_type == "load_case").then(|| labels.get(&js_key(ref_id)).copied().flatten()).flatten();
        let label = if truthy(own) { Some(json!(format!("{} ({})", js_string(own), js_string(ref_id)))) } else { copied(ref_id) };
        cases.push((ref_type, object([("key", Some(json!(key))), ("basis", Some(frozen_basis(ref_type, ref_id))), ("label", label)])));
    }
    load_cases_first(cases, |case| case.0).into_iter().map(|case| case.1).collect()
}

/// `isDrawableRow`.
fn drawable(row: &Value) -> bool {
    finite(row.get("value")).is_some() && !is_str(meta(row, "basis"), RANGE_ENVELOPE_BASIS) && basis_key(row.get("basis_ref")).is_some()
}

// ---------------------------------------------------------------------------
// Results panel
// ---------------------------------------------------------------------------

pub(crate) const FAMILIES: [&str; 8] = ["displacement", "rotation", "reaction", "force", "moment", "stress", "ratio", "other"];

/// `countResultFamilies` from each row's family (`semanticLabels(result).family`).
pub(crate) fn family_counts<'a>(total: usize, families: impl IntoIterator<Item = &'a str>) -> Value {
    let mut counts = Map::new();
    counts.insert("total".into(), json!(total));
    for family in FAMILIES {
        counts.insert(family.into(), json!(0));
    }
    for family in families {
        // A family outside the table's vocabulary has no filter; the table
        // vocabulary is fixed, so this keeps the count rather than a NaN.
        let entry = counts.entry(family.to_string()).or_insert(json!(0));
        *entry = json!(entry.as_u64().unwrap_or(0) + 1);
    }
    Value::Object(counts)
}

const UNSTATED_FRAME: &str = "unstated";

/// `countResultFrames`: counts per frame, element local and global first,
/// then the others by name, unstated last, as `[frame, count]` pairs. The
/// others' order is `localeCompare`'s; it equals UTF-16 order for frames of
/// `a`–`z` and `_` only, which every producer frame is. `others_ordered` is
/// false when another frame occurs, and the consumer sorts those.
pub(crate) fn frame_counts(source: &Value) -> Value {
    let mut counts: Vec<(&str, u64)> = Vec::new();
    let mut index: HashMap<&str, usize> = HashMap::new();
    for row in rows(source) {
        let frame = result_frame(row);
        let at = *index.entry(frame).or_insert_with(|| {
            counts.push((frame, 0));
            counts.len() - 1
        });
        counts[at].1 += 1;
    }
    let class = |frame: &str| match frame {
        "element_local" => 0,
        "global" => 1,
        UNSTATED_FRAME => 3,
        _ => 2,
    };
    let ordered = counts.iter().all(|(frame, _)| class(frame) != 2 || frame.chars().all(|c| c == '_' || c.is_ascii_lowercase()));
    counts.sort_by(|a, b| class(a.0).cmp(&class(b.0)).then_with(|| if ordered { utf16_cmp(a.0, b.0) } else { std::cmp::Ordering::Equal }));
    json!({"counts": counts.iter().map(|(frame, count)| json!([frame, count])).collect::<Vec<_>>(), "others_ordered": ordered})
}

/// `resultFrame(item)`: the frame a row states (`metadata.coordinate_system`,
/// a string with a character that is not white space), else `unstated`.
pub(crate) fn result_frame(row: &Value) -> &str {
    match str_of(meta(row, "coordinate_system")) {
        Some(frame) if frame.chars().any(|c| !js_space(c)) => frame,
        _ => UNSTATED_FRAME,
    }
}

/// `navigatorCases(result.results)`.
pub(crate) fn navigator_cases(source: &Value) -> Vec<Value> {
    struct Entry<'a> {
        key: String,
        basis: Option<&'a Value>,
        envelope: bool,
    }
    let mut entries: Vec<Entry> = Vec::new();
    let mut seen: HashMap<String, usize> = HashMap::new();
    for row in rows(source) {
        let basis = row.get("basis_ref").filter(|b| truthy(Some(b)));
        let key = basis_key(basis).unwrap_or_else(|| "unstated".into());
        let at = *seen.entry(key.clone()).or_insert_with(|| {
            entries.push(Entry { key, basis, envelope: false });
            entries.len() - 1
        });
        if is_str(meta(row, "basis"), RANGE_ENVELOPE_BASIS) {
            entries[at].envelope = true;
        }
    }
    let rank = |basis: Option<&Value>| match basis {
        None => 3,
        Some(b) if is_str(b.get("ref_type"), "load_case") => 0,
        Some(b) if is_str(b.get("ref_type"), "combination") => 1,
        Some(_) => 2,
    };
    // A stable sort keeps first-seen order within a rank.
    entries.sort_by_key(|entry| rank(entry.basis));
    entries
        .into_iter()
        .map(|entry| {
            let basis = entry.basis.map(|b| object([("ref_type", copied(b.get("ref_type"))), ("ref_id", copied(b.get("ref_id")))]));
            let label = match entry.basis {
                None => "No case stated".to_string(),
                Some(b) if is_str(b.get("ref_type"), "load_case") => format!("Load case {}", js_string(b.get("ref_id"))),
                Some(b) if is_str(b.get("ref_type"), "combination") => format!("Combination {}", js_string(b.get("ref_id"))),
                Some(b) => format!("{} {}", js_string(b.get("ref_type")), js_string(b.get("ref_id"))),
            };
            let label = if entry.envelope { format!("{label} (range envelope)") } else { label };
            json!({"key": entry.key, "basis": basis.unwrap_or(Value::Null), "envelope": entry.envelope, "label": label})
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Support load summary (A2)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
enum Of {
    Support,
    Node,
}

/// `SUPPORT_LOAD_COLUMNS`: (id, kind, reaction component, of), in display order.
const COLUMNS: [(&str, &str, Option<&str>, Of); 14] = [
    ("Fx", REACTION_COMPONENT_KIND, Some("Fx"), Of::Support),
    ("Fy", REACTION_COMPONENT_KIND, Some("Fy"), Of::Support),
    ("Fz", REACTION_COMPONENT_KIND, Some("Fz"), Of::Support),
    ("F", REACTION_FORCE_MAGNITUDE_KIND, None, Of::Support),
    ("Mx", REACTION_COMPONENT_KIND, Some("Mx"), Of::Support),
    ("My", REACTION_COMPONENT_KIND, Some("My"), Of::Support),
    ("Mz", REACTION_COMPONENT_KIND, Some("Mz"), Of::Support),
    ("M", REACTION_MOMENT_MAGNITUDE_KIND, None, Of::Support),
    ("ux", "global_nodal_displacement_x", None, Of::Node),
    ("uy", "global_nodal_displacement_y", None, Of::Node),
    ("uz", "global_nodal_displacement_z", None, Of::Node),
    ("rx", "global_nodal_rotation_x", None, Of::Node),
    ("ry", "global_nodal_rotation_y", None, Of::Node),
    ("rz", "global_nodal_rotation_z", None, Of::Node),
];

/// `columnOf(row)`: the column index.
fn column_of(row: &Value) -> Option<usize> {
    let kind = str_of(row.get("kind"))?;
    if kind == REACTION_COMPONENT_KIND {
        let component = str_of(meta(row, "component")).unwrap_or("");
        return COLUMNS.iter().position(|c| c.1 == REACTION_COMPONENT_KIND && c.2 == Some(component));
    }
    COLUMNS.iter().position(|c| c.1 != REACTION_COMPONENT_KIND && c.1 == kind)
}

/// `SupportLoadValue` (also `DisplacementValue`).
fn emitted_value(row: &Value, value: f64) -> Value {
    object([("value", Some(json!(value))), ("unit", copied(row.get("unit"))), ("resultId", copied(row.get("id")))])
}

/// `withheldSupportAttributions(result, case)` as (support id, reason).
fn withheld_attributions<'a>(source: &'a Value, case: &Case) -> Vec<(&'a str, String)> {
    if case.ref_type != "load_case" {
        return Vec::new();
    }
    let preview = source.get("contract_evidence").and_then(|e| e.get("preview_cases"));
    let Some(cases) = preview.and_then(Value::as_array) else { return Vec::new() };
    let entry = cases
        .iter()
        .find(|item| (item.is_object() || item.is_array()) && js_key(item.get("load_case_id")) == js_key(case.ref_id));
    let withheld = entry.and_then(|e| e.get("support_attribution")).and_then(|a| a.get("withheld"));
    array(withheld)
        .iter()
        .filter_map(|item| {
            let support = str_of(item.get("support_id"))?;
            Some((support, str_of(item.get("reason")).unwrap_or("withheld").to_string()))
        })
        .collect()
}

type ByEntity<'a> = HashMap<JsKey<'a>, HashMap<String, Vec<&'a Value>>>;

fn support_summary<'a>(
    support: &'a Value,
    model: &'a Value,
    reactions: &ByEntity<'a>,
    movements: &ByEntity<'a>,
    attributions: &[HashMap<&'a str, String>],
    cases: &[Case<'a>],
    standing: Option<&Standing>,
) -> Value {
    let mut sign_convention: Option<Value> = None;
    let mut rows_out = Vec::new();
    let mut in_extremes: Vec<(usize, Vec<Option<Value>>)> = Vec::new();
    let mut exclusions = (0u64, 0u64, 0u64);
    for (case_index, case) in cases.iter().enumerate() {
        let empty = Vec::new();
        let reaction_rows = reactions.get(&js_key(support.get("id"))).and_then(|by| by.get(&case.key)).unwrap_or(&empty);
        let movement_rows = movements.get(&js_key(support.get("node"))).and_then(|by| by.get(&case.key)).unwrap_or(&empty);
        let attribution = support.get("id").and_then(Value::as_str).and_then(|id| attributions[case_index].get(id));
        if !overlay_may_show(standing, Some(&case.basis)) {
            exclusions.0 += 1;
            rows_out.push(json!({
                "case": case.to_json(), "standing": "withheld", "withheldReason": case_withheld_reason(standing, Some(&case.basis)),
                "reactionAbsence": null, "reactionAbsenceDetail": null, "values": {}, "inExtremes": false
            }));
            continue;
        }
        let mut values: Vec<Option<Value>> = vec![None; COLUMNS.len()];
        let other_frame = reaction_rows.iter().find(|row| !is_str(meta(row, "coordinate_system"), SUPPORT_LOAD_FRAME));
        for row in reaction_rows.iter().chain(movement_rows) {
            let Some(column) = column_of(row) else { continue };
            let of = COLUMNS[column].3;
            if of == Of::Support && other_frame.is_some() {
                continue;
            }
            if of == Of::Node && meta(row, "coordinate_system").is_some() && !is_str(meta(row, "coordinate_system"), SUPPORT_LOAD_FRAME) {
                continue;
            }
            if values[column].is_some() {
                continue;
            }
            values[column] = Some(emitted_value(row, finite(row.get("value")).expect("indexed rows are finite")));
            if of == Of::Support && sign_convention.is_none() && is_str(row.get("kind"), REACTION_COMPONENT_KIND) && truthy(meta(row, "sign_convention")) {
                sign_convention = copied(meta(row, "sign_convention"));
            }
        }
        let absence = if attribution.is_some() && reaction_rows.is_empty() {
            Some("attribution_withheld")
        } else if other_frame.is_some() {
            Some("frame_not_global")
        } else if reaction_rows.is_empty() {
            Some("no_rows")
        } else {
            None
        };
        let detail = match absence {
            Some("attribution_withheld") => json!(attribution.expect("withheld")),
            Some("frame_not_global") => match meta(other_frame.expect("other frame"), "coordinate_system") {
                None | Some(Value::Null) => json!("none stated"),
                Some(value) => value.clone(),
            },
            _ => Value::Null,
        };
        let listed = values.iter().any(Option::is_some);
        let extremes_row = !case.envelope && listed;
        if case.envelope {
            exclusions.1 += 1;
        } else if !extremes_row {
            exclusions.2 += 1;
        }
        let mut map = Map::new();
        for (column, value) in values.iter().enumerate() {
            if let Some(value) = value {
                map.insert(COLUMNS[column].0.into(), value.clone());
            }
        }
        rows_out.push(json!({
            "case": case.to_json(), "standing": "admitted", "withheldReason": null,
            "reactionAbsence": absence, "reactionAbsenceDetail": detail, "values": Value::Object(map), "inExtremes": extremes_row
        }));
        if extremes_row {
            in_extremes.push((case_index, values));
        }
    }
    let extremes: Vec<Value> = COLUMNS
        .iter()
        .enumerate()
        .map(|(column, spec)| {
            let listed: Vec<(usize, &Value)> =
                in_extremes.iter().filter_map(|(case_index, values)| values[column].as_ref().map(|v| (*case_index, v))).collect();
            extreme(spec.0, &listed, cases, "column", |units| format!("Units differ ({units}); no extreme is formed"))
        })
        .collect();
    let boundary = support.get("boundary_association").filter(|b| truthy(Some(b)));
    let equipment = boundary.map(|b| {
        object([
            ("kind", copied(b.get("kind"))),
            ("equipmentReference", copied(b.get("equipment_reference"))),
            ("nozzleReference", Some(b.get("nozzle_reference").filter(|v| !v.is_null()).cloned().unwrap_or(Value::Null))),
        ])
    });
    let interfaces: Vec<Value> = array(model.get("equipment_interfaces"))
        .iter()
        .filter(|item| js_key(item.get("node_ref")) == js_key(support.get("node")))
        .map(|item| copied(or(item.get("label"), item.get("id"))).unwrap_or(Value::Null))
        .collect();
    object([
        ("supportId", copied(support.get("id"))),
        ("label", copied(or(support.get("label"), support.get("id")))),
        ("nodeId", copied(support.get("node"))),
        ("equipment", Some(equipment.unwrap_or(Value::Null))),
        ("equipmentInterfaces", Some(json!(interfaces))),
        ("signConvention", Some(sign_convention.unwrap_or(Value::Null))),
        ("rows", Some(json!(rows_out))),
        ("extremes", Some(json!(extremes))),
        ("extremeRowCount", Some(json!(in_extremes.len()))),
        ("exclusions", Some(json!({"withheld": exclusions.0, "envelope": exclusions.1, "empty": exclusions.2}))),
    ])
}

/// The largest and smallest of `listed` (case index, `{value, unit, resultId}`)
/// in order, strict comparisons, unless their units differ.
fn extreme(id: &str, listed: &[(usize, &Value)], cases: &[Case], key: &str, note: impl Fn(&str) -> String) -> Value {
    let units = {
        let mut units = distinct(listed.iter().map(|(_, v)| v.get("unit")));
        js_default_sort(&mut units);
        units
    };
    let mut out = Map::new();
    if !key.is_empty() {
        out.insert(key.into(), json!(id));
    }
    if units.len() > 1 {
        out.insert("max".into(), Value::Null);
        out.insert("min".into(), Value::Null);
        out.insert("note".into(), json!(note(&js_join(&units))));
        return Value::Object(out);
    }
    let mut max: Option<(f64, usize, &Value)> = None;
    let mut min: Option<(f64, usize, &Value)> = None;
    for &(case_index, value) in listed {
        let x = value["value"].as_f64().expect("a listed value");
        if max.map_or(true, |m| x > m.0) {
            max = Some((x, case_index, value));
        }
        if min.map_or(true, |m| x < m.0) {
            min = Some((x, case_index, value));
        }
    }
    let with_case = |found: Option<(f64, usize, &Value)>| {
        found.map_or(Value::Null, |(_, case_index, value)| {
            let mut value = value.clone();
            let case = &cases[case_index];
            value["caseKey"] = json!(case.key);
            match &case.label {
                Some(label) => value["caseLabel"] = label.clone(),
                None => drop(value.as_object_mut().map(|o| o.remove("caseLabel"))),
            }
            value
        })
    };
    out.insert("max".into(), with_case(max));
    out.insert("min".into(), with_case(min));
    out.insert("note".into(), Value::Null);
    Value::Object(out)
}

fn index_rows<'a>(source: &'a Value) -> (ByEntity<'a>, ByEntity<'a>) {
    let mut reactions: ByEntity = HashMap::new();
    let mut movements: ByEntity = HashMap::new();
    for row in rows(source) {
        let Some(key) = basis_key(row.get("basis_ref")) else { continue };
        if finite(row.get("value")).is_none() {
            continue;
        }
        let target = column_of(row).map(|c| COLUMNS[c].3);
        let index = if target == Some(Of::Support) || is_str(row.get("kind"), REACTION_COMPONENT_KIND) {
            &mut reactions
        } else if target == Some(Of::Node) {
            &mut movements
        } else {
            continue;
        };
        index.entry(js_key(row.get("entity_ref"))).or_default().entry(key).or_default().push(row);
    }
    (reactions, movements)
}

fn unavailable_support_summaries(reason: &str) -> Value {
    json!({"state": "unavailable", "reason": reason, "frame": SUPPORT_LOAD_FRAME, "cases": [], "supports": []})
}

/// `buildSupportLoadSummaries({ model, result, standing })`.
pub(crate) fn support_load_summaries<'a>(model: &'a Value, source: &'a Value, cases: &[Case<'a>], standing: Option<&Standing>) -> Value {
    if !result_solved(source) {
        return unavailable_support_summaries("Mechanics not solved");
    }
    let supports = array(model.get("supports"));
    if supports.is_empty() {
        return unavailable_support_summaries("The model has no supports");
    }
    if cases.is_empty() {
        return unavailable_support_summaries("No load case or combination with result rows");
    }
    let (reactions, movements) = index_rows(source);
    let attributions: Vec<HashMap<&str, String>> = cases.iter().map(|case| withheld_attributions(source, case).into_iter().collect()).collect();
    json!({
        "state": "available", "reason": null, "frame": SUPPORT_LOAD_FRAME,
        "cases": cases.iter().map(Case::to_json).collect::<Vec<_>>(),
        "supports": supports.iter().map(|s| support_summary(s, model, &reactions, &movements, &attributions, cases, standing)).collect::<Vec<_>>()
    })
}

// ---------------------------------------------------------------------------
// Displacement extremes (A8)
// ---------------------------------------------------------------------------

const DISPLACEMENT_COLUMNS: std::ops::Range<usize> = 8..14;

/// `displacementExtremes.ts` `SUPPORT_FAMILIES`, in its order (model document
/// 0.9.0 added `snubber`).
const SUPPORT_FAMILIES: [(&str, &str); 10] = [
    ("anchor", "Anchor"),
    ("guide", "Guide"),
    ("line_stop", "Line stop"),
    ("vertical_support", "Vertical support"),
    ("spring", "Spring"),
    ("variable_spring_hanger", "Variable spring hanger"),
    ("spring_hanger", "Spring hanger"),
    ("constant_effort_support", "Constant-effort support"),
    ("nonlinear", "Nonlinear support"),
    ("snubber", "Snubber"),
];

fn family_label(family: Option<&str>) -> String {
    match family {
        None => "Family not stated".into(),
        Some(family) => match SUPPORT_FAMILIES.iter().find(|f| f.0 == family) {
            Some((_, label)) => label.to_string(),
            None => format!("{} (as stated)", serde_json::to_string(family).expect("a string")),
        },
    }
}

/// `isSpringHangerSupport` (`springHangerSupport.ts`): the trimmed explicit
/// hanger type, else the trimmed family, by Rust's own `str::trim`.
fn spring_hanger(support: &Value) -> bool {
    let explicit = str_of(support.get("hanger").and_then(|h| h.get("hanger_type"))).map(str::trim).unwrap_or("");
    let hanger_type = if !explicit.is_empty() { Some(explicit) } else { str_of(support.get("family")).map(str::trim) };
    matches!(hanger_type, Some("variable_spring_hanger" | "spring_hanger" | "constant_effort_support"))
}

/// `DisplacementSupport` (`supportOf`).
fn displacement_support(support: &Value) -> (Value, Option<String>, bool) {
    let family = str_of(support.get("family")).filter(|f| !f.is_empty());
    let nonlinear = support.get("nonlinear").filter(|n| family == Some("nonlinear") && truthy(Some(n)));
    let suffix = nonlinear.map_or(String::new(), |n| {
        let parts: Vec<String> = [n.get("behavior"), n.get("dof")].into_iter().filter(|v| truthy(*v)).map(|v| js_string(v).into_owned()).collect();
        format!(" ({})", parts.join(", "))
    });
    let hanger = spring_hanger(support);
    let value = object([
        ("supportId", copied(support.get("id"))),
        ("label", copied(or(support.get("label"), support.get("id")))),
        ("family", Some(json!(family))),
        ("kindText", Some(json!(format!("{}{suffix}", family_label(family))))),
        ("hanger", Some(json!(hanger))),
    ]);
    (value, family.map(String::from), hanger)
}

fn unavailable_displacements(reason: &str) -> Value {
    json!({"state": "unavailable", "reason": reason, "cases": [], "nodes": [], "filters": [], "unusedRows": {"otherFrame": 0, "notModelNode": 0}})
}

/// `buildDisplacementExtremes({ model, result, standing })`. An absent slot
/// of a node's values is `null` (`undefined` in the builder's sparse array).
pub(crate) fn displacement_extremes<'a>(model: &'a Value, source: &'a Value, all_cases: &[Case<'a>], standing: Option<&Standing>) -> Value {
    if !result_solved(source) {
        return unavailable_displacements("Mechanics not solved");
    }
    let nodes = array(model.get("nodes"));
    if nodes.is_empty() {
        return unavailable_displacements("The model has no nodes");
    }
    let mut row_counts: HashMap<String, u64> = HashMap::new();
    let mut movement: Vec<(&Value, String, usize)> = Vec::new();
    for row in rows(source) {
        let Some(component) = column_of(row).filter(|c| DISPLACEMENT_COLUMNS.contains(c)) else { continue };
        let Some(key) = basis_key(row.get("basis_ref")) else { continue };
        if finite(row.get("value")).is_none() || case_ref_type(row.get("basis_ref")).is_none() {
            continue;
        }
        *row_counts.entry(key.clone()).or_default() += 1;
        movement.push((row, key, component - DISPLACEMENT_COLUMNS.start));
    }
    let mut cases: Vec<(&Case, bool, bool)> = Vec::new();
    let mut cases_json = Vec::new();
    for case in all_cases.iter().filter(|case| row_counts.contains_key(&case.key)) {
        let admitted = overlay_may_show(standing, Some(&case.basis));
        let choosable = admitted && !case.envelope;
        let mut value = case.to_json();
        value["standing"] = json!(if admitted { "admitted" } else { "withheld" });
        value["withheldReason"] = if admitted { Value::Null } else { json!(case_withheld_reason(standing, Some(&case.basis))) };
        value["rowCount"] = json!(row_counts[&case.key]);
        value["choosable"] = json!(choosable);
        cases_json.push(value);
        cases.push((case, admitted, choosable));
    }
    if cases.is_empty() {
        return unavailable_displacements("No load case or combination with displacement or rotation rows");
    }
    let case_index: HashMap<&str, usize> = cases.iter().enumerate().map(|(i, c)| (c.0.key.as_str(), i)).collect();
    let node_index: HashMap<JsKey, usize> = nodes.iter().enumerate().map(|(i, n)| (js_key(n.get("id")), i)).collect();
    let mut values: Vec<[Vec<Value>; 6]> = nodes.iter().map(|_| std::array::from_fn(|_| vec![Value::Null; cases.len()])).collect();
    let (mut other_frame, mut not_model_node) = (0u64, 0u64);
    for (row, key, component) in movement {
        let Some(&at) = case_index.get(key.as_str()) else { continue };
        if !cases[at].2 {
            continue;
        }
        let Some(&node) = node_index.get(&js_key(row.get("entity_ref"))) else {
            not_model_node += 1;
            continue;
        };
        let frame = meta(row, "coordinate_system");
        if frame.is_some() && !is_str(frame, SUPPORT_LOAD_FRAME) {
            other_frame += 1;
            continue;
        }
        let slot = &mut values[node][component][at];
        if slot.is_null() {
            *slot = emitted_value(row, finite(row.get("value")).expect("finite"));
        }
    }
    let mut supports_by_node: HashMap<JsKey, Vec<(Value, Option<String>, bool)>> = HashMap::new();
    for support in array(model.get("supports")) {
        supports_by_node.entry(js_key(support.get("node"))).or_default().push(displacement_support(support));
    }
    let mut node_families: Vec<(Vec<Option<String>>, bool)> = Vec::new();
    let nodes_json: Vec<Value> = nodes
        .iter()
        .zip(values)
        .enumerate()
        .map(|(order, (node, slots))| {
            let supports = supports_by_node.get(&js_key(node.get("id"))).cloned().unwrap_or_default();
            node_families.push((supports.iter().map(|s| s.1.clone()).collect(), supports.iter().any(|s| s.2)));
            let mut map = Map::new();
            for (component, slot) in slots.into_iter().enumerate() {
                map.insert(COLUMNS[DISPLACEMENT_COLUMNS.start + component].0.into(), Value::Array(slot));
            }
            object([
                ("nodeId", copied(node.get("id"))),
                ("label", copied(or(node.get("label"), node.get("id")))),
                ("order", Some(json!(order))),
                ("supports", Some(Value::Array(supports.into_iter().map(|s| s.0).collect()))),
                ("values", Some(Value::Object(map))),
            ])
        })
        .collect();
    json!({
        "state": "available", "reason": null, "cases": cases_json, "nodes": nodes_json,
        "filters": displacement_filters(&node_families), "unusedRows": {"otherFrame": other_frame, "notModelNode": not_model_node}
    })
}

/// `filtersOf(nodes)` from each node's support families and hanger flag.
fn displacement_filters(nodes: &[(Vec<Option<String>>, bool)]) -> Value {
    let count = |test: &dyn Fn(&(Vec<Option<String>>, bool)) -> bool| nodes.iter().filter(|n| test(n)).count();
    let mut stated: Vec<Option<&str>> = Vec::new();
    for (families, _) in nodes {
        for family in families {
            if !stated.contains(&family.as_deref()) {
                stated.push(family.as_deref());
            }
        }
    }
    let known: Vec<&str> = SUPPORT_FAMILIES.iter().map(|f| f.0).filter(|v| stated.contains(&Some(*v))).collect();
    let unknown: Vec<&str> = stated.iter().filter_map(|v| *v).filter(|v| !SUPPORT_FAMILIES.iter().any(|f| f.0 == *v)).collect();
    let mut filters = vec![
        (json!("all"), json!("All nodes"), nodes.len()),
        (json!("supports"), json!("Support nodes"), count(&|n| !n.0.is_empty())),
        (json!("hangers"), json!("Hangers"), count(&|n| n.1)),
    ];
    for family in known.into_iter().chain(unknown) {
        filters.push((json!(format!("family:{family}")), json!(family_label(Some(family))), count(&|n| n.0.iter().any(|f| f.as_deref() == Some(family)))));
    }
    filters.push((json!("family-not-stated"), json!("Family not stated"), count(&|n| n.0.iter().any(Option::is_none))));
    Value::Array(
        filters
            .into_iter()
            .filter(|(id, _, count)| id == "all" || *count > 0)
            .map(|(id, label, count)| json!({"id": id, "label": label, "nodeCount": count}))
            .collect(),
    )
}

// ---------------------------------------------------------------------------
// Stress and ratio table (A3): emitted stress groups
// ---------------------------------------------------------------------------

/// `STRESS_KINDS`, in display order.
pub(crate) const STRESS_KINDS: [(&str, &str); 18] = [
    ("pipe_elastic_normal_stress_maximum_v2", "Normal-stress maximum"),
    ("element_local_normal_stress_extreme", "Section normal-stress extreme σ±"),
    ("branch_intensified_bending_stress_v1", "Intensified bending stress (tee)"),
    ("component_equal_factor_intensified_bending_stress_v1", "Intensified bending stress (component)"),
    ("curved_bend_directional_intensified_bending_maximum_v1", "Curved-bend intensified bending maximum"),
    ("curved_bend_directional_intensified_bending_stress_v1", "Curved-bend intensified bending stress"),
    ("element_local_axial_normal_stress", "Axial normal stress"),
    ("element_local_bending_normal_stress_y", "Bending normal stress, local y"),
    ("element_local_bending_normal_stress_z", "Bending normal stress, local z"),
    ("pipe_axial_membrane_stress_v2", "Axial membrane stress"),
    ("element_local_torsional_shear_stress", "Torsional shear stress"),
    ("element_local_transverse_shear_stress_maximum", "Transverse shear stress maximum"),
    ("pipe_section_pressure_hoop_stress", "Pressure hoop stress"),
    ("pipe_section_pressure_longitudinal_stress", "Pressure longitudinal stress"),
    ("pipe_lame_hoop_stress_v2", "Lamé hoop stress"),
    ("pipe_lame_radial_stress_v2", "Lamé radial stress"),
    ("pipe_toroidal_membrane_hoop_stress_v1", "Toroidal membrane hoop stress"),
    ("pipe_toroidal_radial_surface_stress_v1", "Toroidal radial surface stress"),
];

const SECTION_BASES: [&str; 2] = ["nominal_section", "reduced_section"];
const SECTION_BASIS_SEPARATOR: &str = "; section_basis=";

/// `sectionBasisOf(row)`.
pub(crate) fn section_basis_of(row: &Value) -> Option<&str> {
    let token = str_of(meta(row, "basis")).filter(|t| !t.is_empty())?;
    let at = token.rfind(SECTION_BASIS_SEPARATOR)?;
    let basis = &token[at + SECTION_BASIS_SEPARATOR.len()..];
    SECTION_BASES.contains(&basis).then_some(basis)
}

/// The cases by key, for [`stress_row`].
pub(crate) fn case_index<'c>(cases: &'c [Case]) -> HashMap<&'c str, usize> {
    cases.iter().enumerate().map(|(i, c)| (c.key.as_str(), i)).collect()
}

/// How `stressGroups` reads one row: `(kind index, case index, admitted,
/// value)` for a finite row of an emitted stress kind whose basis is a known
/// load case or combination, else none.
pub(crate) fn stress_row(row: &Value, cases: &HashMap<&str, usize>, standing: Option<&Standing>) -> Option<(usize, usize, bool, f64)> {
    let kind = str_of(row.get("kind")).and_then(|k| STRESS_KINDS.iter().position(|s| s.0 == k))?;
    let value = finite(row.get("value"))?;
    let basis = row.get("basis_ref");
    if !truthy(basis) || case_ref_type(basis).is_none() {
        return None;
    }
    let case = *cases.get(basis_key(basis).expect("truthy").as_str())?;
    Some((kind, case, overlay_may_show(standing, basis), value))
}

/// An admitted stress row's `caseLabel`: its case's label, with " (range
/// envelope)" for an envelope row.
pub(crate) fn stress_case_label(row: &Value, case: &Case) -> Value {
    if is_str(meta(row, "basis"), RANGE_ENVELOPE_BASIS) {
        json!(format!("{} (range envelope)", js_string(case.label.as_ref())))
    } else {
        case.label.clone().unwrap_or(Value::Null)
    }
}

/// The emitted stress groups of `stressGroups(result, cases, standing)`, in
/// `STRESS_KINDS` order: (kind index, admitted row count, withheld cases).
pub(crate) struct StressGroups {
    pub(crate) groups: Vec<(usize, usize, Vec<Value>)>,
}

/// None for a result whose mechanics are not solved (the table is unavailable).
pub(crate) fn stress_groups(source: &Value, cases: &[Case], standing: Option<&Standing>) -> StressGroups {
    if !result_solved(source) {
        return StressGroups { groups: Vec::new() };
    }
    let index = case_index(cases);
    let mut entries: Vec<Option<(usize, Vec<(usize, u64)>)>> = vec![None; STRESS_KINDS.len()];
    for row in rows(source) {
        let Some((kind, case, admitted, _)) = stress_row(row, &index, standing) else { continue };
        let entry = entries[kind].get_or_insert_with(|| (0, Vec::new()));
        if admitted {
            entry.0 += 1;
        } else {
            match entry.1.iter_mut().find(|held| held.0 == case) {
                Some(held) => held.1 += 1,
                None => entry.1.push((case, 1)),
            }
        }
    }
    let groups = entries
        .into_iter()
        .enumerate()
        .filter_map(|(kind, entry)| {
            let (count, mut withheld) = entry?;
            withheld.sort_by_key(|held| held.0);
            let withheld = withheld
                .into_iter()
                .map(|(case, rows)| {
                    let case = &cases[case];
                    json!({"case": case.to_json(), "rowCount": rows, "reason": case_withheld_reason(standing, Some(&case.basis))})
                })
                .collect();
            Some((kind, count, withheld))
        })
        .collect();
    StressGroups { groups }
}

/// `buildStressRatioTable` without a rule-check run: its header and groups,
/// each group's rows given by `rows` (`{ first, count }` in the stress planes).
pub(crate) fn stress_ratio(source: &Value, groups: &StressGroups) -> Value {
    let run = json!({"state": "none", "rulePackId": null, "outcomeCount": 0, "note": "No rule-check run in this session; emitted stresses are sorted by magnitude."});
    if !result_solved(source) {
        return json!({"state": "unavailable", "reason": "Mechanics not solved", "run": run, "groups": []});
    }
    if groups.groups.is_empty() {
        return json!({"state": "unavailable", "reason": "No emitted stress rows and no rule-check outcomes", "run": run, "groups": []});
    }
    let mut first = 0usize;
    let groups: Vec<Value> = groups
        .groups
        .iter()
        .map(|(kind, count, withheld)| {
            let (kind_id, label) = STRESS_KINDS[*kind];
            let count = *count;
            let group = json!({
                "id": format!("stress:{kind_id}"), "source": "emitted_stress", "label": label, "ref": kind_id,
                "modes": ["stress"], "withheldCases": withheld, "rows": {"first": first, "count": count}
            });
            first += count;
            group
        })
        .collect();
    json!({"state": "available", "reason": null, "run": run, "groups": groups})
}

// ---------------------------------------------------------------------------
// Component bodies, key results, support-state rows, colour quantities
// ---------------------------------------------------------------------------

/// `componentBodies(result)` as `[entity_ref, record]` pairs in insertion order.
pub(crate) fn component_bodies(source: &Value) -> Value {
    let mut seen = HashSet::new();
    let mut bodies = Vec::new();
    for row in rows(source) {
        let body = row.get("component_body");
        if truthy(body) && seen.insert(js_key(row.get("entity_ref"))) {
            bodies.push(json!([copied(row.get("entity_ref")), body]));
        }
    }
    Value::Array(bodies)
}

/// `withheldSupportStateRows(result, standing)`: row id (as a property key) to reason.
pub(crate) fn withheld_support_state_rows(source: &Value, standing: Option<&Standing>) -> Value {
    let mut marks = Map::new();
    for row in rows(source) {
        if is_str(row.get("kind"), SUPPORT_STATE_ROW_KIND) && !overlay_may_show(standing, row.get("basis_ref")) {
            marks.insert(js_string(row.get("id")).into_owned(), json!(case_withheld_reason(standing, row.get("basis_ref").filter(|b| !b.is_null()))));
        }
    }
    Value::Object(marks)
}

const KEY_RESULTS_NOT_SOLVED: &str = "Mechanics not solved, so no view is drawn. The results table lists the run's diagnostics.";
const KEY_RATIO_ABSENT: &str =
    "No rule pack: the model references none and no rule-check run of this result is held. Stress categories and ratios come only from your rule pack, so none is shown.";
const KEY_SUPPORTS_NONE: &str = "No support is declared as an anchor or equipment connection.";

fn empty_ratio() -> Value {
    json!({"state": "absent", "note": KEY_RATIO_ABSENT, "aggregate": null, "incompleteNote": null, "categories": [], "declaredWithoutOutcomes": []})
}

/// `model.rule_pack_refs?.length ?? 0`.
fn pack_count(model: &Value) -> usize {
    match model.get("rule_pack_refs") {
        Some(Value::Array(items)) => items.len(),
        Some(Value::String(text)) => text.encode_utf16().count(),
        _ => 0,
    }
}

/// `buildKeyResults({ model, result, standing })` with no rule-check run and
/// not historical, without `standingText` (the session's health presentation).
pub(crate) fn key_results(model: &Value, source: &Value, overlay_cases: &[Value], support_summaries: &Value) -> Value {
    if !result_solved(source) {
        return json!({
            "state": "not_solved", "reason": KEY_RESULTS_NOT_SOLVED, "ratio": empty_ratio(), "cases": [],
            "supports": {"reason": KEY_RESULTS_NOT_SOLVED, "items": []}, "contact": {"rowCount": 0, "caseCount": 0}
        });
    }
    let packs = pack_count(model);
    let ratio = if packs == 0 {
        empty_ratio()
    } else {
        json!({
            "aggregate": null, "incompleteNote": null, "categories": [], "declaredWithoutOutcomes": [], "state": "not_run",
            "note": format!("The model references {packs} rule pack{}, but no rule-check run of this result is held, so no stress category or ratio is shown.", if packs == 1 { "" } else { "s" })
        })
    };
    let supports = if support_summaries["state"] != "available" {
        json!({"reason": support_summaries["reason"], "items": []})
    } else {
        let families: HashMap<JsKey, Option<&Value>> = array(model.get("supports")).iter().map(|s| (js_key(s.get("id")), s.get("family"))).collect();
        let items: Vec<Value> = array(support_summaries.get("supports"))
            .iter()
            .filter_map(|summary| {
                let mut parts = Vec::new();
                if families.get(&js_key(summary.get("supportId"))).copied().flatten().is_some_and(|f| is_str(Some(f), "anchor")) {
                    parts.push("anchor");
                }
                if truthy(summary.get("equipment")) || !array(summary.get("equipmentInterfaces")).is_empty() {
                    parts.push("equipment connection");
                }
                (!parts.is_empty()).then(|| {
                    let label = summary.get("label");
                    object([
                        ("supportId", copied(summary.get("supportId"))),
                        ("label", copied(label)),
                        ("text", Some(json!(format!("{} ({})", js_string(label), parts.join(", "))))),
                    ])
                })
            })
            .collect();
        json!({"reason": if items.is_empty() { json!(KEY_SUPPORTS_NONE) } else { Value::Null }, "items": items})
    };
    let mut state_rows = 0u64;
    let mut state_cases = HashSet::new();
    for row in rows(source) {
        if is_str(row.get("kind"), SUPPORT_STATE_ROW_KIND) {
            state_rows += 1;
            state_cases.insert(basis_key(row.get("basis_ref")).unwrap_or_else(|| "unstated".into()));
        }
    }
    json!({
        "state": "available", "reason": null, "ratio": ratio, "cases": overlay_cases, "supports": supports,
        "contact": {"rowCount": state_rows, "caseCount": state_cases.len()}
    })
}

/// `RESULT_COLOUR_QUANTITIES`: (id, kind, attachment).
pub(crate) const COLOUR_QUANTITIES: [(&str, &str, &str); 12] = [
    ("displacement", "displacement_magnitude", "end_nodes"),
    ("normal-stress-maximum", "pipe_elastic_normal_stress_maximum_v2", "member"),
    ("normal-stress-extreme", "element_local_normal_stress_extreme", "member"),
    ("intensified-bending", "component_equal_factor_intensified_bending_stress_v1", "source_rows"),
    ("transverse-shear", "element_local_transverse_shear_stress_maximum", "member"),
    ("torsional-shear", "element_local_torsional_shear_stress", "member"),
    ("axial-force", "element_local_axial_force", "member"),
    ("shear-force-y", "element_local_shear_force_y", "member"),
    ("shear-force-z", "element_local_shear_force_z", "member"),
    ("torsional-moment", "element_local_torsional_moment", "member"),
    ("bending-moment-y", "element_local_bending_moment_y", "member"),
    ("bending-moment-z", "element_local_bending_moment_z", "member"),
];

/// The ids of `availableResultColourQuantities(model, result)` (no ratio table).
pub(crate) fn colour_quantities(model: &Value, source: &Value) -> Value {
    let pipes = array(model.get("pipe_segments"));
    let pipe_ids: HashSet<JsKey> = pipes.iter().map(|p| js_key(p.get("id"))).collect();
    let mut at_node: HashSet<JsKey> = HashSet::new();
    for pipe in pipes {
        at_node.insert(js_key(pipe.get("from")));
        at_node.insert(js_key(pipe.get("to")));
    }
    // `rowsById`, for the ids a source-rows quantity's rows name: the last row of each.
    let rows = rows(source);
    let mut wanted: HashSet<JsKey> = HashSet::new();
    for row in rows {
        if COLOUR_QUANTITIES.iter().any(|q| q.2 == "source_rows" && is_str(row.get("kind"), q.1)) {
            wanted.extend(array(row.get("source_result_refs")).iter().map(|r| js_key(Some(r))));
        }
    }
    let mut by_id: HashMap<JsKey, &Value> = HashMap::new();
    if !wanted.is_empty() {
        for row in rows {
            let id = js_key(row.get("id"));
            if wanted.contains(&id) {
                by_id.insert(id, row);
            }
        }
    }
    let attached = |row: &Value, attachment: &str| match attachment {
        "member" => pipe_ids.contains(&js_key(row.get("entity_ref"))),
        "end_nodes" => at_node.contains(&js_key(row.get("entity_ref"))),
        _ => array(row.get("source_result_refs"))
            .iter()
            .any(|r| by_id.get(&js_key(Some(r))).is_some_and(|s| pipe_ids.contains(&js_key(s.get("entity_ref"))))),
    };
    Value::Array(
        COLOUR_QUANTITIES
            .iter()
            .filter(|(_, kind, attachment)| rows.iter().any(|row| is_str(row.get("kind"), kind) && drawable(row) && attached(row, attachment)))
            .map(|(id, _, _)| json!(id))
            .collect(),
    )
}
