//! Rule-pack format 0.5.0 (W-17; PRD §17.2, FR-RULE-001; DEL-06-01 Design
//! `rule-pack-0.5.0.md`): the one current format version, stress categories,
//! required load cases, load-case mappings and the solver-row selector.
//!
//! This module holds what both the validator (this crate) and the run path
//! (`open_pipe_stress_rule_check_runner`, the desktop run command) read from a
//! pack: the version constant, the finding codes, the basis reference, the
//! per-check basis plan and the selector. The validator's findings and the
//! runner's plan are computed by the same functions, so a pack the validator
//! accepts is planned exactly as validated.
//!
//! Categories, required load cases and mappings are the user's own content.
//! The format gives a category no built-in meaning and no fixed list (design
//! §2.3); nothing here names a category of any standard.

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ValidationFinding;

/// The one current rule-pack format (design §2.1). Top-level `schema_version`
/// and `metadata.schema_version` must both equal it; any other value is
/// refused by [`RULE_PACK_SCHEMA_VERSION_UNSUPPORTED`], with no migration.
pub const RULE_PACK_SCHEMA_VERSION: &str = "0.5.0";

// --- Static (validation) finding codes, design §2.1, §2.5 and §2.9 ---------

/// A pack declaring any version other than [`RULE_PACK_SCHEMA_VERSION`]
/// (absent, `0.4.0`, or a top-level value differing from
/// `metadata.schema_version`). Blocking; no check of the pack runs.
pub const RULE_PACK_SCHEMA_VERSION_UNSUPPORTED: &str = "RULE_PACK_SCHEMA_VERSION_UNSUPPORTED";
/// A mapping's category or required load case, or a check's category, that
/// names nothing declared (design §2.9).
pub const RULE_PACK_REFERENCE_UNRESOLVED: &str = "RULE_PACK_REFERENCE_UNRESOLVED";
/// A duplicate id, a duplicate `basis_ref`, or the same required load case
/// twice in one mapping (design §2.9).
pub const RULE_PACK_DUPLICATE_ID: &str = "RULE_PACK_DUPLICATE_ID";
/// A category with no mapping or more than one, a required load case in no
/// mapping, or a required load case no check reaches (design §2.9).
pub const RULE_PACK_MAPPING_INCOMPLETE: &str = "RULE_PACK_MAPPING_INCOMPLETE";
/// A `solver_result` input without a usable `solver_result_ref` selector
/// (design §2.5).
pub const RULE_SOLVER_RESULT_REF_MISSING: &str = "RULE_SOLVER_RESULT_REF_MISSING";

// --- Run-time finding codes, design §2.5 and §2.6 ---------------------------

/// A required load case absent from the model's `load_cases` or
/// `combinations` (design §2.6).
pub const RULE_REQUIRED_LOAD_CASE_MISSING: &str = "RULE_REQUIRED_LOAD_CASE_MISSING";
/// A required load case present in the model with no rows in the solved
/// envelope, for example a withheld combination (design §2.6).
pub const RULE_REQUIRED_LOAD_CASE_UNSOLVED: &str = "RULE_REQUIRED_LOAD_CASE_UNSOLVED";
/// No envelope row matches an input's selector on a basis (design §2.5).
pub const RULE_SOLVER_RESULT_ROW_MISSING: &str = "RULE_SOLVER_RESULT_ROW_MISSING";
/// More than one envelope row matches an input's selector on a basis
/// (design §2.5). The input is unsupplied; never a pass.
pub const RULE_SOLVER_RESULT_ROW_AMBIGUOUS: &str = "RULE_SOLVER_RESULT_ROW_AMBIGUOUS";
/// Caller-supplied `{input_id, result_id}` solver selectors, retired for a
/// 0.5.0 pack: one binding route remains, the pack's own selector (§2.5).
pub const RULE_CALLER_SOLVER_BINDING_RETIRED: &str = "RULE_CALLER_SOLVER_BINDING_RETIRED";

