"""DEL-04-09 I3a (`execution/PKG-04/DEL-04-09/Work/modal-analysis.md` §5.3): the
independent Python modal reader (`tests/verifiers/modal_evidence.py`) over the
committed corpus (`fixtures/results/modal_corpus.json`) and fixtures
(`fixtures/results/modal_*.json`).

Verdict parity: the corpus states each entry's verdict once; the Rust reader is held
to it in `core/reporting/modal_result/tests/corpus.rs`, this reader here, so
the two agree on every closure, binding, relation, health, refusal and decision. All
values are invented.
"""
from __future__ import annotations

import ast
import json
from pathlib import Path

import pytest

from tests.verifiers import canonical_json
from tests.verifiers import modal_evidence as me

ROOT = Path(__file__).resolve().parents[1]
RESULTS = ROOT / "fixtures/results"
CORPUS = json.loads((RESULTS / "modal_corpus.json").read_text(encoding="utf-8"))["entries"]


def load(name):
    return json.loads((RESULTS / name).read_text(encoding="utf-8"))


def _parts(path):
    return [part.replace("~1", "/").replace("~0", "~") for part in path.split("/")[1:]]


def apply(doc, operations):
    """The corpus operations: replace, add, scale, scale_rows, withhold, remove_rows and
    set_rows."""
    for op in operations:
        kind = op["op"]
        if kind in ("replace", "add", "scale"):
            parts = _parts(op["path"])
            parent = doc
            for part in parts[:-1]:
                parent = parent[int(part)] if isinstance(parent, list) else parent[part]
            last = parts[-1]
            key = int(last) if isinstance(parent, list) else last
            if kind == "add":
                assert last not in parent, op
            elif not isinstance(parent, list):
                assert last in parent, op
            parent[key] = parent[key] * op["factor"] if kind == "scale" else op["value"]
        elif kind == "scale_rows":
            for row in doc["tables"][op["table"]]["rows"]:
                if all(row.get(k) == v for k, v in op["where"].items()):
                    row[op["field"]]["value"] = row[op["field"]]["value"] * op["factor"]
        elif kind == "withhold":
            doc["tables"][op["table"]] = {"status": "withheld", "reason": "corpus: payload withheld"}
        elif kind == "remove_rows":
            table = doc["tables"][op["table"]]
            kept = [row for row in table["rows"] if not all(row.get(k) == v for k, v in op["where"].items())]
            assert len(kept) < len(table["rows"]), op
            if "renumber_prefix" in op:
                for i, row in enumerate(kept):
                    row["ordinal"] = i
                    row["row_id"] = f"{op['renumber_prefix']}{i}"
            table["rows"] = kept
        elif kind == "set_rows":
            selected = [row for row in doc["tables"][op["table"]]["rows"] if all(row.get(k) == v for k, v in op["where"].items())]
            assert selected, op
            for row in selected:
                row[op["field"]] = op["value"]
        else:
            raise AssertionError(op)
    return doc


def _reseal_table(table):
    if table.get("status") != "published":
        return
    checksums = []
    for row in table["rows"]:
        body = {k: v for k, v in row.items() if k != "checksum"}
        row["checksum"] = canonical_json.canonical_sha256(body)
        checksums.append(row["checksum"])
    table["row_count"] = len(checksums)
    table["digest"] = canonical_json.canonical_sha256(checksums)


def reseal(doc):
    for table in (doc.get("tables") or {}).values():
        _reseal_table(table)
    if "refusals" in doc:
        _reseal_table(doc["refusals"])


def statuses(verdict):
    return {"closure": verdict["closure"]["status"], "binding": verdict["binding"]["status"],
            "relations": {k: v["status"] for k, v in verdict["relations"].items()}, "health": verdict["health"],
            "decision": verdict["decision"], "refusal_codes": verdict["refusal_codes"]}


def entry_text(entry):
    """The text an entry reads: raw replacements on the committed text, or the mutated
    value serialized."""
    raw = (RESULTS / entry["document"]).read_text(encoding="utf-8")
    if "text_operations" in entry:
        assert not entry["operations"] and not entry["reseal"]
        for replacement in entry["text_operations"]:
            assert replacement["find"] in raw, replacement
            raw = raw.replace(replacement["find"], replacement["replace"], 1)
        return raw
    doc = apply(json.loads(raw), entry["operations"])
    if entry["reseal"]:
        reseal(doc)
    return json.dumps(doc)


