"""RT-A3 increment 3 regenerates current producer fixtures, never numerical references.

The preserved endpoint is reducer 2b's, integrated main 3983f51cf (#80 and #95;
no carrier differs from #80's merge 6757b2341);
test_rt_a3_reducer_2b_text_only_envelope_diff keeps the 2b stage's proof frozen
there. This stage restates producer texts (DEL-04-07 `pressure-contract.md`,
"Constant-effort supports (RT-A3 increment 3)"):

- the pressure-2 identity, formulation_basis.limitations[0] of every unselected
  v3 envelope, gains the constant-effort clause after the tee clause;
- the load/reference-state support limitation (S13), a later limitation of every
  load/reference-state envelope, pressure-2 and pressure-source-1 alike, gains
  D-2's sentence before "Spring base motion";
- the support-attribution limitation (S7), a limitation of every pressure-2 and
  pressure-source-1 envelope, names a consumed constant-effort support's signed
  support action (its constant force) beside linear restraints and springs
  (review of ac7fea603);
- the equivalent-static refusal text (a diagnostic message, in no carrier) says
  that a load-state case's *ordinary* applied loads are its source ledger.

Across this stage only those fields and output binding hashes may change.
Exact numeric tokens, member/array order, row sets, diagnostics, selection and
every input-scoped hash (including embedded JSON and the compact retained
corpus) remain invariant, as in the 2a and 2b stages.

A hash may move only at a named output-binding field (`BINDING_FIELDS`), and only
as the changed texts derive it: a table digest moves to the recomputed digest of
the restated table; a publication or receipt digest to a seal the Python 07e
rule recomputes on a carrier in this scope; other content digests to the digest
of a carrier's bytes, an embedded JSON document or a publication in this scope,
recomputed here. The few binding fields whose subject is not materialized in a
carrier (`UNRECOMPUTED`) must move one-to-one across the whole scope. Every
listed carrier states one of the old texts at the endpoint (a stale carrier
fails), except the named hash-only carriers (`HASH_ONLY`), which state none; and
every JSON file that stated one there is listed or named.
"""
from collections import defaultdict
from functools import cache
from pathlib import Path
import hashlib
import json
import subprocess

import pytest

from tests.test_o4_text_only_envelope_diff import HASH, Obj, load, _embedded, _rt_a3_member_path
from tests.test_rt_a3_reducer_text_only_envelope_diff import (
    INPUT_BINDINGS, check_j7_envelope_digests_only, check_rv120_output_digests,
)
from tests.test_rt_a3_reducer_2b_text_only_envelope_diff import (
    NEW as OLD_ID, ORDINARY as ORDINARY_2B, RETAINED, REPORT_IDENTITY_PATHS as REPORT_PATHS_2B, _matches,
)

ROOT = Path(__file__).resolve().parents[1]
BASE = "3983f51cfcef6af26a7846bd63f267425379cfd3"

TEE_TAIL = "and a realized arc it names carries none"
CE_CLAUSE = (
    "; a constant-effort support (DEC-049) declaring exactly one translational DOF and a finite positive constant "
    "load is consumed: it has zero stiffness and applies that load as a constant nodal force along the DOF's "
    "positive global axis in every solved case, and a mechanics combination over a model with one is withheld "
    "(CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE) when a factor is not finite or the exact factor sum is not "
    "within 64 epsilon of 1; a record that is not consumed applies nothing and warns "
    "(SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED)"
)
assert OLD_ID.count(TEE_TAIL + ". ") == 1
NEW_ID = OLD_ID.replace(TEE_TAIL + ". ", TEE_TAIL + CE_CLAUSE + ". ")
OLD_LS = (
    "Global rigid support translations/rotations are prescribed absolute boundary values through the partitioned "
    "solve; reactions come from the unreduced equations. Spring base motion, device preload/reference, inactive or "
    "locked supports are not provided."
)
D2 = ("A consumed constant-effort support's constant force is a device action of the active support in every "
      "state. ")
NEW_LS = OLD_LS.replace("equations. Spring", "equations. " + D2 + "Spring")
OLD_S7 = ("Signed support actions are attributed only for admitted linear restraints and springs; ambiguous coincident "
          "rigid attribution is refused.")
NEW_S7 = OLD_S7.replace("and springs; ", "and springs and, as its constant force, for a consumed constant-effort "
                                         "support; ")