/// The `basis_ref.ref_type` values a required load case may name (§2.3).
pub const BASIS_REF_TYPES: [&str; 2] = ["load_case", "combination"];
/// The section-basis tokens a selector may name (§2.5): the values of the
/// `section_basis=` token of a row's `metadata.basis`.
pub const SECTION_BASES: [&str; 2] = ["nominal_section", "reduced_section"];
/// `Reference.ref_type` of a check's or mapping's `category_ref`.
pub const CATEGORY_REF_TYPE: &str = "stress_category";
/// `Reference.ref_type` of a mapping's `required_load_case_refs` items.
pub const REQUIRED_LOAD_CASE_REF_TYPE: &str = "required_load_case";

const SELECTOR_REQUIRED: [&str; 2] = ["result_kind", "entity_ref"];
const SELECTOR_OPTIONAL: [&str; 4] = ["location", "component", "section_basis", "tie_rod"];

/// A load case or combination of the model, as a required load case names it
/// and as an envelope row's `basis_ref` carries it (design §2.3).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BasisRef {
    /// `load_case` or `combination`.
    pub ref_type: String,
    /// The model's id for that load case or combination.
    pub ref_id: String,
}

impl BasisRef {
    pub fn new(ref_type: impl Into<String>, ref_id: impl Into<String>) -> Self {
        Self {
            ref_type: ref_type.into(),
            ref_id: ref_id.into(),
        }
    }

    /// A well-formed basis reference: an object whose `ref_type` is one of
    /// [`BASIS_REF_TYPES`] and whose `ref_id` is a non-empty string.
    pub fn from_value(value: &Value) -> Option<Self> {
        let ref_type = value.get("ref_type").and_then(Value::as_str)?;
        let ref_id = value.get("ref_id").and_then(Value::as_str)?;
        (BASIS_REF_TYPES.contains(&ref_type) && !ref_id.trim().is_empty())
            .then(|| Self::new(ref_type, ref_id))
    }

    /// Whether an envelope row's `basis_ref` names this basis. A row without
    /// `basis_ref` names no basis and is never selected (§2.5).
    pub fn names_row(&self, row: &Value) -> bool {
        row.get("basis_ref")
            .and_then(Self::from_value)
            .is_some_and(|basis| basis == *self)
    }
}

/// The selector a 0.5.0 `solver_result_ref` carries (design §2.5). An absent
/// optional member matches only a row without that member. It serializes as
/// the pack member (absent members omitted).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct SolverResultSelector {
    pub result_kind: String,
    pub entity_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    /// `nominal_section` or `reduced_section` ([`SECTION_BASES`]).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_basis: Option<String>,
    /// One rod id of a connector's `tie_rods` record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tie_rod: Option<String>,
}

/// Read a `solver_result_ref` as a selector. `Err` names why it is unusable:
/// not an object, a required member absent or empty, an optional member not
/// a non-empty string, a section basis outside [`SECTION_BASES`], or any other
/// member (the 0.4.0 `result_id` among them).
pub fn parse_solver_result_ref(value: &Value) -> Result<SolverResultSelector, String> {
    let map = value
        .as_object()
        .ok_or_else(|| "solver_result_ref must be an object".to_string())?;
    if let Some(other) = map.keys().find(|key| {
        !SELECTOR_REQUIRED.contains(&key.as_str()) && !SELECTOR_OPTIONAL.contains(&key.as_str())
    }) {
        return Err(format!(
            "solver_result_ref carries the unsupported member '{other}'; a 0.5.0 selector has \
             result_kind, entity_ref and optionally location, component, section_basis, tie_rod"
        ));
    }
    let required = |key: &str| {
        map.get(key)
            .and_then(Value::as_str)
            .filter(|text| !text.is_empty())
            .map(str::to_string)
            .ok_or_else(|| format!("solver_result_ref requires a non-empty string '{key}'"))
    };
    let optional = |key: &str| match map.get(key) {
        None => Ok(None),
        Some(Value::String(text)) if !text.is_empty() => Ok(Some(text.clone())),
        Some(_) => Err(format!(
            "solver_result_ref member '{key}' must be a non-empty string when present"
        )),
    };
    let selector = SolverResultSelector {
        result_kind: required("result_kind")?,
        entity_ref: required("entity_ref")?,
        location: optional("location")?,
        component: optional("component")?,
        section_basis: optional("section_basis")?,
        tie_rod: optional("tie_rod")?,
    };
    if let Some(basis) = &selector.section_basis {
        if !SECTION_BASES.contains(&basis.as_str()) {
            return Err(format!(
                "solver_result_ref section_basis '{basis}' is not one of {SECTION_BASES:?}"
            ));
        }
    }
    Ok(selector)
}

