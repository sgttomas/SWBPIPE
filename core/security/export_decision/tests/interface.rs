//! The interface's texts against the schema, the summary outcome's serialised form and
//! its admission, the order of its finding counts, the digest's written form, and content
//! sources as a producer implements them, with the checks a visitor makes of one.

use std::borrow::Cow;
use std::cell::{Cell, RefCell};

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
    // A decision is not copied: the same decision is made twice here, one kept to compare.
    let decided = decision();
    let bound = decision().bound_to(Sha256Digest::from_bytes([9; 32]));
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

// --- A content source, as a producer implements one, and a visitor, as a rule is one. ---

/// The order of canonical JSON and of a source's walk: UTF-16 code units.
fn utf16_order(left: &str, right: &str) -> std::cmp::Ordering {
    left.encode_utf16().cmp(right.encode_utf16())
}

/// The text these tests take as a value's canonical encoding, written from the value alone
/// and by no source: members in UTF-16 order, no spaces.
fn canonical(value: &Value) -> String {
    match value {
        Value::Object(members) => {
            let mut names: Vec<&String> = members.keys().collect();
            names.sort_by(|left, right| utf16_order(left, right));
            let parts: Vec<String> = names
                .into_iter()
                .map(|name| format!("{}:{}", Value::String(name.clone()), canonical(&members[name])))
                .collect();
            format!("{{{}}}", parts.join(","))
        }
        Value::Array(entries) => {
            let parts: Vec<String> = entries.iter().map(canonical).collect();
            format!("[{}]", parts.join(","))
        }
        scalar => scalar.to_string(),
    }
}

fn scalar_count(value: &Value) -> u64 {
    match value {
        Value::Object(members) => members.values().map(scalar_count).sum(),
        Value::Array(entries) => entries.iter().map(scalar_count).sum(),
        _ => 1,
    }
}

/// A producer over a held value. It stages the encoding of what it shows as it shows it,
/// gives every object and list by walk except a `provenance` member, which it holds as a
/// value, and does not return `Ok` unless every child it handed over had been walked to its
/// end exactly once when the visitor's call for it returned.
struct Produced<'a> {
    value: &'a Value,
    stage: &'a RefCell<String>,
    walks: Cell<u32>,
    ended: Cell<bool>,
}

impl<'a> Produced<'a> {
    fn over(value: &'a Value, stage: &'a RefCell<String>) -> Self {
        Produced { value, stage, walks: Cell::new(0), ended: Cell::new(false) }
    }

    fn begin(&self) -> Result<(), WalkStopped> {
        self.walks.set(self.walks.get() + 1);
        if self.walks.get() == 1 { Ok(()) } else { Err(WalkStopped) }
    }

    fn held_as_value(name: Option<&str>, value: &Value) -> bool {
        name == Some("provenance") || !matches!(value, Value::Object(_) | Value::Array(_))
    }

    fn hand(
        &self,
        name: Option<&str>,
        value: &Value,
        give: &mut dyn FnMut(Member<'_>) -> Result<(), WalkStopped>,
    ) -> Result<(), WalkStopped> {
        if Self::held_as_value(name, value) {
            self.stage.borrow_mut().push_str(&canonical(value));
            return give(Member::Value(value));
        }
        let child = Produced::over(value, self.stage);
        if value.is_object() {
            give(Member::Object(&child))?;
        } else {
            give(Member::List(&child))?;
        }
        if child.walks.get() == 1 && child.ended.get() { Ok(()) } else { Err(WalkStopped) }
    }

    fn object(&self) -> &'a Map<String, Value> {
        self.value.as_object().expect("a source object over an object")
    }
}

