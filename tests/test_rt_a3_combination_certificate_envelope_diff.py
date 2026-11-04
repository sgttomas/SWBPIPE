"""RT-A3 §4 regenerates current producer fixtures, never numerical references.

The preserved endpoint is RT-A3 P's final candidate 78580da7e (over main 252bef969);
test_rt_a3_nonlinear_text_only_envelope_diff keeps increment 4's proof frozen at
its merge 252bef969. This stage (DEL-05-02 Work `combination-certificate.md`)
publishes `contract_evidence.combination_certificate` on a document with a
combination, with one info diagnostic per assessed or not-assessed combination
(`diagnostic:combination-certificate:<combination>`), and restates the pressure-2
table's combinations limitation and combination policy (so its digest moves, and
physics-retained-1's, which inherits it). The EX derived-rows notice, the
message of an EXACT_COMBINATION_DERIVED_FROM_CASE_RESULTS diagnostic
(`exact_combinations.rs` DERIVED_TEXT), is restated to name the certificate.

Across this stage only these may change:

- a published document's `contract_evidence` may gain `combination_certificate`
  (absent at the endpoint), and that document is then read by the certificate's
  own Python check (tests/verifiers/combination_certificate.py), which re-derives
  every record, bound and verdict from the document's unchanged rows;
- its `diagnostics` may gain the certificate's entries and nothing else; every
  other diagnostic keeps its position relative to the others, token for token,
  except that a derived-rows notice states `NEW_DERIVED` where it stated
  `OLD_DERIVED`;
- a report statement that names the pressure-2 table by digest names the new one
  (`REPORT_PATHS`), and nothing else in it moves;
- a digest only at a named output-binding field (the constant-effort stage's
  `binding_kind`), and only to the digest recomputed for the subject its endpoint
  value identifies: a table digest to the restated table's; any other to that of a
  carrier's bytes, an embedded JSON document, a published document or a subject
  that binds its own digest (`ce.derived_pairs`), recomputed from this scope.

Exact numeric tokens, member and array order, row sets, selection and every
input-scoped hash remain invariant. The fixture generation records
(`GENERATION`) bind source files by digest; only their source-file, inventory and
output digests move, each to its subject's bytes.

Pending (not regenerated here, no Node dependencies installed in this lane):
the carriers written by desktop vitest writers (`PENDING_DESKTOP_WRITERS`). They
are compared under the same rule; the W-11 requests' AnalysisRun-record digests
(the increment-4 `UNMATERIALIZED` class) have not moved yet. When their writers
run, each moved occurrence must be named here with its reviewed pair, as
increment 4 named them.
"""
from functools import cache
from pathlib import Path
import hashlib
import json
import subprocess

import pytest

from tests.test_o4_text_only_envelope_diff import HASH, Obj, load, _embedded
from tests.test_rt_a3_reducer_text_only_envelope_diff import INPUT_BINDINGS
from tests.test_rt_a3_reducer_2b_text_only_envelope_diff import _matches
from tests import test_rt_a3_constant_effort_text_only_envelope_diff as ce

ROOT = Path(__file__).resolve().parents[1]
BASE = "78580da7e0334c35fd1fc7906057844de07e2e69"

CERTIFICATE = "combination_certificate"
DIAGNOSTIC = "diagnostic:combination-certificate:"
TABLES = {
    "fixtures/results/semantic_contract_v0_3_pressure_2.json": (
        "2c20583d3c06e226a13bb7f9649b04aeea0f0f6948b5fa1cbd944afd07413645",
        "21b7426ec1c50ba5f74539984261554d06b9b6a26ec26f7d5765b5faa7d6bb90"),
    "fixtures/results/semantic_contract_v0_3_physics_retained_1.json": (
        "85c2678a78e794b44761c11efb73c215645c7e2560d802ac8ad6075e7804c691",
        "320dc3f1f75944af8b66da1287e6f8ee5e1fcd5fe005752f2c90bf8c89a9c894"),
}
TABLE_SHA256 = TABLES["fixtures/results/semantic_contract_v0_3_pressure_2.json"]
TABLE_PAIRS = frozenset(TABLES.values())
# The restated table texts (DEL-05-02 Work `combination-certificate.md` §1; the table's limitation 6 tail and
# combination_policy.standing).
OLD_LIMITATION_TAIL = ("Combination rows carry no numerical check of their own and never count towards numerical "
                       "eligibility, and a document with a combination attempts no retained-source recovery.")
