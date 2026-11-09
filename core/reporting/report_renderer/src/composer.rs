//! The report section composer (RT-R4; DEL-08-01 Design
//! `report-section-composer.md` §2, slices C1 and C2).
//!
//! A composition selects and orders composable subsections of the governed
//! Results section. It never supplies a value: the renderer input keeps the
//! full bound row list, every boundary keeps its full-row equality check, and
//! the composer only chooses which of those rows each subsection shows and in
//! what order. The frame and the eight governed sections are emitted around
//! the composed subsections whatever the composition.
//!
//! - **Rows.** A row projected from a source row carries its bound `kind`,
//!   `value_text` and `unit` (and `entity_ref`, which may be absent). A row
//!   carrying none of them is a disclosure line, not a result value (the
//!   desktop's numerical-health lines): it is shown in full under every
//!   composition, in its own table, and is never selected, ranked or counted
//!   as a result row. A row carrying some but not all of them is refused.
//! - **Families.** `fixtures/reports/invented/report_composition_families.json`
//!   maps each row `kind` to one family section; a kind it does not list falls
//!   to `result_other_rows`, so the families partition the bound rows.
//! - **Views** are display orderings and selections of bound rows. Magnitude
//!   is the exact decimal value of `value_text` in the display unit (a value
//!   stored in Pa is shifted six places, exactly, and compared in MPa); rows
//!   are never ranked across display units; ties keep source order; `-0` is 0.
//! - **Refusals** (`REPORT-COMPOSITION-*`) block export; a refused
//!   composition is not applied, so the Results section shows the uncomposed
//!   rows under the blocked banner.
//! - **Disclosure.** A composed report states "Result rows shown: n of N.",
//!   states each subsection's cases and view, and lists every catalogue entry
//!   with the SHA-256 of the composition's RFC 8785 bytes in the Audit
//!   Manifest section.
//!
//! Contexts (slice C4). A composition is admitted under `local_private` and
//! under `user_released_report`; every other context refuses it. On the
//! user-released route the native derivation copies the closed composition
//! into the derived request (`report_package::user_release`), so whole-request
//! equality binds it, and [`form_refusals`] refuses a malformed one there.
//!
//! Standard claim fence applies (claims taxonomy per DEC-081).

use crate::{
    meta_paragraph, paragraph, subheading, table, RenderableReportInput, ReportExportContext,
    ResultRow, SectionBlock,
};
use serde::de::{self, Deserializer, SeqAccess, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::OnceLock;

/// The one composition version this renderer reads.
pub const COMPOSITION_VERSION: &str = "1.0.0";

pub const COMPOSITION_VERSION_UNSUPPORTED: &str = "REPORT-COMPOSITION-VERSION-UNSUPPORTED";
pub const COMPOSITION_SECTION_UNKNOWN: &str = "REPORT-COMPOSITION-SECTION-UNKNOWN";
pub const COMPOSITION_SECTION_DUPLICATE: &str = "REPORT-COMPOSITION-SECTION-DUPLICATE";
pub const COMPOSITION_OPTION_INVALID: &str = "REPORT-COMPOSITION-OPTION-INVALID";
pub const COMPOSITION_CASE_UNKNOWN: &str = "REPORT-COMPOSITION-CASE-UNKNOWN";
pub const COMPOSITION_ROWS_OVERLAP: &str = "REPORT-COMPOSITION-ROWS-OVERLAP";
/// A row with some but not all of its bound members, or a `value_text` that
/// is not a JSON number text.
pub const COMPOSITION_ROW_FIELDS_INVALID: &str = "REPORT-COMPOSITION-ROW-FIELDS-INVALID";
/// A composition on a report whose export context admits none (every context
/// other than `local_private` and `user_released_report`).
pub const COMPOSITION_CONTEXT_NOT_ADMITTED: &str = "REPORT-COMPOSITION-CONTEXT-NOT-ADMITTED";
/// The embedded family table does not parse or does not partition.
pub const COMPOSITION_FAMILIES_INVALID: &str = "REPORT-COMPOSITION-FAMILIES-INVALID";

pub const RESULT_ROWS_ALL: &str = "result_rows_all";
pub const RESULT_DISPLACEMENTS: &str = "result_displacements";
pub const RESULT_FORCES_MOMENTS: &str = "result_forces_moments";
pub const RESULT_STRESSES: &str = "result_stresses";
pub const RESULT_SUPPORT_LOADS: &str = "result_support_loads";
pub const RESULT_OTHER_ROWS: &str = "result_other_rows";

/// The family sections, which `result_rows_all` excludes (§2.3, overlap).
pub const FAMILY_SECTIONS: [&str; 5] = [
    RESULT_DISPLACEMENTS,
    RESULT_FORCES_MOMENTS,
    RESULT_STRESSES,
    RESULT_SUPPORT_LOADS,
    RESULT_OTHER_ROWS,
];

/// A view kind.
pub const VIEW_ALL: &str = "all";
pub const VIEW_EXTREMES: &str = "extremes";
pub const VIEW_LARGEST: &str = "largest";
/// The `largest` view's limit range.
pub const LARGEST_LIMIT_MIN: u64 = 1;
pub const LARGEST_LIMIT_MAX: u64 = 1000;

/// One composable section of the closed catalogue (§2.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CatalogueEntry {
    pub id: &'static str,
    pub title: &'static str,
    pub views: &'static [&'static str],
}

