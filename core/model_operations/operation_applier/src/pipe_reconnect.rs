//! Pipe reconnect (W-10, FR-GUI-004): `reconnect_pipe_run` moves one end of a
//! pipe P from the node O it leaves to an existing node T; its other end F
//! stays. Called solely by the common applier. The rules, refusals R1-R13 and
//! warnings are DEL-16-06 Design `pipe-reconnect.md` §1-§2. The same scan
//! (`references`, with the root `diagnostics` skipped) finds the dependents a
//! `delete_pipe_run` refuses on (§7).
//!
//! Orientation is checked through the frame kernel's own constructor
//! (`FrameOrientation::from_x_axis_and_y_reference`), the one the solve uses
//! for every straight element, so a reconnect the applier admits is not
//! refused by the kernel's orthogonality tolerance later. No orientation is
//! ever derived: an absent `y_reference` stays absent, with a warning. The
//! same predicate (`kernel_orientation`) checks `connect_pipe_run` (run-sheet
//! design O4, DEL-07-02 `run-sheet-entry.md` §8).
use crate::geometry_operations::{
    convert, diff, fields, record_shape, reference_segments, render_suffix, text, unique, vector, GeometryError, Seg,
};
use crate::{canonical_json, DiffPreviewRow, Dimension};
use open_pipe_stress_frame_kernel::{FrameKernelError, FrameOrientation};
use open_pipe_stress_product_physics::TANGENCY_TOLERANCE_RAD;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub(crate) const CHANGE_KIND: &str = "reconnect_pipe_run";
/// The frame kernel's `AXIS_TOLERANCE` (private there): a chord whose metre
/// norm is at or below it is degenerate (`normalize`, `frame_kernel` lib.rs).
const AXIS_TOLERANCE: f64 = 1.0e-12;
/// Relative tolerance of the coincident-span warning (§2.4).
const COINCIDENT_RELATIVE: f64 = 1.0e-9;

/// One reconnect warning; never blocking. Code and refs are kept typed so the
/// review shows them (§1, "Return type").
pub(crate) struct ReconnectWarning {
    pub code: &'static str,
    pub message: String,
    pub remediation: String,
    pub affected_refs: Vec<String>,
}
pub(crate) struct ReconnectEdit {
    pub model: Value,
    pub diff_preview: Vec<DiffPreviewRow>,
    pub warnings: Vec<ReconnectWarning>,
}
/// One refusal. Each blocker is its own diagnostic; `affected_refs` lists P
/// first, then the blocker ids (§3, "Refusal display").
pub(crate) struct ReconnectBlock {
    pub code: &'static str,
    pub message: String,
    pub remediation: String,
    pub affected_refs: Vec<String>,
}
impl From<GeometryError> for ReconnectBlock {
    fn from(error: GeometryError) -> Self {
        Self {
            code: error.code,
            message: error.message,
            remediation: "Correct the explicit reconnect payload or the malformed model record before retrying.".into(),
            affected_refs: error.affected_refs,
        }
    }
}
type Blocks = Vec<ReconnectBlock>;

fn block(code: &'static str, message: String, remediation: impl Into<String>, refs: Vec<String>) -> ReconnectBlock {
    ReconnectBlock { code, message, remediation: remediation.into(), affected_refs: dedup(refs) }
}
fn warning(code: &'static str, message: String, remediation: impl Into<String>, refs: Vec<String>) -> ReconnectWarning {
    ReconnectWarning { code, message, remediation: remediation.into(), affected_refs: dedup(refs) }
}
fn dedup(refs: Vec<String>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    refs.into_iter().filter(|r| seen.insert(r.clone())).collect()
}
fn payload_invalid(message: impl Into<String>, refs: Vec<String>) -> Blocks {
    vec![block(
        "OP-RECONNECT-PAYLOAD-INVALID",
        message.into(),
        "Send reconnect_pipe_run with modify/Element/pipe_segments/none/dimensionless and an after payload {pipe_ref, end, target_node_ref, optional y_reference}.",
        refs,
    )]
}

// ------------------------------------------------------------------ paths

/// `*` matches an array index only, never an object key (not even a key
/// spelled `*`); every other token matches one object key exactly.
fn matches(segs: &[Seg], pattern: &[&str]) -> bool {
    segs.len() == pattern.len()
        && segs.iter().zip(pattern).all(|(seg, token)| match seg {
            Seg::Index(_) => *token == "*",
            Seg::Key(key) => *token != "*" && key == token,
        })
}
fn at<'a>(model: &'a Value, segs: &[Seg]) -> Option<&'a Value> {
    segs.iter().try_fold(model, |value, seg| match seg {
        Seg::Key(key) => value.get(key),
        Seg::Index(index) => value.get(*index),
    })
}
/// A path as display text (`render_suffix` without its leading dot).
pub(crate) fn render(segs: &[Seg]) -> String {
    let text = render_suffix(segs);
    text.strip_prefix('.').map(str::to_string).unwrap_or(text)
}
/// §7 owner mapping: the first `collection[i]` segment gives the owning
/// record (its `id`); the rest of the path is the slot. A root object member
/// owns itself.
pub(crate) fn owner_of(model: &Value, segs: &[Seg]) -> (String, String) {
    match (segs.first(), segs.get(1)) {
        (Some(Seg::Key(key)), Some(Seg::Index(index))) => {
            let owner = model
                .get(key)
                .and_then(|rows| rows.get(*index))
                .and_then(|row| row.get("id"))
                .and_then(Value::as_str)
                .map(str::to_string)
                .unwrap_or_else(|| render(&segs[..2]));
            (owner, render(&segs[2..]))
        }
        (Some(Seg::Key(key)), _) => (key.clone(), render(&segs[1..])),
        _ => (render(segs), String::new()),
    }
}