NEW_LIMITATION_TAIL = (
    "Combination rows never count towards numerical eligibility; a subtraction's or mechanics combination's derived "
    "row groups over load cases with certified rows carry the stated bound and verdict of combination_certificate "
    "(established within the stated bound, not established, or not assessed), and every other combination row is an "
    "inspection result; a document with a combination attempts no retained-source recovery.")
OLD_STANDING = ("derived from case results: no numerical check of its own; each operand's standing and numerical "
                "findings are stated in exact_combinations; combination rows never count towards numerical "
                "eligibility, headlines or maxima")
NEW_STANDING = (
    "derived from case results; each operand's standing and numerical findings are stated in exact_combinations; a "
    "subtraction's or mechanics combination's derived row groups over load cases with certified rows carry a stated "
    "bound and verdict in combination_certificate (established within the stated bound against the operand cases' "
    "intended systems, not established, or not assessed below the floor), and every other combination row is an "
    "inspection result; combination rows never count towards numerical eligibility, headlines or maxima")

# The EX derived-rows notice (exact_combinations.rs DERIVED_TEXT), before and after.
OLD_DERIVED = "they carry no numerical check of their own and never count towards numerical eligibility; each operand's standing"
NEW_DERIVED = ("they never count towards numerical eligibility, and only the derived row groups that "
               "contract_evidence.combination_certificate establishes carry a stated bound; each operand's standing")
DERIVED_CODE = "EXACT_COMBINATION_DERIVED_FROM_CASE_RESULTS"

# Published documents that gain the certificate (each holds one or more documents with a combination).
DOCUMENTS = (
    "fixtures/product_preview/invented_demo_result_preview_physics_1_dense.json",
    "fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json",
    "fixtures/results/preview_physics_invented_dense.json",
    "fixtures/results/preview_physics_invented_sparse.json",
    "fixtures/results/preview_physics_range_envelope_sparse.json",
    "fixtures/results/preview_physics_unicode_ids_sparse.json",
    "fixtures/results/preview_physics_nonlinear_operands_corpus.json",
    "fixtures/results/pressure_v3_combination_reader_corpus.json",
    "fixtures/results/rt_a3_ce2_withheld_sparse.json",
)
# Carriers that bind the restated table (and what binds it) by digest only.
BINDINGS = (
    *(f"fixtures/results/pressure_{kind}_{mode}.{member}.json"
      for kind in ("load_state_connected", "load_state_pressure", "plain_connected")
      for mode in ("dense", "sparse") for member in ("analysis_run", "stress_neutral")),
    "fixtures/results/base_reader_outcomes.json",
)
_W11 = ce._W11
REPORTS = (
    *(_W11 + f"pressure_2_{name}.json" for name in ("connector", "load_reference_states", "realized_bend", "straight")),
    "fixtures/reports/invented/exact_report_sections_parity.json",
    "fixtures/reports/invented/user_release_exact/pressure_2_straight.json",
)
# The report fixtures state the table's digest in a statement, only at these paths (an int matches "#").
REPORT_PATHS = {
    **{name: (("request", "report", "report_sections", "limitations", "#", "statement"),) for name in REPORTS},
    "fixtures/reports/invented/exact_report_sections_parity.json": (
        ("vectors", "#", "expected", "limitations", "#", "statement"),),
}
GENERATION = (
    "fixtures/product_preview/demo_fixture_generation.json",
    "fixtures/product_preview/preview_physics_fixture_generation.json",
)
SCOPE = (*DOCUMENTS, *BINDINGS, *REPORTS)
# JSON files that name the endpoint's table digest but are not carriers: the inheriting table and the schemas that pin
# the table digest as a constant (schema_manifest.json binds them; their tests are the schemas' own).
NOT_CARRIERS = {
    "fixtures/results/semantic_contract_v0_3_physics_retained_1.json": "inherits pressure-2 by digest (TABLES)",
    "schemas/analysis_run.v0.3.schema.json": "pins the pressure-2 table digest",
    "schemas/stress_neutral_export.v0.3.schema.json": "pins the pressure-2 table digest",
}
# Written by desktop vitest writers, which need the desktop's Node dependencies (not installed in this lane).
PENDING_DESKTOP_WRITERS = {
    **{_W11 + f"pressure_2_{name}.json": "exactReportBelowGate.test.tsx (OPENPIPESTRESS_WRITE_EXACT_PACKAGE_REQUESTS)"
       for name in ("connector", "load_reference_states", "realized_bend", "straight")},
    "fixtures/reports/invented/user_release_exact/pressure_2_straight.json":
        "userReleaseDerivation.parity.test.ts (OPENPIPESTRESS_WRITE_USER_RELEASE_VECTORS)",
    **{f"apps/desktop/src/test-support/desktop-shaped/desktop_shaped_pressure_plain_connected_{mode}.{member}.json":
       "exactOutputTwins.test.tsx (DESKTOP_STRESS_NEUTRAL_WRITE_FIXTURES)"
       for mode in ("dense", "sparse") for member in ("analysis_run", "stress_neutral")},
}
# Outside this comparison: frozen references, design records, recorded runs and the historical table variants they
# bind (kept verbatim), and the certificate's own new corpus, forgeries and references.
OUTSIDE = (":!validation/", ":!execution/", ":!fixtures/persistence/recorded_*",
           ":!fixtures/results/recorded_pre_rt_a4/", ":!fixtures/results/combination_certificate_*")


