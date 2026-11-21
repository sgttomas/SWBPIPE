//! The interface's texts against the schema and the contract, the summary outcome's
//! serialised form, and the digest's written form.

use open_pipe_stress_export_decision::{
    BlockingFinding, ContentRule, ExportContext, FindingClass, FindingCount, ReasonCode,
    RouteRequest, RuleError, Sha256Digest, SummaryOutcome, WithheldReason,
};
use serde_json::{json, Value};

const SCHEMA: &str = include_str!("../../../../schemas/redaction_export_controls.schema.yaml");
const CONTRACT: &str = include_str!("../../../../docs/security/redaction_export_controls.md");

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
fn export_contexts_are_the_schemas_and_the_contracts_in_order() {
    let ours = texts(ExportContext::ALL, ExportContext::as_str);
    assert_eq!(ours, schema_enum("ExportContext"));
    // The contract's table of export contexts: its first column, in order.
    let section = CONTRACT
        .split("## Export Contexts")
        .nth(1)
        .and_then(|rest| rest.split("\n## ").next())
        .expect("the contract's Export Contexts section");
    let table: Vec<String> = section
        .lines()
        .filter_map(|line| line.strip_prefix("| `"))
        .map(|rest| rest.split('`').next().expect("context cell").to_string())
        .collect();
    assert_eq!(ours, table);
}

#[test]
fn reason_codes_and_finding_classes_are_the_schemas_in_order() {
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
    }
    for class in FindingClass::ALL {
        assert_eq!(FindingClass::from_name(class.as_str()), Some(*class));
        assert_eq!(serde_json::to_value(class).unwrap(), json!(class.as_str()));
    }
    for reason in WithheldReason::ALL {
        assert_eq!(WithheldReason::from_name(reason.as_str()), Some(*reason));
        assert_eq!(serde_json::to_value(reason).unwrap(), json!(reason.as_str()));
    }
    assert_eq!(ExportContext::from_name("Local_Private"), None);
    assert_eq!(ReasonCode::from_name("safe_public_metadata"), None);
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
        finding_counts: vec![FindingCount {
            code: ReasonCode::PrivateLocalAllowedWithWarning,
            class: FindingClass::PrivateDataWarning,
            count: 2,
        }],
        blocking_findings: Vec::new(),
        blocking_finding_total: 0,
    }
}

const OUTCOME_MEMBERS: [&str; 11] = [
    "blocking_finding_total",
    "blocking_findings",
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
