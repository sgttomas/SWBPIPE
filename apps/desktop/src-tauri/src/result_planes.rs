//! C1 slice 2: S's planes and its binary layout (DEL-04-05
//! `c1-native-custody-plan.md`, slice 2).
//!
//! **Layout.** One response: the magic `SWBSUMv1`, the header's length as a
//! little-endian u64, the header (S's JSON, UTF-8), zero padding to a multiple
//! of 8 bytes, then the planes: little-endian f64 arrays, each at the offset
//! and with the element count its descriptor in the header states
//! (`planes: [{name, offset, count}]`, offsets from the planes' start;
//! `planes_bytes` their total). The length is exact: nothing follows the
//! last plane. [`decode`] refuses anything else by name.
//!
//! **Planes.** Columns of the rows S's consumers read value by value, so S's
//! JSON part does not grow with them:
//! - `display.*`: every row of a viewport kind (nodal translations,
//!   rotations and displacement magnitudes per set; support reactions; the
//!   member and station quantities the colour map uses) and the rows the
//!   intensified-bending rows name as their sources. Per row: its ordinal in
//!   the envelope's `results` (the exact row a pick opens), its value (NaN
//!   when it is not a finite number), and the members the overlays read as
//!   codes into S's value table `values` (`-1` absent).
//!   `display.source_offsets` (rows + 1) and `display.source_refs` give each
//!   row's `source_result_refs` resolved to ordinals (`-1`: no such row).
//! - `stress.*`: the admitted emitted stress rows of the stress and ratio
//!   table's groups, each group a contiguous run (`stress_ratio.groups[].rows`).
//!
//! Codes and ordinals are integers below 2^53, exact in f64. Values are the
//! envelope's own numbers, bit for bit (serde_json's `float_roundtrip`).
use crate::result_summary_tables::{self as tables, js_key, meta, Case, JsKey, Standing};
use serde_json::{json, Value};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

pub(crate) const MAGIC: &[u8; 8] = b"SWBSUMv1";
/// The refusal prefix of a summary response that is not the layout above
/// (the webview's decoder, `resultSummaryDecode.ts`, refuses with the same).
#[cfg(test)]
pub(crate) const MALFORMED: &str = "RESULT-SUMMARY-MALFORMED";

/// The display row columns, in plane order.
pub(crate) const DISPLAY_COLUMNS: [&str; 12] = [
    "display.ordinal",
    "display.value",
    "display.kind",
    "display.entity_ref",
    "display.basis_ref",
    "display.unit",
    "display.coordinate_system",
    "display.component",
    "display.location",
    "display.metadata_basis",
    "display.sign_convention",
    "display.dimension",
];
pub(crate) const SOURCE_OFFSETS: &str = "display.source_offsets";
pub(crate) const SOURCE_REFS: &str = "display.source_refs";
/// The stress row columns, in plane order.
pub(crate) const STRESS_COLUMNS: [&str; 9] = [
    "stress.ordinal",
    "stress.value",
    "stress.case",
    "stress.case_label",
    "stress.entity_ref",
    "stress.station",
    "stress.component",
    "stress.section_basis",
    "stress.unit",
];

const NODAL_KINDS: [&str; 7] = [
    "displacement_magnitude",
    "global_nodal_displacement_x",
    "global_nodal_displacement_y",
    "global_nodal_displacement_z",
    "global_nodal_rotation_x",
    "global_nodal_rotation_y",
    "global_nodal_rotation_z",
];
const SOURCE_ROWS_KIND: &str = "component_equal_factor_intensified_bending_stress_v1";

fn display_kind(row: &Value) -> bool {
    let Some(kind) = row.get("kind").and_then(Value::as_str) else { return false };
    NODAL_KINDS.contains(&kind)
        || [tables::REACTION_COMPONENT_KIND, tables::REACTION_FORCE_MAGNITUDE_KIND, tables::REACTION_MOMENT_MAGNITUDE_KIND].contains(&kind)
        || tables::COLOUR_QUANTITIES.iter().any(|q| q.1 == kind)
}

