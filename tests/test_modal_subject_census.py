"""DEL-04-09 §5.3.2, the subject census (`binding.subject_census`), in the Python modal
reader: the record's shape (closure), its counts against the tables on every solved
document (evidence), and, with the bound request, the subject rule's own list
(`modal_model_mass.census`) against the counts, the digest and each table's subject set.

Forgeries change the tables, reseal them (row checksums and table digests) and leave the
census record valid-looking: unchanged, or adjusted so its counts match the forged
tables. Each then fails on coverage, never on a stale hash. All models are invented.
"""
from __future__ import annotations

import copy
import json

import pytest

from tests.test_modal_model_mass import RESULTS, SOLVED, _component_edit, _mutate, _reseal, coherent, fixture, formed, rebind
from tests.verifiers import canonical_json
from tests.verifiers import modal_evidence as me
from tests.verifiers import modal_model_mass as mm


def evidence(doc, request=None):
    verdict = me.read(doc, request)
    assert verdict["closure"]["status"] == "passed", verdict["closure"]
    return verdict["relations"]["evidence"], verdict


def census_items(standing):
    return [text for text in standing["scope"] if text.startswith("subject census")]


def assert_fails(doc, request, prefix):
    standing, verdict = evidence(doc, request)
    assert standing["status"] == "failed", standing
    assert standing["difference"].startswith(prefix), standing["difference"]
    assert verdict["decision"] == "rejected"
    return standing


def assert_scoped_by_census(doc):
    """Without the request the identities are not re-derived: never a pass."""
    standing, verdict = evidence(doc)
    assert standing["status"] == "scoped_unsupported", standing
    assert any("no request is supplied" in text for text in census_items(standing)), standing
    assert "evidence" in verdict["validation"]["unmet"]


# --- intact controls ------------------------------------------------------------------------

@pytest.mark.parametrize("name", SOLVED)
def test_every_solved_fixture_states_its_models_census(name):
    doc, request = fixture(name)
    stated = doc["binding"]["subject_census"]
    derived = mm.census(request)
    assert canonical_json.canonical_sha256(derived.subjects()) == stated["subjects_sha256"]
    assert (len(derived.straights), len(derived.arcs), len(derived.bodies), len(derived.supports), len(derived.nonlinear)) == (
        stated["straight_members"], stated["arcs"], stated["component_bodies"], stated["supports"], stated["nonlinear_supports"])
    # The mass pass forms exactly the census's members and bodies.
    mm.census_self_check(mm.derive(request), derived)
    standing, _ = evidence(doc, request)
    if name.startswith("state_"):
        # The census closes; only the reported support states' qualification stays scoped.
        assert standing["status"] == "scoped_unsupported", standing
        assert not any("census" in item for item in standing["scope"]), standing
        assert [item.split(":")[0] for item in standing["scope"]] == ["supports"], standing
    else:
        assert standing["status"] == "passed", standing
    assert_scoped_by_census(doc)


def test_the_census_list_is_sorted_by_kind_then_id_and_keeps_kinds_apart():
    _, request = fixture("component_body")
    renamed = _mutate(request, _component_edit(["id"], "pipe:p1"))
    assert mm.census(renamed).subjects() == [
        {"kind": "component_body", "id": "pipe:p1"}, {"kind": "straight_member", "id": "pipe:p1"},
        {"kind": "support", "id": "s"}]


def test_a_body_spelled_like_a_member_is_a_different_subject():
    doc, request = fixture("component_body")
    renamed = _mutate(request, _component_edit(["id"], "pipe:p1"))
    bound = rebind(doc, renamed)
    for row in bound["tables"]["identity"]["rows"]:
        if row["subject_kind"] == "component_body":
            row["subject"] = "pipe:p1"
    _reseal(bound)
    bound["binding"]["subject_census"]["subjects_sha256"] = canonical_json.canonical_sha256(mm.census(renamed).subjects())
    assert evidence(bound, renamed)[0]["status"] == "passed"
    # The member of the same spelling does not stand for the body: without its own row, it fails.
    _forge_identity(bound, "component_body", "pipe:p1", "missing", None)
    _reseal(bound)
    bound["binding"]["subject_census"]["component_bodies"] = 0
    standing = assert_fails(bound, renamed, "model-derived")
    assert "component_body subjects []" in standing["difference"], standing["difference"]