def run(entry):
    text = entry_text(entry)
    request = None
    if entry["request"] is not None:
        request = apply(load(entry["request"]), entry["request_operations"])
    verdict = me.read_text(text, request)
    return statuses(verdict), verdict


@pytest.mark.parametrize("entry", CORPUS, ids=[entry["id"] for entry in CORPUS])
def test_the_python_reader_returns_every_corpus_verdict(entry):
    got, verdict = run(entry)
    intent = entry["intent"]
    if intent == "none":
        assert got["decision"] in ("consistent", "consistent_with_unresolved", "consistent_with_scoped_unsupported",
                                   "refusal_consistent"), verdict
    elif intent == "scoped":
        assert got["decision"] in ("consistent_with_unresolved", "consistent_with_scoped_unsupported"), verdict
        assert all(status != "failed" for status in got["relations"].values()), verdict
    elif intent == "unresolved":
        # Left open by an operand's stated set: accepted, never passed.
        assert got["decision"] == "consistent_with_unresolved", verdict
        assert all(status != "failed" for status in got["relations"].values()), verdict
    elif intent in ("closure", "binding"):
        assert got[intent] == "failed" and got["decision"] == "rejected", verdict
    else:
        assert got["relations"][intent] == "failed" and got["decision"] == "rejected", verdict
    needle = entry.get("difference_contains")
    if needle:
        standing = verdict[intent] if intent in ("closure", "binding") else verdict["relations"][intent]
        assert needle.lower() in (standing["difference"] or "").lower(), verdict
    assert got == entry["expected"], verdict


def test_the_corpus_targets_every_relation():
    intents = {entry["intent"] for entry in CORPUS}
    assert {"none", "scoped", "unresolved", "closure", "binding", *me.RELATIONS} <= intents
    assert len(CORPUS) >= 50


def test_the_readers_semantic_table_is_the_committed_table():
    committed = load("semantic_contract_modal_1.json")
    assert me.semantic_table() == committed
    assert canonical_json.canonical_sha256(committed) == me.SEMANTICS_SHA256
    assert me.catalog_differences() == []


def test_every_solved_fixture_is_consistent_only_up_to_its_stated_scope():
    for path in sorted(RESULTS.glob("modal_*.request.json")):
        name = path.name.replace(".request.json", ".json")
        verdict = me.read(load(name), load(path.name))
        if verdict["decision"] == "refusal_consistent":
            assert verdict["refusal_codes"] and verdict["health"] == "not_assessed"
            continue
        assert verdict["decision"] in ("consistent_with_scoped_unsupported", "consistent_with_unresolved"), (name, verdict)
        assert verdict["validation"]["status"] == "not_fully_validated"
        units = verdict["relations"]["units"]
        # Every value has its catalog unit (Design §5.3.5); units passing closes no other mandatory
        # relation: the subject census (evidence) still leaves validation unmet.
        assert units["status"] == "passed" and not units.get("scope"), units
        assert verdict["validation"]["unmet"] == ["evidence"], verdict["validation"]


def test_the_equilibrated_basis_list_is_closed_and_states_every_such_rule():
    """Design §5.3.5: a kind whose rule is `equilibrated_basis` is in modal-1's closed list,
    and every listed kind has that rule; the trace leaves the list empty."""
    listed = {kind for kind, _, _, rules in me.KINDS
              if any(rule["unit_status"] == "equilibrated_basis" for rule in rules.values())}
    assert listed == set(me.EQUILIBRATED_BASIS_KINDS) == set()
    assert load("semantic_contract_modal_1.json")["equilibrated_basis_kinds"] == []
    # No rule states the former catalog_family_absent: every kind has its catalog unit.
    assert all(rule["unit_status"] == "catalog" for _, _, _, rules in me.KINDS for rule in rules.values())
    assert me.RULES["mass_formation_difference"] == me.RULES["reformed_mass_norm"] == {"fixed": me.TIME_SQUARED}