/// S's value table: each distinct member value once, in first-use order.
/// Strings are keyed by their text; other values by their JSON text.
#[derive(Default)]
pub(crate) struct Values<'a> {
    values: Vec<Value>,
    strings: HashMap<Cow<'a, str>, usize>,
    others: HashMap<String, usize>,
}

impl<'a> Values<'a> {
    fn insert(&mut self, value: Option<Cow<'a, Value>>) -> f64 {
        let Some(value) = value else { return -1.0 };
        let next = self.values.len();
        let at = match &value {
            Cow::Borrowed(borrowed) => {
                let borrowed: &'a Value = borrowed;
                match borrowed {
                    Value::String(text) => *self.strings.entry(Cow::Borrowed(text.as_str())).or_insert(next),
                    other => *self.others.entry(serde_json::to_string(other).expect("a JSON value")).or_insert(next),
                }
            }
            Cow::Owned(Value::String(text)) => *self.strings.entry(Cow::Owned(text.clone())).or_insert(next),
            Cow::Owned(other) => *self.others.entry(serde_json::to_string(other).expect("a JSON value")).or_insert(next),
        };
        if at == next {
            self.values.push(value.into_owned());
        }
        at as f64
    }

    fn lookup(&self, value: Option<Cow<'a, Value>>) -> f64 {
        let Some(value) = value else { return -1.0 };
        let at = match value.as_ref() {
            Value::String(text) => self.strings.get(text.as_str()),
            other => self.others.get(&serde_json::to_string(other).expect("a JSON value")),
        };
        *at.expect("pass B codes only values pass A inserted") as f64
    }

    pub(crate) fn as_slice(&self) -> &[Value] {
        &self.values
    }
}

/// How one pass codes a member value: pass A inserts, pass B looks up.
trait Coder<'a> {
    fn code(&mut self, value: Option<Cow<'a, Value>>) -> f64;
}
struct Insert<'v, 'a>(&'v mut Values<'a>);
struct Lookup<'v, 'a>(&'v Values<'a>);
impl<'a> Coder<'a> for Insert<'_, 'a> {
    fn code(&mut self, value: Option<Cow<'a, Value>>) -> f64 {
        self.0.insert(value)
    }
}
impl<'a> Coder<'a> for Lookup<'_, 'a> {
    fn code(&mut self, value: Option<Cow<'a, Value>>) -> f64 {
        self.0.lookup(value)
    }
}

fn borrowed(value: Option<&Value>) -> Option<Cow<'_, Value>> {
    value.map(Cow::Borrowed)
}

fn display_cells<'a>(row: &'a Value, ordinal: usize, coder: &mut dyn Coder<'a>) -> [f64; 12] {
    [
        ordinal as f64,
        tables::finite(row.get("value")).unwrap_or(f64::NAN),
        coder.code(borrowed(row.get("kind"))),
        coder.code(borrowed(row.get("entity_ref"))),
        coder.code(borrowed(row.get("basis_ref"))),
        coder.code(borrowed(row.get("unit"))),
        coder.code(borrowed(meta(row, "coordinate_system"))),
        coder.code(borrowed(meta(row, "component"))),
        coder.code(borrowed(meta(row, "location"))),
        coder.code(borrowed(meta(row, "basis"))),
        coder.code(borrowed(meta(row, "sign_convention"))),
        coder.code(borrowed(row.get("dimension"))),
    ]
}

/// `metadataText(row, key)` as a value: the string, or `null`.
fn metadata_text<'a>(row: &'a Value, key: &str) -> Cow<'a, Value> {
    match meta(row, key) {
        Some(value @ Value::String(_)) => Cow::Borrowed(value),
        _ => Cow::Owned(Value::Null),
    }
}