/// The closed catalogue of composable result sections, in catalogue order
/// (§2.2; slice C5's model-input sections join later).
pub const CATALOGUE: [CatalogueEntry; 6] = [
    CatalogueEntry {
        id: RESULT_ROWS_ALL,
        title: "All result rows",
        views: &[VIEW_ALL],
    },
    CatalogueEntry {
        id: RESULT_DISPLACEMENTS,
        title: "Displacements",
        views: &[VIEW_ALL, VIEW_EXTREMES],
    },
    CatalogueEntry {
        id: RESULT_FORCES_MOMENTS,
        title: "Forces and moments",
        views: &[VIEW_ALL],
    },
    CatalogueEntry {
        id: RESULT_STRESSES,
        title: "Stresses",
        views: &[VIEW_ALL, VIEW_LARGEST],
    },
    CatalogueEntry {
        id: RESULT_SUPPORT_LOADS,
        title: "Support loads",
        views: &[VIEW_ALL],
    },
    CatalogueEntry {
        id: RESULT_OTHER_ROWS,
        title: "Other result rows",
        views: &[VIEW_ALL],
    },
];

/// The catalogue entry of `id`, when it is one.
pub fn catalogue_entry(id: &str) -> Option<&'static CatalogueEntry> {
    CATALOGUE.iter().find(|entry| entry.id == id)
}

/// The user's report composition (§2.3). Closed: no other member is admitted
/// at any depth, and the shared object-form check refuses a positional form.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReportComposition {
    pub composition_version: String,
    pub sections: Vec<CompositionSection>,
}

/// One listed section: included, at its list position within its parent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompositionSection {
    pub section_id: String,
    pub cases: CaseSelection,
    pub view: CompositionView,
}

/// A section's view: `all`, `extremes` or `largest`, the last with a limit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompositionView {
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
}

/// A section's cases: the string `"all"`, or a list of `case_ref` values as
/// the rows carry them. Any other string does not decode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaseSelection {
    All,
    Cases(Vec<String>),
}

impl Serialize for CaseSelection {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            CaseSelection::All => serializer.serialize_str("all"),
            CaseSelection::Cases(cases) => cases.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for CaseSelection {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct CasesVisitor;
        impl<'de> Visitor<'de> for CasesVisitor {
            type Value = CaseSelection;
            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("the string \"all\" or a list of case references")
            }
            fn visit_str<E: de::Error>(self, value: &str) -> Result<CaseSelection, E> {
                if value == "all" {
                    Ok(CaseSelection::All)
                } else {
                    // The message names no input text (value-free decode).
                    Err(E::invalid_value(
                        de::Unexpected::Other("a string other than \"all\""),
                        &self,
                    ))
                }
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<CaseSelection, A::Error> {
                let mut cases = Vec::new();
                while let Some(case) = seq.next_element::<String>()? {
                    cases.push(case);
                }
                Ok(CaseSelection::Cases(cases))
            }
        }
        deserializer.deserialize_any(CasesVisitor)
    }
}

// ---------------------------------------------------------------------------
// Families

