"""RT-A3 §4 D3 (DEL-04-08 `retained-pricing-rules.md` §8.53): the retained carriers regenerated on the registered build.

On the registered build the W1 freeze admits the certificate's member (retained_product.rs preview_case_evidence), so
every retained successor with a combination carries the ordinary run's `contract_evidence.combination_certificate`
and its notices (DEL-05-02 Work `combination-certificate.md`, "Retained successors (for D3)"). The preserved endpoint
is BASE, the registration commit before the carriers were regenerated (§8.53's profile on the old carriers). Across
this regeneration only these may change:

- a successor (a corpus case or a W-CB3 fixture document) may gain the member, the certificate's notices
  (`diagnostic:combination-certificate:<n>:<combination>`, inserted among the diagnostics) and their ids wherever the
  receipt lists diagnostic ids; each product attempt's and operand preparation's adapter counts may rise by k times
  the freeze's three-key evidence check (k >= 0, one k per record; FREEZE_DELTA), and the publication and receipt
  seals and inner identities follow. Removing exactly those, restoring the counts and resealing by the shared corpus
  rule (`apply_mutation`, rehash "all") must reproduce BASE's source token for token;
- a corpus mutation, must-pass entry or B2 parity probe on a regenerated base may have its edit paths remapped to the
  same element of the new base (an index through an array that gained certificate elements) and a whole-array
  replacement keeps the new base's certificate elements; its expected readings never move;
- a corpus case's provenance digests (successor bytes, receipt, producer record) follow its source;
- the native reader outcomes: the same keys, every summary token for token, and an entry's digests move only when its
  input moved, and only for an input derived from a regenerated base.

The legacy W-CB3 successors stay byte for byte as `..._pre_certificate.json` (BASE's fixture bytes).
"""
from copy import deepcopy
from functools import cache
from pathlib import Path
import json
import subprocess

import pytest

from tests.retained_precision_corpus import _STORAGE, load_corpus
from tests.test_retained_precision_contract import apply_mutation
from tests.verifiers import retained_precision as rp

ROOT = Path(__file__).resolve().parents[1]
BASE = "dcb3fe0dca796837b6da1df16d046ebe749f017d"
PREFIX = "diagnostic:combination-certificate:"
# AdapterEvent order (retained_product.rs): SourceVisit, RowVisit, MapWrite, ValidationEntry, IdentityByteRead,
# KeyProbe, AllocationRequest, LibraryBoundary, RequestedCopyBytes, RustCapacityBytes. The member's key sorts first
# among the three and is compared with all three allowed names: RowVisit +1, ValidationEntry and KeyProbe +3 (`same`),
# IdentityByteRead +2 per byte of the one equal-length comparison (23 B).
FREEZE_DELTA = [0, 1, 0, 3, 46, 3, 0, 0, 0, 0]
CORPUS = "fixtures/results/retained_precision_cases.json"
PROBES = "fixtures/results/retained_precision_b2_parity_probes.json"
OUTCOMES = "fixtures/results/retained_precision_reader_outcomes.json"
MODES = ("sparse_interactive", "dense_scrutiny")
FIXTURES = {m: f"fixtures/results/retained_precision_combination_successor_{m}.json" for m in MODES}
MOVED_CASES = {f"{w}_{m}" for w in ("w_cb1", "w_cb2", "w_cb3", "w_cb4a", "w_cb4b", "w_cb5", "b2_c1_range_mechanics",
                                    "w1_member_combination") for m in MODES} | {
    "w_cb1z_sparse_interactive", "b2_operand_preparation_failure", "b2_operand_source_unavailable"}
HOOKED = {"b2_operand_preparation_failure", "b2_operand_source_unavailable"}   # hook_produced: the controlled G8 refusal


def _bytes_at(rev, name):
    return subprocess.check_output(["git", "show", f"{rev}:{name}"], cwd=ROOT)


@cache
def _corpus_at(rev):
    return _STORAGE.expand(json.loads(_bytes_at(rev, CORPUS)))


@cache
def _corpus_now():
    return load_corpus()


def tokens(value):
    """Exact JSON tokens: floats by repr, ints distinct from floats, object keys in order."""
    if isinstance(value, dict):
        return ("o", tuple((k, tokens(v)) for k, v in value.items()))
    if isinstance(value, list):
        return ("a", tuple(tokens(v) for v in value))
    if isinstance(value, float):
        return ("f", repr(value))
    return (type(value).__name__, value)


def _cert(x):
    return (isinstance(x, dict) and str(x.get("id", "")).startswith(PREFIX)) or (isinstance(x, str) and x.startswith(PREFIX))


def _without_ids(value, ids):
    if isinstance(value, dict):
        return {k: _without_ids(v, ids) for k, v in value.items()}
    if isinstance(value, list):
        return [_without_ids(v, ids) for v in value if not (isinstance(v, str) and v in ids)]
    return value