OLD_ES = ("equivalent_static generation is not provided on a load/reference-state document: a case's applied loads "
          "there are exactly its declared source ledger, and generated seismic or wind loads have no ledger class; the "
          "exact route refuses the generation rather than apply loads outside the ledger")
NEW_ES = OLD_ES.replace(
    "a case's applied loads there are exactly its declared source ledger,",
    "a case's ordinary applied loads there are exactly its declared source ledger (a consumed constant-effort "
    "support's force is a device action of the active support),")
TEXT_PAIRS = ((OLD_ID, NEW_ID), (OLD_LS, NEW_LS), (OLD_S7, NEW_S7))
assert all(old != new for old, new in (*TEXT_PAIRS, (OLD_ES, NEW_ES)))
# The restated limitations, old -> new; the identity only at limitations[0].
PAIRS = dict(TEXT_PAIRS)
# ASCII needles (no escape differs between JSON spellings) that only the old texts contain.
NEEDLES = ("a realized arc it names carries none. Pressure regions meeting",
           "reactions come from the unreduced equations. Spring base motion",
           "for admitted linear restraints and springs; ambiguous",
           "a case's applied loads there are exactly its declared source ledger")
# pressure-2 table digest at the endpoint and now (admitted families, combinations limitation, gate 4); and
# physics-retained-1, which inherits it. Both are recomputed from the table files (test below).
TABLES = {
    "fixtures/results/semantic_contract_v0_3_pressure_2.json": (
        "a2db16a515d8e4e1c1bcd6607861df0e7e58cbf1cf82a0b984e2f1c0f128382e",
        "be29fabcfacb1070a51bfb56dcfe61b4e4125638e249c3c0e96d092cc8aba43f"),
    "fixtures/results/semantic_contract_v0_3_physics_retained_1.json": (
        "cf29fd5fc5d61de0f1133e9668b05f3d62ae8f696944fe04de7470029f5357dd",
        "045f6f0d8ce350246b21fb712d2dd14bf3a7f7f3690e66e062cbbf77d3696a3b"),
}
TABLE_SHA256 = TABLES["fixtures/results/semantic_contract_v0_3_pressure_2.json"]
TABLE_PAIRS = frozenset(TABLES.values())

# The pressure-source-1 load/reference-state carriers state S13 and S7 (not the pressure-2 identity).
SOURCE_LOAD_STATE = tuple(
    [f"fixtures/product_preview/pressure_source/load_reference_source/{name}-{mode}.raw.json"
     for name in ("n05", "n06", "eigen_motion", "fields", "mixed")
     for mode in ("sparse_interactive", "dense_scrutiny")]
    + [f"fixtures/results/pressure_source_load_state_{name}_{mode}.{kind}.json"
       for name in ("eigen_motion", "fields", "mixed", "n05", "n06")
       for mode in ("dense", "sparse")
       for kind in ("document", "analysis_run", "stress_neutral")]
    + ["fixtures/results/pressure_source_reader_corpus.json"]
)
# The pressure-source-1 physics carriers state S7 only (review of ac7fea603).
SOURCE_PHYSICS = tuple(
    [f"fixtures/product_preview/pressure_source/physics_source/{name}-{mode}.raw.json"
     for name in ("n05", "n06", "n05_unicode", "n05_units", "fields", "mixed", "mixed_units")
     for mode in ("sparse_interactive", "dense_scrutiny")]
    + ["fixtures/product_preview/pressure_source/o4/o4_source_controls.json"]
    + [f"fixtures/results/pressure_source_plain_n05_{mode}.{kind}.json"
       for mode in ("sparse", "dense") for kind in ("document", "analysis_run", "stress_neutral")]
    + [f"apps/desktop/src/test-support/desktop-shaped/desktop_shaped_pressure_source_plain_n05_{mode}.{kind}.json"
       for mode in ("sparse", "dense") for kind in ("analysis_run", "stress_neutral")]
    + ["fixtures/reports/invented/exact_report_package_requests/pressure_source_1.json",
       "fixtures/reports/invented/user_release_exact/pressure_source_1.json"]
)
ORDINARY = tuple(dict.fromkeys((*ORDINARY_2B, *SOURCE_LOAD_STATE, *SOURCE_PHYSICS)))
assert not set(ORDINARY) & set(RETAINED)
J7 = "fixtures/results/j7_rf_census_expected.json"
READER_OUTCOMES = "fixtures/results/retained_precision_reader_outcomes.json"
RV120 = "fixtures/results/retained_precision_rv120_b3_inputs.json"
# The report fixtures state the texts as statements, not as a published formulation basis: only these exact paths
# (an int matches "#") may carry them. The pressure-source-1 report fixtures restate S7 there.
REPORT_PATHS = {
    **REPORT_PATHS_2B,
    **{f"fixtures/reports/invented/{name}.json": (("request", "report", "report_sections", "limitations", "#", "statement"),)
       for name in ("user_release_exact/pressure_source_1", "exact_report_package_requests/pressure_source_1")},
}
# Listed carriers that state no text: they bind the restated publications only by digest.
HASH_ONLY = {
    **{name: "an AnalysisRun record: it binds the received result and the tables by digest"
       for name in ORDINARY if name.endswith(".analysis_run.json")},
    "fixtures/reports/invented/exact_report_package_requests/pressure_2_straight_substitute.overlay.json":
        "a substitution overlay: its replacements are digests of the restated carriers",
    READER_OUTCOMES: "the native reader outcomes: input and outcome digests (r1_native_reader_outcomes materializes them)",
    J7: "the J7 census: whole-envelope digests only (check_j7_envelope_digests_only)",
}
# JSON files that state an old text at the endpoint but are not compared field by field here.
NOT_COMPARED = {
    # The report statement table is keyed and sorted by each producer string's SHA-256, so S1, S7 and S13 move
    # with their digests; its writer (exact_route_limitations.rs) and the desktop registry check bind it.
    "fixtures/reports/exact_report_statements.json",
}