/// The committed family table, shared with the desktop.
pub const FAMILIES_JSON: &str =
    include_str!("../../../../fixtures/reports/invented/report_composition_families.json");

/// The parsed family table: kind to family section.
#[derive(Debug)]
pub struct FamilyTable {
    by_kind: BTreeMap<String, &'static str>,
}

impl FamilyTable {
    /// The section a row of `kind` belongs to; a kind the table does not list
    /// falls to `result_other_rows`.
    pub fn section_of(&self, kind: &str) -> &'static str {
        self.by_kind.get(kind).copied().unwrap_or(RESULT_OTHER_ROWS)
    }

    /// Every listed kind and its section.
    pub fn entries(&self) -> impl Iterator<Item = (&str, &'static str)> {
        self.by_kind
            .iter()
            .map(|(kind, section)| (kind.as_str(), *section))
    }
}

/// Parse a family table: `{"schema_version": "1.0.0", "sections": {<family
/// section id>: [<kind>, ...]}, ...}`. Only the four mapped family sections
/// may list kinds (`result_other_rows` takes every unlisted kind), and each
/// kind is listed once.
pub fn parse_family_table(text: &str) -> Result<FamilyTable, String> {
    let invalid = |detail: &str| format!("{COMPOSITION_FAMILIES_INVALID}: {detail}");
    let value: serde_json::Value =
        serde_json::from_str(text).map_err(|_| invalid("the table does not parse"))?;
    if value["schema_version"] != "1.0.0" {
        return Err(invalid("schema_version"));
    }
    let sections = value["sections"]
        .as_object()
        .ok_or_else(|| invalid("sections"))?;
    let mut by_kind = BTreeMap::new();
    for (section, kinds) in sections {
        let section = [
            RESULT_DISPLACEMENTS,
            RESULT_FORCES_MOMENTS,
            RESULT_STRESSES,
            RESULT_SUPPORT_LOADS,
        ]
        .into_iter()
        .find(|id| *id == section.as_str())
        .ok_or_else(|| invalid("a section that is not a mapped family"))?;
        for kind in kinds
            .as_array()
            .ok_or_else(|| invalid("a section's kinds"))?
        {
            let kind = kind
                .as_str()
                .filter(|kind| !kind.is_empty())
                .ok_or_else(|| invalid("a kind"))?;
            if by_kind.insert(kind.to_string(), section).is_some() {
                return Err(invalid("a kind listed twice"));
            }
        }
    }
    Ok(FamilyTable { by_kind })
}

/// The embedded family table.
pub fn family_table() -> Result<&'static FamilyTable, &'static str> {
    static TABLE: OnceLock<Result<FamilyTable, String>> = OnceLock::new();
    TABLE
        .get_or_init(|| parse_family_table(FAMILIES_JSON))
        .as_ref()
        .map_err(String::as_str)
}

// ---------------------------------------------------------------------------
// Exact magnitudes

/// The stored unit whose values are displayed (and compared) in MPa.
pub const PASCAL: &str = "Pa";
/// The display unit of a value stored in Pa.
pub const MEGAPASCAL: &str = "MPa";
const PASCAL_TO_MEGAPASCAL_SHIFT: i64 = 6;

/// The display unit of a value stored in `unit` (the report's display rule).
pub fn display_unit(unit: &str) -> &str {
    if unit == PASCAL {
        MEGAPASCAL
    } else {
        unit
    }
}

/// An exact decimal magnitude, `0.<digits> x 10^point`, with no leading or
/// trailing zero digit; zero has no digits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Magnitude {
    digits: Vec<u8>,
    point: i64,
}

impl Ord for Magnitude {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.digits.is_empty(), other.digits.is_empty()) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            // Normalized: the larger point is the larger magnitude; at equal
            // points the digit strings compare as decimals.
            (false, false) => self
                .point
                .cmp(&other.point)
                .then_with(|| self.digits.cmp(&other.digits)),
        }
    }
}

