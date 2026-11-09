//! DEL-04-09 Design §5.3.2: the subject census. Each forged document below
//! changes the identity, discretization or supports tables (resealed: row
//! checksums and table digests recomputed) and leaves `binding.subject_census`
//! unchanged or adjusted to the forged tables' counts, so nothing fails on a
//! stale hash. Payload alone, a count the tables contradict fails and anything
//! else stays scoped (never passed on the producer's claim); with the bound
//! request, every missing, extra, duplicate or substituted subject fails on
//! coverage against the model's own subject list. All values are invented.
use open_pipe_stress_modal_result::{modal_reader_sha256, read_modal_result, ModalReadStanding, ModalReadStatus, ModalSupplied, MODAL_RELATIONS};
use serde_json::{json, Value};

macro_rules! fixture {
    ($name:literal) => {
        (
            $name,
            include_str!(concat!("../../../../fixtures/results/", $name, ".json")),
            include_str!(concat!("../../../../fixtures/results/", $name, ".request.json")),
        )
    };
}

type Fixture = (&'static str, &'static str, &'static str);

const BODY: Fixture = fixture!("modal_component_body");
const ARC: Fixture = fixture!("modal_arc");
const CANTILEVER: Fixture = fixture!("modal_cantilever_whole_spectrum");
const PRESSED: Fixture = fixture!("modal_state_one_way_pressed");
const SLIDING: Fixture = fixture!("modal_state_friction_sliding");
const REFUSED: Fixture = fixture!("modal_refused_no_bound");
const LINEAR: [Fixture; 8] = [
    BODY,
    ARC,
    CANTILEVER,
    fixture!("modal_ex_n05"),
    fixture!("modal_cantilever_cluster_extension"),
    fixture!("modal_cantilever_below_cutoff"),
    fixture!("modal_cantilever_subnormal_cutoff"),
    fixture!("modal_cantilever_sensitive"),
];

fn load(fixture: Fixture) -> (Value, Value) {
    (serde_json::from_str(fixture.1).unwrap(), serde_json::from_str(fixture.2).unwrap())
}

fn sha(value: &Value) -> String {
    modal_reader_sha256(value).unwrap()
}

struct Read {
    closure: ModalReadStanding,
    binding: ModalReadStanding,
    mass: ModalReadStanding,
    evidence: ModalReadStanding,
    validation: &'static str,
}

fn read(doc: &Value, request: Option<&Value>) -> Read {
    let verdict = read_modal_result(doc, ModalSupplied { request });
    let at = |name: &str| verdict.relations[MODAL_RELATIONS.iter().position(|r| *r == name).unwrap()].clone();
    Read { closure: verdict.closure.clone(), binding: verdict.binding.clone(), mass: at("mass"), evidence: at("evidence"), validation: verdict.modal_validation() }
}

/// Renumbers ordinals (and positional row ids), then recomputes every row
/// checksum and table digest.
fn reseal(doc: &mut Value) {
    for (name, table) in doc["tables"].as_object_mut().unwrap().iter_mut() {
        if table["status"] != "published" {
            continue;
        }
        let mut checksums = Vec::new();
        for (i, row) in table["rows"].as_array_mut().unwrap().iter_mut().enumerate() {
            row["ordinal"] = json!(i);
            if name == "identity" {
                row["row_id"] = json!(format!("identity:{i}"));
            }
            let mut body = row.as_object().unwrap().clone();
            body.remove("checksum");
            let checksum = sha(&Value::Object(body));
            row["checksum"] = json!(checksum);
            checksums.push(json!(checksum));
        }
        table["row_count"] = json!(checksums.len());
        table["digest"] = json!(sha(&Value::Array(checksums)));
    }
}

/// Binds the document to a changed request (hashes, run id, state case).
fn rebind(doc: &mut Value, request: &Value) {
    let b = &mut doc["binding"];
    b["request_sha256"] = json!(sha(request));
    b["model_sha256"] = json!(sha(&request["model"]));
    b["materials_sha256"] = json!(sha(request.get("materials").unwrap_or(&json!([]))));
    let run = sha(&json!({"request_sha256": b["request_sha256"], "integrity_parameters_sha256": b["integrity_parameters_sha256"]}));
    if !b["state_case"].is_null() {
        let index = b["state_case"]["case_index"].as_u64().unwrap() as usize;
        b["state_case"]["case_sha256"] = json!(sha(&request["model"]["load_cases"][index]));
    }
    doc["run"]["run_id"] = json!(format!("modal-run:{}", &run[..16]));
}

fn rows<'a>(doc: &'a mut Value, table: &str) -> &'a mut Vec<Value> {
    doc["tables"][table]["rows"].as_array_mut().unwrap()
}

