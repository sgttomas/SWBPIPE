"""RT-A3 increment 4 regenerates current producer fixtures, never numerical references.

The preserved endpoint is RT-A4's, integrated main bbf384dec (#98);
test_rt_a3_constant_effort_text_only_envelope_diff keeps the constant-effort stage's
proof frozen at its own endpoint. This stage restates producer texts (DEL-04-07
`pressure-contract.md`, "Nonlinear supports on EX (RT-A3 increment 4)", Disclosure):

- the pressure-2 identity, formulation_basis.limitations[0] of every unselected v3
  envelope, gains the nonlinear-support clause after the constant-effort clause;
- the support-attribution limitation (S7), a limitation of every pressure-2 and
  pressure-source-1 envelope, names nonlinear supports beside linear restraints and
  springs.

Across this stage only those two fields and output binding hashes may change. Exact
numeric tokens, member/array order, row sets, diagnostics, selection and every
input-scoped hash (including embedded JSON and the compact retained corpus) remain
invariant, as in the earlier stages. The comparison rules are the constant-effort
guard's (its `binding_kind`, `derived_pairs`, `substitute` and `parity_sources` are
reused unchanged); only the texts, the tables, the endpoints and the enumerated
exceptions are this stage's.

A hash may move only at a named output-binding field, and only to the digest
recomputed here for the subject its endpoint value identifies, with no fallback. The
exceptions are enumerated:

- `UNMATERIALIZED`: the AnalysisRun records of the W-11 witness requests and of the
  same-run substitute, which no carrier holds (as in the constant-effort stage). Each
  occurrence is named by carrier, JSON pointer and reviewed (old, new) pair, checked to
  name its request's AnalysisRun in the W-11 writer's carriers, and moves one-to-one
  across the scope. That is explicitly limited consistency evidence, not a derivation
  proof: it rests on the writer's regeneration and staleness check
  (exactReportBelowGate.test.tsx).
- `RETAINED_PENDING`: the retained carriers (the exact and O4 successors, the exact
  derivative goldens, the mirrored corpus cases, the RV120 inputs and the native
  reader outcomes). They are written only on a registered build, and this stage's
  retained-memory registration is a separate later step (DEL-04-08), so they still
  state the endpoint's texts here; each is named, and no other JSON file may.

The endpoint is the working tree: the stage's carriers as committed on its branch.
"""
from functools import cache
from pathlib import Path
import json
import subprocess

import pytest

from tests.test_o4_text_only_envelope_diff import HASH, Obj, load, _embedded, _rt_a3_member_path
from tests.test_rt_a3_reducer_text_only_envelope_diff import INPUT_BINDINGS, check_j7_envelope_digests_only
from tests.test_rt_a3_reducer_2b_text_only_envelope_diff import _matches
from tests import test_rt_a3_constant_effort_text_only_envelope_diff as ce

ROOT = Path(__file__).resolve().parents[1]
BASE = "bbf384dec4b320b88ecdbfece96e0ae5cc8c9205"

CE_TAIL = "(SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED)"
NL_CLAUSE = (
    "; on a document without the load/reference-state group and without an objective connector, gap, one-way, "
    "lift-off and friction supports (friction with an explicit normal or a normal derived from a linear restraint) "
    "are solved in each case by the active-set loop from their authored initial states, with K-D5 confirmation, and "
    "publish their states, final displacements and reactions; a case with a gap support is published with the "
    "unqualified recovery basis (NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED) and no confirmation, and a "
    "mechanics combination over a model with a nonlinear support is withheld (NONLINEAR_COMBINATION_REQUIRES_SOLVE) "
    "unless it is one load case at factor exactly 1 plus at least one combination whose fixed-load count is exactly "
    "0, formed from rows as a solved state plus derived differences or bounds, not an equilibrium state"
)
# The endpoint's texts are the constant-effort stage's restated ones (RT-A4 restated neither).
OLD_ID, OLD_S7 = ce.NEW_ID, ce.NEW_S7
assert OLD_ID.count(CE_TAIL + ". ") == 1
NEW_ID = OLD_ID.replace(CE_TAIL + ". ", CE_TAIL + NL_CLAUSE + ". ")
NEW_S7 = OLD_S7.replace("linear restraints and springs and, as its constant force",
                        "linear restraints, springs and nonlinear supports and, as its constant force")
