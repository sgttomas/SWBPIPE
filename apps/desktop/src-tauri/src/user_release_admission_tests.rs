//! Tests of route 010's model and run-record refusals (W-18b slice (r2b);
//! DEL-08-01 Design `report-user-release.md` §2.5 steps 5, 6 and 8, §2.7,
//! §6). Every model, library, record and hanger value here is invented. The
//! refusals are tested at the functions route 010's admission (r2c) calls;
//! "admitted" below means only that the rule finds no site, which shows the
//! reliance limit, not that any value is the user's own.

use super::*;
use crate::export_disposition_screen::library_ref_member_sites;
use crate::stored_read::UnmeteredReadMemory;
use open_pipe_stress_canonical_json::canonical_json;
use open_pipe_stress_operation_applier::sha256_hex;
use serde_json::{json, Value};

const MEMBER_TABLE: &str = include_str!("../../../../fixtures/model_document/library_ref_member_sites.json");
const SITE_TABLE: &str = include_str!("../../../../fixtures/model_document/library_ref_sites.json");
const DEMO_MODEL: &str = include_str!("../../../../fixtures/product_preview/invented_demo_model.json");
const DEMO_RESULT: &str = include_str!("../../../../fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json");
const VECTORS: &str = include_str!("../../../../fixtures/reports/invented/user_release_derivation_vectors.json");
const CONNECTED_MODEL: &str = include_str!("../../../../fixtures/model_operations/precision_connected_ui_model.json");
const CONNECTED_SPARSE: &str = include_str!("../../../../fixtures/results/preview_physics_connected_sparse.json");

fn parse(text: &str) -> Value {
    serde_json::from_str(text).unwrap()
}

/// A registry on a normal host (the default budget) and an ingress lease of
/// 1 KiB in it, as a command holds one: rule 3 (b) grows it.
fn registry_and_lease() -> (crate::solve_jobs::SolveJobRegistry, Lease) {
    let registry = crate::command_ingress::test_support::registry(crate::memory_pool::test_host::ScriptedHost::normal());
    let lease = registry.acquire_interactive_ingress(1024).unwrap();
    (registry, lease)
}

fn lease() -> Lease {
    registry_and_lease().1
}

/// The first site, with any growth admitted.
fn site(model: &Value) -> Option<LibrarySite> {
    library_reference_site(model, &mut lease()).unwrap()
}

/// The refusal text of a refused model.
fn referenced(model: &Value) -> String {
    match refuse_library_referenced(model, &mut lease()) {
        Err(LibraryRefusal::Referenced(text)) => text,
        other => panic!("not a library refusal: {other:?}"),
    }
}

/// A document holding only `pointer`'s path in `document`, ending in its value
/// (an array index keeps the elements before it as empty objects).
fn isolated(document: &Value, pointer: &str) -> Value {
    fn build(source: &Value, tokens: &[String]) -> Value {
        let Some((token, rest)) = tokens.split_first() else { return source.clone() };
        match source {
            Value::Array(items) => {
                let index: usize = token.parse().unwrap();
                let mut built: Vec<Value> = (0..index).map(|_| json!({})).collect();
                built.push(build(&items[index], rest));
                Value::Array(built)
            }
            Value::Object(object) => {
                let key = token.replace("~1", "/").replace("~0", "~");
                json!({ key.clone(): build(&object[&key], rest) })
            }
            _ => unreachable!("a pointer through a scalar"),
        }
    }
    let tokens: Vec<String> = pointer.split('/').skip(1).map(str::to_string).collect();
    build(document, &tokens)
}

/// The legacy hanger selection evidence exactly as the pre-W-17 writer built
/// it (`hangerSelection.ts:35-39` at `00af9112c0^`): the RFC 8785 text of the
/// six members, over an invented library document and record.
fn legacy_object() -> Value {
    let provenance = json!({
        "source_name": "Invented hanger catalogue",
        "source_location": "invented",
        "source_license": "invented_non_engineering_example",
        "contributor": "invented",
        "contributor_certification": "not_asserted",
        "redistribution_status": "private_only",
        "review_status": "pending"
    });
    json!({
        "kind": LEGACY_HANGER_SELECTION_KIND,
        "library_kind": "hanger",
        "library_id": "hangerlib.invented",
        "hanger_id": "hanger.invented.1",
        "library_metadata": { "library_id": "hangerlib.invented", "name": "Invented hangers", "provenance": provenance },
        "record_snapshot": {
            "hanger_id": "hanger.invented.1",
            "name": "Invented variable spring",
            "provenance": provenance,
            "hanger": {
                "hanger_type": "variable_spring_hanger",
                "source_reference": "invented",
                "stiffness": { "dof": "UZ", "value": { "magnitude": 42000, "unit": "N/m", "dimension": "stiffness", "provenance": provenance } }
            }
        }
    })
}