@cache
def _at(rev, name):
    return subprocess.check_output(["git", "show", f"{rev}:{name}"], cwd=ROOT, text=True)


def current(name):
    return (ROOT / name).read_text(encoding="utf-8")


def binding_kind(path):
    """The constant-effort stage's output-binding fields, plus the base reader outcomes' dispatch table binding."""
    if len(path) >= 3 and path[-3:] == ("dispatch", "binding", "sha256"):
        return "table"
    return ce.binding_kind(path)


def _sha(text):
    return hashlib.sha256(text.encode()).hexdigest()


# ---------------------------------------------------------------- comparison

def compare(before, after, path=(), *, name=None, moves=None, added=None, derived_notice=None):
    """Return the number of restated statements; refuse every other move.

    A certificate gained by a published document is appended to `added` as the document's path (with its count of
    gained diagnostics); a moved digest is appended to `moves` as (path, before, after), and without `moves` a moved
    digest is refused."""
    assert type(before) is type(after), path
    if isinstance(before, Obj):
        b, a = dict(before), dict(after)
        keys_b, keys_a = [k for k, _ in before], [k for k, _ in after]
        if path and path[-1] == "contract_evidence" and CERTIFICATE in a and CERTIFICATE not in b:
            assert added is not None, path
            keys_a = [k for k in keys_a if k != CERTIFICATE]
            added.append(path[:-1])
        assert keys_b == keys_a, path
        if b.get("payload_scope") in {"model", "request", "input_manifest"}:
            assert before == after, path
        notice = (lambda at: True) if b.get("code") == DERIVED_CODE and a.get("code") == DERIVED_CODE else None
        return sum(compare(b[k], a[k], path + (k,), name=name, moves=moves, added=added,
                           derived_notice=notice if k == "message" else None) for k in keys_b)
    if isinstance(before, list):
        if path and path[-1] == "diagnostics" and len(after) > len(before):
            gained = [d for d in after if isinstance(d, Obj) and str(dict(d).get("id", "")).startswith(DIAGNOSTIC)]
            assert not any(isinstance(d, Obj) and str(dict(d).get("id", "")).startswith(DIAGNOSTIC) for d in before), path
            after = [d for d in after if not any(d is g for g in gained)]
            assert added is not None and gained, path
            added.append(("diagnostics", path[:-1], len(gained)))
        assert len(before) == len(after), path
        return sum(compare(x, y, path + (i,), name=name, moves=moves, added=added)
                   for i, (x, y) in enumerate(zip(before, after)))
    if before == after:
        return 0
    if (type(before) is str and len(path) >= 3 and path[-3] == "diagnostics" and path[-1] == "message"
            and derived_notice is not None and derived_notice(path[:-1])):
        assert before.count(OLD_DERIVED) == 1 and after == before.replace(OLD_DERIVED, NEW_DERIVED), path
        return 1
    if type(before) is str and name in REPORT_PATHS and any(_matches(path, p) for p in REPORT_PATHS[name]):
        old, new = TABLE_SHA256
        assert before.count(old) == 1 and after == before.replace(old, new), path
        return 1
    if type(before) is str:
        if HASH.fullmatch(before) and HASH.fullmatch(after):
            assert not INPUT_BINDINGS.intersection(path), path
            assert before.startswith("sha256:") == after.startswith("sha256:"), path
            assert moves is not None and binding_kind(path), path
            moves.append((path, before, after))
            return 0
        x, y = _embedded(before), _embedded(after)
        if x is not None and y is not None:
            return compare(x, y, path + ("<json>",), name=name, moves=moves, added=added)
    raise AssertionError(path)


