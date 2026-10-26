"""SD-NF, SD-L1, SD-ID, SD-EV, SD-WT, SD-DT, SD-RK and the SD owners (retained-pricing-rules.md §8.19), and SD-NR for
the nested records (§8.25): the silent-drop guard's rules apply only where its code exists, zero the
closed-record scan's finding path by its place before admission, bound its loops and texts, re-key two replaced
combination audit keys by name, and stop on a changed source."""
from pathlib import Path
import runpy
import shutil
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
FIELDS = "core/product_physics/src/model_fields.rs"
LIB = "core/product_physics/src/lib.rs"
PREVIEW = "core/product_physics/src/preview_physics.rs"
VALIDATION = "core/product_physics/src/validation.rs"
sys.path.insert(0, str(PRICING / "base"))
import sd_owners_model  # noqa: E402


def decisions(monkeypatch, snapshot=SOURCE, route="E"):
    monkeypatch.setenv("SQ2_SNAP", str(snapshot))
    return runpy.run_path(str(PRICING / f"rules/sq2_decisions_{route}.py"))


def line_of(path, text):
    hits = [i + 1 for i, line in enumerate((SOURCE / path).read_text(encoding="utf-8").split("\n")) if text in line]
    assert len(hits) == 1, (text, hits)
    return hits[0]


def snapshot_with(tmp_path, path, old, new):
    """A copy of the source tree with one replacement in `path` (the product crates' sources only)."""
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    target = tmp_path / path
    text = target.read_text(encoding="utf-8")
    assert text.count(old) == 1
    target.write_text(text.replace(old, new), encoding="utf-8")
    return tmp_path


def test_the_scan_finding_path_is_zeroed_and_its_loops_bound(monkeypatch):
    d = decisions(monkeypatch)
    zeros = {str(k) for k, why in d["O10_SITE_ZERO"] if "SD-NF" in why}
    assert zeros == {f"model_fields.rs:{line_of(FIELDS, t)}" for t in (
        "findings.keep_finding(|| Finding { field: key.clone(), record: name(), path: path.to_string() });",
        "findings.keep_finding(|| Finding { field: key.clone(), record: at.finding_name(), path: at.finding_path() });",
        'findings.keep_finding(|| NonObject { path: format!("{prefix}{collection}[{i}]"), is: value_kind(record) });',
        'return Some(listed_refusal(MODEL_RECORD_NOT_OBJECT, &non_objects, |f| format!("{} is {}", f.path, f.is),',
        'Some(listed_refusal(UNKNOWN_MODEL_FIELD, &findings, |f| format!("unknown field `{}` in {} ({})", f.field, f.record, f.path),')}
    loops = {h: (b, fns) for h, b, why, fns in d["O10_LOOPS_LAST"] if "SD-L1" in why}
    assert loops == {
        'for (i, case) in objects(model, " ")': ("cases", ["model_fields.rs:unknown_field_refusal"]),
        'for (j, load) in objects(case, " ")': ("l", ["model_fields.rs:unknown_field_refusal"]),
        'for (i, combination) in objects(model, " ")': ("z", ["model_fields.rs:unknown_field_refusal"]),
        'for (j, term) in objects(combination, " ")': ("3", ["model_fields.rs:unknown_field_refusal"]),
        # SD-NR: the component geometry's 31 declared members bound one record's keys in both functions.
        "for key in record.keys()": ("31", ["model_fields.rs:check", "model_fields.rs:check_nested"]),
    }
    edges = [(str(a), str(b)) for a, b, why in d["O10_EDGE_ZERO"] if "SD-EV" in why]
    assert len(edges) == 1 and edges[0][0].endswith(":keep_finding") and ":push" in edges[0][1] and "load_ledger.rs" in edges[0][1]


