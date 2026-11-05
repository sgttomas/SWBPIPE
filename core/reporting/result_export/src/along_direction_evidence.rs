//! R-6/R-7 S-K1 (DEL-04-03 Work `skew-restraints.md` §7.1, §10.5, §11.1 R-1): the
//! independent reader's re-check of `support_reaction_along_direction_v1`. It imports
//! no product code: the row's exact strings, one row per skewed record and case beside
//! that record's Fx/Fy/Fz rows, and, with the model, s·|d| = dᵀR within a
//! representation guard (d the record's authored `direction`, R its three global
//! support-on-pipe component rows).
use serde_json::Value;
use std::collections::{HashMap, HashSet};

type Check = Result<(), String>;

pub(crate) const KIND: &str = "support_reaction_along_direction_v1";
const COMPONENT: &str = "along_direction";
const COORDINATE_SYSTEM: &str = "support_direction";
const SIGN: &str = "support_on_pipe_positive_along_authored_direction";
const BASIS: &str = "recovered_from_assembled_support_law";
const SUPPORT_COMPONENT: &str = "support_reaction_component_v2";

fn require(ok: bool, code: &str) -> Check {
    if ok {
        Ok(())
    } else {
        Err(code.to_string())
    }
}

/// The row's declared semantics: a case row's strings exactly; a combination row's
/// component, coordinate system, location and unit.
pub(crate) fn row_semantics(row: &Value, direct: bool) -> Check {
    let md = &row["metadata"];
    require(
        row["unit"] == "N" && md["component"] == COMPONENT && md["coordinate_system"] == COORDINATE_SYSTEM && md["location"] == "node",
        "ROW_SEMANTICS",
    )?;
    if direct {
        require(md["basis"] == BASIS && md["sign_convention"] == SIGN, "ROW_SEMANTICS")?;
    }
    Ok(())
}

/// A skewed record's authored direction: two or more nonzero components (an exactly
/// axis-aligned direction is lowered and publishes no row).
fn skewed_direction(support: &Value) -> Option<[f64; 3]> {
    let d = &support["direction"];
    let v = [d["x"].as_f64()?, d["y"].as_f64()?, d["z"].as_f64()?];
    (v.iter().filter(|c| **c != 0.0).count() >= 2 && support["family"] != "snubber").then_some(v)
}

/// The row id's exact source identity: `{L(part)}:{part}` concatenated, L the UTF-8 byte
/// length (the agreed `exact_source_identity([case, support])`, no separator).
fn exact_source_identity(parts: &[&str]) -> String {
    parts.iter().map(|part| format!("{}:{part}", part.len())).collect()
}

/// d scaled exactly by a power of two to max|d_i| ∈ [1, 2): s·|d| = dᵀR is invariant
/// under a positive scale, and the scaled norm never under- or overflows.
fn canonical(mut d: [f64; 3]) -> [f64; 3] {
    let largest = |d: &[f64; 3]| d.iter().fold(0.0_f64, |m, x| m.max(x.abs()));
    while largest(&d) >= 2.0 {
        let step = if largest(&d) >= 2f64.powi(512) { 2f64.powi(-511) } else { 0.5 };
        d = d.map(|x| x * step);
    }
    while largest(&d) < 1.0 {
        let step = if largest(&d) < 2f64.powi(-511) { 2f64.powi(511) } else { 2.0 };
        d = d.map(|x| x * step);
    }
    d
}

/// One case's along-direction rows (`rows`: the case's direct rows; `case_id`; `model`).
pub(crate) fn check_case(rows: &[&Value], case_id: &str, model: Option<&Value>) -> Check {
    let mut along: HashMap<&str, &Value> = HashMap::new();
    let mut components: HashMap<(&str, &str), f64> = HashMap::new();
    for row in rows {
        let entity = row["entity_ref"].as_str().unwrap_or("");
        if row["kind"] == KIND {
            require(
                row["id"].as_str() == Some(&format!("result:support-along-direction-v1:{}", exact_source_identity(&[case_id, entity]))),
                "ALONG_DIRECTION_ROW_ID",
            )?;
            require(along.insert(entity, row).is_none(), "ALONG_DIRECTION_DUPLICATE")?;
        } else if row["kind"] == SUPPORT_COMPONENT {
            if let (Some(c), Some(v)) = (row["metadata"]["component"].as_str(), row["value"].as_f64()) {
                components.insert((entity, c), v);
            }
        }
    }
    for (entity, row) in &along {
        let r: Option<Vec<f64>> = ["Fx", "Fy", "Fz"].iter().map(|c| components.get(&(*entity, *c)).copied()).collect();
        require(r.is_some(), "ALONG_DIRECTION_COMPONENTS_MISSING")?;
        let Some(model) = model else { continue };
        let support = model["supports"].as_array().into_iter().flatten().find(|s| s["id"] == *entity);
        let d = support.and_then(skewed_direction);
        require(d.is_some(), "ALONG_DIRECTION_UNEXPECTED")?;
        let (d, r) = (canonical(d.unwrap()), r.unwrap());
        let s = row["value"].as_f64().ok_or("ALONG_DIRECTION_VALUE")?;
        let norm = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        let dot: f64 = (0..3).map(|i| d[i] * r[i]).sum();
        let scale: f64 = norm * r.iter().map(|x| x.abs()).sum::<f64>();
        require(
            s.is_finite() && (s * norm - dot).abs() <= 64.0 * f64::EPSILON * scale.max(f64::MIN_POSITIVE),
            "ALONG_DIRECTION_INCONSISTENT",
        )?;
    }
    if let Some(model) = model {
        // Every skewed record that publishes its component rows publishes its along row.
        let published: HashSet<&str> = components.keys().map(|(e, _)| *e).collect();
        for support in model["supports"].as_array().into_iter().flatten() {
            let id = support["id"].as_str().unwrap_or("");
            if skewed_direction(support).is_some() && published.contains(id) {
                require(along.contains_key(id), "ALONG_DIRECTION_MISSING")?;
            }
        }
    }
    Ok(())
}