/// The one scanner (§2.2), `reference_segments`: every exact occurrence of
/// `ids` in the document, as structured segment paths, except the root
/// `diagnostics` (derived records), the root `label`/`name`/`provenance`
/// subtrees, the pipe record `skip_pipe` and the node records in `skip_nodes`
/// (self-references). The model-data collections are not scanned either
/// (W-13 §6): naming an entity in a line, assumption, constraint, equipment
/// interface or rule-pack reference never blocks its edit or deletion, a
/// deleted pipe leaves its line explicitly, and a missing subject is the
/// document validator's warning.
pub(crate) fn scan(model: &Value, ids: &BTreeSet<String>, skip_pipe: Option<&str>, skip_nodes: &BTreeSet<String>) -> Vec<Vec<Seg>> {
    let mut out = Vec::new();
    let Some(root) = model.as_object() else {
        return out;
    };
    for (key, value) in root {
        if matches!(key.as_str(), "diagnostics" | "label" | "name" | "provenance")
            || crate::model_data_authoring::MODEL_DATA_COLLECTIONS.contains(&key.as_str())
        {
            continue;
        }
        let skip = |row: &Value| {
            let id = row.get("id").and_then(Value::as_str);
            match key.as_str() {
                "pipe_segments" => id.is_some() && id == skip_pipe,
                "nodes" => id.is_some_and(|id| skip_nodes.contains(id)),
                _ => false,
            }
        };
        match value.as_array() {
            Some(rows) if matches!(key.as_str(), "pipe_segments" | "nodes") => {
                for (index, row) in rows.iter().enumerate() {
                    if !skip(row) {
                        reference_segments(row, ids, &mut vec![Seg::Key(key.clone()), Seg::Index(index)], &mut out);
                    }
                }
            }
            _ => reference_segments(value, ids, &mut vec![Seg::Key(key.clone())], &mut out),
        }
    }
    out
}

/// §7: the dependents of pipe `pipe` that `delete_pipe_run`'s two existing
/// checks do not already report, as (owner, slot), sorted and de-duplicated.
pub(crate) fn delete_dependents(model: &Value, pipe: &str) -> Vec<(String, String)> {
    let ids = BTreeSet::from([pipe.to_string()]);
    let reported: [&[&str]; 4] = [
        &["load_cases", "*", "primitive_loads", "*", "target", "pipe"],
        &["load_cases", "*", "pressure_regions", "*", "member_pipe_ids", "*"],
        &["load_cases", "*", "analysis_state", "element_states", "*", "pipe_ref"],
        &["reference_configurations", "*", "member_references", "*", "pipe_ref"],
    ];
    let mut out: Vec<(String, String)> = scan(model, &ids, Some(pipe), &BTreeSet::new())
        .into_iter()
        .filter(|segs| !reported.iter().any(|pattern| matches(segs, pattern)))
        .map(|segs| owner_of(model, &segs))
        .collect();
    // A collection the checks read as an array but that holds something else
    // cannot be checked: it is a named dependent, never read as empty.
    out.extend(malformed_collections(model).iter().map(|segs| {
        let (owner, slot) = owner_of(model, segs);
        (owner, if slot.is_empty() { "not an array".to_string() } else { format!("{slot}, not an array") })
    }));
    out.sort();
    out.dedup();
    out
}

/// Every collection a reconnect or delete check reads as an array that is
/// present with any other value (object, string, number, boolean or null):
/// root `nodes`, `pipe_segments`, `supports`, `components`, `load_cases` and
/// `reference_configurations`; per load case `primitive_loads`,
/// `pressure_regions`, `analysis_state.element_states` and
/// `equivalent_static.wind.exposed_pipe_refs` / `exposed_spans`; per region
/// `member_pipe_ids`, `member_connector_ids` and `terminals`; per reference
/// configuration `member_references`. Such a value is unscannable, never empty.
pub(crate) fn malformed_collections(model: &Value) -> Vec<Vec<Seg>> {
    fn key(path: &[Seg], name: &str) -> Vec<Seg> {
        let mut next = path.to_vec();
        next.push(Seg::Key(name.to_string()));
        next
    }
    fn check(parent: &Value, path: &[Seg], names: &[&str], out: &mut Vec<Vec<Seg>>) {
        for name in names {
            if parent.get(*name).is_some_and(|value| !value.is_array()) {
                out.push(key(path, name));
            }
        }
    }
    fn rows<'a>(parent: &'a Value, path: &[Seg], name: &str) -> Vec<(Vec<Seg>, &'a Value)> {
        parent
            .get(name)
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .enumerate()
            .map(|(index, row)| {
                let mut at = key(path, name);
                at.push(Seg::Index(index));
                (at, row)
            })
            .collect()
    }
    let mut out = Vec::new();
    check(model, &[], &["nodes", "pipe_segments", "supports", "components", "load_cases", "reference_configurations"], &mut out);
    for (case_path, case) in rows(model, &[], "load_cases") {
        check(case, &case_path, &["primitive_loads", "pressure_regions"], &mut out);
        if let Some(state) = case.get("analysis_state") {
            check(state, &key(&case_path, "analysis_state"), &["element_states"], &mut out);
        }
        if let Some(wind) = case.pointer("/equivalent_static/wind") {
            let wind_path = key(&key(&case_path, "equivalent_static"), "wind");
            check(wind, &wind_path, &["exposed_pipe_refs", "exposed_spans"], &mut out);
        }
        for (region_path, region) in rows(case, &case_path, "pressure_regions") {
            check(region, &region_path, &["member_pipe_ids", "member_connector_ids", "terminals"], &mut out);
        }
    }
    for (configuration_path, configuration) in rows(model, &[], "reference_configurations") {
        check(configuration, &configuration_path, &["member_references"], &mut out);
    }
    out
}