def test_the_nested_walk_is_bound_and_its_names_formed_only_for_a_finding(monkeypatch):
    """SD-NR: the walk's loops are bound first, only in their functions; the frame's name and path are reached only
    through check_nested's zeroed push, their recursion is reviewed, and each of their sites is audited."""
    d = decisions(monkeypatch, route="L")
    walk = ["model_fields.rs:nested_records"]
    loops = {h: (b, fns) for h, b, why, fns in d["O10_LOOPS_FIRST"] if "SD-NR" in why}
    assert loops == {
        'for (i, node) in objects(model, " ")': ("n", walk),
        'for (i, pipe) in objects(model, " ")': ("m", walk),
        'for (i, section) in objects(model, " ")': ("0", walk),
        "for (name, quantity) in properties": ("0", walk),
        'for (i, support) in objects(model, " ")': ("g", walk),
        'for (i, component) in objects(model, " ")': ("0", walk),
        'for (owner, collection, kind) in [(model, " ", " "), (request, " ", " ")]': ("2", walk),
        'for (i, material) in objects(owner, " ")': ("4", walk),
        'for (k, point) in objects(material, " ")': ("pts", walk),
        "for key in keys": ("13", ["model_fields.rs:nested_quantities"]),
        # The non-object pass: the two owners, the model's eight record collections, one collection's records.
        'for (owner, prefix, collections) in [(model, " ", &RECORD_COLLECTIONS[..]), (request, " ", request_collections)]':
            ("2", ["model_fields.rs:non_object_records"]),
        "for collection in collections": ("8", ["model_fields.rs:non_object_records"]),
        "for (i, record) in owner.get(*collection).and_then(Value::as_array).into_iter().flatten().enumerate()":
            ("n", ["model_fields.rs:non_object_records"]),
    }
    edges = sorted((str(a).split(":")[-1], str(b).split(":")[-1]) for a, b, why in d["O10_EDGE_ZERO"] if "SD-NR" in why)
    assert edges == [("check_nested", "finding_name"), ("check_nested", "finding_path"), ("unknown_field_refusal", "listed_refusal")]
    assert sorted(str(k).split(":")[-1] for k, why in d["O10_RECURSION_REVIEWED"] if "SD-NR" in why) == ["finding_name", "finding_path"]
    audit = {}
    for k, v in d["O10_ID_AUDIT"]:
        audit.setdefault(str(k), {}).update(v)
    member = audit[f"{FIELDS}:{line_of(FIELDS, 'format!("the {key} of {}", parent.finding_name())')}"]
    assert member["key"]["bytes"] == 128 and member["parent.finding_name()"]["bytes"] == 3 * (8 + 128) + 17 + 3 + 128
    entry = audit[f"{FIELDS}:{line_of(FIELDS, 'format!("{}.{key}[{index}]", parent.finding_path())')}"]
    assert entry["key"]["bytes"] == len("temperature_points") and entry["parent.finding_path()"]["bytes"] == 19 + 2 + 20 + 3 * 129
    record = audit[f"{FIELDS}:{line_of(FIELDS, 'At::Record { kind, id: Some(id), .. } => format!(')}"]
    assert record["kind"]["bytes"] == len("temperature point") and record["id"]["bytes"] == 128
    # The bounded refusals: a non-object finding is a static collection path and a static kind; the list is bounded by
    # eight of the longer finding, and the code and remedy are static.
    non_object = audit[f"{FIELDS}:{line_of(FIELDS, 'findings.keep_finding(|| NonObject {')}"]
    assert non_object["prefix"]["bytes"] == len("model.") and non_object["collection"]["bytes"] == len("pipe_segments")
    assert non_object["i"]["bytes"] == 20
    refused = audit[f"{FIELDS}:{line_of(FIELDS, 'listed_refusal(MODEL_RECORD_NOT_OBJECT')}"]
    assert refused["f.path"]["bytes"] == len("model.pipe_segments[]") + 20 and refused["f.is"]["bytes"] == len("an array (a positional record)")
    listed = audit[f"{FIELDS}:{line_of(FIELDS, 'request: {listed}. {remedy}')}"]
    assert listed["code"]["bytes"] == len("MODEL_RECORD_NOT_OBJECT") and listed["remedy"]["bytes"] == 154
    assert audit[f"{FIELDS}:{line_of(FIELDS, 'more", findings.more));')}"]["findings.more"]["bytes"] == 20
    # `named` keeps the root records' kind class (the frame names records at its own sites).
    assert audit[f"{FIELDS}:{line_of(FIELDS, 'Some(id) => format!("{kind} `{id}`"),')}"]["kind"]["bytes"] == len("primitive load")


