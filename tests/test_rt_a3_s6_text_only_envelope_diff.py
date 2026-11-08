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
or desktop-written (`DESKTOP_CARRIERS`, below). The load/reference-state schema is a contract change of S6, not a
carrier (tests/test_load_reference_schema.py).

`DESKTOP_CARRIERS` identifies the maintained desktop writers. Their comparisons are mandatory. The native replay
corpus is compared by request identity because its ordering is derived from request hashes; historical requests and
their held bytes retain their recorded bindings. Native byte counts are checked against their exact source bytes.

The endpoint is the working tree: the stage's carriers as committed on its branch. Freeze END at the stage's merge
when a later stage regenerates these carriers.

Final composition: S6's reviewed head (S6_HEAD) is merged with the accepted predecessor (PREDECESSOR, #164). BASE stays
the endpoint of every carrier and history obligation. The routing inventory applies S6's derivation to the
predecessor's inventory and attributes each moved value to its lineage. The partition attributes to the predecessor
only the files it changed, S6 did not, and the composition keeps byte-identical (INHERITED).
"""
from functools import cache
from pathlib import Path
import hashlib
import json
import subprocess

import pytest

from tests.test_o4_text_only_envelope_diff import HASH, Obj, Token, load, _embedded, _rt_a3_member_path
from tests.test_rt_a3_reducer_text_only_envelope_diff import (
    INPUT_BINDINGS, check_j7_envelope_digests_only, check_rv120_output_digests, _rv120_at)
from tests.test_rt_a3_reducer_2b_text_only_envelope_diff import _matches
from tests import test_rt_a3_constant_effort_text_only_envelope_diff as ce
from tests import test_rt_a3_nonlinear_text_only_envelope_diff as nl

ROOT = Path(__file__).resolve().parents[1]
BASE = "394f4c62313679ccf24e67236d80371a0bf62af3"
# The final composition merges S6's reviewed head into the accepted actual predecessor (#164). Carrier and history
# obligations keep BASE; only the routing inventory and the partition's inherited files are attributed through it.
S6_HEAD = "abc8bac62340a515d12d84392dd2ea105495aa2a"
PREDECESSOR = "00d4820d15f01bcada536c97b2bc129efd64575a"
# The original qualified composition remains a historical leg. Recovery merges the later main's
# governance/resource changes; current routing is re-derived on that actual input, not the old inventory.
QUALIFIED = "5b3f71fa5721546ffd93bb9c51b1ca4a56619b44"
CURRENT_MAIN = "8b02f8b9ec07a02e7427fb07107cb41e6bc2a2e0"
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
REPLAY_REQUESTS = nl.REPLAY_REQUESTS
REPLAY_OUTCOMES = "apps/desktop/src/test-support/native-replay/native_replay_outcomes.json"
PERSISTENCE_REPLIES = "apps/desktop/src/test-support/native-replay/native_persistence_replies.json"
DESKTOP_CARRIERS = {
    **{_W11 + f"pressure_2_{kind}.json": "W-11 exact-report request (exactReportBelowGate.test.tsx)"
       for kind in ("connector", "load_reference_states", "realized_bend", "straight")},
    "fixtures/reports/invented/user_release_exact/pressure_2_straight.json":
        "D-6 user-release vector (its desktop vitest writer)",
    **{f"{_DS}{mode}.{member}.json": f"desktop-shaped {member} (its desktop vitest writer)"
       for mode in ("dense", "sparse") for member in ("stress_neutral", "analysis_run")},
    REPLAY_REQUESTS: "C1 native replay corpus (its desktop vitest writer)",
}
DERIVED_CARRIERS = (ce.OVERLAY,)  # the W-11 writer's substitute hashes follow the restated genuine request
# Moved only by the table digests: byte for byte equal to the endpoint after substituting the two table pairs.
TABLE_ONLY = (
    "fixtures/results/base_reader_outcomes.json",
    "schemas/analysis_run.v0.3.schema.json",
    "schemas/stress_neutral_export.v0.3.schema.json",
)
SCHEMA_MANIFEST = "tools/release/schema_manifest.json"
ROUTING = "tools/hosted-ci-routing.json"
SCHEMA_MOVES = ("schemas/analysis_run.v0.3.schema.json", "schemas/load_reference_state.schema.json",
                "schemas/stress_neutral_export.v0.3.schema.json")
# Current generated consumers, checked byte-for-byte by their maintained writer tests on the same source.
# Their inputs remain under the S6 carrier checks; these are exact named derivations, not hex-hash exceptions.
GENERATED_ORACLES = {
    "apps/desktop/src/test-support/analysis-run-v03/analysis_run_v03_cases.json":
        ("apps/desktop/src/test-support/analysisRunV03Cases.ts", "apps/desktop/src/services/analysisRunV03Parity.test.ts"),
    "apps/desktop/src/test-support/analysis-run-v03/persistence_forms.json":
        ("apps/desktop/src/test-support/persistenceFormsCases.ts", "apps/desktop/src/services/persistenceForms.test.ts"),
    "apps/desktop/src/test-support/native-reopen/historical_reopen_cases.json":
        ("apps/desktop/src/test-support/historicalReopenCases.ts", "apps/desktop/src/services/historicalReopenParity.test.ts"),
}
PROVENANCE_SCOPE = "tools/release/provenance_scope.json"
NOT_CARRIERS = {
    ROUTING: "derived source-tree/read hashes and three exact source-scan additions (check_routing_inventory)",
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
    or name in (_W11 + "pressure_source_1.json",
                "fixtures/reports/invented/user_release_exact/pressure_source_1.json")))
UNMOVED = tuple(name for name in UNMOVED if name not in DESKTOP_CARRIERS)
ORDINARY = tuple(name for name in (*nl.ORDINARY, COMBINATION_CORPUS)
                 if name not in DESKTOP_CARRIERS and name not in UNMOVED and name not in DERIVED_CARRIERS)
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
    return (*ORDINARY, *DERIVED_CARRIERS, *(name for name in DESKTOP_CARRIERS if name != REPLAY_REQUESTS),
            *(name for name in RETAINED if name != READER_OUTCOMES))


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
    content.update(_report_record_pairs(read, seals | content))
    return ce._forward(seals, "seal"), ce._forward(content, "content")


def _report_record_pairs(read, known):
    """Re-materialize W-11's otherwise unstored AnalysisRun from its actual source/manifest with the maintained
    builder. Verify its current seal, then independently reverse the authorized metadata and re-seal: the resulting
    predecessor seal must equal the committed report reference. No new unmaterialized digest exception is admitted.
    This check needs the current-head runner, explicitly bound by the ordinary pytest setup."""
    from core.analysis_runs import build_analysis_run_v0_3
    from tests.verifiers.canonical_json import canonical_sha256
    reverse = ce._forward(((b, a) for a, b in {*TABLE_PAIRS, *known}), "report record reverse")
    reverse.update({b: a for a, b in PAIRS.items()})

    def restore(value):
        if isinstance(value, dict):
            return {k: restore(v) for k, v in value.items()}
        if isinstance(value, list):
            return list(map(restore, value))
        return reverse.get(value, value) if isinstance(value, str) else value

    subjects = []
    for kind in ("connector", "load_reference_states", "realized_bend", "straight"):
        name = _W11 + f"pressure_2_{kind}.json"
        before, after = (json.loads(t) for t in (_at(BASE, name), read(name)))
        subjects.append((before["request"], after["request"], after["input_manifest"], after["model"]))
    genuine = json.loads(read(_W11 + "pressure_2_straight.json"))
    subjects.append((ce.substitute(lambda n: _at(BASE, n)), ce.substitute(read),
                     genuine["input_manifest"], genuine["model"]))
    pairs = set()
    for old_request, request, manifest, model in subjects:
        manifest_hash = request["audit_manifest"]["input_manifest_hash"]
        assert canonical_sha256(manifest) == manifest_hash["value"]
        expected = ([{"object_type": "LoadCase", "ref": c["id"]} for c in model["load_cases"]]
                    + [{"object_type": "Combination", "ref": c["id"]} for c in model.get("combinations", [])])
        record = build_analysis_run_v0_3(
            request["source_results"][0], input_manifest=manifest,
            input_manifest_ref={"object_type": "InputManifest", "ref": manifest_hash["payload_ref"]},
            input_manifest_hash=manifest_hash["value"], expected_basis_refs=expected, hash_fn=canonical_sha256)
        claim = lambda r: r["report"]["calculation_report"]["audit_manifest_refs"][0]["checksum"]["value"]
        checksum = next(h["value"] for h in record["analysis_run"]["hashes"] if h["payload_scope"] == "analysis_run_record")
        assert checksum == claim(request)
        restored = restore(record)
        restored["analysis_run"]["hashes"] = [h for h in restored["analysis_run"]["hashes"]
                                              if h["payload_scope"] != "analysis_run_record"]
        old_checksum = canonical_sha256(restored)
        assert old_checksum == claim(old_request)
        pairs.add((old_checksum, checksum))
    return pairs


@cache
def recomputed():
    return recompute(current)


def verify_moves(name, moves, *, seals, content, allowed=None):
    """Every moved digest is bound to its subject: a table digest is a table pair; a seal or content digest equals the
    recomputation of the subject its old value identifies, with no fallback and no unmaterialized exception."""
    for path, before, after in moves:
        pair = (before.removeprefix("sha256:"), after.removeprefix("sha256:"))
        kind = (allowed or binding_kind)(path)
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


def _successful_reader_outcomes(source, readings):
    """Independently form outcome_json's successful payload from the subject publication and the Python verifier's
    numerical classification equations, without invoking the Rust reader or accepting a stored outcome hash.
    The surrounding preservation check establishes identical rows, statuses and all non-disclosure receipt data.
    Transport readings have no row classifications and make no invocation/eligibility claim."""
    from tests.verifiers import retained_precision as verifier
    body = source["retained_precision"]["body"]
    receipt = source["retained_precision"]
    assert verifier._hash("retained_precision_receipt_mp_v2", body) == receipt["receipt_sha256"]
    publication = body["publication_sha256"]
    if "results" in source:
        assert verifier._hash("retained_precision_publication_mp_v2", {k: v for k, v in source.items()
                                                                    if k != "retained_precision"}) == publication
    classes = []
    if any(name != "transport" and summary.startswith("pass") for name, (_, summary) in readings.items()):
        cases = body["cases"]
        owners = {c["basis_ref"]["ref_id"]: c for c in [*cases, *body["combinations"]]}
        all_rows = {key: [] for key in owners}
        for row in source["results"]:
            all_rows[row["basis_ref"]["ref_id"]].append(row)
        rows = {}
        for key, case in owners.items():
            ref = case.get("source_ref")
            distinct = verifier._distinct_members(body["sources"][ref]) if type(ref) is int else frozenset()
            rows[key] = verifier._bound_rows(all_rows[key], distinct)
        exact = source["producer"]["semantic_contract_id"] == verifier.EXACT_CONTRACT_ID
        classes = verifier._g5_numeric(body, rows, exact_evidence=(source.get("contract_evidence"),) if exact else None,
                                       all_rows_by_case=all_rows)
    out = {}
    for reading, (_, summary) in readings.items():
        if not summary.startswith("pass"):
            continue
        bound = reading == "bound"
        eligible = (bound and source["status"]["mechanics"] == "MECHANICS_SOLVED"
                    and all(c["status"] in {"selected", "not_required"} for c in body["cases"]))
        selected = [] if reading == "transport" else classes
        assert summary == f"pass{' eligible' if eligible else ''} rows={len(selected)}"
        out[reading] = {"ok": True, "validation": {"invocation_bound": bound, "numerical_eligible": eligible,
                                                 "publication_sha256": publication, "classifications": selected}}
    return out


def check_reader_outcomes(before, after):
    """Keys/readings/summaries stay fixed. Re-materialize moved inputs and every successful outcome payload at both
    endpoints, derive refusal hashes from their summaries, and preserve unmoved entries byte for byte."""
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
            successful = {}
            if key in inputs:
                source, invocation = inputs[key]()
                assert canonical_sha256([source, invocation]) == digest, (rev, key)
                if b[key][1] != a[key][1]:
                    successful = _successful_reader_outcomes(source, readings)
                    derived += 1
            for reading, (outcome, summary) in readings.items():
                if not summary.startswith("pass"):
                    assert nl._refusal_digest(summary) == outcome, (rev, key, reading)
                elif b[key][1] != a[key][1]:
                    assert canonical_sha256(successful[reading]) == outcome, ("successful outcome hash", rev, key, reading)
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

@pytest.mark.parametrize("name", [
    "fixtures/results/retained_precision_b2_parity_probes.json",
    "fixtures/results/retained_precision_combination_successor_sparse_interactive.json",
    "fixtures/results/retained_precision_combination_successor_dense_scrutiny.json",
])
def test_certificate_epoch_bridge_keeps_unchanged_probe_and_fixture_bytes(name):
    """The certificate proof ends at BASE; these detached inputs remain exact through current S6.

    The moved corpus and reader outcomes instead pass the per-carrier token/hash/summary checks below.
    """
    assert current(name) == _at(BASE, name), name


@pytest.mark.parametrize("name", sorted(GENERATED_ORACLES))
def test_generated_oracles_keep_the_qualified_writer_and_full_staleness_check(name):
    """The required desktop suite derives each complete file with the same writer and byte-for-byte check.

    The membership registration does not substitute for executing those checks on the current candidate.
    """
    for source in GENERATED_ORACLES[name]:
        assert current(source) == _at(QUALIFIED, source), source


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


S6_TREES = ("core/product_physics/Cargo.toml", "core/reporting/result_export/Cargo.toml",
            "core/solver/nonlinear_integration/Cargo.toml")
S6_READS = ("core/product_physics/src/source_receipt/pressure_source_context_tests.rs",
            "core/product_physics/tests/load_reference_state_runtime.rs",
            "core/product_physics/tests/pressure_source_join.rs",
            "core/reporting/result_export/tests/load_reference_contract.rs",
            "core/reporting/result_export/tests/retained_precision_contract.rs",
            "core/reporting/result_export/tests/retained_precision_derivative_golden.rs",
            "core/solver/nonlinear_integration/src/structural_adapter/k5_tests.rs")
S6_SCANS = (("core/reporting/result_export/tests/retained_precision_carriers.rs", "../../.."),
            ("core/solver/nonlinear_integration/src/s11k_tests.rs", "../../product_physics/src"),
            ("core/solver/nonlinear_integration/src/structural_adapter/k5_tests.rs", "../../product_physics/src"))


def check_routing_inventory(after=None):
    """Re-derive only the three changed Rust trees, seven source-read hashes and three scan additions on the accepted
    main's inventory (recovery composition: S6's own derivation applied to CURRENT_MAIN instead of BASE).
    Every source-scan rule, exclusion, caller, fixture enumeration and unrelated control retains its predecessor value.
    The exact scan contents are checked from the current filesystem, using the selector's declared boundaries."""
    import importlib.util
    module_path = ROOT / "tools/ci/numerical_ci.py"
    spec = importlib.util.spec_from_file_location("s6_routing_inventory", module_path)
    numerical = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(numerical)
    expected = json.loads(_at(CURRENT_MAIN, ROUTING))
    trees = expected["piping_runtime_reads"]["source_trees"]
    for manifest in S6_TREES:
        digest = numerical.rust_tree_digest(ROOT, Path(manifest))
        assert digest != trees[manifest]
        trees[manifest] = digest
    reads = expected["piping_runtime_reads"]["reads"]
    for source in S6_READS:
        digest = ce._sha(current(source))
        assert digest != reads[source]["sha256"]
        reads[source]["sha256"] = digest
    addition = "core/product_physics/src/rt_a3_s6_tests.rs"
    for source, anchor in S6_SCANS:
        entry = next(e for e in reads[source]["inputs"] if e["anchor"] == anchor and e.get("scan"))
        scan = entry["scan"]
        assert entry["base"] == "manifest" and scan["suffix"] == ".rs"
        crate = next(p for p in (ROOT / source).parents if (p / "Cargo.toml").is_file())
        boundary = (crate / anchor).resolve()
        excluded = set(scan.get("exclude", []))
        actual = {p.relative_to(ROOT).as_posix() for p in boundary.rglob("*")
                  if p.is_file() and p.suffix == scan["suffix"]
                  and not excluded.intersection(p.relative_to(boundary).parts)
                  and (not scan.get("hidden") or not any(part.startswith(".") for part in p.relative_to(boundary).parts))}
        assert actual - set(entry["files"]) == {addition} and not set(entry["files"]) - actual
        entry["files"] = sorted(actual)
    assert expected == json.loads(current(ROUTING) if after is None else after), "routing inventory changed outside its derivation"