# --- shape (closure) ------------------------------------------------------------------------

def _closure_difference(doc):
    verdict = me.read(doc)
    return verdict["closure"]["status"], verdict["closure"]["difference"]


def _null(doc):
    doc["binding"]["subject_census"] = None


@pytest.mark.parametrize("edit, phrase", [
    (lambda c: c.pop("arcs"), "missing `arcs`"),
    (lambda c: c.update(extra=0), "undeclared `extra`"),
    (lambda c: c.update(supports=-1), "not a non-negative integer"),
    (lambda c: c.update(arcs=True), "not a non-negative integer"),
    (lambda c: c.update(straight_members=2.0), "not a non-negative integer"),
    (lambda c: c.update(subjects_sha256=c["subjects_sha256"].upper()), "not a lowercase SHA-256"),
])
def test_a_solved_documents_census_has_its_shape(edit, phrase):
    doc, _ = fixture("arc")
    edit(doc["binding"]["subject_census"])
    status, difference = _closure_difference(doc)
    assert status == "failed" and "/binding/subject_census" in difference and phrase in difference, difference


def test_a_solved_document_states_its_census():
    doc, _ = fixture("arc")
    _null(doc)
    status, difference = _closure_difference(doc)
    assert status == "failed" and difference == "/binding/subject_census: not an object", difference


def test_a_refused_document_states_no_census():
    doc = json.loads((RESULTS / "modal_refused_no_bound.json").read_text(encoding="utf-8"))
    assert doc["binding"]["subject_census"] is None
    assert me.read(doc)["closure"]["status"] == "passed"
    stated = copy.deepcopy(doc)
    stated["binding"]["subject_census"] = fixture("arc")[0]["binding"]["subject_census"]
    status, difference = _closure_difference(stated)
    assert status == "failed" and "a refused document states none" in difference
    absent = copy.deepcopy(doc)
    del absent["binding"]["subject_census"]
    assert _closure_difference(absent) == ("failed", "/binding: missing `subject_census`")


# --- forged tables against the census and the model ------------------------------------------

def _renumber(table, prefix):
    for i, row in enumerate(table["rows"]):
        row["ordinal"] = i
        if prefix is not None:
            row["row_id"] = f"{prefix}{i}"


def _forge_rows(table, hit, how, rename, prefix):
    out = []
    for row in table["rows"]:
        if not hit(row):
            out.append(row)
            continue
        if how == "missing":
            continue
        out.append(rename(copy.deepcopy(row)) if how == "substituted" else row)
        if how == "extra":
            out.append(rename(copy.deepcopy(row)))
        elif how == "duplicate":
            out.append(copy.deepcopy(row))
    table["rows"] = out
    _renumber(table, prefix)


def _forge_identity(doc, kind, subject, how, new):
    def rename(row):
        row["subject"] = new
        return row
    _forge_rows(doc["tables"]["identity"], lambda r: r["subject_kind"] == kind and r["subject"] == subject, how, rename,
                "identity:")


def _forge_discretization(doc, member, how, new):
    if how == "duplicate":
        return  # a duplicated row id is a closure failure; the identity's repeat is the coverage case
    def rename(row):
        row["member_ref"], row["row_id"] = new, f"member:{new}"
        return row
    _forge_rows(doc["tables"]["discretization"], lambda r: r["member_ref"] == member, how, rename, None)


def _forge_supports(doc, support, how, new):
    def rename(row):
        row["support_id"], row["row_id"] = new, f"support:{new}"
        return row
    _forge_rows(doc["tables"]["supports"], lambda r: r["support_id"] == support, how, rename, None)


def _adjust_census(doc):
    """The census counts set to the forged tables' (its digest unchanged)."""
    rows = [r for r in doc["tables"]["identity"]["rows"] if r["axis"] == "x"]
    c = doc["binding"]["subject_census"]
    c["straight_members"] = sum(r["subject_kind"] == "member" and r["comparison"] == "nodal" for r in rows)
    c["arcs"] = sum(r["subject_kind"] == "member" and r["comparison"] == "resultant" for r in rows)
    c["component_bodies"] = sum(r["subject_kind"] == "component_body" for r in rows)
    c["nonlinear_supports"] = len(doc["tables"]["supports"]["rows"])
    c["supports"] = max(c["supports"], c["nonlinear_supports"])