def restated(live, base):
    """`live` with the member, its notices and their ids removed and each adapter record's counts restored by
    k * FREEZE_DELTA (k >= 0), resealed; and the k list. A count moving otherwise, or no member, is refused."""
    s = deepcopy(live)
    ids = {d["id"] for d in s["diagnostics"] if _cert(d)}
    assert ids and "combination_certificate" in s["contract_evidence"], "the member and its notices"
    s["diagnostics"] = [d for d in s["diagnostics"] if d["id"] not in ids]
    s["contract_evidence"].pop("combination_certificate")
    s["retained_precision"] = _without_ids(s["retained_precision"], ids)
    ks = []
    for kind in ("product_attempts", "operand_preparations"):
        records = s["retained_precision"]["body"].get(kind) or []
        before = base["retained_precision"]["body"].get(kind) or []
        assert len(records) == len(before), kind
        for record, prior in zip(records, before):
            counts, old = record["adapter"]["counts"], prior["adapter"]["counts"]
            delta = [a - b for a, b in zip(counts, old)]
            k = delta[1]
            assert k >= 0 and delta == [k * d for d in FREEZE_DELTA], (kind, delta)
            ks.append(k)
            record["adapter"]["counts"] = list(old)
    return apply_mutation(s, {"id": "s853", "edits": [], "rehash": "all"}), ks


def _check_successor(label, live, base, invocation, hooked=False):
    resealed, ks = restated(live, base)
    assert tokens(resealed) == tokens(base), f"{label}: only the member, its notices, the freeze meter and the seals move"
    assert any(ks), label
    try:
        reading = rp.validate_retained_precision(deepcopy(live), deepcopy(invocation))
        outcome = ("pass", reading["invocation_bound"], reading["numerical_eligible"])
    except rp.RetainedPrecisionError as error:
        outcome = (error.gate, error.code)
    if hooked:
        assert outcome[0] == "G8", (label, outcome)
    else:
        assert outcome == ("pass", True, True), (label, outcome)


def test_the_regenerated_corpus_cases_are_exactly_the_declared_ones():
    before = {c["id"]: c for c in _corpus_at(BASE)["cases"]}
    after = {c["id"]: c for c in _corpus_now()["cases"]}
    assert set(before) == set(after)
    moved = {k for k in before if tokens(before[k]["source"]) != tokens(after[k]["source"])}
    assert moved == MOVED_CASES
    for k in before:
        b, a = before[k], after[k]
        assert tokens({x: y for x, y in a.items() if x not in ("source", "provenance")}) == \
               tokens({x: y for x, y in b.items() if x not in ("source", "provenance")}), k
        if k not in moved:
            assert tokens(a) == tokens(b), k
            continue
        changed = {f for f in set(a["provenance"]) | set(b["provenance"]) if a["provenance"].get(f) != b["provenance"].get(f)}
        assert changed <= {"successor_bytes_sha256", "receipt_sha256", "producer_record_sha256"}, (k, changed)
        assert a["provenance"].get("receipt_sha256", a["source"]["retained_precision"]["receipt_sha256"]) == \
            a["source"]["retained_precision"]["receipt_sha256"], k


@pytest.mark.parametrize("case", sorted(MOVED_CASES))
def test_a_regenerated_corpus_case_reseals_to_its_base(case):
    before = next(c for c in _corpus_at(BASE)["cases"] if c["id"] == case)
    after = next(c for c in _corpus_now()["cases"] if c["id"] == case)
    assert tokens(after["invocation"]) == tokens(before["invocation"])
    _check_successor(case, after["source"], before["source"], after["invocation"], hooked=case in HOOKED)


@pytest.mark.parametrize("mode", MODES)
def test_the_w_cb3_fixture_documents_reseal_to_their_base_and_keep_the_legacy_bytes(mode):
    before = json.loads(_bytes_at(BASE, FIXTURES[mode]))
    after = json.loads((ROOT / FIXTURES[mode]).read_bytes())
    assert tokens({k: v for k, v in after.items() if k != "source"}) == tokens({k: v for k, v in before.items() if k != "source"})
    _check_successor(mode, after["source"], before["source"], after["invocation"])
    legacy = ROOT / FIXTURES[mode].replace(".json", "_pre_certificate.json")
    assert legacy.read_bytes() == _bytes_at(BASE, FIXTURES[mode])


def _remapped_only(before_edits, after_edits, old_base, new_base):
    """The after edits are the before edits with each path through an array that gained certificate elements remapped
    to the same element, and a whole-array replacement keeping the new base's certificate elements."""
    assert len(before_edits) == len(after_edits)
    o, n = deepcopy(old_base), deepcopy(new_base)
    for eb, ea in zip(before_edits, after_edits):
        assert eb["op"] == ea["op"] and len(eb["path"]) == len(ea["path"])
        po, pn = o, n
        for i, (pb, pa) in enumerate(zip(eb["path"], ea["path"])):
            if isinstance(po, list) and isinstance(pb, int) and pb != pa:
                aligned = [j for j, x in enumerate(pn) if not _cert(x)]
                assert len(aligned) == len(po) and aligned[pb] == pa, (eb["path"], ea["path"])
            else:
                assert pb == pa, (eb["path"], ea["path"])
            last = i == len(eb["path"]) - 1
            if not last:
                po = po[pb] if isinstance(po, (dict, list)) else None
                pn = pn[pa] if isinstance(pn, (dict, list)) else None
        if "value" in eb:
            vb, va = eb["value"], ea["value"]
            if isinstance(va, list) and any(_cert(x) for x in va) and not any(_cert(x) for x in vb):
                assert [x for x in va if not _cert(x)] == vb, ea["path"]
            else:
                assert tokens(va) == tokens(vb), ea["path"]
        for doc, ed in ((o, eb), (n, ea)):
            parent = doc
            for part in ed["path"][:-1]:
                parent = parent[part]
            if ed["op"] == "remove":
                del parent[ed["path"][-1]]
            else:
                parent[ed["path"][-1]] = deepcopy(ed["value"])


