//! The interface's texts against the schema, the summary outcome's serialised form and
//! its admission, the order of its finding counts, and the digest's written form.

use open_pipe_stress_export_decision::{
    BlockingFinding, ContentRule, ExportContext, FindingClass, FindingCount, ReasonCode,
    RouteRequest, RuleError, Sha256Digest, SummaryOutcome, WithheldReason,
};
use serde_json::{json, Value};

const SCHEMA: &str = include_str!("../../../../schemas/redaction_export_controls.schema.yaml");

fn schema_enum(name: &str) -> Vec<String> {
    let schema: Value = serde_json::from_str(SCHEMA).expect("the schema is JSON text");
    schema["$defs"][name]["enum"]
        .as_array()
        .unwrap_or_else(|| panic!("schema $defs.{name}.enum"))
        .iter()
        .map(|text| text.as_str().expect("enum text").to_string())
        .collect()
}

fn texts<T: Copy>(all: &[T], as_str: fn(T) -> &'static str) -> Vec<String> {
    all.iter().map(|value| as_str(*value).to_string()).collect()
}

#[test]
fn contexts_reason_codes_and_finding_classes_are_the_schemas_in_order() {
    assert_eq!(texts(ExportContext::ALL, ExportContext::as_str), schema_enum("ExportContext"));
    assert_eq!(texts(ReasonCode::ALL, ReasonCode::as_str), schema_enum("ReasonCode"));
    assert_eq!(texts(FindingClass::ALL, FindingClass::as_str), schema_enum("FindingClass"));
}

#[test]
fn withheld_reasons_are_the_three_texts() {
    assert_eq!(
        texts(WithheldReason::ALL, WithheldReason::as_str),
        ["decision_blocked", "lossless_materialization_withheld", "controlled_copy_differs"]
    );
}

#[test]
fn every_name_reads_back_and_serialises_as_its_text() {
    for context in ExportContext::ALL {
        assert_eq!(ExportContext::from_name(context.as_str()), Some(*context));
        assert_eq!(serde_json::to_value(context).unwrap(), json!(context.as_str()));
        assert_eq!(context.to_string(), context.as_str());
    }
    for code in ReasonCode::ALL {
        assert_eq!(ReasonCode::from_name(code.as_str()), Some(*code));
        assert_eq!(serde_json::to_value(code).unwrap(), json!(code.as_str()));
        assert_eq!(code.to_string(), code.as_str());
    }
    for class in FindingClass::ALL {
        assert_eq!(FindingClass::from_name(class.as_str()), Some(*class));
        assert_eq!(serde_json::to_value(class).unwrap(), json!(class.as_str()));
        assert_eq!(class.to_string(), class.as_str());
    }
    for reason in WithheldReason::ALL {
        assert_eq!(WithheldReason::from_name(reason.as_str()), Some(*reason));
        assert_eq!(serde_json::to_value(reason).unwrap(), json!(reason.as_str()));
        assert_eq!(reason.to_string(), reason.as_str());
    }
    assert_eq!(ExportContext::from_name("Local_Private"), None);
    assert_eq!(ReasonCode::from_name("safe_public_metadata"), None);
    assert_eq!(FindingClass::from_name("safe_metadata"), None);
    assert_eq!(WithheldReason::from_name("DECISION_BLOCKED"), None);
}

const DIGEST: &str = "sha256:00ff10a9b3c4d5e6f708192a3b4c5d6e7f8091a2b3c4d5e6f708192a3b4c5d6e";