// ------------------------------------------------------------------ geometry

type Vec3 = [f64; 3];
fn sub(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn norm(a: Vec3) -> f64 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}
fn position(model: &Value, node: &str) -> Result<Vec3, GeometryError> {
    vector(&unique(model, "nodes", node)?["position"])
}

/// Why the kernel orientation predicate refused a pipe.
pub(crate) enum OrientationFault {
    /// R6: the chord in metres cannot be formed, is nonfinite, or is at or
    /// below the kernel's axis tolerance. The text completes "the chord ...".
    Chord(String),
    /// R7: the kernel's orientation constructor refuses the chord with the
    /// effective `y_reference`.
    Orientation(FrameKernelError),
}

/// The kernel orientation predicate, shared by `reconnect_pipe_run` (R6, R7,
/// §2.3) and `connect_pipe_run` (run-sheet design O4), so the two operations
/// apply one rule. Each endpoint position (stored in the project length unit
/// `unit`) is converted to metres and only then subtracted, `to - from`: the
/// solve converts node positions to metres (`product_physics` unit
/// normalization) and `FrameElement::local_x_axis` subtracts them, so the
/// chord here is the solve's chord bit for bit. The chord is checked as the
/// kernel's private `normalize` checks it, then handed with `y` to
/// `FrameOrientation::from_x_axis_and_y_reference`, the constructor the solve
/// uses for every straight element. Kernel errors map by variant and call
/// order. `y = None` checks the chord only.
pub(crate) fn kernel_orientation(from: Vec3, to: Vec3, unit: &str, y: Option<Vec3>) -> Result<(), OrientationFault> {
    let metres = |value: f64| {
        convert(value, unit, "m", Dimension::Length)
            .map_err(|_| OrientationFault::Chord("cannot be converted to metres as a finite length".into()))
    };
    let mut chord = [0.0; 3];
    for i in 0..3 {
        chord[i] = metres(to[i])? - metres(from[i])?;
    }
    let length = norm(chord);
    if !length.is_finite() || length <= AXIS_TOLERANCE {
        return Err(OrientationFault::Chord(format!(
            "is degenerate or nonfinite (|x| = {length:e} m, frame kernel axis tolerance 1e-12)"
        )));
    }
    let Some(y) = y else {
        return Ok(());
    };
    FrameOrientation::from_x_axis_and_y_reference(chord, y).map(|_| ()).map_err(|error| match error {
        FrameKernelError::NonFiniteInput { name: "local_x_axis", .. } => OrientationFault::Chord("is nonfinite".into()),
        other => OrientationFault::Orientation(other),
    })
}

/// R6 then R7 (§2.3) for the reconnected pipe running `from` → `to`.
fn orientation(from: Vec3, to: Vec3, unit: &str, y: Option<Vec3>, pipe: &str, refs: &[String]) -> Result<(), ReconnectBlock> {
    kernel_orientation(from, to, unit, y).map_err(|fault| match fault {
        OrientationFault::Chord(detail) => block(
            "OP-RECONNECT-GEOMETRY-DEGENERATE",
            format!("Cannot reconnect {pipe}: the new chord {detail}."),
            "Choose a target node at a distinct finite position.",
            refs.to_vec(),
        ),
        OrientationFault::Orientation(other) => block(
            "OP-RECONNECT-ORIENTATION-DEGENERATE",
            format!(
                "Cannot reconnect {pipe}: the frame kernel refuses the new chord with its y_reference ({other:?}); the local axes would not be an orthonormal right-handed basis within 1e-12."
            ),
            "Enter a y_reference that is not parallel to the new pipe direction.",
            refs.to_vec(),
        ),
    })
}

// ------------------------------------------------------------------ resolve

struct Ends<'a> {
    pipe: &'a str,
    end: &'a str,
    fixed: String,
    old: String,
    target: String,
}
impl Ends<'_> {
    fn role(&self, node: &str) -> &'static str {
        if node == self.fixed {
            "changes the pipe's direction"
        } else if node == self.old {
            "removes the pipe"
        } else {
            "adds the pipe"
        }
    }
    fn nodes(&self) -> BTreeSet<String> {
        BTreeSet::from([self.fixed.clone(), self.old.clone(), self.target.clone()])
    }
}