def test_the_names_paths_and_withheld_texts_are_audited(monkeypatch):
    d = decisions(monkeypatch, route="L")
    audit = {}
    for k, v in list(d["ID_AUDIT"]) + list(d["O10_ID_AUDIT"]):
        audit.setdefault(str(k), {}).update(v)
    load_name = audit[f"{FIELDS}:{line_of(FIELDS, 'let load_name = format!(')}"]
    assert load_name["case_name"]["bytes"] == 140 and load_name['named("primitiveload",id_of(load),j)']["bytes"] == 145
    assert audit[f"{FIELDS}:{line_of(FIELDS, '{load_path}.magnitude')}"]["load_name"]["bytes"] == 289
    support = audit[f"{PREVIEW}:{line_of(PREVIEW, 'Err(reason) => withheld.push(format!(')}"]
    assert support["reason"]["bytes"] == len("source support row missing in an operand case")
    named = audit[f"{PREVIEW}:{line_of(PREVIEW, 'withholds {} combined row(s) that could not be formed')}"]["named"]["bytes"]
    assert named == 8 * 1052 + 7 * 2 + len(", and  more") + 20
    # SD-RK: the two replaced basis keys resolve by name to the same sites.
    assert f"{PREVIEW}:{line_of(PREVIEW, 'rows.push(combination_row(combination, &expression, sources[0], qualified_combination_result_id(&combination.id, &base(component)), value, sources.iter()')}" in audit
    # No basis key of the two sites survives un-re-keyed. The test reads the raw basis keys (plain
    # strings, not `Resolved`), since a by-name key can land on a line that spells a basis key: at
    # the unit factor-sum gate (§8.24) the support row push resolves to :1022 itself.
    stale = {"core/product_physics/src/preview_physics.rs:990", "core/product_physics/src/preview_physics.rs:1022"}
    raw = [str(k) for k, _ in list(d["ID_AUDIT"]) + list(d["O10_ID_AUDIT"]) if type(k).__name__ != "Resolved"]
    assert not stale & set(raw)


def test_the_owners_are_present_and_positive():
    assert sd_owners_model.inspect(str(SOURCE))
    assert sd_owners_model.withheld_names(7785, 32) and sd_owners_model.seen_terms() and sd_owners_model.probe_error()


# lib.rs and preview_physics.rs are also under full-file pins, which stop first.
@pytest.mark.parametrize("path, old, new, stop", [
    (LIB, "    if let Some(refusal) = model_fields::unknown_field_refusal(&actual_request) {\n        return Err(refusal);\n    }\n", "", "SD-NF|KD5C4|pin changed"),
    (FIELDS, "    if findings.kept.is_empty() {\n        return None;\n    }\n", "", "SD-NF"),
    (FIELDS, "            self.kept.push(finding());", "            self.kept.push(finding());\n            self.kept.push(finding());", "SD-NF"),
    (FIELDS, "    if !non_objects.kept.is_empty() {", "    if non_objects.kept.len() > 1 {", "SD-NF"),
    (FIELDS, 'format!("{prefix}{collection}[{i}]")', 'format!("{prefix}{collection}.{i}")', "SD-NF"),
    (FIELDS, '["load_cases", "combinations", "nodes",', '["load_cases", "combinations", "nodes", "groups",', "SD-NR"),
    (FIELDS, 'let case_path = format!("model.load_cases[{i}]");', 'let case_path = format!("model.load_cases.{i}");', "SD-ID"),
    (FIELDS, "findings.keep_finding(|| Finding { field: key.clone(), record: at.finding_name(), path: at.finding_path() });",
     "findings.keep_finding(|| Finding { field: key.clone(), record: at.finding_name(), path: at.finding_path() }); at.finding_name();", "SD-NF"),
    (FIELDS, '(request, "materials", "request material")', '(request, "materials", "request-level material")', "SD-NR"),
    (FIELDS, 'let at = At::Entry { parent: &at, key: "temperature_points",', 'let at = At::Entry { parent: &at, key: "points",', "SD-NR"),
    (FIELDS, "const NONLINEAR_QUANTITIES: [&str; 3] =", "const NONLINEAR_QUANTITY: [&str; 3] =", "SD-NR"),
    (PREVIEW, "const NAMED: usize = 8;", "const NAMED: usize = 16;", "SD-WT|KD5C4|pin changed"),
])
def test_a_changed_guard_source_stops(monkeypatch, tmp_path, path, old, new, stop):
    with pytest.raises((SystemExit, ValueError), match=stop):
        decisions(monkeypatch, snapshot_with(tmp_path, path, old, new))