def check(name, after=None):
    """Compare one carrier with the endpoint; return (restated statements, moved digests, gained certificates)."""
    after = current(name) if after is None else after
    moves, added = [], []
    restated = compare(load(_at(BASE, name)), load(after), name=name, moves=moves, added=added)
    return restated, moves, added


# ---------------------------------------------------------------- recomputation

@cache
def carrier_pairs(name, after):
    before = _at(BASE, name)
    s, c = ce.derived_pairs(ce._plain(before), ce._plain(after))
    return frozenset(s), frozenset({(_sha(before), _sha(after)), *c})


def recompute(read):
    """old -> new for every seal and content digest this scope recomputes from its subjects."""
    seals, content = set(), set()
    for name in SCOPE:
        s, c = carrier_pairs(name, read(name))
        seals |= s
        content |= c
    return ce._forward(seals, "seal"), ce._forward(content, "content")


@cache
def recomputed():
    return recompute(current)


def verify_moves(name, moves, *, seals, content):
    """Every moved digest is bound to its subject: a table digest is a table pair; a seal or content digest equals
    the recomputation of the subject its old value identifies, with no fallback."""
    for path, before, after in moves:
        pair = (before.removeprefix("sha256:"), after.removeprefix("sha256:"))
        kind = binding_kind(path)
        if kind == "table":
            assert pair in TABLE_PAIRS, path
        elif kind == "seal":
            assert seals.get(pair[0]) == pair[1], path
        else:
            assert kind == "content" and content.get(pair[0]) == pair[1], (name, path)


def read_certificates(name, added, text=None):
    """Each document that gained the certificate passes the certificate's own check, which re-derives every record
    from the document's rows; its gained diagnostics are exactly the certificate's."""
    from tests.verifiers import combination_certificate
    plain = json.loads(current(name) if text is None else text)
    documents = [p for p in added if not (p and p[0] == "diagnostics")]
    for entry in (p for p in added if p and p[0] == "diagnostics"):
        _, at, count = entry
        assert at in documents, (name, at)
        document = ce._resolve(plain, ce._pointer(at))
        stated = [d for d in document["diagnostics"] if d["id"].startswith(DIAGNOSTIC)]
        assert len(stated) == count
    for at in documents:
        document = ce._resolve(plain, ce._pointer(at))
        combination_certificate.check(document)
    return len(documents)


# ---------------------------------------------------------------- per carrier

@pytest.mark.parametrize("name", DOCUMENTS)
def test_documents_gain_only_the_derived_certificate_and_its_diagnostics(name):
    restated, moves, added = check(name)
    assert restated == 0 or OLD_DERIVED in _at(BASE, name)
    assert read_certificates(name, added) > 0, f"{name}: gained no certificate"
    verify_moves(name, moves, **dict(zip(("seals", "content"), recomputed())))


@pytest.mark.parametrize("name", BINDINGS + REPORTS)
def test_binding_carriers_move_only_the_table_statement_and_derived_digests(name):
    restated, moves, added = check(name)
    assert added == []
    assert moves or restated, f"{name}: binds the restated table but moved nothing"
    assert restated > 0 or name not in REPORT_PATHS, f"{name}: still states the endpoint table's digest"
    verify_moves(name, moves, **dict(zip(("seals", "content"), recomputed())))


