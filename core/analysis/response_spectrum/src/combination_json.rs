//! The JSON input and output of the combinations (unit S4): the form of the
//! `combine_contributions` example and of the contract fixtures under `fixtures/combination/`.
//!
//! This is the unit's own interchange. It is not the `spectrum-1` publication, its profile or
//! its readers (REQ-006), which a later unit defines and which may take these identities and
//! tags. An input document carries only supplied values: the method choices, the damping ratio,
//! the modal metadata, the rows and the signed contributions, with a free-text `provenance`
//! that says where they came from. The output echoes that provenance unchanged.
//!
//! Input (`openpipestress.spectrum_combination.input/v2`), every member required except as
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
//! - `rows`: the signed rows, `{"quantity", "entity_kind", <entity fields>, "end" (end actions
//!   only), "component", "unit", "frame", "sign_convention"}`. The entity fields are `node`
//!   (a node displacement; a constraint reaction, with `holders`), `member` and `member_family`
//!   (an element end action), `connector` (connector rows; a tie rod, with `rod`) and `support`
//!   (a compliant support's force). The entity kind, unit, frame and sign convention must be the
//!   ones the quantity has; a row is identified by these fields and carries no key;
//! - `contributions`: `{"mode", "direction", "sign_status": "signed_contribution", "values"}`,
//!   one value per row;
//! - `rigid_responses`: `{"direction", "periodic_clusters_left_out", "sign_status":
//!   "signed_contribution", "values"}`, empty without the correction.
//!
//! Output (`openpipestress.spectrum_combination.output/v2`): the statement, the provenance,
//! `outcome` (`combined` or `refused`) with the refusals, and when combined the rules, the
//! cluster statement, the correlation coefficient of each unordered pair of distinct periodic
//! clusters, the groups or coupled pairs the rule used, the signed rows (the input rows, to which
//! the signed cluster sums of each periodic cluster of more than one mode are aligned) and the
//! unsigned combined rows, which state their frame and carry no sign convention. A `label` beside
//! a row or in a refusal is composed for reading only; nothing parses it.
use crate::combine::combine;
use crate::contract::{
    ClusterIndex, Combination, CombinationRequest, ConnectorComponent, Direction, DirectionalRule, DisplacementComponent,
    End, ForceComponent, Frame, MemberFamily, MissingMass, ModalContribution, ModalMode, ModalRule, ModeIndex, Quantity,
    ReactionComponent, RigidContribution, RigidRule, SignConvention, SignStatus, COMBINATION_STATEMENT,
};
use crate::refusal::{Refusal, RefusalCode};
use serde::Deserialize;
use serde_json::{json, Value};

pub const INPUT_SCHEMA: &str = "openpipestress.spectrum_combination.input/v2";
pub const OUTPUT_SCHEMA: &str = "openpipestress.spectrum_combination.output/v2";

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
    quantity: String,
    entity_kind: String,
    #[serde(default)]
    node: Option<String>,
    #[serde(default)]
    member: Option<String>,
    #[serde(default)]
    member_family: Option<String>,
    #[serde(default)]
    connector: Option<String>,
    #[serde(default)]
    rod: Option<String>,
    #[serde(default)]
    support: Option<String>,
    #[serde(default)]
    holders: Option<Vec<String>>,
    #[serde(default)]
    end: Option<String>,
    component: String,
    unit: String,
    frame: String,
    sign_convention: String,
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