fn census(doc: &mut Value) -> &mut Value {
    &mut doc["binding"]["subject_census"]
}

fn bump(doc: &mut Value, count: &str, by: i64) {
    let c = census(doc);
    c[count] = json!(c[count].as_i64().unwrap() + by);
}

/// Removes a subject's identity rows (every axis).
fn drop_identity(doc: &mut Value, kind: &str, subject: &str) {
    let rows = rows(doc, "identity");
    let before = rows.len();
    rows.retain(|r| !(r["subject_kind"] == kind && r["subject"] == subject));
    assert_eq!(before - rows.len(), 3, "{kind} {subject} has a row along each axis");
}

/// Adds a copy of a subject's identity rows (every axis) under another name,
/// just before each axis's aggregate row.
fn copy_identity(doc: &mut Value, kind: &str, subject: &str, new_kind: &str, new: &str) {
    let rows = rows(doc, "identity");
    for axis in ["x", "y", "z"] {
        let mut row = rows.iter().find(|r| r["subject_kind"] == kind && r["subject"] == subject && r["axis"] == axis).unwrap().clone();
        row["subject_kind"] = json!(new_kind);
        row["subject"] = json!(new);
        let at = rows.iter().position(|r| r["subject_kind"] == "model" && r["axis"] == axis).unwrap();
        rows.insert(at, row);
    }
}

fn rename_identity(doc: &mut Value, kind: &str, subject: &str, new: &str) {
    let mut hits = 0;
    for r in rows(doc, "identity").iter_mut().filter(|r| r["subject_kind"] == kind && r["subject"] == subject) {
        r["subject"] = json!(new);
        hits += 1;
    }
    assert_eq!(hits, 3);
}

fn drop_member(doc: &mut Value, member: &str) {
    let rows = rows(doc, "discretization");
    let before = rows.len();
    rows.retain(|r| r["member_ref"] != member);
    assert_eq!(before - rows.len(), 1);
}

fn copy_member(doc: &mut Value, member: &str, new: &str) {
    let rows = rows(doc, "discretization");
    let mut row = rows.iter().find(|r| r["member_ref"] == member).unwrap().clone();
    row["member_ref"] = json!(new);
    row["row_id"] = json!(format!("member:{new}"));
    rows.push(row);
}

fn rename_member(doc: &mut Value, member: &str, new: &str) {
    let row = rows(doc, "discretization").iter_mut().find(|r| r["member_ref"] == member).unwrap();
    row["member_ref"] = json!(new);
    row["row_id"] = json!(format!("member:{new}"));
}

fn copy_support(doc: &mut Value, support: &str, new: &str) {
    let rows = rows(doc, "supports");
    let mut row = rows.iter().find(|r| r["support_id"] == support).unwrap().clone();
    row["support_id"] = json!(new);
    row["row_id"] = json!(format!("support:{new}"));
    rows.push(row);
}

fn rename_support(doc: &mut Value, support: &str, new: &str) {
    let row = rows(doc, "supports").iter_mut().find(|r| r["support_id"] == support).unwrap();
    row["support_id"] = json!(new);
    row["row_id"] = json!(format!("support:{new}"));
}

fn is_census_scope(s: &str) -> bool {
    s.starts_with("subject census:")
}

#[track_caller]
fn assert_scoped_by_census(r: &Read, what: &str) {
    assert_eq!(r.closure.status, ModalReadStatus::Passed, "{what}: {:?}", r.closure);
    assert_eq!(r.evidence.status, ModalReadStatus::ScopedUnsupported, "{what}: {:?}", r.evidence);
    assert!(r.evidence.scope.iter().any(|s| is_census_scope(s)), "{what}: {:?}", r.evidence);
}