@pytest.mark.parametrize("name", GENERATION)
def test_generation_records_move_only_source_and_output_digests(name):
    before, after = json.loads(_at(BASE, name)), json.loads(current(name))
    moving = {"source_input_files", "source_input_files_after", "inventory_json_sha256", "outputs"}
    assert {k: v for k, v in before.items() if k not in moving} == {k: v for k, v in after.items() if k not in moving}
    assert set(after["source_input_files"]) >= set(before["source_input_files"])
    assert after["source_input_files_after"] == after["source_input_files"]
    # generate_product_preview_mechanics.mjs: the digest of JSON.stringify(sourceInventory).
    inventory = json.dumps(after["source_input_files"], separators=(",", ":"), ensure_ascii=False)
    assert after["inventory_json_sha256"] == _sha(inventory)
    for path, digest in after["source_input_files"].items():
        assert digest == hashlib.sha256((ROOT / path).read_bytes()).hexdigest(), path
    assert len(before["outputs"]) == len(after["outputs"])
    for b, a in zip(before["outputs"], after["outputs"]):
        assert {k: v for k, v in b.items() if k != "sha256"} == {k: v for k, v in a.items() if k != "sha256"}
        assert a["sha256"] == hashlib.sha256((ROOT / a["path"]).read_bytes()).hexdigest(), a["path"]


def test_the_table_texts_and_digests_are_the_table_files():
    for name, (old, new) in TABLES.items():
        assert _sha(_at(BASE, name)) == old and _sha(current(name)) == new, name
    table = json.loads(current("fixtures/results/semantic_contract_v0_3_pressure_2.json"))
    base = json.loads(_at(BASE, "fixtures/results/semantic_contract_v0_3_pressure_2.json"))
    assert base["combination_policy"]["standing"] == OLD_STANDING and table["combination_policy"]["standing"] == NEW_STANDING
    changed = [(b, a) for b, a in zip(base["supported_profile_limitations"], table["supported_profile_limitations"]) if b != a]
    assert len(changed) == 1 and changed[0][0].endswith(OLD_LIMITATION_TAIL)
    assert changed[0][1] == changed[0][0].replace(OLD_LIMITATION_TAIL, NEW_LIMITATION_TAIL)


# ---------------------------------------------------------------- the invented capture's dependants

INVENTED = "fixtures/results/preview_physics_invented_sparse.json"
CARRIER_CASES = "fixtures/results/retained_precision_carrier_cases.json"
READER_OUTCOMES = "fixtures/results/retained_precision_reader_outcomes.json"


def test_the_carrier_cases_move_only_the_invented_captures_pin():
    """The shared carrier cases pin the invented capture by digest (RT-A3 P's step 4 precedent): only that pin moves,
    to the capture's bytes."""
    before, after = json.loads(_at(BASE, CARRIER_CASES)), json.loads(current(CARRIER_CASES))
    pinned = [k for k, f in after["fixtures"].items() if f["path"] == INVENTED]
    assert pinned
    for k in pinned:
        assert before["fixtures"][k]["sha256"] == _sha(_at(BASE, INVENTED))
        assert after["fixtures"][k]["sha256"] == _sha(current(INVENTED))
        before["fixtures"][k]["sha256"] = after["fixtures"][k]["sha256"]
    assert before == after


def test_the_reader_outcomes_move_only_the_invented_captures_input_digests():
    """The R1 native reader outcomes (written by r1_native_reader_outcomes): keys, readings and summaries are token
    for token; only the input digest of an entry built from the invented capture moves."""
    before = json.loads(_at(BASE, READER_OUTCOMES))
    after = json.loads(current(READER_OUTCOMES))
    assert {k: v for k, v in before.items() if k != "entries"} == {k: v for k, v in after.items() if k != "entries"}
    assert [e[0] for e in before["entries"]] == [e[0] for e in after["entries"]]
    moved = []
    for b, a in zip(before["entries"], after["entries"]):
        assert b[2] == a[2], b[0]
        if b[1] != a[1]:
            moved.append(b[0])
    assert moved and all("preview_physics_1" in key for key in moved), moved


# ---------------------------------------------------------------- completeness