const HANGER: usize = 5;

/// The demo model with its spring hanger's `provenance` set to `text`.
fn demo_with_hanger_provenance(text: &str) -> Value {
    let mut model = parse(DEMO_MODEL);
    assert_eq!(model["supports"][HANGER]["id"], "support:SH-140");
    model["supports"][HANGER]["provenance"] = json!(text);
    model
}

fn legacy_site(id: Option<&str>, pointer: &str) -> Option<LibrarySite> {
    Some(LibrarySite {
        pointer: pointer.to_string(),
        rule: LibraryRule::LegacyHangerSelection,
        kind: Some("hanger".to_string()),
        id: id.map(str::to_string),
    })
}

const HANGER_PROVENANCE: &str = "/supports/5/provenance";

// --- Library rule 1 -----------------------------------------------------------

/// Rule 1's detector alone, through the rule: each `library_ref` member of
/// the shared member table, held alone in a document, is a rule-1 site named
/// by its pointer, whatever its value; an object states its kind and id, a
/// non-object neither (§6, Codex round 1 at revision 4, S2).
#[test]
fn user_release_library_reference_sites_refused() {
    let table = parse(MEMBER_TABLE);
    let sites = table["sites"].as_array().unwrap();
    let pointers: Vec<&str> = sites.iter().map(|site| site["pointer"].as_str().unwrap()).collect();
    for entry in sites {
        let pointer = entry["pointer"].as_str().unwrap();
        if pointers.iter().any(|outer| *outer != pointer && pointer.starts_with(&format!("{outer}/"))) {
            continue; // held inside another site, which is found first
        }
        let found = site(&isolated(&table["document"], pointer)).unwrap_or_else(|| panic!("{pointer}"));
        assert_eq!((found.pointer.as_str(), found.rule), (pointer, LibraryRule::LibraryRef), "{pointer}");
        let refusal = found.refusal();
        assert!(refusal.starts_with(&format!("{LIBRARY_REFERENCED}: {pointer} holds a library origin (rule library_ref")), "{refusal}");
        if entry["kind"] != "object" {
            assert_eq!((found.kind, found.id), (None, None), "{pointer}: a non-object states neither kind nor id");
            assert!(refusal.contains("(rule library_ref)"), "{refusal}");
        }
    }
    // Each object site of the shared object table, a blank identity among them.
    let objects = parse(SITE_TABLE);
    let object_sites = library_ref_member_sites(&objects["document"]);
    assert!(object_sites.len() >= 8);
    let mut blank = 0;
    for member in &object_sites {
        let found = site(&isolated(&objects["document"], &member.pointer)).unwrap();
        assert_eq!((found.pointer.as_str(), found.rule), (member.pointer.as_str(), LibraryRule::LibraryRef));
        if found.kind.is_none() {
            blank += 1;
        }
    }
    assert!(blank > 0, "a blank-identity site counts too");
    // A spring hanger applied from a library: its `library_ref`, then its `library:hanger:` string.
    let mut model = demo_with_hanger_provenance("library:hanger:hangerlib.invented#h.1");
    model["supports"][HANGER]["provenance_record"] =
        json!({"source_name": "Invented", "library_ref": {"library_kind": "hanger", "library_id": "hangerlib.invented", "record_id": "h.1"}});
    let found = site(&model).unwrap();
    assert_eq!(found, LibrarySite { pointer: "/supports/5/provenance_record/library_ref".into(), rule: LibraryRule::LibraryRef, kind: Some("hanger".into()), id: Some("hangerlib.invented".into()) });
    model["supports"][HANGER]["provenance_record"].as_object_mut().unwrap().remove("library_ref");
    assert_eq!(site(&model).unwrap(), LibrarySite { pointer: HANGER_PROVENANCE.into(), rule: LibraryRule::ProvenanceString, kind: Some("hanger".into()), id: Some("hangerlib.invented".into()) });
    // The same non-object members inside embedded JSON, which the disposition screen never
    // sees: rule 3 (b) through rule 1.
    for value in [json!(null), json!("invented text"), json!(7), json!(false), json!([]), json!(["hanger", "hangerlib.invented", "h.1"]), json!({})] {
        let text = canonical_json(&json!({"note": {"library_ref": value}}));
        assert_eq!(site(&demo_with_hanger_provenance(&text)), legacy_site(None, HANGER_PROVENANCE), "{text}");
    }
}