/// One basis of a check: a required load case mapped to the check's category,
/// in the mapping's order (design §2.4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedBasis {
    pub required_load_case_id: String,
    pub basis_ref: BasisRef,
}

/// A check's category and bases. `bases` is `Err(message)` when the pack's
/// structure does not give the check a basis list (the validator reports the
/// same defect as a blocking finding, so a validated pack never yields one).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckBasisPlan {
    pub check_id: String,
    pub category_ref: String,
    pub bases: Result<Vec<PlannedBasis>, String>,
}

fn ref_id(value: Option<&Value>) -> Option<&str> {
    value
        .and_then(|reference| reference.get("ref_id"))
        .and_then(Value::as_str)
        .filter(|id| !id.is_empty())
}

fn ref_id_of_type<'a>(value: Option<&'a Value>, ref_type: &str) -> Option<&'a str> {
    value
        .filter(|reference| reference.get("ref_type").and_then(Value::as_str) == Some(ref_type))
        .and_then(|reference| ref_id(Some(reference)))
}

fn items<'a>(document: &'a Value, key: &str) -> &'a [Value] {
    document
        .get(key)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[])
}

fn item_id<'a>(item: &'a Value, key: &str) -> Option<&'a str> {
    item.get(key)
        .and_then(Value::as_str)
        .filter(|id| !id.is_empty())
}

/// The basis plan of every check, in `check_definitions` order (design §2.4,
/// §2.7): the check's `category_ref`, the category's one mapping, and that
/// mapping's required load cases in order, each with its `basis_ref`.
pub fn plan_check_bases(document: &Value) -> Vec<CheckBasisPlan> {
    let mut load_cases: HashMap<&str, Vec<&Value>> = HashMap::new();
    for item in items(document, "required_load_cases") {
        if let Some(id) = item_id(item, "required_load_case_id") {
            load_cases.entry(id).or_default().push(item);
        }
    }
    let mut mappings: HashMap<&str, Vec<&Value>> = HashMap::new();
    for mapping in items(document, "load_case_mappings") {
        if let Some(category) = ref_id_of_type(mapping.get("category_ref"), CATEGORY_REF_TYPE) {
            mappings.entry(category).or_default().push(mapping);
        }
    }
    items(document, "check_definitions")
        .iter()
        .map(|check| {
            let check_id = item_id(check, "check_id").unwrap_or_default().to_string();
            let category = ref_id_of_type(check.get("category_ref"), CATEGORY_REF_TYPE);
            let bases = match category {
                None => Err("the check names no stress category".to_string()),
                Some(category) => plan_category(category, &mappings, &load_cases),
            };
            CheckBasisPlan {
                check_id,
                category_ref: category.unwrap_or_default().to_string(),
                bases,
            }
        })
        .collect()
}

