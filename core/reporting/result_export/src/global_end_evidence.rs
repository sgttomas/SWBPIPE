//! W-7 (DEL-04-01 Design `global-end-actions.md`): the reader checks of the
//! member end actions in global coordinates (`element_global_end_force_v1`,
//! `element_global_end_moment_v1`), shared by the pressure-2 reader
//! (`physics_evidence`) and the preview-physics-2 reader
//! (`preview_physics_evidence`).
//!
//! - Each load-case row: kind, unit, component, coordinate system `global`,
//!   location `end_i` or `end_j`, basis (the wall basis only where the member's
//!   wall end action is published at that end), sign convention and no
//!   derivation references. A combination row keeps the kind, unit, component,
//!   frame and location and carries its combination's basis.
//! - Coverage: per load case, exactly one row per member, end and component for
//!   every member that publishes element-local end rows, and none for any other
//!   entity. A document with no global end row at all is a publication made
//!   before W-7 under the same contract id (the kinds were added, nothing was
//!   redefined) and stays readable; one global end row requires them all.
//! - Rotation invariance: at each end the global vector's norm equals the norm
//!   of the published local vector it rotates (the wall action as axial where
//!   the wall basis is named), to [`NORM_TOLERANCE`] relative. A rotation
//!   preserves the norm; this checks the rows against their local operands
//!   without the geometry. It is an internal-consistency check, not an accuracy
//!   claim: it does not establish the vector's direction (a permutation of the
//!   components keeps the norm).
use open_pipe_stress_correct_norm::norm3;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, HashSet};

/// The frame kernel admits local axes (`FrameOrientation::new`) that are unit
/// length and mutually orthogonal each to within `AXIS_TOLERANCE` = 1e-12
/// (`frame_kernel/src/lib.rs`), so R Rᵀ = I + E with |E_ii| ≤ 2t + t² and
/// |E_ij| ≤ t, and ‖E‖₂ ≤ ‖E‖_F ≤ √18·t < 5t. Since ‖Rᵀf‖² = fᵀ R Rᵀ f, a
/// rotated vector's norm differs from the operand's by at most ‖E‖·‖f‖. The
/// bound adds 64 epsilon for the rotation's and the norms' rounding.
/// (Independent W-7 review: an admitted near-parallel `y_reference` gives axes
/// 2.46e-13 from orthogonal and a norm drift of 1.23e-13.)
const ADMITTED_AXIS_TOLERANCE: f64 = 1.0e-12;
pub(crate) const NORM_TOLERANCE: f64 = 5.0 * ADMITTED_AXIS_TOLERANCE + 64.0 * f64::EPSILON;

type Check = Result<(), String>;

pub(crate) const FORCE: &str = "element_global_end_force_v1";
pub(crate) const MOMENT: &str = "element_global_end_moment_v1";
pub(crate) const BASIS: &str = "rotated_from_published_element_local_end_actions_v1";
pub(crate) const WALL_BASIS: &str =
    "rotated_from_published_element_local_end_actions_v1;axial_is_pipe_wall_action";
pub(crate) const SIGN: &str = "global end action on the element at its node (node on element), F = R\u{1d40} f; the element acts on its node with \u{2212}F";
const COMBINATION_BASES: [&str; 3] = [
    "explicit_user_linear_combination",
    "explicit_user_result_state_subtraction",
    "explicit_user_range_envelope",
];
const FORCE_COMPONENTS: [&str; 3] = ["Fx", "Fy", "Fz"];
const MOMENT_COMPONENTS: [&str; 3] = ["Mx", "My", "Mz"];
const LOCAL_KINDS: [&str; 6] = [
    "element_local_axial_force",
    "element_local_shear_force_y",
    "element_local_shear_force_z",
    "element_local_torsional_moment",
    "element_local_bending_moment_y",
    "element_local_bending_moment_z",
];
const WALL_KIND: &str = "pipe_wall_endpoint_action_v2";
const ENDS: [&str; 2] = ["end_i", "end_j"];

fn require(ok: bool, code: &str) -> Check {
    if ok {
        Ok(())
    } else {
        Err(format!("SOURCE_GLOBAL_END_{code}"))
    }
}

pub(crate) fn is_kind(kind: &str) -> bool {
    kind == FORCE || kind == MOMENT
}

fn case_of(row: &Value) -> Option<&str> {
    let b = &row["basis_ref"];
    (b["ref_type"] == "load_case").then(|| b["ref_id"].as_str()).flatten()
}

fn end_index(location: &Value) -> Option<usize> {
    ENDS.iter().position(|end| location == *end)
}

