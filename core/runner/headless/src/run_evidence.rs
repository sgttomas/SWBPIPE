//! Run evidence (DEL-10-05 REQ-017, AC-013): a member of every result the
//! local-private control passes, formed in the run from the request bytes the
//! runner read and the result it emits, so that a result paired with a request
//! or a model document it did not answer fails a check that needs only the kept
//! bytes.
//!
//! The evidence is not a signature: whoever can rewrite a result can rewrite its
//! evidence. Where a result came from is checked by running the same request on
//! the build the evidence names and comparing the result digest.
//!
//! Every digest but `request.raw_sha256` is SHA-256 over the canonical text of
//! the checked profile `openpipestress_jcs_ijson_v1`
//! (`open_pipe_stress_canonical_json::canonical_json_checked_v1_text`). A value
//! the profile refuses (an integer beyond 2^53, a duplicate member, a lone
//! surrogate, a number out of range) is stated as `{"unavailable": <code>}`, the
//! profile's refusal code without its detail, and is never hashed in another
//! form. `binding_sha256` is the domain-separated SHA-256 (domain
//! `openpipestress_runner_evidence_v1`) of the evidence less that member, in the
//! canonical crate's domain form (`checked_v1_domain_canonical` over
//! `DomainPayload::ObjectWithout`).
//!
//! Where the control acts: the evidence is formed only after the control has
//! passed the output (a blocked output carries none, nor does a run that exits
//! `2`), and the result digest is taken over the controlled output exactly as it
//! is emitted, less `run_evidence`.

use open_pipe_stress_canonical_json::{
    canonical_json, canonical_json_checked_v1_text, checked_v1_domain_canonical, ByteSink,
    DomainPayload, CHECKED_PROFILE_V1,
};
use open_pipe_stress_product_physics::{
    retained_profile, solver_component_name, solver_component_version, ProfileStatus,
};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};

/// The member of the controlled output that carries the evidence.
pub const MEMBER: &str = "run_evidence";
pub const EVIDENCE_VERSION: &str = "1.0.0";
pub const BINDING_DOMAIN: &str = "openpipestress_runner_evidence_v1";
/// RFC 6901 pointers to the model document a command's input carries.
pub const SOLVE_MODEL_POINTER: &str = "/solve/preview_model/model";
pub const SIZE_HANGERS_MODEL_POINTER: &str = "/request/model";
/// `model.absent` when the command's input carries no model document.
pub const ABSENT_NOT_CARRIED: &str = "COMMAND_CARRIES_NO_MODEL_DOCUMENT";
/// `model.absent` when the command carries one but this request has none.
pub const ABSENT_NOT_PRESENT: &str = "MODEL_DOCUMENT_NOT_IN_REQUEST";

/// The command, solver mode and intent flag as the runner ran them. The solver
/// mode is the one `solve` ran with (the default when none was given) and
/// `None` for every other command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Invocation<'a> {
    pub command: &'a str,
    pub solver_mode: Option<&'a str>,
    pub explicit_local_private_intent: bool,
}

/// The pointer to the model document `command`'s input carries, if any.
pub fn model_pointer(command: &str) -> Option<&'static str> {
    match command {
        "solve" => Some(SOLVE_MODEL_POINTER),
        "size-hangers" => Some(SIZE_HANGERS_MODEL_POINTER),
        _ => None,
    }
}