pub(crate) fn resolve(model: &Value, intent: &Value) -> Result<ReconnectEdit, Blocks> {
    let target_ref = intent["target"]["ref"].as_str().unwrap_or("").to_string();
    if model["document_kind"] != "openpipestress.product_preview.model"
        || model["schema_version"] != open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION
    {
        return Err(payload_invalid(
            format!(
                "Reconnect requires the current product_preview.model schema {}.",
                open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION
            ),
            vec![target_ref],
        ));
    }
    if intent["operation_kind"] != "modify"
        || intent["target"]["object_type"] != "Element"
        || intent["change"]["field_path"] != "pipe_segments"
        || intent["change"]["unit"] != "none"
        || intent["change"]["dimension"] != "dimensionless"
    {
        return Err(payload_invalid(
            "Reconnect wire requires modify/Element/pipe_segments/none/dimensionless.",
            vec![target_ref],
        ));
    }
    // R1: payload shape.
    let payload: Value = intent["change"]["after"]
        .as_str()
        .and_then(|after| serde_json::from_str(after).ok())
        .ok_or_else(|| payload_invalid("change.after must be the reconnect payload as JSON text.", vec![target_ref.clone()]))?;
    fields(&payload, &["pipe_ref", "end", "target_node_ref"], &["y_reference"])
        .map_err(|e| payload_invalid(format!("Reconnect payload: {}", e.message), vec![target_ref.clone()]))?;
    let pipe_id = text(&payload, "pipe_ref").map_err(|e| payload_invalid(e.message, vec![target_ref.clone()]))?;
    let end = text(&payload, "end").map_err(|e| payload_invalid(e.message, vec![target_ref.clone()]))?;
    let target = text(&payload, "target_node_ref").map_err(|e| payload_invalid(e.message, vec![target_ref.clone()]))?;
    if pipe_id != target_ref {
        return Err(payload_invalid(
            format!("pipe_ref {pipe_id} must equal target.ref {target_ref}."),
            vec![target_ref.clone(), pipe_id.to_string()],
        ));
    }
    if !matches!(end, "from" | "to") {
        return Err(payload_invalid(format!("end must be from or to, not {end}."), vec![pipe_id.to_string()]));
    }
    let entered_y = match payload.get("y_reference") {
        Some(y) => Some(
            vector(y).map_err(|e| payload_invalid(format!("y_reference: {}", e.message), vec![pipe_id.to_string()]))?,
        ),
        None => None,
    };
    // R1: P present exactly once.
    let pipes = model.get("pipe_segments").and_then(Value::as_array);
    let count = pipes.map_or(0, |rows| rows.iter().filter(|p| p["id"] == pipe_id).count());
    if count != 1 {
        return Err(vec![block(
            "OP-TARGET-NOT-FOUND",
            format!("Pipe segment `{pipe_id}` was not found exactly once in the current model."),
            "Select an existing pipe segment before reconnecting it.",
            vec![pipe_id.to_string()],
        )]);
    }
    let pipe = unique(model, "pipe_segments", pipe_id).map_err(|e| vec![e.into()])?;
    // R2: the before value is P's current canonical record.
    let current = canonical_json(pipe);
    if intent["change"]["before"].as_str() != Some(current.as_str()) {
        return Err(vec![block(
            "OP-STALE-BEFORE-VALUE",
            format!("Intent before-value no longer matches the current canonical record `{current}` of `{pipe_id}`."),
            "Re-queue the edit from the current model state; stale intents are not applied.",
            vec![pipe_id.to_string()],
        )]);
    }
    record_shape(pipe, false).map_err(|e| vec![ReconnectBlock::from(e)])?;
    let unit = text(&model["project"]["units"], "length").map_err(|e| vec![e.into()])?;
    convert(1.0, unit, "m", Dimension::Length).map_err(|e| vec![e.into()])?;
    let other = if end == "to" { "from" } else { "to" };
    let ends = Ends {
        pipe: pipe_id,
        end,
        fixed: text(pipe, other).map_err(|e| vec![e.into()])?.to_string(),
        old: text(pipe, end).map_err(|e| vec![e.into()])?.to_string(),
        target: target.to_string(),
    };
    // R3-R5.
    let nodes = model.get("nodes").and_then(Value::as_array);
    if nodes.map_or(0, |rows| rows.iter().filter(|n| n["id"] == target).count()) != 1 {
        return Err(vec![block(
            "OP-RECONNECT-TARGET-NODE-NOT-FOUND",
            format!("Cannot reconnect {pipe_id}: target node {target} is not a node of the model."),
            "Choose an existing node, or create it in the same batch before the reconnect.",
            vec![pipe_id.to_string(), target.to_string()],
        )]);
    }
    if target == ends.old {
        return Err(vec![block(
            "OP-RECONNECT-NO-CHANGE",
            format!("Cannot reconnect {pipe_id}: its {end} end is already at {target}."),
            "Choose a different target node.",
            vec![pipe_id.to_string(), target.to_string()],
        )]);
    }
    if target == ends.fixed {
        return Err(vec![block(
            "OP-RECONNECT-ZERO-LENGTH",
            format!("Cannot reconnect {pipe_id}: {target} is its other end, so the pipe would have zero length."),
            "Choose a target node other than the pipe's fixed end.",
            vec![pipe_id.to_string(), target.to_string()],
        )]);
    }
    for node in ends.nodes() {
        let record = unique(model, "nodes", &node).map_err(|e| vec![e.into()])?;
        record_shape(record, true).map_err(|e| vec![e.into()])?;
    }
    let (fixed_at, target_at) = (
        position(model, &ends.fixed).map_err(|e| vec![e.into()])?,
        position(model, &ends.target).map_err(|e| vec![e.into()])?,
    );
    let (from_at, to_at) = if end == "to" { (fixed_at, target_at) } else { (target_at, fixed_at) };
    let kept_y = match pipe.get("y_reference") {
        Some(y) => Some(vector(y).map_err(|e| vec![e.into()])?),
        None => None,
    };
    let effective_y = entered_y.or(kept_y);

    let mut blocks = Vec::new();
    let mut warnings = Vec::new();
    // R6, R7.
    if let Err(error) = orientation(from_at, to_at, unit, effective_y, pipe_id, &[pipe_id.to_string(), ends.fixed.clone(), target.to_string()]) {
        blocks.push(error);
    }
    for segs in malformed_collections(model) {
        let (owner, _) = owner_of(model, &segs);
        blocks.push(block(
            "OP-RECONNECT-ATTACHMENT-UNSUPPORTED",
            format!("Cannot reconnect {pipe_id}: {} is not an array, so reconnect cannot check the records it holds.", render(&segs)),
            "Repair the collection as an array of records, then reconnect.",
            vec![pipe_id.to_string(), owner],
        ));
    }
    components(model, &ends, &mut blocks);
    regions(model, &ends, &mut blocks);
    pipe_slots(model, &ends, &mut blocks, &mut warnings);
    node_slots(model, &ends, &mut blocks, &mut warnings);
    if !blocks.is_empty() {
        return Err(blocks);
    }

    let mut next = pipe.clone();
    next[end] = Value::String(target.to_string());
    if let Some(y) = payload.get("y_reference") {
        next["y_reference"] = y.clone();
    }
    let mut candidate = model.clone();
    let list = candidate["pipe_segments"].as_array_mut().expect("checked above");
    let index = list.iter().position(|p| p["id"] == pipe_id).expect("checked above");
    list[index] = next.clone();
    topology_warnings(model, &candidate, &ends, &mut warnings).map_err(|e| vec![e.into()])?;
    if next.get("y_reference").is_none() {
        warnings.push(warning(
            "OP-RECONNECT-ORIENTATION-MISSING",
            format!("{pipe_id} has no y_reference after this reconnect; the solve will refuse it as PIPE_ORIENTATION_INPUT_MISSING until one is entered."),
            "Enter the pipe's y_reference (in this reconnect or with a later edit) before solving.",
            vec![pipe_id.to_string()],
        ));
    }
    Ok(ReconnectEdit {
        diff_preview: vec![diff(pipe_id, "Element", "pipe_segments", Some(pipe), &next, CHANGE_KIND)],
        model: candidate,
        warnings,
    })
}