/// One global end row's declared semantics (`row["kind"]` is one of the two kinds).
pub(crate) fn row_semantics(row: &Value, wall_admitted: bool) -> Check {
    let kind = row["kind"].as_str().unwrap_or_default();
    let (unit, components) = match kind {
        FORCE => ("N", FORCE_COMPONENTS),
        MOMENT => ("N*m", MOMENT_COMPONENTS),
        _ => return require(false, "KIND"),
    };
    require(row["unit"] == unit, "UNIT")?;
    require(row["value"].as_f64().is_some_and(f64::is_finite), "VALUE")?;
    let md = &row["metadata"];
    require(
        md.as_object().is_some_and(|m| {
            m.len() == 5
                && ["component", "coordinate_system", "location", "basis", "sign_convention"]
                    .iter()
                    .all(|k| m.contains_key(*k))
        }),
        "METADATA_SHAPE",
    )?;
    require(md["component"].as_str().is_some_and(|c| components.contains(&c)), "COMPONENT")?;
    require(md["coordinate_system"] == "global", "COORDINATE_SYSTEM")?;
    require(end_index(&md["location"]).is_some(), "LOCATION")?;
    if case_of(row).is_some() {
        require(
            md["basis"] == BASIS || (wall_admitted && md["basis"] == WALL_BASIS),
            "BASIS",
        )?;
        require(md["sign_convention"] == SIGN, "SIGN_CONVENTION")?;
        require(
            row.get("source_result_refs").is_none_or(|r| r.as_array().is_some_and(Vec::is_empty)),
            "DERIVATION",
        )?;
    } else {
        require(
            md["basis"].as_str().is_some_and(|b| COMBINATION_BASES.contains(&b)),
            "BASIS",
        )?;
    }
    Ok(())
}

type Key<'a> = (&'a str, &'a str);

