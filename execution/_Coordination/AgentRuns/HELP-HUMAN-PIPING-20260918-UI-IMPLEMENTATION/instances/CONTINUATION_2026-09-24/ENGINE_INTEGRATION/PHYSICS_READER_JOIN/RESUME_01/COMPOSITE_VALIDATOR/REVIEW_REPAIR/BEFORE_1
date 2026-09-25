//! Explicit composite validation of untouched physical and retained-source records.
//! Hash/recipe consistency is not custody; numerical use needs captured invocation.
use crate::source_blocks::{domain_hash, shape_in};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

pub const CONTRACT_ID: &str = "openpipestress.result_semantics/0.3.0/physics-source-1";
pub const PROFILE: &str = "exact_straight_pressure_v2";
pub const EXACT: &str = "retained_source_blocks_exact_v1";
pub const MAX_BASIS: &str = "retained_source_endpoint_normal_max_v1";
pub const MAX_SIGN: &str = "nonnegative maximum absolute axial-plus-bending normal stress over an unloaded circular straight span; retained endpoint actions with projected-action and arithmetic bounds; endpoint witness does not imply uniqueness; torsional shear separate";
pub const SUPPORT_SIGN: &str = "support-on-pipe; positive global force and right-hand couple about attached node; force and moment norms remain separate";
fn require(ok: bool, detail: &str) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(format!("PHYSICS_SOURCE_{detail}"))
    }
}
fn array(v: &Value) -> Result<&Vec<Value>, String> {
    v.as_array().ok_or("PHYSICS_SOURCE_ARRAY".into())
}
fn text(v: &Value) -> Result<&str, String> {
    v.as_str()
        .filter(|s| !s.is_empty())
        .ok_or("PHYSICS_SOURCE_TEXT".into())
}
fn num(v: &Value) -> Result<f64, String> {
    v.as_f64()
        .filter(|v| v.is_finite())
        .ok_or("PHYSICS_SOURCE_NUMBER".into())
}
fn same(a: &Value, b: f64) -> Result<bool, String> {
    Ok(num(a)?.to_bits() == b.to_bits())
}
fn schema() -> &'static Value {
    static SCHEMA: OnceLock<Value> = OnceLock::new();
    SCHEMA.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../../../../schemas/physics_source_recovery.schema.json"
        ))
        .expect("closed composite receipt schema")
    })
}
pub fn validate_receipt_shape(receipt: &Value) -> Result<(), String> {
    require(shape_in(receipt, schema(), schema()), "RECEIPT_SHAPE")
}
fn case<'a>(source: &'a Value, cid: &str) -> Result<&'a Value, String> {
    let found = array(&source["source_block_recovery"]["body"]["cases"])?
        .iter()
        .filter(|c| c["basis_ref"]["ref_id"] == cid)
        .collect::<Vec<_>>();
    require(found.len() == 1, "CASE_ID")?;
    Ok(found[0])
}
/// Transport statements only: no raw publication hash, invocation, or Current claim.
pub fn validate_transport_metadata(source: &Value) -> Result<(), String> {
    let receipt = &source["source_block_recovery"];
    validate_receipt_shape(receipt)?;
    let body = &receipt["body"];
    require(
        receipt["receipt_sha256"] == domain_hash("source_blocks_receipt_v1", body)?,
        "RECEIPT_HASH",
    )?;
    let evidence = &source["contract_evidence"];
    require(
        shape_in(evidence, &schema()["$defs"]["physical_evidence"], schema()),
        "PHYSICAL_SHAPE",
    )?;
    let physical = array(&evidence["exact_cases"])?;
    let records = array(&body["cases"])?;
    require(
        physical
            .iter()
            .map(|c| &c["load_case_id"])
            .eq(records.iter().map(|c| &c["basis_ref"]["ref_id"])),
        "CASE_ORDER",
    )?;
    let ids = physical
        .iter()
        .map(|c| text(&c["load_case_id"]))
        .collect::<Result<HashSet<_>, _>>()?;
    require(ids.len() == physical.len(), "CASE_UNIQUENESS")?;
    require(
        records.iter().any(|c| c["selected_method"] == EXACT),
        "SOURCE_SELECTION_REQUIRED",
    )?;
    for (record, exact) in records.iter().zip(physical) {
        let cid = text(&exact["load_case_id"])?;
        let pressure = array(&evidence["pressure"])?
            .iter()
            .filter(|p| p["load_case_id"] == cid)
            .collect::<Vec<_>>();
        require(
            exact["recovery_method"] == record["selected_method"],
            "RECOVERY_METHOD",
        )?;
        require(
            record["physical_evidence_sha256"]
                == domain_hash(
                    "physics_source_case_evidence_v1",
                    &json!({"exact_case":exact,"pressure":pressure}),
                )?,
            "PHYSICAL_CASE_HASH",
        )?;
        if record["selected_method"] == EXACT {
            require(pressure.is_empty(), "SOURCE_PRESSURE_INVENTORY")?;
            let plan = &record["source"];
            for key in ["pipe_sections", "pipe_materials"] {
                require(
                    array(&exact[key])?
                        .iter()
                        .map(|p| &p["pipe_id"])
                        .eq(array(&plan["member_ids"])?.iter()),
                    "PHYSICAL_MEMBER_ORDER",
                )?;
            }
            require(
                array(&plan["endpoint_sections"])?
                    .iter()
                    .map(|p| &p["pipe_id"])
                    .eq(array(&plan["member_ids"])?.iter()),
                "ENDPOINT_MEMBER_ORDER",
            )?;
            require(
                exact["stress_maximum_coverage"]
                    == json!({"complete":true,"unavailable_pipe_ids":[]}),
                "MAXIMUM_COMPLETE",
            )?;
            let maxima = array(&exact["pipe_stress_extrema"])?;
            let members = array(&plan["member_ids"])?;
            require(
                maxima.len() == members.len()
                    && members
                        .iter()
                        .all(|m| maxima.iter().filter(|x| x["pipe_id"] == *m).count() == 1),
                "MAXIMUM_COVERAGE",
            )?;
            section_functionals(record)?;
            for ex in maxima {
                maximum_link(exact, ex, record)?;
            }
        } else {
            require(
                array(&record["derived_checks"])?.is_empty()
                    && array(&record["section_stress_checks"])?.is_empty()
                    && record["selected_method"] == ordinary_method(&record["requested_mode"])?,
                "ORDINARY_METHOD",
            )?;
        }
    }
    require(
        array(&evidence["pressure"])?.iter().all(|p| {
            p["load_case_id"]
                .as_str()
                .is_some_and(|id| ids.contains(id))
        }),
        "PRESSURE_CASE",
    )
}
fn ordinary_method(mode: &Value) -> Result<&'static str, String> {
    match mode.as_str() {
        Some("sparse_interactive") => Ok("ordinary_sparse_structural_v1"),
        Some("dense_scrutiny") => Ok("ordinary_dense_structural_v1"),
        _ => Err("PHYSICS_SOURCE_MODE".into()),
    }
}
fn finite(v: f64) -> Result<f64, String> {
    if v.is_finite() {
        Ok(v)
    } else {
        Err("PHYSICS_SOURCE_ARITHMETIC_RANGE".into())
    }
}
fn out(v: f64, upper: bool) -> Result<f64, String> {
    require(v.is_finite() && v >= 0.0, "ARITHMETIC_RANGE")?;
    finite(if upper {
        v.next_up()
    } else {
        v.next_down().max(0.0)
    })
}
fn div(a: f64, b: f64, upper: bool) -> Result<f64, String> {
    if a == 0.0 || b == 1.0 {
        Ok(a)
    } else if a == b {
        Ok(1.0)
    } else {
        out(a / b, upper)
    }
}
fn mul(a: f64, b: f64, upper: bool) -> Result<f64, String> {
    if a == 0.0 || b == 0.0 {
        Ok(0.0)
    } else if a == 1.0 {
        Ok(b)
    } else if b == 1.0 {
        Ok(a)
    } else {
        out(a * b, upper)
    }
}
fn add(a: f64, b: f64, upper: bool) -> Result<f64, String> {
    if a == 0.0 {
        Ok(b)
    } else if b == 0.0 {
        Ok(a)
    } else {
        out(a + b, upper)
    }
}
fn interval(v: &Value) -> Result<[f64; 2], String> {
    let v = array(v)?;
    require(v.len() == 2, "INTERVAL_SHAPE")?;
    let p = [num(&v[0])?, num(&v[1])?];
    require(p[0] <= p[1], "INTERVAL_ORDER")?;
    Ok(p)
}
fn absolute([lo, hi]: [f64; 2]) -> [f64; 2] {
    if lo >= 0.0 {
        [lo, hi]
    } else if hi <= 0.0 {
        [-hi, -lo]
    } else {
        [0.0, (-lo).max(hi)]
    }
}
fn norm(values: &[f64]) -> Result<f64, String> {
    let scale = values.iter().map(|v| v.abs()).fold(0.0_f64, f64::max);
    if scale == 0.0 {
        return Ok(0.0);
    }
    let squares = values
        .iter()
        .map(|v| {
            let r = v / scale;
            r * r
        })
        .collect::<Vec<_>>();
    let mut total = squares[0] + squares[1];
    if squares.len() == 3 {
        total += squares[2];
    }
    finite(scale * total.sqrt())
}
fn norm_bound(values: [f64; 3], upper: bool) -> Result<f64, String> {
    let scale = values.into_iter().fold(0.0_f64, f64::max);
    if scale == 0.0 {
        return Ok(0.0);
    }
    let mut square = [0.0; 3];
    for (i, v) in values.into_iter().enumerate() {
        square[i] = if v == 0.0 {
            0.0
        } else if v == scale {
            1.0
        } else {
            let r = out(v / scale, upper)?;
            out(r * r, upper)?
        };
    }
    let sum = add(add(square[0], square[1], upper)?, square[2], upper)?;
    let root = if sum == 1.0 {
        1.0
    } else {
        out(sum.sqrt(), upper)?
    };
    if root == 1.0 {
        Ok(scale)
    } else {
        out(scale * root, upper)
    }
}
fn bounds(value: f64, lo: f64, hi: f64) -> Result<(f64, f64), String> {
    require(
        lo >= 0.0 && lo <= value && value <= hi && [lo, value, hi].iter().all(|v| v.is_finite()),
        "ENCLOSURE",
    )?;
    let absolute = if lo == value && value == hi {
        0.0
    } else {
        out((value - lo).max(hi - value), true)?
    };
    if hi == 0.0 {
        return Ok((absolute, 0.0));
    }
    require(
        lo.is_normal() && value.is_normal() && hi.is_normal(),
        "PUBLICATION_RANGE",
    )?;
    let relative = if absolute == 0.0 {
        0.0
    } else {
        out(absolute / lo, true)?
    };
    require(relative <= 1e-9, "PROTECTED_CRITERION")?;
    Ok((absolute, relative))
}
fn endpoint(area: f64, z: f64, actions: &Value) -> Result<(f64, [f64; 2]), String> {
    require(area > 0.0 && z > 0.0, "SECTION_RANGE")?;
    let a = array(actions)?;
    require(a.len() == 3, "ENDPOINT_ACTION_COUNT")?;
    let value = finite(
        finite(num(&a[0]["value"])?.abs() / area)?
            + norm(&[
                finite(num(&a[1]["value"])?.abs() / z)?,
                finite(num(&a[2]["value"])?.abs() / z)?,
            ])?,
    )?;
    let p = [
        absolute(interval(&a[0]["interval"])?),
        absolute(interval(&a[1]["interval"])?),
        absolute(interval(&a[2]["interval"])?),
    ];
    let mut bound = [0.0; 2];
    for (side, upper) in [false, true].into_iter().enumerate() {
        let axial = div(p[0][side], area, upper)?;
        let y = div(p[1][side], z, upper)?;
        let zz = div(p[2][side], z, upper)?;
        let bending = if y == 0.0 || zz == 0.0 {
            y.max(zz)
        } else {
            let scale = y.max(zz);
            let a = div(y, scale, upper)?;
            let b = div(zz, scale, upper)?;
            mul(
                scale,
                out(
                    add(mul(a, a, upper)?, mul(b, b, upper)?, upper)?.sqrt(),
                    upper,
                )?,
                upper,
            )?
        };
        bound[side] = add(axial, bending, upper)?;
    }
    require(bound[0] <= value && value <= bound[1], "ENDPOINT_ENCLOSURE")?;
    Ok((value, bound))
}
fn maximum_link(exact: &Value, ex: &Value, record: &Value) -> Result<(), String> {
    let plan = &record["source"];
    require(
        ex["load_case_id"] == exact["load_case_id"]
            && ex["basis"] == MAX_BASIS
            && ex["coefficient_basis"] == "retained_section_functionals_binary64",
        "MAXIMUM_METHOD",
    )?;
    require(
        ex["source_identity_sha256"] == plan["retained_identity_sha256"],
        "MAXIMUM_SOURCE_IDENTITY",
    )?;
    let section = array(&exact["pipe_sections"])?
        .iter()
        .find(|s| s["pipe_id"] == ex["pipe_id"])
        .ok_or("PHYSICS_SOURCE_MAXIMUM_SECTION")?;
    let source = array(&plan["endpoint_sections"])?
        .iter()
        .find(|s| s["pipe_id"] == ex["pipe_id"])
        .ok_or("PHYSICS_SOURCE_MAXIMUM_ENDPOINTS")?;
    require(
        same(&ex["area_m2"], num(&section["As_m2"])?)?
            && same(&ex["section_modulus_m3"], num(&section["Z_m3"])?)?,
        "MAXIMUM_SECTION",
    )?;
    let ends = array(&ex["endpoints"])?;
    let claimed = array(&source["endpoints"])?;
    require(
        ends.len() == 2 && claimed.len() == 2,
        "MAXIMUM_ENDPOINT_COUNT",
    )?;
    let mut indices = HashSet::new();
    let cid = text(&exact["load_case_id"])?;
    for (endpoint, (end, claim)) in ends.iter().zip(claimed).enumerate() {
        for key in [
            "station_fraction",
            "functional_indices",
            "functional_ids",
            "actions",
        ] {
            require(end[key] == claim[key], "MAXIMUM_ENDPOINT_SOURCE")?;
        }
        require(
            num(&end["station_fraction"])? == endpoint as f64,
            "MAXIMUM_ENDPOINT_ORDER",
        )?;
        for ((index, id), action) in array(&end["functional_indices"])?
            .iter()
            .zip(array(&end["functional_ids"])?)
            .zip(array(&end["actions"])?)
        {
            let i = index.as_u64().ok_or("PHYSICS_SOURCE_FUNCTIONAL_INDEX")?;
            require(
                indices.insert(i)
                    && i < plan["functional_count"]
                        .as_u64()
                        .ok_or("PHYSICS_SOURCE_FUNCTIONAL_COUNT")?
                    && *id == format!("source-functional:{}:{cid}:{i}", cid.len()),
                "MAXIMUM_FUNCTIONAL_ID",
            )?;
            let p = interval(&action["interval"])?;
            let v = num(&action["value"])?;
            require(p[0] <= v && v <= p[1], "MAXIMUM_ACTION_INTERVAL")?;
        }
    }
    require(indices.len() == 6, "MAXIMUM_FUNCTIONAL_BIJECTION")
}
pub(crate) fn validate_maximum(
    source: &Value,
    exact: &Value,
    ex: &Value,
    rows: &HashMap<&str, &Value>,
) -> Result<(), String> {
    let record = case(source, text(&exact["load_case_id"])?)?;
    maximum_link(exact, ex, record)?;
    let row = rows
        .get(text(&ex["result_id"])?)
        .ok_or("PHYSICS_SOURCE_MAXIMUM_ROW")?;
    require(
        row["kind"] == "pipe_elastic_normal_stress_maximum_v2"
            && row["entity_ref"] == ex["pipe_id"]
            && row["basis_ref"] == record["basis_ref"]
            && row["unit"] == "Pa"
            && same(&row["value"], num(&ex["value_pa"])?)?,
        "MAXIMUM_ROW",
    )?;
    let ends = array(&ex["endpoints"])?;
    let n0 = interval(&ends[0]["actions"][0]["interval"])?;
    let n1 = interval(&ends[1]["actions"][0]["interval"])?;
    require(n0[0] <= n1[1] && n1[0] <= n0[1], "CONSTANT_AXIAL_BOUND")?;
    for end in ends {
        let (value, bound) = endpoint(
            num(&ex["area_m2"])?,
            num(&ex["section_modulus_m3"])?,
            &end["actions"],
        )?;
        require(
            same(&end["value_pa"], value)?
                && same(&end["interval_pa"][0], bound[0])?
                && same(&end["interval_pa"][1], bound[1])?,
            "MAXIMUM_ENDPOINT_RECIPE",
        )?;
    }
    let values = [num(&ends[0]["value_pa"])?, num(&ends[1]["value_pa"])?];
    let intervals = [
        interval(&ends[0]["interval_pa"])?,
        interval(&ends[1]["interval_pa"])?,
    ];
    let value = values[0].max(values[1]);
    let lo = intervals[0][0].max(intervals[1][0]);
    let hi = intervals[0][1].max(intervals[1][1]);
    let (abs, rel) = bounds(value, lo, hi)?;
    for (key, v) in [
        ("value_pa", value),
        ("value_lower_pa", lo),
        ("value_upper_pa", hi),
        ("absolute_error_bound_pa", abs),
        ("relative_error_bound", rel),
    ] {
        require(same(&ex[key], v)?, "MAXIMUM_BOUNDS")?;
    }
    require(
        ex["relative_limit"] == 1e-9
            && num(&ex["station_fraction"])? == if values[1] > values[0] { 1.0 } else { 0.0 },
        "MAXIMUM_WITNESS",
    )?;
    let equal = |component: usize, magnitude: bool| -> Result<bool, String> {
        let a = &ends[0]["actions"][component];
        let b = &ends[1]["actions"][component];
        let x = interval(&a["interval"])?;
        let y = interval(&b["interval"])?;
        Ok(x[0] == x[1]
            && y[0] == y[1]
            && if magnitude {
                x[0].abs() == y[0].abs()
            } else {
                x[0] == y[0]
            })
    };
    let locations = if equal(1, false)? && equal(2, false)? {
        json!({"kind":"whole_span_constant"})
    } else if intervals[0][0] > intervals[1][1] {
        json!({"kind":"strict_endpoint","endpoint":"i"})
    } else if intervals[1][0] > intervals[0][1] {
        json!({"kind":"strict_endpoint","endpoint":"j"})
    } else {
        let mut overlap = true;
        for c in [1, 2] {
            let a = interval(&ends[0]["actions"][c]["interval"])?;
            let b = interval(&ends[1]["actions"][c]["interval"])?;
            overlap &= a[0] <= b[1] && b[0] <= a[1];
        }
        json!({"kind":"endpoint_candidates","exact_tie_proven":equal(1,true)?&&equal(2,true)?,"interior_equal_possible":overlap})
    };
    require(ex["locations"] == locations, "MAXIMUM_LOCATION_CLAIM")
}
pub(crate) fn is_composite_recipe(recipe: &Value) -> bool {
    matches!(
        recipe.as_str(),
        Some(
            MAX_BASIS
                | "support_force_norm_scaled_checked_v1"
                | "support_moment_norm_scaled_checked_v1"
                | "retained_source_straight_stress_v1"
        )
    )
}
pub(crate) fn validate_derived(
    source: &Value,
    record: &Value,
    treatment: &Value,
    row: &Value,
    inputs: &[&Value],
) -> Result<(), String> {
    let recipe = text(&treatment["recipe_id"])?;
    if recipe == MAX_BASIS {
        let exact = array(&source["contract_evidence"]["exact_cases"])?
            .iter()
            .find(|c| c["load_case_id"] == record["basis_ref"]["ref_id"])
            .ok_or("PHYSICS_SOURCE_CASE")?;
        let ex = array(&exact["pipe_stress_extrema"])?
            .iter()
            .find(|e| e["result_id"] == row["id"])
            .ok_or("PHYSICS_SOURCE_MAXIMUM_DERIVED_BINDING")?;
        require(inputs.is_empty(), "MAXIMUM_INPUTS")?;
        let rows = array(&source["results"])?
            .iter()
            .map(|r| Ok((text(&r["id"])?, r)))
            .collect::<Result<HashMap<_, _>, String>>()?;
        return validate_maximum(source, exact, ex, &rows);
    }
    if recipe == "retained_source_straight_stress_v1" {
        return validate_stress(source, record, row, inputs);
    }
    let checks = array(&record["derived_checks"])?
        .iter()
        .filter(|c| c["result_id"] == row["id"])
        .collect::<Vec<_>>();
    require(checks.len() == 1, "NORM_CHECK_ID")?;
    let check = checks[0];
    let force = recipe == "support_force_norm_scaled_checked_v1";
    let components = if force {
        ["Fx", "Fy", "Fz"]
    } else {
        ["Mx", "My", "Mz"]
    };
    require(
        check["recipe_id"] == recipe
            && row["kind"]
                == if force {
                    "support_reaction_force_magnitude_v2"
                } else {
                    "support_reaction_moment_magnitude_v2"
                }
            && check["support_id"] == row["entity_ref"]
            && row["unit"] == if force { "N" } else { "N*m" },
        "NORM_SIGNATURE",
    )?;
    require(inputs.len() == 3, "NORM_INPUT_COUNT")?;
    let cid = text(&record["basis_ref"]["ref_id"])?;
    let mut values = [0.0; 3];
    let mut ranges = [[0.0; 2]; 3];
    for (i, input) in inputs.iter().enumerate() {
        require(
            input["kind"] == "support_reaction_component_v2"
                && input["metadata"]["component"] == components[i]
                && input["entity_ref"] == row["entity_ref"]
                && input["unit"] == row["unit"],
            "NORM_INPUTS",
        )?;
        let p = array(&record["projections"])?
            .iter()
            .filter(|p| p["result_id"] == input["id"])
            .collect::<Vec<_>>();
        require(p.len() == 1, "NORM_PROJECTION")?;
        let p = p[0];
        require(
            check["functional_ids"][i] == p["functional_id"]
                && same(&check["values"][i], num(&p["value"])?)?
                && check["intervals"][i] == p["interval"],
            "NORM_PROJECTION_BINDING",
        )?;
        let index = check["functional_indices"][i]
            .as_u64()
            .ok_or("PHYSICS_SOURCE_NORM_INDEX")?;
        require(
            check["functional_ids"][i] == format!("source-functional:{}:{cid}:{index}", cid.len()),
            "NORM_FUNCTIONAL_INDEX",
        )?;
        values[i] = num(&check["values"][i])?;
        ranges[i] = absolute(interval(&check["intervals"][i])?);
    }
    let lo = norm_bound(ranges.map(|p| p[0]), false)?;
    let hi = norm_bound(ranges.map(|p| p[1]), true)?;
    let value = norm(&values)?;
    let (abs, rel) = bounds(value, lo, hi)?;
    require(
        same(&row["value"], value)?
            && same(&check["value"], value)?
            && same(&check["interval"][0], lo)?
            && same(&check["interval"][1], hi)?
            && same(&check["absolute_error_bound"], abs)?
            && same(&check["relative_error_bound"], rel)?
            && check["relative_limit"] == 1e-9,
        "NORM_RECIPE",
    )
}
pub(crate) fn validate_source_case(
    source: &Value,
    record: &Value,
    invocation: Option<&Value>,
) -> Result<(), String> {
    require(
        matches!(
            record["ordinary_attempt"]["outcome"].as_str(),
            Some("sensitive" | "rejected")
        ),
        "SOURCE_FALLBACK_TRIGGER",
    )?;
    let exact = array(&source["contract_evidence"]["exact_cases"])?
        .iter()
        .find(|c| c["load_case_id"] == record["basis_ref"]["ref_id"])
        .ok_or("PHYSICS_SOURCE_CASE")?;
    let rhs = &exact["pressure_rhs_assembly"];
    require(array(&rhs["groups"])?.is_empty(), "SOURCE_PRESSURE_RHS")?;
    for key in [
        "assembled_pressure_rhs_global",
        "rounded_cap_rhs_global",
        "rounded_poisson_rhs_global",
    ] {
        require(
            array(&rhs[key])?.iter().all(|v| v.as_f64() == Some(0.0)),
            "SOURCE_PRESSURE_RHS",
        )?;
    }
    section_functionals(record)?;
    let stresses = array(&record["section_stress_checks"])?;
    let expected_stress = array(&record["rows"])?
        .iter()
        .filter(|r| r["recipe_id"] == "retained_source_straight_stress_v1")
        .map(|r| text(&r["result_id"]))
        .collect::<Result<HashSet<_>, _>>()?;
    require(
        stresses.len() == expected_stress.len()
            && stresses.len() == 20 * array(&record["source"]["member_ids"])?.len()
            && stresses
                .iter()
                .map(|r| text(&r["result_id"]))
                .collect::<Result<HashSet<_>, _>>()?
                == expected_stress,
        "STRESS_CHECK_COVERAGE",
    )?;
    let checks = array(&record["derived_checks"])?;
    let expected = array(&record["rows"])?
        .iter()
        .filter(|r| {
            matches!(
                r["recipe_id"].as_str(),
                Some(
                    "support_force_norm_scaled_checked_v1"
                        | "support_moment_norm_scaled_checked_v1"
                )
            )
        })
        .map(|r| text(&r["result_id"]))
        .collect::<Result<HashSet<_>, _>>()?;
    require(
        checks.len() == expected.len()
            && checks.len() == 2 * array(&record["source"]["support_ids"])?.len()
            && checks
                .iter()
                .map(|r| text(&r["result_id"]))
                .collect::<Result<HashSet<_>, _>>()?
                == expected,
        "DERIVED_CHECK_COVERAGE",
    )?;
    if let Some(invocation) = invocation {
        let model = &invocation["request"]["model"];
        require(
            model["schema_version"] == "0.3.0"
                && model["pressure_contract"] == json!({"version":"2.0.0","mode":PROFILE}),
            "ACTUAL_EXACT_PROFILE",
        )?;
        let actual = array(&model["load_cases"])?
            .iter()
            .find(|c| c["id"] == record["basis_ref"]["ref_id"])
            .ok_or("PHYSICS_SOURCE_ACTUAL_CASE")?;
        require(
            actual["pressure_regions"] == json!([]),
            "ACTUAL_EMPTY_PRESSURE_INVENTORY",
        )?;
        require(
            model["components"].as_array().is_none_or(|a| a.is_empty())
                && model["combinations"]
                    .as_array()
                    .is_none_or(|a| a.is_empty())
                && actual["equivalent_static"].is_null(),
            "ACTUAL_SOURCE_FAMILY",
        )?;
        require(
            array(&actual["primitive_loads"])?
                .iter()
                .all(|l| l["target"]["type"] == "node" && l["category"] != "thermal"),
            "ACTUAL_NODAL_LOADS",
        )?;
        require(
            array(&model["supports"])?.iter().all(|s| {
                s["nonlinear"].is_null()
                    && s["family"] != "constant_effort_support"
                    && s["hanger"]["constant_load"].is_null()
            }),
            "ACTUAL_SUPPORT_FAMILY",
        )?;
        actual_materials(invocation, actual, exact)?;
    }
    Ok(())
}
fn actual_materials(invocation: &Value, actual: &Value, exact: &Value) -> Result<(), String> {
    let model = &invocation["request"]["model"];
    let materials = invocation["request"]
        .get("materials")
        .filter(|m| m.as_array().is_some_and(|a| !a.is_empty()))
        .unwrap_or(&model["materials"]);
    for ((pipe, section), material) in array(&model["pipe_segments"])?
        .iter()
        .zip(array(&exact["pipe_sections"])?)
        .zip(array(&exact["pipe_materials"])?)
    {
        require(
            pipe["id"] == section["pipe_id"]
                && pipe["id"] == material["pipe_id"]
                && pipe["material"] == material["material_id"],
            "ACTUAL_MATERIAL_MEMBER",
        )?;
        let authored_section = &pipe["section"];
        let quantity = |q: &Value| -> Result<f64, String> {
            let factor = match text(&q["unit"])? {
                "m" => 1.0,
                "mm" => 0.001,
                "cm" => 0.01,
                "in" => 0.0254,
                "ft" => 0.3048,
                _ => return Err("PHYSICS_SOURCE_LENGTH_UNIT".into()),
            };
            Ok(num(&q["value"])? * factor)
        };
        let tolerance = if authored_section["mill_tolerance"].is_null() {
            0.0
        } else {
            quantity(&authored_section["mill_tolerance"])?
        };
        require(
            same(
                &section["outside_diameter_m"],
                quantity(&authored_section["outside_diameter"])?,
            )? && same(
                &section["effective_wall_thickness_m"],
                quantity(&authored_section["wall_thickness"])? - tolerance,
            )?,
            "ACTUAL_SOURCE_GEOMETRY",
        )?;
        let authored = array(materials)?
            .iter()
            .find(|m| m["id"] == pipe["material"])
            .ok_or("PHYSICS_SOURCE_ACTUAL_MATERIAL")?;
        require(
            authored["constitutive_basis"] == material["constitutive_basis"],
            "ACTUAL_CONSTITUTIVE_BASIS",
        )?;
        let selected = if !actual["modulus_basis_ref"].is_null() {
            array(&authored["temperature_points"])?
                .iter()
                .find(|p| p["id"] == actual["modulus_basis_ref"])
                .ok_or("PHYSICS_SOURCE_ACTUAL_POINT")?
                .clone()
        } else if actual["modulus_basis_temperature"].is_null() {
            authored.clone()
        } else {
            let mut points = array(&authored["temperature_points"])?
                .iter()
                .collect::<Vec<_>>();
            points.sort_by(|a, b| {
                a["temperature"]["value"]
                    .as_f64()
                    .unwrap_or(f64::NAN)
                    .total_cmp(&b["temperature"]["value"].as_f64().unwrap_or(f64::NAN))
            });
            let t = num(&actual["modulus_basis_temperature"]["value"])?;
            let p = points
                .windows(2)
                .find(|p| {
                    p[0]["temperature"]["value"].as_f64().is_some_and(|a| a < t)
                        && p[1]["temperature"]["value"].as_f64().is_some_and(|b| t < b)
                })
                .ok_or("PHYSICS_SOURCE_ACTUAL_BRACKET")?;
            let a = num(&p[0]["temperature"]["value"])?;
            let b = num(&p[1]["temperature"]["value"])?;
            let f = (t - a) / (b - a);
            let mut selected = json!({});
            for k in ["elastic_modulus", "poisson_ratio"] {
                selected[k] =
                    json!({"value":(1.0-f)*num(&p[0][k]["value"])?+f*num(&p[1][k]["value"])?});
            }
            selected
        };
        require(
            same(
                &material["E_pa"],
                num(&selected["elastic_modulus"]["value"])?,
            )? && same(&material["nu"], num(&selected["poisson_ratio"]["value"])?)?
                && material["thermal_consumed"] == false,
            "ACTUAL_SELECTED_MATERIAL",
        )?;
    }
    Ok(())
}
fn section_functionals(record: &Value) -> Result<(), String> {
    let plan = &record["source"];
    let members = array(&plan["section_functionals"])?;
    let endpoints = array(&plan["endpoint_sections"])?;
    require(
        members
            .iter()
            .map(|m| &m["pipe_id"])
            .eq(array(&plan["member_ids"])?.iter())
            && members.len() == endpoints.len(),
        "SECTION_FUNCTIONAL_MEMBERS",
    )?;
    let cid = text(&record["basis_ref"]["ref_id"])?;
    let mut indices = HashSet::new();
    for (member, endpoint) in members.iter().zip(endpoints) {
        let stations = array(&member["stations"])?;
        require(stations.len() == 5, "SECTION_FUNCTIONAL_STATIONS")?;
        for (slot, station) in stations.iter().enumerate() {
            require(
                num(&station["station_fraction"])? == [0.0, 0.25, 0.5, 0.75, 1.0][slot],
                "SECTION_FUNCTIONAL_STATIONS",
            )?;
            for ((i, id), action) in array(&station["functional_indices"])?
                .iter()
                .zip(array(&station["functional_ids"])?)
                .zip(array(&station["actions"])?)
            {
                let i = i.as_u64().ok_or("PHYSICS_SOURCE_SECTION_INDEX")?;
                require(
                    indices.insert(i)
                        && i < plan["functional_count"]
                            .as_u64()
                            .ok_or("PHYSICS_SOURCE_FUNCTIONAL_COUNT")?
                        && *id == format!("source-functional:{}:{cid}:{i}", cid.len()),
                    "SECTION_FUNCTIONAL_ID",
                )?;
                let [lo, hi] = interval(&action["interval"])?;
                let v = num(&action["value"])?;
                require(lo <= v && v <= hi, "SECTION_FUNCTIONAL_INTERVAL")?;
                if v == 0.0 {
                    require(lo == 0.0 && hi == 0.0, "SECTION_ZERO_CERTIFICATE")?;
                } else {
                    let arithmetic = 128.0 * f64::EPSILON;
                    let limit = (1e-9 - arithmetic) / (1.0 + arithmetic);
                    require(
                        lo.signum() == hi.signum()
                            && lo != 0.0
                            && hi != 0.0
                            && (v - lo).abs().max((hi - v).abs()) / lo.abs().min(hi.abs()) <= limit,
                        "SECTION_FUNCTIONAL_CRITERION",
                    )?;
                }
            }
        }
        for (i, station) in [0usize, 4].into_iter().map(|s| &stations[s]).enumerate() {
            let pick = |key: &str| {
                [0usize, 4, 5]
                    .into_iter()
                    .map(|c| station[key][c].clone())
                    .collect::<Vec<_>>()
            };
            require(
                endpoint["endpoints"][i]
                    == json!({"station_fraction":station["station_fraction"],"functional_indices":pick("functional_indices"),"functional_ids":pick("functional_ids"),"actions":pick("actions")}),
                "ENDPOINT_SECTION_LINK",
            )?;
        }
    }
    Ok(())
}
fn validate_stress(
    source: &Value,
    record: &Value,
    row: &Value,
    inputs: &[&Value],
) -> Result<(), String> {
    let checks = array(&record["section_stress_checks"])?
        .iter()
        .filter(|c| c["result_id"] == row["id"])
        .collect::<Vec<_>>();
    require(checks.len() == 1 && inputs.is_empty(), "STRESS_CHECK_ID")?;
    let check = checks[0];
    let (component, label) = match text(&row["kind"])? {
        "element_local_axial_normal_stress" => (0, "axial_normal_stress"),
        "element_local_bending_normal_stress_y" => (4, "bending_normal_stress_y"),
        "element_local_bending_normal_stress_z" => (5, "bending_normal_stress_z"),
        "element_local_torsional_shear_stress" => (3, "torsional_shear_stress"),
        _ => return Err("PHYSICS_SOURCE_STRESS_KIND".into()),
    };
    let location = text(&row["metadata"]["location"])?;
    let index = ["end_i", "quarter_1", "midspan", "quarter_3", "end_j"]
        .iter()
        .position(|s| *s == location)
        .ok_or("PHYSICS_SOURCE_STRESS_LOCATION")?;
    require(
        row["metadata"]["component"] == label
            && row["unit"] == "MPa"
            && check["pipe_id"] == row["entity_ref"]
            && check["location"] == location
            && check["component"] == label
            && check["recipe_id"] == "retained_source_straight_stress_v1",
        "STRESS_SIGNATURE",
    )?;
    let member = array(&record["source"]["section_functionals"])?
        .iter()
        .find(|m| m["pipe_id"] == row["entity_ref"])
        .ok_or("PHYSICS_SOURCE_STRESS_MEMBER")?;
    let station = &member["stations"][index];
    require(
        check["functional_index"] == station["functional_indices"][component]
            && check["functional_id"] == station["functional_ids"][component]
            && check["action"] == station["actions"][component],
        "STRESS_FUNCTIONAL_LINK",
    )?;
    let exact = array(&source["contract_evidence"]["exact_cases"])?
        .iter()
        .find(|c| c["load_case_id"] == record["basis_ref"]["ref_id"])
        .ok_or("PHYSICS_SOURCE_STRESS_CASE")?;
    let section = array(&exact["pipe_sections"])?
        .iter()
        .find(|s| s["pipe_id"] == row["entity_ref"])
        .ok_or("PHYSICS_SOURCE_STRESS_SECTION")?;
    let params = &check["parameters"];
    require(
        *params
            == json!({"area_m2":section["As_m2"],"section_modulus_m3":section["Z_m3"],"torsion_radius_m":section["ro_m"],"torsion_constant_m4":section["J_m4"],"pa_per_mpa":1e6}),
        "STRESS_SECTION_PARAMETERS",
    )?;
    let action = num(&check["action"]["value"])?;
    let pa = if component == 0 {
        action / num(&params["area_m2"])?
    } else if component == 3 {
        action * num(&params["torsion_radius_m"])? / num(&params["torsion_constant_m4"])?
    } else {
        action / num(&params["section_modulus_m3"])?
    };
    let value = pa / 1e6;
    require(
        if action == 0.0 {
            pa == 0.0 && value == 0.0
        } else {
            pa.is_normal() && value.is_normal()
        },
        "STRESS_PUBLICATION_RANGE",
    )?;
    require(same(&row["value"], value)?, "STRESS_RECIPE_VALUE")
}
pub fn validate(source: &Value, actual_invocation: Option<&Value>) -> Result<bool, String> {
    validate_transport_metadata(source)?;
    crate::physics_evidence::validate_physics_evidence_in(source, true)?;
    crate::source_blocks::validate_in(source, actual_invocation, true)
}
