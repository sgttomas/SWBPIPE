//! The interface's texts against the schema, the summary outcome's serialised form and
//! its admission, the order of its finding counts, the digest's written form, and a content
//! source as a caller implements one.

use std::borrow::Cow;
use std::cell::RefCell;

use open_pipe_stress_export_decision::{
    BlockingFinding, Content, ContentRule, ExportContext, FindingClass, FindingCount, Member, Peek,
    ReasonCode, RouteRequest, RuleError, Sha256Digest, SourceList, SourceObject, SummaryDecision,
    SummaryOutcome, WalkStopped, WithheldReason, MAX_SOURCE_NESTING,
};
use serde_json::{json, Map, Value};

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

fn decision() -> SummaryDecision {
    SummaryDecision {
        route_id: "INVENTED-ROUTE-001".to_string(),
        export_context: ExportContext::LocalPrivate,
        explicit_local_private_intent: true,
        rule_revision: "invented-revision".to_string(),
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

fn outcome() -> SummaryOutcome {
    decision().bound_to(Sha256Digest::parse(DIGEST).unwrap())
}

#[test]
fn binding_adds_the_digest_and_changes_nothing_else() {
    let decided = decision();
    let bound = decided.clone().bound_to(Sha256Digest::from_bytes([9; 32]));
    assert_eq!(bound.text_sha256, Sha256Digest::from_bytes([9; 32]));
    assert_eq!(bound.destination, None);
    assert_eq!(
        (
            &bound.route_id,
            bound.export_context,
            bound.explicit_local_private_intent,
            &bound.rule_revision,
            bound.withheld,
            bound.withheld_reason,
            bound.decision_total,
            &bound.finding_counts,
            &bound.blocking_findings,
            bound.blocking_finding_total,
        ),
        (
            &decided.route_id,
            decided.export_context,
            decided.explicit_local_private_intent,
            &decided.rule_revision,
            decided.withheld,
            decided.withheld_reason,
            decided.decision_total,
            &decided.finding_counts,
            &decided.blocking_findings,
            decided.blocking_finding_total,
        )
    );
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
fn a_refusal_has_its_code() {
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
    let source = RuleError::ContentSourceInconsistent { path: "$.invented".to_string() };
    assert_eq!(source.code(), "EXPORT_CONTENT_SOURCE_INCONSISTENT");
    assert!(source.to_string().starts_with("EXPORT_CONTENT_SOURCE_INCONSISTENT: "), "{source}");
    assert!(source.to_string().contains("$.invented"));
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
        _content: Content<'_>,
        request: &RouteRequest<'_>,
        _blocking_bound: usize,
    ) -> Result<SummaryDecision, RuleError> {
        Ok(SummaryDecision {
            route_id: request.route_id.to_string(),
            export_context: request.export_context,
            explicit_local_private_intent: request.explicit_local_private_intent,
            rule_revision: self.rule_revision().to_string(),
            ..decision()
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
    let content = json!({});
    let decided = rule
        .decide_summary(Content::from(&content), &request, 10)
        .unwrap()
        .bound_to(Sha256Digest::from_bytes([7; 32]));
    assert_eq!(decided.route_id, "INVENTED-ROUTE-002");
    assert_eq!(decided.text_sha256, Sha256Digest::from_bytes([7; 32]));
    assert_eq!(decided.rule_revision, rule.rule_revision());
    assert!(decided.may_expose());
}

// --- A content source, as a caller implements one, and a visitor, as a rule is one. ---

/// The order of canonical JSON and of a source's walk: UTF-16 code units.
fn utf16_order(left: &str, right: &str) -> std::cmp::Ordering {
    left.encode_utf16().cmp(right.encode_utf16())
}

/// A source over a held value that gives every object and every list by walk, and writes
/// what it hands over, as a producer does: `written` holds one line per value written.
struct Walked<'a> {
    value: &'a Value,
    written: &'a RefCell<Vec<String>>,
}

impl<'a> Walked<'a> {
    fn hand(&self, value: &Value, give: &mut dyn FnMut(Member<'_>) -> Result<(), WalkStopped>) -> Result<(), WalkStopped> {
        let inner = Walked { value, written: self.written };
        match value {
            Value::Object(_) => give(Member::Object(&inner)),
            Value::Array(_) => give(Member::List(&inner)),
            scalar => {
                self.written.borrow_mut().push(scalar.to_string());
                give(Member::Value(scalar))
            }
        }
    }

    fn object(&self) -> &'a Map<String, Value> {
        self.value.as_object().expect("a source object over an object")
    }
}

impl SourceObject for Walked<'_> {
    fn member_names(&self) -> Vec<String> {
        self.object().keys().cloned().collect()
    }

    fn peek(&self, name: &str) -> Option<Peek<'_>> {
        Some(match self.object().get(name)? {
            Value::Object(_) => Peek::Object,
            Value::Array(_) => Peek::List,
            scalar => Peek::Value(Cow::Borrowed(scalar)),
        })
    }

    fn walk(&self, visit: &mut dyn FnMut(&str, Member<'_>) -> Result<(), WalkStopped>) -> Result<(), WalkStopped> {
        let mut names: Vec<&String> = self.object().keys().collect();
        names.sort_by(|left, right| utf16_order(left, right));
        for name in names {
            self.hand(&self.object()[name], &mut |member| visit(name, member))?;
        }
        Ok(())
    }
}

impl SourceList for Walked<'_> {
    fn walk(&self, visit: &mut dyn FnMut(Member<'_>) -> Result<(), WalkStopped>) -> Result<(), WalkStopped> {
        for entry in self.value.as_array().expect("a source list over a list") {
            self.hand(entry, visit)?;
        }
        Ok(())
    }
}

/// Rebuilds the value a walk shows, checking what a visitor can see of the source's
/// promises: names in order, and each peek agreeing with what is then walked.
fn rebuild_object(source: &dyn SourceObject, depth: usize) -> Result<Value, WalkStopped> {
    assert!(depth <= MAX_SOURCE_NESTING);
    let mut names = source.member_names();
    names.sort_by(|left, right| utf16_order(left, right));
    let mut walked: Vec<String> = Vec::new();
    let mut rebuilt = Map::new();
    source.walk(&mut |name, member| {
        match (source.peek(name).expect("a walked name is a member"), &member) {
            (Peek::Value(peeked), Member::Value(shown)) => assert_eq!(peeked.as_ref(), *shown),
            (Peek::Object, Member::Object(_)) | (Peek::List, Member::List(_)) => {}
            _ => panic!("the peek of {name} disagrees with its walk"),
        }
        walked.push(name.to_string());
        rebuilt.insert(name.to_string(), rebuild(member, depth)?);
        Ok(())
    })?;
    assert_eq!(walked, names);
    assert!(source.peek("a name that is not a member").is_none());
    Ok(Value::Object(rebuilt))
}

fn rebuild(member: Member<'_>, depth: usize) -> Result<Value, WalkStopped> {
    match member {
        Member::Value(value) => Ok(value.clone()),
        Member::Object(object) => rebuild_object(object, depth + 1),
        Member::List(list) => {
            let mut entries = Vec::new();
            list.walk(&mut |entry| {
                entries.push(rebuild(entry, depth + 1)?);
                Ok(())
            })?;
            Ok(Value::Array(entries))
        }
    }
}

fn held() -> Value {
    json!({
        "result_sets": [{"values": [{"row": 1, "unit": "MPa"}, {"row": 2, "unit": "MPa"}], "name": "invented"}],
        "\u{10000}": "above the Basic Multilingual Plane",
        "\u{ffff}": "the last code unit of the plane",
        "note": null,
        "counts": [1, 2.5, true]
    })
}

#[test]
fn a_source_is_walked_in_canonical_order_and_shows_what_it_holds() {
    let held = held();
    let written = RefCell::new(Vec::new());
    let source = Walked { value: &held, written: &written };
    let content = Content::Object(&source);
    let Content::Object(object) = content else { panic!("a source") };
    assert_eq!(rebuild_object(object, 1).unwrap(), held);

    // UTF-16 order, not code-point order: the surrogate pair of U+10000 sorts before U+FFFF.
    let mut order = Vec::new();
    SourceObject::walk(&source, &mut |name, _| {
        order.push(name.to_string());
        Ok(())
    })
    .unwrap();
    assert_eq!(order, ["counts", "note", "result_sets", "\u{10000}", "\u{ffff}"]);
}

#[test]
fn what_a_visitor_is_shown_is_what_the_source_wrote() {
    let held = held();
    let written = RefCell::new(Vec::new());
    let source = Walked { value: &held, written: &written };
    let mut shown = Vec::new();
    fn scalars(member: Member<'_>, shown: &mut Vec<String>) -> Result<(), WalkStopped> {
        match member {
            Member::Value(value) => {
                shown.push(value.to_string());
                Ok(())
            }
            Member::Object(object) => object.walk(&mut |_, inner| scalars(inner, shown)),
            Member::List(list) => list.walk(&mut |inner| scalars(inner, shown)),
        }
    }
    SourceObject::walk(&source, &mut |_, member| scalars(member, &mut shown)).unwrap();
    assert_eq!(shown, *written.borrow());
    assert_eq!(shown.len(), 11);
}

#[test]
fn a_stopped_walk_does_not_return_ok() {
    let held = held();
    let written = RefCell::new(Vec::new());
    let source = Walked { value: &held, written: &written };
    let mut seen = 0;
    let stopped = SourceObject::walk(&source, &mut |_, _| {
        seen += 1;
        if seen == 2 { Err(WalkStopped) } else { Ok(()) }
    });
    assert_eq!(stopped, Err(WalkStopped));
    assert_eq!(seen, 2);
}