#[test]
fn a_digest_is_written_sha256_and_64_lowercase_hex_digits() {
    let digest = Sha256Digest::parse(DIGEST).expect("a well-formed digest");
    assert_eq!(digest.as_bytes()[..3], [0x00, 0xff, 0x10]);
    assert_eq!(digest.to_string(), DIGEST);
    assert_eq!(serde_json::to_value(digest).unwrap(), json!(DIGEST));
    assert_eq!(Sha256Digest::from_bytes(*digest.as_bytes()), digest);
    assert_eq!(Sha256Digest::from_bytes([0; 32]).to_string(), format!("sha256:{}", "0".repeat(64)));
    let hex = &DIGEST["sha256:".len()..];
    for refused in [
        hex.to_string(),
        format!("SHA256:{hex}"),
        format!("sha256:{}", hex.to_uppercase()),
        format!("sha256:{}", &hex[..63]),
        format!("sha256:{hex}0"),
        format!("sha256:{}g", &hex[..63]),
        format!("sha256: {}", &hex[..63]),
        format!("sha256:{}é", &hex[..62]),
        String::new(),
    ] {
        assert_eq!(Sha256Digest::parse(&refused), None, "{refused:?}");
    }
}

fn outcome() -> SummaryOutcome {
    SummaryOutcome {
        route_id: "INVENTED-ROUTE-001".to_string(),
        export_context: ExportContext::LocalPrivate,
        explicit_local_private_intent: true,
        rule_revision: "invented-revision".to_string(),
        text_sha256: Sha256Digest::parse(DIGEST).unwrap(),
        destination: None,
        withheld: false,
        withheld_reason: None,
        decision_total: 5,
        finding_counts: vec![FindingCount {
            code: ReasonCode::PrivateLocalAllowedWithWarning,
            class: FindingClass::PrivateDataWarning,
            count: 2,
        }],
        blocking_findings: Vec::new(),
        blocking_finding_total: 0,
    }
}

const OUTCOME_MEMBERS: [&str; 12] = [
    "blocking_finding_total",
    "blocking_findings",
    "decision_total",
    "destination",
    "explicit_local_private_intent",
    "export_context",
    "finding_counts",
    "route_id",
    "rule_revision",
    "text_sha256",
    "withheld",
    "withheld_reason",
];

fn members(value: &Value) -> Vec<&str> {
    let mut names: Vec<&str> = value.as_object().expect("object").keys().map(String::as_str).collect();
    names.sort_unstable();
    names
}

#[test]
fn a_summary_outcome_serialises_every_member_with_null_for_the_absent() {
    let admitted = serde_json::to_value(outcome()).unwrap();
    assert_eq!(members(&admitted), OUTCOME_MEMBERS);
    assert_eq!(
        admitted,
        json!({
            "route_id": "INVENTED-ROUTE-001",
            "export_context": "local_private",
            "explicit_local_private_intent": true,
            "rule_revision": "invented-revision",
            "text_sha256": DIGEST,
            "destination": null,
            "withheld": false,
            "withheld_reason": null,
            "decision_total": 5,
            "finding_counts": [{"code": "PRIVATE_LOCAL_ALLOWED_WITH_WARNING", "class": "PRIVATE_DATA_WARNING", "count": 2}],
            "blocking_findings": [],
            "blocking_finding_total": 0
        })
    );

    let mut withheld = outcome().with_destination("invented destination");
    withheld.withheld = true;
    withheld.withheld_reason = Some(WithheldReason::DecisionBlocked);
    withheld.blocking_findings = vec![BlockingFinding {
        reason_code: ReasonCode::ProtectedContentBlocked,
        path: "$.invented".to_string(),
    }];
    withheld.blocking_finding_total = 3;
    let withheld = serde_json::to_value(withheld).unwrap();
    assert_eq!(members(&withheld), OUTCOME_MEMBERS);
    assert_eq!(withheld["destination"], json!("invented destination"));
    assert_eq!(withheld["withheld_reason"], json!("decision_blocked"));
    assert_eq!(
        withheld["blocking_findings"],
        json!([{"reason_code": "PROTECTED_CONTENT_BLOCKED", "path": "$.invented"}])
    );
    assert_eq!(withheld["blocking_finding_total"], json!(3));
}