def test_routing_inventory_derives_only_s6_source_bindings_and_scan_additions():
    check_routing_inventory()


def _routing_leaves(rev):
    def walk(node, path):
        if isinstance(node, dict):
            for key, value in node.items():
                yield from walk(value, path + (key,))
        elif isinstance(node, list) and any(isinstance(v, (dict, list)) for v in node):
            for index, value in enumerate(node):
                yield from walk(value, path + (index,))
        else:
            yield path, node
    return dict(walk(json.loads(_at(rev, ROUTING) if rev else current(ROUTING)), ()))


def test_routing_composition_attributes_every_moved_value_to_its_lineage():
    """S6's reviewed routing change (BASE -> S6_HEAD) touches exactly the values its derivation names; the accepted
    predecessor's (BASE -> PREDECESSOR) values are kept where S6 does not touch them, S6's reviewed values where the
    predecessor does not, and only the two values both lineages touch are re-derived on the combined source."""
    base, predecessor, s6, now = (_routing_leaves(rev) for rev in (BASE, PREDECESSOR, S6_HEAD, QUALIFIED))
    moved = lambda doc: {k for k in base.keys() | doc.keys() if base.get(k) != doc.get(k)}  # noqa: E731
    by_predecessor, by_s6 = moved(predecessor), moved(s6)
    reads = ("piping_runtime_reads", "reads")
    derived = ({("piping_runtime_reads", "source_trees", m) for m in S6_TREES}
               | {reads + (source, "sha256") for source in S6_READS})
    scanned = {reads + (source, "inputs") for source, _ in S6_SCANS}
    scan_moves = by_s6 - derived
    assert derived <= by_s6 and len(scan_moves) == len(S6_SCANS)
    assert {k[:4] for k in scan_moves} == scanned and all(k[-1] == "files" for k in scan_moves)
    assert moved(now) == by_predecessor | by_s6
    for key in by_predecessor - by_s6:
        assert now[key] == predecessor[key], key
    for key in by_s6 - by_predecessor:
        assert now[key] == s6[key], key
    assert by_predecessor & by_s6 == {("piping_runtime_reads", "source_trees", S6_TREES[0]),
                                      next(k for k in scan_moves if k[2] == S6_SCANS[0][0])}