# Output-binding fields at which a digest may move, by the path's tail (an int matches "#"). Anything else, an
# input binding included, refuses a moved digest.
TABLE_FIELDS = (("semantic_contract", "sha256"),)
SEAL_FIELDS = (("publication_sha256",), ("receipt_sha256",))
CONTENT_FIELDS = (
    ("hashes", "#", "value"), ("received_source_checksums", "#", "value"), ("package_members", "#", "checksum", "value"),
    ("checksums", "#", "value"), ("package_checksum", "value"), ("source_carrier_checksum", "value"),
    ("derivative_hash", "value"), ("raw_source_hashes", "#", "value"), ("run_hashes", "#", "value"),
    ("original_producer_checksum", "value"), ("received_carrier_checksum", "value"),
    ("audit_manifest_refs", "#", "checksum", "value"), ("report_section_refs", "#", "checksum", "value"),
    ("admission", "received_result_sha256"), ("exact_route_core", "received_result_sha256"),
    ("admission", "received_bytes_sha256"), ("admission", "derivative_sha256"),
    ("bound_to", "result_document_derivative_sha256"), ("hash_refs", "#", "value"), ("replacements", "#", "value"),
    ("provenance", "producer_record_sha256"), ("provenance", "fixture_sha256"), ("provenance", "receipt_sha256"),
)
# Binding fields whose subject (a package's member bytes, a run record, a derivative, a report section) is not
# itself a carrier here, so the digest cannot be recomputed from this scope: they must move one-to-one.
UNRECOMPUTED = (
    ("received_source_checksums", "#", "value"), ("package_members", "#", "checksum", "value"),
    ("checksums", "#", "value"), ("package_checksum", "value"), ("hashes", "#", "value"),
    ("derivative_hash", "value"), ("run_hashes", "#", "value"), ("audit_manifest_refs", "#", "checksum", "value"),
    ("report_section_refs", "#", "checksum", "value"), ("admission", "derivative_sha256"),
    ("bound_to", "result_document_derivative_sha256"), ("hash_refs", "#", "value"), ("replacements", "#", "value"),
    ("admission", "received_result_sha256"), ("exact_route_core", "received_result_sha256"),
)


def _tail(path, fields):
    return any(_matches(path[-len(f):], f) for f in fields if len(path) >= len(f))


def binding_kind(path):
    """The class of output-binding field at `path`, or None (a digest may not move there)."""
    if _tail(path, TABLE_FIELDS):
        return "table"
    if _tail(path, SEAL_FIELDS):
        return "seal"
    if _tail(path, CONTENT_FIELDS):
        return "content"
    return None


def _at(rev, name):
    return subprocess.check_output(["git", "show", f"{rev}:{name}"], cwd=ROOT, text=True)


def _states_old(text):
    return any(needle in text for needle in NEEDLES)