impl SourceObject for Produced<'_> {
    fn member_names(&self) -> Vec<String> {
        self.object().keys().cloned().collect()
    }

    fn peek(&self, name: &str) -> Option<Peek<'_>> {
        let value = self.object().get(name)?;
        Some(if Self::held_as_value(Some(name), value) {
            Peek::Value(Cow::Borrowed(value))
        } else if value.is_object() {
            Peek::Object
        } else {
            Peek::List
        })
    }

    fn walk(&self, visit: &mut dyn FnMut(&str, Member<'_>) -> Result<(), WalkStopped>) -> Result<(), WalkStopped> {
        self.begin()?;
        let mut names: Vec<&String> = self.object().keys().collect();
        names.sort_by(|left, right| utf16_order(left, right));
        self.stage.borrow_mut().push('{');
        for (index, name) in names.into_iter().enumerate() {
            {
                let mut stage = self.stage.borrow_mut();
                if index > 0 {
                    stage.push(',');
                }
                stage.push_str(&Value::String(name.clone()).to_string());
                stage.push(':');
            }
            self.hand(Some(name.as_str()), &self.object()[name], &mut |member| visit(name, member))?;
        }
        self.stage.borrow_mut().push('}');
        self.ended.set(true);
        Ok(())
    }
}

impl SourceList for Produced<'_> {
    fn walk(&self, visit: &mut dyn FnMut(Member<'_>) -> Result<(), WalkStopped>) -> Result<(), WalkStopped> {
        self.begin()?;
        self.stage.borrow_mut().push('[');
        for (index, entry) in self.value.as_array().expect("a source list over a list").iter().enumerate() {
            if index > 0 {
                self.stage.borrow_mut().push(',');
            }
            self.hand(None, entry, visit)?;
        }
        self.stage.borrow_mut().push(']');
        self.ended.set(true);
        Ok(())
    }
}

/// What a visitor can check of a source object, as a rule checks it, and the value it was
/// shown. Every read by name is made before the walk. A failed check is recorded in
/// `problems` with its path and stops the walk.
fn see_object(source: &dyn SourceObject, depth: usize, path: &str, problems: &mut Vec<String>) -> Result<Value, WalkStopped> {
    let fail = |problem: &str, problems: &mut Vec<String>| {
        problems.push(format!("{path}: {problem}"));
        Err(WalkStopped)
    };
    if depth > MAX_SOURCE_NESTING {
        return fail("nested deeper than the bound", problems);
    }
    let mut names = source.member_names();
    names.sort_by(|left, right| utf16_order(left, right));
    if names.windows(2).any(|pair| pair[0] == pair[1]) {
        return fail("a member name is given twice", problems);
    }
    let peeks: Vec<Option<Peek<'_>>> = names.iter().map(|name| source.peek(name)).collect();
    if source.peek("a name that is not a member").is_some() {
        return fail("a name that is not a member was answered", problems);
    }
    if let Some(at) = names.iter().position(|name| name == "provenance") {
        if !matches!(peeks[at], Some(Peek::Value(_))) {
            return fail("provenance is not held as a value", problems);
        }
    }
    let mut position = 0;
    let mut shown = Map::new();
    let walked = source.walk(&mut |name, member| {
        if names.get(position).map(String::as_str) != Some(name) {
            problems.push(format!("{path}: {name} is out of order, repeated or not a member"));
            return Err(WalkStopped);
        }
        let agrees = match (&peeks[position], &member) {
            (Some(Peek::Value(peeked)), Member::Value(value)) => **peeked == **value,
            (Some(Peek::Object), Member::Object(_)) | (Some(Peek::List), Member::List(_)) => true,
            _ => false,
        };
        if !agrees {
            problems.push(format!("{path}.{name}: its peek disagrees with its walk"));
            return Err(WalkStopped);
        }
        position += 1;
        let value = see(member, depth, &format!("{path}.{name}"), problems)?;
        shown.insert(name.to_string(), value);
        Ok(())
    });
    if walked.is_err() {
        if problems.is_empty() {
            problems.push(format!("{path}: the walk stopped"));
        }
        return Err(WalkStopped);
    }
    if position != names.len() {
        return fail("a member was not walked", problems);
    }
    Ok(Value::Object(shown))
}