fn component_kind(component: &Value) -> &str {
    component.get("kind").and_then(Value::as_str).unwrap_or("unknown")
}

/// R9: a component that names P; a non-terminal component at F, O or T; a
/// component that names F, O or T in any slot other than a terminal's `node`.
fn components(model: &Value, ends: &Ends, blocks: &mut Blocks) {
    let pipe = ends.pipe;
    let remediation = |component: &str| format!("Delete or re-author {component}, reconnect, then author the component again.");
    let pipe_ids = BTreeSet::from([pipe.to_string()]);
    let node_ids = ends.nodes();
    for (index, component) in model.get("components").and_then(Value::as_array).into_iter().flatten().enumerate() {
        let id = component
            .get("id")
            .and_then(Value::as_str)
            .map(str::to_string)
            .unwrap_or_else(|| format!("components[{index}]"));
        let kind = component_kind(component);
        let mut named = Vec::new();
        reference_segments(component, &pipe_ids, &mut Vec::new(), &mut named);
        for segs in named {
            let slot = render(&segs);
            blocks.push(block(
                "OP-RECONNECT-COMPONENT-ATTACHED",
                format!("Cannot reconnect {pipe}: component {id} ({kind}) names it in {slot}."),
                remediation(&id),
                vec![pipe.to_string(), id.clone()],
            ));
        }
        let mut at_nodes = Vec::new();
        reference_segments(component, &node_ids, &mut Vec::new(), &mut at_nodes);
        for segs in at_nodes {
            let slot = render(&segs);
            let node = at(component, &segs).and_then(Value::as_str).unwrap_or("").to_string();
            if segs == [Seg::Key("node".into())] {
                if kind == "terminal" {
                    continue;
                }
                blocks.push(block(
                    "OP-RECONNECT-COMPONENT-ATTACHED",
                    format!(
                        "Cannot reconnect {pipe}: component {id} ({kind}) sits at node {node}, where this reconnect {}.",
                        ends.role(&node)
                    ),
                    remediation(&id),
                    vec![pipe.to_string(), id.clone(), node],
                ));
            } else {
                blocks.push(block(
                    "OP-RECONNECT-COMPONENT-ATTACHED",
                    format!(
                        "Cannot reconnect {pipe}: component {id} ({kind}) names node {node} in {slot}, where this reconnect {}.",
                        ends.role(&node)
                    ),
                    remediation(&id),
                    vec![pipe.to_string(), id.clone(), node],
                ));
            }
        }
    }
}