PAIRS = {OLD_ID: NEW_ID, OLD_S7: NEW_S7}
assert all(old != new for old, new in PAIRS.items())
assert (len(NEW_ID.encode()) - len(OLD_ID.encode()), len(NEW_S7.encode()) - len(OLD_S7.encode())) == (854, 20)
# ASCII needles that only the old texts contain.
NEEDLES = (CE_TAIL + ". Pressure regions meeting",
           "for admitted linear restraints and springs and, as its constant force")
# pressure-2 table digest at the endpoint and now (admitted-families and combinations limitations); and
# physics-retained-1, which inherits it. Both are recomputed from the table files (test below).
TABLES = {
    "fixtures/results/semantic_contract_v0_3_pressure_2.json": (
        "10455bf348b46efac998b6635b537c9e3149a7d1c7a29ea96c0e3783a2de9ad6",
        "5e224d1730033151f8de2ebcc47c6c80558f9c907f9f15ee477a7ee4b18593e6"),
    "fixtures/results/semantic_contract_v0_3_physics_retained_1.json": (
        "040ef9a613607cb6cb4aab6f34646481efd81e060681c364b72df4e129c8c034",
        "ca6bb9834f1cf2f970df4fd21a81e101c9289f2fa8cb3e26e7a11a794f535b5f"),
}
TABLE_SHA256 = TABLES["fixtures/results/semantic_contract_v0_3_pressure_2.json"]
TABLE_PAIRS = frozenset(TABLES.values())

CE2 = "fixtures/results/rt_a3_ce2_withheld_sparse.json"
# The constant-effort stage's ordinary scope (every pressure-2 and pressure-source-1 carrier states S7), and the
# CE-2 gate-withheld envelope committed in that stage's review repair.
ORDINARY = (*ce.ORDINARY, CE2)
RETAINED_PENDING = ce.RETAINED
J7 = ce.J7
REPORT_PATHS = ce.REPORT_PATHS
HASH_ONLY = {
    **{name: "an AnalysisRun record: it binds the received result and the tables by digest"
       for name in ORDINARY if name.endswith(".analysis_run.json")},
    ce.OVERLAY: "a substitution overlay: its replacements are digests of the restated carriers",
    J7: "the J7 census: whole-envelope digests only (check_j7_envelope_digests_only)",
}
NOT_COMPARED = dict.fromkeys(ce.NOT_COMPARED)
# Outside this stage's comparison: frozen references, design records, and recorded runs and the historical table
# variants they bind (kept verbatim; their texts are history).
OUTSIDE = (":!validation/", ":!execution/", ":!fixtures/persistence/recorded_model_*",
           ":!fixtures/persistence/recorded_pre_rt_a4_pf/", ":!fixtures/results/recorded_pre_rt_a4/")