@pytest.mark.parametrize("mutation", ["tree_hash", "scan_exclusion", "caller", "predecessor_value", "predecessor_scan",
                                     "main_fixture", "main_routing_rule"])
def test_routing_inventory_refuses_unbound_hashes_weakened_scans_or_lost_callers(mutation):
    doc = json.loads(current(ROUTING))
    reads = doc["piping_runtime_reads"]
    base = json.loads(_at(BASE, ROUTING))["piping_runtime_reads"]
    if mutation == "predecessor_value":  # the composition must keep the predecessor's moved value, not BASE's
        parity = "core/handoff/caepipe_mbf_engine/tests/parity.rs"
        assert reads["reads"][parity]["sha256"] != base["reads"][parity]["sha256"]
        reads["reads"][parity]["sha256"] = base["reads"][parity]["sha256"]
    elif mutation == "main_fixture":
        entries = [e for declaration in reads["reads"].values() for e in declaration["inputs"]
                   if "fixtures/embedded_agent/agent-witness-pin.json" in e["files"]]
        assert len(entries) == 1
        entries[0]["files"].remove("fixtures/embedded_agent/agent-witness-pin.json")
    elif mutation == "main_routing_rule":
        doc["path_rules"].pop(0)
    elif mutation == "predecessor_scan":  # dropping the predecessor's support.rs from the shared complete scan
        entry = next(e for e in reads["reads"][S6_SCANS[0][0]]["inputs"] if e["anchor"] == S6_SCANS[0][1] and e.get("scan"))
        entry["files"].remove("core/solver/modal_eigen/src/support.rs")
    elif mutation == "tree_hash":
        reads["source_trees"]["core/product_physics/Cargo.toml"] = "0" * 64
    elif mutation == "caller":
        reads["reads"]["core/product_physics/tests/pressure_source_join.rs"]["source_trees"].pop()
    else:
        entry = next(e for e in reads["reads"]["core/solver/nonlinear_integration/src/s11k_tests.rs"]["inputs"] if e.get("scan"))
        entry["scan"].setdefault("exclude", []).append("src")
    with pytest.raises(AssertionError, match="routing inventory changed outside its derivation"):
        check_routing_inventory(json.dumps(doc))


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