#[track_caller]
fn assert_evidence_failed(r: &Read, needle: &str, what: &str) {
    assert_eq!(r.closure.status, ModalReadStatus::Passed, "{what}: {:?}", r.closure);
    assert_eq!(r.evidence.status, ModalReadStatus::Failed, "{what}: {:?}", r.evidence);
    let difference = r.evidence.difference.as_deref().unwrap_or("");
    assert!(difference.contains(needle), "{what}: `{difference}` lacks `{needle}`");
}

/// A forgery read both ways: payload alone (scoped, or failed on a count the
/// tables contradict, `payload_fails`), and with the bound request (failed).
#[track_caller]
fn both_ways(doc: &Value, request: &Value, payload_fails: Option<&str>, bound_needle: &str, what: &str) {
    let alone = read(doc, None);
    match payload_fails {
        Some(needle) => assert_evidence_failed(&alone, needle, &format!("{what} (payload)")),
        None => assert_scoped_by_census(&alone, &format!("{what} (payload)")),
    }
    let bound = read(doc, Some(request));
    assert_eq!(bound.binding.status, ModalReadStatus::Passed, "{what}: {:?}", bound.binding);
    assert_evidence_failed(&bound, bound_needle, &format!("{what} (bound)"));
}

// ---------------------------------------------------------------------------
// Intact controls and shape.

#[test]
fn intact_fixtures_pass_the_census_with_the_request_and_stay_scoped_without() {
    for fixture in LINEAR {
        let (doc, request) = load(fixture);
        let r = read(&doc, Some(&request));
        assert_eq!(r.evidence.status, ModalReadStatus::Passed, "{}: {:?}", fixture.0, r.evidence);
        // Without the request, every count agrees with the tables and evidence stays scoped.
        let r = read(&doc, None);
        assert_scoped_by_census(&r, fixture.0);
        assert_eq!(r.validation, "not_fully_validated", "{}", fixture.0);
    }
    // A reported nonlinear state stays conditional: mass, participation and evidence unmet.
    for fixture in [PRESSED, SLIDING] {
        let (doc, request) = load(fixture);
        let r = read(&doc, Some(&request));
        assert_eq!(r.evidence.status, ModalReadStatus::ScopedUnsupported, "{}: {:?}", fixture.0, r.evidence);
        assert!(!r.evidence.scope.iter().any(|s| is_census_scope(s)), "{}: {:?}", fixture.0, r.evidence);
        assert!(r.evidence.scope.iter().any(|s| s.contains("reported nonlinear support states are conditional")), "{}: {:?}", fixture.0, r.evidence);
        assert_eq!(r.mass.status, ModalReadStatus::ScopedUnsupported, "{}", fixture.0);
        assert_eq!(r.validation, "not_fully_validated", "{}", fixture.0);
    }
    // A refused document's census is null.
    let (doc, request) = load(REFUSED);
    assert!(doc["binding"]["subject_census"].is_null());
    let r = read(&doc, Some(&request));
    assert_eq!((r.closure.status, r.binding.status), (ModalReadStatus::Passed, ModalReadStatus::Passed));
}

#[test]
fn the_census_shape_is_an_object_when_solved_and_null_when_refused() {
    let (solved, _) = load(BODY);
    let (refused, _) = load(REFUSED);
    let census_of = |doc: &Value| doc["binding"]["subject_census"].clone();
    let object = census_of(&solved);
    let cases: Vec<(&str, &Value, Value, &str)> = vec![
        ("solved, null", &solved, Value::Null, "/binding/subject_census: not an object"),
        ("refused, an object", &refused, object.clone(), "a refused document forms no subject census"),
        ("a count missing", &solved, { let mut c = object.clone(); c.as_object_mut().unwrap().remove("arcs"); c }, "missing `arcs`"),
        ("an undeclared member", &solved, { let mut c = object.clone(); c["bends"] = json!(0); c }, "undeclared `bends`"),
        ("a negative count", &solved, { let mut c = object.clone(); c["supports"] = json!(-1); c }, "supports: not a non-negative integer"),
        ("a fractional count", &solved, { let mut c = object.clone(); c["arcs"] = json!(0.5); c }, "arcs: not a non-negative integer"),
        ("an upper-case digest", &solved, { let mut c = object.clone(); c["subjects_sha256"] = json!(c["subjects_sha256"].as_str().unwrap().to_uppercase()); c }, "not a lowercase SHA-256"),
    ];
    for (what, doc, value, needle) in cases {
        let mut doc = doc.clone();
        doc["binding"]["subject_census"] = value;
        let r = read(&doc, None);
        assert_eq!(r.closure.status, ModalReadStatus::Failed, "{what}: {:?}", r.closure);
        assert!(r.closure.difference.as_deref().unwrap_or("").contains(needle), "{what}: {:?}", r.closure);
    }
    // An absent census fails closure too.
    let mut doc = solved.clone();
    doc["binding"].as_object_mut().unwrap().remove("subject_census");
    assert_eq!(read(&doc, None).closure.status, ModalReadStatus::Failed);
}