_W11 = ce._W11
UNMATERIALIZED = {
    (_W11 + "pressure_2_connector.json", "/request/report/calculation_report/audit_manifest_refs/0/checksum/value"): (
        "6f1fa9d067df249ba255ea17b2a23556634112ea5a36f17d8fa4b5578c518b3e",
        "d33a15b84e4d199fca28261c52d9573923fc74ff935a6fb567199d4d20f3bcc0"),
    (_W11 + "pressure_2_connector.json", "/request/result_documents/0/result_envelope/reproducibility/run_hashes/0/value"): (
        "6f1fa9d067df249ba255ea17b2a23556634112ea5a36f17d8fa4b5578c518b3e",
        "d33a15b84e4d199fca28261c52d9573923fc74ff935a6fb567199d4d20f3bcc0"),
    (_W11 + "pressure_2_connector.json", "/request/state_comparison_handoff_records/0/sections/state_run_sections/0/hash_refs/0/value"): (
        "6f1fa9d067df249ba255ea17b2a23556634112ea5a36f17d8fa4b5578c518b3e",
        "d33a15b84e4d199fca28261c52d9573923fc74ff935a6fb567199d4d20f3bcc0"),
    (_W11 + "pressure_2_load_reference_states.json", "/request/report/calculation_report/audit_manifest_refs/0/checksum/value"): (
        "780b7956345f6c5949d081247cdc5a64f7d9921f861e946bfd73e2d2dd2c6afe",
        "0bf567b22f0e4233369d41d8ef066f1742e63bde87be34d08b7cf6f07dd45e49"),
    (_W11 + "pressure_2_load_reference_states.json", "/request/result_documents/0/result_envelope/reproducibility/run_hashes/0/value"): (
        "780b7956345f6c5949d081247cdc5a64f7d9921f861e946bfd73e2d2dd2c6afe",
        "0bf567b22f0e4233369d41d8ef066f1742e63bde87be34d08b7cf6f07dd45e49"),
    (_W11 + "pressure_2_load_reference_states.json", "/request/state_comparison_handoff_records/0/sections/state_run_sections/0/hash_refs/0/value"): (
        "780b7956345f6c5949d081247cdc5a64f7d9921f861e946bfd73e2d2dd2c6afe",
        "0bf567b22f0e4233369d41d8ef066f1742e63bde87be34d08b7cf6f07dd45e49"),
    (_W11 + "pressure_2_realized_bend.json", "/request/report/calculation_report/audit_manifest_refs/0/checksum/value"): (
        "3b309ec8cedb1e3f1a29ccf8cde75900093a089f6df027932390f9995d6a826b",
        "be6d2452a2309a658a17bcd86e05bf712092bb926f3d4c4566994cc50ffa3949"),
    (_W11 + "pressure_2_realized_bend.json", "/request/result_documents/0/result_envelope/reproducibility/run_hashes/0/value"): (
        "3b309ec8cedb1e3f1a29ccf8cde75900093a089f6df027932390f9995d6a826b",
        "be6d2452a2309a658a17bcd86e05bf712092bb926f3d4c4566994cc50ffa3949"),
    (_W11 + "pressure_2_realized_bend.json", "/request/state_comparison_handoff_records/0/sections/state_run_sections/0/hash_refs/0/value"): (
        "3b309ec8cedb1e3f1a29ccf8cde75900093a089f6df027932390f9995d6a826b",
        "be6d2452a2309a658a17bcd86e05bf712092bb926f3d4c4566994cc50ffa3949"),
    (_W11 + "pressure_2_straight.json", "/request/report/calculation_report/audit_manifest_refs/0/checksum/value"): (
        "473ce7e4470fc91473ea1c1dd1135b891bd403dada0b799e3956cdc00f4341f8",
        "a8607e873ffe3eab728bc1ca30b9cb050c37c6573219eb2b2e586127e29f4986"),
    (_W11 + "pressure_2_straight.json", "/request/result_documents/0/result_envelope/reproducibility/run_hashes/0/value"): (
        "473ce7e4470fc91473ea1c1dd1135b891bd403dada0b799e3956cdc00f4341f8",
        "a8607e873ffe3eab728bc1ca30b9cb050c37c6573219eb2b2e586127e29f4986"),
    (_W11 + "pressure_2_straight.json", "/request/state_comparison_handoff_records/0/sections/state_run_sections/0/hash_refs/0/value"): (
        "473ce7e4470fc91473ea1c1dd1135b891bd403dada0b799e3956cdc00f4341f8",
        "a8607e873ffe3eab728bc1ca30b9cb050c37c6573219eb2b2e586127e29f4986"),
    (_W11 + "pressure_source_1.json", "/request/report/calculation_report/audit_manifest_refs/0/checksum/value"): (
        "98f1e65ce6dba54439c2865b24647d5b8ec138ab0facd32ecb0a5da6d7a5cfad",
        "5bb99dc36ab19e7502afa3ba407f88c541f28ed1991b3ace3e064f64ccf69807"),
    (_W11 + "pressure_source_1.json", "/request/result_documents/0/result_envelope/reproducibility/run_hashes/0/value"): (
        "98f1e65ce6dba54439c2865b24647d5b8ec138ab0facd32ecb0a5da6d7a5cfad",
        "5bb99dc36ab19e7502afa3ba407f88c541f28ed1991b3ace3e064f64ccf69807"),
    (_W11 + "pressure_source_1.json", "/request/state_comparison_handoff_records/0/sections/state_run_sections/0/hash_refs/0/value"): (
        "98f1e65ce6dba54439c2865b24647d5b8ec138ab0facd32ecb0a5da6d7a5cfad",
        "5bb99dc36ab19e7502afa3ba407f88c541f28ed1991b3ace3e064f64ccf69807"),
    (_W11 + "pressure_2_straight_substitute.overlay.json", "/replacements/1/value"): (
        "b6b34276bd647412ca5132ebf8fc7a929581897540151081e374c121c1ca904a",
        "d36a55e57df2798a5daa0f8f11840871b8b2c517154343a2d3d5bf4eb289354c"),
    (_W11 + "pressure_2_straight_substitute.overlay.json", "/replacements/6/value"): (
        "b6b34276bd647412ca5132ebf8fc7a929581897540151081e374c121c1ca904a",
        "d36a55e57df2798a5daa0f8f11840871b8b2c517154343a2d3d5bf4eb289354c"),
    (_W11 + "pressure_2_straight_substitute.overlay.json", "/replacements/16/value"): (
        "b6b34276bd647412ca5132ebf8fc7a929581897540151081e374c121c1ca904a",
        "d36a55e57df2798a5daa0f8f11840871b8b2c517154343a2d3d5bf4eb289354c"),
}


