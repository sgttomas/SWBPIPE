//! Numerical health indicator (DEL-07-05 Design `numerical-health-indicator.md`):
//! a presentation projection of the ordinary solve's existing diagnostics, per
//! load case, for the desktop. Nothing here changes a state, a threshold, a
//! reader verdict or output admission; it reads the published envelope only.
//!
//! - The state follows `numerical_quality.cases[].solve_quality` (§3).
//! - The conditioning figure is read from the integrity diagnostic's
//!   `StructuralReport` text under the exact parse contract of §2.6, for linear
//!   cases only (§2 decision 4); any clause that fails leaves the figure absent
//!   and the state unchanged.
//! - Causes are emitted only where their evidence is present (§3); every
//!   location is parsed strictly and omitted rather than guessed.
//! - `verdict_only` is §2.6's result-level projection; it never takes part in
//!   admission or standing.
//!
//! Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
use open_pipe_stress_product_physics::result_export::reader_verdict as rv;
use serde::Serialize;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

const POLICY: &str = "M03-INTEGRITY-v1";
const INTEGRITY_ID_PREFIX: &str = "diagnostic:numerical-integrity:";
const CHECKS_PASSED: &str = "NUMERICAL_INTEGRITY_CHECKS_PASSED";
const SENSITIVE: &str = "NUMERICAL_INTEGRITY_SENSITIVE";
const RECOVERY_BASIS_UNQUALIFIED: &str = "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED";
const PHYSICAL_MECHANISM: &str = "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM";
const NEGATIVE_ENERGY: &str = "NUMERICAL_INTEGRITY_NEGATIVE_ENERGY";
const LOAD_CONTRIBUTION_ABSORBED: &str = "LOAD_CONTRIBUTION_ABSORBED";
const CURVED_BEND_SHORT: &str = "CURVED_BEND_SHORT_RELATIVE_TO_NEIGHBOURS";
const NONLINEAR_LOOP_PREFIX: &str = "NONLINEAR_SUPPORT_LOOP_";
const NOT_QUALIFIED: &str = "NUMERICAL_INTEGRITY_NOT_QUALIFIED";
const EVIDENCE_INCOMPLETE: &str = "NUMERICAL_CASE_EVIDENCE_INCOMPLETE";

/// `StructuralReport`'s Debug fields around rcond (§2.6).
const RCOND_FIELD: &str = ", reciprocal_condition_estimate: ";
const RESIDUAL_FIELD: &str = ", residual_rows: ";
/// The end of the report span (`append_integrity_report`).
const DOF_MAP_FIELD: &str = "; global_dof_map=";
/// The linear report's fixed sentence after the DOF map
/// (`append_integrity_report` with no equilibrium evidence).
const LINEAR_TEXT: &str = "The contribution audit distinguishes intended assembly from stored equations; physical formulation limitations remain applicable.";
const DOF_NAMES: [&str; 6] = ["UX", "UY", "UZ", "RX", "RY", "RZ"];

/// K-D5's evidence line (`formation_check_evidence_line`).
const KD5_MARK: &str = " formation_check: reason=";
const KD5_ESTIMATE: &str = " formation_check: reason=estimate; row=";
const KD5_ROW_END: &str = "; doubled_correction=";
/// S11-G sentences (`formation_guard::load_row_finding`, `recovery_finding`).
const LOAD_ROW_TAIL: &str = " load row(s) carry formed-term formation noise above the 1e-9 criterion; every value is kept for inspection. Rows: [";
const LOAD_ROW_REASONS: [&str; 3] = ["range failure (", "no conservative formation bound (", "net formation defect "];
const RECOVERY_TAIL: &str = " member or support group(s) of the certified rows exceed the 1e-9 member-relative criterion or have an unavailable comparison; every value is kept for inspection. Groups: [";
const RECOVERY_REASONS: [&str; 3] = [": q=", ": member comparison unavailable (", ": the member bound is unavailable ("];
/// The guard's named-row limit (`formation_guard::NAMED`).
const NAMED: usize = 6;
/// Recovery-guard demotions without named groups (product_physics: the
/// member-relative check unavailable, or a nonlinear state not established).
const RECOVERY_UNAVAILABLE: [&str; 2] = [
    " the member-relative formation check is not available on this route; member rows are binary64 recoveries",
    " the converged support state is not established at 128-bit precision (",
];
const RECOVERY_UNAVAILABLE_END: &str = "; member rows are binary64 recoveries";

#[derive(Debug, Serialize)]
struct Health {
    route: &'static str,
    verdict_only: bool,
    cases: Vec<CaseHealth>,
    combinations: Vec<CombinationHealth>,
}

#[derive(Debug, Serialize)]
struct CaseHealth {
    case_id: String,
    state: &'static str,
    code: Option<String>,
    integrity_diagnostic_id: Option<String>,
    rcond: Option<f64>,
    digits: Option<f64>,
    conditioning_sensitive: bool,
    causes: Vec<Cause>,
}

#[derive(Debug, Serialize)]
struct Cause {
    kind: &'static str,
    diagnostic_id: Option<String>,
    locations: Vec<Location>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum Location {
    NodeDof { node: String, dof: String },
    Ref {
        #[serde(rename = "ref")]
        reference: String,
    },
    Text { text: String },
}

#[derive(Debug, Serialize)]
struct CombinationHealth {
    combination_id: String,
    state: &'static str,
    cases: Vec<String>,
    withheld: bool,
}

/// The numerical health of a result envelope, as JSON text (design §2.6).
/// `reader_findings_json` is the Rust reader's findings for the same result:
/// an array of finding codes (or of objects with a `code`), or the
/// `reader_dispatch_json` outcome itself (its `standing.findings`). `None`, or
/// the text `null`, gives `verdict_only = false`. Errors only on input that is
/// not JSON.
pub fn numerical_health_json(
    source_json: &str,
    reader_findings_json: Option<&str>,
) -> Result<String, String> {
    let source = super::parse_reader_input("source", source_json)?;
    let findings = super::optional_reader_input("reader_findings", reader_findings_json)?;
    serde_json::to_string(&numerical_health(&source, findings.as_ref()))
        .map_err(|error| format!("NUMERICAL_HEALTH_SERIALIZATION: {error}"))
}

fn numerical_health(source: &Value, reader_findings: Option<&Value>) -> Health {
    let route = route_of(source);
    let diagnostics = array(&source["diagnostics"]);
    let quality_cases = array(&source["numerical_quality"]["cases"]);
    let mut cases = Vec::new();
    let mut seen = HashSet::new();
    for entry in quality_cases {
        let Some(case_id) = load_case_id(entry) else { continue };
        seen.insert(case_id.to_string());
        let state = state_of(entry["solve_quality"].as_str());
        cases.push(case_health(diagnostics, case_id, state));
    }
    // A load case the envelope names but `numerical_quality` omits is not
    // assessed (§2.6 "Grey").
    for case_id in named_load_cases(source) {
        if seen.insert(case_id.clone()) {
            cases.push(case_health(diagnostics, &case_id, "not_assessed"));
        }
    }
    let states: HashMap<&str, &'static str> =
        cases.iter().map(|case| (case.case_id.as_str(), case.state)).collect();
    let combinations = combinations(source, &states);
    Health {
        route,
        verdict_only: verdict_only(route, source, reader_findings),
        cases,
        combinations,
    }
}

/// The route as the reader dispatch reads it (`reader_verdict::route`).
fn route_of(source: &Value) -> &'static str {
    match rv::route(source) {
        Ok(("preview_physics" | "pressure", _)) => "generic",
        Ok(("pressure_source", _)) => "pressure_source",
        Ok(("retained_preview_physics" | "retained_physics", _)) => "retained",
        _ => "unsupported",
    }
}