fn see_list(list: &dyn SourceList, depth: usize, path: &str, problems: &mut Vec<String>) -> Result<Value, WalkStopped> {
    if depth > MAX_SOURCE_NESTING {
        problems.push(format!("{path}: nested deeper than the bound"));
        return Err(WalkStopped);
    }
    let mut entries = Vec::new();
    let walked = list.walk(&mut |entry| {
        let value = see(entry, depth, &format!("{path}[{}]", entries.len()), problems)?;
        entries.push(value);
        Ok(())
    });
    if walked.is_err() {
        if problems.is_empty() {
            problems.push(format!("{path}: the walk stopped"));
        }
        return Err(WalkStopped);
    }
    Ok(Value::Array(entries))
}

/// A handed member, walked to its end once inside the call that handed it over.
fn see(member: Member<'_>, depth: usize, path: &str, problems: &mut Vec<String>) -> Result<Value, WalkStopped> {
    match member {
        Member::Value(value) => Ok(value.clone()),
        Member::Object(object) => see_object(object, depth + 1, path, problems),
        Member::List(list) => see_list(list, depth + 1, path, problems),
    }
}

fn held() -> Value {
    json!({
        "result_sets": [{"values": [{"row": 1, "unit": "MPa"}, {"row": 2, "unit": "MPa"}], "name": "invented"}],
        "provenance": {"review_status": "accepted", "source": {"kind": "invented"}},
        "\u{10000}": "above the Basic Multilingual Plane",
        "\u{ffff}": "the last code unit of the plane",
        "note": null,
        "empty_object": {},
        "empty_list": [],
        "counts": [1, 2.5, true]
    })
}

#[test]
fn a_producer_stages_exactly_the_encoding_of_the_root_it_shows() {
    let held = held();
    let stage = RefCell::new(String::new());
    let source = Produced::over(&held, &stage);
    let mut problems = Vec::new();
    let shown = see_object(&source, 1, "$", &mut problems).expect("a source that keeps its promises");
    assert_eq!(problems, Vec::<String>::new());
    assert_eq!(shown, held);

    // The complete staged document against text written from the held value alone: names,
    // nesting, order, the empty containers and the end of the document.
    let staged = stage.borrow().clone();
    assert_eq!(staged, canonical(&held));
    assert!(staged.starts_with("{\"counts\":[1,2.5,true],\"empty_list\":[],\"empty_object\":{},\"note\":null,\"provenance\":{"));
    // UTF-16 order, not code-point order: the surrogate pair of U+10000 sorts before U+FFFF.
    assert!(staged.find('\u{10000}').unwrap() < staged.find('\u{ffff}').unwrap());
    assert!(staged.ends_with("\"the last code unit of the plane\"}"));

    // What is read by name: a held member as its value, a member given by walk as its kind,
    // provenance as its value although it is an object, and nothing for a non-member.
    assert_eq!(source.peek("note"), Some(Peek::Value(Cow::Owned(Value::Null))));
    assert_eq!(source.peek("result_sets"), Some(Peek::List));
    assert_eq!(source.peek("empty_object"), Some(Peek::Object));
    assert_eq!(source.peek("provenance"), Some(Peek::Value(Cow::Borrowed(&held["provenance"]))));
    assert_eq!(source.peek("absent"), None);
    // Reading by name staged nothing.
    assert_eq!(*stage.borrow(), staged);
}

#[test]
fn a_producer_refuses_a_visitor_that_skips_repeats_or_cuts_short_a_child() {
    let held = json!({"list": [1, 2]});

    // Returning Ok without walking the handed list: the producer's walk does not return Ok.
    let stage = RefCell::new(String::new());
    let source = Produced::over(&held, &stage);
    assert_eq!(SourceObject::walk(&source, &mut |_, _| Ok(())), Err(WalkStopped));

    // Walking the handed list twice: the second walk stops, and the stop is passed on.
    let stage = RefCell::new(String::new());
    let source = Produced::over(&held, &stage);
    let repeated = SourceObject::walk(&source, &mut |_, member| match member {
        Member::List(list) => {
            list.walk(&mut |_| Ok(()))?;
            list.walk(&mut |_| Ok(()))
        }
        _ => Ok(()),
    });
    assert_eq!(repeated, Err(WalkStopped));

    // Stopping the handed list's walk and returning Ok all the same: the list did not reach
    // its end, so the producer's walk does not return Ok.
    let stage = RefCell::new(String::new());
    let source = Produced::over(&held, &stage);
    let swallowed = SourceObject::walk(&source, &mut |_, member| {
        if let Member::List(list) = member {
            assert_eq!(list.walk(&mut |_| Err(WalkStopped)), Err(WalkStopped));
        }
        Ok(())
    });
    assert_eq!(swallowed, Err(WalkStopped));

    // The root is walked once.
    let empty = json!({});
    let stage = RefCell::new(String::new());
    let source = Produced::over(&empty, &stage);
    assert_eq!(SourceObject::walk(&source, &mut |_, _| Ok(())), Ok(()));
    assert_eq!(SourceObject::walk(&source, &mut |_, _| Ok(())), Err(WalkStopped));
}