def binding_kind(path):
    return ce.binding_kind(path)


@cache
def _at(rev, name):
    return subprocess.check_output(["git", "show", f"{rev}:{name}"], cwd=ROOT, text=True)


def current(name):
    return (ROOT / name).read_text(encoding="utf-8")


def _states_old(text):
    return any(needle in text for needle in NEEDLES)


def compare(before, after, path=(), *, moves=None, allowed=None):
    """Return the restated-field count; refuse every non-binding or numerical move (the earlier stages' rule).

    Each moved digest is appended to `moves` as (path, before, after) for `verify_moves`; without `moves`, a moved
    digest is refused. `allowed` (default `binding_kind`) names the paths at which a digest may move."""
    allowed = allowed or binding_kind
    assert type(before) is type(after), path
    if isinstance(before, Obj):
        assert [k for k, _ in before] == [k for k, _ in after], path
        if dict(before).get("payload_scope") in {"model", "request", "input_manifest"}:
            assert before == after, path
        return sum(compare(a, b, _rt_a3_member_path(before, key, path), moves=moves, allowed=allowed)
                   for (key, a), (_, b) in zip(before, after))
    if isinstance(before, list):
        assert len(before) == len(after), path
        return sum(compare(a, b, path + (i,), moves=moves, allowed=allowed) for i, (a, b) in enumerate(zip(before, after)))
    if (type(before) is str and before in PAIRS and len(path) >= 3
            and path[-3:-1] == ("formulation_basis", "limitations") and isinstance(path[-1], int)
            and (before != OLD_ID or path[-1] == 0)):
        assert after == PAIRS[before], path
        return 1
    if before == after:
        return 0
    if type(before) is str:
        if HASH.fullmatch(before) and HASH.fullmatch(after):
            assert not INPUT_BINDINGS.intersection(path), path
            assert before.startswith("sha256:") == after.startswith("sha256:"), path
            assert moves is not None and allowed(path), path
            moves.append((path, before, after))
            return 0
        a, b = _embedded(before), _embedded(after)
        if a is not None and b is not None:
            return compare(a, b, path + ("<json>",), moves=moves, allowed=allowed)
    raise AssertionError(path)


# ---------------------------------------------------------------- recomputation

def scope():
    return ORDINARY


@cache
def carrier_pairs(name, after):
    """(seal pairs, content pairs) of one carrier: its bytes' digest and what `derived_pairs` derives."""
    before = _at(BASE, name)
    s, c = ce.derived_pairs(ce._plain(before), ce._plain(after),
                            documents=name != "fixtures/results/pressure_source_reader_corpus.json")
    return frozenset(s), frozenset({(ce._sha(before), ce._sha(after)), *c})


