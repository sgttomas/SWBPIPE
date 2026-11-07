"""RT-A3 S6 regenerates current producer fixtures, never numerical references.

The preserved endpoint is main 394f4c623 (S6's selected input basis, with the S6 source merged at 8a6596b50, which
changes no carrier). This stage restates producer texts of the uniform ordinary disclosure (DEL-04-07
`pressure-contract.md`; `exact_admission.rs`):

- S1, the pressure-2 identity at formulation_basis.limitations[0] of every unselected v3 envelope: nonlinear supports
  are solved beside objective connectors and on load/reference-state documents (the clause no longer excludes them);
- S13, the load/reference-state support limitation of every load/reference-state envelope (pressure-2 and
  pressure-source-1): boundary motion on an active nonlinear support's DOF and spring base motion are provided;
- S15, the history limitation: contact and friction are static and history-free (DEC-067).

The pressure-2 table restates its admitted-families limitation (so its digest moves, and physics-retained-1's, which
inherits it). Across this stage only those texts and output binding hashes may change. Exact numeric tokens,
member/array order, row sets, diagnostics, selection and every input-scoped hash (embedded JSON and the compact
retained corpus included) remain invariant. The comparison rules are the earlier stages' (`ce.binding_kind`,
`ce.derived_pairs`, `ce.substitute`, `ce.parity_sources`, reused unchanged): a digest moves only at a named
output-binding field and only to the digest recomputed for the subject its endpoint value identifies.

Partition of the earlier stages' scope (`nl.ORDINARY`, `nl.RETAINED`) and of every JSON file this stage changes:
compared field by field (`ORDINARY`, `RETAINED`, with `READER_OUTCOMES`, `RV120` and `J7` by their own derivations);
moved only by the table digests (`TABLE_ONLY`, byte for byte after the substitution); the schema manifest (each moved
digest is its schema file's); left byte-identical (`UNMOVED`: they state no S6 text and bind no restated subject);
or named pending (`DESKTOP_PENDING`, below). The load/reference-state schema is a contract change of S6, not a
carrier (tests/test_load_reference_schema.py).

`DESKTOP_PENDING`: carriers whose writers are the desktop vitest suites, which cannot run in a checkout without the
desktop's installed toolchain. Each still states the endpoint's texts and may differ from the endpoint only by the
table digests (checked); its full comparison is a strict xfail, so a regenerated carrier fails here until it is moved
into the compared scope. They are an explicit exception, not evidence.

The endpoint is the working tree: the stage's carriers as committed on its branch. Freeze END at the stage's merge
when a later stage regenerates these carriers.
"""
from functools import cache
from pathlib import Path
import hashlib
import json
import subprocess

import pytest

from tests.test_o4_text_only_envelope_diff import HASH, Obj, load, _embedded, _rt_a3_member_path
from tests.test_rt_a3_reducer_text_only_envelope_diff import (
    INPUT_BINDINGS, check_j7_envelope_digests_only, check_rv120_output_digests, _rv120_at)
from tests.test_rt_a3_reducer_2b_text_only_envelope_diff import _matches
from tests import test_rt_a3_constant_effort_text_only_envelope_diff as ce
from tests import test_rt_a3_nonlinear_text_only_envelope_diff as nl

ROOT = Path(__file__).resolve().parents[1]
BASE = "394f4c62313679ccf24e67236d80371a0bf62af3"
END = None  # the working tree


@cache
def _at(rev, name):
    return subprocess.check_output(["git", "show", f"{rev}:{name}"], cwd=ROOT, text=True)


def current(name):
    return (ROOT / name).read_text(encoding="utf-8") if END is None else _at(END, name)


# ---------------------------------------------------------------- the texts

_ROUTE = json.loads(_at(BASE, "fixtures/reports/exact_route_limitations.json"))["strings"]


def _one(prefix):
    found = [s for s in _ROUTE if s.startswith(prefix)]
    assert len(found) == 1, prefix
    return found[0]


OLD_ID = _one("3.0.0/exact_pressure_v3 under pressure-2 semantics.")
OLD_LS = _one("Global rigid support translations/rotations")
OLD_HS = _one("History is independent equilibrium only")
_NL = ("gap, one-way, lift-off and friction supports (friction with an explicit normal or a normal derived from a "
       "linear restraint)")