// --- Entries built for their call, as a producer over a held result yields rows. ---

fn row(index: u64) -> Value {
    json!({"provenance": {"review_status": "accepted"}, "row": index, "unit": "MPa"})
}

/// A list whose every entry is built for its call and dropped after it.
struct Rows<'a> {
    count: u64,
    stage: &'a RefCell<String>,
}

impl SourceList for Rows<'_> {
    fn walk(&self, visit: &mut dyn FnMut(Member<'_>) -> Result<(), WalkStopped>) -> Result<(), WalkStopped> {
        self.stage.borrow_mut().push('[');
        for index in 0..self.count {
            let built = row(index);
            {
                let mut stage = self.stage.borrow_mut();
                if index > 0 {
                    stage.push(',');
                }
                stage.push_str(&canonical(&built));
            }
            visit(Member::Value(&built))?;
        }
        self.stage.borrow_mut().push(']');
        Ok(())
    }
}

/// An object built for its call that holds a list of rows.
struct Set<'a> {
    rows: u64,
    stage: &'a RefCell<String>,
}

impl SourceObject for Set<'_> {
    fn member_names(&self) -> Vec<String> {
        vec!["values".to_string(), "name".to_string()]
    }

    fn peek(&self, name: &str) -> Option<Peek<'_>> {
        match name {
            "name" => Some(Peek::Value(Cow::Owned(json!("invented")))),
            "values" => Some(Peek::List),
            _ => None,
        }
    }

    fn walk(&self, visit: &mut dyn FnMut(&str, Member<'_>) -> Result<(), WalkStopped>) -> Result<(), WalkStopped> {
        self.stage.borrow_mut().push_str("{\"name\":\"invented\"");
        visit("name", Member::Value(&json!("invented")))?;
        self.stage.borrow_mut().push_str(",\"values\":");
        visit("values", Member::List(&Rows { count: self.rows, stage: self.stage }))?;
        self.stage.borrow_mut().push('}');
        Ok(())
    }
}

/// A list of one object built for its call: the shape of a document whose sets hold rows.
struct Sets<'a> {
    rows: u64,
    stage: &'a RefCell<String>,
}

impl SourceList for Sets<'_> {
    fn walk(&self, visit: &mut dyn FnMut(Member<'_>) -> Result<(), WalkStopped>) -> Result<(), WalkStopped> {
        self.stage.borrow_mut().push('[');
        visit(Member::Object(&Set { rows: self.rows, stage: self.stage }))?;
        self.stage.borrow_mut().push(']');
        Ok(())
    }
}

struct Document<'a> {
    rows: u64,
    stage: &'a RefCell<String>,
}

impl SourceObject for Document<'_> {
    fn member_names(&self) -> Vec<String> {
        vec!["schema_version".to_string(), "result_sets".to_string()]
    }

    fn peek(&self, name: &str) -> Option<Peek<'_>> {
        match name {
            "result_sets" => Some(Peek::List),
            "schema_version" => Some(Peek::Value(Cow::Owned(json!("1.0")))),
            _ => None,
        }
    }

    fn walk(&self, visit: &mut dyn FnMut(&str, Member<'_>) -> Result<(), WalkStopped>) -> Result<(), WalkStopped> {
        self.stage.borrow_mut().push_str("{\"result_sets\":");
        visit("result_sets", Member::List(&Sets { rows: self.rows, stage: self.stage }))?;
        self.stage.borrow_mut().push_str(",\"schema_version\":\"1.0\"");
        visit("schema_version", Member::Value(&json!("1.0")))?;
        self.stage.borrow_mut().push('}');
        Ok(())
    }
}