impl PartialOrd for Magnitude {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The exact magnitude of a JSON number text (`-?(0|[1-9][0-9]*)(.[0-9]+)?
/// ([eE][+-]?[0-9]+)?`), its decimal point moved `shift` places to the left;
/// `None` for any other text. No floating-point arithmetic.
pub fn exact_magnitude(text: &str, shift: i64) -> Option<Magnitude> {
    let unsigned = text.strip_prefix('-').unwrap_or(text);
    let (mantissa, exponent) = match unsigned.split_once(['e', 'E']) {
        Some((mantissa, exponent)) => {
            let digits = exponent.strip_prefix(['+', '-']).unwrap_or(exponent);
            if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
                return None;
            }
            (
                mantissa,
                exponent.trim_start_matches('+').parse::<i64>().ok()?,
            )
        }
        None => (unsigned, 0),
    };
    let (integer, fraction) = match mantissa.split_once('.') {
        Some((integer, fraction)) if !fraction.is_empty() => (integer, fraction),
        Some(_) => return None,
        None => (mantissa, ""),
    };
    if integer.is_empty()
        || (integer.len() > 1 && integer.starts_with('0'))
        || !integer
            .bytes()
            .chain(fraction.bytes())
            .all(|byte| byte.is_ascii_digit())
    {
        return None;
    }
    let all: Vec<u8> = integer
        .bytes()
        .chain(fraction.bytes())
        .map(|byte| byte - b'0')
        .collect();
    let leading = all.iter().take_while(|digit| **digit == 0).count();
    let mut digits = all[leading..].to_vec();
    while digits.last() == Some(&0) {
        digits.pop();
    }
    if digits.is_empty() {
        return Some(Magnitude { digits, point: 0 });
    }
    let point = (integer.len() as i64)
        .checked_add(exponent)?
        .checked_sub(leading as i64)?
        .checked_sub(shift)?;
    Some(Magnitude { digits, point })
}

// ---------------------------------------------------------------------------
// Rows

/// A bound row's composition members.
#[derive(Debug, Clone)]
struct BoundRow<'a> {
    index: usize,
    section: &'static str,
    case_ref: &'a str,
    label: &'a str,
    entity: Option<&'a str>,
    display_unit: &'a str,
    magnitude: Magnitude,
}

/// The rows split into bound rows and disclosure lines.
struct ClassifiedRows<'a> {
    bound: Vec<BoundRow<'a>>,
    disclosures: Vec<usize>,
}

fn classify<'a>(
    rows: &'a [ResultRow],
    families: &FamilyTable,
) -> Result<ClassifiedRows<'a>, Vec<String>> {
    let mut bound = Vec::new();
    let mut disclosures = Vec::new();
    let mut refusals = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        match (&row.kind, &row.value_text, &row.unit, &row.entity_ref) {
            (None, None, None, None) => disclosures.push(index),
            (Some(kind), Some(value_text), Some(unit), entity) => {
                let shift = if unit == PASCAL { PASCAL_TO_MEGAPASCAL_SHIFT } else { 0 };
                match exact_magnitude(value_text, shift) {
                    Some(magnitude) => bound.push(BoundRow {
                        index,
                        section: families.section_of(kind),
                        case_ref: &row.case_ref,
                        label: &row.label,
                        entity: entity.as_deref(),
                        display_unit: display_unit(unit),
                        magnitude,
                    }),
                    None => refusals.push(format!(
                        "{COMPOSITION_ROW_FIELDS_INVALID}: result_rows[{index}].value_text is not a number text"
                    )),
                }
            }
            _ => refusals.push(format!(
                "{COMPOSITION_ROW_FIELDS_INVALID}: result_rows[{index}] carries some but not all of kind, value_text and unit"
            )),
        }
    }
    if refusals.is_empty() {
        Ok(ClassifiedRows { bound, disclosures })
    } else {
        Err(refusals)
    }
}

// ---------------------------------------------------------------------------
// Composition

/// One composed subsection: its catalogue entry, its options as listed, and
/// the indexes into the input's `result_rows` it shows, in display order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposedSection {
    pub entry: &'static CatalogueEntry,
    pub cases: CaseSelection,
    pub view: CompositionView,
    pub rows: Vec<usize>,
}

/// An applied composition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Composed {
    /// The listed sections, in list order.
    pub sections: Vec<ComposedSection>,
    /// The disclosure lines (rows that are not bound result rows), in source
    /// order; every one is shown.
    pub disclosure_rows: Vec<usize>,
    /// The number of bound result rows (N of "n of N").
    pub bound_rows: usize,
}

