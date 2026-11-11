"""RT-A3 P (DEL-04-07 Work `rt-a3-p-scale-repair.md` §8.3, §8.9): the committed carriers P moved, and only as declared.

P put the sparse-mode friction coupling on the pattern (step 4, a declared φ⁰ move) and left dense scrutiny's path
unchanged. This guard compares the committed carriers at P's base, main 252bef969, and P's final candidate 78580da7e
(both fixed revisions, so later stages' regenerations do not reach it). Increment 4's guard ends at 252bef969 and the
certificate's begins at 78580da7e; P's interval is this file's alone. P is not a text-only stage.

- **The carrier set.** The fixtures that changed are exactly the declared ones: the three sparse captures of the
  invented coupled model (the result fixture, the range envelope and the demo), their two generation records, the
  carrier-case pin of the invented capture and the reader outcomes. No dense carrier and no other fixture changed:
  dense scrutiny is byte-identical on every committed carrier.
- **The moves.** In each moved capture the row ids, kinds, units, metadata and order, the status, numerical quality and
  every diagnostic but the two integrity statements (which embed the final iterate's report) are unchanged. Values
  move only in the loop's binary64 evidence rows (`nonlinear_support_*`) and in certified member-derived rows that are
  zero up to noise at both endpoints (|value| <= 1e-30 in their unit; the largest is 4.49e-33).
- **The reader outcomes.** Only entries whose input is a mutation of the invented sparse capture move, and only in
  their input digest; every reading and summary is unchanged. The carrier-case pin is that capture's SHA-256 at each
  endpoint. The current outcomes' digests are re-derived at the head by the native reader test
  (`r1_native_reader_outcomes`) and the desktop's wasm equivalence test.

What this guard cannot re-prove: P's |new - old| <= r_old + r_new enclosure check over the 100-output committed
nonlinear corpus, and that corpus's dense byte identity, needed P's pre-change solver and a scratch build printing each
certified row's radius (radii are not published). They are one-time migration evidence, kept in P's record as
history. The maintained regressions of the mechanism are NI's shadow check (every sparse-mode loop solve re-run on the
dense-derived sequence, `Debug`-equal), kd5's F-2 bound in both modes and the K-1 twin corpus.
"""
from pathlib import Path
import hashlib
import json
import subprocess

ROOT = Path(__file__).resolve().parents[1]
BASE = "252bef9693a682a2f9330dce7850869591800500"
END = "78580da7e0334c35fd1fc7906057844de07e2e69"

CAPTURES = (
    "fixtures/results/preview_physics_invented_sparse.json",
    "fixtures/results/preview_physics_range_envelope_sparse.json",
    "fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json",
)
DECLARED = set(CAPTURES) | {
    "fixtures/product_preview/demo_fixture_generation.json",
    "fixtures/product_preview/preview_physics_fixture_generation.json",
    "fixtures/results/retained_precision_carrier_cases.json",
    "fixtures/results/retained_precision_reader_outcomes.json",
}
CARRIER_CASES = "fixtures/results/retained_precision_carrier_cases.json"
OUTCOMES = "fixtures/results/retained_precision_reader_outcomes.json"
NOISE = 1e-30


def _bytes(rev, name):
    return subprocess.check_output(["git", "show", f"{rev}:{name}"], cwd=ROOT)


def _json(rev, name):
    return json.loads(_bytes(rev, name))


def test_the_changed_fixtures_are_exactly_the_declared_ones_and_no_dense_carrier_moved():
    changed = set(subprocess.check_output(["git", "diff", "--name-only", BASE, END, "--", "fixtures/"], cwd=ROOT, text=True).split())
    assert changed == DECLARED, sorted(changed ^ DECLARED)
    assert not any("dense" in name for name in changed)


def _without(item, key):
    return {k: v for k, v in item.items() if k != key}


def test_each_moved_capture_moves_only_loop_evidence_and_noise_level_member_rows():
    for name in CAPTURES:
        before, after = _json(BASE, name), _json(END, name)
        assert set(before) == set(after), name
        assert {k: v for k, v in before.items() if k not in ("results", "diagnostics")} == \
               {k: v for k, v in after.items() if k not in ("results", "diagnostics")}, name
        rows = list(zip(before["results"], after["results"]))
        assert len(before["results"]) == len(after["results"]), name
        assert all(_without(a, "value") == _without(b, "value") for a, b in rows), name
        moved = [(a, b) for a, b in rows if a["value"] != b["value"]]
        assert moved, name
        for a, b in moved:
            if a["kind"].startswith("nonlinear_support_"):
                continue
            assert abs(a["value"]) <= NOISE and abs(b["value"]) <= NOISE, (name, a["id"], a["value"], b["value"])
        diagnostics = list(zip(before["diagnostics"], after["diagnostics"]))
        assert len(before["diagnostics"]) == len(after["diagnostics"]), name
        assert all(_without(a, "message") == _without(b, "message") for a, b in diagnostics), name
        assert {a["code"] for a, b in diagnostics if a != b} == {"NUMERICAL_INTEGRITY_CHECKS_PASSED"}, name


def _entries(rev):
    out = {}
    for line in _bytes(rev, OUTCOMES).decode("utf-8").splitlines():
        line = line.strip().rstrip(",")
        if line.startswith('["'):
            key, digest, readings = json.loads(line)
            out[key] = (digest, readings)
    return out


def test_the_reader_outcomes_move_only_the_invented_captures_input_digests():
    before, after = _entries(BASE), _entries(END)
    assert set(before) == set(after) and len(before) > 900
    moved = {k for k in before if before[k][0] != after[k][0]}
    assert moved and all("preview_physics_1" in k for k in moved), sorted(moved)
    assert all(before[k][1] == after[k][1] for k in before), "a reading or summary moved"


def test_the_carrier_case_pin_is_the_capture_at_each_endpoint():
    for rev in (BASE, END):
        pins = _json(rev, CARRIER_CASES)["fixtures"]["preview_physics_1_invented_sparse"]
        assert pins["path"] == CAPTURES[0]
        assert pins["sha256"] == hashlib.sha256(_bytes(rev, CAPTURES[0])).hexdigest(), rev


def test_a_numerical_move_in_a_dense_carrier_or_a_large_member_move_is_refused():
    name = CAPTURES[0]
    before, after = _json(BASE, name), _json(END, name)
    index = next(i for i, row in enumerate(after["results"]) if row["kind"] == "element_local_axial_force" and abs(row["value"]) > 1)
    forged = json.loads(json.dumps(after))
    forged["results"][index]["value"] = forged["results"][index]["value"] * (1 + 1e-15) + 1e-12
    a, b = before["results"][index], forged["results"][index]
    assert not (abs(a["value"]) <= NOISE and abs(b["value"]) <= NOISE)