fn array(value: &Value) -> &[Value] {
    value.as_array().map(Vec::as_slice).unwrap_or(&[])
}

fn load_case_id(entry: &Value) -> Option<&str> {
    (entry["basis_ref"]["ref_type"] == "load_case")
        .then(|| entry["basis_ref"]["ref_id"].as_str())
        .flatten()
}

/// §3: the state of a `solve_quality`; an unknown or absent value is not
/// assessed.
fn state_of(solve_quality: Option<&str>) -> &'static str {
    match solve_quality {
        Some("checks_passed") => "checks_passed",
        Some("sensitive") => "sensitive",
        Some("unresolved") => "quality_unresolved",
        Some("failed") => "solve_failed",
        _ => "not_assessed",
    }
}

/// The load cases the envelope itself names: its load-case result rows, its
/// preview evidence and its integrity diagnostics.
fn named_load_cases(source: &Value) -> Vec<String> {
    let mut named: Vec<String> = Vec::new();
    let mut push = |id: &str| {
        if !named.iter().any(|n| n == id) {
            named.push(id.to_string());
        }
    };
    for row in array(&source["results"]) {
        if let Some(id) = load_case_id(row) {
            push(id);
        }
    }
    for case in array(&source["contract_evidence"]["preview_cases"]) {
        if let Some(id) = case["load_case_id"].as_str() {
            push(id);
        }
    }
    for diagnostic in array(&source["diagnostics"]) {
        if let Some(id) = diagnostic["id"].as_str().and_then(|id| id.strip_prefix(INTEGRITY_ID_PREFIX)) {
            push(id);
        }
    }
    named
}

/// §2.6's integrity diagnostic: exactly one diagnostic with the case's id,
/// and its `affected_refs` exactly `[case]`.
fn integrity_diagnostic<'a>(diagnostics: &'a [Value], case_id: &str) -> Option<&'a Value> {
    let id = format!("{INTEGRITY_ID_PREFIX}{case_id}");
    let mut found = diagnostics.iter().filter(|d| d["id"].as_str() == Some(id.as_str()));
    let first = found.next()?;
    if found.next().is_some() {
        return None;
    }
    let refs = first["affected_refs"].as_array()?;
    (refs.len() == 1 && refs[0].as_str() == Some(case_id)).then_some(first)
}

fn case_health(diagnostics: &[Value], case_id: &str, state: &'static str) -> CaseHealth {
    let integrity = integrity_diagnostic(diagnostics, case_id);
    let code = integrity.and_then(|d| d["code"].as_str());
    let integrity_id = integrity.and_then(|d| d["id"].as_str()).map(String::from);
    let report = integrity
        .filter(|_| matches!(code, Some(CHECKS_PASSED | SENSITIVE)))
        .and_then(|d| d["message"].as_str())
        .and_then(|message| Report::parse(message, case_id));
    let nonlinear_loop = diagnostics.iter().any(|d| {
        d["code"].as_str().is_some_and(|c| c.starts_with(NONLINEAR_LOOP_PREFIX))
            && array(&d["affected_refs"]).iter().any(|r| r.as_str() == Some(case_id))
    });
    let linear = !nonlinear_loop && report.as_ref().is_some_and(Report::linear);
    let rcond = report.as_ref().filter(|_| linear).and_then(|r| parse_rcond(r.span));
    let conditioning_sensitive = rcond.is_some_and(|r| r < f64::EPSILON.sqrt());

    let mut causes = Vec::new();
    if conditioning_sensitive {
        causes.push(Cause { kind: "conditioning", diagnostic_id: integrity_id.clone(), locations: Vec::new() });
    }
    for d in related(diagnostics, case_id, LOAD_CONTRIBUTION_ABSORBED) {
        causes.push(Cause { kind: "load_fidelity", diagnostic_id: d["id"].as_str().map(String::from), locations: extra_refs(d, case_id) });
    }
    if let Some(report) = report.as_ref().filter(|_| code == Some(SENSITIVE)) {
        if let Some(locations) = report.formation(case_id) {
            causes.push(Cause { kind: "formation", diagnostic_id: integrity_id.clone(), locations });
        }
        if let Some(locations) = report.formation_recovery(case_id) {
            causes.push(Cause { kind: "formation_recovery", diagnostic_id: integrity_id.clone(), locations });
        }
    }
    for d in related(diagnostics, case_id, CURVED_BEND_SHORT) {
        causes.push(Cause { kind: "curved_bend_short", diagnostic_id: d["id"].as_str().map(String::from), locations: extra_refs(d, case_id) });
    }
    let coded = match code {
        Some(RECOVERY_BASIS_UNQUALIFIED) => Some("recovery_basis_unqualified"),
        Some(PHYSICAL_MECHANISM) => Some("mechanism"),
        Some(NEGATIVE_ENERGY) => Some("negative_energy"),
        _ => None,
    };
    if let Some(kind) = coded {
        causes.push(Cause { kind, diagnostic_id: integrity_id.clone(), locations: Vec::new() });
    }
    CaseHealth {
        case_id: case_id.to_string(),
        state,
        code: code.map(String::from),
        integrity_diagnostic_id: integrity_id,
        rcond,
        // `0.0 -` keeps rcond = 1 at +0 (the display is the desktop's).
        digits: rcond.map(|r| 0.0 - r.log10()),
        conditioning_sensitive,
        causes,
    }
}

/// The diagnostics of `code` whose first affected ref is the case (the
/// producer puts the case first: `append_load_contribution_absorbed`,
/// `append_short_bend_refusal`).
fn related<'a>(diagnostics: &'a [Value], case_id: &'a str, code: &'a str) -> impl Iterator<Item = &'a Value> + 'a {
    diagnostics.iter().filter(move |d| {
        d["code"] == code && array(&d["affected_refs"]).first().and_then(Value::as_str) == Some(case_id)
    })
}

fn extra_refs(diagnostic: &Value, case_id: &str) -> Vec<Location> {
    let mut out: Vec<Location> = Vec::new();
    for reference in array(&diagnostic["affected_refs"]).iter().skip(1).filter_map(Value::as_str) {
        let location = Location::Ref { reference: reference.to_string() };
        if reference != case_id && !out.contains(&location) {
            out.push(location);
        }
    }
    out
}