/// Coverage, the wall basis and rotation invariance over every load-case row.
pub(crate) fn coverage(rows: &[Value]) -> Check {
    // (case, member) -> [end][slot 0..6] local value; wall actions per end.
    let mut local: BTreeMap<Key<'_>, [[Option<f64>; 6]; 2]> = BTreeMap::new();
    let mut wall: BTreeMap<Key<'_>, [Option<f64>; 2]> = BTreeMap::new();
    let mut global: BTreeMap<Key<'_>, [[Option<f64>; 6]; 2]> = BTreeMap::new();
    let mut wall_basis: BTreeMap<Key<'_>, [BTreeSet<bool>; 2]> = BTreeMap::new();
    for row in rows {
        let Some(case) = case_of(row) else { continue };
        let (Some(kind), Some(entity)) = (row["kind"].as_str(), row["entity_ref"].as_str()) else {
            continue;
        };
        let md = &row["metadata"];
        let Some(end) = end_index(&md["location"]) else { continue };
        let value = row["value"].as_f64();
        if let Some(slot) = LOCAL_KINDS.iter().position(|k| *k == kind) {
            if md["coordinate_system"] == "element_local" {
                local.entry((case, entity)).or_default()[end][slot] = value;
            }
        } else if kind == WALL_KIND {
            wall.entry((case, entity)).or_default()[end] = value;
        } else if is_kind(kind) {
            let components = if kind == FORCE { FORCE_COMPONENTS } else { MOMENT_COMPONENTS };
            let slot = components
                .iter()
                .position(|c| md["component"] == *c)
                .ok_or("SOURCE_GLOBAL_END_COMPONENT")?
                + if kind == FORCE { 0 } else { 3 };
            let cell = &mut global.entry((case, entity)).or_default()[end][slot];
            require(cell.is_none(), "SLOT_DUPLICATE")?;
            *cell = value;
            wall_basis.entry((case, entity)).or_default()[end].insert(md["basis"] == WALL_BASIS);
        }
    }
    // A publication made before W-7 carries none (the contract id is unchanged).
    if global.is_empty() {
        return Ok(());
    }
    // Members publishing element-local end rows: the shear rows, which no
    // route removes (a pressure member's axial rows are replaced by its wall).
    let members: HashSet<Key<'_>> = local
        .iter()
        .filter(|(_, ends)| ends.iter().all(|e| e[1].is_some() && e[2].is_some()))
        .map(|(key, _)| *key)
        .collect();
    for key in global.keys() {
        require(members.contains(key), "MEMBER_UNRESOLVED")?;
    }
    for key in &members {
        let ends = global.get(key).ok_or("SOURCE_GLOBAL_END_COVERAGE")?;
        let locals = &local[key];
        let walls = wall.get(key).copied().unwrap_or([None; 2]);
        for end in 0..2 {
            require(ends[end].iter().all(Option::is_some), "COVERAGE")?;
            let is_wall = walls[end].is_some();
            require(wall_basis[key][end] == BTreeSet::from([is_wall]), "BASIS")?;
            let mut f = [0.0; 6];
            for slot in 0..6 {
                f[slot] = if slot == 0 && is_wall {
                    require(locals[end][0].is_none(), "AXIAL_OPERAND")?;
                    walls[end].ok_or("SOURCE_GLOBAL_END_AXIAL_OPERAND")?
                } else {
                    locals[end][slot].ok_or("SOURCE_GLOBAL_END_LOCAL_OPERAND")?
                };
            }
            let g = ends[end].map(|v| v.unwrap_or(f64::NAN));
            for block in 0..2 {
                let (a, b) = (
                    norm3(f[3 * block], f[3 * block + 1], f[3 * block + 2]),
                    norm3(g[3 * block], g[3 * block + 1], g[3 * block + 2]),
                );
                require(
                    (a - b).abs() <= NORM_TOLERANCE * a.max(f64::MIN_POSITIVE),
                    "ROTATION_NORM",
                )?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn local(case: &str, member: &str, kind: &str, end: &str, value: f64) -> Value {
        json!({"id": format!("{case}:{member}:{kind}:{end}"), "kind": kind, "value": value, "unit": "N", "entity_ref": member,
            "basis_ref": {"ref_type": "load_case", "ref_id": case},
            "metadata": {"component": "c", "coordinate_system": "element_local", "location": end, "basis": "recovered_from_local_element_stiffness", "sign_convention": "s"}})
    }
    fn global(case: &str, member: &str, end: &str, name: &str, value: f64, basis: &str) -> Value {
        let force = name.starts_with('F');
        json!({"id": format!("{case}:{member}:{end}:{name}"), "kind": if force { FORCE } else { MOMENT }, "value": value,
            "unit": if force { "N" } else { "N*m" }, "entity_ref": member, "basis_ref": {"ref_type": "load_case", "ref_id": case},
            "source_result_refs": [],
            "metadata": {"component": name, "coordinate_system": "global", "location": end, "basis": basis, "sign_convention": SIGN}})
    }
    /// One member along global y (local x = +y, y = -x, z = +z): f = (3, 4, 12), m = (1, 2, 2) at each end.
    fn member(wall: bool) -> Vec<Value> {
        let mut rows = Vec::new();
        for end in ENDS {
            for (slot, kind) in LOCAL_KINDS.iter().enumerate() {
                if slot == 0 && wall {
                    continue;
                }
                rows.push(local("c", "pipe:a", kind, end, [3.0, 4.0, 12.0, 1.0, 2.0, 2.0][slot]));
            }
            if wall {
                rows.push(local("c", "pipe:a", WALL_KIND, end, 3.0));
            }
            let basis = if wall { WALL_BASIS } else { BASIS };
            // F = R^T f with rows x=(0,1,0), y=(-1,0,0), z=(0,0,1): (-4, 3, 12), (-2, 1, 2).
            for (name, value) in [("Fx", -4.0), ("Fy", 3.0), ("Fz", 12.0), ("Mx", -2.0), ("My", 1.0), ("Mz", 2.0)] {
                rows.push(global("c", "pipe:a", end, name, value, basis));
            }
        }
        rows
    }
    fn refused(rows: &[Value], code: &str) {
        assert_eq!(coverage(rows).unwrap_err(), format!("SOURCE_GLOBAL_END_{code}"));
    }

    /// The norm bound is the kernel's admitted frame error plus rounding: a
    /// global force block whose norm is off by half the bound is read, and by
    /// twice the bound is refused (|f| = 13 here).
    #[test]
    fn the_rotation_norm_bound_is_the_admitted_frame_error() {
        let scaled = |factor: f64| {
            let mut rows = member(false);
            for row in rows.iter_mut().filter(|r| r["kind"] == FORCE) {
                let value = row["value"].as_f64().unwrap();
                row["value"] = json!(value * factor);
            }
            rows
        };
        coverage(&scaled(1.0 + 0.5 * NORM_TOLERANCE)).unwrap();
        refused(&scaled(1.0 + 2.0 * NORM_TOLERANCE), "ROTATION_NORM");
        assert!(NORM_TOLERANCE > 4.25e-12 && NORM_TOLERANCE < 5.1e-12);
    }

    #[test]
    fn well_formed_rows_pass_on_both_bases() {
        for wall in [false, true] {
            let rows = member(wall);
            coverage(&rows).unwrap();
            for row in rows.iter().filter(|r| is_kind(r["kind"].as_str().unwrap())) {
                row_semantics(row, true).unwrap();
            }
        }
    }

    #[test]
    fn malformed_rows_are_refused_by_name() {
        let base = member(false)
            .into_iter()
            .find(|r| r["kind"] == FORCE)
            .unwrap();
        let mutate = |f: &dyn Fn(&mut Value)| {
            let mut row = base.clone();
            f(&mut row);
            row_semantics(&row, false).unwrap_err()
        };
        let code = |c: &str| format!("SOURCE_GLOBAL_END_{c}");
        assert_eq!(mutate(&|r| r["unit"] = json!("kN")), code("UNIT"));
        assert_eq!(mutate(&|r| r["kind"] = json!("element_global_end_action_v1")), code("KIND"));
        assert_eq!(mutate(&|r| r["metadata"]["component"] = json!("Mx")), code("COMPONENT"));
        assert_eq!(mutate(&|r| r["metadata"]["coordinate_system"] = json!("element_local")), code("COORDINATE_SYSTEM"));
        assert_eq!(mutate(&|r| r["metadata"]["location"] = json!("midspan")), code("LOCATION"));
        assert_eq!(mutate(&|r| r["metadata"]["basis"] = json!(WALL_BASIS)), code("BASIS"));
        assert_eq!(mutate(&|r| r["metadata"]["basis"] = json!("recovered_from_local_element_stiffness")), code("BASIS"));
        assert_eq!(mutate(&|r| r["metadata"]["sign_convention"] = json!("node on element")), code("SIGN_CONVENTION"));
        assert_eq!(mutate(&|r| { r["metadata"].as_object_mut().unwrap().insert("extra".into(), json!(1)); }), code("METADATA_SHAPE"));
        assert_eq!(mutate(&|r| r["source_result_refs"] = json!(["result:x"])), code("DERIVATION"));
        assert_eq!(mutate(&|r| r["value"] = json!(null)), code("VALUE"));
        // A combination row carries its combination's basis, never the case basis.
        let mut combined = base.clone();
        combined["basis_ref"] = json!({"ref_type": "combination", "ref_id": "k"});
        assert_eq!(row_semantics(&combined, false).unwrap_err(), code("BASIS"));
        combined["metadata"]["basis"] = json!("explicit_user_linear_combination");
        row_semantics(&combined, false).unwrap();
    }

    #[test]
    fn coverage_and_rotation_are_refused_by_name() {
        let rows = member(false);
        // A missing row.
        let missing: Vec<Value> = rows.iter().filter(|r| !(r["kind"] == MOMENT && r["metadata"]["location"] == "end_j" && r["metadata"]["component"] == "My")).cloned().collect();
        refused(&missing, "COVERAGE");
        // A document without any global row is a pre-W-7 publication and is read; a
        // member without its rows beside another member's is refused.
        let none: Vec<Value> = rows.iter().filter(|r| !is_kind(r["kind"].as_str().unwrap())).cloned().collect();
        coverage(&none).unwrap();
        let mut partial = rows.clone();
        partial.extend(member(false).into_iter().filter(|r| !is_kind(r["kind"].as_str().unwrap())).map(|mut r| {
            r["entity_ref"] = json!("pipe:b");
            r["id"] = json!(format!("{}:b", r["id"].as_str().unwrap()));
            r
        }));
        refused(&partial, "COVERAGE");
        // A duplicate row.
        let mut duplicate = rows.clone();
        duplicate.push(rows.iter().find(|r| r["kind"] == FORCE).unwrap().clone());
        refused(&duplicate, "SLOT_DUPLICATE");
        // A row on an entity with no local end rows.
        let mut stray = rows.clone();
        stray.push(global("c", "pipe:b", "end_i", "Fx", 0.0, BASIS));
        refused(&stray, "MEMBER_UNRESOLVED");
        // A value that is not a rotation of the local vector.
        let mut scaled = rows.clone();
        scaled.iter_mut().find(|r| r["kind"] == FORCE && r["metadata"]["component"] == "Fz").unwrap()["value"] = json!(12.5);
        refused(&scaled, "ROTATION_NORM");
        // The wall basis without a published wall action, and the plain basis with one.
        let mut wrong = rows.clone();
        wrong.iter_mut().filter(|r| is_kind(r["kind"].as_str().unwrap())).for_each(|r| r["metadata"]["basis"] = json!(WALL_BASIS));
        refused(&wrong, "BASIS");
        let mut plain = member(true);
        plain.iter_mut().filter(|r| is_kind(r["kind"].as_str().unwrap())).for_each(|r| r["metadata"]["basis"] = json!(BASIS));
        refused(&plain, "BASIS");
        // The wall action replaces the local axial: both present is refused.
        let mut both = member(true);
        both.push(local("c", "pipe:a", "element_local_axial_force", "end_i", 3.0));
        refused(&both, "AXIAL_OPERAND");
    }
}