def compare(before, after, path=(), *, moves=None, rv120_output_hashes=False, allowed=None):
    """Return the restated-field count; refuse every non-binding or numerical move (2a's rule, three fields).

    Each moved digest is appended to `moves` as (path, before, after) for `verify_moves`; without `moves`, a moved
    digest is refused. `allowed` (default `binding_kind`) names the paths at which a digest may move."""
    allowed = allowed or binding_kind
    assert type(before) is type(after), path
    if isinstance(before, Obj):
        assert [k for k, _ in before] == [k for k, _ in after], path
        if dict(before).get("payload_scope") in {"model", "request", "input_manifest"}:
            assert before == after, path
        return sum(compare(a, b, _rt_a3_member_path(before, key, path), moves=moves,
                           rv120_output_hashes=rv120_output_hashes, allowed=allowed)
                   for (key, a), (_, b) in zip(before, after))
    if isinstance(before, list):
        assert len(before) == len(after), path
        return sum(compare(a, b, path + (i,), moves=moves, rv120_output_hashes=rv120_output_hashes, allowed=allowed)
                   for i, (a, b) in enumerate(zip(before, after)))
    if (type(before) is str and before in PAIRS and len(path) >= 3
            and path[-3:-1] == ("formulation_basis", "limitations") and isinstance(path[-1], int)
            and (before != OLD_ID or path[-1] == 0)):
        assert after == PAIRS[before], path
        return 1
    if before == after:
        return 0
    if type(before) is str:
        if HASH.fullmatch(before) and HASH.fullmatch(after):
            rv120_digest = (rv120_output_hashes and len(path) == 3 and path[0] == "shapes"
                            and isinstance(path[1], int) and path[2] == "input_sha256")
            assert rv120_digest or not INPUT_BINDINGS.intersection(path), path
            assert before.startswith("sha256:") == after.startswith("sha256:"), path
            if not rv120_digest:
                assert moves is not None and allowed(path), path
                moves.append((path, before, after))
            return 0
        a, b = _embedded(before), _embedded(after)
        if a is not None and b is not None:
            return compare(a, b, path + ("<json>",), moves=moves, rv120_output_hashes=rv120_output_hashes, allowed=allowed)
    raise AssertionError(path)


# ---------------------------------------------------------------- recomputation

def _plain(text):
    from tests.retained_precision_corpus import _STORAGE
    document = json.loads(text)
    if isinstance(document, dict) and str(document.get("format", "")).startswith("retained-precision-corpus-dag/"):
        return _STORAGE.expand(document)
    return document


def _sha(text):
    return hashlib.sha256(text.encode()).hexdigest()


SEALS = (("retained_precision", "retained_precision_publication_mp_v2", "retained_precision_receipt_mp_v2"),
         ("source_block_recovery", "source_blocks_publication_v1", "source_blocks_receipt_v1"))


def seal_pairs(before, after):
    """(old, new) publication and receipt digests of a sealed publication whose seals the Python 07e rule
    recomputes at both endpoints; nothing for a copy that is not a whole publication."""
    from tests.verifiers.retained_precision import _hash
    out = []
    for key, publication, receipt in SEALS:
        if not (isinstance(before, dict) and isinstance(before.get(key), dict)
                and isinstance(before[key].get("body"), dict) and "publication_sha256" in before[key]["body"]):
            continue
        sealed = all(
            _hash(publication, {k: v for k, v in doc.items() if k != key}) == doc[key]["body"]["publication_sha256"]
            and _hash(receipt, doc[key]["body"]) == doc[key]["receipt_sha256"]
            for doc in (before, after))
        if sealed:
            out += [(before[key]["body"]["publication_sha256"], after[key]["body"]["publication_sha256"]),
                    (before[key]["receipt_sha256"], after[key]["receipt_sha256"])]
    return out


def derived_pairs(before, after, *, documents=True):
    """(seal pairs, content pairs) recomputed from two endpoints of one carrier, walked in parallel: its sealed
    publications, its embedded JSON (bytes and canonical digests) and, with `documents`, the canonical digest of
    every published document (an object with a formulation basis)."""
    from tests.verifiers.canonical_json import canonical_sha256
    seals, content = set(), set()

    def walk(b, a):
        if isinstance(b, dict) and isinstance(a, dict):
            seals.update(seal_pairs(b, a))
            if documents and "formulation_basis" in b and b != a:
                content.add((canonical_sha256(b), canonical_sha256(a)))
            for key in b.keys() & a.keys():
                walk(b[key], a[key])
        elif isinstance(b, list) and isinstance(a, list):
            for x, y in zip(b, a):
                walk(x, y)
        elif isinstance(b, str) and isinstance(a, str) and b != a and b[:1] in "{[":
            try:
                x, y = json.loads(b), json.loads(a)
            except ValueError:
                return
            content.update({(_sha(b), _sha(a)), (canonical_sha256(x), canonical_sha256(y))})
            walk(x, y)

    walk(before, after)
    return seals, content


