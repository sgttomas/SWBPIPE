//! The writer sidecar as the importer reads it (Design §15): the self-digest (si_writer `sidecar_canonical_bytes`),
//! the strict `_SidecarParser`, and the regeneration identity's comparison (`canonical_sidecar`,
//! `sidecar_difference`, `summed_entries`, `primitive_key`).

use crate::pyfmt::{dumps_sorted, py_float_repr, py_repr};
use crate::pystr::{match_sha256, rust_blank, take_chars, PyError};
use crate::rational::bigint_to_f64;
use crate::value::{py_eq, Dict, Value};
use num_bigint::BigInt;
use num_traits::{One, Signed};
use sha2::{Digest, Sha256};

pub const KEYED_MAPS: [&str; 7] =
    ["node_map", "element_map", "support_map", "load_case_map", "material_map", "section_map", "load_map"];
/// The current writer's format: 4, which is 3 with `section_map` (Design §15, Format 4: section identities).
pub const SIDECAR_FORMAT: i64 = 4;
pub const SIDECAR_FORMATS: [i64; 3] = [2, 3, SIDECAR_FORMAT];
/// The formats that carry the self-digest and may record springs.
pub const DIGESTED_FORMATS: [i64; 2] = [3, SIDECAR_FORMAT];
pub const SIDECAR_DIGEST_KEY: &str = "sidecar_sha256";
pub const DOFS: [&str; 6] = ["UX", "UY", "UZ", "RX", "RY", "RZ"];
pub const RIGID_FAMILIES: [&str; 4] = ["anchor", "guide", "line_stop", "vertical_support"];
pub const SPRING_FAMILY: &str = "spring";
pub const MATERIAL_ORIGINS: [&str; 2] = ["project_invented", "user_supplied"];
pub const LOAD_CATEGORIES: [&str; 5] =
    ["concentrated_force", "concentrated_moment", "weight", "thermal", "imposed_displacement"];
pub const SHEAR_MODELS: [&str; 2] = ["euler_bernoulli", "timoshenko_annulus_energy_v1"];
pub const CLOSURE_TRANSFERS: [&str; 2] = ["transfers_to_wall", "separately_supported_or_compensated"];
pub const RESTORABLE_DIRECTIONS: [&str; 27] = [
    "global_x", "GLOBAL_X", "global_y", "GLOBAL_Y", "global_z", "GLOBAL_Z", "rotation_x", "rotation_y", "rotation_z",
    "UX", "Ux", "ux", "UY", "Uy", "uy", "UZ", "Uz", "uz", "RX", "Rx", "rx", "RY", "Ry", "ry", "RZ", "Rz", "rz",
];

/// `_DIRECTION_AXIS[token]`: ('t' or 'r', axis index).
pub fn direction_axis(token: &str) -> Option<(char, usize)> {
    for (index, axis) in ["x", "y", "z"].iter().enumerate() {
        let upper = axis.to_uppercase();
        let translations = [format!("global_{axis}"), format!("GLOBAL_{upper}"), format!("U{upper}"), format!("U{axis}"), format!("u{axis}")];
        if translations.iter().any(|item| item == token) {
            return Some(('t', index));
        }
        let rotations = [format!("rotation_{axis}"), format!("R{upper}"), format!("R{axis}"), format!("r{axis}")];
        if rotations.iter().any(|item| item == token) {
            return Some(('r', index));
        }
    }
    None
}

/// `SidecarNotCanonical(path, reason)`.
#[derive(Clone, Debug)]
pub struct NotCanonical {
    pub path: String,
    pub reason: String,
}

fn child(path: &str, key: &str) -> String {
    crate::pyjson::json_path(path, key)
}

/// si_writer `_canonical_number`: ECMAScript's Number::toString from Python's repr digits.
pub fn canonical_number(value: f64) -> Result<String, PyError> {
    if value == 0.0 {
        return Ok("0".into());
    }
    let text = py_float_repr(value.abs())?;
    let (mantissa, exponent) = match text.find('e') {
        Some(at) => (&text[..at], text[at + 1..].parse::<i64>().expect("repr exponent")),
        None => (text.as_str(), 0),
    };
    let (whole, fraction) = match mantissa.find('.') {
        Some(at) => (&mantissa[..at], &mantissa[at + 1..]),
        None => (mantissa, ""),
    };
    let joined = format!("{whole}{fraction}");
    let significant = joined.trim_start_matches('0');
    let n = whole.len() as i64 + exponent - (joined.len() as i64 - significant.len() as i64);
    let digits = significant.trim_end_matches('0');
    let k = digits.len() as i64;
    let sign = if value < 0.0 { "-" } else { "" };
    Ok(if k <= n && n <= 21 {
        format!("{sign}{digits}{}", "0".repeat((n - k) as usize))
    } else if 0 < n && n <= 21 {
        format!("{sign}{}.{}", &digits[..n as usize], &digits[n as usize..])
    } else if -6 < n && n <= 0 {
        format!("{sign}0.{}{digits}", "0".repeat((-n) as usize))
    } else {
        let e = n - 1;
        let tail = if k > 1 { format!(".{}", &digits[1..]) } else { String::new() };
        format!("{sign}{}{tail}e{}{}", &digits[..1], if e >= 0 { "+" } else { "-" }, e.abs())
    })
}

