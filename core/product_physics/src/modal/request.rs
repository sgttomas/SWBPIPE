//! The modal analysis request (DEL-04-09 Design §1.4) and its request-level
//! refusals: the required extraction bound, the state case where §1.4
//! requires one, the modulus-basis conflict and the weight-to-mass
//! acceleration. No value is defaulted: an absent input is refused by name
//! where it is required, and recorded as absent otherwise.
//!
//! Every member is a request member, as `solver_mode` is (D-h);
//! none is a model-document member, so no document version changes.
use super::{
    MODAL_EXTRACTION_BOUND_INVALID, MODAL_EXTRACTION_BOUND_REQUIRED, MODAL_MODULUS_BASIS_CONFLICT,
    MODAL_STATE_CASE_REQUIRED, MODAL_STATE_CASE_UNAVAILABLE, MODAL_WEIGHT_ACCELERATION_INVALID,
};
use crate::{diag, normalize_quantity, ClosedMaterialRecord, Diagnostic, MaterialInput, PreviewModel, Quantity};
use open_pipe_stress_units::Dimension;
use serde::Deserialize;

/// The modal analysis request: the static request's `{model, materials}`
/// and the modal members of Design §1.4.
#[derive(Debug, Clone)]
pub(crate) struct ModalPreviewRequest {
    pub(crate) model: PreviewModel,
    pub(crate) materials: Vec<MaterialInput>,
    pub(crate) mode_count: Option<u32>,
    pub(crate) frequency_cutoff: Option<Quantity>,
    pub(crate) state_case: Option<String>,
    pub(crate) modulus_basis_ref: Option<String>,
    pub(crate) modulus_basis_temperature: Option<Quantity>,
    pub(crate) weight_to_mass_acceleration: Option<Quantity>,
    pub(crate) include_dynamic_only_restraints: Option<bool>,
}

/// Closed wire form: an undeclared request member refuses by name, and each
/// material record is read as the static request reads it, so a request-level
/// `expansion_laws` key is recorded rather than dropped (in
/// `parse_modal_request`).
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ModalPreviewRequestWire {
    model: PreviewModel,
    #[serde(default)]
    materials: Vec<ClosedMaterialRecord<serde::de::IgnoredAny>>,
    #[serde(default)]
    mode_count: Option<u32>,
    #[serde(default, deserialize_with = "crate::model_fields::closed_optional_quantity")]
    frequency_cutoff: Option<Quantity>,
    #[serde(default)]
    state_case: Option<String>,
    #[serde(default)]
    modulus_basis_ref: Option<String>,
    #[serde(default, deserialize_with = "crate::model_fields::closed_optional_quantity")]
    modulus_basis_temperature: Option<Quantity>,
    #[serde(default, deserialize_with = "crate::model_fields::closed_optional_quantity")]
    weight_to_mass_acceleration: Option<Quantity>,
    #[serde(default)]
    include_dynamic_only_restraints: Option<bool>,
}

/// The request's validated inputs, normalized through the unit catalog.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RequestInputs {
    pub(crate) mode_count: Option<u32>,
    /// f_c in Hz.
    pub(crate) frequency_cutoff_hz: Option<f64>,
    pub(crate) state_case: Option<String>,
    /// a in m/s², finite and positive; only its magnitude is ever used.
    pub(crate) weight_to_mass_acceleration: Option<f64>,
    pub(crate) include_dynamic_only_restraints: Option<bool>,
}

/// Why §1.4 requires a state case for this model, or `None`.
pub(crate) fn state_case_requirement(model: &PreviewModel) -> Option<String> {
    let mut reasons = Vec::new();
    if crate::case_state::is_load_state(model) {
        reasons.push("the document is a load/reference-state document".to_string());
    }
    let nonlinear: Vec<&str> =
        model.supports.iter().filter(|support| support.nonlinear.is_some()).map(|support| support.id.as_str()).collect();
    if !nonlinear.is_empty() {
        reasons.push(format!("the model has nonlinear supports ({})", nonlinear.join(", ")));
    }
    (!reasons.is_empty()).then(|| reasons.join(" and "))
}

fn request_diag(field: &str, code: &str, message: String, refs: Vec<String>) -> Diagnostic {
    diag(&format!("diagnostic:modal:request:{field}"), code, "blocking", message, refs)
}

/// The quantity in the catalog's canonical unit of `dimension`, or the reason
/// it is not one (unit outside the catalog or of another dimension, a
/// non-finite value or one that does not convert).
fn normalized_request_quantity(quantity: &Quantity, dimension: Dimension) -> Result<f64, String> {
    if !quantity.value.is_finite() {
        return Err(format!("its value {} is not finite", quantity.value));
    }
    let mut converted = quantity.clone();
    let mut scratch = Vec::new();
    normalize_quantity(&mut converted, dimension, "diagnostic:modal:request:quantity", Vec::new(), &mut scratch);
    if let Some(finding) = scratch.first() {
        return Err(format!(
            "its unit `{}` is not a {} unit of the units catalog ({})",
            quantity.unit,
            dimension.as_str(),
            finding.message
        ));
    }
    if !converted.value.is_finite() {
        return Err(format!("{} {} does not normalize to a finite value", quantity.value, quantity.unit));
    }
    Ok(converted.value)
}