/// R11: T is an internal node of a region (an end of a member pipe or member
/// connector span that is not one of its terminals). R10 comes from the scan.
fn regions(model: &Value, ends: &Ends, blocks: &mut Blocks) {
    let pipe_ends = |id: &str| -> Vec<String> {
        model
            .get("pipe_segments")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter(|p| p["id"] == id)
            .flat_map(|p| ["from", "to"].map(|k| p.get(k).and_then(Value::as_str).unwrap_or("").to_string()))
            .collect()
    };
    let connector_span = |id: &str| -> Option<String> {
        model
            .get("components")
            .and_then(Value::as_array)?
            .iter()
            .find(|c| c["id"] == id)?
            .pointer("/objective_connector/topology/span_ref")
            .and_then(Value::as_str)
            .map(str::to_string)
    };
    for case in model.get("load_cases").and_then(Value::as_array).into_iter().flatten() {
        let case_id = case.get("id").and_then(Value::as_str).unwrap_or("load_case:unknown");
        for region in case.get("pressure_regions").and_then(Value::as_array).into_iter().flatten() {
            let region_id = region.get("id").and_then(Value::as_str).unwrap_or("region:unknown");
            let mut spans: Vec<String> = region
                .get("member_pipe_ids")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect();
            spans.extend(
                region
                    .get("member_connector_ids")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .filter_map(Value::as_str)
                    .filter_map(connector_span),
            );
            let terminals: BTreeSet<&str> = region
                .get("terminals")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(|t| t.get("node_ref").and_then(Value::as_str))
                .collect();
            let internal = spans.iter().flat_map(|s| pipe_ends(s)).any(|n| n == ends.target)
                && !terminals.contains(ends.target.as_str());
            if internal {
                let (end, pipe, node) = (ends.end, ends.pipe, &ends.target);
                blocks.push(block(
                    "OP-RECONNECT-PRESSURE-REGION-BRANCH",
                    format!("Cannot move the {end} end of {pipe} to {node}: {node} is an internal node of pressure region {region_id} in load case {case_id}, and a pipe outside the region cannot join it there."),
                    format!("Choose another node, or end {region_id} at {node} before reconnecting."),
                    vec![pipe.to_string(), region_id.to_string(), case_id.to_string(), node.clone()],
                ));
            }
        }
    }
}

fn unsupported(model: &Value, ends: &Ends, id: &str, segs: &[Seg], shape: bool) -> ReconnectBlock {
    let pipe = ends.pipe;
    let (owner, _) = owner_of(model, segs);
    let path = render(segs);
    let detail = if shape { " (the record there is not of the exact supported shape)" } else { "" };
    block(
        "OP-RECONNECT-ATTACHMENT-UNSUPPORTED",
        format!("Cannot reconnect {pipe}: {id} is named at {path}, which reconnect does not carry across a geometry change{detail}."),
        "Remove or re-author that record, then reconnect.",
        vec![pipe.to_string(), id.to_string(), owner],
    )
}
fn keys_within(value: Option<&Value>, allowed: &[&str], required: &[&str]) -> bool {
    value.and_then(Value::as_object).is_some_and(|map| {
        map.keys().all(|k| allowed.contains(&k.as_str())) && required.iter().all(|k| map.contains_key(*k))
    })
}

/// §2.2, pipe id P: the allowed slots (kept, with their warnings), R10 and
/// R13; any other slot refuses (R12). Components are R9's.
fn pipe_slots(model: &Value, ends: &Ends, blocks: &mut Blocks, warnings: &mut Vec<ReconnectWarning>) {
    let pipe = ends.pipe;
    let ids = BTreeSet::from([pipe.to_string()]);
    for segs in scan(model, &ids, Some(pipe), &BTreeSet::new()) {
        let path = render(&segs);
        // Only R9's array records are exempt; any other `components` value is scanned.
        if matches!(segs.as_slice(), [Seg::Key(k), Seg::Index(_), ..] if k == "components") {
            continue;
        }
        let case_id = || {
            at(model, &segs[..2]).and_then(|c| c.get("id")).and_then(Value::as_str).unwrap_or("load_case:unknown").to_string()
        };
        if matches(&segs, &["load_cases", "*", "pressure_regions", "*", "member_pipe_ids", "*"]) {
            let region = at(model, &segs[..4]).and_then(|r| r.get("id")).and_then(Value::as_str).unwrap_or("region:unknown").to_string();
            let case = case_id();
            blocks.push(block(
                "OP-RECONNECT-PRESSURE-REGION-MEMBER",
                format!("Cannot reconnect {pipe}: it is a member of pressure region {region} in load case {case}."),
                format!("Remove {pipe} from {region}, reconnect, then re-enter the region's members and terminals."),
                vec![pipe.to_string(), region, case],
            ));
        } else if matches(&segs, &["load_cases", "*", "primitive_loads", "*", "target", "pipe"]) {
            let load = at(model, &segs[..4]);
            let category = load.and_then(|l| l.get("category")).and_then(Value::as_str);
            let exact = keys_within(load, &["id", "category", "target", "direction", "magnitude", "dimension", "provenance"], &[])
                && keys_within(load.and_then(|l| l.get("target")), &["type", "pipe"], &["type", "pipe"])
                && load.and_then(|l| l.pointer("/target/type")) == Some(&Value::String("element".into()))
                && matches!(category, Some("distributed_force" | "thermal"));
            if !exact {
                blocks.push(unsupported(model, ends, pipe, &segs, true));
            } else if category == Some("distributed_force") {
                let load_id = load.and_then(|l| l.get("id")).and_then(Value::as_str).unwrap_or("primitive_load:unknown");
                warnings.push(warning(
                    "OP-RECONNECT-MEMBER-LOAD-LENGTH-CHANGED",
                    format!("Distributed load {load_id} on {pipe} is kept per unit length; its resultant follows the new length of {pipe}."),
                    "Review the load's magnitude for the new span.",
                    vec![pipe.to_string(), load_id.to_string(), case_id()],
                ));
            }
        } else if matches(&segs, &["load_cases", "*", "equivalent_static", "wind", "exposed_pipe_refs", "*"])
            || (matches(&segs, &["load_cases", "*", "equivalent_static", "wind", "exposed_spans", "*", "pipe_ref"])
                && keys_within(at(model, &segs[..6]), &["pipe_ref", "start_fraction", "end_fraction"], &["pipe_ref"]))
        {
            let case = case_id();
            warnings.push(warning(
                "OP-RECONNECT-WIND-EXPOSURE-REMAPPED",
                format!("Wind exposure of {pipe} in load case {case} is kept at {path}; the exposed length now follows the new span."),
                "Review the marked wind exposure for the new span.",
                vec![pipe.to_string(), case],
            ));
        } else if matches(&segs, &["load_cases", "*", "equivalent_static", "wind", "exposed_spans", "*", "pipe_ref"]) {
            blocks.push(unsupported(model, ends, pipe, &segs, true));
        } else if matches(&segs, &["load_cases", "*", "analysis_state", "element_states", "*", "pipe_ref"]) {
            // Kept with no diagnostic (§2.2).
        } else if matches(&segs, &["reference_configurations", "*", "member_references", "*", "pipe_ref"]) {
            let member = at(model, &segs[..4]);
            if !keys_within(member, &["pipe_ref", "basis", "fit", "provenance"], &["pipe_ref"]) {
                blocks.push(unsupported(model, ends, pipe, &segs, true));
                continue;
            }
            let fit = member.and_then(|m| m.pointer("/fit/kind")).and_then(Value::as_str);
            if fit.is_some_and(|kind| kind != "none") || member.and_then(|m| m.get("fit")).is_some_and(|f| f.get("kind").is_none()) {
                let configuration = at(model, &segs[..2]).and_then(|c| c.get("id")).and_then(Value::as_str).unwrap_or("reference:unknown").to_string();
                warnings.push(warning(
                    "OP-RECONNECT-MEMBER-FIT-REVIEW",
                    format!("Member reference of {pipe} in {configuration} declares a fit ({}); it is kept, and its effect is now taken over the new length of {pipe}.", fit.unwrap_or("unknown")),
                    "Review the declared fit for the new span.",
                    vec![pipe.to_string(), configuration],
                ));
            }
        } else {
            blocks.push(unsupported(model, ends, pipe, &segs, false));
        }
    }
}