#[test]
fn only_a_coherent_admitted_outcome_may_expose() {
    assert!(outcome().may_expose());

    let mut withheld = outcome();
    withheld.withheld = true;
    withheld.withheld_reason = Some(WithheldReason::ControlledCopyDiffers);
    assert!(!withheld.may_expose());

    // Outcomes that say "not withheld" and contradict it: none exposes.
    let mut reason_present = outcome();
    reason_present.withheld_reason = Some(WithheldReason::LosslessMaterializationWithheld);
    assert!(!reason_present.may_expose());

    let mut counted = outcome();
    counted.blocking_finding_total = 1;
    assert!(!counted.may_expose());

    let mut listed = outcome();
    listed.blocking_findings = vec![BlockingFinding {
        reason_code: ReasonCode::ProtectedContentBlocked,
        path: "$.invented".to_string(),
    }];
    assert!(!listed.may_expose());
}

#[test]
fn finding_counts_are_ordered_by_code_text_then_class_text() {
    let count = |code, class| FindingCount { code, class, count: 1 };
    // The schema lists SAFE_PUBLIC_METADATA first; by text it follows PRIVATE_DATA_REDACTED.
    assert_eq!(ReasonCode::ALL[0], ReasonCode::SafePublicMetadata);
    let mut counts = vec![
        count(ReasonCode::SafePublicMetadata, FindingClass::SafeMetadata),
        count(ReasonCode::PrivateDataRedacted, FindingClass::ProvenanceWarning),
        count(ReasonCode::PrivateDataRedacted, FindingClass::IpBoundaryWarning),
        count(ReasonCode::ConcretePathRedacted, FindingClass::PrivateDataWarning),
    ];
    counts.sort_by(FindingCount::summary_order);
    let ordered: Vec<(&str, &str)> = counts.iter().map(|entry| (entry.code.as_str(), entry.class.as_str())).collect();
    assert_eq!(
        ordered,
        [
            ("CONCRETE_PATH_REDACTED", "PRIVATE_DATA_WARNING"),
            ("PRIVATE_DATA_REDACTED", "IP_BOUNDARY_WARNING"),
            ("PRIVATE_DATA_REDACTED", "PROVENANCE_WARNING"),
            ("SAFE_PUBLIC_METADATA", "SAFE_METADATA"),
        ]
    );
}

#[test]
fn a_refusal_has_the_contracts_code() {
    let refusals = [
        RuleError::UserReleaseRouteRequired {
            route_id: "INVENTED-ROUTE-001".to_string(),
            export_context: ExportContext::UserReleasedReport,
        },
        RuleError::UserReleaseIntentOutsideContext {
            export_context: ExportContext::PublicReport,
        },
    ];
    for refusal in refusals {
        assert_eq!(refusal.code(), "USER_RELEASE_ROUTE_REQUIRED");
        assert!(refusal.to_string().starts_with("USER_RELEASE_ROUTE_REQUIRED: "), "{refusal}");
    }
}

/// A stand-in, as a caller's own tests use one: the trait is implementable and usable as a
/// trait object.
struct StandIn;

impl ContentRule for StandIn {
    fn rule_revision(&self) -> &'static str {
        "invented-revision"
    }

    fn decide_summary(
        &self,
        _content: &Value,
        request: &RouteRequest<'_>,
        content_sha256: Sha256Digest,
        _blocking_bound: usize,
    ) -> Result<SummaryOutcome, RuleError> {
        Ok(SummaryOutcome {
            route_id: request.route_id.to_string(),
            export_context: request.export_context,
            explicit_local_private_intent: request.explicit_local_private_intent,
            rule_revision: self.rule_revision().to_string(),
            text_sha256: content_sha256,
            ..outcome()
        })
    }
}

#[test]
fn the_rule_is_usable_as_a_trait_object() {
    let rule: &dyn ContentRule = &StandIn;
    let request = RouteRequest {
        route_id: "INVENTED-ROUTE-002",
        export_context: ExportContext::LocalPrivate,
        explicit_local_private_intent: false,
        require_lossless_materialization: true,
    };
    let decided = rule
        .decide_summary(&json!({}), &request, Sha256Digest::from_bytes([7; 32]), 10)
        .unwrap();
    assert_eq!(decided.route_id, "INVENTED-ROUTE-002");
    assert_eq!(decided.text_sha256, Sha256Digest::from_bytes([7; 32]));
    assert_eq!(decided.rule_revision, rule.rule_revision());
}