fn plan_category(
    category: &str,
    mappings: &HashMap<&str, Vec<&Value>>,
    load_cases: &HashMap<&str, Vec<&Value>>,
) -> Result<Vec<PlannedBasis>, String> {
    let mapping = match mappings.get(category).map(Vec::as_slice) {
        Some([mapping]) => *mapping,
        Some([]) | None => return Err(format!("category '{category}' has no load-case mapping")),
        Some(_) => {
            return Err(format!(
                "category '{category}' has more than one load-case mapping"
            ))
        }
    };
    let refs = items(mapping, "required_load_case_refs");
    if refs.is_empty() {
        return Err(format!(
            "the mapping of category '{category}' names no required load case"
        ));
    }
    let mut bases: Vec<PlannedBasis> = Vec::with_capacity(refs.len());
    for reference in refs {
        let id = ref_id_of_type(Some(reference), REQUIRED_LOAD_CASE_REF_TYPE).ok_or_else(|| {
            format!("the mapping of category '{category}' carries a malformed required load case reference")
        })?;
        let declared = match load_cases.get(id).map(Vec::as_slice) {
            Some([declared]) => *declared,
            Some([]) | None => return Err(format!("required load case '{id}' is not declared")),
            Some(_) => {
                return Err(format!(
                    "required load case '{id}' is declared more than once"
                ))
            }
        };
        let basis_ref = declared
            .get("basis_ref")
            .and_then(BasisRef::from_value)
            .ok_or_else(|| format!("required load case '{id}' carries no well-formed basis_ref"))?;
        if bases.iter().any(|basis| basis.required_load_case_id == id) {
            return Err(format!(
                "the mapping of category '{category}' names required load case '{id}' twice"
            ));
        }
        bases.push(PlannedBasis {
            required_load_case_id: id.to_string(),
            basis_ref,
        });
    }
    Ok(bases)
}

fn finding(
    code: &str,
    subject: impl Into<String>,
    message: impl Into<String>,
) -> ValidationFinding {
    ValidationFinding {
        code: code.to_string(),
        subject: subject.into(),
        message: message.into(),
        blocking: true,
    }
}

/// The version finding (design §2.1): `None` when both the top-level and the
/// metadata `schema_version` equal [`RULE_PACK_SCHEMA_VERSION`].
pub fn schema_version_finding(document: &Value) -> Option<ValidationFinding> {
    let top = document.get("schema_version").and_then(Value::as_str);
    let metadata = document
        .pointer("/metadata/schema_version")
        .and_then(Value::as_str);
    if top == Some(RULE_PACK_SCHEMA_VERSION) && metadata == Some(RULE_PACK_SCHEMA_VERSION) {
        return None;
    }
    let shown =
        |value: Option<&str>| value.map_or_else(|| "absent".to_string(), |v| format!("'{v}'"));
    Some(finding(
        RULE_PACK_SCHEMA_VERSION_UNSUPPORTED,
        "schema_version",
        format!(
            "rule-pack format {} (metadata.schema_version {}) is not supported; the supported \
             format is '{RULE_PACK_SCHEMA_VERSION}', and both members must declare it. There is \
             no migration: re-author the pack in format {RULE_PACK_SCHEMA_VERSION}",
            shown(top),
            shown(metadata)
        ),
    ))
}