# ---------------------------------------------------------------- desktop carriers and native replay


def _stale(text):
    """States an endpoint text or binds an endpoint table digest."""
    return _states_old(text) or any(old in text for old, _ in TABLE_PAIRS)


@pytest.mark.parametrize("name", sorted(DESKTOP_CARRIERS))
def test_desktop_carriers_restate_only_the_texts_and_derived_bindings(name):
    if name == REPLAY_REQUESTS:
        check_replay_requests()
        return
    assert not _stale(current(name)), name
    _, moves = check(name)
    seals, content = recomputed()
    verify_moves(name, moves, seals=seals, content=content)


def test_w11_substitute_overlay_preserves_its_edits_and_rebinds_only_derived_hashes():
    before, after = (json.loads(t) for t in (_at(BASE, ce.OVERLAY), current(ce.OVERLAY)))
    pointers = [r["pointer"] for r in after["replacements"]]
    assert [r["pointer"] for r in before["replacements"]] == pointers

    def allowed(path):
        if len(path) >= 3 and path[0] == "replacements" and type(path[1]) is int and path[2] == "value":
            return binding_kind((*ce._tokens(pointers[path[1]]), *path[3:]))
        return binding_kind(path)

    moves = []
    compare(load(_at(BASE, ce.OVERLAY)), load(current(ce.OVERLAY)), moves=moves, allowed=allowed)
    seals, content = recomputed()
    verify_moves(ce.OVERLAY, moves, seals=seals, content=content, allowed=allowed)


def _js_ordered(value):
    """JSON.stringify for these persisted request objects: retain member order and use the independent ECMA number
    formatter. Every predecessor key is checked as well as every current key, so no new serializer premise is hidden."""
    from tests.verifiers.canonical_json import _string, ecma_number
    if isinstance(value, dict):
        return "{" + ",".join(_string(k) + ":" + _js_ordered(v) for k, v in value.items()) + "}"
    if isinstance(value, list):
        return "[" + ",".join(map(_js_ordered, value)) + "]"
    if type(value) in (int, float):
        return ecma_number(float(value))
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"))


def _replay_key(request):
    text = request["source_sha256"] + "\0" + request["command"] + "\0" + _js_ordered(request["args"])
    if "recorded_table" in request:
        text += "\0" + _js_ordered(request["recorded_table"])
    return ce._sha(text)


def check_replay_requests(read=current):
    """Preserve every predecessor request and historical binding. Complete the pre-existing S-K1 presence lookup
    beside each recorded intensified-kind lookup, on that same receiver. Only source metadata/derived keys change;
    sorting by keys may reorder the index, never an authored/member/result array."""
    before = json.loads(_at(BASE, REPLAY_REQUESTS))
    after = json.loads(read(REPLAY_REQUESTS))
    assert {k: v for k, v in before.items() if k not in {"sources", "requests"}} == {
        k: v for k, v in after.items() if k not in {"sources", "requests"}}
    seals, content = recomputed()
    replacements = {**dict(TABLE_PAIRS), **seals, **content, **PAIRS}
    actual = {r["key"]: r for r in after["requests"]}
    assert len(actual) == len(after["requests"])
    assert list(actual) == sorted(actual)
    panel = "apps/desktop/src/features/results/NativeResultsPanel.tsx"
    assert read(panel) == _at(BASE, panel)  # these paired lookups predate S6; no new product caller is introduced
    for kind in ("PREVIEW_INTENSIFIED_KIND", "PREVIEW_ALONG_DIRECTION_KIND"):
        assert _at(BASE, panel).count(f"useNativeKindPresence(result, {kind})") == 1
    expected_sources, mapped, resolved, probes = {}, {}, {}, {}
    for request in before["requests"]:
        assert request["key"] == _replay_key(request)
        historical = bool(request.get("recorded_table"))
        subject = (request.get("source_path"), request["source_sha256"], historical)
        if subject not in resolved:
            if "source_path" in request:
                old = _at(BASE, request["source_path"])
                new = read(request["source_path"])
            else:
                old = before["sources"][request["source_sha256"]]
                new = old
                if not historical:
                    for a, b in replacements.items():
                        new = new.replace(a, b)
                expected_sources[ce._sha(new)] = new
            assert ce._sha(old) == request["source_sha256"]
            if historical:
                assert new == old
            else:
                assert not _states_old(new), request["key"]
                if old != new:
                    moves = []
                    compare(load(old), load(new), moves=moves)
                    verify_moves(REPLAY_REQUESTS, moves, seals=seals, content=content)
            resolved[subject] = old, new
        old, new = resolved[subject]
        expected = {**request, "source_sha256": ce._sha(new)}
        expected["key"] = _replay_key(expected)
        assert expected["key"] in actual, (request["key"], expected["key"])
        assert actual[expected["key"]] == expected, request["key"]
        mapped[request["key"]] = (expected["key"], old, new)
        if request["command"] == "lookup_preview_mechanics_job_rows" and request["args"] == {
                "jobId": "$JOB", "kinds": ["preview_intensified_bending_stress"], "match": "first"}:
            probe = {**expected, "args": {"jobId": "$JOB", "kinds": ["support_reaction_along_direction_v1"], "match": "first"}}
            probe["key"] = _replay_key(probe)
            assert actual.get(probe["key"]) == probe
            probes[probe["key"]] = (probe, new)
    assert len({v[0] for v in mapped.values()}) == len(before["requests"])
    assert {v[0] for v in mapped.values()} | set(probes) == set(actual)
    assert after["sources"] == expected_sources
    nl.check_replay_history(after, read=read)
    # This S6-old source is genuine recorded history in addition to the earlier guard's pre-S5 history.
    history = "9f980d9a04e113fcb6221bf5ee4ac442220117384924671ecefd1c1aa97e268e"
    carrier = ("fixtures/persistence/recorded_pre_rt_a4_pf/physics_retained_1/main_252bef969/"
               "retained_precision_exact_successor_sparse_interactive.json")
    table_path = "fixtures/results/recorded_pre_rt_a4/semantic_contract_v0_3_physics_retained_1.85c2678a.json"
    assert json.loads(after["sources"][history]) == json.loads(read(carrier))["source"]
    table = nl._record_binding(json.loads(read(carrier.removesuffix(".json") + ".analysis_run.json")))
    assert table["sha256"] == ce._sha(read(table_path))
    holding = [r for r in after["requests"] if r["source_sha256"] == history]
    assert holding and all(r.get("recorded_table") == table and "source_path" not in r for r in holding)
    return mapped, probes