fn document_value(rows: u64) -> Value {
    json!({
        "result_sets": [{"name": "invented", "values": (0..rows).map(row).collect::<Vec<Value>>()}],
        "schema_version": "1.0"
    })
}

#[test]
fn entries_built_for_their_call_are_shown_and_staged() {
    for rows in [0, 1, 3] {
        let stage = RefCell::new(String::new());
        let source = Document { rows, stage: &stage };
        let mut problems = Vec::new();
        let shown = see_object(&source, 1, "$", &mut problems).expect("a source that keeps its promises");
        assert_eq!(problems, Vec::<String>::new());
        assert_eq!(shown, document_value(rows));
        assert_eq!(*stage.borrow(), canonical(&document_value(rows)));
    }
}

#[test]
fn a_stop_inside_a_nested_list_stops_every_walk_above_it() {
    fn until_second_row(member: Member<'_>, rows: &mut u32) -> Result<(), WalkStopped> {
        match member {
            Member::Value(value) => {
                if value.get("row").is_some() {
                    *rows += 1;
                    if *rows == 2 {
                        return Err(WalkStopped);
                    }
                }
                Ok(())
            }
            Member::Object(object) => object.walk(&mut |_, inner| until_second_row(inner, rows)),
            Member::List(list) => list.walk(&mut |inner| until_second_row(inner, rows)),
        }
    }
    let stage = RefCell::new(String::new());
    let source = Document { rows: 5, stage: &stage };
    let mut rows = 0;
    let stopped = source.walk(&mut |_, member| until_second_row(member, &mut rows));
    assert_eq!(stopped, Err(WalkStopped));
    assert_eq!(rows, 2);
    // What was staged is not a document: it ends at the row the visitor stopped on, which
    // is why a stopped walk's stage is discarded.
    assert_eq!(
        *stage.borrow(),
        format!("{{\"result_sets\":[{{\"name\":\"invented\",\"values\":[{},{}", canonical(&row(0)), canonical(&row(1)))
    );
}

// --- Sources that break a promise, each of which a visitor's checks refuse. ---

#[derive(Clone, Copy, Debug, PartialEq)]
enum Lie {
    None,
    NameTwice,
    OutOfOrder,
    MemberNotWalked,
    MemberWalkedTwice,
    PeekedValueDiffers,
    PeekedKindDiffers,
    AnswersForANonMember,
    ProvenanceOnlyByWalk,
    StopsItself,
}

/// A source over `{"a": 1, "b": 2, "provenance": {...}}` that tells one lie.
struct Lying {
    lie: Lie,
    held: Value,
    stage: RefCell<String>,
}

impl Lying {
    fn telling(lie: Lie) -> Self {
        Lying { lie, held: json!({"a": 1, "b": 2, "provenance": {"review_status": "accepted"}}), stage: RefCell::new(String::new()) }
    }
}

impl SourceObject for Lying {
    fn member_names(&self) -> Vec<String> {
        let mut names = vec!["a".to_string(), "b".to_string(), "provenance".to_string()];
        if self.lie == Lie::NameTwice {
            names.push("a".to_string());
        }
        names
    }