/// The static findings of design §2.5 and §2.9, all blocking, in document
/// order. Whether a named load case exists in a model is a run-time question
/// (§2.6), never checked here.
pub fn format_findings(document: &Value) -> Vec<ValidationFinding> {
    let mut findings = Vec::new();

    // Each new member is required with at least one item (§2.3).
    for key in [
        "required_load_cases",
        "stress_categories",
        "load_case_mappings",
    ] {
        if items(document, key).is_empty() {
            findings.push(finding(
                RULE_PACK_MAPPING_INCOMPLETE,
                key,
                format!("the pack must declare {key} with at least one item"),
            ));
        }
    }

    // Required load cases: unique ids, well-formed and unique basis_ref.
    let mut load_case_ids: HashSet<&str> = HashSet::new();
    let mut basis_refs: HashSet<BasisRef> = HashSet::new();
    for (index, item) in items(document, "required_load_cases").iter().enumerate() {
        let subject = format!("required_load_cases[{index}]");
        match item_id(item, "required_load_case_id") {
            None => findings.push(finding(
                RULE_PACK_REFERENCE_UNRESOLVED,
                subject.clone(),
                "the required load case carries no required_load_case_id",
            )),
            Some(id) if !load_case_ids.insert(id) => findings.push(finding(
                RULE_PACK_DUPLICATE_ID,
                id,
                format!("required load case id '{id}' is declared more than once"),
            )),
            Some(_) => {}
        }
        match item.get("basis_ref").and_then(BasisRef::from_value) {
            None => findings.push(finding(
                RULE_PACK_REFERENCE_UNRESOLVED,
                format!("{subject}.basis_ref"),
                "basis_ref must name a model load_case or combination by a non-empty ref_id",
            )),
            Some(basis) if !basis_refs.insert(basis.clone()) => findings.push(finding(
                RULE_PACK_DUPLICATE_ID,
                format!("{subject}.basis_ref"),
                format!(
                    "two required load cases name the same {} '{}'",
                    basis.ref_type, basis.ref_id
                ),
            )),
            Some(_) => {}
        }
    }

    // Stress categories: unique ids.
    let mut category_ids: HashSet<&str> = HashSet::new();
    for (index, item) in items(document, "stress_categories").iter().enumerate() {
        match item_id(item, "category_id") {
            None => findings.push(finding(
                RULE_PACK_REFERENCE_UNRESOLVED,
                format!("stress_categories[{index}]"),
                "the stress category carries no category_id",
            )),
            Some(id) if !category_ids.insert(id) => findings.push(finding(
                RULE_PACK_DUPLICATE_ID,
                id,
                format!("stress category id '{id}' is declared more than once"),
            )),
            Some(_) => {}
        }
    }

    // Mappings: unique ids, resolved references, no load case twice, and one
    // mapping per category.
    let mut mapping_ids: HashSet<&str> = HashSet::new();
    let mut mappings_per_category: HashMap<&str, usize> = HashMap::new();
    let mut mapped_load_cases: HashSet<&str> = HashSet::new();
    for (index, mapping) in items(document, "load_case_mappings").iter().enumerate() {
        let subject = format!("load_case_mappings[{index}]");
        match item_id(mapping, "mapping_id") {
            None => findings.push(finding(
                RULE_PACK_REFERENCE_UNRESOLVED,
                subject.clone(),
                "the load-case mapping carries no mapping_id",
            )),
            Some(id) if !mapping_ids.insert(id) => findings.push(finding(
                RULE_PACK_DUPLICATE_ID,
                id,
                format!("load-case mapping id '{id}' is declared more than once"),
            )),
            Some(_) => {}
        }
        match ref_id_of_type(mapping.get("category_ref"), CATEGORY_REF_TYPE) {
            Some(category) if category_ids.contains(category) => {
                *mappings_per_category.entry(category).or_default() += 1;
            }
            other => findings.push(finding(
                RULE_PACK_REFERENCE_UNRESOLVED,
                format!("{subject}.category_ref"),
                format!(
                    "the mapping's category_ref {} names no declared stress category",
                    other.map_or_else(
                        || "(absent or malformed)".to_string(),
                        |id| format!("'{id}'")
                    )
                ),
            )),
        }
        let refs = items(mapping, "required_load_case_refs");
        if refs.is_empty() {
            findings.push(finding(
                RULE_PACK_MAPPING_INCOMPLETE,
                format!("{subject}.required_load_case_refs"),
                "a mapping must name at least one required load case",
            ));
        }
        let mut within: HashSet<&str> = HashSet::new();
        for (ref_index, reference) in refs.iter().enumerate() {
            let ref_subject = format!("{subject}.required_load_case_refs[{ref_index}]");
            match ref_id_of_type(Some(reference), REQUIRED_LOAD_CASE_REF_TYPE) {
                Some(id) if load_case_ids.contains(id) => {
                    mapped_load_cases.insert(id);
                    if !within.insert(id) {
                        findings.push(finding(
                            RULE_PACK_DUPLICATE_ID,
                            ref_subject,
                            format!("the mapping names required load case '{id}' more than once"),
                        ));
                    }
                }
                other => findings.push(finding(
                    RULE_PACK_REFERENCE_UNRESOLVED,
                    ref_subject,
                    format!(
                        "{} names no declared required load case",
                        other.map_or_else(
                            || "the reference (absent or malformed)".to_string(),
                            |id| format!("'{id}'")
                        )
                    ),
                )),
            }
        }
    }
    for item in items(document, "stress_categories") {
        if let Some(id) = item_id(item, "category_id") {
            match mappings_per_category.get(id).copied().unwrap_or(0) {
                1 => {}
                0 => findings.push(finding(
                    RULE_PACK_MAPPING_INCOMPLETE,
                    id,
                    format!("stress category '{id}' has no load-case mapping"),
                )),
                n => findings.push(finding(
                    RULE_PACK_MAPPING_INCOMPLETE,
                    id,
                    format!("stress category '{id}' has {n} load-case mappings; exactly one is required"),
                )),
            }
        }
    }
    for item in items(document, "required_load_cases") {
        if let Some(id) = item_id(item, "required_load_case_id") {
            if !mapped_load_cases.contains(id) {
                findings.push(finding(
                    RULE_PACK_MAPPING_INCOMPLETE,
                    id,
                    format!("required load case '{id}' appears in no load-case mapping"),
                ));
            }
        }
    }

    // Checks name a declared category (§2.4).
    for (index, check) in items(document, "check_definitions").iter().enumerate() {
        match ref_id_of_type(check.get("category_ref"), CATEGORY_REF_TYPE) {
            Some(category) if category_ids.contains(category) => {}
            other => findings.push(finding(
                RULE_PACK_REFERENCE_UNRESOLVED,
                format!(
                    "check_definitions[{index}].category_ref ({})",
                    item_id(check, "check_id").unwrap_or("unidentified check")
                ),
                format!(
                    "the check's category_ref {} names no declared stress category",
                    other.map_or_else(
                        || "(absent or malformed)".to_string(),
                        |id| format!("'{id}'")
                    )
                ),
            )),
        }
    }

    // Reachability (§2.9, review round 2 finding 5): every required load case
    // is a basis of at least one check through that check's category mapping,
    // so a missing load case always produces an outcome the aggregate counts.
    let mut reached: HashSet<String> = HashSet::new();
    for plan in plan_check_bases(document) {
        if let Ok(bases) = plan.bases {
            reached.extend(bases.into_iter().map(|basis| basis.required_load_case_id));
        }
    }
    for item in items(document, "required_load_cases") {
        if let Some(id) = item_id(item, "required_load_case_id") {
            if mapped_load_cases.contains(id) && !reached.contains(id) {
                findings.push(finding(
                    RULE_PACK_MAPPING_INCOMPLETE,
                    id,
                    format!(
                        "required load case '{id}' is reached by no check: no check's category \
                         maps it, so it would produce no outcome"
                    ),
                ));
            }
        }
    }

    // Solver inputs carry a usable selector (§2.5).
    for (index, input) in items(document, "required_inputs").iter().enumerate() {
        if input.get("source_kind").and_then(Value::as_str) != Some("solver_result") {
            continue;
        }
        let input_id = item_id(input, "input_id").unwrap_or("unidentified input");
        let problem = match input.get("solver_result_ref") {
            None => {
                Some("the solver_result input carries no solver_result_ref selector".to_string())
            }
            Some(reference) => parse_solver_result_ref(reference).err(),
        };
        if let Some(problem) = problem {
            findings.push(finding(
                RULE_SOLVER_RESULT_REF_MISSING,
                format!("required_inputs[{index}] ({input_id})"),
                problem,
            ));
        }
    }

    findings
}