def scope():
    return (*ORDINARY, *RETAINED)


@cache
def recomputed():
    """Every seal and content digest pair the scope's carriers recompute, with each carrier's own bytes."""
    seals, content = set(), set()
    for name in scope():
        before, after = _at(BASE, name), (ROOT / name).read_text()
        content.add((_sha(before), _sha(after)))
        s, c = derived_pairs(_plain(before), _plain(after), documents=name != "fixtures/results/pressure_source_reader_corpus.json")
        seals |= s
        content |= c
    return frozenset(seals), frozenset(content)


def verify_moves(moves, *, seals, content, unrecomputed):
    """Every moved digest derives from the restated texts (module docstring). `unrecomputed` collects the
    one-to-one moves of the fields named in UNRECOMPUTED."""
    for path, before, after in moves:
        pair = (before.removeprefix("sha256:"), after.removeprefix("sha256:"))
        kind = binding_kind(path)
        if kind == "table":
            assert pair in TABLE_PAIRS, path
        elif kind == "seal":
            assert pair in seals, path
        elif pair not in content:
            assert kind == "content" and _tail(path, UNRECOMPUTED), path
            unrecomputed.append((path, pair))


def check_one_to_one(unrecomputed):
    """A digest no carrier materializes moves the same way wherever it is bound, and no two subjects meet."""
    forward, backward = defaultdict(set), defaultdict(set)
    for _, (before, after) in unrecomputed:
        forward[before].add(after)
        backward[after].add(before)
    assert all(len(v) == 1 for v in forward.values()), {k: v for k, v in forward.items() if len(v) > 1}
    assert all(len(v) == 1 for v in backward.values()), {k: v for k, v in backward.items() if len(v) > 1}
    assert not set(forward) & set(backward)


# ---------------------------------------------------------------- per carrier

def _restore_report_statements(value, patterns, path=()):
    """Reverse the authorized restatements at the named report paths only; return (value, count)."""
    if isinstance(value, Obj):
        out, count = [], 0
        for key, item in value:
            item, n = _restore_report_statements(item, patterns, path + (key,))
            out.append((key, item))
            count += n
        return Obj(out), count
    if isinstance(value, list):
        out, count = [], 0
        for i, item in enumerate(value):
            item, n = _restore_report_statements(item, patterns, path + (i,))
            out.append(item)
            count += n
        return out, count
    if type(value) is str and any(_matches(path, p) for p in patterns):
        for old, new in PAIRS.items():
            if value == new:
                return old, 1
        # A report statement that quotes the pressure-2 table's digest: only that digest moves.
        if TABLE_SHA256[1] in value:
            return value.replace(TABLE_SHA256[1], TABLE_SHA256[0]), 1
    return value, 0


def check(name, after=None):
    """Compare one carrier with the endpoint; return (restated count, moved digests)."""
    before = _at(BASE, name)
    after = (ROOT / name).read_text() if after is None else after
    moves = []
    if name == READER_OUTCOMES:
        # Input hashes there bind result statements plus invocations; keys, readings and verdict summaries
        # are compared token for token (r1_native_reader_outcomes checks the materialization).
        compare(load(before), load(after), moves=moves, allowed=reader_outcome_digest)
        assert before != after, "reader outcomes still state the predecessor bindings"
        # Only an entry's input digest and its readings' outcome digests move; the eight RV120/N2b inputs move as
        # their pins in retained_precision_contract.rs do (which the Python materialization derives).
        pins = dict(zip(rv120_jcs_pins(_at(BASE, RC_TESTS)), rv120_jcs_pins((ROOT / RC_TESTS).read_text())))
        assert len(pins) == 8
        for path, old, new in moves:
            if old in pins:
                assert new == pins[old], path
        assert {old for _, old, _ in moves} >= set(pins)
        return 0, []
    rv120 = name == RV120
    if rv120:
        check_rv120_output_digests(json.loads(before), json.loads(after), base=BASE)
    if name in REPORT_PATHS:
        restored, moved = _restore_report_statements(load(after), REPORT_PATHS[name])
        moved += compare(load(before), restored, moves=moves)
    else:
        moved = compare(load(before), load(after), moves=moves, rv120_output_hashes=rv120)
    # A carrier that stated an old text must now state the new one: a stale file fails here.
    assert moved > 0 or not _states_old(before), f"{name}: still states the endpoint's texts"
    return moved, moves