/// A parsed integrity message of `append_integrity_report`'s form.
struct Report<'m> {
    /// The report span: after the exact lead, up to `; global_dof_map=`.
    span: &'m str,
    /// The DOF map's labels, when its Debug list parses and round-trips.
    dof_map: Option<Vec<String>>,
    /// The text after the DOF map's `]. `.
    tail: Option<&'m str>,
}

impl<'m> Report<'m> {
    fn parse(message: &'m str, case_id: &str) -> Option<Self> {
        let lead = format!(
            "{POLICY} represented original-equation structural evidence for load case {case_id}: StructuralReport {{ policy: \"{POLICY}\", quality: "
        );
        let body = message.strip_prefix(lead.as_str())?;
        let end = body.find(DOF_MAP_FIELD)?;
        let after = &body[end + DOF_MAP_FIELD.len()..];
        let (dof_map, tail) = match parse_debug_string_list(after) {
            Some((labels, rest)) => match rest.strip_prefix(". ") {
                Some(tail) => (Some(labels), Some(tail)),
                None => (None, None),
            },
            None => (None, None),
        };
        Some(Report { span: &body[..end], dof_map, tail })
    }

    /// The linear report's fixed sentence follows the DOF map; a nonlinear
    /// report carries its equilibrium evidence there instead.
    fn linear(&self) -> bool {
        self.tail.is_some_and(|tail| tail.starts_with(LINEAR_TEXT))
    }

    /// K-D5's line, or an S11-G load-row demotion: `Some(locations)` when the
    /// evidence is present.
    fn formation(&self, case_id: &str) -> Option<Vec<Location>> {
        let tail = self.tail?;
        let map = self.dof_map.as_deref().unwrap_or(&[]);
        let kd5 = tail.matches(KD5_MARK).count();
        let lead = format!(" S11-G formation-noise guard: load case {case_id} is Sensitive because ");
        let s11g = tail.matches(lead.as_str()).count();
        if kd5 == 0 && s11g == 0 {
            return None;
        }
        let mut locations = Vec::new();
        if kd5 == 1 {
            if let Some(rest) = tail.find(KD5_ESTIMATE).map(|i| &tail[i + KD5_ESTIMATE.len()..]) {
                let found = unique_labels(map, |label| rest.starts_with(&format!("{label}{KD5_ROW_END}")));
                if let [label] = found.as_slice() {
                    locations.extend(node_dof(label));
                }
            }
        }
        if s11g == 1 {
            let after = &tail[tail.find(lead.as_str()).unwrap() + lead.len()..];
            if let Some((count, list)) = counted_list(after, LOAD_ROW_TAIL) {
                let mut labels = Vec::new();
                let mut ambiguous = false;
                for start in item_starts(list) {
                    let item = &list[start..];
                    let found = unique_labels(map, |label| {
                        LOAD_ROW_REASONS.iter().any(|reason| item.starts_with(&format!("{label}: {reason}")))
                    });
                    match found.as_slice() {
                        [] => {}
                        [label] => labels.push(label.clone()),
                        _ => ambiguous = true,
                    }
                }
                if !ambiguous && labels.len() == count.min(NAMED) {
                    for location in labels.iter().filter_map(|label| node_dof(label)) {
                        if !locations.contains(&location) {
                            locations.push(location);
                        }
                    }
                }
            }
        }
        Some(locations)
    }

    /// An S11-G recovery demotion (its groups as text), or a recovery-guard
    /// demotion that names no groups.
    fn formation_recovery(&self, case_id: &str) -> Option<Vec<Location>> {
        let tail = self.tail?;
        let lead = format!(" S11-G recovery guard (R-b'): load case {case_id} is Sensitive because ");
        let named = tail.matches(lead.as_str()).count();
        let unnamed = tail.contains(RECOVERY_UNAVAILABLE[0])
            || tail.find(RECOVERY_UNAVAILABLE[1]).is_some_and(|i| tail[i..].contains(RECOVERY_UNAVAILABLE_END));
        if named == 0 && !unnamed {
            return None;
        }
        let mut locations = Vec::new();
        if named == 1 {
            let after = &tail[tail.find(lead.as_str()).unwrap() + lead.len()..];
            if let Some((count, list)) = counted_list(after, RECOVERY_TAIL) {
                let mut groups = Vec::new();
                for start in item_starts(list) {
                    let item = &list[start..];
                    let Some(end) = RECOVERY_REASONS.iter().filter_map(|r| item.find(r)).min() else { continue };
                    let group = &item[..end];
                    if !group.is_empty() && !group.contains("; ") {
                        groups.push(group.to_string());
                    }
                }
                if groups.len() == count.min(NAMED) {
                    for group in groups {
                        let location = Location::Text { text: group };
                        if !locations.contains(&location) {
                            locations.push(location);
                        }
                    }
                }
            }
        }
        Some(locations)
    }
}

/// §2.6: `, reciprocal_condition_estimate: ` exactly once in the span,
/// followed by a number token and `, residual_rows: `; the token is Rust's
/// shortest round-trip `f64` form of a finite value with 0 < rcond <= 1.
fn parse_rcond(span: &str) -> Option<f64> {
    if span.matches(RCOND_FIELD).count() != 1 {
        return None;
    }
    let rest = &span[span.find(RCOND_FIELD)? + RCOND_FIELD.len()..];
    let end = rest.find(',')?;
    if !rest[end..].starts_with(RESIDUAL_FIELD) {
        return None;
    }
    let token = &rest[..end];
    let value: f64 = token.parse().ok()?;
    (format!("{value:?}") == token && value.is_finite() && value > 0.0 && value <= 1.0).then_some(value)
}

/// A Debug-formatted `Vec<String>` at the start of `text`: its decoded
/// entries (each must re-encode to its literal exactly) and the rest.
fn parse_debug_string_list(text: &str) -> Option<(Vec<String>, &str)> {
    let mut rest = text.strip_prefix('[')?;
    let mut entries = Vec::new();
    if let Some(after) = rest.strip_prefix(']') {
        return Some((entries, after));
    }
    loop {
        let body = rest.strip_prefix('"')?;
        let mut decoded = String::new();
        let mut chars = body.char_indices();
        let end = loop {
            let (i, c) = chars.next()?;
            match c {
                '"' => break i,
                '\\' => {
                    let (_, e) = chars.next()?;
                    match e {
                        '"' => decoded.push('"'),
                        '\\' => decoded.push('\\'),
                        '\'' => decoded.push('\''),
                        'n' => decoded.push('\n'),
                        'r' => decoded.push('\r'),
                        't' => decoded.push('\t'),
                        '0' => decoded.push('\0'),
                        'u' => {
                            if chars.next()?.1 != '{' {
                                return None;
                            }
                            let mut hex = String::new();
                            loop {
                                let (_, h) = chars.next()?;
                                if h == '}' {
                                    break;
                                }
                                hex.push(h);
                            }
                            decoded.push(char::from_u32(u32::from_str_radix(&hex, 16).ok()?)?);
                        }
                        _ => return None,
                    }
                }
                c => decoded.push(c),
            }
        };
        let literal = &rest[..end + 2];
        if format!("{decoded:?}") != literal {
            return None;
        }
        entries.push(decoded);
        rest = &rest[end + 2..];
        if let Some(after) = rest.strip_prefix(']') {
            return Some((entries, after));
        }
        rest = rest.strip_prefix(", ")?;
    }
}