/// Reads one signed row. The quantity kind selects the entity fields it takes; a missing field,
/// a field another kind takes, a component, end or family that is not one of the contract's, and
/// an entity kind, unit, frame or sign convention other than the quantity's are refused by name
/// (`SPECTRUM_ROW_INVALID`), naming the row by its position.
fn row(index: usize, input: &RowInput, refusals: &mut Vec<Refusal>) -> Option<Quantity> {
    let object = format!("rows[{index}]");
    let before = refusals.len();
    let mut bad = |field: &str, detail: String| refusals.push(refusal(RefusalCode::RowInvalid, &object, field, detail));
    let takes: &[&str] = match input.quantity.as_str() {
        "node_displacement" => &["node"],
        "element_end_action" => &["member", "member_family", "end"],
        "connector_generalized_force" => &["connector"],
        "connector_end_action" => &["connector", "end"],
        "tie_rod_force" => &["connector", "rod"],
        "constraint_reaction" => &["node", "holders"],
        "compliant_support_force" => &["support"],
        other => {
            bad("quantity", format!("`{other}` is not a quantity kind"));
            return None;
        }
    };
    let present = [
        ("node", input.node.is_some()),
        ("member", input.member.is_some()),
        ("member_family", input.member_family.is_some()),
        ("connector", input.connector.is_some()),
        ("rod", input.rod.is_some()),
        ("support", input.support.is_some()),
        ("holders", input.holders.is_some()),
        ("end", input.end.is_some()),
    ];
    for (field, is_present) in present {
        if is_present && !takes.contains(&field) {
            bad(field, format!("a {} has no `{field}`", input.quantity));
        } else if !is_present && takes.contains(&field) {
            bad(field, format!("a {} requires `{field}`", input.quantity));
        }
    }
    let text = |v: &Option<String>| v.clone().unwrap_or_default();
    let end = match input.end.as_deref() {
        None => None,
        Some(e) => match End::parse(e) {
            Some(end) => Some(end),
            None => {
                bad("end", format!("`{e}` is not `i` or `j`"));
                None
            }
        },
    };
    let mut component_of = |parsed: Option<()>| {
        if parsed.is_none() {
            bad("component", format!("`{}` is not a component of a {}", input.component, input.quantity));
        }
    };
    let quantity = match input.quantity.as_str() {
        "node_displacement" => {
            let c = DisplacementComponent::parse(&input.component);
            component_of(c.map(|_| ()));
            c.map(|component| Quantity::NodeDisplacement { node: text(&input.node), component })
        }
        "element_end_action" => {
            let c = ForceComponent::parse(&input.component);
            component_of(c.map(|_| ()));
            let family = input.member_family.as_deref().and_then(MemberFamily::parse);
            let frame = Frame::parse(&input.frame);
            match (c, family, frame, end) {
                (Some(component), Some(family), Some(frame), Some(end)) => {
                    Some(Quantity::ElementEndAction { member: text(&input.member), family, end, component, frame })
                }
                _ => None,
            }
        }
        "connector_generalized_force" => {
            let c = ConnectorComponent::parse(&input.component);
            component_of(c.map(|_| ()));
            c.map(|component| Quantity::ConnectorGeneralizedForce { connector: text(&input.connector), component })
        }
        "connector_end_action" => {
            let c = ForceComponent::parse(&input.component);
            component_of(c.map(|_| ()));
            match (c, end) {
                (Some(component), Some(end)) => Some(Quantity::ConnectorEndAction { connector: text(&input.connector), end, component }),
                _ => None,
            }
        }
        "tie_rod_force" => {
            let ok = input.component == crate::contract::TIE_ROD_COMPONENT;
            component_of(ok.then_some(()));
            ok.then(|| Quantity::TieRodForce { connector: text(&input.connector), rod: text(&input.rod) })
        }
        "constraint_reaction" => {
            let c = ReactionComponent::parse(&input.component);
            component_of(c.map(|_| ()));
            c.map(|component| Quantity::ConstraintReaction {
                node: text(&input.node),
                component,
                holders: input.holders.clone().unwrap_or_default(),
            })
        }
        _ => {
            let c = ReactionComponent::parse(&input.component);
            component_of(c.map(|_| ()));
            c.map(|component| Quantity::CompliantSupportForce { support: text(&input.support), component })
        }
    };
    if input.member_family.is_some() && input.member_family.as_deref().and_then(MemberFamily::parse).is_none() {
        bad("member_family", format!("`{}` is not `straight` or `arc`", text(&input.member_family)));
    }
    if Frame::parse(&input.frame).is_none() {
        bad("frame", format!("`{}` is not a frame of the spectrum result family", input.frame));
    }
    let quantity = quantity?;
    if input.entity_kind != quantity.entity_kind() {
        bad("entity_kind", format!("the entity of a {} is a {}, not `{}`", quantity.kind(), quantity.entity_kind(), input.entity_kind));
    }
    if Frame::parse(&input.frame).is_some_and(|f| f != quantity.frame()) {
        bad("frame", format!("a {} is stated in {} only, not `{}`", quantity.kind(), quantity.frame().as_str(), input.frame));
    }
    if input.unit != quantity.unit().catalog_id() {
        bad("unit", format!("`{}` is not the unit of this row, `{}`", input.unit, quantity.unit().catalog_id()));
    }
    if SignConvention::parse(&input.sign_convention) != Some(quantity.sign_convention()) {
        bad("sign_convention", format!("`{}` is not the sign convention of a {}, `{}`", input.sign_convention, quantity.kind(),
            quantity.sign_convention().as_str()));
    }
    (refusals.len() == before).then_some(quantity)
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
    let rows: Vec<Option<Quantity>> = doc.rows.iter().enumerate().map(|(i, r)| row(i, r, &mut refusals)).collect();
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

/// A row's identity fields, unit and frame as JSON, with its readable label. No sign convention
/// or sign status: [`signed_row_json`] adds the convention of a signed row.
pub fn quantity_json(quantity: &Quantity) -> Value {
    let mut value = json!({
        "quantity": quantity.kind(),
        "entity_kind": quantity.entity_kind(),
        "component": quantity.component_name(),
        "unit": quantity.unit().catalog_id(),
        "frame": quantity.frame().as_str(),
    });
    match quantity {
        Quantity::NodeDisplacement { node, .. } => value["node"] = json!(node),
        Quantity::ElementEndAction { member, family, .. } => {
            value["member"] = json!(member);
            value["member_family"] = json!(family.as_str());
        }
        Quantity::ConnectorGeneralizedForce { connector, .. } | Quantity::ConnectorEndAction { connector, .. } => {
            value["connector"] = json!(connector)
        }
        Quantity::TieRodForce { connector, rod } => {
            value["connector"] = json!(connector);
            value["rod"] = json!(rod);
        }
        Quantity::ConstraintReaction { node, holders, .. } => {
            value["node"] = json!(node);
            value["holders"] = json!(holders);
        }
        Quantity::CompliantSupportForce { support, .. } => value["support"] = json!(support),
    }
    if let Some(end) = quantity.end() {
        value["end"] = json!(end.as_str());
    }
    value
}

/// A signed row: the identity fields, unit and frame with the quantity's sign convention.
pub fn signed_row_json(quantity: &Quantity) -> Value {
    let mut value = quantity_json(quantity);
    value["sign_convention"] = json!(quantity.sign_convention().as_str());
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
        "cqc_correlation": c.correlation.as_ref().map(|pairs| pairs.iter().map(|p| json!({
            "clusters": [p.first.0, p.second.0],
            "rho": p.rho,
        })).collect::<Vec<_>>()),
        "grouping_groups": c.groups.as_ref().map(|g| g.iter().map(|group| group.iter().map(|i| i.0).collect::<Vec<_>>()).collect::<Vec<_>>()),
        "ten_percent_pairs": c.close_pairs.as_ref().map(|p| p.iter().map(|(a, b)| [a.0, b.0]).collect::<Vec<_>>()),
        "cluster_sums": c.cluster_sums.iter().map(|s| json!({
            "cluster": s.cluster.0,
            "direction": s.direction.as_str(),
            "sign_status": SignStatus::SignedClusterSum.as_str(),
            "values": s.values,
        })).collect::<Vec<_>>(),
        "signed_rows": c.rows.iter().map(|r| {
            let mut v = signed_row_json(&r.quantity);
            v["label"] = json!(r.quantity.label());
            v
        }).collect::<Vec<_>>(),
        "rows": c.rows.iter().map(|r| {
            let mut v = quantity_json(&r.quantity);
            v["label"] = json!(r.quantity.label());
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
        "rows": request.rows.iter().map(signed_row_json).collect::<Vec<_>>(),
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