RC_TESTS = "core/reporting/result_export/tests/retained_precision_contract.rs"


def reader_outcome_digest(path):
    """A reader-outcome entry's input digest or one reading's outcome digest."""
    return _matches(path, ("entries", "#", 1)) or any(
        _matches(path, ("entries", "#", 2, reading, 0)) for reading in ("bound", "unbound", "transport"))


def rv120_jcs_pins(text):
    """The eight RV120/N2b canonical input pins of retained_precision_contract.rs, in order (N2b, N2, F2)."""
    import re
    out = []
    for name, close in (("REPAIR03_N2B_INPUTS", r"\];"), ("RV120_N2_INPUTS", r"\];"), ("RV120_FORGERY_INPUTS", r"\n\];")):
        out += re.findall(r"[0-9a-f]{64}", re.search(r"const %s[^=]*= \[(.*?)%s" % (name, close), text, re.S).group(1))
    return out


def _check_and_verify(name):
    if name == READER_OUTCOMES:
        return check(name)[0]
    moved, moves = check(name)
    seals, content = recomputed()
    verify_moves(moves, seals=seals, content=content, unrecomputed=[])
    return moved


@pytest.mark.parametrize("name", ORDINARY)
def test_constant_effort_ordinary_carriers_move_only_the_texts_and_derived_binding_hashes(name):
    _check_and_verify(name)


@pytest.mark.parametrize("name", RETAINED)
def test_constant_effort_retained_carriers_move_only_the_texts_and_derived_binding_hashes(name):
    _check_and_verify(name)


def test_constant_effort_j7_moves_only_whole_envelope_digests():
    check_j7_envelope_digests_only(load(_at(BASE, J7)), load((ROOT / J7).read_text()))


def test_unrecomputed_binding_digests_move_one_to_one_across_the_scope():
    seals, content = recomputed()
    unrecomputed = []
    for name in scope():
        if name == READER_OUTCOMES:
            continue
        _, moves = check(name)
        verify_moves(moves, seals=seals, content=content, unrecomputed=unrecomputed)
    assert unrecomputed
    check_one_to_one(unrecomputed)


def test_the_table_digests_are_the_table_files():
    for name, (old, new) in TABLES.items():
        assert _sha(_at(BASE, name)) == old and hashlib.sha256((ROOT / name).read_bytes()).hexdigest() == new, name


# ---------------------------------------------------------------- completeness

def _stating(rev):
    found = set()
    for needle in NEEDLES:
        args = ["git", "grep", "-l", "-F", "-e", needle] + ([rev] if rev else []) + [
            "--", "*.json", ":!validation/", ":!execution/", ":!fixtures/persistence/recorded_model_*"]
        out = subprocess.run(args, cwd=ROOT, text=True, capture_output=True).stdout
        found |= {line.split(":", 1)[1] if rev else line for line in out.splitlines()}
    return found


def check_completeness(listed, hash_only, at_base, base_text):
    """Every file that stated an old text is listed; every listed file stated one, except the named hash-only
    carriers, which state none."""
    assert at_base - listed == set(), sorted(at_base - listed)
    assert set(hash_only) <= listed
    for name in listed:
        if name in hash_only:
            assert name not in at_base and not _states_old(base_text(name)), f"{name}: named hash-only but states a text"
        else:
            assert name in at_base, f"{name}: listed but states no endpoint text and is not named hash-only"


def test_every_json_file_stating_an_old_text_is_compared_or_named():
    """Completeness, outside frozen references, design records and recorded runs; none states an old text now."""
    listed = set(ORDINARY) | set(RETAINED) | {J7} | NOT_COMPARED
    check_completeness(listed, HASH_ONLY, _stating(BASE), lambda n: _at(BASE, n))
    assert _stating(None) == set(), "a JSON file still states an endpoint text"