def _native_payload(text):
    """The two maintained native wire formats have a u64 header length; summary planes are 8-byte aligned.
    Preserve the remaining bytes exactly (numeric planes and detail text spool), including zero padding."""
    import base64
    raw = base64.b64decode(text, validate=True)
    if raw[:8] not in (b"SWBSUMv1", b"SWBDETv1"):
        return raw.decode("utf-8"), b"", b""
    size = int.from_bytes(raw[8:16], "little")
    end = 16 + size
    assert end <= len(raw)
    tail = (end + 7) // 8 * 8 if raw[:8] == b"SWBSUMv1" else end
    assert raw[end:tail] == b"\0" * (tail - end)
    return raw[16:end].decode("utf-8"), raw[tail:], raw[:8]


def _full_native_record(header, request, source, current_source):
    """The native preview transports a header, not the complete sealed record. Restore its documented row-scale
    omissions with the maintained full builder on the same source/manifest. S6 leaves these source rows/diagnostics
    and the table's row definitions unchanged; bind each restored reference to this endpoint's declared table.
    Re-sealing must reproduce the native full-record claim, at both endpoints."""
    from core.analysis_runs import build_analysis_run_v0_3
    from tests.verifiers.canonical_json import canonical_sha256
    assert source["results"] == current_source["results"]
    assert source.get("diagnostics") == current_source.get("diagnostics")
    args = request["args"]
    evidence = args["inputManifest"]
    manifest = json.loads(evidence["canonical_bytes"])
    record = json.loads(json.dumps(header["record"]))
    template = build_analysis_run_v0_3(
        current_source, input_manifest=manifest, input_manifest_ref=evidence["manifest_ref"],
        input_manifest_hash=evidence["manifest_sha256"], expected_basis_refs=record["analysis_run"]["load_basis_refs"],
        rule_check_status=header.get("rule_check_status"), rule_pack_ref_exclusions=args.get("rulePackExclusions") or (),
        hash_fn=canonical_sha256)
    omitted = {"result_refs", "diagnostics", "contract_evidence", "source_block_recovery", "retained_precision"}
    assert not omitted.intersection(record["analysis_run"])
    for name in omitted.intersection(template["analysis_run"]):
        record["analysis_run"][name] = template["analysis_run"][name]
    binding = record["analysis_run"]["reproducibility"]["semantic_contract"]
    for row in record["analysis_run"]["result_refs"]:
        row["semantic_contract"].update(binding)
    for name in ("contract_evidence", "source_block_recovery", "retained_precision"):
        if name in record["analysis_run"]:
            record["analysis_run"][name] = source[name]
    projection = json.loads(json.dumps(record))
    projection["analysis_run"]["hashes"] = [h for h in projection["analysis_run"]["hashes"]
                                           if h["payload_scope"] != "analysis_run_record"]
    claim = next(h["value"] for h in record["analysis_run"]["hashes"] if h["payload_scope"] == "analysis_run_record")
    assert canonical_sha256(projection) == claim, "native full-record materialization"
    return record


def check_replay_outcomes(read=current):
    mapped, probes = check_replay_requests(read)
    before, after = (json.loads(t) for t in (_at(BASE, REPLAY_OUTCOMES), read(REPLAY_OUTCOMES)))
    old = {o["key"]: o for o in before["outcomes"]}
    new = {o["key"]: o for o in after["outcomes"]}
    assert len(old) == len(mapped) and len(new) == len(after["outcomes"]) == len(mapped) + len(probes)
    assert list(new) == sorted(new)
    assert {k: v for k, v in before.items() if k != "outcomes"} == {
        k: v for k, v in after.items() if k != "outcomes"}
    seals, content = recomputed()
    requests = {r["key"]: r for r in json.loads(read(REPLAY_REQUESTS))["requests"]}
    for key, (changed_key, old_source, source) in mapped.items():
        a, b = old[key], new[changed_key]
        assert a["command"] == b["command"]
        assert a["source_sha256"] == ce._sha(old_source)
        assert b["source_sha256"] == ce._sha(source)
        x, y = a["reply"], b["reply"]
        assert set(x) == set(y) and x["ok"] == y["ok"]
        if x == y:
            continue
        if x.get("body") == "raw":
            assert y["body"] == "raw"
            p, planes, magic = _native_payload(x["base64"])
            q, changed_planes, changed_magic = _native_payload(y["base64"])
            assert (planes, magic) == (changed_planes, changed_magic), f"native wire planes changed: {key}"
        else:
            assert x.get("body") == y.get("body")
            p, q = json.dumps(x, ensure_ascii=False), json.dumps(y, ensure_ascii=False)
        if requests[changed_key].get("recorded_table"):
            # The held text and recorded-table receiver are immutable. The current dispatch and diagnostics
            # presentation still name the current registry table (native_verdict.rs and diagnostics_presentation.rs).
            # Exactly those two report bindings may follow the table; no historical result text/seal is restated.
            def historical(left, right, path=()):
                assert type(left) is type(right), path
                if left == right:
                    return
                if isinstance(left, Obj):
                    assert [k for k, _ in left] == [k for k, _ in right], path
                    for (key, a), (_, b) in zip(left, right):
                        historical(a, b, path + (key,))
                elif isinstance(left, list):
                    assert len(left) == len(right), path
                    for i, (a, b) in enumerate(zip(left, right)):
                        historical(a, b, path + (i,))
                else:
                    assert path in {("value", "binding", "sha256"),
                                    ("diagnostics_presentation", "binding", "reader_table", "sha256")}, path
                    assert (left, right) in TABLE_PAIRS, path
            historical(load(p), load(q))
            continue
        moves = []
        # A route header binds the registry table directly at binding.sha256.
        value = x.get("value")
        header = (a["command"] == "read_preview_mechanics_job_verdict" and x.get("body") == "json"
                  and isinstance(value, dict) and isinstance(value.get("binding"), dict))
        allowed = lambda path: ("table" if (header and path == ("value", "binding", "sha256"))
                                or path == ("diagnostics_presentation", "binding", "reader_table", "sha256")
                                else binding_kind(path))
        left, right = load(p), load(q)
        presentation = dict(left).get("diagnostics_presentation") if isinstance(left, Obj) else None
        if isinstance(presentation, Obj):
            old_binding = dict(dict(presentation)["binding"])
            new_binding_tokens = dict(dict(right)["diagnostics_presentation"])["binding"]
            new_binding = dict(new_binding_tokens)
            # diagnostics_presentation.rs binds the held UTF-8 text, not a numerical result quantity.
            for binding, source_text in ((old_binding, old_source), (new_binding, source)):
                assert isinstance(binding["result_bytes"], Token) and int(binding["result_bytes"]) == len(source_text.encode())
                assert binding["source_sha256"] == ce._sha(source_text)
            for i, (field, _) in enumerate(new_binding_tokens):
                if field in {"result_bytes", "source_sha256"}:
                    new_binding_tokens[i] = (field, old_binding[field])
        compare(left, right, moves=moves, allowed=allowed)
        # Native AnalysisRun replies carry their own sealed record; independently derive that subject too.
        own_seals, own_content = ce.derived_pairs(ce._plain(p), ce._plain(q))
        if a["command"] == "read_preview_mechanics_job_analysis_run_preview" and p != q:
            old_header, new_header = json.loads(p), json.loads(q)
            if isinstance(old_header.get("record"), dict) and isinstance(new_header.get("record"), dict):
                current_source = json.loads(source)
                old_record = _full_native_record(old_header, requests[changed_key], json.loads(old_source), current_source)
                new_record = _full_native_record(new_header, requests[changed_key], current_source, current_source)
                extra_seals, extra_content = ce.derived_pairs(old_record, new_record)
                own_seals |= extra_seals
                own_content |= extra_content
        verify_moves(REPLAY_OUTCOMES, moves, seals=ce._forward({*seals.items(), *own_seals}, "native seal"),
                     content=ce._forward({*content.items(), *own_content}, "native content"), allowed=allowed)
    for key, (request, source) in probes.items():
        outcome = new[key]
        assert outcome["command"] == request["command"] and outcome["source_sha256"] == ce._sha(source)
        reply = outcome["reply"]
        assert set(reply) == {"ok", "body", "base64"} and reply["ok"] is True and reply["body"] == "raw"
        text, tail, magic = _native_payload(reply["base64"])
        assert magic == b"SWBDETv1" and tail == b""
        rows = json.loads(source)["results"]
        assert not any(r.get("kind") == "support_reaction_along_direction_v1" for r in rows)
        assert json.loads(text) == {
            "document_kind": "swbpipe.desktop.result_detail_lookup", "version": 1,
            "job_id": "restored-stored-result-1" if request.get("recorded_table") else "backend-solve-job-1",
            "row_count": len(rows), "match": "first", "ids": [],
            "kinds": [{"kind": "support_reaction_along_direction_v1", "row": None}], "ordinals": [], "lengths": []}