/// §2.2, node ids F, O, T: the allowed slots; any other refuses (R12).
/// Components are R9's.
fn node_slots(model: &Value, ends: &Ends, blocks: &mut Blocks, warnings: &mut Vec<ReconnectWarning>) {
    let pipe = ends.pipe;
    let node_ids = ends.nodes();
    for segs in scan(model, &node_ids, Some(pipe), &node_ids) {
        // Only R9's array records are exempt; any other `components` value is scanned.
        if matches!(segs.as_slice(), [Seg::Key(k), Seg::Index(_), ..] if k == "components") {
            continue;
        }
        let node = at(model, &segs).and_then(Value::as_str).unwrap_or("").to_string();
        if matches(&segs, &["pipe_segments", "*", "from"])
            || matches(&segs, &["pipe_segments", "*", "to"])
            || matches(&segs, &["supports", "*", "node"])
            || matches(&segs, &["load_cases", "*", "primitive_loads", "*", "target", "node"])
        {
            continue;
        }
        if matches(&segs, &["load_cases", "*", "pressure_regions", "*", "terminals", "*", "node_ref"]) {
            if node == ends.old || node == ends.target {
                let case = at(model, &segs[..2]).and_then(|c| c.get("id")).and_then(Value::as_str).unwrap_or("load_case:unknown").to_string();
                let region = at(model, &segs[..4]).and_then(|r| r.get("id")).and_then(Value::as_str).unwrap_or("region:unknown").to_string();
                let verb = if node == ends.old { "leaves" } else { "joins" };
                warnings.push(warning(
                    "OP-RECONNECT-PRESSURE-TERMINAL-REVIEW",
                    format!("{pipe}, outside pressure region {region} (load case {case}), {verb} its terminal {node}, whose declared closure_transfer is kept."),
                    "Review the terminal's closure transfer for the changed connection.",
                    vec![pipe.to_string(), region, case, node],
                ));
            }
            continue;
        }
        blocks.push(unsupported(model, ends, &node, &segs, false));
    }
}

fn pipe_rows(model: &Value) -> impl Iterator<Item = (&str, &str, &str)> {
    model.get("pipe_segments").and_then(Value::as_array).into_iter().flatten().filter_map(|p| {
        Some((p.get("id")?.as_str()?, p.get("from")?.as_str()?, p.get("to")?.as_str()?))
    })
}
/// Connected parts of the pipe graph (vertices: nodes that end a pipe).
pub(crate) fn pipe_graph_parts(model: &Value) -> usize {
    let mut parent: BTreeMap<String, String> = BTreeMap::new();
    fn root(parent: &mut BTreeMap<String, String>, node: &str) -> String {
        let mut current = node.to_string();
        while let Some(next) = parent.get(&current).filter(|n| **n != current).cloned() {
            current = next;
        }
        current
    }
    for (_, from, to) in pipe_rows(model) {
        for node in [from, to] {
            parent.entry(node.to_string()).or_insert_with(|| node.to_string());
        }
        let (a, b) = (root(&mut parent, from), root(&mut parent, to));
        if a != b {
            parent.insert(a, b);
        }
    }
    let nodes: Vec<String> = parent.keys().cloned().collect();
    nodes.iter().map(|n| root(&mut parent, n)).collect::<BTreeSet<_>>().len()
}

