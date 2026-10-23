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
//! - Causes are emitted only where their evidence is present (§3). Evidence
//!   records appended to the integrity message are recognized only at their
//!   fixed positions after the base message, never by searching text that
//!   carries identifiers; every location is parsed strictly and omitted rather
//!   than guessed. Work is linear in the message length.
//! - `verdict_only` is §2.6's result-level projection; it never takes part in
//!   admission or standing.
//!
//! Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
use open_pipe_stress_product_physics::result_export::reader_verdict as rv;
use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeSet, HashMap, HashSet};

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

// The producer text this parser copies. `tests::producer_text_is_pinned`
// asserts each piece occurs verbatim in `core/product_physics`.

/// `append_integrity_report`'s lead: `{LEAD_A}{case}{LEAD_B}`, then the report.
pub(crate) const LEAD_A: &str =
    "M03-INTEGRITY-v1 represented original-equation structural evidence for load case ";
pub(crate) const LEAD_B: &str = ": StructuralReport { policy: \"M03-INTEGRITY-v1\", quality: ";
/// `StructuralReport`'s Debug fields around rcond (§2.6).
const RCOND_FIELD: &str = ", reciprocal_condition_estimate: ";
const RESIDUAL_FIELD: &str = ", residual_rows: ";
/// The end of the report span, then the Debug DOF map and `. `.
pub(crate) const DOF_MAP_FIELD: &str = "; global_dof_map=";
/// The linear report's fixed sentence after the DOF map.
pub(crate) const LINEAR_TEXT: &str = "The contribution audit distinguishes intended assembly from stored equations; physical formulation limitations remain applicable.";
/// The end of the nonlinear report's equilibrium sentence.
pub(crate) const NONLINEAR_TEXT_END: &str = "separate zero count/cap/contact/sliding checks passed";
/// The base message's closing sentence; appended records follow it.
pub(crate) const CLAIM_TEXT: &str = " No certified inertia, guaranteed forward accuracy, or pressure/component/stress engineering qualification is claimed.";
const DOF_NAMES: [&str; 6] = ["UX", "UY", "UZ", "RX", "RY", "RZ"];

/// K-D5's evidence line (`formation_check_evidence_line`), the first record.
pub(crate) const KD5_LINE: &str = " formation_check: reason=";
pub(crate) const KD5_ESTIMATE: &str = "estimate; row=";
pub(crate) const KD5_ROW_END: &str = "; doubled_correction=";
/// S11-G's load-row sentence (`formation_guard::load_row_finding`), the first
/// record: `{LOAD_ROW_A}{case}{SENSITIVE_BECAUSE}{N}{LOAD_ROW_TAIL}` items `]`.
pub(crate) const LOAD_ROW_A: &str = " S11-G formation-noise guard: load case ";
pub(crate) const SENSITIVE_BECAUSE: &str = " is Sensitive because ";
pub(crate) const LOAD_ROW_TAIL: &str = " load row(s) carry formed-term formation noise above the 1e-9 criterion; every value is kept for inspection. Rows: [";
pub(crate) const ROW_RANGE: &str = "range failure (";
pub(crate) const ROW_CANNOT_BOUND: &str =
    "no conservative formation bound (CannotBound: curved-span consistent load vector from ";
pub(crate) const ROW_DEFECT: [&str; 8] = [
    "net formation defect ",
    ", self-equilibrated defect ",
    ", bound ",
    ", intended net at least ",
    ", body scale ",
    ", threshold ",
    " (self-equilibrated threshold ",
    "); formed sources ",
];
/// S11-G's recovery sentence (`formation_guard::recovery_finding`), the last
/// record: `{RECOVERY_A}{case}{SENSITIVE_BECAUSE}{N}{RECOVERY_TAIL}` items `]`.
pub(crate) const RECOVERY_A: &str = " S11-G recovery guard (R-b'): load case ";
pub(crate) const RECOVERY_TAIL: &str = " member or support group(s) of the certified rows exceed the 1e-9 member-relative criterion or have an unavailable comparison; every value is kept for inspection. Groups: [";
pub(crate) const GROUP_Q: [&str; 2] = [": q=", ", B="];
pub(crate) const GROUP_COMPARISON: &str = ": member comparison unavailable (";
pub(crate) const GROUP_BOUND: &str = ": the member bound is unavailable (";
/// `formation_guard::member_group_decision`'s reasons.
pub(crate) const COMPARISON_REASONS: [&str; 6] = [
    "nonfinite published group norm",
    "nonfinite certified group magnitude",
    "nonfinite member bound",
    "nonfinite body action scale",
    "invalid member comparison inputs",
    "exact member comparison range",
];
/// Both guards' overflow suffix.
pub(crate) const AND_MORE: [&str; 2] = ["; and ", " more"];
/// The linear recovery demotion that names no groups (product_physics: the
/// member-relative check unavailable), the last record.
pub(crate) const MEMBER_RELATIVE_UNAVAILABLE: &str = " the member-relative formation check is not available on this route; member rows are binary64 recoveries";
/// The range-scaling evidence record that may follow the first record.
pub(crate) const RANGE_SCALING: &str = " range_scaling: ";
/// The guard's named-row limit (`formation_guard::NAMED`).
const NAMED: usize = 6;
/// The nonlinear confirmed-state evidence line (product_physics
/// `NONLINEAR_CONFIRMED_EVIDENCE_LINE`), which a recovery demotion may follow.
pub(crate) const NONLINEAR_CONFIRMED_LINE: &str = "Converged support state: established at 128-bit precision by one formation-checked solve; each support's state has a sign margin larger than its bound, and a sliding friction force satisfies the friction law of that state within its bound; the published nodal displacements, and the support displacement rows that follow them, are the formation check's corrected displacements; the iteration's residual rows are its own.";

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum Location {
    NodeDof {
        node: String,
        dof: String,
    },
    Ref {
        #[serde(rename = "ref")]
        reference: String,
    },
    Text {
        text: String,
    },
}

#[derive(Debug, Serialize)]
struct CombinationHealth {
    combination_id: String,
    /// `None` for a withheld combination: it keeps its finding and has no badge.
    state: Option<&'static str>,
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

/// The envelope's diagnostics, indexed once.
struct Diagnostics<'a> {
    by_id: HashMap<&'a str, Vec<&'a Value>>,
    /// (code, first affected ref) for the causes named by other diagnostics.
    by_code_case: HashMap<&'a str, HashMap<&'a str, Vec<&'a Value>>>,
    /// Cases a nonlinear support loop names.
    nonlinear: HashSet<&'a str>,
}

impl<'a> Diagnostics<'a> {
    fn new(source: &'a Value) -> Self {
        let mut index = Diagnostics {
            by_id: HashMap::new(),
            by_code_case: HashMap::new(),
            nonlinear: HashSet::new(),
        };
        for d in array(&source["diagnostics"]) {
            if let Some(id) = d["id"].as_str() {
                index.by_id.entry(id).or_default().push(d);
            }
            let refs = array(&d["affected_refs"]);
            match d["code"].as_str() {
                Some(code @ (LOAD_CONTRIBUTION_ABSORBED | CURVED_BEND_SHORT)) => {
                    if let Some(case) = refs.first().and_then(Value::as_str) {
                        index
                            .by_code_case
                            .entry(code)
                            .or_default()
                            .entry(case)
                            .or_default()
                            .push(d);
                    }
                }
                Some(code) if code.starts_with(NONLINEAR_LOOP_PREFIX) => {
                    index
                        .nonlinear
                        .extend(refs.iter().filter_map(Value::as_str));
                }
                _ => {}
            }
        }
        index
    }

    fn with_id(&self, id: &str) -> &[&'a Value] {
        self.by_id.get(id).map(Vec::as_slice).unwrap_or(&[])
    }

    /// §2.6's integrity diagnostic: exactly one diagnostic with the case's id,
    /// and its `affected_refs` exactly `[case]`.
    fn integrity(&self, case_id: &str) -> Option<&'a Value> {
        let [d] = self.with_id(&format!("{INTEGRITY_ID_PREFIX}{case_id}")) else {
            return None;
        };
        let refs = d["affected_refs"].as_array()?;
        (refs.len() == 1 && refs[0].as_str() == Some(case_id)).then_some(*d)
    }

    fn related(&self, code: &'static str, case_id: &str) -> &[&'a Value] {
        self.by_code_case
            .get(code)
            .and_then(|cases| cases.get(case_id))
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }
}