    fn peek(&self, name: &str) -> Option<Peek<'_>> {
        match (self.lie, name) {
            (Lie::PeekedValueDiffers, "a") => Some(Peek::Value(Cow::Owned(json!(9)))),
            (Lie::PeekedKindDiffers, "a") => Some(Peek::List),
            (Lie::ProvenanceOnlyByWalk, "provenance") => Some(Peek::Object),
            (Lie::AnswersForANonMember, name) if self.held.get(name).is_none() => Some(Peek::Value(Cow::Owned(Value::Null))),
            (_, name) => self.held.get(name).map(|value| Peek::Value(Cow::Borrowed(value))),
        }
    }

    fn walk(&self, visit: &mut dyn FnMut(&str, Member<'_>) -> Result<(), WalkStopped>) -> Result<(), WalkStopped> {
        let order: &[&str] = match self.lie {
            Lie::OutOfOrder => &["b", "a", "provenance"],
            Lie::MemberNotWalked => &["a", "provenance"],
            Lie::MemberWalkedTwice => &["a", "a", "b", "provenance"],
            _ => &["a", "b", "provenance"],
        };
        for (index, name) in order.iter().enumerate() {
            if self.lie == Lie::StopsItself && index == 1 {
                return Err(WalkStopped);
            }
            if self.lie == Lie::ProvenanceOnlyByWalk && *name == "provenance" {
                visit(name, Member::Object(&Produced::over(&self.held["provenance"], &self.stage)))?;
            } else {
                visit(name, Member::Value(&self.held[*name]))?;
            }
        }
        Ok(())
    }
}

#[test]
fn a_source_that_breaks_a_promise_is_refused_where_a_visitor_can_see_it() {
    let mut problems = Vec::new();
    let honest = Lying::telling(Lie::None);
    assert_eq!(see_object(&honest, 1, "$", &mut problems), Ok(honest.held.clone()));
    assert_eq!(problems, Vec::<String>::new());

    for (lie, problem) in [
        (Lie::NameTwice, "$: a member name is given twice"),
        (Lie::OutOfOrder, "$: b is out of order, repeated or not a member"),
        (Lie::MemberNotWalked, "$: provenance is out of order, repeated or not a member"),
        (Lie::MemberWalkedTwice, "$: a is out of order, repeated or not a member"),
        (Lie::PeekedValueDiffers, "$.a: its peek disagrees with its walk"),
        (Lie::PeekedKindDiffers, "$.a: its peek disagrees with its walk"),
        (Lie::AnswersForANonMember, "$: a name that is not a member was answered"),
        (Lie::ProvenanceOnlyByWalk, "$: provenance is not held as a value"),
        (Lie::StopsItself, "$: the walk stopped"),
    ] {
        let mut problems = Vec::new();
        assert_eq!(see_object(&Lying::telling(lie), 1, "$", &mut problems), Err(WalkStopped), "{lie:?}");
        assert_eq!(problems, [problem], "{lie:?}");
    }
}

// --- The bound on nesting, counted from the root as 1, for objects and for lists. ---

/// An object holding one more of itself, `inner` levels deep below it.
struct NestedObject {
    inner: usize,
}

impl SourceObject for NestedObject {
    fn member_names(&self) -> Vec<String> {
        vec!["inner".to_string()]
    }

    fn peek(&self, name: &str) -> Option<Peek<'_>> {
        (name == "inner").then(|| if self.inner > 0 { Peek::Object } else { Peek::Value(Cow::Owned(Value::Null)) })
    }

    fn walk(&self, visit: &mut dyn FnMut(&str, Member<'_>) -> Result<(), WalkStopped>) -> Result<(), WalkStopped> {
        if self.inner > 0 {
            visit("inner", Member::Object(&NestedObject { inner: self.inner - 1 }))
        } else {
            visit("inner", Member::Value(&Value::Null))
        }
    }
}

/// A list holding one more of itself, `inner` levels deep below it.
struct NestedList {
    inner: usize,
}

impl SourceList for NestedList {
    fn walk(&self, visit: &mut dyn FnMut(Member<'_>) -> Result<(), WalkStopped>) -> Result<(), WalkStopped> {
        if self.inner > 0 {
            visit(Member::List(&NestedList { inner: self.inner - 1 }))
        } else {
            visit(Member::Value(&Value::Null))
        }
    }
}

/// A root object holding a chain of `lists` lists.
struct ListRoot {
    lists: usize,
}

impl SourceObject for ListRoot {
    fn member_names(&self) -> Vec<String> {
        vec!["list".to_string()]
    }

    fn peek(&self, name: &str) -> Option<Peek<'_>> {
        (name == "list").then_some(Peek::List)
    }