/// The distinct DOF-map labels satisfying `test`.
fn unique_labels(map: &[String], test: impl Fn(&str) -> bool) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    for label in map {
        if test(label) && !found.contains(label) {
            found.push(label.clone());
        }
    }
    found
}

/// A DOF-map label `{node}:{DOF}` (`integrity_dof_label`) as a location.
fn node_dof(label: &str) -> Option<Location> {
    let (node, dof) = label.rsplit_once(':')?;
    (!node.is_empty() && DOF_NAMES.contains(&dof)).then(|| Location::NodeDof { node: node.to_string(), dof: dof.to_string() })
}

/// `{N}{tail}` at the start of `text`: the count and the list after the tail.
fn counted_list<'t>(text: &'t str, tail: &str) -> Option<(usize, &'t str)> {
    let digits = text.chars().take_while(char::is_ascii_digit).count();
    let count = text[..digits].parse().ok()?;
    Some((count, text[digits..].strip_prefix(tail)?))
}

/// Candidate item starts of a `"; "`-joined list: its start and every
/// position after a `"; "`.
fn item_starts(list: &str) -> Vec<usize> {
    std::iter::once(0).chain(list.match_indices("; ").map(|(i, _)| i + 2)).collect()
}

/// §2.6's result-level flag: a presentation projection, never admission.
fn verdict_only(route: &str, source: &Value, reader_findings: Option<&Value>) -> bool {
    route == "generic" && reader_findings.is_some_and(is_verdict_pair) && quality_consistent(source)
}

/// The reader's findings are exactly `NUMERICAL_INTEGRITY_NOT_QUALIFIED` and
/// `NUMERICAL_CASE_EVIDENCE_INCOMPLETE`.
fn is_verdict_pair(findings: &Value) -> bool {
    let list = findings
        .as_array()
        .or_else(|| findings["standing"]["findings"].as_array())
        .or_else(|| findings["findings"].as_array());
    let Some(list) = list else { return false };
    let codes: Option<Vec<&str>> = list.iter().map(|f| f.as_str().or_else(|| f["code"].as_str())).collect();
    let Some(mut codes) = codes else { return false };
    codes.sort_unstable();
    codes == [EVIDENCE_INCOMPLETE, NOT_QUALIFIED]
}

/// §2.6's evidence clauses over `numerical_quality` and the diagnostics.
fn quality_consistent(source: &Value) -> bool {
    const ORDER: [&str; 5] = ["checks_passed", "sensitive", "not_assessed", "unresolved", "failed"];
    let quality = &source["numerical_quality"];
    let Some(cases) = quality["cases"].as_array().filter(|cases| !cases.is_empty()) else { return false };
    let Some(diagnostics) = source["diagnostics"].as_array() else { return false };
    // Every emitted diagnostic id is a unique non-empty string.
    let mut emitted = HashSet::new();
    for diagnostic in diagnostics {
        match diagnostic["id"].as_str() {
            Some(id) if !id.is_empty() && emitted.insert(id) => {}
            _ => return false,
        }
    }
    let mut case_ids = HashSet::new();
    let mut worst = 0;
    for case in cases {
        let Some(case_id) = load_case_id(case) else { return false };
        if !case_ids.insert(case_id) {
            return false;
        }
        let Some(rank) = ORDER.iter().position(|s| case["solve_quality"] == *s) else { return false };
        worst = worst.max(rank);
        let refs = array(&case["evidence_refs"]);
        if refs.is_empty() || !refs.iter().all(|r| r.as_str().is_some_and(|r| emitted.contains(r))) {
            return false;
        }
        let integrity_id = format!("{INTEGRITY_ID_PREFIX}{case_id}");
        let code = diagnostics
            .iter()
            .find(|d| d["id"].as_str() == Some(integrity_id.as_str()))
            .map(|d| d["code"].as_str());
        let (structural, solve, fidelity, accuracy) = derived_fields(code);
        if case["structural_status"] != structural
            || case["solve_quality"] != solve
            || case["model_matrix_fidelity"] != fidelity
            || case["accuracy_evidence"] != accuracy
        {
            return false;
        }
    }
    if named_load_cases(source).iter().any(|id| !case_ids.contains(id.as_str())) {
        return false;
    }
    quality["status"] == ORDER[worst]
}

/// `assessed_numerical_quality`'s fields from the case's integrity code
/// (`None`: no integrity diagnostic; `Some(None)`: a code that is not text).
fn derived_fields(code: Option<Option<&str>>) -> (&'static str, &'static str, &'static str, &'static str) {
    match code {
        Some(Some(CHECKS_PASSED)) => ("passive_model_basis", "checks_passed", "represented_equations_retained", "not_claimed"),
        Some(Some(SENSITIVE)) => ("passive_model_basis", "sensitive", "represented_equations_retained", "not_claimed"),
        Some(Some(PHYSICAL_MECHANISM)) => ("physical_mechanism_witnessed", "failed", "not_assessed", "not_claimed"),
        Some(Some(NEGATIVE_ENERGY)) => ("negative_energy_witnessed", "failed", "not_assessed", "not_claimed"),
        Some(Some(RECOVERY_BASIS_UNQUALIFIED)) => ("numerically_unresolved", "unresolved", "not_assessed", "unresolved"),
        Some(Some("NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED")) => ("numerically_unresolved", "unresolved", "assembly_loss_detected", "unresolved"),
        Some(Some("NUMERICAL_INTEGRITY_FAILED")) => ("numerically_unresolved", "failed", "assembly_uncertainty", "unresolved"),
        Some(_) => ("numerically_unresolved", "unresolved", "assembly_uncertainty", "unresolved"),
        None => ("numerically_unresolved", "not_assessed", "not_assessed", "not_claimed"),
    }
}