def test_the_reader_imports_only_the_standard_library_and_its_siblings():
    tree = ast.parse((ROOT / "tests/verifiers/modal_evidence.py").read_text(encoding="utf-8"))
    imported = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            imported |= {alias.name for alias in node.names}
        elif isinstance(node, ast.ImportFrom):
            imported.add(node.module or "")
    assert imported <= {"__future__", "fractions", "json", "math", "typing", "tests.verifiers"}, imported


def test_one_mass_entry_is_one_variable_in_its_excitation_sum():
    """Third re-check, finding 3: a free/free off-diagonal entry meets φᵀp_k in two rows.
    With coefficients +1 and −1 it is ONE term with coefficient exactly 0 (its allowance
    held to the coefficients' magnitudes), so with m = 2⁻¹⁰²² and Γ = 0.5 the relation
    Γ·m = φᵀp_k is contradicted even for a flagged (nonzero, below 2⁻¹⁰²²) entry."""
    free = [True] * 13
    phi = [0.0] * 13
    phi[6], phi[12] = 1.0, -1.0
    terms, counts = me._excitation_terms(free, [(6, 12)], phi)
    assert terms[0] == {0: [0.0, 2.0]} and counts[0] == 2
    flagged = me.Values.of(type("V", (), {"value": 0.0, "below": True, "undefined": None})(), signed=True)
    e = me.ALLOW * float(counts[0] + 6) * me.U
    s = me.Values([me.Piece(-e * 2.0, e * 2.0)]) * flagged
    gamma_m = me.Values.point(0.5) * me.Values.point(2.0 ** -1022)
    assert not gamma_m.widen(e).meets(s)
    # Two independent unknowns would admit it: the identity is what decides.
    independent = me._total([(me.Values.point(1.0) * flagged), (me.Values.point(-1.0) * flagged)])
    assert gamma_m.widen(e).meets(independent)


def test_a_flagged_value_is_nonzero_and_unbounded_below():
    """Third re-check, finding 1: the producer flags any nonzero value below 2⁻¹⁰²²
    (modal_eigen dd.rs:476), 2⁻²⁰⁰³ included. A product of nonzero values that underflows
    is never 0."""
    flagged = me.Values.of(type("V", (), {"value": 0.0, "below": True, "undefined": None})())
    assert not flagged.meets(me.Values.point(0.0))
    product = me.Values.point(3.37e-99) * me.Values.point(1e-300)  # ≈ 3.4·10⁻³⁹⁹
    assert product.as_point() is None and not product.meets(me.Values.point(0.0)) and product.meets(flagged)


EDGES = json.loads((RESULTS / "modal_producer_edges.json").read_text(encoding="utf-8"))


def test_the_producers_underflowing_generalized_mass_is_accepted_unresolved():
    """Class confirmation of cab0ea356, finding 1: the producer sums m's triple products
    exactly (`mass_bilinear`, modal_quantities.rs:294). Its probe (a unit rotation, a 2⁻⁶⁰⁰
    translation, mass 1 on that translation only) has m = 2⁻¹²⁰⁰, a flagged 0. The reader's
    own m reconstruction keeps the underflowing coefficient nonzero, so it accepts the
    probe as unresolved, where a binary64 coefficient would have dropped the term."""
    probe = EDGES["relation_probes"][0]
    shape = [0.0] * 12
    for dof, vo in zip(probe["global_dofs"], probe["shape"]):
        assert not vo["below_normal"]
        shape[dof] = vo["value"]
    (i, j, value), = probe["mass_entries"]
    assert (1.0 * shape[i]) * shape[j] == 0.0, "the binary64 coefficient underflows"
    term = me._generalized_mass_term(i, j, shape, me.Values.point(value))
    gm_vo = type("V", (), {"value": probe["generalized_mass"]["value"], "below": probe["generalized_mass"]["below_normal"],
                           "undefined": None})()
    scope = []
    me._generalized_mass_check(scope, 1, gm_vo, me.Values.of(gm_vo), [term], 1)
    assert scope and scope[0].startswith(me.UNRESOLVED), scope