    fn walk(&self, visit: &mut dyn FnMut(&str, Member<'_>) -> Result<(), WalkStopped>) -> Result<(), WalkStopped> {
        visit("list", Member::List(&NestedList { inner: self.lists - 1 }))
    }
}

#[test]
fn nesting_is_counted_from_the_root_for_objects_and_for_lists() {
    // The root is 1, so a root with MAX - 1 objects inside it is at the bound.
    let mut problems = Vec::new();
    assert!(see_object(&NestedObject { inner: MAX_SOURCE_NESTING - 1 }, 1, "$", &mut problems).is_ok());
    assert_eq!(see_object(&NestedObject { inner: MAX_SOURCE_NESTING }, 1, "$", &mut problems), Err(WalkStopped));
    assert_eq!(problems.len(), 1);
    assert!(problems[0].ends_with(": nested deeper than the bound"), "{problems:?}");

    // A root with MAX - 1 lists inside it is at the bound; one more list is beyond it.
    let mut problems = Vec::new();
    assert!(see_object(&ListRoot { lists: MAX_SOURCE_NESTING - 1 }, 1, "$", &mut problems).is_ok());
    assert_eq!(see_object(&ListRoot { lists: MAX_SOURCE_NESTING }, 1, "$", &mut problems), Err(WalkStopped));
    assert_eq!(problems.len(), 1);
    assert!(problems[0].ends_with(": nested deeper than the bound"), "{problems:?}");
}

// --- A rule, as a caller reaches one: through the trait object, for a value and a source. ---

/// A stand-in that reads the whole content and counts one decision per scalar.
struct Counting;

impl ContentRule for Counting {
    fn rule_revision(&self) -> &'static str {
        "invented-counting-revision"
    }

    fn decide_summary(
        &self,
        content: Content<'_>,
        request: &RouteRequest<'_>,
        _blocking_bound: usize,
    ) -> Result<SummaryDecision, RuleError> {
        let shown = match content {
            Content::Value(value) => value.clone(),
            Content::Object(source) => {
                let mut problems = Vec::new();
                let seen = see_object(source, 1, "$", &mut problems);
                match seen {
                    Ok(value) => value,
                    Err(WalkStopped) => return Err(RuleError::ContentSourceInconsistent { path: problems.remove(0) }),
                }
            }
        };
        Ok(SummaryDecision {
            route_id: request.route_id.to_string(),
            export_context: request.export_context,
            explicit_local_private_intent: request.explicit_local_private_intent,
            rule_revision: self.rule_revision().to_string(),
            decision_total: scalar_count(&shown),
            finding_counts: Vec::new(),
            ..decision()
        })
    }
}

#[test]
fn a_value_and_a_source_of_it_are_decided_alike_through_the_trait_object() {
    let rule: &dyn ContentRule = &Counting;
    let request = RouteRequest {
        route_id: "INVENTED-ROUTE-003",
        export_context: ExportContext::LocalPrivate,
        explicit_local_private_intent: true,
        require_lossless_materialization: true,
    };
    let held = held();
    let of_value = rule.decide_summary(Content::from(&held), &request, 10).unwrap();

    let stage = RefCell::new(String::new());
    let source = Produced::over(&held, &stage);
    let of_source = rule.decide_summary(Content::Object(&source), &request, 10).unwrap();
    assert_eq!(of_source, of_value);
    assert_eq!(of_source.decision_total, 13);

    // The decision is bound afterwards, to the digest of the complete staged bytes (here a
    // stand-in for one: this crate does not hash).
    assert_eq!(*stage.borrow(), canonical(&held));
    let outcome = of_source.bound_to(Sha256Digest::from_bytes([3; 32]));
    assert!(outcome.may_expose());

    // A source that breaks a promise is a refusal, with where it was seen, and no decision.
    let refused = rule.decide_summary(Content::Object(&Lying::telling(Lie::OutOfOrder)), &request, 10);
    assert_eq!(
        refused,
        Err(RuleError::ContentSourceInconsistent { path: "$: b is out of order, repeated or not a member".to_string() })
    );
}