fn numerical_health(source: &Value, reader_findings: Option<&Value>) -> Health {
    let route = route_of(source);
    let index = Diagnostics::new(source);
    let mut cases = Vec::new();
    let mut seen = HashSet::new();
    for entry in array(&source["numerical_quality"]["cases"]) {
        // One entry per case id; a duplicate is refused by `verdict_only`.
        let Some(case_id) = load_case_id(entry) else {
            continue;
        };
        if seen.insert(case_id.to_string()) {
            cases.push(case_health(
                &index,
                case_id,
                state_of(entry["solve_quality"].as_str()),
            ));
        }
    }
    // A load case the envelope names but `numerical_quality` omits is not
    // assessed (§2.6 "Grey").
    for case_id in named_load_cases(source) {
        if seen.insert(case_id.to_string()) {
            cases.push(case_health(&index, case_id, "not_assessed"));
        }
    }
    let states: HashMap<&str, &'static str> = cases
        .iter()
        .map(|case| (case.case_id.as_str(), case.state))
        .collect();
    let combinations = combinations(source, &states);
    Health {
        route,
        verdict_only: verdict_only(route, source, &index, reader_findings),
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

/// The load cases the envelope itself names, in first-seen order: its
/// load-case result rows, its preview evidence and its integrity diagnostics.
fn named_load_cases(source: &Value) -> Vec<&str> {
    let mut seen = HashSet::new();
    let rows = array(&source["results"]).iter().filter_map(load_case_id);
    let preview = array(&source["contract_evidence"]["preview_cases"])
        .iter()
        .filter_map(|c| c["load_case_id"].as_str());
    let integrity = array(&source["diagnostics"]).iter().filter_map(|d| {
        d["id"]
            .as_str()
            .and_then(|id| id.strip_prefix(INTEGRITY_ID_PREFIX))
    });
    rows.chain(preview)
        .chain(integrity)
        .filter(|id| seen.insert(*id))
        .collect()
}

fn case_health(index: &Diagnostics, case_id: &str, state: &'static str) -> CaseHealth {
    let integrity = index.integrity(case_id);
    let code = integrity.and_then(|d| d["code"].as_str());
    let integrity_id = integrity.and_then(|d| d["id"].as_str()).map(String::from);
    let report = integrity
        .filter(|_| matches!(code, Some(CHECKS_PASSED | SENSITIVE)))
        .and_then(|d| d["message"].as_str())
        .and_then(|message| Report::parse(message, case_id));
    let linear = !index.nonlinear.contains(case_id) && report.as_ref().is_some_and(|r| r.linear);
    let rcond = report
        .as_ref()
        .filter(|_| linear)
        .and_then(|r| parse_rcond(r.span));
    let conditioning_sensitive = rcond.is_some_and(|r| r < f64::EPSILON.sqrt());
    let sensitive = code == Some(SENSITIVE);

    let mut causes = Vec::new();
    // Every cause the integrity record carries is a demotion's: SENSITIVE only.
    if conditioning_sensitive && sensitive {
        causes.push(Cause {
            kind: "conditioning",
            diagnostic_id: integrity_id.clone(),
            locations: Vec::new(),
        });
    }
    for d in index.related(LOAD_CONTRIBUTION_ABSORBED, case_id) {
        causes.push(Cause {
            kind: "load_fidelity",
            diagnostic_id: d["id"].as_str().map(String::from),
            locations: extra_refs(d, case_id),
        });
    }
    if let Some(records) = report
        .as_ref()
        .filter(|_| sensitive)
        .map(|r| r.records(case_id))
    {
        if let Some(locations) = records.formation {
            causes.push(Cause {
                kind: "formation",
                diagnostic_id: integrity_id.clone(),
                locations,
            });
        }
        if let Some(locations) = records.formation_recovery {
            causes.push(Cause {
                kind: "formation_recovery",
                diagnostic_id: integrity_id.clone(),
                locations,
            });
        }
    }
    for d in index.related(CURVED_BEND_SHORT, case_id) {
        causes.push(Cause {
            kind: "curved_bend_short",
            diagnostic_id: d["id"].as_str().map(String::from),
            locations: extra_refs(d, case_id),
        });
    }
    let coded = match code {
        Some(RECOVERY_BASIS_UNQUALIFIED) => Some("recovery_basis_unqualified"),
        Some(PHYSICAL_MECHANISM) => Some("mechanism"),
        Some(NEGATIVE_ENERGY) => Some("negative_energy"),
        _ => None,
    };
    if let Some(kind) = coded {
        causes.push(Cause {
            kind,
            diagnostic_id: integrity_id.clone(),
            locations: Vec::new(),
        });
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

fn extra_refs(diagnostic: &Value, case_id: &str) -> Vec<Location> {
    let mut seen = HashSet::new();
    array(&diagnostic["affected_refs"])
        .iter()
        .skip(1)
        .filter_map(Value::as_str)
        .filter(|r| *r != case_id && seen.insert(*r))
        .map(|r| Location::Ref {
            reference: r.to_string(),
        })
        .collect()
}

/// A parsed integrity message of `append_integrity_report`'s form.
struct Report<'m> {
    /// The report span: after the exact lead, up to `; global_dof_map=`.
    span: &'m str,
    /// Whether the base message is the linear one.
    linear: bool,
    /// The DOF map's labels and the appended records (the text after the base
    /// message's closing sentence), when the DOF map parses and round-trips
    /// and the fixed text after it is the linear or nonlinear base.
    tail: Option<(Labels, &'m str)>,
}

/// The causes the appended records state: `Some(locations)` when present.
#[derive(Default)]
struct Records {
    formation: Option<Vec<Location>>,
    formation_recovery: Option<Vec<Location>>,
}

impl<'m> Report<'m> {
    fn parse(message: &'m str, case_id: &str) -> Option<Self> {
        let body = message
            .strip_prefix(LEAD_A)?
            .strip_prefix(case_id)?
            .strip_prefix(LEAD_B)?;
        let end = body.find(DOF_MAP_FIELD)?;
        let after = &body[end + DOF_MAP_FIELD.len()..];
        let mut linear = false;
        let tail = parse_debug_string_list(after).and_then(|(labels, rest)| {
            let rest = rest.strip_prefix(". ")?;
            let records = if let Some(records) = rest
                .strip_prefix(LINEAR_TEXT)
                .and_then(|r| r.strip_prefix(CLAIM_TEXT))
            {
                linear = true;
                records
            } else {
                // The nonlinear equilibrium evidence is numbers and fixed text.
                let close = rest.find(CLAIM_TEXT)?;
                if !rest[..close].ends_with(NONLINEAR_TEXT_END) {
                    return None;
                }
                &rest[close + CLAIM_TEXT.len()..]
            };
            Some((Labels::new(labels), records))
        });
        Some(Report {
            span: &body[..end],
            linear: tail.is_some() && linear,
            tail,
        })
    }

    /// The appended records, in the producer's order: an S11-G load-row
    /// sentence or K-D5's line first; then optional range-scaling and
    /// nonlinear evidence lines; an S11-G recovery demotion last. Under the
    /// guards' no-op rule at most one demotion record is present.
    fn records(&self, case_id: &str) -> Records {
        let Some((labels, records)) = &self.tail else {
            return Records::default();
        };
        let mut out = Records::default();
        let load_row = [LOAD_ROW_A, case_id, SENSITIVE_BECAUSE].concat();
        if let Some(text) = records.strip_prefix(load_row.as_str()) {
            let located = parse_load_rows(text, labels);
            out.formation = Some(located.unwrap_or_default());
            return out;
        }
        if let Some(text) = records.strip_prefix(KD5_LINE) {
            let located = text
                .strip_prefix(KD5_ESTIMATE)
                .and_then(|row| labels.prefix(row, KD5_ROW_END))
                .and_then(|(label, _)| node_dof(label));
            out.formation = Some(located.into_iter().collect());
            return out;
        }
        if records.ends_with(MEMBER_RELATIVE_UNAVAILABLE) {
            out.formation_recovery = Some(Vec::new());
            return out;
        }
        // The S11-G recovery sentence is the final record: after the base
        // message, K-D5-free and load-row-free records, the nonlinear
        // confirmation line or a range-scaling line. The cause is recognized
        // by that position and the message's list ending (no earlier record
        // ends in `]`); its groups are parsed separately and only when the
        // lead occurs once.
        if !ends_as_list(records) {
            return out;
        }
        let lead = [RECOVERY_A, case_id, SENSITIVE_BECAUSE].concat();
        let confirmed = [" ", NONLINEAR_CONFIRMED_LINE].concat();
        let mut occurrences = 0;
        let mut recognized = None;
        for (start, _) in records.match_indices(lead.as_str()) {
            occurrences += 1;
            let positioned =
                start == 0 || records[..start] == *confirmed || records.starts_with(RANGE_SCALING);
            if recognized.is_none()
                && positioned
                && counted_list(&records[start + lead.len()..], RECOVERY_TAIL).is_some()
            {
                recognized = Some(start + lead.len());
            }
        }
        if let Some(after) = recognized {
            // A lead inside a group's text makes the groups ambiguous; an
            // unparseable list keeps the cause without locations.
            out.formation_recovery = Some(if occurrences == 1 {
                parse_recovery_groups(&records[after..]).unwrap_or_default()
            } else {
                Vec::new()
            });
        }
        out
    }
}

/// The DOF map's labels, for exact lookups.
struct Labels {
    set: HashSet<String>,
    /// The labels' distinct byte lengths, ascending.
    lengths: Vec<usize>,
}

#[cfg(test)]
thread_local! {
    /// Bytes examined by `Labels::prefix` (a wall-clock-independent bound).
    static LABEL_WORK: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

impl Labels {
    fn new(labels: Vec<String>) -> Self {
        let mut lengths: Vec<usize> = labels.iter().map(String::len).collect();
        lengths.sort_unstable();
        lengths.dedup();
        Labels {
            set: labels.into_iter().collect(),
            lengths,
        }
    }

    /// The unique label at the start of `text` followed by `separator`, and the
    /// text after the separator. Only spans of a known label length that end
    /// at the separator are hashed, so the work is bounded by the labels'
    /// distinct lengths (at most the DOF map's size), not by the text.
    fn prefix<'t>(&self, text: &'t str, separator: &str) -> Option<(&'t str, &'t str)> {
        let mut found = None;
        for &length in &self.lengths {
            if length > text.len() {
                break;
            }
            #[cfg(test)]
            LABEL_WORK.with(|work| work.set(work.get() + 1));
            if !text.is_char_boundary(length) || !text[length..].starts_with(separator) {
                continue;
            }
            #[cfg(test)]
            LABEL_WORK.with(|work| work.set(work.get() + length));
            if self.set.contains(&text[..length]) {
                if found.is_some() {
                    return None;
                }
                found = Some(length);
            }
        }
        found.map(|length| (&text[..length], &text[length + separator.len()..]))
    }
}

/// The S11-G load-row list (`{N}{LOAD_ROW_TAIL}` already past the lead): each
/// item a DOF-map label and one of the guard's three reasons; the list closed
/// by `]`, the optional overflow suffix, and the record's end. The named rows
/// as locations, or `None` when anything fails to parse.
fn parse_load_rows(text: &str, labels: &Labels) -> Option<Vec<Location>> {
    let (count, mut rest) = counted_list(text, LOAD_ROW_TAIL)?;
    let mut located = Vec::new();
    loop {
        if located.len() == NAMED {
            return None;
        }
        let (label, after) = labels.prefix(rest, ": ")?;
        rest = parse_row_reason(after)?;
        located.push(node_dof(label)?);
        if let Some(next) = rest.strip_prefix("; ") {
            rest = next;
            continue;
        }
        rest = rest.strip_prefix(']')?;
        break;
    }
    rest = strip_and_more(rest)?;
    let ends = rest.is_empty() || rest.starts_with(RANGE_SCALING);
    (ends && located.len() == count.min(NAMED)).then(|| {
        let mut seen = HashSet::new();
        located
            .into_iter()
            .filter(|l| seen.insert(l.clone()))
            .collect()
    })
}

/// One load-row reason (`formation_guard::decide_row`); the text after it.
fn parse_row_reason(text: &str) -> Option<&str> {
    if let Some(rest) = text.strip_prefix(ROW_RANGE) {
        return paren_close(rest);
    }
    if let Some(rest) = text.strip_prefix(ROW_CANNOT_BOUND) {
        return parse_debug_string_list(rest)?.1.strip_prefix(')');
    }
    let mut rest = text.strip_prefix(ROW_DEFECT[0])?;
    for literal in &ROW_DEFECT[1..] {
        rest = number(rest)?.strip_prefix(literal)?;
    }
    Some(parse_debug_string_list(rest)?.1)
}

/// The S11-G recovery list (`{N}{RECOVERY_TAIL}` already past the lead), which
/// must end the message. Group names as text, or `None`.
fn parse_recovery_groups(text: &str) -> Option<Vec<Location>> {
    let (count, mut rest) = counted_list(text, RECOVERY_TAIL)?;
    let mut groups = Vec::new();
    loop {
        if groups.len() == NAMED {
            return None;
        }
        // A group name is `{member}.{location}.{kind}`: it ends at the first
        // reason; a name holding "; " or "]" is not parsed.
        let end = [GROUP_Q[0], GROUP_COMPARISON, GROUP_BOUND]
            .iter()
            .filter_map(|m| rest.find(m))
            .min()?;
        let name = &rest[..end];
        if name.is_empty() || name.contains("; ") || name.contains(']') {
            return None;
        }
        let reason = &rest[end..];
        rest = if let Some(r) = reason.strip_prefix(GROUP_Q[0]) {
            number(number(r)?.strip_prefix(GROUP_Q[1])?)?
        } else if let Some(r) = reason.strip_prefix(GROUP_COMPARISON) {
            let reason = COMPARISON_REASONS.iter().find(|c| r.starts_with(**c))?;
            r[reason.len()..].strip_prefix(')')?
        } else {
            paren_close(reason.strip_prefix(GROUP_BOUND)?)?
        };
        groups.push(Location::Text {
            text: name.to_string(),
        });
        if let Some(next) = rest.strip_prefix("; ") {
            rest = next;
            continue;
        }
        rest = rest.strip_prefix(']')?;
        break;
    }
    (strip_and_more(rest)?.is_empty() && groups.len() == count.min(NAMED)).then(|| {
        let mut seen = HashSet::new();
        groups
            .into_iter()
            .filter(|g| seen.insert(g.clone()))
            .collect()
    })
}

/// A number token in `{:e}` or `{:?}` form; the text after it.
fn number(text: &str) -> Option<&str> {
    let end = text
        .find(|c: char| !(c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '+')))
        .unwrap_or(text.len());
    (end > 0).then(|| &text[end..])
}

/// A parenthesized free reason: the text after the first `)` that closes an
/// item (`)` followed by `; ` or `]`).
fn paren_close(text: &str) -> Option<&str> {
    text.match_indices(')')
        .map(|(i, _)| &text[i + 1..])
        .find(|rest| rest.starts_with("; ") || rest.starts_with(']'))
}

/// Whether the records end as a guard's list: `]`, or `]; and {n} more`.
fn ends_as_list(records: &str) -> bool {
    let end = match records.strip_suffix(AND_MORE[1]) {
        Some(rest) => {
            let digits = rest.bytes().rev().take_while(u8::is_ascii_digit).count();
            match rest[..rest.len() - digits].strip_suffix(AND_MORE[0]) {
                Some(rest) if digits > 0 => rest,
                _ => return false,
            }
        }
        None => records,
    };
    end.ends_with(']')
}

/// The guards' optional `; and {n} more`.
fn strip_and_more(text: &str) -> Option<&str> {
    match text.strip_prefix(AND_MORE[0]) {
        None => Some(text),
        Some(rest) => {
            let digits = rest.bytes().take_while(u8::is_ascii_digit).count();
            (digits > 0)
                .then(|| rest[digits..].strip_prefix(AND_MORE[1]))
                .flatten()
        }
    }
}

/// §2.6: `, reciprocal_condition_estimate: ` exactly once in the span,
/// followed by a number token and `, residual_rows: `; the token is Rust's
/// shortest round-trip `f64` form of a finite value with 0 < rcond <= 1.
fn parse_rcond(span: &str) -> Option<f64> {
    let mut found = span.match_indices(RCOND_FIELD);
    let (start, _) = found.next()?;
    if found.next().is_some() {
        return None;
    }
    let rest = &span[start + RCOND_FIELD.len()..];
    let end = rest.find(',')?;
    if !rest[end..].starts_with(RESIDUAL_FIELD) {
        return None;
    }
    let token = &rest[..end];
    let value: f64 = token.parse().ok()?;
    (format!("{value:?}") == token && value.is_finite() && value > 0.0 && value <= 1.0)
        .then_some(value)
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
                                if hex.len() == 6 {
                                    return None;
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

/// A DOF-map label `{node}:{DOF}` (`integrity_dof_label`) as a location.
fn node_dof(label: &str) -> Option<Location> {
    let (node, dof) = label.rsplit_once(':')?;
    (!node.is_empty() && DOF_NAMES.contains(&dof)).then(|| Location::NodeDof {
        node: node.to_string(),
        dof: dof.to_string(),
    })
}

/// `{N}{tail}` at the start of `text`: the count and the list after the tail.
fn counted_list<'t>(text: &'t str, tail: &str) -> Option<(usize, &'t str)> {
    let digits = text.bytes().take_while(u8::is_ascii_digit).count();
    let count = text[..digits].parse().ok()?;
    Some((count, text[digits..].strip_prefix(tail)?))
}

/// §2.6's result-level flag: a presentation projection, never admission.
fn verdict_only(
    route: &str,
    source: &Value,
    index: &Diagnostics,
    reader_findings: Option<&Value>,
) -> bool {
    route == "generic"
        && reader_findings.is_some_and(is_verdict_pair)
        && quality_consistent(source, index)
}

/// The reader's findings are exactly `NUMERICAL_INTEGRITY_NOT_QUALIFIED` and
/// `NUMERICAL_CASE_EVIDENCE_INCOMPLETE`.
fn is_verdict_pair(findings: &Value) -> bool {
    let list = findings
        .as_array()
        .or_else(|| findings["standing"]["findings"].as_array())
        .or_else(|| findings["findings"].as_array());
    let Some(list) = list else { return false };
    let codes: Option<Vec<&str>> = list
        .iter()
        .map(|f| f.as_str().or_else(|| f["code"].as_str()))
        .collect();
    let Some(mut codes) = codes else { return false };
    codes.sort_unstable();
    codes == [EVIDENCE_INCOMPLETE, NOT_QUALIFIED]
}

/// §2.6's evidence clauses over `numerical_quality` and the diagnostics.
fn quality_consistent(source: &Value, index: &Diagnostics) -> bool {
    const ORDER: [&str; 5] = [
        "checks_passed",
        "sensitive",
        "not_assessed",
        "unresolved",
        "failed",
    ];
    let quality = &source["numerical_quality"];
    let Some(cases) = quality["cases"]
        .as_array()
        .filter(|cases| !cases.is_empty())
    else {
        return false;
    };
    let Some(diagnostics) = source["diagnostics"].as_array() else {
        return false;
    };
    // Every emitted diagnostic id is a unique non-empty string.
    if diagnostics
        .iter()
        .any(|d| d["id"].as_str().map_or(true, str::is_empty))
        || index.by_id.values().any(|ds| ds.len() != 1)
    {
        return false;
    }
    let mut case_ids = HashSet::new();
    let mut worst = 0;
    for case in cases {
        let Some(case_id) = load_case_id(case) else {
            return false;
        };
        if !case_ids.insert(case_id) {
            return false;
        }
        let Some(rank) = ORDER.iter().position(|s| case["solve_quality"] == *s) else {
            return false;
        };
        worst = worst.max(rank);
        let refs = array(&case["evidence_refs"]);
        if refs.is_empty()
            || !refs
                .iter()
                .all(|r| r.as_str().is_some_and(|r| index.by_id.contains_key(r)))
        {
            return false;
        }
        let code = index
            .with_id(&format!("{INTEGRITY_ID_PREFIX}{case_id}"))
            .first()
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
    if named_load_cases(source)
        .iter()
        .any(|id| !case_ids.contains(id))
    {
        return false;
    }
    quality["status"] == ORDER[worst]
}

/// `assessed_numerical_quality`'s fields from the case's integrity code
/// (`None`: no integrity diagnostic; `Some(None)`: a code that is not text).
fn derived_fields(
    code: Option<Option<&str>>,
) -> (&'static str, &'static str, &'static str, &'static str) {
    match code {
        Some(Some(CHECKS_PASSED)) => (
            "passive_model_basis",
            "checks_passed",
            "represented_equations_retained",
            "not_claimed",
        ),
        Some(Some(SENSITIVE)) => (
            "passive_model_basis",
            "sensitive",
            "represented_equations_retained",
            "not_claimed",
        ),
        Some(Some(PHYSICAL_MECHANISM)) => (
            "physical_mechanism_witnessed",
            "failed",
            "not_assessed",
            "not_claimed",
        ),
        Some(Some(NEGATIVE_ENERGY)) => (
            "negative_energy_witnessed",
            "failed",
            "not_assessed",
            "not_claimed",
        ),
        Some(Some(RECOVERY_BASIS_UNQUALIFIED)) => (
            "numerically_unresolved",
            "unresolved",
            "not_assessed",
            "unresolved",
        ),
        Some(Some("NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED")) => (
            "numerically_unresolved",
            "unresolved",
            "assembly_loss_detected",
            "unresolved",
        ),
        Some(Some("NUMERICAL_INTEGRITY_FAILED")) => (
            "numerically_unresolved",
            "failed",
            "assembly_uncertainty",
            "unresolved",
        ),
        Some(_) => (
            "numerically_unresolved",
            "unresolved",
            "assembly_uncertainty",
            "unresolved",
        ),
        None => (
            "numerically_unresolved",
            "not_assessed",
            "not_assessed",
            "not_claimed",
        ),
    }
}

/// A combination row's resolution: the load cases at its leaves, and whether
/// every branch reached a load case.
#[derive(Clone, Default)]
struct Resolution<'a> {
    cases: BTreeSet<&'a str>,
    unresolved: bool,
}

impl<'a> Resolution<'a> {
    fn merge(&mut self, other: &Resolution<'a>) {
        self.cases.extend(other.cases.iter().copied());
        self.unresolved |= other.unresolved;
    }
}

/// Resolves the combination row `start` to its load-case leaves, iteratively,
/// memoizing finished rows (shared ancestry) apart from rows still on the
/// path (a cycle, including a self-reference). A branch with no source refs,
/// a missing or malformed ref, a cycle or a non-load-case leaf is unresolved.
fn resolve<'a>(
    start: &'a str,
    by_id: &HashMap<&'a str, &'a Value>,
    memo: &mut HashMap<&'a str, Option<Resolution<'a>>>,
) -> Resolution<'a> {
    if let Some(Some(done)) = memo.get(start) {
        return done.clone();
    }
    memo.insert(start, None);
    let mut stack: Vec<(&'a str, usize, Resolution<'a>)> = vec![(start, 0, Resolution::default())];
    while let Some(&(id, next, _)) = stack.last() {
        let refs = by_id
            .get(id)
            .copied()
            .map_or(&[][..], |row: &'a Value| array(&row["source_result_refs"]));
        if next >= refs.len() {
            let (id, _, mut done) = stack.pop().unwrap();
            if refs.is_empty() {
                done.unresolved = true;
            }
            memo.insert(id, Some(done.clone()));
            match stack.last_mut() {
                Some(parent) => parent.2.merge(&done),
                None => return done,
            }
            continue;
        }
        let reference = &refs[next];
        let top = stack.len() - 1;
        stack[top].1 += 1;
        let Some(source_row) = reference
            .as_str()
            .and_then(|r| by_id.get_key_value(r))
            .map(|(r, row)| (*r, *row))
        else {
            stack[top].2.unresolved = true;
            continue;
        };
        match (
            source_row.1["basis_ref"]["ref_type"].as_str(),
            source_row.1["basis_ref"]["ref_id"].as_str(),
        ) {
            (Some("load_case"), Some(case)) => {
                stack[top].2.cases.insert(case);
            }
            (Some("combination"), Some(_)) => match memo.get(source_row.0) {
                Some(Some(done)) => {
                    let done = done.clone();
                    stack[top].2.merge(&done);
                }
                Some(None) => stack[top].2.unresolved = true,
                None => {
                    memo.insert(source_row.0, None);
                    stack.push((source_row.0, 0, Resolution::default()));
                }
            },
            _ => stack[top].2.unresolved = true,
        }
    }
    unreachable!("the start row is popped last")
}

/// §2 decision 5: a combination inherits the worst badge of the load cases
/// its rows are formed from (red > grey > green); a named case without a
/// state, or an unresolved branch, reads not assessed. A withheld combination
/// publishes no rows and gets no state.
fn combinations(source: &Value, states: &HashMap<&str, &'static str>) -> Vec<CombinationHealth> {
    let results = array(&source["results"]);
    let mut by_id: HashMap<&str, &Value> = HashMap::new();
    let mut rows: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut ids: Vec<(&str, bool)> = Vec::new();
    let mut seen = HashSet::new();
    for gate in array(&source["contract_evidence"]["combination_gates"]) {
        if let Some(id) = gate["combination_id"].as_str() {
            if seen.insert(id) {
                ids.push((id, gate["withheld"] == true));
            }
        }
    }
    for row in results {
        let Some(id) = row["id"].as_str() else {
            continue;
        };
        by_id.entry(id).or_insert(row);
        if let (Some("combination"), Some(combination)) = (
            row["basis_ref"]["ref_type"].as_str(),
            row["basis_ref"]["ref_id"].as_str(),
        ) {
            // A modulus-basis presence record (`append_combination_modulus_basis_records`)
            // carries no value formed by result algebra: it is not a branch.
            if row["kind"] != MODULUS_BASIS_RECORD {
                rows.entry(combination).or_default().push(id);
            }
            if seen.insert(combination) {
                ids.push((combination, false));
            }
        }
    }
    let order: HashMap<&str, usize> = array(&source["numerical_quality"]["cases"])
        .iter()
        .filter_map(load_case_id)
        .enumerate()
        .map(|(i, id)| (id, i))
        .collect();
    let mut memo = HashMap::new();
    ids.into_iter()
        .map(|(combination_id, withheld)| {
            let mut resolution = Resolution::default();
            let branches = rows.get(combination_id).map(Vec::as_slice).unwrap_or(&[]);
            for &row in branches {
                let branch = resolve(row, &by_id, &mut memo);
                resolution.merge(&branch);
            }
            let mut cases: Vec<&str> = resolution.cases.into_iter().collect();
            cases.sort_by_key(|c| (order.get(c).copied().unwrap_or(usize::MAX), *c));
            let state = (!withheld).then(|| {
                let mut worst = if cases.is_empty() || resolution.unresolved {
                    badge_rank("not_assessed")
                } else {
                    0
                };
                for case in &cases {
                    worst = worst.max(badge_rank(
                        states.get(case).copied().unwrap_or("not_assessed"),
                    ));
                }
                BADGE_ORDER[worst]
            });
            CombinationHealth {
                combination_id: combination_id.to_string(),
                state,
                cases: cases.into_iter().map(String::from).collect(),
                withheld,
            }
        })
        .collect()
}

/// The combination presence record kind that names an operand's solve basis.
pub(crate) const MODULUS_BASIS_RECORD: &str = "combination_modulus_basis_record";

/// Worst-badge order: green < grey < red, the reds in the engine's order.
const BADGE_ORDER: [&str; 5] = [
    "checks_passed",
    "not_assessed",
    "sensitive",
    "quality_unresolved",
    "solve_failed",
];
fn badge_rank(state: &str) -> usize {
    BADGE_ORDER.iter().position(|s| *s == state).unwrap_or(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const MULTICASE: &str = include_str!(
        "../../../../fixtures/product_preview/w1_controls/ui/multicase-sparse_interactive.raw.json"
    );
    const PAIR: &str =
        r#"["NUMERICAL_INTEGRITY_NOT_QUALIFIED","NUMERICAL_CASE_EVIDENCE_INCOMPLETE"]"#;

    /// A report message in `append_integrity_report`'s linear form.
    fn message(
        case: &str,
        quality: &str,
        rcond: &str,
        after_rcond: &str,
        map: &str,
        tail: &str,
    ) -> String {
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
        serde_json::from_str(&numerical_health_json(&source.to_string(), findings).unwrap())
            .unwrap()
    }

    fn one_case(case: &str, code: &str, solve_quality: &str, message: String) -> Value {
        let out = health(
            &envelope(case, solve_quality, vec![integrity(case, code, message)]),
            None,
        );
        out["cases"][0].clone()
    }

    #[test]
    fn a_linear_report_yields_its_rcond_and_digits() {
        let c = one_case(
            "case",
            CHECKS_PASSED,
            "checks_passed",
            message(
                "case",
                "Passed",
                &format!("{:?}", 1.0578136350861028e-3_f64),
                RESIDUAL_FIELD,
                MAP,
                "",
            ),
        );
        assert_eq!(c["state"], "checks_passed");
        assert_eq!(c["code"], CHECKS_PASSED);
        assert_eq!(
            c["integrity_diagnostic_id"],
            "diagnostic:numerical-integrity:case"
        );
        assert_eq!(
            c["rcond"].as_f64().unwrap().to_bits(),
            1.0578136350861028e-3_f64.to_bits()
        );
        assert_eq!(
            c["digits"].as_f64().unwrap(),
            -(1.0578136350861028e-3_f64.log10())
        );
        assert_eq!(c["conditioning_sensitive"], false);
        assert_eq!(c["causes"], json!([]));
    }

    #[test]
    fn the_conditioning_cause_is_the_kernels_comparison_on_the_parsed_value() {
        let line = f64::EPSILON.sqrt();
        let below = f64::from_bits(line.to_bits() - 1);
        for (value, sensitive) in [(below, true), (line, false), (1.0578136350861028e-11, true)] {
            let c = one_case(
                "case",
                SENSITIVE,
                "sensitive",
                message(
                    "case",
                    "Sensitive",
                    &format!("{value:?}"),
                    RESIDUAL_FIELD,
                    MAP,
                    "",
                ),
            );
            assert_eq!(c["rcond"].as_f64().unwrap().to_bits(), value.to_bits());
            assert_eq!(c["conditioning_sensitive"], sensitive, "{value:?}");
            assert_eq!(
                c["causes"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|k| k["kind"] == "conditioning"),
                sensitive
            );
        }
        // The cause is a demotion's: under CHECKS_PASSED the flag stands alone.
        let c = one_case(
            "case",
            CHECKS_PASSED,
            "checks_passed",
            message("case", "Passed", "1e-11", RESIDUAL_FIELD, MAP, ""),
        );
        assert_eq!(
            (c["conditioning_sensitive"].clone(), c["causes"].clone()),
            (json!(true), json!([]))
        );
    }

    #[test]
    fn rcond_one_is_zero_digits_and_out_of_range_tokens_give_no_figure() {
        let c = one_case(
            "case",
            CHECKS_PASSED,
            "checks_passed",
            message("case", "Passed", "1.0", RESIDUAL_FIELD, MAP, ""),
        );
        assert_eq!(c["rcond"], json!(1.0));
        assert_eq!(c["digits"].as_f64().unwrap().to_bits(), 0.0_f64.to_bits());
        for token in [
            "0.0",
            "-0.0",
            "-1e-5",
            "NaN",
            "inf",
            "1.0000000000000002",
            "2.0",
            "1e-400",
            "0.50",
            "5e-1",
            "1",
            "x",
            "1.0578136350861028e-3",
            "+0.5",
            " 0.5",
        ] {
            let c = one_case(
                "case",
                CHECKS_PASSED,
                "checks_passed",
                message("case", "Passed", token, RESIDUAL_FIELD, MAP, ""),
            );
            assert_eq!(
                (c["rcond"].clone(), c["digits"].clone()),
                (Value::Null, Value::Null),
                "{token}"
            );
            assert_eq!(
                c["state"], "checks_passed",
                "a missing figure never changes a state"
            );
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
        let c = one_case(
            "a",
            CHECKS_PASSED,
            "checks_passed",
            message("b", "Passed", "0.125", RESIDUAL_FIELD, MAP, ""),
        );
        assert_eq!(c["rcond"], Value::Null);
        // The field twice in the span is ambiguous.
        let twice = message(
            "case",
            "Passed",
            "0.125",
            &format!("{RESIDUAL_FIELD}[]{RCOND_FIELD}0.5{RESIDUAL_FIELD}"),
            MAP,
            "",
        );
        assert_eq!(
            one_case("case", CHECKS_PASSED, "checks_passed", twice)["rcond"],
            Value::Null
        );
    }

    #[test]
    fn intended_residual_rows_is_not_the_residual_field() {
        let c = one_case(
            "case",
            CHECKS_PASSED,
            "checks_passed",
            message(
                "case",
                "Passed",
                "0.125",
                ", intended_residual_rows: ",
                MAP,
                "",
            ),
        );
        assert_eq!(c["rcond"], Value::Null);
    }

    #[test]
    fn two_diagnostics_with_the_case_id_give_no_figure_and_no_code() {
        let case = "case";
        let d = integrity(
            case,
            CHECKS_PASSED,
            message(case, "Passed", "0.125", RESIDUAL_FIELD, MAP, ""),
        );
        let out = health(
            &envelope(case, "checks_passed", vec![d.clone(), d]),
            Some(PAIR),
        );
        let c = &out["cases"][0];
        assert_eq!(
            (
                c["rcond"].clone(),
                c["code"].clone(),
                c["integrity_diagnostic_id"].clone()
            ),
            (Value::Null, Value::Null, Value::Null)
        );
        assert_eq!(c["state"], "checks_passed");
        // affected_refs other than exactly [case] also refuse.
        let mut wide = integrity(
            case,
            CHECKS_PASSED,
            message(case, "Passed", "0.125", RESIDUAL_FIELD, MAP, ""),
        );
        wide["affected_refs"] = json!([case, "x"]);
        assert_eq!(
            health(&envelope(case, "checks_passed", vec![wide]), None)["cases"][0]["rcond"],
            Value::Null
        );
    }

    #[test]
    fn only_passed_or_sensitive_codes_carry_a_figure() {
        for code in [
            "NUMERICAL_INTEGRITY_UNRESOLVED",
            RECOVERY_BASIS_UNQUALIFIED,
            "OTHER",
        ] {
            let c = one_case(
                "case",
                code,
                "unresolved",
                message("case", "Passed", "0.125", RESIDUAL_FIELD, MAP, ""),
            );
            assert_eq!(c["rcond"], Value::Null, "{code}");
            assert_eq!(c["state"], "quality_unresolved");
        }
    }

    #[test]
    fn a_debug_quality_passed_under_code_sensitive_still_parses() {
        // S11-G demotion: the code changes, the report's Debug quality does not.
        let tail = " S11-G formation-noise guard: load case case is Sensitive because 2 load row(s) carry formed-term formation noise above the 1e-9 criterion; every value is kept for inspection. Rows: [N1:RX: net formation defect 1e-3, self-equilibrated defect 0e0, bound 0e0, intended net at least 1e0, body scale 1e0, threshold 1e-9 (self-equilibrated threshold 1e-9); formed sources [\"a; N0:UX: range failure (x)\"]; N0:UZ: range failure (overflow)]";
        let c = one_case(
            "case",
            SENSITIVE,
            "sensitive",
            message("case", "Passed", "0.125", RESIDUAL_FIELD, MAP, tail),
        );
        assert_eq!(c["rcond"], json!(0.125));
        assert_eq!(c["conditioning_sensitive"], false);
        // The row list is parsed by its structure: a label-like text inside a
        // Debug source id is not an item.
        let rows = json!([{"kind": "node_dof", "node": "N1", "dof": "RX"}, {"kind": "node_dof", "node": "N0", "dof": "UZ"}]);
        assert_eq!(
            c["causes"],
            json!([{"kind": "formation", "diagnostic_id": "diagnostic:numerical-integrity:case", "locations": rows}])
        );
        let clean = tail.replace("[\"a; N0:UX: range failure (x)\"]", "[\"a\"]");
        let c = one_case(
            "case",
            SENSITIVE,
            "sensitive",
            message("case", "Passed", "0.125", RESIDUAL_FIELD, MAP, &clean),
        );
        assert_eq!(c["causes"][0]["locations"], rows);
        // A miscounted list keeps the cause (its record is at the fixed
        // position) and omits the locations; so does an overflow suffix.
        let miscounted = clean.replace("because 2 load", "because 3 load");
        let c = one_case(
            "case",
            SENSITIVE,
            "sensitive",
            message("case", "Passed", "0.125", RESIDUAL_FIELD, MAP, &miscounted),
        );
        assert_eq!(c["causes"][0]["locations"], json!([]));
        let more = clean.replace("because 2 load", "because 8 load") + "; and 6 more";
        let c = one_case(
            "case",
            SENSITIVE,
            "sensitive",
            message("case", "Passed", "0.125", RESIDUAL_FIELD, MAP, &more),
        );
        assert_eq!(
            c["causes"][0]["locations"],
            json!([]),
            "two named rows are not min(8, 6)"
        );
        // The same sentence under code CHECKS_PASSED is not a demotion.
        let c = one_case(
            "case",
            CHECKS_PASSED,
            "checks_passed",
            message("case", "Passed", "0.125", RESIDUAL_FIELD, MAP, &clean),
        );
        assert_eq!(c["causes"], json!([]));
    }

    #[test]
    fn the_kd5_line_names_its_row_only_when_it_is_a_dof_map_label() {
        let kd5 = |row: &str| {
            format!(" formation_check: reason=estimate; row={row}; doubled_correction=8.09e-14; scale=3.3e-5; trigger_ratio=2.4")
        };
        let locations = |row: &str| {
            one_case(
                "case",
                SENSITIVE,
                "sensitive",
                message("case", "Sensitive", "0.125", RESIDUAL_FIELD, MAP, &kd5(row)),
            )["causes"][0]["locations"]
                .clone()
        };
        assert_eq!(
            locations("N1:RX"),
            json!([{"kind": "node_dof", "node": "N1", "dof": "RX"}])
        );
        for row in ["N9:RX", "global_dof=7", "none"] {
            assert_eq!(locations(row), json!([]), "{row}");
        }
        let unavailable = " formation_check: reason=formation_check_unavailable; detail=x";
        let c = one_case(
            "case",
            SENSITIVE,
            "sensitive",
            message(
                "case",
                "Sensitive",
                "0.125",
                RESIDUAL_FIELD,
                MAP,
                unavailable,
            ),
        );
        assert_eq!(
            c["causes"],
            json!([{"kind": "formation", "diagnostic_id": "diagnostic:numerical-integrity:case", "locations": []}])
        );
        // Node ids with colons and escapes decode through the Debug map.
        let map = r#"["a:b\"c:UX", "a:b\"c:UY"]"#;
        let c = one_case(
            "case",
            SENSITIVE,
            "sensitive",
            message(
                "case",
                "Sensitive",
                "0.125",
                RESIDUAL_FIELD,
                map,
                &kd5("a:b\"c:UY"),
            ),
        );
        assert_eq!(
            c["causes"][0]["locations"],
            json!([{"kind": "node_dof", "node": "a:b\"c", "dof": "UY"}])
        );
    }

    #[test]
    fn recovery_demotions_name_their_groups_as_text() {
        let tail = " S11-G recovery guard (R-b'): load case case is Sensitive because 2 member or support group(s) of the certified rows exceed the 1e-9 member-relative criterion or have an unavailable comparison; every value is kept for inspection. Groups: [pipe:P.end_i.force: q=1e0, B=2e-9; support:S.node.moment: the member bound is unavailable (x)]";
        let c = one_case(
            "case",
            SENSITIVE,
            "sensitive",
            message("case", "Passed", "0.125", RESIDUAL_FIELD, MAP, tail),
        );
        assert_eq!(
            c["causes"],
            json!([{"kind": "formation_recovery", "diagnostic_id": "diagnostic:numerical-integrity:case",
            "locations": [{"kind": "text", "text": "pipe:P.end_i.force"}, {"kind": "text", "text": "support:S.node.moment"}]}])
        );
        // The recovery sentence ends the message after unparsed records, so it
        // counts only when it parses exactly to the end.
        let ranged = format!(" range_scaling: force_scale_exponent=3; basis=exact power-of-two; subnormal=reaction@N0:UX:relative_precision=1e-3{tail}");
        let c = one_case(
            "case",
            SENSITIVE,
            "sensitive",
            message("case", "Passed", "0.125", RESIDUAL_FIELD, MAP, &ranged),
        );
        assert_eq!(c["causes"][0]["locations"].as_array().unwrap().len(), 2);
        // Recognition is by position; a list that does not parse keeps the
        // cause without locations. A group name holding `]` (a valid pipe id
        // `pipe]`) is such a list.
        let recovery = json!([{"kind": "formation_recovery", "diagnostic_id": "diagnostic:numerical-integrity:case", "locations": []}]);
        for unparsed in [
            tail.replace("because 2 member", "because 9 member"),
            tail.replace("q=1e0", "q=1 e0"),
            tail.replace("pipe:P.end_i", "pipe].end_i"),
            tail.replace("pipe:P.end_i", "pipe; x.end_i"),
        ] {
            let c = one_case(
                "case",
                SENSITIVE,
                "sensitive",
                message("case", "Passed", "0.125", RESIDUAL_FIELD, MAP, &unparsed),
            );
            assert_eq!(c["causes"], recovery, "{unparsed}");
        }
        // After the nonlinear confirmation line, the position still holds.
        let confirmed = format!(" {NONLINEAR_CONFIRMED_LINE}{tail}");
        let c = one_case(
            "case",
            SENSITIVE,
            "sensitive",
            message("case", "Passed", "0.125", RESIDUAL_FIELD, MAP, &confirmed),
        );
        assert_eq!(c["causes"][0]["locations"].as_array().unwrap().len(), 2);
        // Not the final record, or not at a record position: no cause.
        for misplaced in [format!("{tail} x"), format!(" x{tail}")] {
            let c = one_case(
                "case",
                SENSITIVE,
                "sensitive",
                message("case", "Passed", "0.125", RESIDUAL_FIELD, MAP, &misplaced),
            );
            assert_eq!(c["causes"], json!([]), "{misplaced}");
        }
        let unnamed = " the member-relative formation check is not available on this route; member rows are binary64 recoveries";
        let c = one_case(
            "case",
            SENSITIVE,
            "sensitive",
            message("case", "Passed", "0.125", RESIDUAL_FIELD, MAP, unnamed),
        );
        assert_eq!(
            c["causes"],
            json!([{"kind": "formation_recovery", "diagnostic_id": "diagnostic:numerical-integrity:case", "locations": []}])
        );
        // The nonlinear "not established" demotion names no cause.
        let nonlinear = " the converged support state is not established at 128-bit precision (confirmation attempt: not run); member rows are binary64 recoveries";
        let c = one_case(
            "case",
            SENSITIVE,
            "sensitive",
            message("case", "Passed", "0.125", RESIDUAL_FIELD, MAP, nonlinear),
        );
        assert_eq!(c["causes"], json!([]));
        // A lead inside a real group's name: the demotion is real, its groups ambiguous.
        let nested = tail.replace("[pipe:P.end_i.force", "[pipe:P S11-G recovery guard (R-b'): load case case is Sensitive because 2 member or support group(s) of the certified rows exceed the 1e-9 member-relative criterion or have an unavailable comparison; every value is kept for inspection. Groups: [W.end_i.force");
        let c = one_case(
            "case",
            SENSITIVE,
            "sensitive",
            message("case", "Passed", "0.125", RESIDUAL_FIELD, MAP, &nested),
        );
        assert_eq!(
            c["causes"],
            json!([{"kind": "formation_recovery", "diagnostic_id": "diagnostic:numerical-integrity:case", "locations": []}])
        );
    }

    #[test]
    fn a_nonlinear_report_has_no_figure() {
        let mut m = message("case", "Passed", "0.125", RESIDUAL_FIELD, MAP, "");
        m = m.replace(LINEAR_TEXT, "M03-PRODUCT-EQUILIBRIUM-v1 final same-state evaluated equilibrium and derived residual-work evidence: X");
        let c = one_case("case", CHECKS_PASSED, "checks_passed", m);
        assert_eq!(
            (
                c["rcond"].clone(),
                c["digits"].clone(),
                c["conditioning_sensitive"].clone()
            ),
            (Value::Null, Value::Null, json!(false))
        );
        // A nonlinear support loop naming the case also refuses the figure.
        let case = "case";
        let mut source = envelope(
            case,
            "checks_passed",
            vec![integrity(
                case,
                CHECKS_PASSED,
                message(case, "Passed", "0.125", RESIDUAL_FIELD, MAP, ""),
            )],
        );
        source["diagnostics"].as_array_mut().unwrap().push(json!({"id": "diagnostic:nonlinear:case:loop", "code": "NONLINEAR_SUPPORT_LOOP_CONVERGED", "affected_refs": [case, "DEC-046"]}));
        assert_eq!(health(&source, None)["cases"][0]["rcond"], Value::Null);
    }

    #[test]
    fn located_causes_from_other_diagnostics_and_failure_codes() {
        let case = "load:L";
        let mut source = envelope(
            case,
            "sensitive",
            vec![integrity(
                case,
                SENSITIVE,
                message(case, "Sensitive", "0.125", RESIDUAL_FIELD, MAP, ""),
            )],
        );
        let diagnostics = source["diagnostics"].as_array_mut().unwrap();
        diagnostics.push(json!({"id": "diagnostic:load-fidelity:load-L", "code": LOAD_CONTRIBUTION_ABSORBED, "affected_refs": [case, "load:L-X", "load:L-Y"]}));
        diagnostics.push(json!({"id": "diagnostic:load-fidelity:other", "code": LOAD_CONTRIBUTION_ABSORBED, "affected_refs": ["load:M", "load:M-X"]}));
        diagnostics.push(json!({"id": "diagnostic:curved-bend-short:load-L", "code": CURVED_BEND_SHORT, "affected_refs": [case, "component:B", "pipe:P"]}));
        let c = &health(&source, None)["cases"][0];
        assert_eq!(
            c["causes"],
            json!([
                {"kind": "load_fidelity", "diagnostic_id": "diagnostic:load-fidelity:load-L", "locations": [{"kind": "ref", "ref": "load:L-X"}, {"kind": "ref", "ref": "load:L-Y"}]},
                {"kind": "curved_bend_short", "diagnostic_id": "diagnostic:curved-bend-short:load-L", "locations": [{"kind": "ref", "ref": "component:B"}, {"kind": "ref", "ref": "pipe:P"}]},
            ])
        );
        for (code, quality, state, kind) in [
            (PHYSICAL_MECHANISM, "failed", "solve_failed", "mechanism"),
            (NEGATIVE_ENERGY, "failed", "solve_failed", "negative_energy"),
            (
                RECOVERY_BASIS_UNQUALIFIED,
                "unresolved",
                "quality_unresolved",
                "recovery_basis_unqualified",
            ),
        ] {
            let c = one_case(case, code, quality, format!("Load case {case}: x"));
            assert_eq!(c["state"], state);
            assert_eq!(
                c["causes"],
                json!([{"kind": kind, "diagnostic_id": "diagnostic:numerical-integrity:load:L", "locations": []}])
            );
        }
    }

    #[test]
    fn states_follow_solve_quality_and_absent_cases_are_not_assessed() {
        let mut source = envelope("a", "failed", vec![]);
        source["results"] =
            json!([{"id": "r1", "basis_ref": {"ref_type": "load_case", "ref_id": "b"}}]);
        let out = health(&source, None);
        assert_eq!(out["cases"][0]["state"], "solve_failed");
        assert_eq!(
            (
                out["cases"][1]["case_id"].clone(),
                out["cases"][1]["state"].clone()
            ),
            (json!("b"), json!("not_assessed"))
        );
        // A duplicated case id gives one entry.
        let mut duplicated = envelope("a", "sensitive", vec![]);
        let copy = duplicated["numerical_quality"]["cases"][0].clone();
        duplicated["numerical_quality"]["cases"]
            .as_array_mut()
            .unwrap()
            .push(copy);
        assert_eq!(
            health(&duplicated, None)["cases"].as_array().unwrap().len(),
            1
        );
        for (quality, state) in [
            ("checks_passed", "checks_passed"),
            ("sensitive", "sensitive"),
            ("unresolved", "quality_unresolved"),
            ("not_assessed", "not_assessed"),
            ("bogus", "not_assessed"),
        ] {
            assert_eq!(
                health(&envelope("a", quality, vec![]), None)["cases"][0]["state"],
                state
            );
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
                {"id": "a:modulus", "kind": "combination_modulus_basis_record", "entity_ref": "a", "basis_ref": {"ref_type": "combination", "ref_id": "combination:a"}},
                {"id": "ac:x", "basis_ref": {"ref_type": "combination", "ref_id": "combination:ac"}, "source_result_refs": ["ra", "rc"]},
                {"id": "am:x", "basis_ref": {"ref_type": "combination", "ref_id": "combination:am"}, "source_result_refs": ["ra", "missing"]},
                {"id": "shared:1", "basis_ref": {"ref_type": "combination", "ref_id": "combination:shared"}, "source_result_refs": ["a:x", "ab:x"]},
                {"id": "shared:2", "basis_ref": {"ref_type": "combination", "ref_id": "combination:shared"}, "source_result_refs": ["a:x"]},
                {"id": "cyc:x", "basis_ref": {"ref_type": "combination", "ref_id": "combination:cyc"}, "source_result_refs": ["ra", "cyc:y"]},
                {"id": "cyc:y", "basis_ref": {"ref_type": "combination", "ref_id": "combination:cyc"}, "source_result_refs": ["cyc:x"]},
                {"id": "self:x", "basis_ref": {"ref_type": "combination", "ref_id": "combination:self"}, "source_result_refs": ["ra", "self:x"]},
                {"id": "empty:x", "basis_ref": {"ref_type": "combination", "ref_id": "combination:empty"}, "source_result_refs": []},
                {"id": "empty:y", "basis_ref": {"ref_type": "combination", "ref_id": "combination:empty2"}, "source_result_refs": ["ra"]},
                {"id": "empty:z", "basis_ref": {"ref_type": "combination", "ref_id": "combination:empty2"}}]
        });
        let out = health(&source, None);
        assert_eq!(
            out["combinations"],
            json!([
                {"combination_id": "combination:ab", "state": "sensitive", "cases": ["a", "b"], "withheld": false},
                {"combination_id": "combination:w", "state": null, "cases": [], "withheld": true},
                {"combination_id": "combination:a", "state": "checks_passed", "cases": ["a"], "withheld": false},
                {"combination_id": "combination:ac", "state": "not_assessed", "cases": ["a", "c"], "withheld": false},
                {"combination_id": "combination:am", "state": "not_assessed", "cases": ["a"], "withheld": false},
                // Shared ancestry is not a cycle.
                {"combination_id": "combination:shared", "state": "sensitive", "cases": ["a", "b"], "withheld": false},
                // A cycle, a self-reference, or a branch without sources never reads green.
                {"combination_id": "combination:cyc", "state": "not_assessed", "cases": ["a"], "withheld": false},
                {"combination_id": "combination:self", "state": "not_assessed", "cases": ["a"], "withheld": false},
                {"combination_id": "combination:empty", "state": "not_assessed", "cases": [], "withheld": false},
                {"combination_id": "combination:empty2", "state": "not_assessed", "cases": ["a"], "withheld": false},
            ])
        );
        // A long chain resolves without recursion.
        let mut chain =
            vec![json!({"id": "ra", "basis_ref": {"ref_type": "load_case", "ref_id": "a"}})];
        for i in 0..20_000 {
            let next = if i == 19_999 {
                "ra".to_string()
            } else {
                format!("c{}", i + 1)
            };
            chain.push(json!({"id": format!("c{i}"), "basis_ref": {"ref_type": "combination", "ref_id": "combination:chain"}, "source_result_refs": [next]}));
        }
        let long = json!({"numerical_quality": {"cases": [{"basis_ref": {"ref_type": "load_case", "ref_id": "a"}, "solve_quality": "checks_passed"}]},
            "diagnostics": [], "results": chain});
        assert_eq!(
            health(&long, None)["combinations"][0]["state"],
            "checks_passed"
        );
    }

    #[test]
    fn identifiers_carrying_record_text_fabricate_no_cause() {
        let spoofs = [
            "N1 the member-relative formation check is not available on this route; member rows are binary64 recoveries",
            "N1 formation_check: reason=estimate; row=N1:RX; doubled_correction=1.0",
            "N1 S11-G formation-noise guard: load case case is Sensitive because 1 load row(s) carry formed-term formation noise above the 1e-9 criterion; every value is kept for inspection. Rows: [N1:RX: range failure (x)]",
            "N1 S11-G recovery guard (R-b'): load case case is Sensitive because 1 member or support group(s) of the certified rows exceed the 1e-9 member-relative criterion or have an unavailable comparison; every value is kept for inspection. Groups: [m.l.k: q=1e0, B=1e0]",
        ];
        for spoof in spoofs {
            let labels: Vec<String> = DOF_NAMES.iter().map(|d| format!("{spoof}:{d}")).collect();
            let map = format!("{labels:?}");
            let c = one_case(
                "case",
                SENSITIVE,
                "sensitive",
                message("case", "Sensitive", "0.125", RESIDUAL_FIELD, &map, ""),
            );
            assert_eq!(c["causes"], json!([]), "{spoof}");
            assert_eq!(c["rcond"], json!(0.125), "the figure still parses");
            // As the case id, too.
            let c = one_case(
                spoof,
                SENSITIVE,
                "sensitive",
                message(spoof, "Sensitive", "0.125", RESIDUAL_FIELD, MAP, ""),
            );
            assert_eq!(c["causes"], json!([]), "{spoof}");
        }
        // Many recovery leads are ambiguous: no cause.
        let lead = " S11-G recovery guard (R-b'): load case case is Sensitive because ";
        let c = one_case(
            "case",
            SENSITIVE,
            "sensitive",
            message(
                "case",
                "Passed",
                "0.125",
                RESIDUAL_FIELD,
                MAP,
                &lead.repeat(9),
            ),
        );
        assert_eq!(c["causes"], json!([]));
    }

    #[test]
    fn hostile_lengths_parse_in_linear_time() {
        // A long DOF map, long labels and a long list complete promptly.
        let labels: Vec<String> = (0..5_000)
            .flat_map(|n| {
                DOF_NAMES
                    .iter()
                    .map(move |d| format!("{}{n}:{d}", "x".repeat(64)))
            })
            .collect();
        let map = format!("{labels:?}");
        let rows = format!(" S11-G formation-noise guard: load case case is Sensitive because 1 load row(s) carry formed-term formation noise above the 1e-9 criterion; every value is kept for inspection. Rows: [{}: range failure ({})]", labels[7], "; ".repeat(200_000));
        let c = one_case(
            "case",
            SENSITIVE,
            "sensitive",
            message("case", "Passed", "0.125", RESIDUAL_FIELD, &map, &rows),
        );
        assert_eq!(c["causes"][0]["kind"], "formation");
        let groups = format!(" S11-G recovery guard (R-b'): load case case is Sensitive because 1 member or support group(s) of the certified rows exceed the 1e-9 member-relative criterion or have an unavailable comparison; every value is kept for inspection. Groups: [{}]", ": q=".repeat(200_000));
        let c = one_case(
            "case",
            SENSITIVE,
            "sensitive",
            message("case", "Passed", "0.125", RESIDUAL_FIELD, &map, &groups),
        );
        assert_eq!(c["causes"][0]["locations"], json!([]));
    }

    /// The label lookup's work is bounded by the DOF map, not by separators in
    /// the text: a label of length L before a text of L separators would hash
    /// every growing prefix (about L^2 / 4 bytes) in a quadratic lookup.
    #[test]
    fn label_lookup_work_is_linear() {
        for n in [10_000, 40_000] {
            let long = ": ".repeat(n);
            let labels: Vec<String> = DOF_NAMES
                .iter()
                .map(|d| format!("{long}:{d}"))
                .chain(["N1:RX".to_string()])
                .collect();
            let map = format!("{labels:?}");
            let kd5 = format!(
                " formation_check: reason=estimate; row={}; doubled_correction=1.0",
                ": ".repeat(2 * n)
            );
            let rows = format!(" S11-G formation-noise guard: load case case is Sensitive because 6 load row(s) carry formed-term formation noise above the 1e-9 criterion; every value is kept for inspection. Rows: [{}]", vec![": ".repeat(2 * n); 6].join("; "));
            for tail in [kd5, rows] {
                LABEL_WORK.with(|work| work.set(0));
                let c = one_case(
                    "case",
                    SENSITIVE,
                    "sensitive",
                    message("case", "Sensitive", "0.125", RESIDUAL_FIELD, &map, &tail),
                );
                assert_eq!(c["causes"][0]["locations"], json!([]));
                let work = LABEL_WORK.with(|work| work.get());
                // Linear: a few passes over the map; quadratic would be ~n^2.
                assert!(work <= 8 * (map.len() + tail.len()), "n={n}: work {work}");
            }
        }
    }

    /// The producer text the parser copies occurs verbatim in
    /// `core/product_physics` (compile reads of the producer sources).
    #[test]
    fn producer_text_is_pinned() {
        const LIB: &str = include_str!("../../../product_physics/src/lib.rs");
        const GUARD: &str = include_str!("../../../product_physics/src/formation_guard.rs");
        let lib_pins = [
            "\"{} represented original-equation structural evidence for load case {}: {:?}; global_dof_map={:?}. {} No certified inertia, guaranteed forward accuracy, or pressure/component/stress engineering qualification is claimed.\"".to_string(),
            format!("\"{LINEAR_TEXT}\""),
            format!("const NONLINEAR_CONFIRMED_EVIDENCE_LINE: &str = \"{NONLINEAR_CONFIRMED_LINE}\";"),
            format!("{NONLINEAR_TEXT_END}\",e.policy,e)"),
            "\"formation_check: reason=estimate; row={}; doubled_correction={:?}; scale={:?}; trigger_ratio={:?}\"".to_string(),
            "\"formation_check: reason=formation_check_unavailable; detail={detail}\"".to_string(),
            "\"range_scaling: force_scale_exponent={}; basis=exact power-of-two{named}{more}\"".to_string(),
            "let route = \"on this route\";".to_string(),
            "format!(\"the member-relative formation check is not available {route}; member rows are binary64 recoveries\")".to_string(),
            "format!(\"the converged support state is not established at 128-bit precision ({reason}); member rows are binary64 recoveries\")".to_string(),
            "\"{} {}\",\n            record.message,\n            formation_check_evidence_line(model, check)".to_string(),
            "fn integrity_diagnostic_id(case_id: &str) -> String {\n    format!(\"diagnostic:numerical-integrity:{case_id}\")".to_string(),
            "[\"UX\", \"UY\", \"UZ\", \"RX\", \"RY\", \"RZ\"].map(|dof| format!(\"{}:{dof}\", node.id))".to_string(),
            format!("kind: \"{MODULUS_BASIS_RECORD}\""),
        ];
        let guard_pins = [
            format!("\"S11-G formation-noise guard: load case {{case_id}}{SENSITIVE_BECAUSE}{{}}{LOAD_ROW_TAIL}{{}}]{{}}\""),
            format!("\"S11-G recovery guard (R-b'): load case {{case_id}}{SENSITIVE_BECAUSE}{{}}{RECOVERY_TAIL}{{}}]{{}}\""),
            "format!(\"{}: {reason}\", dof_label(row.dof))".to_string(),
            "format!(\"range failure ({failure})\")".to_string(),
            "\"no conservative formation bound (CannotBound: curved-span consistent load vector from {:?})\"".to_string(),
            "\"net formation defect {net:e}, self-equilibrated defect {self_equilibrated:e}, bound {:e}, intended net at least {:e}, body scale {s_star:e}, threshold {t0:e} (self-equilibrated threshold {tf:e}); formed sources {:?}\"".to_string(),
            "\"{}.{}.{}: q={:e}, B={bound:e}\"".to_string(),
            "\"{}.{}.{}: member comparison unavailable ({reason})\"".to_string(),
            "\"{}.{}.{}: the member bound is unavailable ({reason})\"".to_string(),
            "format!(\"; and {} more\", fired.len() - NAMED)".to_string(),
            "pub(crate) const NAMED: usize = 6;".to_string(),
            "diagnostic.message = format!(\"{} {}\", diagnostic.message, finding.sentence);".to_string(),
        ];
        for pin in &lib_pins {
            assert!(
                LIB.contains(pin.as_str()),
                "product_physics/src/lib.rs: {pin}"
            );
        }
        for pin in &guard_pins {
            assert!(GUARD.contains(pin.as_str()), "formation_guard.rs: {pin}");
        }
        for reason in COMPARISON_REASONS {
            assert!(GUARD.contains(&format!("\"{reason}\"")), "{reason}");
        }
        // The composed constants agree with those templates.
        assert_eq!(format!("{LEAD_A}{{}}{LEAD_B}"), "M03-INTEGRITY-v1 represented original-equation structural evidence for load case {}: StructuralReport { policy: \"M03-INTEGRITY-v1\", quality: ");
        assert_eq!(ROW_DEFECT.concat(), "net formation defect , self-equilibrated defect , bound , intended net at least , body scale , threshold  (self-equilibrated threshold ); formed sources ");
        assert_eq!(format!("{}{}", &MEMBER_RELATIVE_UNAVAILABLE[1..], ""), "the member-relative formation check is not available on this route; member rows are binary64 recoveries");
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
        assert_eq!(
            health(&source, Some(&dispatch.to_string()))["verdict_only"],
            true
        );
        let objects = json!([{"code": EVIDENCE_INCOMPLETE}, {"code": NOT_QUALIFIED}]);
        assert_eq!(
            health(&source, Some(&objects.to_string()))["verdict_only"],
            true
        );
        for findings in [
            None,
            Some("null"),
            Some("[]"),
            Some(r#"["NUMERICAL_INTEGRITY_NOT_QUALIFIED"]"#),
            Some(
                r#"["NUMERICAL_INTEGRITY_NOT_QUALIFIED","NUMERICAL_CASE_EVIDENCE_INCOMPLETE","MECHANICS_NOT_SOLVED"]"#,
            ),
            Some(r#"["NUMERICAL_INTEGRITY_NOT_QUALIFIED","NUMERICAL_INTEGRITY_NOT_QUALIFIED"]"#),
            Some(
                r#"["NUMERICAL_INTEGRITY_NOT_QUALIFIED","NUMERICAL_CASE_EVIDENCE_INCOMPLETE","NUMERICAL_AGGREGATE_CONTRADICTION"]"#,
            ),
        ] {
            assert_eq!(
                health(&source, findings)["verdict_only"],
                false,
                "{findings:?}"
            );
        }
        assert!(numerical_health_json(MULTICASE, Some("not json")).is_err());
        assert!(numerical_health_json("not json", None).is_err());
    }

    /// The multicase result with its first case made a passing companion.
    fn with_passing_companion() -> Value {
        let mut source = multicase();
        let first = source["numerical_quality"]["cases"][0]["basis_ref"]["ref_id"]
            .as_str()
            .unwrap()
            .to_string();
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
            for refs in [
                json!([]),
                json!(["diagnostic:numerical-integrity:renamed"]),
                Value::Null,
            ] {
                let mut source = companion.clone();
                source["numerical_quality"]["cases"][index]["evidence_refs"] = refs.clone();
                assert_eq!(
                    health(&source, Some(PAIR))["verdict_only"],
                    false,
                    "case {index} refs {refs}"
                );
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
        source["numerical_quality"]["cases"]
            .as_array_mut()
            .unwrap()
            .push(copy);
        assert_eq!(health(&source, Some(PAIR))["verdict_only"], false);
        let mut source = multicase();
        source["numerical_quality"]["cases"]
            .as_array_mut()
            .unwrap()
            .remove(1);
        assert_eq!(health(&source, Some(PAIR))["verdict_only"], false);
        let mut source = multicase();
        source["numerical_quality"]["status"] = json!("unresolved");
        assert_eq!(health(&source, Some(PAIR))["verdict_only"], false);
        for field in [
            "structural_status",
            "model_matrix_fidelity",
            "accuracy_evidence",
        ] {
            let mut source = multicase();
            source["numerical_quality"]["cases"][1][field] = json!("not_assessed_x");
            assert_eq!(
                health(&source, Some(PAIR))["verdict_only"],
                false,
                "{field}"
            );
        }
        let mut source = multicase();
        source["numerical_quality"]["cases"][1]["solve_quality"] = json!("checks_passed");
        assert_eq!(
            health(&source, Some(PAIR))["verdict_only"],
            false,
            "solve_quality against the code"
        );
    }
}