def _stating(rev, needle):
    args = ["git", "grep", "-l", "-F", "-e", needle] + ([rev] if rev else []) + ["--", "*.json", *OUTSIDE]
    out = subprocess.run(args, cwd=ROOT, text=True, capture_output=True).stdout
    return {line.split(":", 1)[1] if rev else line for line in out.splitlines()}


def test_every_json_file_binding_the_endpoint_table_is_compared_or_pending():
    """Every JSON file that named the endpoint's pressure-2 table digest is compared or pending its desktop writer;
    outside the pending desktop carriers, none names it now."""
    old = TABLE_SHA256[0]
    at_base = _stating(BASE, old)
    listed = set(SCOPE) | set(PENDING_DESKTOP_WRITERS) | set(NOT_CARRIERS)
    assert at_base - listed == set(), sorted(at_base - listed)
    assert _stating(None, old) <= set(PENDING_DESKTOP_WRITERS), sorted(_stating(None, old))


def test_every_published_document_with_a_combination_states_the_certificate():
    """The producer's completeness (the readers accept a document without the member, which states nothing): every
    solved published document in the compared carriers with a combination gate carries the certificate, gained here,
    and every published document passes the certificate's check."""
    from tests.verifiers import combination_certificate
    for name in DOCUMENTS:
        _, _, added = check(name)
        documents = {p for p in added if not (p and p[0] == "diagnostics")}
        found = []

        def walk(value, path=()):
            if isinstance(value, dict):
                evidence = value.get("contract_evidence")
                if isinstance(evidence, dict) and "formulation_basis" in value:
                    combination_certificate.check(value)
                    solved = isinstance(value.get("status"), dict) and value["status"].get("mechanics") == "MECHANICS_SOLVED"
                    if solved and combination_certificate._gates(evidence):
                        assert CERTIFICATE in evidence, (name, path)
                    if CERTIFICATE in evidence:
                        found.append(path)
                for k, v in value.items():
                    walk(v, path + (k,))
            elif isinstance(value, list):
                for i, v in enumerate(value):
                    walk(v, path + (i,))
        walk(json.loads(current(name)))
        assert set(found) == documents and found, name


def test_every_json_file_stating_the_old_derived_notice_is_compared():
    """Every JSON file that stated the endpoint's derived-rows notice is a compared document carrier, and none states
    it now."""
    at_base = _stating(BASE, OLD_DERIVED)
    assert at_base and at_base <= set(DOCUMENTS), sorted(at_base - set(DOCUMENTS))
    assert _stating(None, OLD_DERIVED) == set(), sorted(_stating(None, OLD_DERIVED))


def test_the_derived_notice_is_the_producers_and_the_readers():
    from tests.verifiers import exact_combinations
    for name in ("core/product_physics/src/exact_combinations.rs",
                 "core/reporting/result_export/src/exact_combination_evidence.rs"):
        assert NEW_DERIVED in current(name) and OLD_DERIVED not in current(name), name
        assert OLD_DERIVED in _at(BASE, name), name
    assert NEW_DERIVED in exact_combinations.DERIVED_TEXT and OLD_DERIVED not in exact_combinations.DERIVED_TEXT


# ---------------------------------------------------------------- negative controls

def _doc(evidence, diagnostics):
    return Obj([("contract_evidence", Obj(evidence)), ("diagnostics", diagnostics), ("results", [Obj([("value", "1.0")])])])


def test_the_guard_refuses_other_moves():
    gate = ("combination_gates", [])
    diag = Obj([("id", "diagnostic:x")])
    new = Obj([("id", DIAGNOSTIC + "c")])
    before = _doc([gate], [diag])
    added = []
    assert compare(before, _doc([gate, (CERTIFICATE, Obj([]))], [diag, new]), added=added) == 0
    assert added == [(), ("diagnostics", (), 1)]
    for changed in (
        _doc([gate, (CERTIFICATE, Obj([]))], [new]),                                     # a diagnostic lost
        _doc([gate, (CERTIFICATE, Obj([]))], [diag, Obj([("id", "diagnostic:y")])]),     # another diagnostic gained
        _doc([gate, ("other", Obj([]))], [diag]),                                        # another member gained
        _doc([(CERTIFICATE, Obj([]))], [diag]),                                          # a member lost
        Obj([("contract_evidence", Obj([gate])), ("diagnostics", [diag]), ("results", [Obj([("value", "1.5")])])]),
    ):
        with pytest.raises(AssertionError):
            compare(before, changed, added=[])
    certified = _doc([gate, (CERTIFICATE, Obj([]))], [diag])
    with pytest.raises(AssertionError):
        compare(certified, _doc([gate, (CERTIFICATE, Obj([("x", "1")]))], [diag]), added=[])  # a stated certificate moved
    with pytest.raises(AssertionError):
        compare(Obj([("model_sha256", "1" * 64)]), Obj([("model_sha256", "2" * 64)]), moves=[])