SUBJECTS = {
    "straight_member": ("arc", "pipe:a-b"),
    "arc": ("arc", "pipe:b-c"),
    "component_body": ("component_body", "component:valve"),
    "support": ("state_one_way_pressed", "support:NL-U"),
}


def forged(kind, how, adjusted):
    name, subject = SUBJECTS[kind]
    doc, request = fixture(name)
    doc = copy.deepcopy(doc)
    new = subject + ":forged"
    if kind == "support":
        _forge_supports(doc, subject, how, new)
    else:
        _forge_identity(doc, "component_body" if kind == "component_body" else "member", subject, how, new)
        if kind == "straight_member":
            _forge_discretization(doc, subject, how, new)
    _reseal(doc)
    if adjusted:
        _adjust_census(doc)
    return doc, request


@pytest.mark.parametrize("adjusted", [False, True], ids=["census_unchanged", "census_adjusted"])
@pytest.mark.parametrize("how", ["missing", "extra", "duplicate", "substituted"])
@pytest.mark.parametrize("kind", list(SUBJECTS))
def test_a_forged_subject_fails_on_coverage(kind, how, adjusted):
    doc, request = forged(kind, how, adjusted)
    if kind == "support" and how == "duplicate":
        # A support row's id is `support:<id>`, unique in the document.
        status, difference = _closure_difference(doc)
        assert status == "failed" and "is not unique" in difference, difference
        return
    if how == "duplicate":
        for supplied in (None, request):
            assert_fails(doc, supplied, "identity: a subject repeats")
        return
    counts_disagree = how in ("missing", "extra") and not adjusted
    if counts_disagree:
        # The payload check: the tables against the record's own counts, with or without the request.
        for supplied in (None, request):
            assert_fails(doc, supplied, "subject census:")
    else:
        # The counts agree, so the payload cannot see it: scoped, never passed ...
        if kind == "support" and how == "missing":
            assert_fails(doc, None, "supports: a linearized state case lists no support")
        else:
            assert_scoped_by_census(doc)
        # ... and the model's own subject list rejects it.
        assert_fails(doc, request, "model-derived")


# --- the bound request against the record ----------------------------------------------------

def test_a_substituted_body_id_fails_on_the_digest_alone():
    """The tables agree with the model, and every count agrees: only the digest of the
    run's subject list (formed from the original id) differs."""
    doc, request = fixture("component_body")
    renamed = _mutate(request, _component_edit(["id"], "component:other"))
    bound = rebind(doc, renamed)
    for row in bound["tables"]["identity"]["rows"]:
        if row["subject"] == "component:valve":
            row["subject"] = "component:other"
    _reseal(bound)
    standing = assert_fails(bound, renamed, "model-derived")
    assert "subjects_sha256" in standing["difference"]
    assert evidence(bound)[0]["status"] == "scoped_unsupported"


def test_a_substituted_support_id_fails_on_the_digest():
    doc, request = fixture("arc")
    changed = _mutate(request, lambda m: m["supports"][1].update(id="support:other"))
    standing = assert_fails(rebind(doc, changed), changed, "model-derived")
    assert "subjects_sha256" in standing["difference"]


@pytest.mark.parametrize("label, name, edit, phrase", [
    ("duplicate support", "arc", lambda m: m["supports"].append(copy.deepcopy(m["supports"][1])), "supports, the model has 3"),
    ("extra support", "arc", lambda m: m["supports"].append({"id": "support:x", "node": "d", "family": "anchor",
                                                             "restraints": ["UX"]}), "supports, the model has 3"),
    ("missing support", "arc", lambda m: m["supports"].pop(1), "supports, the model has 1"),
    ("missing nonlinear support", "state_one_way_pressed", lambda m: m["supports"][1].pop("nonlinear"),
     "supports table's supports"),
    ("extra straight member", "arc", lambda m: m["pipe_segments"].append(dict(copy.deepcopy(m["pipe_segments"][0]), id="pipe:x")),
     "straight_member subjects"),
    ("missing straight member", "arc", lambda m: m["pipe_segments"].pop(0), "straight_member subjects"),
    ("missing arc", "arc", lambda m: m.__setitem__("components", []), "straight_member subjects"),
    ("ineligible body", "component_body", lambda m: m["components"][0].pop("mechanics_interface"), "straight_member subjects"),
    ("missing body", "component_body", lambda m: m.__setitem__("components", []), "straight_member subjects"),
])
def test_the_bound_models_subjects_hold_the_tables_and_the_record(label, name, edit, phrase):
    doc, request = fixture(name)
    changed = _mutate(request, edit)
    standing = assert_fails(rebind(doc, changed), changed, "model-derived")
    assert phrase in standing["difference"], (label, standing["difference"])