/// Codex round 2's counterexample (§6): a bend applied from a component
/// library. Its `library_ref` is rule 1; with it removed, its modifiers'
/// `library:component:` source reference is rule 2; with both removed, no
/// library origin is detected (the reliance limit: the test pins the limit
/// and does not show the values are the user's own).
#[test]
fn user_release_library_applied_bend_refused() {
    let mut model = parse(DEMO_MODEL);
    assert_eq!(site(&model), None, "the demo model carries no library mark");
    let bend = &mut model["components"][0];
    assert_eq!(bend["kind"], "bend");
    assert!(bend["modifiers"]["sif_user_value"].is_object() && bend["modifiers"]["flexibility_factor_user_value"].is_object());
    bend["modifiers"]["source_reference"] = json!("library:component:complib.invented#bend.1");
    bend["provenance_record"] = json!({"source_name": "Invented", "library_ref": {"library_kind": "component", "library_id": "complib.invented", "record_id": "bend.1"}});
    let found = site(&model).unwrap();
    assert_eq!((found.pointer.as_str(), found.rule, found.kind.as_deref()), ("/components/0/provenance_record/library_ref", LibraryRule::LibraryRef, Some("component")));
    model["components"][0]["provenance_record"].as_object_mut().unwrap().remove("library_ref");
    let found = site(&model).unwrap();
    assert_eq!(found, LibrarySite { pointer: "/components/0/modifiers/source_reference".into(), rule: LibraryRule::ProvenanceString, kind: Some("component".into()), id: Some("complib.invented".into()) });
    model["components"][0]["modifiers"]["source_reference"] = json!("invented_user_entered_preview_no_code_table");
    assert_eq!(site(&model), None);
    assert_eq!(refuse_library_referenced(&model, &mut lease()), Ok(()));
}

/// Rule 2 at any depth and for each kind, and the re-apply note that sits
/// beside a new `library_ref` (`libraryApply.ts`).
#[test]
fn library_provenance_strings_are_rule_2() {
    for kind in ["material", "section", "component", "hanger"] {
        let model = json!({"materials": [{"id": "m", "notes": ["first", {"deep": format!("see library:{kind}:lib.{kind}#r; then")}]}]});
        let found = site(&model).unwrap();
        assert_eq!(found, LibrarySite { pointer: "/materials/0/notes/1/deep".into(), rule: LibraryRule::ProvenanceString, kind: Some(kind.into()), id: Some(format!("lib.{kind}")) });
        assert!(found.refusal().contains(&format!("(rule provenance_string, library {kind} lib.{kind})")));
    }
    let note = json!({"provenance_record": {"verification_note": "Applied over the previous library reference library:material:old#r1"}});
    assert_eq!(site(&note).unwrap().rule, LibraryRule::ProvenanceString);
    // Keys are not tested, and the test is case-sensitive.
    assert_eq!(site(&json!({"library:hanger:key": "x", "note": "LIBRARY:HANGER:x"})), None);
    // Rule 1 comes before rule 2, whatever the document order.
    let both = json!({"a": "library:hanger:h#1", "z": {"library_ref": null}});
    assert_eq!(site(&both).unwrap().pointer, "/z/library_ref");
}

// --- Library rule 3: the legacy hanger selection -------------------------------