def recompute(read):
    """old -> new for every seal and every content digest the scope recomputes from its subjects (the constant-effort
    stage's `recompute`, at this stage's endpoints)."""
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


def verify_moves(name, moves, *, seals, content, unmaterialized):
    """Every moved digest in carrier `name` is bound to its subject: a table digest is a table pair; a seal or content
    digest equals the recomputation of the subject its old value identifies, with no fallback; only an occurrence
    named in UNMATERIALIZED may instead move, exactly as named there, and only when its subject is not recomputable."""
    for path, before, after in moves:
        pair = (before.removeprefix("sha256:"), after.removeprefix("sha256:"))
        kind = binding_kind(path)
        if kind == "table":
            assert pair in TABLE_PAIRS, path
        elif kind == "seal":
            assert seals.get(pair[0]) == pair[1], path
        elif (name, ce._pointer(path)) in UNMATERIALIZED:
            assert kind == "content" and pair == UNMATERIALIZED[name, ce._pointer(path)], path
            assert pair[0] not in content, f"{path}: recomputable, so not an exception"
            unmaterialized.append(((name, ce._pointer(path)), pair))
        else:
            assert kind == "content" and content.get(pair[0]) == pair[1], path


def check_unmaterialized_subjects(read):
    """Each named occurrence binds the AnalysisRun record of its request's run, in a carrier the W-11 writer produces
    (the constant-effort stage's check, over this stage's occurrences)."""
    for name, pointer in UNMATERIALIZED:
        carrier = json.loads(read(name))
        assert carrier["generator"] == "apps/desktop/src/features/report/exactReportBelowGate.test.tsx", name
        assert pointer.endswith("/value"), pointer
        if name == ce.OVERLAY:
            request, at = ce.substitute(read), ce._resolve(carrier, pointer[:-len("/value")])["pointer"]
        else:
            request, at = carrier["request"], pointer.removeprefix("/request")
        held = ce._resolve(request, at[:-len("/value")])
        assert held["value"] == ce._resolve(carrier, pointer), (name, pointer)
        ref = held["payload_ref"]
        assert ref.get("ref_type", ref.get("object_type")) in {"analysis_run", "AnalysisRun"}, (name, pointer)
        assert ref.get("ref_id", ref.get("ref")) == request["report"]["exact_route"]["admission"]["run_ref"], (name, pointer)


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
        if TABLE_SHA256[1] in value:
            return value.replace(TABLE_SHA256[1], TABLE_SHA256[0]), 1
    return value, 0


def check(name, after=None):
    """Compare one carrier with the endpoint; return (restated count, moved digests)."""
    before = _at(BASE, name)
    after = current(name) if after is None else after
    moves = []
    if name in REPORT_PATHS:
        restored, moved = _restore_report_statements(load(after), REPORT_PATHS[name])
        moved += compare(load(before), restored, moves=moves)
    else:
        moved = compare(load(before), load(after), moves=moves)
    # A carrier that stated an old text must now state the new one: a stale file fails here.
    assert moved > 0 or not _states_old(before), f"{name}: still states the endpoint's texts"
    return moved, moves


@pytest.mark.parametrize("name", ORDINARY)
def test_nonlinear_ordinary_carriers_move_only_the_texts_and_derived_binding_hashes(name):
    moved, moves = check(name)
    seals, content = recomputed()
    verify_moves(name, moves, seals=seals, content=content, unmaterialized=[])


def test_nonlinear_j7_moves_only_whole_envelope_digests():
    check_j7_envelope_digests_only(load(_at(BASE, J7)), load(current(J7)))


def unmaterialized_moves(read=current):
    seals, content = recompute(read)
    unmaterialized = []
    for name in scope():
        moves = check(name, read(name))[1]
        verify_moves(name, moves, seals=seals, content=content, unmaterialized=unmaterialized)
    return unmaterialized


def test_the_unmaterialized_digests_are_exactly_the_named_occurrences_and_move_one_to_one():
    unmaterialized = unmaterialized_moves()
    assert sorted(occurrence for occurrence, _ in unmaterialized) == sorted(UNMATERIALIZED)
    ce.check_one_to_one(unmaterialized)
    for read in ((lambda name: _at(BASE, name)), current):
        check_unmaterialized_subjects(read)