def test_native_replay_replies_preserve_the_actual_commands_results():
    check_replay_outcomes()


def check_native_persistence(read=current):
    """Five unchanged store scenarios. Derive transient/recorded seals and the persisted project hash from their
    actual source and manifest; sizes follow the source's UTF-8 change. The small-store test keeps its same one-byte
    boundary; no product limit or mechanical value is moved."""
    from core.analysis_runs.compatibility import recorded_revision
    from tests.verifiers.canonical_json import canonical_sha256
    import re
    before, after = (json.loads(t) for t in (_at(BASE, PERSISTENCE_REPLIES), read(PERSISTENCE_REPLIES)))
    assert before["source_path"] == after["source_path"]
    path = before["source_path"]
    texts = [_at(BASE, path), read(path)]
    delta = len(texts[1].encode()) - len(texts[0].encode())
    # SaveRun's Stored writer normalizes JSON numbers and whitespace. Only the authorized ASCII statement moves,
    # so that stored form changes by the same count; the predecessor transcript supplies the preserved size basis.
    assert len(_js_ordered(json.loads(texts[1])).encode()) - len(_js_ordered(json.loads(texts[0])).encode()) == delta
    documents, records = [before, after], []
    pairs, ids = set(), {}

    def claim(record, scope):
        return next(h["value"] for h in record["analysis_run"]["hashes"] if h["payload_scope"] == scope)

    def record_digest(record):
        projection = json.loads(json.dumps(record))
        projection["analysis_run"]["hashes"] = [h for h in projection["analysis_run"]["hashes"]
                                               if h["payload_scope"] != "analysis_run_record"]
        return canonical_sha256(projection)

    for index, doc in enumerate(documents):
        get = (lambda name: _at(BASE, name)) if index == 0 else read
        requests = json.loads(get(REPLAY_REQUESTS))["requests"]
        outcomes = {o["key"]: o for o in json.loads(get(REPLAY_OUTCOMES))["outcomes"]}
        args = doc["scenarios"][0]["args"]
        selected = [r for r in requests if r["command"] == "read_preview_mechanics_job_analysis_run_preview"
                    and r.get("source_path") == path and r["args"].get("ruleCheckAggregate") is None
                    and r["args"]["inputManifest"] == args["inputManifest"]]
        assert len(selected) == 1
        body, tail, magic = _native_payload(outcomes[selected[0]["key"]]["reply"]["base64"])
        assert tail == magic == b""
        record = _full_native_record(json.loads(body), selected[0], json.loads(texts[index]), json.loads(texts[1]))
        assert record_digest(record) == claim(record, "analysis_run_record")
        assert canonical_sha256(json.loads(texts[index])) == claim(record, "received_result")
        assert args["expectedRecord"] == {"run_id": record["analysis_run"]["run_id"],
                                          "record_sha256": claim(record, "analysis_run_record"),
                                          "received_sha256": claim(record, "received_result")}
        payload = {"model": args["model"], "editor_intents": args["editorIntents"], "proposal": args["proposal"],
                   "selected_review_target": args["selectedReviewTarget"], "model_hash": args["modelHash"],
                   "mechanics_result": json.loads(texts[index]), "analysis_run": record}
        project_hash = canonical_sha256(payload)
        assert doc["scenarios"][0]["reply"]["value"]["project_envelope_hash"]["value"] == "sha256:" + project_hash
        records.append((record, project_hash))
    for scope in ("analysis_run_record", "received_result"):
        pairs.add((claim(records[0][0], scope), claim(records[1][0], scope)))
    pairs.add((records[0][1], records[1][1]))
    for i in (1, 2):
        sums = []
        for index, doc in enumerate(documents):
            summary = doc["scenarios"][i]["reply"]["value"]["run"]
            original = before["scenarios"][i]["reply"]["value"]["run"]
            assert (summary["state_id"], summary["created_at"]) == (original["state_id"], original["created_at"])
            manifest = json.loads(doc["scenarios"][0]["args"]["inputManifest"]["canonical_bytes"])
            revised = recorded_revision(records[index][0], state_id=summary["state_id"], created_at=summary["created_at"],
                                        input_manifest=manifest, hash_fn=canonical_sha256)
            digest = record_digest(revised)
            assert digest == summary["run_record_hash"] and summary["recording_id"] == "runrec:" + digest[:32]
            sums.append(digest)
        pairs.add(tuple(sums))
        ids["runrec:" + sums[0][:32]] = "runrec:" + sums[1][:32]
    counts = {("scenarios", i, "sqlite_length_limit") for i in (3, 4)} | {
        ("scenarios", 0, "reply", "value", "result", "result_bytes"),
        *(('scenarios', 3, 'reply', 'value', 'result', key) for key in ('result_bytes', 'row_bytes', 'limit_bytes')),
        ("scenarios", 3, "reply", "value", "result", "refused", "bytes")}
    hash_tails = (("expectedRecord", "received_sha256"), ("expectedRecord", "record_sha256"),
                  ("project_envelope_hash", "value"), ("summary", "persisted_project_envelope_hash_ref"),
                  ("request", "expected_base"), ("run", "run_record_hash"), ("run", "result_hash"))

    def walk(a, b, p=()):
        assert type(a) is type(b), p
        if isinstance(a, dict):
            assert list(a) == list(b), p
            for k in a:
                walk(a[k], b[k], p + (k,))
        elif isinstance(a, list):
            assert len(a) == len(b), p
            for i, (x, y) in enumerate(zip(a, b)):
                walk(x, y, p + (i,))
        elif a != b:
            if p in counts:
                assert type(a) is int and b == a + delta, ("native persistence byte count", p)
            elif type(a) is str and HASH.fullmatch(a) and HASH.fullmatch(b):
                assert any(p[-len(t):] == t for t in hash_tails), p
                assert (a.removeprefix("sha256:"), b.removeprefix("sha256:")) in pairs, ("native persistence hash", p)
            elif p[-2:] == ("run", "recording_id"):
                assert ids.get(a) == b, p
            elif len(p) > 3 and p[2] == "reply" and p[-1] in {"message", "error"}:
                expected = a
                for old, new in ids.items():
                    expected = expected.replace(old, new)
                expected = re.sub(r"\b(result_bytes|row_bytes|limit_bytes)=(\d+)",
                                  lambda m: f"{m[1]}={int(m[2]) + delta}", expected)
                expected = re.sub(r"\b(row is |length limit of )(\d+)( bytes)",
                                  lambda m: f"{m[1]}{int(m[2]) + delta}{m[3]}", expected)
                assert expected == b, p
            else:
                raise AssertionError(p)

    walk(before, after)
    for doc in documents:
        result = doc["scenarios"][3]["reply"]["value"]["result"]
        assert result["row_bytes"] == result["refused"]["bytes"]
        assert doc["scenarios"][3]["sqlite_length_limit"] == result["row_bytes"] - 1
        assert doc["scenarios"][4]["sqlite_length_limit"] == result["row_bytes"] - 1