// ---------------------------------------------------------------------------
// Forged tables: straight member, arc, component body, support.

#[test]
fn a_straight_member_missing_extra_duplicated_or_substituted_fails() {
    let (base, request) = load(ARC);
    // Missing, census unchanged: the tables contradict the counts.
    let mut doc = base.clone();
    drop_identity(&mut doc, "member", "pipe:a-b");
    drop_member(&mut doc, "pipe:a-b");
    reseal(&mut doc);
    both_ways(&doc, &request, Some("nodal member rows along x"), "nodal member rows along x", "missing straight, census unchanged");
    // Missing, census adjusted: only the bound model sees it.
    bump(&mut doc, "straight_members", -1);
    both_ways(&doc, &request, None, "missing [\"pipe:a-b\"]", "missing straight, census adjusted");
    // Extra.
    let mut doc = base.clone();
    copy_identity(&mut doc, "member", "pipe:a-b", "member", "pipe:ghost");
    copy_member(&mut doc, "pipe:a-b", "pipe:ghost");
    reseal(&mut doc);
    both_ways(&doc, &request, Some("3 nodal member rows along x"), "", "extra straight, census unchanged");
    bump(&mut doc, "straight_members", 1);
    both_ways(&doc, &request, None, "not in the model [\"pipe:ghost\"]", "extra straight, census adjusted");
    // Duplicated: the identity table lists it twice (coverage).
    let mut doc = base.clone();
    copy_identity(&mut doc, "member", "pipe:a-b", "member", "pipe:a-b");
    reseal(&mut doc);
    both_ways(&doc, &request, Some("compared twice"), "compared twice", "duplicated straight");
    // Substituted: counts agree, the identity differs.
    let mut doc = base.clone();
    rename_identity(&mut doc, "member", "pipe:a-b", "pipe:forged");
    rename_member(&mut doc, "pipe:a-b", "pipe:forged");
    reseal(&mut doc);
    both_ways(&doc, &request, None, "nodal members along x are not the bound model's straight members", "substituted straight");
}

#[test]
fn an_arc_missing_extra_duplicated_or_substituted_fails() {
    let (base, request) = load(ARC);
    let mut doc = base.clone();
    drop_identity(&mut doc, "member", "pipe:b-c");
    reseal(&mut doc);
    both_ways(&doc, &request, Some("resultant member rows along x"), "", "missing arc, census unchanged");
    bump(&mut doc, "arcs", -1);
    both_ways(&doc, &request, None, "missing [\"pipe:b-c\"]", "missing arc, census adjusted");
    let mut doc = base.clone();
    copy_identity(&mut doc, "member", "pipe:b-c", "member", "pipe:ghost-arc");
    reseal(&mut doc);
    both_ways(&doc, &request, Some("resultant member rows along x"), "", "extra arc, census unchanged");
    bump(&mut doc, "arcs", 1);
    both_ways(&doc, &request, None, "not in the model [\"pipe:ghost-arc\"]", "extra arc, census adjusted");
    let mut doc = base.clone();
    copy_identity(&mut doc, "member", "pipe:b-c", "member", "pipe:b-c");
    reseal(&mut doc);
    both_ways(&doc, &request, Some("compared twice"), "compared twice", "duplicated arc");
    let mut doc = base.clone();
    rename_identity(&mut doc, "member", "pipe:b-c", "pipe:forged-arc");
    reseal(&mut doc);
    both_ways(&doc, &request, None, "resultant members along x are not the bound model's arcs", "substituted arc");
    // An arc presented as a straight member (nodal, with a discretization row).
    let mut doc = base.clone();
    for r in rows(&mut doc, "identity").iter_mut().filter(|r| r["subject"] == "pipe:b-c") {
        r["comparison"] = json!("nodal");
    }
    copy_member(&mut doc, "pipe:a-b", "pipe:b-c");
    reseal(&mut doc);
    bump(&mut doc, "arcs", -1);
    bump(&mut doc, "straight_members", 1);
    both_ways(&doc, &request, None, "not the bound model's straight members", "arc presented as straight");
}