def test_the_table_digests_are_the_table_files():
    for name, (old, new) in TABLES.items():
        assert ce._sha(_at(BASE, name)) == old and ce._sha(current(name)) == new, name


# ---------------------------------------------------------------- completeness

def _stating(rev):
    found = set()
    for needle in NEEDLES:
        args = ["git", "grep", "-l", "-F", "-e", needle] + ([rev] if rev else []) + ["--", "*.json", *OUTSIDE]
        out = subprocess.run(args, cwd=ROOT, text=True, capture_output=True).stdout
        found |= {line.split(":", 1)[1] if rev else line for line in out.splitlines()}
    return found


def test_every_json_file_stating_an_old_text_is_compared_named_or_pending():
    """Completeness, outside frozen references, design records and recorded runs: every file that stated an old text
    at the endpoint is compared, named or pending registration; now only the named retained carriers still do."""
    listed = set(ORDINARY) | set(RETAINED_PENDING) | {J7} | set(NOT_COMPARED)
    hash_only = {**HASH_ONLY, **{name: "pending registration" for name in RETAINED_PENDING if not _states_old(_at(BASE, name))}}
    at_base = _stating(BASE)
    assert at_base - listed == set(), sorted(at_base - listed)
    for name in listed:
        if name in hash_only:
            assert name not in at_base and not _states_old(_at(BASE, name)), f"{name}: named hash-only but states a text"
        else:
            assert name in at_base, f"{name}: listed but states no endpoint text and is not named hash-only"
    # Now: only the retained carriers awaiting the registered build may still state an endpoint text.
    assert _stating(None) == {name for name in RETAINED_PENDING if name in at_base}, sorted(_stating(None))


# ---------------------------------------------------------------- negative controls

def test_nonlinear_guard_rejects_other_moves():
    before = Obj([("formulation_basis", Obj([("limitations", [OLD_ID, "x", OLD_S7])])), ("results", [Obj([("value", 1.0)])])])
    after = Obj([("formulation_basis", Obj([("limitations", [NEW_ID, "x", NEW_S7])])), ("results", [Obj([("value", 1.0)])])])
    assert compare(before, after) == 2
    for changed in (
        Obj([("formulation_basis", Obj([("limitations", [NEW_ID, "x", NEW_S7])])), ("results", [Obj([("value", 1)])])]),
        Obj([("formulation_basis", Obj([("limitations", [NEW_ID, "x", NEW_S7])])), ("results", [])]),
        Obj([("formulation_basis", Obj([("limitations", [NEW_ID, "y", NEW_S7])])), ("results", [Obj([("value", 1.0)])])]),
        Obj([("formulation_basis", Obj([("limitations", [NEW_ID + " ", "x", NEW_S7])])), ("results", [Obj([("value", 1.0)])])]),
        Obj([("formulation_basis", Obj([("limitations", [NEW_ID, "x", OLD_S7 + " "])])), ("results", [Obj([("value", 1.0)])])]),
        # The constant-effort stage's restatement is not this stage's.
        Obj([("formulation_basis", Obj([("limitations", [ce.OLD_ID, "x", NEW_S7])])), ("results", [Obj([("value", 1.0)])])]),
    ):
        with pytest.raises(AssertionError):
            compare(before, changed)
    with pytest.raises(AssertionError):
        compare(Obj([("formulation_basis", Obj([("limitations", ["x", OLD_ID])]))]),
                Obj([("formulation_basis", Obj([("limitations", ["x", NEW_ID])]))]))
    for old, new in PAIRS.items():
        with pytest.raises(AssertionError):
            compare(Obj([("description", old)]), Obj([("description", new)]))
    with pytest.raises(AssertionError):
        compare(Obj([("model_sha256", "1" * 64)]), Obj([("model_sha256", "2" * 64)]), moves=[])