assert OLD_ID.count("; on a document without the load/reference-state group and without an objective connector, "
                    + _NL + " are solved") == 1
NEW_ID = OLD_ID.replace(
    "; on a document without the load/reference-state group and without an objective connector, " + _NL + " are solved",
    "; " + _NL + ", beside objective connectors and on load/reference-state documents included, are solved")
MOTION = ("Boundary motion on an active nonlinear support's own DOF moves its ground: its state is measured on u - g_s "
          "and an active or sticking state prescribes the moved ground (plus or minus the clearance for a gap). Base "
          "motion on an active spring's stiffness axis adds the formed load k*g and the spring publishes -k*(u - g). "
          "Each motion is an absolute reference displacement. ")
assert OLD_LS.count("equations. A consumed") == 1 and OLD_LS.count("Spring base motion, device preload") == 1
NEW_LS = (OLD_LS.replace("equations. A consumed", "equations. " + MOTION + "A consumed")
          .replace("Spring base motion, device preload", "Device preload"))
assert OLD_HS == ("History is independent equilibrium only; no installation, contact, friction or predecessor history "
                  "is represented.")
NEW_HS = ("History is independent equilibrium only: contact and friction are static and history-free (DEC-067), each "
          "case's active-set loop starts from the authored initial states, and no installation or predecessor history "
          "is represented.")
PAIRS = {OLD_ID: NEW_ID, OLD_LS: NEW_LS, OLD_HS: NEW_HS}
assert all(old != new for old, new in PAIRS.items())
assert ([len(new.encode()) - len(old.encode()) for old, new in PAIRS.items()]) == [-12, 351, 114]
# ASCII needles that only the old texts contain.
NEEDLES = ("group and without an objective connector, gap", "Spring base motion, device preload",
           "History is independent equilibrium only; no installation")

TABLES = {
    "fixtures/results/semantic_contract_v0_3_pressure_2.json": (
        "21b7426ec1c50ba5f74539984261554d06b9b6a26ec26f7d5765b5faa7d6bb90",
        "c5fe09a2dc82068f46ce8dd0b5873071af1ec13e62c9bacf0a009570d7e5d2ad"),
    "fixtures/results/semantic_contract_v0_3_physics_retained_1.json": (
        "320dc3f1f75944af8b66da1287e6f8ee5e1fcd5fe005752f2c90bf8c89a9c894",
        "f17bd9d57ee8d1cef4b465715e4edbf030faa5aa4f5282ad7927c25e3f6668d3"),
}
TABLE_SHA256 = TABLES["fixtures/results/semantic_contract_v0_3_pressure_2.json"]
TABLE_PAIRS = frozenset(TABLES.values())
# The pressure-2 table's admitted-families limitation, restated with S1.
TABLE_NL_OLD = "are admitted on documents without the load/reference-state group or an objective connector and are solved"
TABLE_NL_NEW = ("are admitted, beside objective connectors and on load/reference-state documents included, and are "
                "solved")
TABLE_TAIL_OLD = "by the active-set loop;"
TABLE_TAIL_NEW = "by the active-set loop from their authored initial states;"

# ---------------------------------------------------------------- the partition

COMBINATION_CORPUS = "fixtures/results/combination_certificate_corpus.json"
READER_OUTCOMES = nl.READER_OUTCOMES
RV120 = nl.RV120
J7 = nl.J7
STATEMENTS = "fixtures/reports/exact_report_statements.json"
_W11 = ce._W11
_DS = "apps/desktop/src/test-support/desktop-shaped/desktop_shaped_pressure_plain_connected_"
DESKTOP_PENDING = {
    **{_W11 + f"pressure_2_{kind}.json": "W-11 exact-report request (exactReportBelowGate.test.tsx)"
       for kind in ("connector", "load_reference_states", "realized_bend", "straight")},
    "fixtures/reports/invented/user_release_exact/pressure_2_straight.json":
        "D-6 user-release vector (its desktop vitest writer)",
    **{f"{_DS}{mode}.{member}.json": f"desktop-shaped {member} (its desktop vitest writer)"
       for mode in ("dense", "sparse") for member in ("stress_neutral", "analysis_run")},
    nl.REPLAY_REQUESTS: "C1 native replay corpus (its desktop vitest writer)",
}
# Moved only by the table digests: byte for byte equal to the endpoint after substituting the two table pairs.
TABLE_ONLY = (
    "fixtures/results/base_reader_outcomes.json",
    "apps/desktop/src/test-support/native-replay/native_replay_outcomes.json",
    "schemas/analysis_run.v0.3.schema.json",
    "schemas/stress_neutral_export.v0.3.schema.json",
)
SCHEMA_MANIFEST = "tools/release/schema_manifest.json"
SCHEMA_MOVES = ("schemas/analysis_run.v0.3.schema.json", "schemas/load_reference_state.schema.json",
                "schemas/stress_neutral_export.v0.3.schema.json")