/// §2.4 topology and geometry warnings, on the reconnected candidate.
fn topology_warnings(model: &Value, candidate: &Value, ends: &Ends, warnings: &mut Vec<ReconnectWarning>) -> Result<(), GeometryError> {
    let (pipe, fixed, old, target) = (ends.pipe, ends.fixed.as_str(), ends.old.as_str(), ends.target.as_str());
    let pair = BTreeSet::from([fixed, target]);
    let (fixed_at, target_at) = (position(candidate, fixed)?, position(candidate, target)?);
    let span = norm(sub(target_at, fixed_at));
    let tolerance = COINCIDENT_RELATIVE * span;
    for (id, from, to) in pipe_rows(candidate) {
        if id == pipe {
            continue;
        }
        if BTreeSet::from([from, to]) == pair {
            warnings.push(warning(
                "OP-RECONNECT-PARALLEL-PIPE",
                format!("{id} already joins {fixed} and {target}; after this reconnect {pipe} runs beside it as a separate element (stiffness and weight add)."),
                "Confirm the parallel member is intended; two members in one pressure region are refused at solve.",
                vec![pipe.to_string(), id.to_string(), fixed.to_string(), target.to_string()],
            ));
            continue;
        }
        let (Ok(a), Ok(b)) = (position(candidate, from), position(candidate, to)) else {
            continue;
        };
        let near = |p: Vec3, q: Vec3| norm(sub(p, q)) <= tolerance;
        if (near(a, fixed_at) && near(b, target_at)) || (near(a, target_at) && near(b, fixed_at)) {
            warnings.push(warning(
                "OP-RECONNECT-COINCIDENT-SPAN",
                format!("{id} ({from} to {to}) spans the same positions as {pipe} ({fixed} to {target}) within {COINCIDENT_RELATIVE:e} of the span length, through different nodes."),
                "Confirm the coincident member is intended, or merge the nodes.",
                vec![pipe.to_string(), id.to_string(), from.to_string(), to.to_string()],
            ));
        }
    }
    if !pipe_rows(candidate).any(|(_, from, to)| from == old || to == old) {
        let mut attached: Vec<String> = Vec::new();
        for (key, field) in [("supports", "node"), ("components", "node")] {
            for row in candidate.get(key).and_then(Value::as_array).into_iter().flatten() {
                if row.get(field).and_then(Value::as_str) == Some(old) {
                    attached.push(row.get("id").and_then(Value::as_str).unwrap_or("unknown").to_string());
                }
            }
        }
        for case in candidate.get("load_cases").and_then(Value::as_array).into_iter().flatten() {
            for load in case.get("primitive_loads").and_then(Value::as_array).into_iter().flatten() {
                if load.pointer("/target/node").and_then(Value::as_str) == Some(old) {
                    attached.push(load.get("id").and_then(Value::as_str).unwrap_or("unknown").to_string());
                }
            }
            for region in case.get("pressure_regions").and_then(Value::as_array).into_iter().flatten() {
                if region.get("terminals").and_then(Value::as_array).into_iter().flatten().any(|t| t.get("node_ref").and_then(Value::as_str) == Some(old)) {
                    attached.push(region.get("id").and_then(Value::as_str).unwrap_or("unknown").to_string());
                }
            }
        }
        let listed = if attached.is_empty() { "none".to_string() } else { attached.join(", ") };
        let mut refs = vec![pipe.to_string(), old.to_string()];
        refs.extend(attached);
        warnings.push(warning(
            "OP-RECONNECT-NODE-ORPHANED",
            format!("Node {old} has no pipe after this reconnect; it and its attachments (listed: {listed}) are kept. If it has no restraint either, the solve refuses the case as NUMERICAL_INTEGRITY_UNRESOLVED and publishes no result."),
            format!("Delete {old} in the same batch, or connect it again, before solving."),
            refs,
        ));
    }
    let (before, after) = (pipe_graph_parts(model), pipe_graph_parts(candidate));
    if after > before {
        warnings.push(warning(
            "OP-RECONNECT-MODEL-SPLIT",
            format!("After this reconnect the pipe graph has {after} connected parts (before: {before})."),
            "Confirm each part is restrained, or reconnect the parts before solving.",
            vec![pipe.to_string(), old.to_string()],
        ));
    }
    let degree = pipe_rows(candidate).filter(|(_, from, to)| *from == target || *to == target).count();
    if degree >= 3 {
        let markers: Vec<String> = candidate
            .get("components")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter(|c| c.get("node").and_then(Value::as_str) == Some(target) && component_kind(c) == "terminal")
            .map(|c| c.get("id").and_then(Value::as_str).unwrap_or("unknown").to_string())
            .collect();
        let marker = if markers.is_empty() { String::new() } else { format!(" A terminal marker ({}) sits there.", markers.join(", ")) };
        let mut refs = vec![pipe.to_string(), target.to_string()];
        refs.extend(markers);
        warnings.push(warning(
            "OP-RECONNECT-JUNCTION-WITHOUT-BRANCH-COMPONENT",
            format!("{target} now joins {degree} pipes with no branch component there.{marker}"),
            "Author a tee or branch component at the junction if one is intended.",
            refs,
        ));
    }
    for node in [fixed, target] {
        for (span, theta) in crate::bend_authoring::realized_arc_junctions(candidate, pipe, node) {
            if theta > TANGENCY_TOLERANCE_RAD {
                warnings.push(warning(
                    "OP-RECONNECT-BEND-JUNCTION-NOT-TANGENT",
                    format!("{pipe} meets the realized bend arc {span} at {node} with a direction change of {theta:e} rad, above alpha_tan = {TANGENCY_TOLERANCE_RAD:e} rad; inside a pressure region that kink is a mitre the solve refuses."),
                    "Make the junction tangent, or end any pressure region at that node.",
                    vec![pipe.to_string(), span, node.to_string()],
                ));
            }
        }
    }
    Ok(())
}