def test_a_digest_moves_only_at_a_binding_field_and_only_as_derived():
    old, new = "1" * 64, "2" * 64
    with pytest.raises(AssertionError):
        compare(Obj([("unrelated_digest", old)]), Obj([("unrelated_digest", new)]), moves=[])
    with pytest.raises(AssertionError):
        compare(Obj([("semantic_contract", Obj([("sha256", old)]))]), Obj([("semantic_contract", Obj([("sha256", new)]))]))
    sealed, content = {old: new}, {"3" * 64: "4" * 64}
    for name, path, pair in (
            ("x.json", ("semantic_contract", "sha256"), (old, new)),
            ("x.json", ("semantic_contract", "sha256"), ce.TABLE_SHA256),                 # the earlier stage's pair
            ("x.json", ("source_block_recovery", "body", "publication_sha256"), ("5" * 64, "6" * 64)),
            ("x.json", ("retained_precision", "receipt_sha256"), (old, "5" * 64)),
            ("x.json", ("result_envelope", "reproducibility", "raw_source_hashes", 0, "value"), ("3" * 64, "5" * 64)),
            ("x.json", ("package_checksum", "value"), (old, new))):
        with pytest.raises(AssertionError):
            verify_moves(name, [(path, *pair)], seals=sealed, content=content, unmaterialized=[])
    moved = []
    verify_moves("x.json", [(("semantic_contract", "sha256"), *TABLE_SHA256),
                            (("retained_precision", "receipt_sha256"), old, new),
                            (("result_envelope", "reproducibility", "raw_source_hashes", 0, "value"), "3" * 64, "4" * 64)],
                 seals=sealed, content=content, unmaterialized=moved)
    assert moved == []


def test_a_consistent_unrelated_digest_on_a_real_carrier_is_refused():
    """The constant-effort stage's counterexample at this endpoint: the dense connected load-state document's
    derivative hash replaced by an unrelated digest wherever the scope binds it is refused by every carrier binding
    it, although it moves one-to-one."""
    name = ce.CE_DOCUMENT
    digest = json.loads(current(name))["result_envelope"]["reproducibility"]["derivative_hash"]["value"]
    sites = {n for n in scope() if digest in current(n)}
    assert name in sites, sites

    def read(n):
        return current(n).replace(digest, "7" * 64) if n in sites else current(n)
    seals, content = recompute(read)
    for n in sorted(sites):
        moves = check(n, read(n))[1]
        assert any(after == "7" * 64 for _, _, after in moves), n
        with pytest.raises(AssertionError):
            verify_moves(n, moves, seals=seals, content=content, unmaterialized=[])


def test_nonlinear_report_paths_are_exact():
    name = "fixtures/reports/exact_route_limitations.json"
    good = Obj([("lists", [Obj([("limitations", [NEW_ID, "x", NEW_S7])])]), ("strings", [NEW_ID, NEW_S7])])
    restored, moved = _restore_report_statements(good, REPORT_PATHS[name])
    assert moved == 4 and restored == Obj([("lists", [Obj([("limitations", [OLD_ID, "x", OLD_S7])])]),
                                           ("strings", [OLD_ID, OLD_S7])])
    planted = Obj([("lists", [Obj([("producer", NEW_S7)])])])
    restored, moved = _restore_report_statements(planted, REPORT_PATHS[name])
    assert moved == 0
    with pytest.raises(AssertionError):
        compare(Obj([("lists", [Obj([("producer", OLD_S7)])])]), restored)


def test_nonlinear_texts_are_the_producers():
    source = current("core/product_physics/src/exact_admission.rs")
    base = _at(BASE, "core/product_physics/src/exact_admission.rs")
    for old, new in PAIRS.items():
        assert source.count(json.dumps(new, ensure_ascii=False)) == 1
        assert json.dumps(old, ensure_ascii=False) not in source
        assert base.count(json.dumps(old, ensure_ascii=False)) == 1


def test_a_stale_carrier_and_an_unnamed_pending_exception_are_refused():
    """A carrier left at the endpoint's bytes is refused (its old identity is not restated as the new one); and an
    ordinary carrier is never a pending retained exception."""
    stale = "fixtures/results/pressure_connected_mechanics_sparse.json"
    with pytest.raises(AssertionError):
        check(stale, _at(BASE, stale))
    assert not set(ORDINARY) & set(RETAINED_PENDING)
    assert CE2 in ORDINARY and CE2 not in ce.ORDINARY