def test_bessel_correlated_mode_witness():
    """Agent 0's Bessel contract: a witness that the re-formed M̂'s deviation d̂ bounds
    nothing on the assembled M. With M = I, M̂ = [[1, −1], [−1, 2]], φ₁ = (1, 0), φ₂ = (1, 1)
    and r = (1, 1), the modes are exactly M̂-orthogonal (d̂ = 0), yet on M
    Σ m_eff = (φ₁ᵀr)²/φ₁ᵀφ₁ + (φ₂ᵀr)²/φ₂ᵀφ₂ = 1 + 2 = 3 > T = rᵀr = 2. The corpus carries
    these two modes in a document with T_x = 3.5 (established: passed) and T_x = 2 (beyond
    the strict allowance: unresolved), each also with a wrong Γ²m (refused)."""
    def dot(a, b, m):
        return sum(a[i] * m[i][j] * b[j] for i in range(2) for j in range(2))
    M, M_hat, r = [[1, 0], [0, 1]], [[1, -1], [-1, 2]], (1, 1)
    phi = [(1, 0), (1, 1)]
    assert dot(phi[0], phi[1], M_hat) == 0 and dot(phi[0], phi[1], M) != 0
    total = sum(dot(p, r, M) ** 2 / dot(p, p, M) for p in phi)
    assert total == 3 and dot(r, r, M) == 2
    verdicts = {e["id"]: e["expected"] for e in CORPUS}
    assert verdicts["none:bessel_established_for_correlated_modes"]["relations"]["mass"] != "failed"
    assert verdicts["unresolved:bessel_beyond_the_strict_allowance_without_a_common_m_bound"]["relations"]["mass"] == "unresolved"
    for refused in ("participation:bessel_established_beside_a_wrong_gamma_squared",
                    "participation:bessel_unresolved_beside_a_wrong_gamma_squared"):
        assert verdicts[refused]["decision"] == "rejected" and verdicts[refused]["relations"]["mass"] != "failed"


# ---- review of f5c66b167: an exact Bessel comparison; two unconditional producer domains ----------
def test_bessel_comparison_is_exact_at_the_strict_boundary():
    """T = 1.3, effective masses [1.3000000000000094, 0]: exactly, the sum is 8.88e-17 above
    T·(1 + 16(n+2)u), so it is not established (a round-to-nearest comparison passed it)."""
    from fractions import Fraction
    t, n, highs = 1.3, 2, [1.3000000000000094, 0.0]
    excess = sum(Fraction(h) for h in highs) - Fraction(t) * (1 + Fraction(16) * (n + 2) * Fraction(2) ** -53)
    assert excess > 0
    assert not me.bessel_established(highs, t, n)
    assert me.bessel_established([0.6, 0.6], t, n) and me.bessel_established([1.3, 0.0], t, n)


def _mutated_component_body(operations):
    doc = load("modal_component_body.json")
    apply(doc, operations)
    reseal(doc)
    return statuses(me.read_text(json.dumps(doc), load("modal_component_body.request.json")))


def test_a_flagged_formation_separation_ratio_violates_its_producer_domain():
    base = "/tables/sturm_candidates/rows/0/formation_separation_ratio"
    got = _mutated_component_body([{"op": "replace", "path": f"{base}/value", "value": 0.0},
                                   {"op": "replace", "path": f"{base}/below_normal", "value": True}])
    assert got["closure"] == "passed" and got["relations"]["sturm"] == "failed" and got["decision"] == "rejected"


def test_the_identity_relative_domain_holds_whatever_the_tolerance():
    doc = load("modal_component_body.json")
    absolute = 1e-9
    doc["binding"]["integrity_parameters"]["identity_tolerance"]["absolute"] = absolute
    doc["binding"]["integrity_parameters_sha256"] = canonical_json.canonical_sha256(doc["binding"]["integrity_parameters"])
    apply(doc, [{"op": "replace", "path": "/tables/identity/rows/0/relative/value", "value": 0.0},
                {"op": "replace", "path": "/tables/identity/rows/0/relative/below_normal", "value": True}])
    reseal(doc)
    got = statuses(me.read_text(json.dumps(doc), None))
    assert got["relations"]["evidence"] == "failed", got