def test_native_persistence_rebinds_only_derived_identities_and_source_byte_counts():
    check_native_persistence()


@pytest.mark.parametrize("mutation", ["missing_request", "forged_key", "rewritten_history"])
def test_native_replay_refuses_lost_requests_forged_keys_and_resealed_history(mutation):
    document = json.loads(current(REPLAY_REQUESTS))
    if mutation == "missing_request":
        document["requests"].pop()
    elif mutation == "forged_key":
        document["requests"][0]["key"] = "0" * 64
    else:
        sha = "9f980d9a04e113fcb6221bf5ee4ac442220117384924671ecefd1c1aa97e268e"
        text = document["sources"].pop(sha)
        changed = nl._bump_first_number(text, '"results"')
        assert changed != text
        new_sha = ce._sha(changed)
        document["sources"][new_sha] = changed
        for request in document["requests"]:
            if request["source_sha256"] == sha:
                request["source_sha256"] = new_sha
                request["key"] = _replay_key(request)
        document["requests"].sort(key=lambda r: r["key"])
    with pytest.raises(AssertionError):
        check_replay_requests(lambda name: json.dumps(document) if name == REPLAY_REQUESTS else current(name))


def test_native_replay_refuses_an_edited_numeric_plane():
    import base64
    document = json.loads(current(REPLAY_OUTCOMES))
    reply = next(o["reply"] for o in document["outcomes"] if o["reply"].get("body") == "raw"
                 and base64.b64decode(o["reply"]["base64"]).startswith(b"SWBSUMv1"))
    raw = bytearray(base64.b64decode(reply["base64"]))
    raw[-1] ^= 1
    reply["base64"] = base64.b64encode(raw).decode()
    with pytest.raises(AssertionError, match="native wire planes changed"):
        check_replay_outcomes(lambda name: json.dumps(document) if name == REPLAY_OUTCOMES else current(name))


def _stating(rev):
    found = set()
    for needle in NEEDLES:
        args = ["git", "grep", "-l", "-F", "-e", needle] + ([rev] if rev else []) + ["--", "*.json", *OUTSIDE]
        out = subprocess.run(args, cwd=ROOT, text=True, capture_output=True).stdout
        found |= {line.split(":", 1)[1] if rev else line for line in out.splitlines()}
    return found


def _diff_names(*revs):
    out = subprocess.run(["git", "diff", "--name-only", *revs, "--", "*.json", *OUTSIDE], cwd=ROOT, text=True,
                         capture_output=True, check=True).stdout
    return set(out.split())


# JSON files the accepted predecessor (#164, the MBF import landing) changed from BASE. S6 changes none of them, and the
# composition keeps each byte-identical to the predecessor, so they are attributed to it rather than to this stage.
INHERITED = frozenset({"fixtures/caepipe_mbf/invented/desktop_import/import_replies.json",
                       "fixtures/caepipe_mbf/invented/desktop_import/import_replies.provenance.json"})
INHERITED_MAIN = frozenset({
    "fixtures/embedded_agent/agent-witness-pin.json",
    "software-workflow.json",
    "tools/ci/python_ci_timings.json",
    "tools/software_workflow/documentation-inputs.json",
    "tools/software_workflow/fixtures/library_adoption_paths.json",
    "tools/tools-test-routing.json",
})


def _inherited():
    by_predecessor = _diff_names(BASE, PREDECESSOR) - _diff_names(BASE, S6_HEAD)
    assert by_predecessor == INHERITED, sorted(by_predecessor ^ INHERITED)
    for name in by_predecessor:
        assert current(name) == _at(PREDECESSOR, name), name
    return by_predecessor


def _changed():
    return _diff_names(BASE) - _inherited() - _inherited_main()


def _inherited_main(read=current):
    changed = _diff_names(PREDECESSOR, CURRENT_MAIN) - {ROUTING}
    assert changed == INHERITED_MAIN, sorted(changed ^ INHERITED_MAIN)
    for name in changed:
        assert read(name) == _at(CURRENT_MAIN, name), name
    return changed