fn jcs_text(value: &str, out: &mut String) {
    out.push('"');
    for c in value.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{8}' => out.push_str("\\b"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\u{c}' => out.push_str("\\f"),
            '\r' => out.push_str("\\r"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}

fn jcs_walk(value: &Value, path: &str, out: &mut String) -> Result<(), Result<NotCanonical, PyError>> {
    let fail = |reason: &str| Err(Ok(NotCanonical { path: path.to_string(), reason: reason.into() }));
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(true) => out.push_str("true"),
        Value::Bool(false) => out.push_str("false"),
        Value::Int(number) => {
            let exact = bigint_to_f64(number).filter(|&float| float.is_finite() && py_eq(&Value::Float(float), value));
            match exact {
                Some(float) => out.push_str(&canonical_number(float).map_err(Err)?),
                None => return fail("is an integer that is not exactly a binary64 number"),
            }
        }
        Value::Float(number) => {
            if !number.is_finite() {
                return fail("is not a finite number");
            }
            out.push_str(&canonical_number(*number).map_err(Err)?);
        }
        Value::Str(text) => jcs_text(text, out),
        Value::List(items) => {
            out.push('[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                jcs_walk(item, &format!("{path}[{index}]"), out)?;
            }
            out.push(']');
        }
        Value::Dict(dict) => {
            let mut keys: Vec<&String> = dict.keys().collect();
            keys.sort_by(|a, b| a.encode_utf16().cmp(b.encode_utf16()));
            out.push('{');
            for (index, key) in keys.into_iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                jcs_text(key, out);
                out.push(':');
                jcs_walk(dict.get(key).expect("key"), &child(path, key), out)?;
            }
            out.push('}');
        }
        Value::Bad => {
            let shown = if path.is_empty() { "sidecar" } else { path };
            return Err(Ok(NotCanonical { path: shown.into(), reason: "is a object, not a JSON value".into() }));
        }
    }
    Ok(())
}

/// si_writer `sidecar_canonical_bytes`.
pub fn sidecar_canonical_bytes(sidecar: &Value) -> Result<Vec<u8>, Result<NotCanonical, PyError>> {
    let Some(dict) = sidecar.as_dict() else {
        return Err(Ok(NotCanonical { path: "sidecar".into(), reason: "must be an object".into() }));
    };
    let mut body = Dict::new();
    for (key, value) in dict.iter() {
        if key != SIDECAR_DIGEST_KEY {
            body.insert(key.clone(), value.clone());
        }
    }
    let mut out = String::new();
    jcs_walk(&Value::Dict(body), "", &mut out)?;
    Ok(out.into_bytes())
}

/// si_writer `sidecar_self_digest`.
pub fn sidecar_self_digest(sidecar: &Value) -> Result<String, Result<NotCanonical, PyError>> {
    let bytes = sidecar_canonical_bytes(sidecar)?;
    Ok(hex(&Sha256::digest(&bytes)))
}

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// `runtime_support_findings`: what the solve refuses among (support_id, node, family, DOFs), in order.
pub fn runtime_support_findings(supports: &[(String, String, Option<String>, Vec<String>)]) -> Vec<(String, String)> {
    let allowed = |family: &str| -> Vec<&'static str> {
        match family {
            "anchor" => DOFS.to_vec(),
            "guide" | "line_stop" => DOFS[..3].to_vec(),
            _ => vec!["UZ"],
        }
    };
    let mut restrained: std::collections::HashSet<(String, String)> = std::collections::HashSet::new();
    let mut found = Vec::new();
    for (support_id, node, family, dofs) in supports {
        if family.as_deref() == Some(SPRING_FAMILY) {
            if dofs.len() != 1 {
                found.push((support_id.clone(), "a spring acts on exactly one DOF".to_string()));
            }
            continue;
        }
        let family = match family {
            Some(name) if RIGID_FAMILIES.contains(&name.as_str()) => name.clone(),
            _ => (if dofs.len() == DOFS.len() { "anchor" } else { "guide" }).to_string(),
        };
        if dofs.is_empty() {
            found.push((support_id.clone(), "a rigid support restrains no DOF".to_string()));
        }
        for dof in dofs {
            if !allowed(&family).contains(&dof.as_str()) {
                found.push((support_id.clone(), format!("{dof} is not a DOF a {family} restrains")));
            } else if restrained.contains(&(node.clone(), dof.clone())) {
                found.push((support_id.clone(), format!("{dof} at its node is already restrained by another support")));
            } else {
                restrained.insert((node.clone(), dof.clone()));
            }
        }
    }
    found
}

// ---------------------------------------------------------------------------
// the strict parser
// ---------------------------------------------------------------------------