def test_a_changed_owner_stops(tmp_path):
    snap = snapshot_with(tmp_path, VALIDATION, "if !seen.insert(term.load_case.as_str()) {", "if !seen.insert(term.load_case.as_str()) && true {")
    with pytest.raises(SystemExit, match="SD owners"):
        sd_owners_model.inspect(str(snap))


def test_the_duplicate_loops_are_bound_by_their_own_functions(monkeypatch, tmp_path):
    """SD-DL: the enumerating duplicate loops are bound at 3, each only in its function; a changed count stops."""
    d = decisions(monkeypatch, route="L")
    loops = {h: (b, fns) for h, b, why, fns in d["O10_LOOPS_LAST"] if "SD-DL" in why}
    assert loops == {
        "for (index, term) in combination.terms.iter().enumerate()": ("3", ["validation.rs:validate_mechanics_combination"]),
        "for (index, operand_id) in operand_ids.iter().enumerate()": ("3", ["validation.rs:validate_range_envelope_combination"]),
    }
    snap = snapshot_with(tmp_path, VALIDATION, "operand_ids[..index].iter().filter(|other| *other == operand_id).count() != 1",
                         "operand_ids.iter().filter(|other| *other == operand_id).count() > 1")
    with pytest.raises(SystemExit, match="SD-DL"):
        decisions(monkeypatch, snap)


MECHANICS_COUNT = "combination.terms[..index]\n                .iter()\n                .filter(|other| other.load_case == term.load_case)\n                .count();"
RANGE_COUNT = "operand_ids[..index].iter().filter(|other| *other == operand_id).count() != 1"


@pytest.mark.parametrize("old, new", [
    (MECHANICS_COUNT, MECHANICS_COUNT.replace("[..index]", "[..]")),
    (MECHANICS_COUNT, MECHANICS_COUNT.replace("[..index]", "[..index].to_vec()")),
    (MECHANICS_COUNT, MECHANICS_COUNT.replace(".count();", ".cloned().collect::<Vec<_>>().len();")),
    (MECHANICS_COUNT, MECHANICS_COUNT.replace("other.load_case == term.load_case", "other.load_case != term.load_case")),
    (RANGE_COUNT, RANGE_COUNT.replace("[..index]", "[..]")),
    (RANGE_COUNT, RANGE_COUNT.replace("[..index]", "[..index].to_vec()")),
    (RANGE_COUNT, RANGE_COUNT.replace(".count() != 1", ".cloned().collect::<Vec<_>>().len() != 1")),
    (RANGE_COUNT, RANGE_COUNT.replace("*other == operand_id", "*other != operand_id")),
], ids=["mechanics-full-slice", "mechanics-to_vec", "mechanics-cloned-collect", "mechanics-predicate",
        "range-full-slice", "range-to_vec", "range-cloned-collect", "range-predicate"])
def test_a_changed_earlier_occurrence_count_stops(monkeypatch, tmp_path, old, new):
    """SD-DL: the complete count statement is pinned in both the decision guard and the owner guard."""
    snap = snapshot_with(tmp_path, VALIDATION, old, new)
    with pytest.raises(SystemExit, match="SD-DL"):
        decisions(monkeypatch, snap)
    with pytest.raises(SystemExit, match="SD owners"):
        sd_owners_model.inspect(str(snap))