def test_a_census_the_rule_does_not_cover_stays_scoped_with_its_reason():
    doc, request = fixture("arc")

    def twice(model):
        model["components"].append(dict(copy.deepcopy(model["components"][0]), id="component:bend2"))

    changed = _mutate(request, twice)
    standing, _ = evidence(rebind(doc, changed), changed)
    assert standing["status"] == "scoped_unsupported", standing
    assert any("does not cover" in text and "two curved-bend" in text for text in census_items(standing)), standing


# --- the pass is structural ------------------------------------------------------------------

def _tiny_body(doc, request):
    changed = _mutate(request, _component_edit(["geometry", "weight"], {"unit": "N", "value": 2.3195656276910255e-308}))
    changed = _mutate(changed, _component_edit(["geometry", "center_of_gravity"],
                                               {"unit": "m", "x": 250575.49268495973, "y": 2.116592064317826, "z": 0}))
    changed["weight_to_mass_acceleration"] = {"unit": "m/s^2", "value": 1.0}
    bound = rebind(doc, changed)
    bound["basis"]["weight_to_mass_acceleration"]["value"] = 1.0
    bound["binding"]["options"]["weight_to_mass_acceleration"]["value"] = 1.0
    produced = formed(changed)
    produced[(15, 16)] = -1.2302189740973076e-302  # what the producer publishes there
    return coherent(bound, produced), changed


def test_an_unresolved_mass_entry_drops_no_subject():
    doc, request = fixture("component_body")
    published, changed = _tiny_body(doc, request)
    assert mm.census(changed).bodies == ["component:valve"]
    verdict = me.read(published, changed)
    assert verdict["relations"]["mass"]["status"] == "unresolved"
    assert verdict["relations"]["evidence"]["status"] == "passed", verdict["relations"]["evidence"]
    # The body is still a subject: dropped from the tables, it fails.
    dropped = copy.deepcopy(published)
    _forge_identity(dropped, "component_body", "component:valve", "missing", None)
    _reseal(dropped)
    _adjust_census(dropped)
    assert_fails(dropped, changed, "model-derived")


def test_a_mass_outside_the_rounding_model_drops_no_subject():
    doc, request = fixture("cantilever_whole_spectrum")

    def tilted(model):
        for pipe in model["pipe_segments"]:
            pipe["y_reference"] = {"x": 9e15, "y": 1e-7, "z": 0}

    changed = _mutate(request, tilted)
    verdict = me.read(coherent(rebind(doc, changed), formed(changed)), changed)
    assert verdict["relations"]["mass"]["status"] == "unresolved"
    assert sorted(mm.census(changed).straights) == ["p0", "p1"]
    assert verdict["relations"]["evidence"]["status"] == "passed", verdict["relations"]["evidence"]


def test_a_body_the_producer_refuses_stays_a_subject():
    """A body outside the range premise is the mass relation's failure; it never changes
    the subject list (the census still compares, and agrees)."""
    doc, request = fixture("component_body")
    changed = _mutate(request, _component_edit(["geometry", "center_of_gravity"], {"unit": "m", "x": 1e-200, "y": 0, "z": 0}))
    assert mm.census(changed).bodies == ["component:valve"]
    verdict = me.read(rebind(doc, changed), changed)
    assert verdict["relations"]["mass"]["status"] == "failed"
    assert verdict["relations"]["evidence"]["status"] == "passed", verdict["relations"]["evidence"]


def test_a_mass_pass_that_disagrees_with_the_census_is_a_reader_defect(monkeypatch):
    doc, request = fixture("arc")
    real = mm.derive

    def disagreeing(req):
        model = real(req)
        model.straights.append("pipe:phantom")
        return model

    monkeypatch.setattr(mm, "derive", disagreeing)
    standing = me.read(doc, request)["relations"]["evidence"]
    assert standing["status"] == "unresolved", standing
    assert any("cross-check disagrees" in text for text in standing["unresolved"]), standing