#[derive(Clone)]
enum F {
    Text,
    Number,
    Count,
    Flag,
    Choice(Vec<Option<&'static str>>),
    Vector,
    Bend,
    BendComponent,
    Element,
    Support,
    Material,
    Section,
    SectionIdentity,
    TextOrNull,
    Target,
    Load,
    Contract,
    Region,
    Terminal,
    Pressure,
    Digest,
    Format,
    Items(Box<F>, Option<usize>, bool),
    Mapping(Box<F>),
}

fn choice(items: &[&'static str]) -> F {
    F::Choice(items.iter().map(|item| Some(*item)).collect())
}

fn choice_null(items: &[&'static str]) -> F {
    F::Choice(std::iter::once(None).chain(items.iter().map(|item| Some(*item))).collect())
}

/// `_SidecarParser`.
pub struct SidecarParser {
    pub errors: Vec<(String, String)>,
    pub current: bool,
    pub format_number: Option<i64>,
    failure: Option<PyError>,
}

impl Default for SidecarParser {
    fn default() -> Self {
        Self::new()
    }
}

impl SidecarParser {
    pub fn new() -> Self {
        SidecarParser { errors: Vec::new(), current: false, format_number: None, failure: None }
    }

    fn fail(&mut self, path: &str, reason: impl Into<String>) -> Value {
        self.errors.push((path.to_string(), reason.into()));
        Value::Bad
    }

    fn record(&mut self, value: &Value, path: &str, required: &[(&str, F)], optional: &[(&str, F)]) -> Value {
        let Some(dict) = value.as_dict() else { return self.fail(path, "must be an object") };
        let field = |key: &str| if path.is_empty() { key.to_string() } else { format!("{path}.{key}") };
        let mut out = Dict::new();
        for (key, item) in dict.iter() {
            let parser = required.iter().chain(optional.iter()).find(|(name, _)| *name == key.as_str()).map(|(_, f)| f.clone());
            match parser {
                None => {
                    self.fail(&field(key), "is not a field the writer records");
                }
                Some(parser) => {
                    let parsed = self.parse_field(&parser, item, &field(key));
                    out.insert(key.clone(), parsed);
                }
            }
        }
        for (key, _) in required {
            if !dict.contains_key(&key.to_string()) {
                self.fail(&field(key), "is missing");
            }
        }
        Value::Dict(out)
    }

    fn any_bad(entry: &Value) -> bool {
        match entry {
            Value::Bad => true,
            Value::Dict(dict) => dict.values().any(Value::is_bad),
            _ => false,
        }
    }

    fn parse_field(&mut self, parser: &F, value: &Value, path: &str) -> Value {
        match parser {
            F::Text => match value {
                Value::Str(text) if !rust_blank(text) => value.clone(),
                Value::Str(_) => self.fail(path, "must not be empty or only whitespace"),
                _ => self.fail(path, "must be a string"),
            },
            F::Number => match value {
                Value::Int(number) => match bigint_to_f64(number) {
                    Some(float) => Value::Float(float),
                    None => self.fail(path, "must be a finite binary64 number"),
                },
                Value::Float(number) if number.is_finite() => value.clone(),
                _ => self.fail(path, "must be a finite binary64 number"),
            },
            F::Count => match value {
                Value::Int(number) if !number.is_negative() && number <= &(BigInt::one() << 53usize) => value.clone(),
                _ => self.fail(path, "must be a non-negative integer no larger than 2^53 (exact in binary64)"),
            },
            F::Flag => match value {
                Value::Bool(_) => value.clone(),
                _ => self.fail(path, "must be true or false"),
            },
            F::Choice(allowed) => {
                let ok = match value {
                    Value::Null => allowed.contains(&None),
                    Value::Str(text) => allowed.iter().any(|item| *item == Some(text.as_str())),
                    _ => false,
                };
                if ok {
                    value.clone()
                } else {
                    let shown: Vec<&str> = allowed.iter().map(|item| item.unwrap_or("null")).collect();
                    self.fail(path, format!("must be one of {}", shown.join(", ")))
                }
            }
            F::Vector => self.record(value, path, &[("x", F::Number), ("y", F::Number), ("z", F::Number)], &[]),
            F::Bend => self.record(
                value,
                path,
                &[
                    ("component", F::Text),
                    ("radius_mm", F::Number),
                    ("angle_deg", F::Number),
                    ("flexibility_factor", F::Number),
                    ("tangent_intersection_mm", F::Items(Box::new(F::Number), Some(3), true)),
                    ("caepipe_row_node", F::Text),
                ],
                &[],
            ),
            F::BendComponent => self.record(value, path, &[("component", F::Text)], &[]),
            F::Element => self.element(value, path),
            F::Support => self.support(value, path),
            F::Material => {
                if self.current {
                    let mut origins = MATERIAL_ORIGINS.to_vec();
                    origins.push("undeclared");
                    self.record(value, path, &[("swbpipe_material", F::Text), ("origin", choice(&origins))], &[])
                } else {
                    self.record(
                        value,
                        path,
                        &[
                            ("swbpipe_material", F::Text),
                            ("weight_density_kgf_per_m3", F::Number),
                            ("poisson_ratio", F::Number),
                            ("elastic_modulus_n_per_mm2", F::Number),
                            ("thermal_expansion_mm_per_mm_per_c", F::Number),
                            ("temperatures_c", F::Items(Box::new(F::Number), None, true)),
                            ("elements", F::Items(Box::new(F::Text), None, true)),
                        ],
                        &[("origin", choice(&MATERIAL_ORIGINS))],
                    )
                }
            }
            F::Section => self.record(
                value,
                path,
                &[
                    ("outside_diameter_mm", F::Number),
                    ("wall_thickness_mm", F::Number),
                    ("elements", F::Items(Box::new(F::Text), None, true)),
                ],
                &[],
            ),
            // Format 4: the library section a PIPE record's pipes reference, or null for none (Design §15).
            F::SectionIdentity => self.record(value, path, &[("swbpipe_section", F::TextOrNull)], &[]),
            F::TextOrNull => match value {
                Value::Null => Value::Null,
                _ => self.parse_field(&F::Text, value, path),
            },
            F::Target => {
                if let Some(dict) = value.as_dict() {
                    let keys: std::collections::BTreeSet<&str> = dict.keys().map(String::as_str).collect();
                    let only = |names: &[&str]| keys.len() == names.len() && names.iter().all(|name| keys.contains(name));
                    if only(&["node"]) || only(&["pipe"]) {
                        let fields: Vec<(&str, F)> = dict.keys().map(|key| (key.as_str(), F::Text)).collect();
                        return self.record(value, path, &fields, &[]);
                    }
                    if only(&["support", "dof"]) {
                        return self.record(value, path, &[("support", F::Text), ("dof", choice(&DOFS))], &[]);
                    }
                }
                self.fail(path, "must be an object naming a node, a pipe, or a support and its DOF")
            }
            F::Load => self.load(value, path),
            F::Contract => {
                if value.as_str() == Some("none") {
                    return value.clone();
                }
                self.record(value, path, &[("version", choice(&["3.0.0"])), ("mode", choice(&["exact_pressure_v3"]))], &[])
            }
            F::Terminal => self.record(
                value,
                path,
                &[("node_ref", F::Text), ("closure_transfer", choice(&CLOSURE_TRANSFERS)), ("provenance", F::Text)],
                &[],
            ),
            F::Region => self.record(
                value,
                path,
                &[
                    ("id", F::Text),
                    ("members", F::Items(Box::new(F::Text), None, true)),
                    ("terminals", F::Items(Box::new(F::Terminal), Some(2), true)),
                ],
                &[],
            ),
            F::Pressure => self.pressure(value, path),
            F::Digest => {
                if (value.is_null() && !self.current) || matches!(value, Value::Str(text) if match_sha256(text)) {
                    value.clone()
                } else {
                    let tail = if self.current { "" } else { " or null" };
                    self.fail(path, format!("must be a lowercase SHA-256 hex digest{tail}"))
                }
            }
            F::Format => match value {
                Value::Int(number) if SIDECAR_FORMATS.iter().any(|format| &BigInt::from(*format) == number) => value.clone(),
                _ => self.fail(path, "must be 2 or 3 or 4, a format this importer reads (or absent, for a legacy sidecar)"),
            },
            F::Items(item, length, nonempty) => {
                let Some(items) = value.as_list() else { return self.fail(path, "must be a list") };
                if length.map(|wanted| items.len() != wanted).unwrap_or(false) || (*nonempty && items.is_empty()) {
                    let shown = match length {
                        Some(wanted) => wanted.to_string(),
                        None => "at least one".into(),
                    };
                    return self.fail(path, format!("must hold {shown} item(s)"));
                }
                Value::List(items.iter().enumerate().map(|(index, entry)| self.parse_field(item, entry, &format!("{path}[{index}]"))).collect())
            }
            F::Mapping(item) => {
                let Some(dict) = value.as_dict() else { return self.fail(path, "must be an object") };
                let mut out = Dict::new();
                for (key, entry) in dict.iter() {
                    let where_ = format!("{path}[{key}]");
                    if rust_blank(key) {
                        self.fail(&where_, "an id must be a non-empty string");
                        continue;
                    }
                    let parsed = self.parse_field(item, entry, &where_);
                    out.insert(key.clone(), parsed);
                }
                Value::Dict(out)
            }
        }
    }

    fn element(&mut self, value: &Value, path: &str) -> Value {
        let has_bend = value.has("bend");
        if self.current {
            let mut fields = vec![("from", F::Text), ("to", F::Text), ("reversed", F::Flag)];
            if has_bend {
                fields.push(("bend", F::BendComponent));
            } else {
                fields.push(("y_reference", F::Vector));
                fields.push(("y_reference_authored", F::Flag));
            }
            return self.record(value, path, &fields, &[]);
        }
        let mut fields = vec![
            ("from", F::Text),
            ("to", F::Text),
            ("reversed", F::Flag),
            ("layout_order", F::Count),
            ("layout_row", F::Count),
            ("material", F::Text),
            ("section", F::Text),
            ("load", F::Text),
        ];
        if has_bend {
            fields.push(("bend", F::Bend));
            return self.record(value, path, &fields, &[]);
        }
        self.record(value, path, &fields, &[("y_reference", F::Vector)])
    }

    fn support(&mut self, value: &Value, path: &str) -> Value {
        let restrained = ("restrained", F::Items(Box::new(choice(&DOFS)), None, true));
        let entry = if self.current {
            let mut families: Vec<&'static str> = RIGID_FAMILIES.to_vec();
            if self.format_number.is_some_and(|number| DIGESTED_FORMATS.contains(&number)) {
                families.push(SPRING_FAMILY);
            }
            self.record(
                value,
                path,
                &[("node", F::Text), restrained, ("family", choice(&families)), ("family_authored", F::Flag)],
                &[],
            )
        } else {
            self.record(
                value,
                path,
                &[("node", F::Text), restrained, ("caepipe_kind", choice(&["anchor", "restraint"]))],
                &[("family", choice_null(&RIGID_FAMILIES))],
            )
        };
        if Self::any_bad(&entry) || !entry.has("restrained") {
            return entry;
        }
        // The list keeps a refused DOF as the parser's sentinel, which counts in its length and its set.
        let dofs: Vec<Value> = entry.get("restrained").and_then(Value::as_list).cloned().unwrap_or_default();
        let mut unique: Vec<Option<&str>> = Vec::new();
        for item in &dofs {
            let key = item.as_str();
            if !unique.contains(&key) {
                unique.push(key);
            }
        }
        if unique.len() != dofs.len() {
            return self.fail(&format!("{path}.restrained"), "repeats a DOF");
        }
        let family = entry.get("family").and_then(Value::as_str).map(String::from);
        if family.as_deref() == Some("anchor") && dofs.len() != DOFS.len() {
            return self.fail(&format!("{path}.family"), "an anchor restrains all six DOFs, as the writer exports it");
        }
        if family.as_deref() == Some(SPRING_FAMILY) {
            if dofs.len() != 1 || !dofs[0].as_str().map(|dof| DOFS[..3].contains(&dof)).unwrap_or(false) {
                return self.fail(
                    &format!("{path}.restrained"),
                    "a spring acts on exactly one translation, as the writer exports it (a flexible anchor's TRAK)",
                );
            }
            if entry.get("family_authored").and_then(Value::as_bool) != Some(true) {
                return self.fail(&format!("{path}.family_authored"), "a spring's family is authored: the solve infers none");
            }
        }
        let inferred = if dofs.len() == DOFS.len() { "anchor" } else { "guide" };
        if self.current && entry.get("family_authored").and_then(Value::as_bool) == Some(false) && family.as_deref() != Some(inferred) {
            return self.fail(&format!("{path}.family"), format!("an unauthored family is the solve's inference, {inferred}"));
        }
        entry
    }

    fn load(&mut self, value: &Value, path: &str) -> Value {
        let entry = if self.current {
            self.record(
                value,
                path,
                &[
                    ("load_case", F::Text),
                    ("category", choice(&LOAD_CATEGORIES)),
                    ("target", F::Target),
                    ("direction", choice(&RESTORABLE_DIRECTIONS)),
                ],
                &[],
            )
        } else {
            let mut fields = vec![
                ("load_case", F::Text),
                ("caepipe_load_case", F::Text),
                ("category", choice(&LOAD_CATEGORIES)),
                ("target", F::Target),
                ("mbf", F::Text),
            ];
            let pipe_target = value.get("target").map(|target| target.has("pipe") && target.as_dict().is_some()).unwrap_or(false);
            if value.as_dict().is_some() && pipe_target {
                fields.push(("caepipe_element", F::Element));
            } else {
                fields.push(("caepipe_node", F::Text));
            }
            self.record(value, path, &fields, &[("direction", choice_null(&RESTORABLE_DIRECTIONS))])
        };
        if Self::any_bad(&entry) || !(entry.has("category") && entry.has("target")) {
            return entry;
        }
        let category = entry.get("category").and_then(Value::as_str).unwrap_or_default().to_string();
        let kind = target_kind(&category);
        if !entry.get("target").map(|target| target.has(kind)).unwrap_or(false) {
            return self.fail(&format!("{path}.target"), format!("must name a {kind}: a {category} load targets a {kind}"));
        }
        let wanted = match category.as_str() {
            "concentrated_force" => Some('t'),
            "concentrated_moment" => Some('r'),
            _ => None,
        };
        if let (Some(wanted), Some(Value::Str(direction))) = (wanted, entry.get("direction")) {
            if direction_axis(direction).map(|axis| axis.0) != Some(wanted) {
                let what = if wanted == 't' { "translation" } else { "rotation" };
                return self.fail(&format!("{path}.direction"), format!("must be a {what} direction for a {category} load"));
            }
        }
        entry
    }

    fn pressure(&mut self, value: &Value, path: &str) -> Value {
        if !self.current {
            return self.fail(
                path,
                "is not a field main's tranche-1 writer records; a sidecar with pressure has sidecar_format 2 or 3 or 4",
            );
        }
        let entry = self.record(
            value,
            path,
            &[
                ("contract", F::Contract),
                ("shear_model", choice(&SHEAR_MODELS)),
                ("regions", F::Items(Box::new(F::Region), None, false)),
            ],
            &[],
        );
        let complete = entry.as_dict().map(|dict| dict.len() == 3).unwrap_or(false);
        if entry.is_bad() || !complete || Self::any_bad(&entry) {
            return entry;
        }
        let none = entry.get("contract").and_then(Value::as_str) == Some("none");
        let euler = entry.get("shear_model").and_then(Value::as_str) == Some("euler_bernoulli");
        let regions = entry.get("regions").map(Value::truthy).unwrap_or(false);
        if none && (!euler || regions) {
            return self.fail(path, "without a contract the shear model is the route's euler_bernoulli and there is no region");
        }
        entry
    }

    /// `parse(sidecar)`: the typed copy, or None with `errors`.
    pub fn parse(&mut self, sidecar: &Value) -> Result<Option<Value>, PyError> {
        let maps: Vec<(&str, F)> = vec![
            ("node_map", F::Mapping(Box::new(F::Text))),
            ("element_map", F::Mapping(Box::new(F::Element))),
            ("support_map", F::Mapping(Box::new(F::Support))),
            ("load_case_map", F::Mapping(Box::new(F::Text))),
            ("material_map", F::Mapping(Box::new(F::Material))),
            ("section_map", F::Mapping(Box::new(F::Section))),
            ("load_map", F::Mapping(Box::new(F::Load))),
            ("pressure_map", F::Pressure),
        ];
        let Some(dict) = sidecar.as_dict() else {
            self.fail("sidecar", "must be an object");
            return Ok(None);
        };
        self.current = dict.contains_key(&"sidecar_format".to_string());
        let format_value = dict.get(&"sidecar_format".to_string());
        self.format_number = match format_value {
            Some(Value::Int(number)) if self.current => Some(crate::value::small_int(&Value::Int(number.clone())).unwrap_or(i64::MIN)),
            _ => None,
        };
        let parsed = if self.current {
            let mut fields: Vec<(&str, F)> = maps.iter().filter(|(name, _)| *name != "section_map").cloned().collect();
            fields.push(("mbf_sha256", F::Digest));
            fields.push(("sidecar_format", F::Format));
            if self.format_number.is_some_and(|number| DIGESTED_FORMATS.contains(&number)) {
                fields.push((SIDECAR_DIGEST_KEY, F::Digest));
            }
            if self.format_number == Some(SIDECAR_FORMAT) {
                fields.push(("section_map", F::Mapping(Box::new(F::SectionIdentity))));
            }
            self.record(sidecar, "", &fields, &[])
        } else {
            let fields: Vec<(&str, F)> = maps.iter().filter(|(name, _)| *name != "pressure_map").cloned().collect();
            self.record(sidecar, "", &fields, &[("mbf_sha256", F::Digest), ("pressure_map", F::Pressure)])
        };
        if let Some(error) = self.failure.take() {
            return Err(error);
        }
        if !self.errors.is_empty() {
            return Ok(None);
        }
        self.format_number = parsed.get("sidecar_format").and_then(crate::value::small_int);
        let mut out = Dict::new();
        for (name, _) in &maps {
            out.insert(name.to_string(), parsed.get(name).cloned().unwrap_or_else(|| Value::Dict(Dict::new())));
        }
        out.insert("mbf_sha256".into(), parsed.get("mbf_sha256").cloned().unwrap_or(Value::Null));
        let state = parsed.get("pressure_map").cloned();
        let pressure = match state {
            None => Value::Null,
            Some(state) if state.get("contract").and_then(Value::as_str) == Some("none") => Value::Null,
            Some(state) => {
                let mut contract = state.get("contract").and_then(Value::as_dict).cloned().unwrap_or_default();
                contract.insert("shear_model".into(), state.get("shear_model").cloned().unwrap_or(Value::Null));
                crate::dict![("contract", Value::Dict(contract)), ("regions", state.get("regions").cloned().unwrap_or(Value::Null))]
            }
        };
        out.insert("pressure_map".into(), pressure);
        let out = Value::Dict(out);
        // each load id is its case's and unique
        let mut seen: crate::value::OMap<String, String> = crate::value::OMap::new();
        let mut failures = Vec::new();
        if let Some(loads) = out.get("load_map").and_then(Value::as_dict) {
            for (key, entry) in loads.iter() {
                let prefix = format!("{}/", entry.get("load_case").and_then(Value::as_str).unwrap_or_default());
                let suffix = if key.starts_with(&prefix) { key[prefix.len()..].to_string() } else { String::new() };
                if rust_blank(&suffix) {
                    failures.push((format!("load_map[{key}]"), format!("the key must be '{prefix}<load id>' with a non-empty load id")));
                } else if let Some(previous) = seen.get(&suffix) {
                    failures.push((format!("load_map[{key}]"), format!("repeats the primitive load id of {previous} (DUPLICATE_ID)")));
                } else {
                    seen.insert(suffix, key.clone());
                }
            }
        }
        let supports: Vec<(String, String, Option<String>, Vec<String>)> = out
            .get("support_map")
            .and_then(Value::as_dict)
            .map(|map| {
                map.iter()
                    .map(|(support_id, entry)| {
                        (
                            support_id.clone(),
                            entry.get("node").and_then(Value::as_str).unwrap_or_default().to_string(),
                            entry.get("family").and_then(Value::as_str).map(String::from),
                            entry.get("restrained").and_then(Value::as_list).map(|items| items.iter().filter_map(|item| item.as_str().map(String::from)).collect()).unwrap_or_default(),
                        )
                    })
                    .collect()
            })
            .unwrap_or_default();
        for (support_id, reason) in runtime_support_findings(&supports) {
            failures.push((format!("support_map[{support_id}]"), format!("{reason}; the solve refuses it (SUPPORT_INPUT_INVALID)")));
        }
        let mut names: crate::value::OMap<String, String> = crate::value::OMap::new();
        if let Some(nodes) = out.get("node_map").and_then(Value::as_dict) {
            for (swb_id, name) in nodes.iter() {
                let name = name.as_str().unwrap_or_default().to_string();
                if let Some(previous) = names.get(&name) {
                    failures.push((format!("node_map[{swb_id}]"), format!("names CAEPIPE node {name} as {previous} does; it must be one-to-one")));
                }
                names.insert(name, swb_id.clone());
            }
        }
        if self.format_number == Some(SIDECAR_FORMAT) {
            let mut sections: crate::value::OMap<String, String> = crate::value::OMap::new();
            for (name, entry) in out.get("section_map").and_then(Value::as_dict).into_iter().flat_map(|map| map.iter()) {
                let Some(swb_id) = entry.get("swbpipe_section").and_then(Value::as_str) else { continue };
                if let Some(previous) = sections.get(&swb_id.to_string()) {
                    failures.push((
                        format!("section_map[{name}].swbpipe_section"),
                        format!("names section {swb_id} as PIPE record {previous} does; it must be one-to-one"),
                    ));
                } else {
                    sections.insert(swb_id.to_string(), name.clone());
                }
            }
        }
        for (path, reason) in failures {
            self.fail(&path, reason);
        }
        if !self.current {
            self.legacy_duplicates(&out)?;
        }
        Ok(if self.errors.is_empty() { Some(out) } else { None })
    }

    fn legacy_duplicates(&mut self, out: &Value) -> Result<(), PyError> {
        let empty = Dict::new();
        let loads = out.get("load_map").and_then(Value::as_dict).unwrap_or(&empty).clone();
        let cases = out.get("load_case_map").and_then(Value::as_dict).unwrap_or(&empty).clone();
        let elements = out.get("element_map").and_then(Value::as_dict).unwrap_or(&empty).clone();
        let nodes = out.get("node_map").and_then(Value::as_dict).unwrap_or(&empty).clone();
        let supports = out.get("support_map").and_then(Value::as_dict).unwrap_or(&empty).clone();
        for (key, entry) in loads.iter() {
            let where_ = format!("load_map[{key}]");
            let target = entry.get_or_null("target");
            let label = cases.get(&entry.get_or_null("load_case").as_str().unwrap_or_default().to_string()).cloned().unwrap_or(Value::Null);
            if !py_eq(entry.get_or_null("caepipe_load_case"), &label) {
                self.fail(&format!("{where_}.caepipe_load_case"), format!("must be its case's label in load_case_map ({})", py_repr(&label)?));
            }
            let element = if target.has("pipe") {
                elements.get(&target.get_or_null("pipe").as_str().unwrap_or_default().to_string()).cloned()
            } else {
                None
            };
            if target.has("pipe") {
                let given = element.clone().unwrap_or(Value::Null);
                if !py_eq(entry.get_or_null("caepipe_element"), &given) {
                    self.fail(
                        &format!("{where_}.caepipe_element"),
                        format!("must be element_map[{}], as the writer recorded it", py_str(target.get_or_null("pipe"))?),
                    );
                }
            } else {
                let node = if target.has("node") {
                    nodes.get(&target.get_or_null("node").as_str().unwrap_or_default().to_string()).cloned().unwrap_or(Value::Null)
                } else {
                    supports
                        .get(&target.get_or_null("support").as_str().unwrap_or_default().to_string())
                        .map(|support| support.get_or_null("node").clone())
                        .unwrap_or(Value::Null)
                };
                if !py_eq(entry.get_or_null("caepipe_node"), &node) {
                    self.fail(&format!("{where_}.caepipe_node"), format!("must be the CAEPIPE node of its target ({})", py_repr(&node)?));
                }
            }
            let category = entry.get_or_null("category").as_str().unwrap_or_default();
            let mut mbf = match category {
                "concentrated_force" => Value::str("FOR"),
                "concentrated_moment" => Value::str("MOM"),
                "imposed_displacement" => {
                    if py_str(target.get_or_null("dof"))?.starts_with('U') {
                        Value::str("D")
                    } else {
                        Value::str("ROT")
                    }
                }
                _ => Value::Null,
            };
            if let Some(element) = &element {
                mbf = if category == "thermal" {
                    Value::Str(format!("LOADS {} Temperature 1", py_str(element.get_or_null("load"))?))
                } else {
                    Value::Str(format!("MATERIAL {} weight density", py_str(element.get_or_null("material"))?))
                };
            }
            if !py_eq(entry.get_or_null("mbf"), &mbf) {
                self.fail(&format!("{where_}.mbf"), format!("must be the record main's writer wrote for it ({})", py_repr(&mbf)?));
            }
        }
        Ok(())
    }
}

/// `str(value)` for the values an f-string interpolates.
pub fn py_str(value: &Value) -> Result<String, PyError> {
    match value {
        Value::Str(text) => Ok(text.clone()),
        other => py_repr(other),
    }
}

pub fn target_kind(category: &str) -> &'static str {
    match category {
        "concentrated_force" | "concentrated_moment" => "node",
        "weight" | "thermal" => "pipe",
        _ => "support",
    }
}

// ---------------------------------------------------------------------------
// the regeneration identity's comparison
// ---------------------------------------------------------------------------

/// `canonical_sidecar(sidecar)`.
pub fn canonical_sidecar(sidecar: &Value) -> Value {
    fn walk(value: &Value) -> Value {
        match value {
            Value::Dict(dict) => {
                let mut out = Dict::new();
                for (key, item) in dict.iter() {
                    out.insert(key.clone(), walk(item));
                }
                Value::Dict(out)
            }
            Value::List(items) => Value::List(items.iter().map(walk).collect()),
            Value::Int(number) if number.abs() <= (BigInt::one() << 1023usize) => {
                Value::Float(bigint_to_f64(number).expect("within range"))
            }
            other => other.clone(),
        }
    }
    let mut top = Dict::new();
    if let Some(dict) = sidecar.as_dict() {
        for (key, value) in dict.iter() {
            if key != "mbf_sha256" && key != SIDECAR_DIGEST_KEY {
                top.insert(key.clone(), value.clone());
            }
        }
    }
    let mut out = walk(&Value::Dict(top));
    if let Some(Value::Dict(loads)) = out.as_dict_mut().and_then(|dict| dict.get_mut(&"load_map".to_string())) {
        for entry in loads.values_mut() {
            if let Some(dict) = entry.as_dict_mut() {
                if let Some(Value::Str(direction)) = dict.get(&"direction".to_string()) {
                    if let Some((kind, index)) = direction_axis(direction) {
                        let token = format!("{}{}", if kind == 't' { "U" } else { "R" }, ["X", "Y", "Z"][index]);
                        dict.insert("direction".into(), Value::Str(token));
                    }
                }
            }
        }
    }
    out
}

/// `summed_entries(id_map)`.
pub fn summed_entries(id_map: &Value) -> Vec<(String, Vec<String>)> {
    let mut out = Vec::new();
    if let Some(loads) = id_map.get("load_map").and_then(Value::as_dict) {
        for (load_id, entry) in loads.iter() {
            if let Some(Value::List(keys)) = entry.get("summed_entries") {
                let case = entry.get_or_null("case").as_str().unwrap_or_default();
                out.push((format!("{case}/{load_id}"), keys.iter().filter_map(|key| key.as_str().map(String::from)).collect()));
            }
        }
    }
    out
}

fn shown(value: &Value) -> Result<String, PyError> {
    let text = if value.is_bad() { "absent".to_string() } else { dumps_sorted(value)? };
    Ok(if text.chars().count() <= 160 { text } else { format!("{}...", take_chars(&text, 157)) })
}

fn same(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Null, Value::Null) | (Value::Bad, Value::Bad) => true,
        (Value::Bool(x), Value::Bool(y)) => x == y,
        (Value::Int(x), Value::Int(y)) => x == y,
        (Value::Float(x), Value::Float(y)) => x == y,
        (Value::Str(x), Value::Str(y)) => x == y,
        (Value::List(_), Value::List(_)) | (Value::Dict(_), Value::Dict(_)) => py_eq(a, b),
        _ => false,
    }
}