#[test]
fn a_component_body_missing_extra_duplicated_or_substituted_fails() {
    let (base, request) = load(BODY);
    let mut doc = base.clone();
    drop_identity(&mut doc, "component_body", "component:valve");
    reseal(&mut doc);
    both_ways(&doc, &request, Some("component-body rows along x"), "", "missing body, census unchanged");
    bump(&mut doc, "component_bodies", -1);
    both_ways(&doc, &request, None, "missing [\"component:valve\"]", "missing body, census adjusted");
    let mut doc = base.clone();
    copy_identity(&mut doc, "component_body", "component:valve", "component_body", "component:ghost");
    reseal(&mut doc);
    both_ways(&doc, &request, Some("component-body rows along x"), "", "extra body, census unchanged");
    bump(&mut doc, "component_bodies", 1);
    both_ways(&doc, &request, None, "not in the model [\"component:ghost\"]", "extra body, census adjusted");
    let mut doc = base.clone();
    copy_identity(&mut doc, "component_body", "component:valve", "component_body", "component:valve");
    reseal(&mut doc);
    both_ways(&doc, &request, Some("compared twice"), "compared twice", "duplicated body");
    let mut doc = base.clone();
    rename_identity(&mut doc, "component_body", "component:valve", "component:forged");
    reseal(&mut doc);
    both_ways(&doc, &request, None, "component bodies along x are not the bound model's component bodies", "substituted body");
    // Kind is identity: the body presented as a resultant member (an arc) of the same name.
    let mut doc = base.clone();
    for r in rows(&mut doc, "identity").iter_mut().filter(|r| r["subject_kind"] == "component_body") {
        r["subject_kind"] = json!("member");
    }
    reseal(&mut doc);
    both_ways(&doc, &request, Some("resultant member rows along x"), "", "body as a member, census unchanged");
    bump(&mut doc, "component_bodies", -1);
    bump(&mut doc, "arcs", 1);
    both_ways(&doc, &request, None, "not the bound model's arcs", "body as a member, census adjusted");
    // The body's member listed as a straight member instead of the body.
    let mut doc = base.clone();
    drop_identity(&mut doc, "component_body", "component:valve");
    copy_identity(&mut doc, "member", "pipe:p1", "member", "body:v");
    copy_member(&mut doc, "pipe:p1", "body:v");
    reseal(&mut doc);
    bump(&mut doc, "component_bodies", -1);
    bump(&mut doc, "straight_members", 1);
    both_ways(&doc, &request, None, "not in the model [\"body:v\"]", "body's span listed as straight");
}

#[test]
fn a_support_missing_extra_or_substituted_fails() {
    let (base, request) = load(PRESSED);
    // Missing: the linearized case then lists no support (state coverage).
    let mut doc = base.clone();
    rows(&mut doc, "supports").clear();
    reseal(&mut doc);
    both_ways(&doc, &request, Some("supports"), "", "missing support, census unchanged");
    bump(&mut doc, "nonlinear_supports", -1);
    both_ways(&doc, &request, Some("lists its supports"), "lists its supports", "missing support, census adjusted");
    // Extra: the anchor reported as a nonlinear support.
    let mut doc = base.clone();
    copy_support(&mut doc, "support:NL-U", "support:ANCHOR");
    reseal(&mut doc);
    both_ways(&doc, &request, Some("supports table has 2 rows"), "", "extra support, census unchanged");
    bump(&mut doc, "nonlinear_supports", 1);
    both_ways(&doc, &request, None, "not in the model [\"support:ANCHOR\"]", "extra support, census adjusted");
    // More nonlinear supports than supports.
    let mut doc = base.clone();
    census(&mut doc)["supports"] = json!(0);
    let r = read(&doc, None);
    assert_evidence_failed(&r, "exceed", "nonlinear above supports");
    // Duplicated: the supports table's row ids are unique (closure).
    let mut doc = base.clone();
    copy_support(&mut doc, "support:NL-U", "support:NL-U");
    reseal(&mut doc);
    assert_eq!(read(&doc, None).closure.status, ModalReadStatus::Failed);
    // Substituted.
    let mut doc = base.clone();
    rename_support(&mut doc, "support:NL-U", "support:NL-X");
    reseal(&mut doc);
    both_ways(&doc, &request, None, "supports table's supports are not the bound model's nonlinear supports", "substituted support");
    // The all-supports count: no table lists it, so only the request holds it.
    let mut doc = base.clone();
    bump(&mut doc, "supports", 1);
    both_ways(&doc, &request, None, "supports is 3, but the bound model has 2", "support count forged");
}