/// Codex round 3's blocker (§6): a model saved before `00af9112c0` whose hanger
/// support's `provenance` is the legacy selection evidence, with no
/// `library_ref` and no `library:` string, and whose solve has genuine
/// `spring_hanger_user_input_review` rows. One case per recognition path,
/// each refused under the rule `legacy_hanger_selection`; then the controls.
#[test]
fn user_release_legacy_hanger_selection_refused() {
    let result = parse(DEMO_RESULT);
    let reviews = result["results"].as_array().unwrap().iter().filter(|row| row["kind"] == "spring_hanger_user_input_review" && row["entity_ref"] == "support:SH-140").count();
    assert!(reviews > 0, "the demo solve has genuine review rows for the hanger");
    let full = canonical_json(&legacy_object());
    assert!(!full.contains("library_ref") && !full.contains("library:"), "the legacy form carries neither W-17 mark");

    // The full legacy string (rules 3 (a) and 3 (b)).
    let refusal = referenced(&demo_with_hanger_provenance(&full));
    assert!(refusal.starts_with(&format!("{LIBRARY_REFERENCED}: {HANGER_PROVENANCE} holds a library origin (rule legacy_hanger_selection, library kind hanger)")), "{refusal}");
    assert!(!refusal.contains("record_snapshot") && !refusal.contains("Invented hangers"), "the refusal quotes no site text: {refusal}");
    // `kind` removed: each embedded-JSON member kept alone (3 (b)).
    for member in LEGACY_HANGER_MEMBERS {
        let mut object = legacy_object();
        let object_map = object.as_object_mut().unwrap();
        object_map.remove("kind");
        for other in LEGACY_HANGER_MEMBERS.iter().filter(|other| **other != member) {
            object_map.remove(*other);
        }
        let text = canonical_json(&object);
        assert!(!text.contains(LEGACY_HANGER_SELECTION_KIND));
        let id = (member == "library_id").then_some("hangerlib.invented");
        assert_eq!(site(&demo_with_hanger_provenance(&text)), legacy_site(id, HANGER_PROVENANCE), "{member}");
    }
    // Every one of the four removed, `kind` kept (3 (a)).
    let mut kind_only = legacy_object();
    for member in LEGACY_HANGER_MEMBERS {
        kind_only.as_object_mut().unwrap().remove(member);
    }
    assert_eq!(site(&demo_with_hanger_provenance(&canonical_json(&kind_only))), legacy_site(None, HANGER_PROVENANCE));
    // Leading and trailing JSON whitespace.
    assert_eq!(site(&demo_with_hanger_provenance(&format!(" \n\t{full}\r\n "))), legacy_site(None, HANGER_PROVENANCE));
    let mut no_kind = legacy_object();
    no_kind.as_object_mut().unwrap().remove("kind");
    let padded = format!(" \n\t{}\r\n ", canonical_json(&no_kind));
    assert_eq!(site(&demo_with_hanger_provenance(&padded)), legacy_site(Some("hangerlib.invented"), HANGER_PROVENANCE));
    // An embedded object holding only a nested `library_ref` (3 (b) through rule 1).
    let nested = canonical_json(&json!({"note": {"deeper": {"library_ref": {"library_kind": "hanger", "library_id": "x"}}}}));
    assert_eq!(site(&demo_with_hanger_provenance(&nested)), legacy_site(None, HANGER_PROVENANCE));
    // One holding only a `library:hanger:` string: as written, rule 2 already finds it in the
    // outer text; with its colon escaped, only the parse finds it (3 (b) through rule 2).
    let plain = canonical_json(&json!({"note": "library:hanger:hangerlib.invented#h.1"}));
    assert_eq!(site(&demo_with_hanger_provenance(&plain)).unwrap().rule, LibraryRule::ProvenanceString);
    // The colon written as the JSON escape backslash-u-003a.
    let escaped = format!("{{\"note\":\"library{}hanger:hangerlib.invented#h.1\"}}", concat!("\\", "u003a"));
    assert!(!escaped.contains("library:hanger:"), "{escaped}");
    assert_eq!(serde_json::from_str::<Value>(&escaped).unwrap()["note"], "library:hanger:hangerlib.invented#h.1");
    assert_eq!(site(&demo_with_hanger_provenance(&escaped)), legacy_site(None, HANGER_PROVENANCE));
    // The legacy string in an array element and at a nested object member.
    let mut model = parse(DEMO_MODEL);
    model["supports"][HANGER]["notes"] = json!(["plain", full]);
    assert_eq!(site(&model), legacy_site(None, "/supports/5/notes/1"));
    let mut model = parse(DEMO_MODEL);
    model["project"]["history"] = json!({"selection": {"evidence": canonical_json(&no_kind)}});
    assert_eq!(site(&model), legacy_site(Some("hangerlib.invented"), "/project/history/selection/evidence"));
    // Candidates the prescan cannot check: nesting deeper than 127 containers, and the
    // reserved raw-value key. Unchecked, so refused under rule 3.
    let deep = format!("{{\"a\":{}{}}}", "[".repeat(200), "]".repeat(200));
    assert_eq!(site(&demo_with_hanger_provenance(&deep)), legacy_site(None, HANGER_PROVENANCE));
    let raw = r#"{"$serde_json::private::RawValue":"x"}"#;
    assert_eq!(site(&demo_with_hanger_provenance(raw)), legacy_site(None, HANGER_PROVENANCE));
    // A legacy string inside a string of an embedded object: 3 (a) sees it at any depth.
    let inside = canonical_json(&json!({"note": full}));
    assert_eq!(site(&demo_with_hanger_provenance(&inside)), legacy_site(None, HANGER_PROVENANCE));

    // Controls, each with no detected library origin (the reliance limit, §2.7).
    for control in [
        "{ this begins with a brace but is not JSON".to_string(),
        canonical_json(&json!([{"hanger_id": "h", "note": "an embedded array"}])),
        canonical_json(&json!({"note": "user entered", "values": [1, 2]})),
        canonical_json(&json!({"hanger_id": "hanger.invented.1"})),
        "{\"a\":1e400}".to_string(),
    ] {
        assert_eq!(site(&demo_with_hanger_provenance(&control)), None, "{control}");
    }
}