def test_the_corpus_entries_on_regenerated_bases_are_remapped_only():
    before, after = _corpus_at(BASE), _corpus_now()
    sources_before = {c["id"]: c["source"] for c in before["cases"]}
    sources_after = {c["id"]: c["source"] for c in after["cases"]}
    for kind in ("mutations", "must_pass"):
        assert [e["id"] for e in before[kind]] == [e["id"] for e in after[kind]]
        for eb, ea in zip(before[kind], after[kind]):
            assert tokens({k: v for k, v in ea.items() if k != "edits"}) == tokens({k: v for k, v in eb.items() if k != "edits"}), ea["id"]
            if eb["base"] not in MOVED_CASES:
                assert tokens(ea) == tokens(eb), ea["id"]
                continue
            _remapped_only(eb.get("edits", []), ea.get("edits", []), sources_before[eb["base"]], sources_after[eb["base"]])


def test_the_b2_parity_probes_are_remapped_only():
    before, after = json.loads(_bytes_at(BASE, PROBES)), json.loads((ROOT / PROBES).read_bytes())
    assert tokens({k: v for k, v in after.items() if k != "shapes"}) == tokens({k: v for k, v in before.items() if k != "shapes"})
    sources_before = {c["id"]: c["source"] for c in _corpus_at(BASE)["cases"]}
    sources_after = {c["id"]: c["source"] for c in _corpus_now()["cases"]}
    for sb, sa in zip(before["shapes"], after["shapes"], strict=True):
        assert tokens({k: v for k, v in sa.items() if k != "edits"}) == tokens({k: v for k, v in sb.items() if k != "edits"}), sa["name"]
        if sb["base"] not in MOVED_CASES:
            assert tokens(sa) == tokens(sb), sa["name"]
            continue
        _remapped_only(sb["edits"], sa["edits"], sources_before[sb["base"]], sources_after[sb["base"]])


def _entries(text):
    out = {}
    for line in text.splitlines():
        line = line.strip().rstrip(",")
        if line.startswith('["'):
            key, digest, readings = json.loads(line)
            out[key] = (digest, readings)
    return out


def test_the_native_reader_outcomes_move_only_with_a_regenerated_input():
    before, after = _entries(_bytes_at(BASE, OUTCOMES).decode()), _entries((ROOT / OUTCOMES).read_text())
    assert set(before) == set(after)
    corpus = _corpus_now()
    base_of = {e["id"]: e["base"] for kind in ("mutations", "must_pass") for e in corpus[kind]}
    probes = {s["name"]: s["base"] for s in json.loads((ROOT / PROBES).read_bytes())["shapes"]}

    def derived(key):
        kind, _, rest = key.partition(":")
        if kind == "corpus":
            sub, _, name = rest.partition(":")
            return name in MOVED_CASES if sub == "case" else base_of.get(name) in MOVED_CASES
        if kind == "b2_parity":
            return probes.get(rest) in MOVED_CASES
        return kind == "successor" and rest.startswith("combination_successor_")
    moved = set()
    for key in before:
        (din, rb), (dout, ra) = before[key], after[key]
        assert {r: v[1] for r, v in rb.items()} == {r: v[1] for r, v in ra.items()}, f"{key}: a summary moved"
        if din == dout:
            assert rb == ra, f"{key}: a reading moved without its input"
        else:
            assert derived(key), f"{key}: an input not derived from a regenerated base moved"
            moved.add(key)
    assert len(moved) > 100


def test_a_numerical_change_or_an_undeclared_count_is_refused():
    base = next(c for c in _corpus_at(BASE)["cases"] if c["id"] == "w_cb5_sparse_interactive")["source"]
    live = next(c for c in _corpus_now()["cases"] if c["id"] == "w_cb5_sparse_interactive")["source"]
    numeric = deepcopy(live)
    row = next(r for r in numeric["results"] if isinstance(r.get("value"), float) and r["value"] != 0.0)
    row["value"] = row["value"] * (1 + 2 ** -52)
    resealed, _ = restated(numeric, base)
    assert tokens(resealed) != tokens(base)
    counted = deepcopy(live)
    counted["retained_precision"]["body"]["product_attempts"][0]["adapter"]["counts"][4] += 2
    with pytest.raises(AssertionError):
        restated(counted, base)
    bare = deepcopy(live)
    bare["contract_evidence"].pop("combination_certificate")
    with pytest.raises(AssertionError):
        restated(bare, base)