def test_a_digest_moves_only_at_a_binding_field_and_only_as_derived():
    old, new = "1" * 64, "2" * 64
    with pytest.raises(AssertionError):
        compare(Obj([("unrelated_digest", old)]), Obj([("unrelated_digest", new)]), moves=[])
    for path, pair in ((("semantic_contract", "sha256"), (old, new)),
                       (("dispatch", "binding", "sha256"), (old, new)),
                       (("package_checksum", "value"), (old, new))):
        with pytest.raises(AssertionError):
            verify_moves("x.json", [(path, *pair)], seals={}, content={"3" * 64: "4" * 64})
    verify_moves("x.json", [(("semantic_contract", "sha256"), *TABLE_SHA256),
                            (("dispatch", "binding", "sha256"), *TABLE_SHA256),
                            (("package_checksum", "value"), "3" * 64, "4" * 64)], seals={}, content={"3" * 64: "4" * 64})


def test_a_report_statement_moves_only_by_the_table_digest():
    name = REPORTS[0]
    path = ("request", "report", "report_sections", "limitations", 0, "statement")
    old, new = TABLE_SHA256
    text = f"The profile limitations of the semantic table (sha256 {old}) are stated."

    def doc(statement):
        return Obj([("request", Obj([("report", Obj([("report_sections", Obj([("limitations", [Obj([("statement", statement)])])]))]))]))])
    assert compare(doc(text), doc(text.replace(old, new)), name=name) == 1
    with pytest.raises(AssertionError):
        compare(doc(text), doc(text.replace(old, new) + " "), name=name)
    with pytest.raises(AssertionError):
        compare(doc(text), doc(text.replace(old, "3" * 64)), name=name)
    assert path[-1] == "statement"


def test_a_forged_certificate_and_a_numerical_mutation_are_refused():
    """Negative controls on a real carrier: a gained certificate whose bound is forged fails the certificate's own
    check; one numerical token changed in a row fails the comparison."""
    name = "fixtures/results/pressure_v3_combination_reader_corpus.json"
    import re
    text = current(name)
    # The first stated bound after a certificate member is a combination group's ("combinations" precedes
    # "operand_cases"); it is doubled in place, every other token kept.
    m = re.compile(r'"bound": (-?[0-9.e+-]+)').search(text, text.index('"' + CERTIFICATE + '"'))
    forged = text[:m.start(1)] + repr(float(m.group(1)) * 2) + text[m.end(1):]
    _, _, added = check(name, forged)
    with pytest.raises(ValueError):
        read_certificates(name, added, forged)
    at = text.index('"results": [')
    m = re.compile(r'(?<=[:\[,\s])(-?\d+\.\d*[1-9])(?=[,\]\}\s])').search(text, at)
    digits = m.group(1)
    mutated = text[:m.start()] + digits[:-1] + str((int(digits[-1]) % 9) + 1) + text[m.end():]
    with pytest.raises(AssertionError):
        check(name, mutated)


def test_pending_desktop_carriers_are_named_with_their_writers():
    """The carriers this lane could not regenerate are named, each with the desktop writer that regenerates it."""
    for name, writer in PENDING_DESKTOP_WRITERS.items():
        assert (ROOT / name).is_file(), name
        test_file, variable = writer.split(" ", 1)
        found = [p for p in subprocess.check_output(["git", "ls-files", "apps/desktop/src"], cwd=ROOT, text=True).split()
                 if p.endswith("/" + test_file)]
        assert len(found) == 1, writer
        assert variable.strip("()") in (ROOT / found[0]).read_text(encoding="utf-8"), writer