NOT_CARRIERS = {
    "schemas/load_reference_state.schema.json": "S6's support-law shape (tests/test_load_reference_schema.py)",
    **dict.fromkeys(TABLES, "the restated tables (test_the_tables_restate_only_the_admitted_families_limitation)"),
    # The statement table is keyed and sorted by each producer string's SHA-256, so S1, S13 and S15 move with their
    # digests; its writer (exact_route_limitations.rs) and the desktop registry check bind it (as in ce.NOT_COMPARED).
    STATEMENTS: "the report statement table (test_the_statement_table_restates_back_to_the_endpoint)",
}
# Earlier stages' carriers that S6 leaves byte-identical (they state no S6 text and bind no restated subject).
UNMOVED = tuple(sorted(
    name for name in (*nl.ORDINARY, *nl.RETAINED)
    if name.startswith(("fixtures/product_preview/pressure_source/physics_source/",
                        "fixtures/product_preview/pressure_source/o4/",
                        "fixtures/results/pressure_source_plain_n05_",
                        "apps/desktop/src/test-support/desktop-shaped/"))
    or name in (ce.OVERLAY, _W11 + "pressure_source_1.json",
                "fixtures/reports/invented/user_release_exact/pressure_source_1.json")))
UNMOVED = tuple(name for name in UNMOVED if name not in DESKTOP_PENDING)
ORDINARY = tuple(name for name in (*nl.ORDINARY, COMBINATION_CORPUS)
                 if name not in DESKTOP_PENDING and name not in UNMOVED)
RETAINED = nl.RETAINED
REPORT_PATHS = ce.REPORT_PATHS
OUTSIDE = nl.OUTSIDE


def binding_kind(path):
    return ce.binding_kind(path)


def _states_old(text):
    return any(needle in text for needle in NEEDLES)


def _substitute_tables(text):
    for old, new in TABLE_PAIRS:
        text = text.replace(old, new)
    return text


# ---------------------------------------------------------------- comparison