def test_the_inherited_files_are_the_predecessors_and_only_its():
    assert _inherited() == INHERITED
    assert _diff_names(BASE, PREDECESSOR) & _diff_names(BASE, S6_HEAD) == {ROUTING}


def test_later_main_json_is_preserved_without_expanding_s6_carrier_exceptions():
    assert _inherited_main() == INHERITED_MAIN


@pytest.mark.parametrize("name", sorted(INHERITED_MAIN))
def test_an_edited_main_json_input_is_refused(name):
    with pytest.raises(AssertionError, match=name):
        _inherited_main(lambda path: current(path) + " " if path == name else current(path))


def check_provenance_scope(text=None):
    text = current(PROVENANCE_SCOPE) if text is None else text
    before, after = (json.loads(value) for value in (_at(BASE, PROVENANCE_SCOPE), text))
    restored = json.loads(text)
    assert len(before["members"]) == len(after["members"])
    for old, new, original in zip(before["members"], after["members"], restored["members"], strict=True):
        assert old["path"] == new["path"]
        assert new["sha256"] == ce._sha(current(new["path"])), new["path"]
        original["sha256"] = old["sha256"]
    assert restored == before


def test_provenance_scope_keeps_membership_and_labels_and_derives_only_carrier_hashes():
    check_provenance_scope()


@pytest.mark.parametrize("change", ["member", "label", "hash"])
def test_an_undeclared_provenance_scope_change_is_refused(change):
    document = json.loads(current(PROVENANCE_SCOPE))
    if change == "member":
        document["members"].pop()
    elif change == "label":
        document["members"][0]["labels"].append(["invented", "undeclared"])
    else:
        document["members"][0]["sha256"] = "0" * 64
    with pytest.raises(AssertionError):
        check_provenance_scope(json.dumps(document))


def test_every_json_file_is_in_exactly_one_part():
    """The partition is exact: every changed JSON file outside frozen references, design records and recorded runs is
    compared, table-only, the manifest or a named non-carrier; every earlier-stage carrier is compared or unmoved;
    every file stating an old text is compared, including the replay corpus's explicitly bound history."""
    parts = [set(ORDINARY), set(RETAINED), {J7}, set(TABLE_ONLY), {SCHEMA_MANIFEST}, set(DESKTOP_CARRIERS), set(DERIVED_CARRIERS),
             {REPLAY_OUTCOMES, PERSISTENCE_REPLIES}, set(NOT_CARRIERS), set(UNMOVED), set(GENERATED_ORACLES),
             {PROVENANCE_SCOPE}]
    assert sum(map(len, parts)) == len(set().union(*parts)), "a file in two parts"
    changed = _changed()
    assert changed - set().union(*parts) == set(), sorted(changed - set().union(*parts))
    for name in UNMOVED:
        assert name not in changed and not _states_old(_at(BASE, name)), name
        assert not any(old in _at(BASE, name) for old, _ in TABLE_PAIRS), name
    assert set(nl.ORDINARY) | set(nl.RETAINED) <= set(ORDINARY) | set(RETAINED) | set(DESKTOP_CARRIERS) | set(DERIVED_CARRIERS) | set(UNMOVED)
    at_base = _stating(BASE)
    compared = set(ORDINARY) | set(RETAINED) | set(DESKTOP_CARRIERS) | {STATEMENTS}
    assert at_base <= compared, sorted(at_base - compared)
    assert _stating(None) <= {REPLAY_REQUESTS}, sorted(_stating(None) - {REPLAY_REQUESTS})


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
    """A stale ordinary or desktop carrier and an unrelated move in a table-only carrier are refused."""
    stale = "fixtures/results/pressure_load_state_connected_sparse.document.json"
    with pytest.raises(AssertionError):
        check(stale, _at(BASE, stale))
    name = TABLE_ONLY[0]
    after = current(name)
    assert _substitute_tables(_at(BASE, name)) == after
    assert _substitute_tables(_at(BASE, name)) != after.replace(TABLE_SHA256[1], "0" * 64, 1)
    desktop = _W11 + "pressure_2_straight.json"
    with pytest.raises(AssertionError):
        check(desktop, _substitute_tables(_at(BASE, desktop)))


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


@pytest.mark.parametrize("change", ["certificate", "adapter_count"])
def test_undeclared_current_certificate_changes_are_refused(change):
    from tests.retained_precision_corpus import _STORAGE

    name = "fixtures/results/retained_precision_cases.json"
    corpus = _STORAGE.expand(json.loads(current(name)))
    check(name, json.dumps(corpus))
    case = next(c for c in corpus["cases"] if c["id"] == "w1_member_combination_sparse_interactive")
    if change == "certificate":
        case["source"]["contract_evidence"]["combination_certificate"]["combinations"][0]["uncovered_row_count"] += 1
    else:
        case["source"]["retained_precision"]["body"]["product_attempts"][0]["adapter"]["counts"][4] += 1
    with pytest.raises(AssertionError):
        check(name, json.dumps(corpus))


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


def test_a_forged_successful_reader_outcome_hash_is_refused():
    document = json.loads(current(READER_OUTCOMES))
    entry = next(e for e in document["entries"] if e[0] == "corpus:case:w1_member_route_e_baseline_sparse_interactive")
    assert entry[2]["bound"][1].startswith("pass")
    entry[2]["bound"][0] = "7" * 64
    with pytest.raises(AssertionError, match="successful outcome hash"):
        check_reader_outcomes(_at(BASE, READER_OUTCOMES), json.dumps(document))


def test_a_single_reader_outcome_rolled_back_to_the_predecessor_is_refused():
    document = json.loads(current(READER_OUTCOMES))
    key = "corpus:case:w1_member_route_e_baseline_sparse_interactive"
    before = _at(BASE, READER_OUTCOMES)
    original = next(e for e in json.loads(before)["entries"] if e[0] == key)
    index = next(i for i, e in enumerate(document["entries"]) if e[0] == key)
    assert document["entries"][index][1] != original[1]
    document["entries"][index] = original
    with pytest.raises(AssertionError, match=key):
        check_reader_outcomes(before, json.dumps(document))


@pytest.mark.parametrize("change", ["summary", "added_reading", "removed_entry"])
def test_undeclared_current_reader_outcome_changes_are_refused(change):
    document = json.loads(current(READER_OUTCOMES))
    entry = next(e for e in document["entries"] if e[0] == "corpus:case:w1_member_route_e_baseline_sparse_interactive")
    if change == "summary":
        entry[2]["bound"][1] += " undeclared"
    elif change == "added_reading":
        entry[2]["undeclared"] = entry[2]["bound"]
    else:
        document["entries"].remove(entry)
    with pytest.raises(AssertionError):
        check_reader_outcomes(_at(BASE, READER_OUTCOMES), json.dumps(document))