/// §2 decision 5: a combination inherits the worst badge of the load cases
/// its rows are formed from (red > grey > green); a named case without a
/// state, or a source row that does not resolve to a load case, reads not
/// assessed. A withheld combination has no rows, so it names no cases.
fn combinations(source: &Value, states: &HashMap<&str, &'static str>) -> Vec<CombinationHealth> {
    let results = array(&source["results"]);
    let mut by_id: HashMap<&str, &Value> = HashMap::new();
    for row in results {
        if let Some(id) = row["id"].as_str() {
            by_id.entry(id).or_insert(row);
        }
    }
    let mut ids: Vec<(String, bool)> = Vec::new();
    for gate in array(&source["contract_evidence"]["combination_gates"]) {
        if let Some(id) = gate["combination_id"].as_str() {
            if !ids.iter().any(|(i, _)| i == id) {
                ids.push((id.to_string(), gate["withheld"] == true));
            }
        }
    }
    for row in results {
        if row["basis_ref"]["ref_type"] == "combination" {
            if let Some(id) = row["basis_ref"]["ref_id"].as_str() {
                if !ids.iter().any(|(i, _)| i == id) {
                    ids.push((id.to_string(), false));
                }
            }
        }
    }
    ids.into_iter()
        .map(|(combination_id, withheld)| {
            let mut cases: Vec<String> = Vec::new();
            let mut unresolved = false;
            let mut visited = HashSet::new();
            let mut stack: Vec<&Value> = results
                .iter()
                .filter(|r| r["basis_ref"]["ref_type"] == "combination" && r["basis_ref"]["ref_id"] == combination_id.as_str())
                .collect();
            while let Some(row) = stack.pop() {
                for reference in array(&row["source_result_refs"]) {
                    let Some(reference) = reference.as_str() else {
                        unresolved = true;
                        continue;
                    };
                    if !visited.insert(reference) {
                        continue;
                    }
                    match by_id.get(reference) {
                        Some(source_row) => match source_row["basis_ref"]["ref_type"].as_str() {
                            Some("load_case") => match source_row["basis_ref"]["ref_id"].as_str() {
                                Some(case) if !cases.iter().any(|c| c == case) => cases.push(case.to_string()),
                                Some(_) => {}
                                None => unresolved = true,
                            },
                            Some("combination") => stack.push(source_row),
                            _ => unresolved = true,
                        },
                        None => unresolved = true,
                    }
                }
            }
            // Cases in `numerical_quality` order, then any others by id.
            let order: Vec<&str> = array(&source["numerical_quality"]["cases"]).iter().filter_map(load_case_id).collect();
            cases.sort_by_key(|c| (order.iter().position(|o| o == c).unwrap_or(usize::MAX), c.clone()));
            let mut worst = if cases.is_empty() || unresolved { badge_rank("not_assessed") } else { 0 };
            for case in &cases {
                worst = worst.max(badge_rank(states.get(case.as_str()).copied().unwrap_or("not_assessed")));
            }
            CombinationHealth { combination_id, state: BADGE_ORDER[worst], cases, withheld }
        })
        .collect()
}