// ---------------------------------------------------------------------------
// The model side: the bound request's subjects against an unchanged document.

#[test]
fn a_bound_model_with_other_subjects_fails() {
    let rebound = |fixture: Fixture, mutate: &dyn Fn(&mut Value)| -> Read {
        let (mut doc, mut request) = load(fixture);
        mutate(&mut request);
        rebind(&mut doc, &request);
        let r = read(&doc, Some(&request));
        assert_eq!(r.binding.status, ModalReadStatus::Passed, "{:?}", r.binding);
        r
    };
    let model_support = |r: &mut Value, support: Value| r["model"]["supports"].as_array_mut().unwrap().push(support);
    let cases: Vec<(&str, Fixture, Box<dyn Fn(&mut Value)>, &str)> = vec![
        ("a massless support added", BODY, Box::new(move |r| model_support(r, json!({"id": "s2", "node": "a", "family": "constant_effort_support", "restraints": []}))),
            "supports is 1, but the bound model has 2"),
        ("a duplicate support record", BODY, Box::new(|r| { let s = r["model"]["supports"][0].clone(); r["model"]["supports"].as_array_mut().unwrap().push(s) }),
            "supports is 1, but the bound model has 2"),
        ("the support renamed", BODY, Box::new(|r| r["model"]["supports"][0]["id"] = json!("s-renamed")), "subjects_sha256"),
        ("the body's component renamed", BODY, Box::new(|r| r["model"]["components"][0]["id"] = json!("component:other")),
            "component bodies along x are not the bound model's"),
        ("the body unmapped", BODY, Box::new(|r| r["model"]["components"][0]["geometry"]["rigid_pipe_ref"] = json!("pipe:p1-absent")),
            "nodal members along x are not the bound model's straight members"),
        ("the arc unrealized", ARC, Box::new(|r| r["model"]["components"][0]["mechanics_interface"]["solver_consumption"] = json!(MM_GEOMETRY_ONLY)),
            "nodal members along x are not the bound model's straight members"),
        ("a pipe renamed", ARC, Box::new(|r| r["model"]["pipe_segments"][0]["id"] = json!("pipe:other")),
            "nodal members along x are not the bound model's straight members"),
        ("a nonlinear support renamed", PRESSED, Box::new(|r| r["model"]["supports"][1]["id"] = json!("support:NL-X")),
            "supports table's supports are not the bound model's nonlinear supports"),
        ("the nonlinear record removed", PRESSED, Box::new(|r| r["model"]["supports"][1]["nonlinear"] = Value::Null),
            "supports table's supports are not the bound model's nonlinear supports"),
    ];
    for (what, fixture, mutate, needle) in cases {
        let r = rebound(fixture, &*mutate);
        assert_evidence_failed(&r, needle, what);
        assert!(r.evidence.difference.as_deref().unwrap().starts_with("model-derived"), "{what}: {:?}", r.evidence);
    }
    // A supplied request that is not the bound one derives nothing: scoped, not failed.
    let (doc, mut request) = load(BODY);
    request["model"]["supports"][0]["id"] = json!("s-renamed");
    let r = read(&doc, Some(&request));
    assert_eq!(r.binding.status, ModalReadStatus::Failed);
    assert_eq!(r.evidence.status, ModalReadStatus::ScopedUnsupported, "{:?}", r.evidence);
    assert!(r.evidence.scope.iter().any(|s| s.contains("not the bound request")), "{:?}", r.evidence);
}

const MM_GEOMETRY_ONLY: &str = "mechanics_geometry_only";