fn first_difference(a: &Value, b: &Value, path: &str) -> Result<Option<(String, String, String)>, PyError> {
    if let (Value::Dict(x), Value::Dict(y)) = (a, b) {
        let mut keys: Vec<&String> = x.keys().chain(y.keys()).collect();
        keys.sort();
        keys.dedup();
        for key in keys {
            let where_ = child(path, key);
            let left = x.get(key).unwrap_or(&Value::Bad);
            let right = y.get(key).unwrap_or(&Value::Bad);
            if let Some(found) = first_difference(left, right, &where_)? {
                return Ok(Some(found));
            }
        }
        return Ok(None);
    }
    if let (Value::List(x), Value::List(y)) = (a, b) {
        if x.len() == y.len() {
            for (index, (left, right)) in x.iter().zip(y).enumerate() {
                if let Some(found) = first_difference(left, right, &format!("{path}[{index}]"))? {
                    return Ok(Some(found));
                }
            }
            return Ok(None);
        }
    }
    if same(a, b) {
        Ok(None)
    } else {
        Ok(Some((path.to_string(), shown(a)?, shown(b)?)))
    }
}

/// `sidecar_difference(given, regenerated, summed)`.
pub fn sidecar_difference(given: &Value, regenerated: &Value, summed: &[(String, Vec<String>)]) -> Result<Option<(String, String, String)>, PyError> {
    let mut left = canonical_sidecar(given);
    let right = canonical_sidecar(regenerated);
    let again = right.get("load_map").cloned();
    for (formed, keys) in summed {
        let loads_is_dict = matches!(left.get("load_map"), Some(Value::Dict(_)));
        let Some(Value::Dict(again)) = &again else { break };
        if !loads_is_dict {
            break;
        }
        let loads = left.as_dict_mut().and_then(|dict| dict.get_mut(&"load_map".to_string())).and_then(Value::as_dict_mut).expect("dict");
        if loads.contains_key(formed) && !keys.contains(formed) {
            continue;
        }
        for key in keys {
            let a = loads.get(key).cloned().unwrap_or(Value::Bad);
            let b = again.get(formed).cloned().unwrap_or(Value::Bad);
            if let Some(found) = first_difference(&a, &b, &format!("load_map[{key}]"))? {
                return Ok(Some(found));
            }
        }
        let mut merged = Vec::new();
        for key in keys {
            if let Some(value) = loads.remove(key) {
                merged.push(value);
            }
        }
        if let Some(value) = merged.into_iter().next() {
            loads.insert(formed.clone(), value);
        }
    }
    first_difference(&left, &right, "")
}