fn stress_cells<'a>(row: &'a Value, ordinal: usize, value: f64, case: usize, cases: &[Case], coder: &mut dyn Coder<'a>) -> [f64; 9] {
    let section_basis = match tables::section_basis_of(row) {
        Some(basis) => Cow::Owned(json!(basis)),
        None => Cow::Owned(Value::Null),
    };
    [
        ordinal as f64,
        value,
        case as f64,
        coder.code(Some(Cow::Owned(tables::stress_case_label(row, &cases[case])))),
        coder.code(borrowed(row.get("entity_ref"))),
        coder.code(Some(metadata_text(row, "location"))),
        coder.code(Some(metadata_text(row, "component"))),
        coder.code(Some(section_basis)),
        coder.code(borrowed(row.get("unit"))),
    ]
}

/// The planes' selection and value table, from pass A.
pub(crate) struct Selection<'a> {
    pub(crate) values: Values<'a>,
    /// The display rows' count; pass B selects them again ([`is_display_row`]).
    display_count: usize,
    /// The ordinals of the rows named as sources (display rows whatever their kind).
    source_ordinals: HashSet<i64>,
    /// The source rows' ordinals of each intensified-bending row, by id.
    sources: HashMap<JsKey<'a>, i64>,
    source_count: usize,
    /// Admitted stress rows per group, in group order.
    group_counts: Vec<(usize, usize)>,
}

impl Selection<'_> {
    pub(crate) fn display_count(&self) -> usize {
        self.display_count
    }
    pub(crate) fn source_count(&self) -> usize {
        self.source_count
    }
    pub(crate) fn stress_count(&self) -> usize {
        self.group_counts.iter().map(|g| g.1).sum()
    }
    /// The planes' element counts, in plane order.
    pub(crate) fn plane_counts(&self) -> Vec<(&'static str, usize)> {
        let rows = self.display_count();
        let mut counts: Vec<(&'static str, usize)> = DISPLAY_COLUMNS.iter().map(|name| (*name, rows)).collect();
        counts.push((SOURCE_OFFSETS, rows + 1));
        counts.push((SOURCE_REFS, self.source_count));
        counts.extend(STRESS_COLUMNS.iter().map(|name| (*name, self.stress_count())));
        counts
    }
}

/// Pass A: how many rows the planes hold, and every member value they code.
pub(crate) fn select<'a>(source: &'a Value, cases: &'a [Case<'a>], groups: &tables::StressGroups, standing: Option<&Standing>) -> Selection<'a> {
    let rows = tables::rows(source);
    // The ids the intensified-bending rows name, then the last row of each
    // (`rowsById` keeps the last row of an id).
    let mut wanted: HashSet<JsKey> = HashSet::new();
    for row in rows {
        if row.get("kind").and_then(Value::as_str) == Some(SOURCE_ROWS_KIND) {
            for reference in row.get("source_result_refs").and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]) {
                wanted.insert(js_key(Some(reference)));
            }
        }
    }
    let mut sources: HashMap<JsKey, i64> = HashMap::new();
    if !wanted.is_empty() {
        for (ordinal, row) in rows.iter().enumerate() {
            let id = js_key(row.get("id"));
            if wanted.contains(&id) {
                sources.insert(id, ordinal as i64);
            }
        }
    }
    let source_ordinals: HashSet<i64> = sources.values().copied().collect();
    let mut values = Values::default();
    let mut display_count = 0usize;
    let mut source_count = 0usize;
    for (ordinal, row) in rows.iter().enumerate() {
        if is_display_row(row, ordinal, &source_ordinals) {
            display_count += 1;
            display_cells(row, ordinal, &mut Insert(&mut values));
            source_count += source_refs_of(row).len();
        }
    }
    let group_counts: Vec<(usize, usize)> = groups.groups.iter().map(|(kind, count, _)| (*kind, *count)).collect();
    let index = tables::case_index(cases);
    if !group_counts.is_empty() {
        for (ordinal, row) in rows.iter().enumerate() {
            if let Some((_, case, true, value)) = tables::stress_row(row, &index, standing) {
                stress_cells(row, ordinal, value, case, cases, &mut Insert(&mut values));
            }
        }
    }
    Selection { values, display_count, source_ordinals, sources, source_count, group_counts }
}

