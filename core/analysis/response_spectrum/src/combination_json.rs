//! The JSON input and output of the combinations (unit S4): the form of the
//! `combine_contributions` example and of the contract fixtures under `fixtures/combination/`.
//!
//! This is the unit's own interchange. It is not the `spectrum-1` publication, its profile or
//! its readers (REQ-006), which a later unit defines and which may take these identities and
//! tags. An input document carries only supplied values: the method choices, the damping ratio,
//! the modal metadata, the rows and the signed contributions, with a free-text `provenance`
//! that says where they came from. The output echoes that provenance unchanged.
//!
//! Input (`openpipestress.spectrum_combination.input/v1`), every member required except as
//! noted:
//! - `schema`, `provenance` (text);
//! - `damping_ratio`;
//! - `modal_combination` (`SRSS`, `CQC`, `GROUPING`, `TEN_PERCENT`, `ABSOLUTE_SUM`),
//!   `directional_combination` (`SRSS`, `100-40-40`, `ABSOLUTE_SUM`) and `missing_mass`
//!   (`{"applied": false}` or `{"applied": true, "rigid_frequency_hz": f, "rigid_combination":
//!   "SRSS" | "ABSOLUTE_SUM"}`). An absent or `null` choice is refused by name
//!   (`SPECTRUM_METHOD_CHOICE_MISSING`; an absent rigid frequency `SPECTRUM_RIGID_FREQUENCY_INVALID`);
//!   none has a default;
//! - `directions`: `X`, `Y`, `Z`, each once;
//! - `modes`: `{"mode", "cluster", "frequency_hz"}` for every extracted mode;
//! - `rows`: `{"key", "quantity", "object", "end" (element end forces only), "component",
//!   "unit", "frame"}`. The key, unit and frame must be the ones the quantity has;
//! - `contributions`: `{"mode", "direction", "sign_status": "signed_contribution", "values"}`,
//!   one value per row;
//! - `rigid_responses`: `{"direction", "periodic_clusters_left_out", "sign_status":
//!   "signed_contribution", "values"}`, empty without the correction.
//!
//! Output (`openpipestress.spectrum_combination.output/v1`): the statement, the provenance,
//! `outcome` (`combined` or `refused`) with the refusals, and when combined the rules, the
//! cluster statement, the ρ matrix, groups or coupled pairs the rule used, the signed cluster
//! sums and the unsigned combined rows, each tagged with its sign status.
use crate::combine::combine;
use crate::contract::{
    ClusterIndex, Combination, CombinationRequest, Direction, DirectionalRule, DisplacementComponent, End,
    ForceComponent, Frame, MissingMass, ModalContribution, ModalMode, ModalRule, ModeIndex, Quantity,
    ReactionComponent, RigidContribution, RigidRule, SignStatus, COMBINATION_STATEMENT,
};
use crate::refusal::{Refusal, RefusalCode};
use serde::Deserialize;
use serde_json::{json, Value};