#[test]
fn the_census_digest_and_counts_are_held_to_the_bound_model() {
    let (base, request) = load(CANTILEVER);
    // A digest that is not the model's: payload alone cannot see it.
    let mut doc = base.clone();
    census(&mut doc)["subjects_sha256"] = json!("0".repeat(64));
    both_ways(&doc, &request, None, "subjects_sha256", "digest forged");
    // The digest is the canonical sorted list's.
    let (body, request) = load(BODY);
    let expected = sha(&json!([
        {"kind": "component_body", "id": "component:valve"},
        {"kind": "straight_member", "id": "pipe:p1"},
        {"kind": "support", "id": "s"},
    ]));
    assert_eq!(body["binding"]["subject_census"]["subjects_sha256"], json!(expected));
    assert_eq!(read(&body, Some(&request)).evidence.status, ModalReadStatus::Passed);
}

// ---------------------------------------------------------------------------
// Structural: no subject is dropped for a mass quantity.

#[test]
fn an_unresolved_or_refused_mass_drops_no_subject() {
    // A body outside the rounding model (its double-word low word may underflow): the
    // mass is unresolved, the body is still a subject and the census passes.
    let (mut doc, mut request) = load(BODY);
    request["model"]["components"][0]["geometry"]["weight"] = json!({"unit": "N", "value": 2.3195656276910255e-308});
    request["model"]["components"][0]["geometry"]["center_of_gravity"] = json!({"unit": "m", "x": 250575.49268495973, "y": 2.116592064317826, "z": 0});
    request["weight_to_mass_acceleration"] = json!({"unit": "m/s^2", "value": 1.0});
    rebind(&mut doc, &request);
    doc["basis"]["weight_to_mass_acceleration"]["value"] = json!(1.0);
    doc["binding"]["options"]["weight_to_mass_acceleration"]["value"] = json!(1.0);
    let r = read(&doc, Some(&request));
    assert_eq!(r.binding.status, ModalReadStatus::Passed, "{:?}", r.binding);
    assert_eq!(r.mass.status, ModalReadStatus::Unresolved, "{:?}", r.mass);
    assert_eq!(r.evidence.status, ModalReadStatus::Passed, "{:?}", r.evidence);
    // A body the producer refuses (outside the range premise) fails the mass relation
    // only; it stays a body in the subject list.
    let (mut doc, mut request) = load(BODY);
    request["model"]["components"][0]["geometry"]["center_of_gravity"] = json!({"unit": "m", "x": 1e-200, "y": 0, "z": 0});
    rebind(&mut doc, &request);
    let r = read(&doc, Some(&request));
    assert_eq!(r.mass.status, ModalReadStatus::Failed, "{:?}", r.mass);
    assert!(r.mass.difference.as_deref().unwrap().contains("MODAL_COMPONENT_MASS_OUT_OF_RANGE"));
    assert_eq!(r.evidence.status, ModalReadStatus::Passed, "{:?}", r.evidence);
    // A member whose frame conditioning is outside the rounding model: unresolved mass,
    // every member still a subject.
    let (mut doc, mut request) = load(CANTILEVER);
    for pipe in request["model"]["pipe_segments"].as_array_mut().unwrap() {
        pipe["y_reference"] = json!({"x": 9e15, "y": 1e-7, "z": 0});
    }
    rebind(&mut doc, &request);
    let r = read(&doc, Some(&request));
    assert_eq!(r.mass.status, ModalReadStatus::Unresolved, "{:?}", r.mass);
    assert_eq!(r.evidence.status, ModalReadStatus::Passed, "{:?}", r.evidence);
}

#[test]
fn a_route_outside_the_model_basis_keeps_the_census_scoped() {
    // An objective connector on the pressure-free route is outside the model basis: the
    // census is not re-derived, and evidence is scoped with that reason, never passed.
    let (mut doc, mut request) = load(ARC);
    request["model"]["components"].as_array_mut().unwrap().push(json!({
        "id": "component:joint", "kind": "expansion_joint", "node": "a",
        "objective_connector": {"topology": {"span_ref": "pipe:a-b"}}}));
    rebind(&mut doc, &request);
    let r = read(&doc, Some(&request));
    assert_eq!(r.binding.status, ModalReadStatus::Passed, "{:?}", r.binding);
    assert_eq!(r.evidence.status, ModalReadStatus::ScopedUnsupported, "{:?}", r.evidence);
    assert!(r.evidence.scope.iter().any(|s| s.starts_with("subject census not re-derived") && s.contains("pressure-free")), "{:?}", r.evidence);
}