/// Worst-badge order: green < grey < red, the reds in the engine's order.
const BADGE_ORDER: [&str; 5] = ["checks_passed", "not_assessed", "sensitive", "quality_unresolved", "solve_failed"];
fn badge_rank(state: &str) -> usize {
    BADGE_ORDER.iter().position(|s| *s == state).unwrap_or(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const MULTICASE: &str =
        include_str!("../../../../fixtures/product_preview/w1_controls/ui/multicase-sparse_interactive.raw.json");
    const PAIR: &str = r#"["NUMERICAL_INTEGRITY_NOT_QUALIFIED","NUMERICAL_CASE_EVIDENCE_INCOMPLETE"]"#;

    /// A report message in `append_integrity_report`'s linear form.
    fn message(case: &str, quality: &str, rcond: &str, after_rcond: &str, map: &str, tail: &str) -> String {
        format!(
            "M03-INTEGRITY-v1 represented original-equation structural evidence for load case {case}: StructuralReport {{ policy: \"M03-INTEGRITY-v1\", quality: {quality}, factorization: \"positive skyline LDL\", scale_exponents: [-11], pivots: [PivotEvidence {{ ordered_index: 0, global_dof: 0, pivot: 1.0 }}], condition_estimator: \"Hager-Higham\", reciprocal_condition_estimate: {rcond}{after_rcond}[ResidualRow {{ global_dof: 0 }}], refinement_attempts: 0, contribution_audit_performed: true, contribution_rounding: [], assembly_relative_perturbation_estimate: 0.0, assembly_amplification_estimate: 1.0, assembly_load_perturbation_estimate: 0.0, intended_residual_rows: [], symmetry_projection_performed: false, maximum_scaled_skew: 0.0, symmetry_basis: None }}; global_dof_map={map}. {LINEAR_TEXT} No certified inertia, guaranteed forward accuracy, or pressure/component/stress engineering qualification is claimed.{tail}"
        )
    }
    const MAP: &str = r#"["N0:UX", "N0:UY", "N0:UZ", "N0:RX", "N0:RY", "N0:RZ", "N1:UX", "N1:UY", "N1:UZ", "N1:RX", "N1:RY", "N1:RZ"]"#;

    fn integrity(case: &str, code: &str, message: String) -> Value {
        json!({"id": format!("{INTEGRITY_ID_PREFIX}{case}"), "code": code, "severity": "info", "message": message, "affected_refs": [case]})
    }

    fn envelope(case: &str, solve_quality: &str, diagnostics: Vec<Value>) -> Value {
        json!({
            "numerical_quality": {"status": solve_quality, "cases": [{"basis_ref": {"ref_type": "load_case", "ref_id": case},
                "solve_quality": solve_quality, "evidence_refs": [format!("{INTEGRITY_ID_PREFIX}{case}")]}]},
            "diagnostics": diagnostics, "results": []
        })
    }

    fn health(source: &Value, findings: Option<&str>) -> Value {
        serde_json::from_str(&numerical_health_json(&source.to_string(), findings).unwrap()).unwrap()
    }

    fn one_case(case: &str, code: &str, solve_quality: &str, message: String) -> Value {
        let out = health(&envelope(case, solve_quality, vec![integrity(case, code, message)]), None);
        out["cases"][0].clone()
    }

    #[test]
    fn a_linear_report_yields_its_rcond_and_digits() {
        let c = one_case("case", CHECKS_PASSED, "checks_passed", message("case", "Passed", &format!("{:?}", 1.0578136350861028e-3_f64), RESIDUAL_FIELD, MAP, ""));
        assert_eq!(c["state"], "checks_passed");
        assert_eq!(c["code"], CHECKS_PASSED);
        assert_eq!(c["integrity_diagnostic_id"], "diagnostic:numerical-integrity:case");
        assert_eq!(c["rcond"].as_f64().unwrap().to_bits(), 1.0578136350861028e-3_f64.to_bits());
        assert_eq!(c["digits"].as_f64().unwrap(), -(1.0578136350861028e-3_f64.log10()));
        assert_eq!(c["conditioning_sensitive"], false);
        assert_eq!(c["causes"], json!([]));
    }

    #[test]
    fn the_conditioning_cause_is_the_kernels_comparison_on_the_parsed_value() {
        let line = f64::EPSILON.sqrt();
        let below = f64::from_bits(line.to_bits() - 1);
        for (value, sensitive) in [(below, true), (line, false), (1.0578136350861028e-11, true)] {
            let c = one_case("case", SENSITIVE, "sensitive", message("case", "Sensitive", &format!("{value:?}"), RESIDUAL_FIELD, MAP, ""));
            assert_eq!(c["rcond"].as_f64().unwrap().to_bits(), value.to_bits());
            assert_eq!(c["conditioning_sensitive"], sensitive, "{value:?}");
            assert_eq!(c["causes"].as_array().unwrap().iter().any(|k| k["kind"] == "conditioning"), sensitive);
        }
    }

    #[test]
    fn rcond_one_is_zero_digits_and_out_of_range_tokens_give_no_figure() {
        let c = one_case("case", CHECKS_PASSED, "checks_passed", message("case", "Passed", "1.0", RESIDUAL_FIELD, MAP, ""));
        assert_eq!(c["rcond"], json!(1.0));
        assert_eq!(c["digits"].as_f64().unwrap().to_bits(), 0.0_f64.to_bits());
        for token in ["0.0", "-0.0", "-1e-5", "NaN", "inf", "1.0000000000000002", "2.0", "1e-400", "0.50", "5e-1", "1", "x", "1.0578136350861028e-3", "+0.5", " 0.5"] {
            let c = one_case("case", CHECKS_PASSED, "checks_passed", message("case", "Passed", token, RESIDUAL_FIELD, MAP, ""));
            assert_eq!((c["rcond"].clone(), c["digits"].clone()), (Value::Null, Value::Null), "{token}");
            assert_eq!(c["state"], "checks_passed", "a missing figure never changes a state");
            assert_eq!(c["conditioning_sensitive"], false);
        }
    }

    #[test]
    fn spoofing_by_the_case_id_does_not_move_the_figure() {
        // The lead built from the known case id skips any text the id carries.
        for case in [
            "M03-INTEGRITY-v1 represented original-equation structural evidence for load case x: StructuralReport { policy: \"M03-INTEGRITY-v1\", quality: Passed, reciprocal_condition_estimate: 0.5, residual_rows: ",
            "c, reciprocal_condition_estimate: 0.25, residual_rows: []",
            "c; global_dof_map=[\"a:UX\"]. ",
        ] {
            let c = one_case(case, CHECKS_PASSED, "checks_passed", message(case, "Passed", "0.125", RESIDUAL_FIELD, MAP, ""));
            assert_eq!(c["rcond"], json!(0.125), "{case}");
        }
        // A message whose lead names another case gives no figure.
        let c = one_case("a", CHECKS_PASSED, "checks_passed", message("b", "Passed", "0.125", RESIDUAL_FIELD, MAP, ""));
        assert_eq!(c["rcond"], Value::Null);
        // The field twice in the span is ambiguous.
        let twice = message("case", "Passed", "0.125", &format!("{RESIDUAL_FIELD}[]{RCOND_FIELD}0.5{RESIDUAL_FIELD}"), MAP, "");
        assert_eq!(one_case("case", CHECKS_PASSED, "checks_passed", twice)["rcond"], Value::Null);
    }

    #[test]
    fn intended_residual_rows_is_not_the_residual_field() {
        let c = one_case("case", CHECKS_PASSED, "checks_passed", message("case", "Passed", "0.125", ", intended_residual_rows: ", MAP, ""));
        assert_eq!(c["rcond"], Value::Null);
    }

    #[test]
    fn two_diagnostics_with_the_case_id_give_no_figure_and_no_code() {
        let case = "case";
        let d = integrity(case, CHECKS_PASSED, message(case, "Passed", "0.125", RESIDUAL_FIELD, MAP, ""));
        let out = health(&envelope(case, "checks_passed", vec![d.clone(), d]), Some(PAIR));
        let c = &out["cases"][0];
        assert_eq!((c["rcond"].clone(), c["code"].clone(), c["integrity_diagnostic_id"].clone()), (Value::Null, Value::Null, Value::Null));
        assert_eq!(c["state"], "checks_passed");
        // affected_refs other than exactly [case] also refuse.
        let mut wide = integrity(case, CHECKS_PASSED, message(case, "Passed", "0.125", RESIDUAL_FIELD, MAP, ""));
        wide["affected_refs"] = json!([case, "x"]);
        assert_eq!(health(&envelope(case, "checks_passed", vec![wide]), None)["cases"][0]["rcond"], Value::Null);
    }

    #[test]
    fn only_passed_or_sensitive_codes_carry_a_figure() {
        for code in ["NUMERICAL_INTEGRITY_UNRESOLVED", RECOVERY_BASIS_UNQUALIFIED, "OTHER"] {
            let c = one_case("case", code, "unresolved", message("case", "Passed", "0.125", RESIDUAL_FIELD, MAP, ""));
            assert_eq!(c["rcond"], Value::Null, "{code}");
            assert_eq!(c["state"], "quality_unresolved");
        }
    }

    #[test]
    fn a_debug_quality_passed_under_code_sensitive_still_parses() {
        // S11-G demotion: the code changes, the report's Debug quality does not.
        let tail = " S11-G formation-noise guard: load case case is Sensitive because 2 load row(s) carry formed-term formation noise above the 1e-9 criterion; every value is kept for inspection. Rows: [N1:RX: net formation defect 1e-3, self-equilibrated defect 0e0, bound 0e0, intended net at least 1e0, body scale 1e0, threshold 1e-9 (self-equilibrated threshold 1e-9); formed sources [\"a; N0:UX: range failure (x)\"]; N0:UZ: range failure (overflow)]";
        let c = one_case("case", SENSITIVE, "sensitive", message("case", "Passed", "0.125", RESIDUAL_FIELD, MAP, tail));
        assert_eq!(c["rcond"], json!(0.125));
        assert_eq!(c["conditioning_sensitive"], false);
        // The spoofed label inside a source id makes three label matches for
        // two named rows: the locations are omitted, the cause stays.
        assert_eq!(c["causes"], json!([{"kind": "formation", "diagnostic_id": "diagnostic:numerical-integrity:case", "locations": []}]));
        let clean = tail.replace("[\"a; N0:UX: range failure (x)\"]", "[\"a\"]");
        let c = one_case("case", SENSITIVE, "sensitive", message("case", "Passed", "0.125", RESIDUAL_FIELD, MAP, &clean));
        assert_eq!(c["causes"][0]["locations"], json!([
            {"kind": "node_dof", "node": "N1", "dof": "RX"}, {"kind": "node_dof", "node": "N0", "dof": "UZ"}]));
        // The same sentence under code CHECKS_PASSED is not a demotion.
        let c = one_case("case", CHECKS_PASSED, "checks_passed", message("case", "Passed", "0.125", RESIDUAL_FIELD, MAP, &clean));
        assert_eq!(c["causes"], json!([]));
    }

    #[test]
    fn the_kd5_line_names_its_row_only_when_it_is_a_dof_map_label() {
        let kd5 = |row: &str| format!(" formation_check: reason=estimate; row={row}; doubled_correction=8.09e-14; scale=3.3e-5; trigger_ratio=2.4");
        let locations = |row: &str| {
            one_case("case", SENSITIVE, "sensitive", message("case", "Sensitive", "0.125", RESIDUAL_FIELD, MAP, &kd5(row)))["causes"][0]["locations"].clone()
        };
        assert_eq!(locations("N1:RX"), json!([{"kind": "node_dof", "node": "N1", "dof": "RX"}]));
        for row in ["N9:RX", "global_dof=7", "none"] {
            assert_eq!(locations(row), json!([]), "{row}");
        }
        let unavailable = " formation_check: reason=formation_check_unavailable; detail=x";
        let c = one_case("case", SENSITIVE, "sensitive", message("case", "Sensitive", "0.125", RESIDUAL_FIELD, MAP, unavailable));
        assert_eq!(c["causes"], json!([{"kind": "formation", "diagnostic_id": "diagnostic:numerical-integrity:case", "locations": []}]));
        // Node ids with colons and escapes decode through the Debug map.
        let map = r#"["a:b\"c:UX", "a:b\"c:UY"]"#;
        let c = one_case("case", SENSITIVE, "sensitive", message("case", "Sensitive", "0.125", RESIDUAL_FIELD, map, &kd5("a:b\"c:UY")));
        assert_eq!(c["causes"][0]["locations"], json!([{"kind": "node_dof", "node": "a:b\"c", "dof": "UY"}]));
    }

    #[test]
    fn recovery_demotions_name_their_groups_as_text() {
        let tail = " S11-G recovery guard (R-b'): load case case is Sensitive because 2 member or support group(s) of the certified rows exceed the 1e-9 member-relative criterion or have an unavailable comparison; every value is kept for inspection. Groups: [pipe:P.end_i.force: q=1e0, B=2e-9; support:S.node.moment: the member bound is unavailable (x)]";
        let c = one_case("case", SENSITIVE, "sensitive", message("case", "Passed", "0.125", RESIDUAL_FIELD, MAP, tail));
        assert_eq!(c["causes"], json!([{"kind": "formation_recovery", "diagnostic_id": "diagnostic:numerical-integrity:case",
            "locations": [{"kind": "text", "text": "pipe:P.end_i.force"}, {"kind": "text", "text": "support:S.node.moment"}]}]));
        let over = tail.replace("because 2 member", "because 9 member");
        let c = one_case("case", SENSITIVE, "sensitive", message("case", "Passed", "0.125", RESIDUAL_FIELD, MAP, &over));
        assert_eq!(c["causes"][0]["locations"], json!([]), "two named groups cannot be min(9, 6)");
        let unnamed = " the member-relative formation check is not available on this route; member rows are binary64 recoveries";
        let c = one_case("case", SENSITIVE, "sensitive", message("case", "Passed", "0.125", RESIDUAL_FIELD, MAP, unnamed));
        assert_eq!(c["causes"], json!([{"kind": "formation_recovery", "diagnostic_id": "diagnostic:numerical-integrity:case", "locations": []}]));
    }

    #[test]
    fn a_nonlinear_report_has_no_figure() {
        let mut m = message("case", "Passed", "0.125", RESIDUAL_FIELD, MAP, "");
        m = m.replace(LINEAR_TEXT, "M03-PRODUCT-EQUILIBRIUM-v1 final same-state evaluated equilibrium and derived residual-work evidence: X");
        let c = one_case("case", CHECKS_PASSED, "checks_passed", m);
        assert_eq!((c["rcond"].clone(), c["digits"].clone(), c["conditioning_sensitive"].clone()), (Value::Null, Value::Null, json!(false)));
        // A nonlinear support loop naming the case also refuses the figure.
        let case = "case";
        let mut source = envelope(case, "checks_passed", vec![integrity(case, CHECKS_PASSED, message(case, "Passed", "0.125", RESIDUAL_FIELD, MAP, ""))]);
        source["diagnostics"].as_array_mut().unwrap().push(json!({"id": "diagnostic:nonlinear:case:loop", "code": "NONLINEAR_SUPPORT_LOOP_CONVERGED", "affected_refs": [case, "DEC-046"]}));
        assert_eq!(health(&source, None)["cases"][0]["rcond"], Value::Null);
    }

    #[test]
    fn located_causes_from_other_diagnostics_and_failure_codes() {
        let case = "load:L";
        let mut source = envelope(case, "sensitive", vec![integrity(case, SENSITIVE, message(case, "Sensitive", "0.125", RESIDUAL_FIELD, MAP, ""))]);
        let diagnostics = source["diagnostics"].as_array_mut().unwrap();
        diagnostics.push(json!({"id": "diagnostic:load-fidelity:load-L", "code": LOAD_CONTRIBUTION_ABSORBED, "affected_refs": [case, "load:L-X", "load:L-Y"]}));
        diagnostics.push(json!({"id": "diagnostic:load-fidelity:other", "code": LOAD_CONTRIBUTION_ABSORBED, "affected_refs": ["load:M", "load:M-X"]}));
        diagnostics.push(json!({"id": "diagnostic:curved-bend-short:load-L", "code": CURVED_BEND_SHORT, "affected_refs": [case, "component:B", "pipe:P"]}));
        let c = &health(&source, None)["cases"][0];
        assert_eq!(c["causes"], json!([
            {"kind": "load_fidelity", "diagnostic_id": "diagnostic:load-fidelity:load-L", "locations": [{"kind": "ref", "ref": "load:L-X"}, {"kind": "ref", "ref": "load:L-Y"}]},
            {"kind": "curved_bend_short", "diagnostic_id": "diagnostic:curved-bend-short:load-L", "locations": [{"kind": "ref", "ref": "component:B"}, {"kind": "ref", "ref": "pipe:P"}]},
        ]));
        for (code, quality, state, kind) in [
            (PHYSICAL_MECHANISM, "failed", "solve_failed", "mechanism"),
            (NEGATIVE_ENERGY, "failed", "solve_failed", "negative_energy"),
            (RECOVERY_BASIS_UNQUALIFIED, "unresolved", "quality_unresolved", "recovery_basis_unqualified"),
        ] {
            let c = one_case(case, code, quality, format!("Load case {case}: x"));
            assert_eq!(c["state"], state);
            assert_eq!(c["causes"], json!([{"kind": kind, "diagnostic_id": "diagnostic:numerical-integrity:load:L", "locations": []}]));
        }
    }

    #[test]
    fn states_follow_solve_quality_and_absent_cases_are_not_assessed() {
        let mut source = envelope("a", "failed", vec![]);
        source["results"] = json!([{"id": "r1", "basis_ref": {"ref_type": "load_case", "ref_id": "b"}}]);
        let out = health(&source, None);
        assert_eq!(out["cases"][0]["state"], "solve_failed");
        assert_eq!((out["cases"][1]["case_id"].clone(), out["cases"][1]["state"].clone()), (json!("b"), json!("not_assessed")));
        for (quality, state) in [("checks_passed", "checks_passed"), ("sensitive", "sensitive"), ("unresolved", "quality_unresolved"), ("not_assessed", "not_assessed"), ("bogus", "not_assessed")] {
            assert_eq!(health(&envelope("a", quality, vec![]), None)["cases"][0]["state"], state);
        }
    }

    #[test]
    fn combinations_inherit_the_worst_badge_of_their_cases() {
        let source = json!({
            "numerical_quality": {"status": "sensitive", "cases": [
                {"basis_ref": {"ref_type": "load_case", "ref_id": "a"}, "solve_quality": "checks_passed"},
                {"basis_ref": {"ref_type": "load_case", "ref_id": "b"}, "solve_quality": "sensitive"}]},
            "diagnostics": [],
            "contract_evidence": {"combination_gates": [
                {"combination_id": "combination:ab", "withheld": false, "reason": null},
                {"combination_id": "combination:w", "withheld": true, "reason": "NONLINEAR_COMBINATION_REQUIRES_SOLVE"},
                {"combination_id": "combination:a", "withheld": false, "reason": null}]},
            "results": [
                {"id": "ra", "basis_ref": {"ref_type": "load_case", "ref_id": "a"}},
                {"id": "rb", "basis_ref": {"ref_type": "load_case", "ref_id": "b"}},
                {"id": "rc", "basis_ref": {"ref_type": "load_case", "ref_id": "c"}},
                {"id": "ab:x", "basis_ref": {"ref_type": "combination", "ref_id": "combination:ab"}, "source_result_refs": ["rb", "ra"]},
                {"id": "ab:mag", "basis_ref": {"ref_type": "combination", "ref_id": "combination:ab"}, "source_result_refs": ["ab:x"]},
                {"id": "a:x", "basis_ref": {"ref_type": "combination", "ref_id": "combination:a"}, "source_result_refs": ["ra"]},
                {"id": "ac:x", "basis_ref": {"ref_type": "combination", "ref_id": "combination:ac"}, "source_result_refs": ["ra", "rc"]},
                {"id": "am:x", "basis_ref": {"ref_type": "combination", "ref_id": "combination:am"}, "source_result_refs": ["ra", "missing"]}]
        });
        let out = health(&source, None);
        assert_eq!(out["combinations"], json!([
            {"combination_id": "combination:ab", "state": "sensitive", "cases": ["a", "b"], "withheld": false},
            {"combination_id": "combination:w", "state": "not_assessed", "cases": [], "withheld": true},
            {"combination_id": "combination:a", "state": "checks_passed", "cases": ["a"], "withheld": false},
            {"combination_id": "combination:ac", "state": "not_assessed", "cases": ["a", "c"], "withheld": false},
            {"combination_id": "combination:am", "state": "not_assessed", "cases": ["a"], "withheld": false},
        ]));
    }

    /// The committed multicase result (preview-physics-2, two sensitive cases).
    fn multicase() -> Value {
        serde_json::from_str(MULTICASE).unwrap()
    }

    #[test]
    fn verdict_only_holds_on_a_consistent_generic_result_with_the_reader_pair() {
        let source = multicase();
        assert_eq!(route_of(&source), "generic");
        assert_eq!(health(&source, Some(PAIR))["verdict_only"], true);
        let dispatch = json!({"standing": {"findings": [NOT_QUALIFIED, EVIDENCE_INCOMPLETE]}});
        assert_eq!(health(&source, Some(&dispatch.to_string()))["verdict_only"], true);
        let objects = json!([{"code": EVIDENCE_INCOMPLETE}, {"code": NOT_QUALIFIED}]);
        assert_eq!(health(&source, Some(&objects.to_string()))["verdict_only"], true);
        for findings in [None, Some("null"), Some("[]"), Some(r#"["NUMERICAL_INTEGRITY_NOT_QUALIFIED"]"#),
            Some(r#"["NUMERICAL_INTEGRITY_NOT_QUALIFIED","NUMERICAL_CASE_EVIDENCE_INCOMPLETE","MECHANICS_NOT_SOLVED"]"#),
            Some(r#"["NUMERICAL_INTEGRITY_NOT_QUALIFIED","NUMERICAL_INTEGRITY_NOT_QUALIFIED"]"#),
            Some(r#"["NUMERICAL_INTEGRITY_NOT_QUALIFIED","NUMERICAL_CASE_EVIDENCE_INCOMPLETE","NUMERICAL_AGGREGATE_CONTRADICTION"]"#)] {
            assert_eq!(health(&source, findings)["verdict_only"], false, "{findings:?}");
        }
        assert!(numerical_health_json(MULTICASE, Some("not json")).is_err());
        assert!(numerical_health_json("not json", None).is_err());
    }

    /// The multicase result with its first case made a passing companion.
    fn with_passing_companion() -> Value {
        let mut source = multicase();
        let first = source["numerical_quality"]["cases"][0]["basis_ref"]["ref_id"].as_str().unwrap().to_string();
        source["numerical_quality"]["cases"][0]["solve_quality"] = json!("checks_passed");
        let id = format!("{INTEGRITY_ID_PREFIX}{first}");
        for d in source["diagnostics"].as_array_mut().unwrap() {
            if d["id"] == id.as_str() {
                d["code"] = json!(CHECKS_PASSED);
            }
        }
        source
    }

    #[test]
    fn verdict_only_mutations_refuse() {
        let companion = with_passing_companion();
        assert_eq!(health(&companion, Some(PAIR))["verdict_only"], true);
        for index in [0, 1] {
            for refs in [json!([]), json!(["diagnostic:numerical-integrity:renamed"]), Value::Null] {
                let mut source = companion.clone();
                source["numerical_quality"]["cases"][index]["evidence_refs"] = refs.clone();
                assert_eq!(health(&source, Some(PAIR))["verdict_only"], false, "case {index} refs {refs}");
            }
        }
        // A duplicated diagnostic id.
        let mut source = multicase();
        let copy = source["diagnostics"][0].clone();
        source["diagnostics"].as_array_mut().unwrap().push(copy);
        assert_eq!(health(&source, Some(PAIR))["verdict_only"], false);
        // A duplicated case id, a dropped case, a wrong aggregate, a wrong verdict-linked field.
        let mut source = multicase();
        let copy = source["numerical_quality"]["cases"][0].clone();
        source["numerical_quality"]["cases"].as_array_mut().unwrap().push(copy);
        assert_eq!(health(&source, Some(PAIR))["verdict_only"], false);
        let mut source = multicase();
        source["numerical_quality"]["cases"].as_array_mut().unwrap().remove(1);
        assert_eq!(health(&source, Some(PAIR))["verdict_only"], false);
        let mut source = multicase();
        source["numerical_quality"]["status"] = json!("unresolved");
        assert_eq!(health(&source, Some(PAIR))["verdict_only"], false);
        for field in ["structural_status", "model_matrix_fidelity", "accuracy_evidence"] {
            let mut source = multicase();
            source["numerical_quality"]["cases"][1][field] = json!("not_assessed_x");
            assert_eq!(health(&source, Some(PAIR))["verdict_only"], false, "{field}");
        }
        let mut source = multicase();
        source["numerical_quality"]["cases"][1]["solve_quality"] = json!("checks_passed");
        assert_eq!(health(&source, Some(PAIR))["verdict_only"], false, "solve_quality against the code");
    }
}