/// The runner's and the product's identity, from what the product already
/// exposes: the runner crate's manifest name and version; the product solver's
/// `solver_component_name` and `solver_component_version`; the build reference
/// `<name>@<version>` that `export-results` already binds
/// (`LinkedSolverIdentity::solver_build_ref`); and the retained build identity
/// (`retained_profile().identity()`) when this build is a registered one, else
/// `{"unavailable": "<ProfileStatus>"}`.
pub fn producer() -> Value {
    let product_name = solver_component_name();
    let product_version = solver_component_version();
    let build_identity = match retained_profile() {
        Ok(facts) => Value::String(facts.identity().to_string()),
        Err(status) => json!({"unavailable": match status {
            ProfileStatus::Missing => "RETAINED_BUILD_IDENTITY_MISSING",
            ProfileStatus::Stale => "RETAINED_BUILD_IDENTITY_UNREGISTERED",
            ProfileStatus::Registered => "RETAINED_BUILD_IDENTITY_UNREADABLE",
        }}),
    };
    json!({
        "runner": {
            "component_name": env!("CARGO_PKG_NAME"),
            "component_version": env!("CARGO_PKG_VERSION"),
        },
        "product": {
            "component_name": product_name,
            "component_version": product_version,
        },
        "build_ref": format!("{product_name}@{product_version}"),
        "retained_build_identity": build_identity,
    })
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

/// The profile's refusal code, without its detail (which can quote a value).
pub fn refusal_code(message: &str) -> String {
    message.split(':').next().unwrap_or(message).trim().to_string()
}

fn unavailable(message: &str) -> Value {
    json!({"unavailable": refusal_code(message)})
}

/// The request and model members, from the bytes as read.
fn request_and_model(raw: &[u8], command: &str) -> (Value, Value) {
    let checked = std::str::from_utf8(raw)
        .map_err(|_| "CHECKED-JSON-INVALID: input is not UTF-8".to_string())
        .and_then(canonical_json_checked_v1_text);
    let (request_digest, parsed) = match &checked {
        Ok(text) => (
            Value::String(sha256_hex(text.as_bytes())),
            // Canonical text in the profile parses back to the same value.
            Some(serde_json::from_str::<Value>(text).expect("canonical text is JSON")),
        ),
        Err(message) => (unavailable(message), None),
    };
    let request = json!({"raw_sha256": sha256_hex(raw), "canonical_sha256": request_digest});
    let model = match (model_pointer(command), &parsed, &checked) {
        (None, _, _) => json!({"absent": ABSENT_NOT_CARRIED}),
        (Some(pointer), Some(value), _) => match value.pointer(pointer) {
            Some(model) => json!({
                "pointer": pointer,
                "canonical_sha256": sha256_hex(canonical_json(model).as_bytes()),
            }),
            None => json!({"absent": ABSENT_NOT_PRESENT}),
        },
        // The request is outside the profile: its model document is not hashed
        // from another form of it.
        (Some(pointer), None, Err(message)) => {
            json!({"pointer": pointer, "canonical_sha256": unavailable(message)})
        }
        (Some(_), None, Ok(_)) => unreachable!("a checked request always parses"),
    };
    (request, model)
}

/// SHA-256 over the canonical text of the emitted controlled output less its
/// run evidence (`result` must not hold `run_evidence`).
fn result_digest(result: &Value) -> Value {
    let text = serde_json::to_string(result).expect("a JSON value serializes");
    match canonical_json_checked_v1_text(&text) {
        Ok(canonical) => Value::String(sha256_hex(canonical.as_bytes())),
        Err(message) => unavailable(&message),
    }
}

/// The binding digest, or `None` when the evidence holds a value the profile
/// refuses (never for evidence this module forms: it holds only strings,
/// booleans and nulls).
fn binding(evidence: &Map<String, Value>) -> Option<String> {
    let mut hasher = Sha256::new();
    checked_v1_domain_canonical(
        BINDING_DOMAIN,
        DomainPayload::ObjectWithout(evidence, "binding_sha256"),
        &mut ByteSink(|bytes: &[u8]| hasher.update(bytes)),
    )
    .ok()?;
    Some(hex(&hasher.finalize()))
}

fn assemble(
    raw: &[u8],
    invocation: &Value,
    producer: Value,
    result_without_evidence: &Value,
) -> Map<String, Value> {
    let command = invocation["command"].as_str().unwrap_or_default();
    let (request, model) = request_and_model(raw, command);
    let mut evidence = Map::new();
    evidence.insert("evidence_version".into(), json!(EVIDENCE_VERSION));
    evidence.insert("canonical_profile".into(), json!(CHECKED_PROFILE_V1));
    evidence.insert("invocation".into(), invocation.clone());
    evidence.insert("producer".into(), producer);
    evidence.insert("request".into(), request);
    evidence.insert("model".into(), model);
    evidence.insert(
        "result".into(),
        json!({"canonical_sha256": result_digest(result_without_evidence)}),
    );
    let digest = binding(&evidence).map_or(Value::Null, Value::String);
    evidence.insert("binding_sha256".into(), digest);
    evidence
}

/// The run evidence of one run: `raw` is the request as read, and
/// `controlled_output` the controlled output as it will be emitted, without
/// `run_evidence`. Call it only for an output the control passed.
pub fn form(raw: &[u8], invocation: Invocation<'_>, controlled_output: &Value) -> Value {
    let invocation = json!({
        "command": invocation.command,
        "solver_mode": invocation.solver_mode,
        "explicit_local_private_intent": invocation.explicit_local_private_intent,
    });
    Value::Object(assemble(raw, &invocation, producer(), controlled_output))
}

/// Check an emitted result against the request bytes it is said to answer,
/// from those bytes alone. `Err` lists every evidence member whose recomputed
/// value differs, by dotted name (`request.raw_sha256`,
/// `request.canonical_sha256`, `model`, `result.canonical_sha256`,
/// `binding_sha256`, and so on), or `run_evidence` when the member is missing or
/// not an object. The invocation and producer are taken from the evidence (they
/// cannot be recomputed from bytes); a change to either fails the binding.
pub fn verify(raw: &[u8], emitted: &Value) -> Result<(), Vec<String>> {
    let Some(object) = emitted.as_object() else {
        return Err(vec![MEMBER.to_string()]);
    };
    let Some(Value::Object(stated)) = object.get(MEMBER) else {
        return Err(vec![MEMBER.to_string()]);
    };
    let mut result = object.clone();
    result.remove(MEMBER);
    let invocation = stated.get("invocation").cloned().unwrap_or(Value::Null);
    let producer = stated.get("producer").cloned().unwrap_or(Value::Null);
    let mut expected = assemble(raw, &invocation, producer, &Value::Object(result));
    // The binding is checked over what the evidence states, so that a stated
    // member changed without its binding fails by `binding_sha256`. Evidence
    // the profile refuses binds nothing (`null` never equals a stated digest).
    expected.insert("binding_sha256".into(), binding(stated).map_or(Value::Null, Value::String));
    let mut differing = Vec::new();
    let mut keys: Vec<&String> = expected.keys().chain(stated.keys()).collect();
    keys.sort();
    keys.dedup();
    for key in keys {
        match (expected.get(key), stated.get(key)) {
            (Some(Value::Object(want)), Some(Value::Object(have)))
                if key == "request" || key == "result" =>
            {
                let mut inner: Vec<&String> = want.keys().chain(have.keys()).collect();
                inner.sort();
                inner.dedup();
                for name in inner {
                    if want.get(name) != have.get(name) {
                        differing.push(format!("{key}.{name}"));
                    }
                }
            }
            (want, have) if key == "binding_sha256" => {
                if !matches!(want, Some(Value::String(_))) || want != have {
                    differing.push(key.clone());
                }
            }
            (want, have) if want != have => differing.push(match key.as_str() {
                "model" => "model.canonical_sha256".to_string(),
                other => other.to_string(),
            }),
            _ => {}
        }
    }
    if differing.is_empty() {
        Ok(())
    } else {
        Err(differing)
    }
}