def compare(before, after, path=(), *, moves=None, allowed=None, rv120_output_hashes=False):
    """Return the restated-field count; refuse every non-binding or numerical move (the earlier stages' rule): S1 only
    at formulation_basis.limitations[0], S13 and S15 at any formulation_basis.limitations index."""
    allowed = allowed or binding_kind
    assert type(before) is type(after), path
    if isinstance(before, Obj):
        assert [k for k, _ in before] == [k for k, _ in after], path
        if dict(before).get("payload_scope") in {"model", "request", "input_manifest"}:
            assert before == after, path
        return sum(compare(a, b, _rt_a3_member_path(before, key, path), moves=moves, allowed=allowed,
                           rv120_output_hashes=rv120_output_hashes)
                   for (key, a), (_, b) in zip(before, after))
    if isinstance(before, list):
        assert len(before) == len(after), path
        return sum(compare(a, b, path + (i,), moves=moves, allowed=allowed, rv120_output_hashes=rv120_output_hashes)
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
            return compare(a, b, path + ("<json>",), moves=moves, allowed=allowed,
                           rv120_output_hashes=rv120_output_hashes)
    raise AssertionError(path)


def scope():
    """Every carrier whose moved digests verify_moves derives; the reader outcomes have check_reader_outcomes."""
    return (*ORDINARY, *(name for name in RETAINED if name != READER_OUTCOMES))


@cache
def carrier_pairs(name, after):
    before = _at(BASE, name)
    s, c = ce.derived_pairs(ce._plain(before), ce._plain(after),
                            documents=name != "fixtures/results/pressure_source_reader_corpus.json")
    return frozenset(s), frozenset({(ce._sha(before), ce._sha(after)), *c})


def recompute(read):
    """old -> new for every seal and content digest the scope recomputes from its subjects (the earlier stages'
    `recompute`, at this stage's endpoints)."""
    from tests.verifiers.canonical_json import canonical_sha256
    seals, content = set(), set()
    base = lambda name: _at(BASE, name)  # noqa: E731
    for name in scope():
        s, c = carrier_pairs(name, read(name))
        seals |= s
        content |= c
    s, c = ce.derived_pairs(ce.substitute(base), ce.substitute(read))
    seals |= s
    content |= c
    content.update((canonical_sha256(b), canonical_sha256(a))
                   for b, a in zip(ce.parity_sources(base), ce.parity_sources(read)))
    return ce._forward(seals, "seal"), ce._forward(content, "content")


@cache
def recomputed():
    return recompute(current)


def verify_moves(name, moves, *, seals, content):
    """Every moved digest is bound to its subject: a table digest is a table pair; a seal or content digest equals the
    recomputation of the subject its old value identifies, with no fallback and no unmaterialized exception."""
    for path, before, after in moves:
        pair = (before.removeprefix("sha256:"), after.removeprefix("sha256:"))
        kind = binding_kind(path)
        if kind == "table":
            assert pair in TABLE_PAIRS, path
        elif kind == "seal":
            assert seals.get(pair[0]) == pair[1], path
        else:
            assert kind == "content" and content.get(pair[0]) == pair[1], (name, path)


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
        if TABLE_SHA256[1] in value:
            return value.replace(TABLE_SHA256[1], TABLE_SHA256[0]), 1
    return value, 0


def _reader_inputs(rev):
    """The reader inputs this file can rematerialize at `rev` (None: the working tree), keyed as
    r1_native_reader_outcomes keys them (the increment-4 guard's `_reader_inputs`)."""
    from tests.retained_precision_corpus import _STORAGE
    from tests.test_retained_precision_contract import apply_entry
    read = current if rev is None else (lambda name: _at(rev, name))
    corpus = _STORAGE.expand(json.loads(read("fixtures/results/retained_precision_cases.json")))
    cases = {c["id"]: c for c in corpus["cases"]}
    out = {f"corpus:case:{c['id']}": (lambda c=c: [c["source"], c["invocation"]]) for c in corpus["cases"]}
    for kind, member in (("mutation", "mutations"), ("must_pass", "must_pass")):
        for e in corpus[member]:
            out[f"corpus:{kind}:{e['id']}"] = lambda e=e: list(apply_entry(cases[e["base"]], e))
    for s in json.loads(read("fixtures/results/retained_precision_b2_parity_probes.json"))["shapes"]:
        out[f"b2_parity:{s['name']}"] = lambda s=s: list(apply_entry(cases[s["base"]], s))
    from tests import test_retained_precision_b3 as rv
    for s in json.loads(read(RV120))["shapes"]:
        out[f"rv120:{s['name']}"] = ((lambda s=s: list(rv.rv120_input(s))) if rev is None
                                     else (lambda s=s: list(_rv120_at(rev, s))))
    for path in sorted(set(RETAINED) | {f"fixtures/results/retained_precision_{n}.json" for n in (
            "l0_successor_sparse_interactive", "l0_successor_dense_scrutiny", "milestone_successor_sparse_interactive",
            "milestone_successor_dense_scrutiny", "w_c2_successor_sparse_interactive", "w_c2_successor_dense_scrutiny",
            "combination_successor_sparse_interactive", "combination_successor_dense_scrutiny")}):
        stem = path.removeprefix("fixtures/results/retained_precision_").removesuffix(".json")
        if "successor" not in stem:
            continue
        doc = json.loads(read(path))
        out[f"successor:{stem}"] = (lambda doc=doc: [doc["result_envelope"], None]) if "result_envelope" in doc else (
            lambda doc=doc: [doc["source"], doc["invocation"]])
    return out


def check_reader_outcomes(before, after):
    """The native reader outcomes (the increment-4 rule): keys, readings and summaries token for token; every moved
    input digest rematerialized at both endpoints; unmoved entries byte-identical; every refusal digest recomputed
    from its summary; the eight RV120/N2b pins move as retained_precision_contract.rs's do."""
    from tests.verifiers.canonical_json import canonical_sha256
    moves = []
    compare(load(before), load(after), moves=moves, allowed=ce.reader_outcome_digest)
    assert before != after, "the reader outcomes still state the predecessor bindings"
    b, a = ({e[0]: e for e in json.loads(text)["entries"]} for text in (before, after))
    assert list(b) == list(a)
    derived = 0
    for rev, entries in ((BASE, b), (None, a)):
        inputs = _reader_inputs(rev)
        for key, (_, digest, readings) in entries.items():
            if key in inputs and b[key][1] != a[key][1]:
                assert canonical_sha256(inputs[key]()) == digest, (rev, key)
                derived += 1
            for reading, (outcome, summary) in readings.items():
                if not summary.startswith("pass"):
                    assert nl._refusal_digest(summary) == outcome, (rev, key, reading)
    working = _reader_inputs(None)
    for key in b:
        if b[key][1] == a[key][1]:
            assert b[key] == a[key], f"{key}: an unmoved input's outcomes moved"
        else:
            assert key in working, f"{key}: a moved input this file cannot rematerialize"
    pins = dict(zip(ce.rv120_jcs_pins(_at(BASE, ce.RC_TESTS)), ce.rv120_jcs_pins(current(ce.RC_TESTS))))
    assert len(pins) == 8
    for _, old, new in moves:
        if old in pins:
            assert new == pins[old]
    assert {old for _, old, _ in moves} >= set(pins)
    return derived


def check(name, after=None):
    """Compare one carrier with the endpoint; return (restated count, moved digests)."""
    before = _at(BASE, name)
    after = current(name) if after is None else after
    moves = []
    if name == READER_OUTCOMES:
        return check_reader_outcomes(before, after), []
    if name == RV120:
        check_rv120_output_digests(json.loads(before), json.loads(after), base=BASE, end=END)
    if name in REPORT_PATHS:
        restored, moved = _restore_report_statements(load(after), REPORT_PATHS[name])
        moved += compare(load(before), restored, moves=moves)
    else:
        moved = compare(load(before), load(after), moves=moves, rv120_output_hashes=name == RV120)
    # A carrier that stated an old text must now state the new one: a stale file fails here.
    assert moved > 0 or not _states_old(before), f"{name}: still states the endpoint's texts"
    return moved, moves


# ---------------------------------------------------------------- per carrier

@pytest.mark.parametrize("name", ORDINARY)
def test_s6_ordinary_carriers_move_only_the_texts_and_derived_binding_hashes(name):
    _, moves = check(name)
    seals, content = recomputed()
    verify_moves(name, moves, seals=seals, content=content)


@pytest.mark.parametrize("name", RETAINED)
def test_s6_retained_carriers_move_only_the_texts_and_derived_binding_hashes(name):
    moved, moves = check(name)
    if name == READER_OUTCOMES:
        assert moved > 0 and moves == []
        return
    assert moved > 0, f"{name}: states no restated text"
    seals, content = recomputed()
    verify_moves(name, moves, seals=seals, content=content)


def test_s6_j7_moves_only_whole_envelope_digests():
    check_j7_envelope_digests_only(load(_at(BASE, J7)), load(current(J7)))


@pytest.mark.parametrize("name", TABLE_ONLY)
def test_table_only_carriers_move_only_the_table_digests(name):
    before, after = _at(BASE, name), current(name)
    assert before != after and _substitute_tables(before) == after, name
    assert not _states_old(before)


def test_the_schema_manifest_moves_only_the_changed_schemas_digests():
    before, after = (json.loads(text) for text in (_at(BASE, SCHEMA_MANIFEST), current(SCHEMA_MANIFEST)))
    entries = lambda doc: {e["path"]: e["sha256"] for e in doc["schemas"]}  # noqa: E731
    b, a = entries(before), entries(after)
    assert list(b) == list(a)
    assert {path for path in b if b[path] != a[path]} == set(SCHEMA_MOVES)
    assert {k: v for k, v in before.items() if k != "schemas"} == {k: v for k, v in after.items() if k != "schemas"}
    for path in SCHEMA_MOVES:
        assert ce._sha(_at(BASE, path)) == b[path] and ce._sha(current(path)) == a[path], path


def test_the_tables_restate_only_the_admitted_families_limitation():
    """The pressure-2 table differs only in its admitted-families limitation, as S1 does; physics-retained-1 only in
    the digest it inherits. Both digests are the table files'."""
    for name, (old, new) in TABLES.items():
        assert ce._sha(_at(BASE, name)) == old and ce._sha(current(name)) == new, name
    p2 = "fixtures/results/semantic_contract_v0_3_pressure_2.json"
    before = _at(BASE, p2)
    assert before.count(TABLE_NL_OLD) == 1 and before.count(TABLE_NL_OLD + " in each case " + TABLE_TAIL_OLD) == 1
    restated = before.replace(TABLE_NL_OLD + " in each case " + TABLE_TAIL_OLD,
                              TABLE_NL_NEW + " in each case " + TABLE_TAIL_NEW)
    assert restated == current(p2)
    pr1 = "fixtures/results/semantic_contract_v0_3_physics_retained_1.json"
    assert _at(BASE, pr1).replace(TABLE_SHA256[0], TABLE_SHA256[1]) == current(pr1)


def test_the_statement_table_restates_back_to_the_endpoint():
    """Restating S1, S13 and S15 back in the statement table's entries reproduces the endpoint's entries (its keys and
    order are the statements' digests, recomputed here)."""
    before, after = (json.loads(text) for text in (_at(BASE, STATEMENTS), current(STATEMENTS)))
    reverse = {new: old for old, new in PAIRS.items()}

    def restated(doc):
        out = json.loads(json.dumps(doc))
        for entry in out["entries"]:
            if entry["statement"] in reverse:
                # S6's three entries are keyed by their own producer string.
                assert entry["producer_sha256"] == ce._sha(entry["statement"])
                entry["statement"] = reverse[entry["statement"]]
                entry["producer_sha256"] = ce._sha(entry["statement"])
        out["entries"].sort(key=lambda entry: entry["producer_sha256"])
        return out

    assert sum(entry["statement"] in reverse for entry in after["entries"]) == 3
    assert restated(after) == before
    # Negative control: an unrelated statement edited beside them is refused.
    edited = json.loads(json.dumps(after))
    other = next(entry for entry in edited["entries"] if entry["statement"] not in reverse)
    other["statement"] += " "
    assert restated(edited) != before


# ---------------------------------------------------------------- pending and completeness

@pytest.mark.parametrize("name", sorted(DESKTOP_PENDING))
def test_desktop_pending_carriers_moved_only_their_table_digests(name):
    """A named pending carrier still states the endpoint's texts and is the endpoint's bytes or differs from them only
    by the table digests (the repin): nothing else in it was hand-edited."""
    before, after = _at(BASE, name), current(name)
    assert _stale(after), f"{name}: regenerated; move it into the compared scope"
    assert after in (before, _substitute_tables(before)), name


def _stale(text):
    """States an endpoint text or binds an endpoint table digest."""
    return _states_old(text) or any(old in text for old, _ in TABLE_PAIRS)


@pytest.mark.xfail(strict=True, reason="desktop writers not run: no desktop toolchain in this checkout (named pending)")
@pytest.mark.parametrize("name", sorted(DESKTOP_PENDING))
def test_desktop_pending_carriers_restate_the_texts(name):
    if name == nl.REPLAY_REQUESTS or name.endswith(".analysis_run.json"):
        assert not _stale(current(name))
        return
    _, moves = check(name)
    seals, content = recomputed()
    verify_moves(name, moves, seals=seals, content=content)


def _stating(rev):
    found = set()
    for needle in NEEDLES:
        args = ["git", "grep", "-l", "-F", "-e", needle] + ([rev] if rev else []) + ["--", "*.json", *OUTSIDE]
        out = subprocess.run(args, cwd=ROOT, text=True, capture_output=True).stdout
        found |= {line.split(":", 1)[1] if rev else line for line in out.splitlines()}
    return found


def _changed():
    out = subprocess.run(["git", "diff", "--name-only", BASE, "--", "*.json", *OUTSIDE], cwd=ROOT, text=True,
                         capture_output=True, check=True).stdout
    return set(out.split())


def test_every_json_file_is_in_exactly_one_part():
    """The partition is exact: every changed JSON file outside frozen references, design records and recorded runs is
    compared, table-only, the manifest, pending or a named non-carrier; every earlier-stage carrier is compared,
    pending or unmoved; every file stating an old text at the endpoint is compared or pending; and none states one now
    except a named pending carrier."""
    parts = [set(ORDINARY), set(RETAINED), {J7}, set(TABLE_ONLY), {SCHEMA_MANIFEST}, set(DESKTOP_PENDING),
             set(NOT_CARRIERS), set(UNMOVED)]
    assert sum(map(len, parts)) == len(set().union(*parts)), "a file in two parts"
    changed = _changed()
    assert changed - set().union(*parts) == set(), sorted(changed - set().union(*parts))
    for name in UNMOVED:
        assert name not in changed and not _states_old(_at(BASE, name)), name
        assert not any(old in _at(BASE, name) for old, _ in TABLE_PAIRS), name
    assert set(nl.ORDINARY) | set(nl.RETAINED) <= set(ORDINARY) | set(RETAINED) | set(DESKTOP_PENDING) | set(UNMOVED)
    at_base = _stating(BASE)
    compared = set(ORDINARY) | set(RETAINED) | {STATEMENTS}
    assert at_base <= compared | set(DESKTOP_PENDING), sorted(at_base - compared - set(DESKTOP_PENDING))
    assert _stating(None) <= set(DESKTOP_PENDING), sorted(_stating(None) - set(DESKTOP_PENDING))


# ---------------------------------------------------------------- the producer

def test_s6_texts_are_the_producers():
    source = current("core/product_physics/src/exact_admission.rs")
    base = _at(BASE, "core/product_physics/src/exact_admission.rs")
    for old, new in PAIRS.items():
        assert source.count(json.dumps(new, ensure_ascii=False)) == 1
        assert json.dumps(old, ensure_ascii=False) not in source
        assert base.count(json.dumps(old, ensure_ascii=False)) == 1
    strings = json.loads(current("fixtures/reports/exact_route_limitations.json"))["strings"]
    assert all(new in strings for new in PAIRS.values()) and not any(old in strings for old in PAIRS)


# ---------------------------------------------------------------- negative controls

def _basis(*limitations):
    return Obj([("formulation_basis", Obj([("limitations", list(limitations))])), ("results", [Obj([("value", 1.0)])])])


def test_s6_guard_rejects_other_moves():
    assert compare(_basis(OLD_ID, "x", OLD_LS, OLD_HS), _basis(NEW_ID, "x", NEW_LS, NEW_HS)) == 3
    for changed in (
        Obj([("formulation_basis", Obj([("limitations", [NEW_ID, "x", NEW_LS, NEW_HS])])), ("results", [Obj([("value", 1)])])]),
        Obj([("formulation_basis", Obj([("limitations", [NEW_ID, "x", NEW_LS, NEW_HS])])), ("results", [])]),
        _basis(NEW_ID, "y", NEW_LS, NEW_HS),
        _basis(NEW_ID + " ", "x", NEW_LS, NEW_HS),
        _basis(NEW_ID, "x", NEW_LS, OLD_HS + " "),
        # The earlier stage's restatement is not this stage's.
        _basis(nl.OLD_ID, "x", NEW_LS, NEW_HS),
    ):
        with pytest.raises(AssertionError):
            compare(_basis(OLD_ID, "x", OLD_LS, OLD_HS), changed)
    # S1 only at limitations[0]; no text outside formulation_basis.limitations.
    with pytest.raises(AssertionError):
        compare(_basis("x", OLD_ID), _basis("x", NEW_ID))
    for old, new in PAIRS.items():
        with pytest.raises(AssertionError):
            compare(Obj([("description", old)]), Obj([("description", new)]))
    with pytest.raises(AssertionError):
        compare(Obj([("model_sha256", "1" * 64)]), Obj([("model_sha256", "2" * 64)]), moves=[])


def test_a_digest_moves_only_at_a_binding_field_and_only_as_derived():
    old, new = "1" * 64, "2" * 64
    with pytest.raises(AssertionError):
        compare(Obj([("unrelated_digest", old)]), Obj([("unrelated_digest", new)]), moves=[])
    sealed, content = {old: new}, {"3" * 64: "4" * 64}
    for path, pair in ((("semantic_contract", "sha256"), (old, new)),
                       (("semantic_contract", "sha256"), nl.TABLE_SHA256),   # an earlier stage's pair
                       (("retained_precision", "receipt_sha256"), (old, "5" * 64)),
                       (("result_envelope", "reproducibility", "raw_source_hashes", 0, "value"), ("3" * 64, "5" * 64)),
                       (("package_checksum", "value"), (old, new))):
        with pytest.raises(AssertionError):
            verify_moves("x.json", [(path, *pair)], seals=sealed, content=content)
    verify_moves("x.json", [(("semantic_contract", "sha256"), *TABLE_SHA256),
                            (("retained_precision", "receipt_sha256"), old, new),
                            (("result_envelope", "reproducibility", "raw_source_hashes", 0, "value"), "3" * 64, "4" * 64)],
                 seals=sealed, content=content)


def test_a_stale_or_table_only_regression_is_refused():
    """A carrier left at the endpoint's bytes, a table-only carrier with any other byte moved, and a pending carrier
    with a hand-edited text are each refused."""
    stale = "fixtures/results/pressure_load_state_connected_sparse.document.json"
    with pytest.raises(AssertionError):
        check(stale, _at(BASE, stale))
    name = TABLE_ONLY[0]
    after = current(name)
    assert _substitute_tables(_at(BASE, name)) == after
    assert _substitute_tables(_at(BASE, name)) != after.replace(TABLE_SHA256[1], "0" * 64, 1)
    pending = sorted(DESKTOP_PENDING)[0]
    assert _substitute_tables(_at(BASE, pending)) != current(pending).replace(NEEDLES[0], NEEDLES[0].upper(), 1)


@pytest.mark.parametrize("name", [
    "fixtures/results/retained_precision_exact_successor_sparse_interactive.json",
    "fixtures/results/retained_precision_o4_w1e_successor_dense_scrutiny.json",
    "fixtures/results/retained_precision_exact_successor_derivative_sparse_interactive.json",
    "fixtures/results/retained_precision_cases.json",
    "fixtures/results/pressure_load_state_connected_dense.document.json",
    RV120,
])
def test_a_numerical_mutation_is_refused(name):
    """One numerical token changed in a regenerated carrier (in the compact corpus, a stored scalar node, so every
    occurrence) fails the comparison, however its digests are left."""
    text = current(name)
    anchor = next(k for k in ('"nodes"', '"results"', '"result_sets"', '"edits"') if k in text)
    mutated = nl._bump_first_number(text, anchor)
    assert mutated != text
    with pytest.raises(AssertionError):
        _, moves = check(name, mutated)
        seals, content = recompute(lambda n: mutated if n == name else current(n))
        verify_moves(name, moves, seals=seals, content=content)


def test_a_leftover_reader_outcome_or_forged_digest_is_refused():
    with pytest.raises(AssertionError):
        check(READER_OUTCOMES, _at(BASE, READER_OUTCOMES))
    text = current(READER_OUTCOMES)
    entries = json.loads(text)["entries"]
    base = {e[0]: e for e in json.loads(_at(BASE, READER_OUTCOMES))["entries"]}
    moved = next(e for e in entries if base[e[0]][1] != e[1])
    with pytest.raises(AssertionError):
        check_reader_outcomes(_at(BASE, READER_OUTCOMES), text.replace(moved[1], "7" * 64))
    refusal = next(e for e in entries if not e[2]["unbound"][1].startswith("pass"))
    with pytest.raises(AssertionError):
        check_reader_outcomes(_at(BASE, READER_OUTCOMES), text.replace(refusal[2]["unbound"][0], "8" * 64))