impl Composed {
    /// The number of bound result rows shown (n of "n of N"). Each row is
    /// shown at most once (the overlap rule and the family partition).
    pub fn shown_rows(&self) -> usize {
        self.sections.iter().map(|section| section.rows.len()).sum()
    }
}

/// Check `composition` against the closed catalogue and the bound rows.
/// Every refusal is returned; an empty list admits it.
fn check(composition: &ReportComposition, bound: &[BoundRow<'_>]) -> Vec<String> {
    let carried: BTreeSet<&str> = bound
        .iter()
        .map(|row| row.case_ref)
        .filter(|case| *case != "TBD")
        .collect();
    check_in(composition, Some(&carried))
}

/// The refusals of a composition that need no result row: its version, each
/// id in the closed catalogue (a governed kind is not one), duplicates, view
/// options, case-list form and overlap. A case that no bound row carries is
/// refused only with the rows (by [`compose`], when the renderer applies it).
pub fn form_refusals(composition: &ReportComposition) -> Vec<String> {
    check_in(composition, None)
}

fn check_in(composition: &ReportComposition, carried: Option<&BTreeSet<&str>>) -> Vec<String> {
    let mut refusals = Vec::new();
    if composition.composition_version != COMPOSITION_VERSION {
        refusals.push(format!(
            "{COMPOSITION_VERSION_UNSUPPORTED}: composition_version is not {COMPOSITION_VERSION}"
        ));
        return refusals;
    }
    let mut listed: BTreeSet<&str> = BTreeSet::new();
    for (position, section) in composition.sections.iter().enumerate() {
        let at = format!("sections[{position}]");
        let Some(entry) = catalogue_entry(&section.section_id) else {
            refusals.push(format!(
                "{COMPOSITION_SECTION_UNKNOWN}: {at}.section_id is not a composable section"
            ));
            continue;
        };
        if !listed.insert(entry.id) {
            refusals.push(format!(
                "{COMPOSITION_SECTION_DUPLICATE}: {at} lists {} again",
                entry.id
            ));
        }
        let view = &section.view;
        if !entry.views.contains(&view.kind.as_str()) {
            refusals.push(format!(
                "{COMPOSITION_OPTION_INVALID}: {at}.view.kind is not a view {} takes",
                entry.id
            ));
        } else if view.kind == VIEW_LARGEST {
            if !view
                .limit
                .is_some_and(|limit| (LARGEST_LIMIT_MIN..=LARGEST_LIMIT_MAX).contains(&limit))
            {
                refusals.push(format!(
                    "{COMPOSITION_OPTION_INVALID}: {at}.view.limit is not in {LARGEST_LIMIT_MIN}-{LARGEST_LIMIT_MAX}"
                ));
            }
        } else if view.limit.is_some() {
            refusals.push(format!("{COMPOSITION_OPTION_INVALID}: {at}.view.limit applies only to the {VIEW_LARGEST} view"));
        }
        if let CaseSelection::Cases(cases) = &section.cases {
            if cases.is_empty() {
                refusals.push(format!(
                    "{COMPOSITION_OPTION_INVALID}: {at}.cases is an empty list"
                ));
            }
            let mut seen = BTreeSet::new();
            for case in cases {
                if !seen.insert(case.as_str()) {
                    refusals.push(format!(
                        "{COMPOSITION_OPTION_INVALID}: {at}.cases lists {case} twice"
                    ));
                } else if case == "TBD" {
                    refusals.push(format!("{COMPOSITION_OPTION_INVALID}: {at}.cases lists TBD, which is shown only under all"));
                } else if carried.is_some_and(|carried| !carried.contains(case.as_str())) {
                    refusals.push(format!("{COMPOSITION_CASE_UNKNOWN}: {at}.cases names {case}, which no result row carries"));
                }
            }
        }
    }
    if listed.contains(RESULT_ROWS_ALL) && FAMILY_SECTIONS.iter().any(|id| listed.contains(id)) {
        refusals.push(format!(
            "{COMPOSITION_ROWS_OVERLAP}: {RESULT_ROWS_ALL} is listed with a family section"
        ));
    }
    refusals
}

fn first_appearance<K: Ord + Clone>(keys: impl Iterator<Item = K>) -> BTreeMap<K, usize> {
    let mut order = BTreeMap::new();
    for key in keys {
        let next = order.len();
        order.entry(key).or_insert(next);
    }
    order
}

/// The rows one admitted section shows, in display order.
fn select(
    entry: &CatalogueEntry,
    cases: &CaseSelection,
    view: &CompositionView,
    bound: &[BoundRow<'_>],
) -> Vec<usize> {
    let selected: Vec<&BoundRow<'_>> = bound
        .iter()
        .filter(|row| entry.id == RESULT_ROWS_ALL || row.section == entry.id)
        .filter(|row| match cases {
            CaseSelection::All => true,
            CaseSelection::Cases(cases) => cases.iter().any(|case| case == row.case_ref),
        })
        .collect();
    match (entry.id, view.kind.as_str()) {
        (RESULT_DISPLACEMENTS, VIEW_EXTREMES) => {
            // Per case, result and display unit, the row of largest magnitude;
            // the first such row in source order on a tie. Groups in order of
            // first appearance.
            let mut groups: Vec<((&str, &str, &str), &BoundRow<'_>)> = Vec::new();
            let mut positions: BTreeMap<(&str, &str, &str), usize> = BTreeMap::new();
            for row in selected {
                let key = (row.case_ref, row.label, row.display_unit);
                if let Some(&position) = positions.get(&key) {
                    let best: &mut &BoundRow<'_> = &mut groups[position].1;
                    if row.magnitude > best.magnitude {
                        *best = row;
                    }
                } else {
                    positions.insert(key, groups.len());
                    groups.push((key, row));
                }
            }
            groups.into_iter().map(|(_, row)| row.index).collect()
        }
        (RESULT_STRESSES, VIEW_LARGEST) => {
            // Within each display unit (in order of first appearance), the
            // largest `limit` by magnitude; the stable sort keeps source order
            // on a tie. Never ranked across display units.
            let limit = view.limit.unwrap_or(0) as usize;
            let units = first_appearance(selected.iter().map(|row| row.display_unit));
            let mut by_unit: Vec<Vec<&BoundRow<'_>>> = vec![Vec::new(); units.len()];
            for row in selected {
                by_unit[units[row.display_unit]].push(row);
            }
            by_unit
                .into_iter()
                .flat_map(|mut rows| {
                    rows.sort_by(|left, right| right.magnitude.cmp(&left.magnitude));
                    rows.truncate(limit);
                    rows.into_iter().map(|row| row.index)
                })
                .collect()
        }
        (RESULT_SUPPORT_LOADS, _) => {
            // Grouped by support, then case, each in order of first
            // appearance; source order within a group (stable sort).
            let entities = first_appearance(selected.iter().map(|row| row.entity));
            let case_order = first_appearance(selected.iter().map(|row| row.case_ref));
            let mut rows = selected;
            rows.sort_by_key(|row| (entities[&row.entity], case_order[row.case_ref]));
            rows.into_iter().map(|row| row.index).collect()
        }
        _ => selected.into_iter().map(|row| row.index).collect(),
    }
}

/// Apply `composition` to `rows`: the refusals, or the composed sections.
pub fn compose(
    rows: &[ResultRow],
    composition: &ReportComposition,
) -> Result<Composed, Vec<String>> {
    let families = family_table().map_err(|detail| vec![detail.to_string()])?;
    let classified = classify(rows, families)?;
    let refusals = check(composition, &classified.bound);
    if !refusals.is_empty() {
        return Err(refusals);
    }
    let sections = composition
        .sections
        .iter()
        .map(|section| {
            let entry = catalogue_entry(&section.section_id).expect("checked");
            ComposedSection {
                entry,
                cases: section.cases.clone(),
                view: section.view.clone(),
                rows: select(entry, &section.cases, &section.view, &classified.bound),
            }
        })
        .collect();
    Ok(Composed {
        sections,
        disclosure_rows: classified.disclosures,
        bound_rows: classified.bound.len(),
    })
}

/// The input's composition refusals, each a blocking reason: none without a
/// composition.
pub fn composition_blocking_reasons(input: &RenderableReportInput) -> Vec<String> {
    match applied(input) {
        None | Some(Ok(_)) => Vec::new(),
        Some(Err(refusals)) => refusals,
    }
}

/// The input's admitted composition, applied; `None` without one or when it
/// is refused (the refusal blocks export, and the rows are shown uncomposed).
pub fn admitted_composition(
    input: &RenderableReportInput,
) -> Option<(&ReportComposition, Composed)> {
    let composed = applied(input)?.ok()?;
    input
        .report_composition
        .as_ref()
        .map(|composition| (composition, composed))
}

/// `None` without a composition; otherwise the composed sections or every
/// refusal.
fn applied(input: &RenderableReportInput) -> Option<Result<Composed, Vec<String>>> {
    let composition = input.report_composition.as_ref()?;
    let context = input
        .report_context
        .as_ref()
        .map(|context| context.export_context);
    if !matches!(
        context,
        Some(ReportExportContext::LocalPrivate | ReportExportContext::UserReleasedReport)
    ) {
        return Some(Err(vec![format!(
            "{COMPOSITION_CONTEXT_NOT_ADMITTED}: composition requires a local private or user-released report"
        )]));
    }
    Some(compose(&input.result_rows, composition))
}

// ---------------------------------------------------------------------------
// Canonical bytes and digest

fn json_string(text: &str) -> String {
    serde_json::to_string(text).expect("a string serializes")
}

/// The composition's RFC 8785 (JCS) bytes. Its members are fixed ASCII keys
/// written here in code-unit order, strings use the JSON escapes JCS and
/// `serde_json` share (the short escapes and `\u00xx` for other controls,
/// nothing else escaped), and the one number is an integer.
pub fn composition_jcs(composition: &ReportComposition) -> String {
    let sections: Vec<String> = composition
        .sections
        .iter()
        .map(|section| {
            let cases = match &section.cases {
                CaseSelection::All => json_string("all"),
                CaseSelection::Cases(cases) => format!(
                    "[{}]",
                    cases
                        .iter()
                        .map(|case| json_string(case))
                        .collect::<Vec<_>>()
                        .join(",")
                ),
            };
            let view = match section.view.limit {
                Some(limit) => format!(
                    "{{\"kind\":{},\"limit\":{limit}}}",
                    json_string(&section.view.kind)
                ),
                None => format!("{{\"kind\":{}}}", json_string(&section.view.kind)),
            };
            format!(
                "{{\"cases\":{cases},\"section_id\":{},\"view\":{view}}}",
                json_string(&section.section_id)
            )
        })
        .collect();
    format!(
        "{{\"composition_version\":{},\"sections\":[{}]}}",
        json_string(&composition.composition_version),
        sections.join(",")
    )
}

/// The SHA-256 of [`composition_jcs`], lowercase hex.
pub fn composition_sha256(composition: &ReportComposition) -> String {
    Sha256::digest(composition_jcs(composition).as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

// ---------------------------------------------------------------------------
// Assembled blocks

/// The coverage line (§2.6).
pub fn coverage_text(composed: &Composed) -> String {
    format!(
        "Result rows shown: {} of {}.",
        composed.shown_rows(),
        composed.bound_rows
    )
}

/// A family section's statement when it shows no row.
pub const NO_ROWS_OF_THIS_KIND: &str = "No result rows of this kind for the selected cases.";
/// The disclosure-lines subheading and statement.
pub const DISCLOSURE_LINES_SUBHEADING: &str = "Result Disclosure Lines";
pub const DISCLOSURE_LINES_TEXT: &str =
    "These lines are disclosures, not result values. Every one is shown whatever the composition, and none is counted, selected or ranked as a result row.";
/// The statement of how views compare magnitudes.
pub const MAGNITUDE_BASIS_TEXT: &str = "Magnitudes are compared exactly from the stored values in the display unit (a value stored in Pa is compared in MPa), before display rounding, never across display units; ties keep source order.";
/// The Audit Manifest subheading.
pub const COMPOSITION_SUBHEADING: &str = "Report Composition";

/// The cases text of a section.
pub fn cases_text(cases: &CaseSelection) -> String {
    match cases {
        CaseSelection::All => "all cases".to_string(),
        CaseSelection::Cases(cases) => cases.join(", "),
    }
}

/// The view text of a section.
pub fn view_text(section_id: &str, view: &CompositionView) -> String {
    match (section_id, view.kind.as_str(), view.limit) {
        (_, VIEW_EXTREMES, _) => {
            "extremes: per case, result and display unit, the row of largest magnitude".to_string()
        }
        (_, VIEW_LARGEST, Some(limit)) => {
            format!("largest {limit} by magnitude within each display unit")
        }
        (RESULT_SUPPORT_LOADS, _, _) => "all rows, grouped by support, then case".to_string(),
        _ => "all rows, in source order".to_string(),
    }
}

const NOT_RECORDED: &str = "not recorded";

/// The composed part of the Results section: the coverage line, each listed
/// subsection in list order, and the disclosure lines.
pub(crate) fn results_blocks(rows: &[ResultRow], composed: &Composed) -> Vec<SectionBlock> {
    let mut blocks = vec![paragraph(&coverage_text(composed))];
    for section in &composed.sections {
        blocks.push(subheading(section.entry.title));
        let mut statement = format!(
            "Cases: {}. View: {}. Result rows shown in this subsection: {}.",
            cases_text(&section.cases),
            view_text(section.entry.id, &section.view),
            section.rows.len()
        );
        if section.view.kind != VIEW_ALL {
            statement.push(' ');
            statement.push_str(MAGNITUDE_BASIS_TEXT);
        }
        blocks.push(meta_paragraph(&statement));
        if section.rows.is_empty() {
            blocks.push(paragraph(NO_ROWS_OF_THIS_KIND));
        } else if section.entry.id == RESULT_ROWS_ALL {
            let cells = section
                .rows
                .iter()
                .map(|index| {
                    let row = &rows[*index];
                    vec![
                        row.label.clone(),
                        row.case_ref.clone(),
                        row.quantity_display.clone(),
                        row.source_ref.clone(),
                    ]
                })
                .collect();
            blocks.push(table(&["Result", "Case", "Value", "Source"], cells));
        } else {
            let cells = section
                .rows
                .iter()
                .map(|index| {
                    let row = &rows[*index];
                    vec![
                        row.entity_ref
                            .clone()
                            .unwrap_or_else(|| NOT_RECORDED.to_string()),
                        row.label.clone(),
                        row.case_ref.clone(),
                        row.quantity_display.clone(),
                        row.source_ref.clone(),
                    ]
                })
                .collect();
            blocks.push(table(
                &["Entity", "Result", "Case", "Value", "Source"],
                cells,
            ));
        }
    }
    if !composed.disclosure_rows.is_empty() {
        blocks.push(subheading(DISCLOSURE_LINES_SUBHEADING));
        blocks.push(meta_paragraph(DISCLOSURE_LINES_TEXT));
        let cells = composed
            .disclosure_rows
            .iter()
            .map(|index| {
                let row = &rows[*index];
                vec![
                    row.label.clone(),
                    row.case_ref.clone(),
                    row.quantity_display.clone(),
                    row.source_ref.clone(),
                ]
            })
            .collect();
        blocks.push(table(&["Disclosure", "Case", "Value", "Source"], cells));
    }
    blocks
}

/// The Audit Manifest's Report Composition table (§2.6): every catalogue
/// entry, included (with its position, cases and view) or omitted, and the
/// composition's SHA-256.
pub(crate) fn audit_blocks(
    composition: &ReportComposition,
    composed: &Composed,
) -> Vec<SectionBlock> {
    let rows = CATALOGUE
        .iter()
        .map(|entry| {
            match composed
                .sections
                .iter()
                .position(|section| section.entry.id == entry.id)
            {
                Some(position) => {
                    let section = &composed.sections[position];
                    vec![
                        entry.id.to_string(),
                        format!("included, position {}", position + 1),
                        cases_text(&section.cases),
                        view_text(entry.id, &section.view),
                    ]
                }
                None => vec![
                    entry.id.to_string(),
                    "omitted".to_string(),
                    String::new(),
                    String::new(),
                ],
            }
        })
        .collect();
    vec![
        subheading(COMPOSITION_SUBHEADING),
        table(&["Section", "Inclusion", "Cases", "View"], rows),
        meta_paragraph(&format!(
            "Composition version {}; SHA-256 of its RFC 8785 canonical JSON bytes: {}. Every result row stays listed with its checksum under Referenced result export, whether or not it is shown.",
            composition.composition_version,
            composition_sha256(composition)
        )),
    ]
}