fn is_display_row(row: &Value, ordinal: usize, sources: &HashSet<i64>) -> bool {
    display_kind(row) || sources.contains(&(ordinal as i64))
}

fn source_refs_of(row: &Value) -> &[Value] {
    if row.get("kind").and_then(Value::as_str) != Some(SOURCE_ROWS_KIND) {
        return &[];
    }
    row.get("source_result_refs").and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[])
}

/// The header's plane descriptors and the planes' total bytes.
pub(crate) fn descriptors(selection: &Selection) -> (Value, usize) {
    let mut offset = 0usize;
    let planes = selection
        .plane_counts()
        .into_iter()
        .map(|(name, count)| {
            let descriptor = json!({"name": name, "offset": offset, "count": count});
            offset += count * 8;
            descriptor
        })
        .collect();
    (Value::Array(planes), offset)
}

/// The response's length for a header of `header_len` bytes.
pub(crate) fn response_len(header_len: usize, planes_bytes: usize) -> usize {
    let head = MAGIC.len() + 8 + header_len;
    head + padding(head) + planes_bytes
}

fn padding(head: usize) -> usize {
    (8 - head % 8) % 8
}

/// Pass B: the response, written into `out` (empty, its capacity reserved
/// as [`response_len`]): the prefix, the header, the padding and the planes,
/// each plane written in place from the rows.
pub(crate) fn write<'a>(out: &mut Vec<u8>, header: &[u8], source: &'a Value, cases: &'a [Case<'a>], selection: &Selection<'a>, standing: Option<&Standing>) {
    let rows = tables::rows(source);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&(header.len() as u64).to_le_bytes());
    out.extend_from_slice(header);
    out.resize(out.len() + padding(out.len()), 0);
    let base = out.len();
    let counts = selection.plane_counts();
    out.resize(base + counts.iter().map(|c| c.1 * 8).sum::<usize>(), 0);
    let mut starts = Vec::with_capacity(counts.len());
    let mut offset = base;
    for (_, count) in &counts {
        starts.push(offset);
        offset += count * 8;
    }
    let mut put = |plane: usize, index: usize, value: f64| {
        let at = starts[plane] + index * 8;
        out[at..at + 8].copy_from_slice(&value.to_le_bytes());
    };
    let lookup = &mut Lookup(&selection.values);
    let mut refs = 0usize;
    let mut index = 0usize;
    for (ordinal, row) in rows.iter().enumerate() {
        if !is_display_row(row, ordinal, &selection.source_ordinals) {
            continue;
        }
        for (column, cell) in display_cells(row, ordinal, lookup).into_iter().enumerate() {
            put(column, index, cell);
        }
        put(DISPLAY_COLUMNS.len(), index, refs as f64);
        for reference in source_refs_of(row) {
            let ordinal = selection.sources.get(&js_key(Some(reference))).copied().unwrap_or(-1);
            put(DISPLAY_COLUMNS.len() + 1, refs, ordinal as f64);
            refs += 1;
        }
        index += 1;
    }
    put(DISPLAY_COLUMNS.len(), index, refs as f64);
    let stress_base = DISPLAY_COLUMNS.len() + 2;
    let index = tables::case_index(cases);
    let mut cursors: Vec<usize> = Vec::new();
    let mut group_of = vec![usize::MAX; tables::STRESS_KINDS.len()];
    let mut first = 0usize;
    for (group, (kind, count)) in selection.group_counts.iter().enumerate() {
        group_of[*kind] = group;
        cursors.push(first);
        first += count;
    }
    // No groups (mechanics not solved): no stress rows.
    let any = !cursors.is_empty();
    for (ordinal, row) in rows.iter().enumerate().filter(|_| any) {
        let Some((kind, case, true, value)) = tables::stress_row(row, &index, standing) else { continue };
        let slot = &mut cursors[group_of[kind]];
        for (column, cell) in stress_cells(row, ordinal, value, case, cases, lookup).into_iter().enumerate() {
            put(stress_base + column, *slot, cell);
        }
        *slot += 1;
    }
}