/// The prescan findings that leave a candidate unchecked (`ingress::prescan`'s
/// reasons, pinned here): every other finding leaves its string as text.
#[test]
fn rule_3b_unchecked_findings_are_the_prescans() {
    let deep = format!("{{\"a\":{}", "[".repeat(130));
    assert!(matches!(candidate(&deep), Candidate::Uncheckable));
    assert!(matches!(candidate(r#"{"$serde_json::private::RawValue":1}"#), Candidate::Uncheckable));
    assert!(matches!(candidate("{ not json"), Candidate::TextOnly));
    assert!(matches!(candidate("{\"a\":1} trailing"), Candidate::TextOnly));
    assert!(matches!(candidate("{\"a\":1}"), Candidate::Checkable(_)));
    let finding = crate::ingress::prescan(deep.as_bytes()).unwrap_err();
    assert_eq!(finding.reason, UNCHECKABLE_FINDINGS[0]);
    let finding = crate::ingress::prescan(br#"{"$serde_json::private::RawValue":1}"#).unwrap_err();
    assert_eq!(finding.reason, UNCHECKABLE_FINDINGS[1]);
    assert!(!is_candidate("x{") && is_candidate(" \t\r\n{") && !is_candidate("\u{a0}{"));
}

/// Rule 3 (b)'s lease (§2.7 "Cost", §6): the ingress lease grows once, by the
/// largest candidate's priced parse, before any embedded parse; the candidates
/// are parsed one at a time, so the measured peak stays within that growth and
/// near a model with the largest candidate alone. No growth happens when an
/// earlier rule finds a site or no candidate is checkable, and a refused
/// growth is returned before any parse. Deep and wide embedded values:
/// `rule_3b_scans_of_deep_and_wide_values_stay_within_the_lease`.
#[test]
fn rule_3b_grows_the_lease_once_and_holds_one_embedded_value() {
    let embedded = |members: usize| canonical_json(&json!({ "values": (0..members).map(|index| json!({"index": index, "note": format!("invented note {index}")})).collect::<Vec<_>>() }));
    let (small, middle, large) = (embedded(500), embedded(1_500), embedded(3_000));
    let priced = |text: &str| crate::command_ingress::text_parse_bytes(text, ArgsDecoding::Moved).unwrap();
    let largest = priced(&large);
    assert!(priced(&small) < priced(&middle) && priced(&middle) < largest);
    let mut model = parse(DEMO_MODEL);
    for (index, text) in [&small, &large, &middle].into_iter().enumerate() {
        model["supports"][index]["notes"] = json!(text);
    }
    let mut alone = parse(DEMO_MODEL);
    alone["supports"][1]["notes"] = json!(large);

    let (registry, mut held) = registry_and_lease();
    let before = held.bytes();
    let (found, measured) = crate::ingress::counting::measure(|| library_reference_site(&model, &mut held));
    assert_eq!(found, Ok(None));
    assert_eq!(held.bytes() - before, largest, "one growth, by the largest candidate's priced parse");
    assert!(measured.peak_priced <= largest, "peak {} within the growth {largest}", measured.peak_priced);
    let (_, single) = crate::ingress::counting::measure(|| library_reference_site(&alone, &mut lease()));
    assert!(
        measured.peak_priced <= single.peak_priced + priced(&small) / 2,
        "at most one embedded value is held: {} against {} alone",
        measured.peak_priced,
        single.peak_priced
    );

    // A growth above the budget the lease was admitted under is refused
    // (`ingress_exceeds_budget`, DEL-04-07) and returned as it is, before any
    // parse; the lease is unchanged.
    drop(held);
    let (pool_bytes, budget) = registry.pool().settings();
    registry.configure_pool(pool_bytes, 1024 + largest - 1, false).unwrap();
    // A lease keeps the budget it was admitted under: admit one under the lower budget.
    let mut low = registry.acquire_interactive_ingress(1024).unwrap();
    let (refused, refused_measure) = crate::ingress::counting::measure(|| library_reference_site(&model, &mut low));
    assert_eq!(refused, Err(ResourceRefusal::IngressExceedsBudget { required_bytes: 1024 + largest, budget_bytes: 1024 + largest - 1 }));
    assert_eq!(low.bytes(), 1024);
    registry.configure_pool(pool_bytes, budget, false).unwrap();
    assert!(refused_measure.peak_priced < priced(&small) / 4, "nothing was parsed: {}", refused_measure.peak_priced);
    // No growth when rule 2 finds a site first, or when no candidate is checkable.
    let mut marked = model.clone();
    marked["supports"][6]["provenance"] = json!("library:hanger:hangerlib.invented#h.1");
    let mut unchanged = lease();
    assert_eq!(library_reference_site(&marked, &mut unchanged).unwrap().unwrap().rule, LibraryRule::ProvenanceString);
    assert_eq!(library_reference_site(&parse(DEMO_MODEL), &mut unchanged), Ok(None));
    assert_eq!(unchanged.bytes(), 1024, "no growth");
}

/// What one embedded or model-side walk may allocate beyond the lease growth
/// (Codex round 1 P1): the returned site alone, its pointer in `M` (built
/// once, at its exact length) and the kind and id kept to `SHOWN_CHARS + 1`
/// characters (at most 4 bytes each), each priced by `allocation_envelope`.
/// The walks themselves allocate nothing per level.
fn site_allowance(pointer: &str) -> u64 {
    let envelope = |bytes: usize| crate::ingress::allocation_envelope(bytes as u64).unwrap();
    (envelope(pointer.len()) + 2 * envelope(4 * (SHOWN_CHARS + 1))) as u64
}

/// Codex round 1 P1 at `57450e1fc8`: rule 3 (b)'s scan of a parsed embedded
/// value collected every `library_ref` site and built each level's path, so a
/// candidate of 111,462 bytes (1,000 `{"library_ref": null}` leaves beneath 90
/// object levels whose keys are 1,000 characters each) priced at 1,542,832
/// bytes peaked at 145,166,048 priced bytes. Every walk now stops at its first
/// site and borrows its path, so the peak stays within the growth (and the
/// site's own allowance, [`site_allowance`]) for that shape, for a deeper one
/// (126 levels, the prescan's limit less the leaves' own), for a wide one
/// (100,000 sites in one list), for each with the site last and no
/// `library_ref` at all (every member visited), and for the same shapes in the
/// model itself (rule 1 and the private-class and rule-pack walks, with no
/// growth).
#[test]
fn rule_3b_scans_of_deep_and_wide_values_stay_within_the_lease() {
    let nested = |levels: usize, key_chars: usize, leaves: Value| {
        let mut value = leaves;
        for level in 0..levels {
            value = json!({ format!("{level:03}{}", "x".repeat(key_chars)): value });
        }
        value
    };
    let sites = |count: usize| json!({ "leaves": (0..count).map(|_| json!({"library_ref": null})).collect::<Vec<_>>() });
    let unmarked = |count: usize| json!({ "leaves": (0..count).map(|index| json!({"note": format!("invented {index}")})).collect::<Vec<_>>() });
    let mut last = unmarked(20_000);
    last["leaves"].as_array_mut().unwrap().push(json!({"zz": {"library_ref": null}}));
    let shapes: [(&str, Value, bool); 6] = [
        ("codex", nested(90, 1_000, sites(1_000)), true),
        ("deep", nested(124, 1_000, sites(1_000)), true),
        ("wide", nested(2, 10, sites(100_000)), true),
        ("deep, unmarked", nested(124, 1_000, unmarked(1_000)), false),
        ("wide, unmarked", nested(2, 10, unmarked(100_000)), false),
        ("wide, the site last", nested(2, 10, last), true),
    ];
    for (name, value, marked) in &shapes {
        let text = serde_json::to_string(value).unwrap();
        assert!(matches!(candidate(&text), Candidate::Checkable(_)), "{name}: the prescan admits it");
        let model = json!({ "note": text });
        let mut held = lease();
        let (found, measured) = crate::ingress::counting::measure(|| library_reference_site(&model, &mut held));
        let growth = held.bytes() - 1024;
        let found = found.unwrap();
        assert_eq!(found.is_some(), *marked, "{name}");
        if let Some(found) = &found {
            assert_eq!((found.pointer.as_str(), found.rule), ("/note", LibraryRule::LegacyHangerSelection), "{name}");
        }
        let allowance = site_allowance("/note");
        eprintln!("{name}: candidate {} bytes, growth {growth}, peak requested {}, peak priced {}", text.len(), measured.peak_requested, measured.peak_priced);
        assert!(growth > 0 && measured.peak_priced <= growth + allowance, "{name}: peak {} within the growth {growth} + {allowance}", measured.peak_priced);

        // The same value as the model itself: no growth; the walks allocate
        // nothing beyond the site they return.
        let mut unchanged = lease();
        let (found, measured) = crate::ingress::counting::measure(|| library_reference_site(value, &mut unchanged));
        assert_eq!(unchanged.bytes(), 1024, "{name}: no growth");
        let found = found.unwrap();
        assert_eq!(found.is_some(), *marked, "{name}");
        let pointer = found.as_ref().map_or(String::new(), |found| found.pointer.clone());
        if *marked {
            assert_eq!(Some(pointer.as_str()), library_ref_member_sites(value).first().map(|member| member.pointer.as_str()), "{name}");
        }
        assert!(measured.peak_priced <= site_allowance(&pointer), "{name}: model-side peak {} within the site {}", measured.peak_priced, site_allowance(&pointer));
        let manifest = json!({"active_rule_packs": []});
        let (refused, measured) = crate::ingress::counting::measure(|| (refuse_private_classes(value, "/model").is_ok(), refuse_rule_pack_referenced(value, &manifest).is_ok()));
        assert_eq!(refused, (true, true), "{name}");
        assert_eq!(measured.peak_priced, 0, "{name}: the private-class and rule-pack walks allocate nothing");
    }
}

// --- Rule packs and private classes -------------------------------------------

#[test]
fn user_release_rule_pack_reference_refused() {
    let vector = &parse(VECTORS)["vectors"][0];
    let model = parse(CONNECTED_MODEL);
    let manifest = &vector["input_manifest"];
    assert_eq!(refuse_rule_pack_referenced(&model, manifest), Ok(()));
    let mut referenced = model.clone();
    referenced["rule_pack_refs"] = json!([{"rule_pack_id": "invented-pack", "rule_pack_version": "1.0.0"}]);
    let refusal = refuse_rule_pack_referenced(&referenced, manifest).unwrap_err();
    assert!(refusal.starts_with(&format!("{RULE_PACK_REFERENCED}: the model's /rule_pack_refs/0 is a rule-pack reference")), "{refusal}");
    referenced["rule_pack_refs"] = json!(["positional"]);
    assert!(refuse_rule_pack_referenced(&referenced, manifest).is_err());
    let mut active = manifest.clone();
    active["active_rule_packs"] = json!([{"rule_pack_id": "invented-pack", "version": "1.0.0", "checksum_sha256": "a".repeat(64), "source_notice": "Invented."}]);
    assert!(refuse_rule_pack_referenced(&model, &active).unwrap_err().contains("records an active rule pack"));
    active["active_rule_packs"] = Value::Null;
    assert!(refuse_rule_pack_referenced(&model, &active).unwrap_err().contains("no active_rule_packs list"));
}

/// Codex round 1's private-class case (§6): a rule-pack class with an
/// unknown, `TBD` or null redistribution status is refused natively, and so
/// is each other non-project private class, under either member name. A
/// project or public class adds nothing.
#[test]
fn user_release_private_class_refused() {
    for redistribution in [json!("unknown"), json!("TBD"), Value::Null] {
        let request = json!({"report": {"sections": [{"note": {"privacy_classification": "private_rule_pack_data", "redistribution_status": redistribution}}]}});
        assert_eq!(
            refuse_private_classes(&request, "/request"),
            Err(format!("{PRIVATE_CLASS}: /request/report/sections/0/note/privacy_classification is private_rule_pack_data, a private class that is not the user's project data; a shareable report package cannot carry it."))
        );
    }
    for class in NON_PROJECT_PRIVATE_CLASSES {
        for member in ["privacy_classification", "privacy_class"] {
            let model = json!({"materials": [{"id": "m", member: class}]});
            assert!(refuse_private_classes(&model, "/model").unwrap_err().contains(&format!("/model/materials/0/{member} is {class}")));
        }
    }
    for class in ["private_project_data", "invented_public_example", "source_evidence"] {
        assert_eq!(refuse_private_classes(&json!({"a": {"privacy_classification": class}}), "/request"), Ok(()));
    }
    // A label elsewhere (a key or another member's value) adds nothing.
    assert_eq!(refuse_private_classes(&json!({"private_material_data": 1, "note": "private_material_data"}), "/request"), Ok(()));
    assert_eq!(refuse_private_classes(&parse(CONNECTED_MODEL), "/model"), Ok(()));
}

// --- The run record (§2.5 step 8) ---------------------------------------------

struct RunCase {
    model: Value,
    source: Value,
    record: Value,
    manifest: Value,
    manifest_hex: String,
}

fn run_case() -> RunCase {
    let vector = parse(VECTORS)["vectors"][0].clone();
    assert_eq!(vector["source_fixture"], "fixtures/results/preview_physics_connected_sparse.json");
    let manifest = vector["input_manifest"].clone();
    RunCase {
        model: parse(CONNECTED_MODEL),
        source: parse(CONNECTED_SPARSE),
        record: vector["run_record"].clone(),
        manifest_hex: sha256_hex(&canonical_json(&manifest)),
        manifest,
    }
}

impl RunCase {
    fn check(&self, record: &Value) -> Result<(), String> {
        let project = self.model["project"]["id"].as_str().unwrap();
        check_run_record(project, record, &self.source, &self.manifest, &self.manifest_hex, &UnmeteredReadMemory)
    }

    /// The record with `edit` applied and its own checksum recomputed, so only
    /// the check under test can refuse it.
    fn rehashed(&self, edit: impl FnOnce(&mut Value)) -> Value {
        let mut record = self.record.clone();
        edit(&mut record["analysis_run"]);
        let hash = analysis_runs::run_record_hash(&record).unwrap();
        record["analysis_run"]["hashes"][0]["value"] = json!(hash);
        record
    }
}

#[test]
fn user_release_run_record_admits_the_solves_own_record() {
    let case = run_case();
    assert_eq!(case.manifest_hex, parse(VECTORS)["vectors"][0]["expected"]["input_manifest_sha256"].as_str().unwrap());
    assert_eq!(case.check(&case.record), Ok(()));
    assert_eq!(case.check(&case.rehashed(|_| {})), Ok(()));
}

/// `user_release_rule_aggregate_refused` (§6): a run record whose status
/// carries `USER_RULE_CHECKED` or `USER_RULE_FAILED` (the session's composed
/// record), with its own checksum recomputed so Record Run's checks pass, is
/// refused by route 010's status check.
#[test]
fn user_release_rule_aggregate_refused() {
    let case = run_case();
    for aggregate in ["USER_RULE_CHECKED", "USER_RULE_FAILED"] {
        let record = case.rehashed(|run| {
            let mut statuses: Vec<String> = run["analysis_status"].as_array().unwrap().iter().map(|status| status.as_str().unwrap().replace("RULE_INPUTS_INCOMPLETE", aggregate)).collect();
            statuses.sort();
            run["analysis_status"] = json!(statuses);
        });
        let refusal = case.check(&record).unwrap_err();
        assert!(refusal.starts_with(&format!("{ANALYSIS_RECORD_UNBOUND}: the run record's analysis status is not the solve's own")), "{aggregate}: {refusal}");
    }
}

/// Each of route 010's run-record checks (§2.5 step 8), alone, and Record
/// Run's own checks through the factored function: every failure is
/// `REPORT-USER-RELEASE-ANALYSIS-RECORD-UNBOUND` with Record Run's detail.
#[test]
fn user_release_run_record_checks_refuse_each_failure() {
    let case = run_case();
    let unbound = |refusal: Result<(), String>, what: &str| {
        let refusal = refusal.unwrap_err();
        assert!(refusal.starts_with(&format!("{ANALYSIS_RECORD_UNBOUND}: ")), "{what}: {refusal}");
        refusal
    };
    // Route 010's builder members, each re-hashed.
    let refusal = unbound(case.check(&case.rehashed(|run| run["run_name"] = json!("invented name"))), "run name");
    assert!(refusal.contains("run_name"));
    let refusal = unbound(case.check(&case.rehashed(|run| run["reproducibility"]["unresolved_tbd"] = json!(["invented"]))), "unresolved");
    assert!(refusal.contains("unresolved TBDs"));
    let refusal = unbound(case.check(&case.rehashed(|run| run["professional_boundary"]["software_makes_approval_claim"] = json!(true))), "boundary");
    assert!(refusal.contains("professional boundary"));
    // Record Run's checks: the transient form, the record checksum, the received result, the
    // rows, the diagnostics and the manifest.
    let refusal = unbound(case.check(&case.rehashed(|run| run["created_at"] = json!("2026-10-27T00:00:00Z"))), "created_at");
    assert!(refusal.contains("ANALYSIS-RUN-NOT-TRANSIENT"));
    let mut forged = case.record.clone();
    forged["analysis_run"]["run_name"] = json!("invented name");
    assert!(unbound(case.check(&forged), "checksum").contains("own checksum does not verify"));
    let mut source = case.source.clone();
    source["results"][0]["value"] = json!(source["results"][0]["value"].as_f64().unwrap() + 1.0);
    let project = case.model["project"]["id"].as_str().unwrap();
    assert!(unbound(check_run_record(project, &case.record, &source, &case.manifest, &case.manifest_hex, &UnmeteredReadMemory), "source").contains("ANALYSIS-RUN-RESULT-MISMATCH"));
    let refusal = unbound(case.check(&case.rehashed(|run| {
        run["diagnostics"].as_array_mut().unwrap().push(json!({"source_annotation": analysis_runs::rule_pack_ref_diagnostic(analysis_runs::RULE_PACK_REF_UNRESOLVED, "invented@1")}));
    })), "rule-pack diagnostic");
    assert!(refusal.contains("ANALYSIS-RUN-MANIFEST-MISMATCH"), "{refusal}");
    assert!(unbound(check_run_record(project, &case.record, &case.source, &case.manifest, &"0".repeat(64), &UnmeteredReadMemory), "manifest").contains("ANALYSIS-RUN-MANIFEST-MISMATCH"));
    assert!(unbound(check_run_record("project:another", &case.record, &case.source, &case.manifest, &case.manifest_hex, &UnmeteredReadMemory), "project").contains("ANALYSIS-RUN-MANIFEST-MISMATCH"));
}