pub const INPUT_SCHEMA: &str = "openpipestress.spectrum_combination.input/v1";
pub const OUTPUT_SCHEMA: &str = "openpipestress.spectrum_combination.output/v1";

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct InputDocument {
    schema: String,
    provenance: String,
    damping_ratio: f64,
    #[serde(default)]
    modal_combination: Option<String>,
    #[serde(default)]
    directional_combination: Option<String>,
    #[serde(default)]
    missing_mass: Option<MissingMassInput>,
    directions: Vec<String>,
    modes: Vec<ModeInput>,
    rows: Vec<RowInput>,
    contributions: Vec<ContributionInput>,
    rigid_responses: Vec<RigidInput>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct MissingMassInput {
    applied: bool,
    #[serde(default)]
    rigid_frequency_hz: Option<f64>,
    #[serde(default)]
    rigid_combination: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ModeInput {
    mode: u32,
    cluster: u32,
    frequency_hz: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct RowInput {
    key: String,
    quantity: String,
    object: String,
    #[serde(default)]
    end: Option<String>,
    component: String,
    unit: String,
    frame: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ContributionInput {
    mode: u32,
    direction: String,
    sign_status: String,
    values: Vec<f64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct RigidInput {
    direction: String,
    periodic_clusters_left_out: Vec<u32>,
    sign_status: String,
    values: Vec<f64>,
}

const REQUEST: &str = "request";

fn refusal(code: RefusalCode, object: &str, field: &str, detail: String) -> Refusal {
    Refusal::new(code, object, Some(field), detail)
}

fn row(input: &RowInput, refusals: &mut Vec<Refusal>) -> Option<Quantity> {
    let bad = |field: &str, detail: String| refusal(RefusalCode::RowInvalid, &input.key, field, detail);
    let frame = match input.frame.as_str() {
        "global" => Frame::Global,
        "member_axes" => Frame::MemberAxes,
        other => {
            refusals.push(bad("frame", format!("`{other}` is not `global` or `member_axes`")));
            return None;
        }
    };
    let object = input.object.clone();
    let quantity = match input.quantity.as_str() {
        "node_displacement" => DisplacementComponent::parse(&input.component)
            .map(|component| Quantity::NodeDisplacement { node: object, component }),
        "element_end_force" => {
            let end = match input.end.as_deref() {
                Some("i") => End::I,
                Some("j") => End::J,
                other => {
                    refusals.push(bad("end", format!("{other:?} is not `i` or `j`")));
                    return None;
                }
            };
            ForceComponent::parse(&input.component)
                .map(|component| Quantity::ElementEndForce { member: object, end, component, frame })
        }
        "support_reaction" => ReactionComponent::parse(&input.component)
            .map(|component| Quantity::SupportReaction { support: object, component }),
        other => {
            refusals.push(bad("quantity", format!("`{other}` is not a quantity kind")));
            return None;
        }
    };
    let Some(quantity) = quantity else {
        refusals.push(bad("component", format!("`{}` is not a component of a {}", input.component, input.quantity)));
        return None;
    };
    let mut ok = true;
    if input.end.is_some() && !matches!(quantity, Quantity::ElementEndForce { .. }) {
        refusals.push(bad("end", "only an element end force has an end".into()));
        ok = false;
    }
    if quantity.frame() != frame {
        refusals.push(bad("frame", format!("a {} is stated in global axes only", quantity.kind())));
        ok = false;
    }
    if input.unit != quantity.unit().symbol() {
        refusals.push(bad("unit", format!("`{}` is not the unit of this row, `{}`", input.unit, quantity.unit().symbol())));
        ok = false;
    }
    if input.key != quantity.key() {
        refusals.push(bad("key", format!("the key of this row is `{}`", quantity.key())));
        ok = false;
    }
    ok.then_some(quantity)
}

fn direction(text: &str, object: &str, refusals: &mut Vec<Refusal>) -> Option<Direction> {
    let d = Direction::parse(text);
    if d.is_none() {
        refusals.push(refusal(RefusalCode::DirectionInvalid, object, "direction", format!("`{text}` is not X, Y or Z")));
    }
    d
}

fn sign_status_ok(text: &str, object: &str, code: RefusalCode, refusals: &mut Vec<Refusal>) {
    if text != SignStatus::SignedContribution.as_str() {
        refusals.push(refusal(code, object, "sign_status",
            format!("`{text}` is not `{}`; only signed contributions are combined", SignStatus::SignedContribution.as_str())));
    }
}

fn choice<T>(value: &Option<String>, field: &str, parse: impl Fn(&str) -> Option<T>, names: &str,
    refusals: &mut Vec<Refusal>) -> Option<T> {
    match value {
        None => {
            refusals.push(refusal(RefusalCode::MethodChoiceMissing, REQUEST, field, format!("no {field} is chosen; it has no default")));
            None
        }
        Some(text) => {
            let parsed = parse(text);
            if parsed.is_none() {
                refusals.push(refusal(RefusalCode::MethodChoiceMissing, REQUEST, field, format!("`{text}` is not one of {names}")));
            }
            parsed
        }
    }
}

/// Reads a decoded input document into a request, refusing by name what the contract does not
/// admit at this level.
fn request_from(doc: &InputDocument) -> Result<CombinationRequest, Vec<Refusal>> {
    let mut refusals = Vec::new();
    let modal_rule = choice(&doc.modal_combination, "modal_combination", ModalRule::parse,
        "SRSS, CQC, GROUPING, TEN_PERCENT, ABSOLUTE_SUM", &mut refusals);
    let directional_rule = choice(&doc.directional_combination, "directional_combination", DirectionalRule::parse,
        "SRSS, 100-40-40, ABSOLUTE_SUM", &mut refusals);
    let missing_mass = match &doc.missing_mass {
        None => {
            refusals.push(refusal(RefusalCode::MethodChoiceMissing, REQUEST, "missing_mass",
                "whether to apply the missing-mass correction is not chosen; it has no default".into()));
            None
        }
        Some(MissingMassInput { applied: false, rigid_frequency_hz, rigid_combination }) => {
            if rigid_frequency_hz.is_some() || rigid_combination.is_some() {
                refusals.push(refusal(RefusalCode::MethodChoiceMissing, REQUEST, "missing_mass",
                    "a rigid frequency or rigid combination is given without the correction".into()));
            }
            Some(MissingMass::NotApplied)
        }
        Some(MissingMassInput { applied: true, rigid_frequency_hz, rigid_combination }) => {
            let rule = choice(rigid_combination, "rigid_combination", RigidRule::parse, "SRSS, ABSOLUTE_SUM", &mut refusals);
            if rigid_frequency_hz.is_none() {
                refusals.push(refusal(RefusalCode::RigidFrequencyInvalid, REQUEST, "rigid_frequency_hz",
                    "the correction is applied with no rigid frequency; it has no default".into()));
            }
            match (rigid_frequency_hz, rule) {
                (Some(f), Some(rule)) => Some(MissingMass::Applied { rigid_frequency_hz: *f, rule }),
                _ => None,
            }
        }
    };
    let directions: Vec<Option<Direction>> = doc.directions.iter().map(|d| direction(d, REQUEST, &mut refusals)).collect();
    let rows: Vec<Option<Quantity>> = doc.rows.iter().map(|r| row(r, &mut refusals)).collect();
    let periodic: Vec<Option<ModalContribution>> = doc
        .contributions
        .iter()
        .map(|c| {
            let object = format!("mode:{} {}", c.mode, c.direction);
            sign_status_ok(&c.sign_status, &object, RefusalCode::ContributionInvalid, &mut refusals);
            direction(&c.direction, &object, &mut refusals)
                .map(|direction| ModalContribution { mode: ModeIndex(c.mode), direction, values: c.values.clone() })
        })
        .collect();
    let rigid: Vec<Option<RigidContribution>> = doc
        .rigid_responses
        .iter()
        .map(|r| {
            sign_status_ok(&r.sign_status, &r.direction, RefusalCode::RigidResponseInvalid, &mut refusals);
            direction(&r.direction, &r.direction, &mut refusals).map(|direction| RigidContribution {
                direction,
                periodic_clusters_left_out: r.periodic_clusters_left_out.iter().map(|&c| ClusterIndex(c)).collect(),
                values: r.values.clone(),
            })
        })
        .collect();
    if !refusals.is_empty() {
        return Err(refusals);
    }
    Ok(CombinationRequest {
        damping_ratio: doc.damping_ratio,
        modal_rule: modal_rule.expect("checked"),
        directional_rule: directional_rule.expect("checked"),
        missing_mass: missing_mass.expect("checked"),
        directions: directions.into_iter().map(|d| d.expect("checked")).collect(),
        modes: doc
            .modes
            .iter()
            .map(|m| ModalMode { mode: ModeIndex(m.mode), cluster: ClusterIndex(m.cluster), frequency_hz: m.frequency_hz })
            .collect(),
        rows: rows.into_iter().map(|r| r.expect("checked")).collect(),
        periodic: periodic.into_iter().map(|p| p.expect("checked")).collect(),
        rigid: rigid.into_iter().map(|r| r.expect("checked")).collect(),
    })
}

/// A refusal as JSON: its code, object, field and detail.
pub fn refusal_json(refusal: &Refusal) -> Value {
    json!({"code": refusal.code.as_str(), "object": refusal.object, "field": refusal.field, "detail": refusal.detail})
}

/// A row's identity as JSON, in the input's form.
pub fn quantity_json(quantity: &Quantity) -> Value {
    let mut value = json!({
        "key": quantity.key(),
        "quantity": quantity.kind(),
        "object": quantity.object(),
        "component": quantity.component_name(),
        "unit": quantity.unit().symbol(),
        "frame": quantity.frame().as_str(),
    });
    if let Quantity::ElementEndForce { end, .. } = quantity {
        value["end"] = json!(end.as_str());
    }
    value
}

fn missing_mass_json(missing_mass: MissingMass) -> Value {
    match missing_mass {
        MissingMass::NotApplied => json!({"applied": false}),
        MissingMass::Applied { rigid_frequency_hz, rule } => {
            json!({"applied": true, "rigid_frequency_hz": rigid_frequency_hz, "rigid_combination": rule.as_str()})
        }
    }
}

/// A combination as the output document's members (without schema, statement and provenance).
pub fn combination_json(c: &Combination) -> Value {
    let unsigned = SignStatus::UnsignedCombinedMagnitude;
    json!({
        "rules": {
            "damping_ratio": c.damping_ratio,
            "modal_combination": c.modal_rule.as_str(),
            "directional_combination": c.directional_rule.as_str(),
            "missing_mass": missing_mass_json(c.missing_mass),
        },
        "directions": c.directions.iter().map(|d| d.as_str()).collect::<Vec<_>>(),
        "clusters": c.clusters.iter().map(|s| json!({
            "cluster": s.cluster.0,
            "modes": s.modes.iter().map(|m| m.0).collect::<Vec<_>>(),
            "frequencies_hz": s.frequencies_hz,
            "standing_frequency_hz": s.standing_frequency_hz,
            "class": s.class.as_str(),
        })).collect::<Vec<_>>(),
        "cqc_correlation": c.correlation.as_ref().map(|rho| json!({
            "clusters": c.clusters.iter().filter(|s| s.class == crate::contract::ClusterClass::Periodic)
                .map(|s| s.cluster.0).collect::<Vec<_>>(),
            "rho": rho,
        })),
        "grouping_groups": c.groups.as_ref().map(|g| g.iter().map(|group| group.iter().map(|i| i.0).collect::<Vec<_>>()).collect::<Vec<_>>()),
        "ten_percent_pairs": c.close_pairs.as_ref().map(|p| p.iter().map(|(a, b)| [a.0, b.0]).collect::<Vec<_>>()),
        "cluster_sums": c.cluster_sums.iter().map(|s| json!({
            "cluster": s.cluster.0,
            "direction": s.direction.as_str(),
            "sign_status": SignStatus::SignedClusterSum.as_str(),
            "values": s.values,
        })).collect::<Vec<_>>(),
        "rows": c.rows.iter().map(|r| {
            let mut v = quantity_json(&r.quantity);
            v["sign_status"] = json!(unsigned.as_str());
            v["per_direction"] = json!(r.per_direction.iter().map(|p| json!({
                "direction": p.direction.as_str(),
                "periodic": p.periodic.value(),
                "rigid": p.rigid.map(|m| m.value()),
                "combined": p.combined.value(),
            })).collect::<Vec<_>>());
            v["combined"] = json!(r.combined.value());
            v["governing_direction"] = json!(r.governing_direction.map(|d| d.as_str()));
            v
        }).collect::<Vec<_>>(),
    })
}

/// Writes a request as an input document, with the producer's statement of where its values
/// came from. Decoding the document gives the same request back.
pub fn request_to_json(request: &CombinationRequest, provenance: &str) -> Value {
    let signed = SignStatus::SignedContribution.as_str();
    json!({
        "schema": INPUT_SCHEMA,
        "provenance": provenance,
        "damping_ratio": request.damping_ratio,
        "modal_combination": request.modal_rule.as_str(),
        "directional_combination": request.directional_rule.as_str(),
        "missing_mass": missing_mass_json(request.missing_mass),
        "directions": request.directions.iter().map(|d| d.as_str()).collect::<Vec<_>>(),
        "modes": request.modes.iter().map(|m| json!({"mode": m.mode.0, "cluster": m.cluster.0, "frequency_hz": m.frequency_hz})).collect::<Vec<_>>(),
        "rows": request.rows.iter().map(quantity_json).collect::<Vec<_>>(),
        "contributions": request.periodic.iter().map(|c| json!({
            "mode": c.mode.0, "direction": c.direction.as_str(), "sign_status": signed, "values": c.values,
        })).collect::<Vec<_>>(),
        "rigid_responses": request.rigid.iter().map(|r| json!({
            "direction": r.direction.as_str(),
            "periodic_clusters_left_out": r.periodic_clusters_left_out.iter().map(|c| c.0).collect::<Vec<_>>(),
            "sign_status": signed,
            "values": r.values,
        })).collect::<Vec<_>>(),
    })
}

/// Decodes an input document to a request. A document that does not decode (malformed JSON, a
/// missing or unknown member, a wrong schema) is an `Err` with the reason; a document that
/// decodes but is refused is `Ok(Err(refusals))`.
pub fn request_from_json(text: &str) -> Result<(String, Result<CombinationRequest, Vec<Refusal>>), String> {
    let doc: InputDocument = serde_json::from_str(text).map_err(|e| format!("input does not decode: {e}"))?;
    if doc.schema != INPUT_SCHEMA {
        return Err(format!("input schema is `{}`, not `{INPUT_SCHEMA}`", doc.schema));
    }
    Ok((doc.provenance.clone(), request_from(&doc)))
}

/// Combines an input document. A document that does not decode is an `Err`; a combined or
/// refused document is the `Ok` output.
pub fn combine_json(text: &str) -> Result<Value, String> {
    let (provenance, request) = request_from_json(text)?;
    let outcome = request.and_then(|r| combine(&r));
    let mut out = json!({
        "schema": OUTPUT_SCHEMA,
        "statement": COMBINATION_STATEMENT,
        "provenance": provenance,
    });
    match outcome {
        Ok(combination) => {
            out["outcome"] = json!("combined");
            if let (Value::Object(o), Value::Object(c)) = (&mut out, combination_json(&combination)) {
                o.extend(c);
            }
        }
        Err(refusals) => {
            out["outcome"] = json!("refused");
            out["refusals"] = json!(refusals.iter().map(refusal_json).collect::<Vec<_>>());
        }
    }
    Ok(out)
}