impl ModalPreviewRequest {
    /// Parses the request value as the static value entry does: the model's
    /// closed records first (an undeclared model member refuses by name), then
    /// the closed request.
    /// The request is read through its derived closed wire form and converted
    /// here, with no hand-written `Deserialize` implementation.
    pub(crate) fn parse_modal_request(actual_request: serde_json::Value) -> Result<Self, String> {
        if let Some(refusal) = crate::model_fields::unknown_field_refusal(&actual_request) {
            return Err(refusal);
        }
        let wire: ModalPreviewRequestWire =
            serde_json::from_value(actual_request).map_err(|error| format!("modal request does not parse: {error}"))?;
        let mut model = wire.model;
        let mut materials = Vec::with_capacity(wire.materials.len());
        for (index, record) in wire.materials.into_iter().enumerate() {
            if record.expansion_laws.is_authored() {
                model.request_material_expansion_laws.push(index);
            }
            materials.push(record.material);
        }
        Ok(Self {
            model,
            materials,
            mode_count: wire.mode_count,
            frequency_cutoff: wire.frequency_cutoff,
            state_case: wire.state_case,
            modulus_basis_ref: wire.modulus_basis_ref,
            modulus_basis_temperature: wire.modulus_basis_temperature,
            weight_to_mass_acceleration: wire.weight_to_mass_acceleration,
            include_dynamic_only_restraints: wire.include_dynamic_only_restraints,
        })
    }

    /// The request-level refusals of Design §1.4, all of them, in a fixed
    /// order; or the validated inputs.
    pub(crate) fn validate_modal_request(&self) -> Result<RequestInputs, Vec<Diagnostic>> {
        let mut found = Vec::new();
        if self.mode_count.is_none() && self.frequency_cutoff.is_none() {
            found.push(request_diag(
                "extraction-bound",
                MODAL_EXTRACTION_BOUND_REQUIRED,
                "a modal request names its extraction bound: a mode_count, a frequency_cutoff or both; no bound is defaulted, so nothing is extracted".into(),
                vec!["mode_count".into(), "frequency_cutoff".into()],
            ));
        }
        if self.mode_count == Some(0) {
            found.push(request_diag(
                "mode-count",
                MODAL_EXTRACTION_BOUND_INVALID,
                "mode_count is 0; a mode count is a positive integer".into(),
                vec!["mode_count".into()],
            ));
        }
        let mut frequency_cutoff_hz = None;
        if let Some(cutoff) = &self.frequency_cutoff {
            match normalized_request_quantity(cutoff, Dimension::Frequency) {
                Ok(hz) if hz > 0.0 => frequency_cutoff_hz = Some(hz),
                Ok(hz) => found.push(request_diag(
                    "frequency-cutoff",
                    MODAL_EXTRACTION_BOUND_INVALID,
                    format!("frequency_cutoff {} {} is {hz} Hz; a frequency cut-off is positive", cutoff.value, cutoff.unit),
                    vec!["frequency_cutoff".into()],
                )),
                Err(reason) => found.push(request_diag(
                    "frequency-cutoff",
                    MODAL_EXTRACTION_BOUND_INVALID,
                    format!("frequency_cutoff is not a frequency: {reason}"),
                    vec!["frequency_cutoff".into(), cutoff.unit.clone()],
                )),
            }
        }
        if let Some(why) = state_case_requirement(&self.model) {
            if self.state_case.is_none() {
                found.push(request_diag(
                    "state-case",
                    MODAL_STATE_CASE_REQUIRED,
                    format!("a modal request names its state_case because {why}; the stiffness, mass and support states come from a named solved case, and none is defaulted"),
                    vec!["state_case".into()],
                ));
            }
        }
        if let Some(case) = &self.state_case {
            let matching = self.model.load_cases.iter().filter(|load_case| &load_case.id == case).count();
            if case.trim().is_empty() || matching != 1 {
                let why = if matching == 0 { "names no load case of the model" } else { "names more than one load case" };
                found.push(request_diag(
                    "state-case",
                    MODAL_STATE_CASE_UNAVAILABLE,
                    format!("state_case `{case}` {why}; the modal model is defined by exactly one named case"),
                    vec!["state_case".into(), case.clone()],
                ));
            }
            for (field, present) in [
                ("modulus_basis_ref", self.modulus_basis_ref.is_some()),
                ("modulus_basis_temperature", self.modulus_basis_temperature.is_some()),
            ] {
                if present {
                    found.push(request_diag(
                        &format!("modulus-basis:{}", field.replace('_', "-")),
                        MODAL_MODULUS_BASIS_CONFLICT,
                        format!("the request names state_case `{case}` and {field}; with a state case the modulus comes from that case's resolved state, so a request modulus basis conflicts with it"),
                        vec!["state_case".into(), field.into()],
                    ));
                }
            }
        }
        let mut weight_to_mass_acceleration = None;
        if let Some(acceleration) = &self.weight_to_mass_acceleration {
            match normalized_request_quantity(acceleration, Dimension::Acceleration) {
                Ok(a) if a > 0.0 => weight_to_mass_acceleration = Some(a),
                Ok(a) => found.push(request_diag(
                    "weight-to-mass-acceleration",
                    MODAL_WEIGHT_ACCELERATION_INVALID,
                    format!("weight_to_mass_acceleration {} {} is {a} m/s^2; it is a positive acceleration", acceleration.value, acceleration.unit),
                    vec!["weight_to_mass_acceleration".into()],
                )),
                Err(reason) => found.push(request_diag(
                    "weight-to-mass-acceleration",
                    MODAL_WEIGHT_ACCELERATION_INVALID,
                    format!("weight_to_mass_acceleration is not an acceleration: {reason}"),
                    vec!["weight_to_mass_acceleration".into(), acceleration.unit.clone()],
                )),
            }
        }
        if !found.is_empty() {
            return Err(found);
        }
        Ok(RequestInputs {
            mode_count: self.mode_count,
            frequency_cutoff_hz,
            state_case: self.state_case.clone(),
            weight_to_mass_acceleration,
            include_dynamic_only_restraints: self.include_dynamic_only_restraints,
        })
    }
}