def test_completeness_rejects_a_silent_list_entry_and_an_unlisted_carrier():
    base = {"a.json": OLD_S7, "b.json": "digests only", "c.json": OLD_LS}
    at_base = {"a.json", "c.json"}
    check_completeness({"a.json", "b.json", "c.json"}, {"b.json": "why"}, at_base, base.get)
    # A listed carrier that stated no text and is not named hash-only.
    with pytest.raises(AssertionError, match="b.json"):
        check_completeness({"a.json", "b.json", "c.json"}, {}, at_base, base.get)
    # A carrier that stated a text but is not listed.
    with pytest.raises(AssertionError):
        check_completeness({"a.json", "b.json"}, {"b.json": "why"}, at_base, base.get)
    # A carrier named hash-only that does state a text.
    with pytest.raises(AssertionError, match="c.json"):
        check_completeness({"a.json", "b.json", "c.json"}, {"b.json": "why", "c.json": "why"}, at_base, base.get)


# ---------------------------------------------------------------- negative controls

def test_constant_effort_guard_rejects_other_moves():
    before = Obj([("formulation_basis", Obj([("limitations", [OLD_ID, "x", OLD_S7, OLD_LS])])),
                  ("results", [Obj([("value", 1.0)])])])
    after = Obj([("formulation_basis", Obj([("limitations", [NEW_ID, "x", NEW_S7, NEW_LS])])),
                 ("results", [Obj([("value", 1.0)])])])
    assert compare(before, after) == 3
    for changed in (
        # A number token or a row moves beside the texts.
        Obj([("formulation_basis", Obj([("limitations", [NEW_ID, "x", NEW_S7, NEW_LS])])), ("results", [Obj([("value", 1)])])]),
        Obj([("formulation_basis", Obj([("limitations", [NEW_ID, "x", NEW_S7, NEW_LS])])), ("results", [])]),
        # Another limitation moves, or a text is restated otherwise.
        Obj([("formulation_basis", Obj([("limitations", [NEW_ID, "y", NEW_S7, NEW_LS])])), ("results", [Obj([("value", 1.0)])])]),
        Obj([("formulation_basis", Obj([("limitations", [NEW_ID + " ", "x", NEW_S7, NEW_LS])])),
             ("results", [Obj([("value", 1.0)])])]),
        Obj([("formulation_basis", Obj([("limitations", [NEW_ID, "x", NEW_S7, OLD_LS + " "])])),
             ("results", [Obj([("value", 1.0)])])]),
        Obj([("formulation_basis", Obj([("limitations", [NEW_ID, "x", OLD_S7 + " ", NEW_LS])])),
             ("results", [Obj([("value", 1.0)])])]),
    ):
        with pytest.raises(AssertionError):
            compare(before, changed)
    # The identity is restated only as limitation 0; no text elsewhere.
    with pytest.raises(AssertionError):
        compare(Obj([("formulation_basis", Obj([("limitations", ["x", OLD_ID])]))]),
                Obj([("formulation_basis", Obj([("limitations", ["x", NEW_ID])]))]))
    for old, new in (*PAIRS.items(), (OLD_ES, NEW_ES)):
        with pytest.raises(AssertionError):
            compare(Obj([("description", old)]), Obj([("description", new)]))
    # Input-scoped hashes stay protected in this stage too.
    with pytest.raises(AssertionError):
        compare(Obj([("model_sha256", "1" * 64)]), Obj([("model_sha256", "2" * 64)]), moves=[])