/// A decoded response: the header and each plane's values.
#[cfg(test)]
#[derive(Debug)]
pub(crate) struct Decoded {
    pub(crate) header: Value,
    pub(crate) planes: Vec<(String, Vec<f64>)>,
}

#[cfg(test)]
impl Decoded {
    pub(crate) fn plane(&self, name: &str) -> &[f64] {
        &self.planes.iter().find(|p| p.0 == name).unwrap_or_else(|| panic!("no plane {name}")).1
    }
}

/// Reads a response, refusing by name any layout other than the module's:
/// a short or foreign prefix, a header past the end, a header that is not a
/// JSON object with plane descriptors, non-zero padding, a plane that is not
/// 8-aligned, contiguous and in bounds, or bytes after the last plane.
#[cfg(test)]
pub(crate) fn decode(bytes: &[u8]) -> Result<Decoded, String> {
    let refuse = |reason: &str| format!("{MALFORMED}: {reason}");
    if bytes.len() < MAGIC.len() + 8 || &bytes[..MAGIC.len()] != MAGIC {
        return Err(refuse("not a result summary"));
    }
    let header_len = u64::from_le_bytes(bytes[8..16].try_into().expect("8 bytes"));
    let head = usize::try_from(header_len).ok().and_then(|len| len.checked_add(16)).filter(|end| *end <= bytes.len()).ok_or_else(|| refuse("the header runs past the end"))?;
    let header: Value = serde_json::from_slice(&bytes[16..head]).map_err(|e| refuse(&format!("the header is not JSON: {e}")))?;
    let pad = padding(head);
    let start = head.checked_add(pad).filter(|s| *s <= bytes.len()).ok_or_else(|| refuse("the padding runs past the end"))?;
    if bytes[head..start].iter().any(|b| *b != 0) {
        return Err(refuse("the padding is not zero"));
    }
    let total = header.get("planes_bytes").and_then(Value::as_u64).ok_or_else(|| refuse("no planes_bytes"))?;
    if bytes.len() - start != total as usize || bytes.len() as u64 - start as u64 != total {
        return Err(refuse("the planes are not the response's remaining bytes"));
    }
    let descriptors = header.get("planes").and_then(Value::as_array).ok_or_else(|| refuse("no plane descriptors"))?;
    let mut planes = Vec::with_capacity(descriptors.len());
    let mut expected = 0u64;
    for descriptor in descriptors {
        let name = descriptor.get("name").and_then(Value::as_str).ok_or_else(|| refuse("a plane without a name"))?;
        let offset = descriptor.get("offset").and_then(Value::as_u64).ok_or_else(|| refuse("a plane without an offset"))?;
        let count = descriptor.get("count").and_then(Value::as_u64).ok_or_else(|| refuse("a plane without a count"))?;
        if offset != expected || offset % 8 != 0 {
            return Err(refuse(&format!("plane {name} is not where the previous one ends")));
        }
        let end = count.checked_mul(8).and_then(|len| offset.checked_add(len)).filter(|end| *end <= total).ok_or_else(|| refuse(&format!("plane {name} runs past the planes")))?;
        if planes.iter().any(|(existing, _): &(String, Vec<f64>)| existing == name) {
            return Err(refuse(&format!("plane {name} is stated twice")));
        }
        let slice = &bytes[start + offset as usize..start + end as usize];
        planes.push((name.to_string(), slice.chunks_exact(8).map(|c| f64::from_le_bytes(c.try_into().expect("8 bytes"))).collect()));
        expected = end;
    }
    if expected != total {
        return Err(refuse("bytes follow the last plane"));
    }
    Ok(Decoded { header, planes })
}