/// A primitive's identity (primitive_key): (case, category, target kind, target id, axis).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PrimKey {
    pub case: String,
    pub category: String,
    pub kind: String,
    pub target: String,
    pub axis: Axis,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Axis {
    None,
    Star,
    Pair(char, usize),
}

impl PrimKey {
    /// `repr(key)`.
    pub fn repr(&self) -> String {
        let axis = match &self.axis {
            Axis::None => "None".to_string(),
            Axis::Star => "'*'".to_string(),
            Axis::Pair(kind, index) => format!("('{kind}', {index})"),
        };
        format!(
            "({}, {}, {}, {}, {})",
            crate::pystr::py_repr_str(&self.case),
            crate::pystr::py_repr_str(&self.category),
            crate::pystr::py_repr_str(&self.kind),
            crate::pystr::py_repr_str(&self.target),
            axis
        )
    }
    pub fn with_star(&self) -> PrimKey {
        PrimKey { axis: Axis::Star, ..self.clone() }
    }
}

/// `primitive_key(entry)` for a parsed sidecar load_map entry.
pub fn primitive_key(entry: &Value) -> PrimKey {
    let target = entry.get_or_null("target");
    let category = entry.get_or_null("category").as_str().unwrap_or_default().to_string();
    let kind = if target.has("support") {
        "support".to_string()
    } else {
        target.as_dict().and_then(|dict| dict.first().map(|(key, _)| key.clone())).unwrap_or_default()
    };
    let axis = match category.as_str() {
        "concentrated_force" | "concentrated_moment" => match entry.get("direction") {
            None | Some(Value::Null) => Axis::Star,
            Some(Value::Str(direction)) => direction_axis(direction).map(|(k, i)| Axis::Pair(k, i)).unwrap_or(Axis::None),
            _ => Axis::None,
        },
        "imposed_displacement" => match target.get("dof") {
            Some(Value::Str(dof)) => direction_axis(dof).map(|(k, i)| Axis::Pair(k, i)).unwrap_or(Axis::None),
            _ => Axis::None,
        },
        _ => Axis::None,
    };
    PrimKey {
        case: entry.get_or_null("load_case").as_str().unwrap_or_default().to_string(),
        category,
        kind: kind.clone(),
        target: target.get_or_null(&kind).as_str().unwrap_or_default().to_string(),
        axis,
    }
}