def test_a_digest_moves_only_at_a_binding_field_and_only_as_derived():
    old, new = "1" * 64, "2" * 64
    # The review's counterexample: an unrelated digest-shaped string moves.
    with pytest.raises(AssertionError):
        compare(Obj([("unrelated_digest", old)]), Obj([("unrelated_digest", new)]), moves=[])
    with pytest.raises(AssertionError):
        compare(Obj([("summary", Obj([("results_sha256", old)]))]), Obj([("summary", Obj([("results_sha256", new)]))]), moves=[])
    # Without a collector no digest may move at all.
    with pytest.raises(AssertionError):
        compare(Obj([("semantic_contract", Obj([("sha256", old)]))]), Obj([("semantic_contract", Obj([("sha256", new)]))]))
    sealed = {(old, new)}
    content = {("3" * 64, "4" * 64)}
    for path, pair in ((("semantic_contract", "sha256"), (old, new)),                      # not the table's digests
                       (("source_block_recovery", "body", "publication_sha256"), ("5" * 64, "6" * 64)),  # no seal
                       (("result_envelope", "reproducibility", "source_origin_bindings", 0,
                         "original_producer_checksum", "value"), (old, new))):           # recomputable, not derived
        with pytest.raises(AssertionError):
            verify_moves([(path, *pair)], seals=sealed, content=content, unrecomputed=[])
    # Derived moves pass: the table pair, a recomputed seal and a recomputed content digest.
    verify_moves([(("semantic_contract", "sha256"), *TABLE_SHA256),
                  (("retained_precision", "receipt_sha256"), old, new),
                  (("result_envelope", "reproducibility", "raw_source_hashes", 0, "value"), "3" * 64, "4" * 64)],
                 seals=sealed, content=content, unrecomputed=[])
    # An unrecomputed binding moves one-to-one or not at all.
    moved = []
    verify_moves([(("package_checksum", "value"), old, new), (("manifest", "checksums", 0, "value"), old, new)],
                 seals=set(), content=set(), unrecomputed=moved)
    check_one_to_one(moved)
    # Reader outcomes: only an entry's input digest and its readings' outcome digests.
    assert reader_outcome_digest(("entries", 3, 1)) and reader_outcome_digest(("entries", 3, 2, "bound", 0))
    for path in (("entries", 3, 0), ("entries", 3, 2, "bound", 1), ("description",)):
        assert not reader_outcome_digest(path)
        with pytest.raises(AssertionError):
            compare(old, new, path, moves=[], allowed=reader_outcome_digest)
    for pairs in ([(old, new), (old, "5" * 64)], [(old, new), ("5" * 64, new)], [(old, new), (new, "5" * 64)]):
        with pytest.raises(AssertionError):
            check_one_to_one([(("package_checksum", "value"), p) for p in pairs])


def test_a_restated_seal_is_derived_and_a_forged_one_is_not():
    """The seal recomputation on a real retained successor: restating S7 and resealing gives a derived pair; a
    receipt moved without its subject does not."""
    from tests.test_retained_precision_b3 import reseal
    from tests.verifiers.retained_precision import EXACT_DEFINITION_HASH
    name = "fixtures/results/retained_precision_exact_successor_sparse_interactive.json"
    document = json.loads((ROOT / name).read_text())
    source, invocation = document["source"], document["invocation"]
    old = json.loads(json.dumps(source))
    old["formulation_basis"]["limitations"] = [OLD_S7 if t == NEW_S7 else t for t in old["formulation_basis"]["limitations"]]
    old = reseal(old, invocation, EXACT_DEFINITION_HASH)
    seals, _ = derived_pairs(old, source)
    assert (old["retained_precision"]["receipt_sha256"], source["retained_precision"]["receipt_sha256"]) in seals
    forged = json.loads(json.dumps(source))
    forged["retained_precision"]["receipt_sha256"] = "7" * 64
    assert derived_pairs(old, forged)[0] == set()


def test_constant_effort_report_paths_are_exact():
    name = "fixtures/reports/exact_route_limitations.json"
    good = Obj([("lists", [Obj([("limitations", [NEW_ID, "x", NEW_S7, NEW_LS])])]), ("strings", [NEW_ID, NEW_S7, NEW_LS])])
    restored, moved = _restore_report_statements(good, REPORT_PATHS[name])
    assert moved == 6 and restored == Obj([("lists", [Obj([("limitations", [OLD_ID, "x", OLD_S7, OLD_LS])])]),
                                           ("strings", [OLD_ID, OLD_S7, OLD_LS])])
    planted = Obj([("lists", [Obj([("producer", NEW_S7)])])])
    restored, moved = _restore_report_statements(planted, REPORT_PATHS[name])
    assert moved == 0
    with pytest.raises(AssertionError):
        compare(Obj([("lists", [Obj([("producer", OLD_S7)])])]), restored)


def test_constant_effort_texts_are_the_producers():
    source = (ROOT / "core/product_physics/src/exact_admission.rs").read_text()
    base = _at(BASE, "core/product_physics/src/exact_admission.rs")
    for old, new in (*PAIRS.items(), (OLD_ES, NEW_ES)):
        assert source.count(json.dumps(new, ensure_ascii=False)) == 1
        assert json.dumps(old, ensure_ascii=False) not in source
    for old in (OLD_LS, OLD_S7, OLD_ES):
        assert base.count(json.dumps(old)) == 1
