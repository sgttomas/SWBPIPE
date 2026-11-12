"""Regression anchor for the pressure-elongation switch (DEL-04-07 `Work/pressure-elongation.md` §7 item 1).

"Included" must be today's mechanics exactly. For every entry of
`pressure_elongation_inventory.json` this re-produces each pressure document with the
switch (`tools/validation/pressure_elongation.py`: absent, all or none), solves it with the
maintained `physics_source_connected` example (the value entry the desktop and the
fixture writers use; `exact_pressure_connected` where the inventory names it), and
compares the envelope with a reference envelope, leaf by leaf:

- every number bit for bit (binary64; -0.0 differs from 0.0; an integer differs from a float);
- every string, boolean and null exactly, and the object keys and array lengths;
- except the explicit, minimal exclusions below (identity, setting and label bytes).

Reference envelopes. By default: the committed raw envelope the inventory names (a raw
file, or the envelope stored beside the model in a corpus), else a baseline captured with
`capture` from an engine that predates the switch. After the engine change, an absent
member means "not included", so a baseline must be captured at a pre-change commit and
kept outside the repository. The committed envelopes are regenerated whenever their
fixtures are re-produced, so after the fixture re-production a default check compares
those entries with the same engine's own output, not with the pre-change mechanics.
`--baseline-only` (with `--baseline DIR`) takes every reference from the captured
baseline and never from a committed envelope: every document and mode that is solved must
have its `<key>.json` in DIR, and an entry missing one, or with nothing compared, is an
ERROR (exit status 1). The
documents and modes solved are the same as by default: the modes of the committed
envelope where the inventory names one, else every requested mode.

Re-production: each maintained Python producer applies the same helper to the
documents it writes. The committed fixtures of those producers were re-produced with the
switch at `all` (the inventory's `verify` commands pass `--pressure-elongation all`), so
their committed documents already carry the setting naming every pressurised case. The
helper is idempotent: `--switch all` solves such a document as committed, and so does
`--switch absent`, which leaves a document as it is; `--switch none` removes the member.
Writers whose model is copied from a committed source, and frozen run records, still
commit documents without the setting; `--switch all` adds it to them as their producer
would. Rust and TypeScript writers that embed these models copy them from their
committed sources.

Usage (from the repository root):

    python3 tools/validation/pressure_elongation_anchor.py check --switch absent
    python3 tools/validation/pressure_elongation_anchor.py capture --baseline DIR
    python3 tools/validation/pressure_elongation_anchor.py check --switch all --baseline DIR
    python3 tools/validation/pressure_elongation_anchor.py check --switch all --baseline DIR --baseline-only

Request-bound receipt chain. A retained-source envelope (`source_block_recovery`) commits
to the request through `invocation.value`, so naming the cases changes that hash and the
hashes chained from it, and the work charged for the request's bytes. Those leaves (the
`CHAINED_RECEIPT_LEAVES` and `WORK_LEAVES` below) are allowed to differ only when the hashed
preimages agree: for both the reference and the actual solve, the test-only
`o4_source_payload_dump` (in `core/product_physics/src/source_receipt/o4_source_tests.rs`)
writes each source case's private payload and functional plan. Each dumped text must hash
to the envelope's commitment, the payload's `invocation_sha256` must be the envelope's
`invocation.value`, and the reference and actual payloads must be equal leaf by leaf, every
float bit pattern included, except `invocation_sha256` in the source payload and
`normalized_source_sha256` in the plan. A case's `physical_evidence_sha256`, the
`publication_sha256` and the `receipt_sha256` hash published members that are compared
leaf by leaf above. Such an entry passes as PASS_CHAINED, and its work-charge deltas are
listed for the retained-pricing (D3) turn, which owns them. `capture` writes the
reference dumps beside the envelopes (`<key>.source-dump.json`).

Options: `--examples-dir` (prebuilt `core/product_physics` examples; otherwise they are
built with `$CARGO`, default `cargo`, release profile), `--entry ID` (repeatable),
`--modes sparse,dense`, `--json OUT` (machine-readable report), `--baseline-only` (above).
Exit status 0 when no
entry FAILs or ERRORs; REFUSED (the engine does not accept the member yet) exits 3.
"""
from __future__ import annotations

import argparse
import copy
import hashlib
import json
import os
import re
import struct
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any, Iterator

ROOT = Path(__file__).resolve().parents[2]
if str(ROOT) not in sys.path:
    sys.path.append(str(ROOT))
from tools.validation import pressure_elongation as PE  # noqa: E402

INVENTORY = ROOT / "tools/validation/pressure_elongation_inventory.json"
MANIFEST = "core/product_physics/Cargo.toml"
EXAMPLES = ("physics_source_connected", "exact_pressure_connected")
MODES = {"sparse": "sparse_interactive", "dense": "dense_scrutiny"}
WIRE_MODES = {wire: short for short, wire in MODES.items()}

# ----------------------------------------------------------------------------- exclusions
# The only bytes allowed to differ between a switched ("all") solve and today's envelope.
# Design §3.1 and §5 name exactly these; extend only with a reason recorded here.
EXCLUDED_KEYS = frozenset({
    "pressure_elongation",  # §3.1 the setting member (echoed models); §5 the per-case label
})
IDENTITY_SUBSTITUTIONS = (
    ("pressure-3", "pressure-2"),  # §5 the semantic identity pressure-2 -> pressure-3, in any string
)
LIMITATIONS_KEY = "limitations"
ADDED_LIMITATION_STATEMENTS = frozenset({  # §5 the three fixed statements, dropped from any `limitations` list
    "Pressure elongation is not included in any load case: the closed-end free pressure strain is omitted; "
    "pressure thrust at unbalanced locations is applied.",
    "Pressure elongation is included only in the operating load cases the model settings name, as the "
    "closed-end free strain (1 − 2ν)·p·A_i/(E·A_s), with A_i on the pressure bore and A_s the nominal "
    "section area; it is not a thin-wall mean-diameter form.",
    "Pressure elongation is set, but none of the cases it names carries pressure; it has no effect.",
})
# §5 the bend statement's pressure clause, revised to say the push applies only in a case that includes
# pressure elongation; the solve's revised clause is mapped back to today's before comparing.
BEND_CLAUSE_SUBSTITUTIONS = (
    ("each owns the term -c_b (c_b its own end caps along the end tangents) and, in a load case that includes "
     "pressure elongation, the term K_b*u_free(eps_p) from the arc's own stiffness, with eps_p = (1-2nu)pAi/(E As) "
     "(the straight Lame mean Poisson term on the arc);",
     "each owns the term K_b*u_free(eps_p) - c_b, with eps_p = (1-2nu)pAi/(E As) (the straight Lame mean Poisson "
     "term on the arc) and c_b its own end caps along the end tangents;"),
)


RECEIPT = "/source_block_recovery"
# Leaves chained from the request hash (`invocation.value`); see the module docstring.
CHAINED_RECEIPT_LEAVES = tuple(re.compile(f"^{RECEIPT}{pattern}$") for pattern in (
    r"/body/invocation/value",
    r"/body/cases/\d+/source/normalized_source_sha256",
    r"/body/cases/\d+/source/functional_plan_sha256",
    r"/body/cases/\d+/physical_evidence_sha256",
    r"/body/publication_sha256",
    r"/receipt_sha256",
))
# Work charged for the request's bytes; owned by the retained-pricing (D3) turn.
WORK_LEAVES = tuple(re.compile(f"^{RECEIPT}{pattern}$") for pattern in (
    r"/body/cases/\d+/work/charged",
    r"/body/invocation_work/charged",
    r"/body/invocation_work/publication_charged",
))
# The only payload leaves a request-bound dump may change.
PAYLOAD_CHAINED = {"normalized_source_text": "/payload/invocation_sha256",
                   "functional_plan_text": "/payload/normalized_source_sha256"}
DUMP_TEST = "o4_source_payload_dump"


def _norm_string(value: str) -> str:
    for old, new in IDENTITY_SUBSTITUTIONS + BEND_CLAUSE_SUBSTITUTIONS:
        value = value.replace(old, new)
    return value


def _escape(key: str) -> str:
    return str(key).replace("~", "~0").replace("/", "~1")


def _kind(value: Any) -> str:
    if value is None:
        return "null"
    if isinstance(value, bool):
        return "bool"
    if isinstance(value, int):
        return "int"
    if isinstance(value, float):
        return "float"
    if isinstance(value, str):
        return "string"
    if isinstance(value, list):
        return "array"
    return "object"


def differences(expected: Any, actual: Any, path: str = "", key: str | None = None) -> Iterator[tuple[str, str]]:
    """Yield (JSON pointer, reason) for every difference outside the exclusions, in document order."""
    ek, ak = _kind(expected), _kind(actual)
    if ek != ak:
        yield path or "/", f"type {ek} != {ak}"
        return
    if ek == "object":
        names = [k for k in expected if k not in EXCLUDED_KEYS]
        names += [k for k in actual if k not in EXCLUDED_KEYS and k not in expected]
        for name in names:
            child = f"{path}/{_escape(name)}"
            if name not in actual:
                yield child, "missing in actual"
            elif name not in expected:
                yield child, "unexpected in actual"
            else:
                yield from differences(expected[name], actual[name], child, name)
    elif ek == "array":
        if key == LIMITATIONS_KEY:
            expected = [v for v in expected if v not in ADDED_LIMITATION_STATEMENTS]
            actual = [v for v in actual if v not in ADDED_LIMITATION_STATEMENTS]
        if len(expected) != len(actual):
            yield path or "/", f"array length {len(expected)} != {len(actual)}"
        for index, (e, a) in enumerate(zip(expected, actual)):
            yield from differences(e, a, f"{path}/{index}")
    elif ek == "float":
        if struct.pack(">d", expected) != struct.pack(">d", actual):
            yield path or "/", f"number {expected!r} != {actual!r}"
    elif ek == "string":
        if _norm_string(expected) != _norm_string(actual):
            yield path or "/", f"string {expected[:80]!r} != {actual[:80]!r}"
    elif expected != actual:
        yield path or "/", f"{expected!r} != {actual!r}"


def compare(expected: Any, actual: Any) -> tuple[int, str | None]:
    """(number of differences, first difference as 'pointer: reason' or None)."""
    count, first = 0, None
    for pointer, reason in differences(expected, actual):
        count += 1
        if first is None:
            first = f"{pointer}: {reason}"
    return count, first


def loads(text: str) -> Any:
    """JSON with binary64 floats; non-finite literals are refused."""
    def constant(name: str):
        raise ValueError(f"non-finite JSON literal {name}")
    return json.loads(text, parse_constant=constant)


# ----------------------------------------------------------------------------- re-authoring twin
# Python twin of core/product_physics/tests/common/current_model.rs `reauthor_model`: the in-memory
# re-authoring every pinning test applies to an older committed copy. Nothing else changes.
MODEL_VERSION = "0.9.0"
RETIRED_MODEL_VERSIONS = ("0.1.0", "0.2.0", "0.3.0", "0.4.0", "0.5.0", "0.6.0", "0.7.0", "0.8.0")
PREVIOUS_MODEL_VERSIONS = ("0.5.0", "0.6.0", "0.7.0", "0.8.0")
CURRENT_CONTRACT = {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"}


def reauthor_model(model: dict) -> dict:
    """Re-author an older copy; a current document is solved as committed, as its tests read it."""
    version = model.get("schema_version")
    if version == MODEL_VERSION:
        return model
    if version in PREVIOUS_MODEL_VERSIONS:
        model["schema_version"] = MODEL_VERSION
        return model
    if version in RETIRED_MODEL_VERSIONS:
        model["schema_version"] = MODEL_VERSION
    contract = model.get("pressure_contract")
    if isinstance(contract, dict) and (contract.get("version"), contract.get("mode")) in (
            ("2.0.0", "exact_straight_pressure_v2"), ("3.0.0", "exact_pressure_v3")):
        model["pressure_contract"] = dict(CURRENT_CONTRACT)
    if model.get("pressure_contract") == CURRENT_CONTRACT:
        for pipe in model.get("pipe_segments") or []:
            section = pipe["section"]
            if "corrosion_allowance" not in section:
                section["corrosion_allowance"] = {"value": 0.0, "unit": "m"}
    return model


# ----------------------------------------------------------------------------- pointers
def resolve(container: Any, pointer: str) -> Any:
    value = container
    for part in pointer.split("/")[1:] if pointer else []:
        part = part.replace("~1", "/").replace("~0", "~")
        value = value[int(part)] if isinstance(value, list) else value[part]
    return value


def parent_pointer(pointer: str, depth: int = 1) -> str:
    parts = pointer.split("/")
    return "/".join(parts[: len(parts) - depth]) if depth else pointer


def request_for(container: Any, pointer: str, switch: str) -> dict:
    """The solve request for one document: its own {model, materials} wrapper, else model-only."""
    doc = copy.deepcopy(resolve(container, pointer))
    parent = resolve(container, parent_pointer(pointer)) if pointer else None
    if pointer.endswith("/model") and isinstance(parent, dict) and set(parent) == {"model", "materials"}:
        request = {"model": doc, "materials": copy.deepcopy(parent["materials"])}
    else:
        request = {"model": doc, "materials": []}
    reauthor_model(request["model"])
    PE.apply(request["model"], switch)
    return request


# ----------------------------------------------------------------------------- reference envelopes
def committed_envelopes(entry: dict, document: dict, container: Any) -> dict[str, Any] | None:
    """{short mode: committed envelope} for one document, or None when the inventory names none."""
    anchor = entry["anchor"]
    kind = anchor["kind"]
    if kind == "raw_files":
        files = (anchor.get("per_document") or {}).get(document["pointer"], anchor.get("files"))
        if files is None:
            return None
        return {WIRE_MODES[mode]: loads((ROOT / path).read_text(encoding="utf-8")) for mode, path in files.items()}
    if kind == "inline":
        case = resolve(container, parent_pointer(document["pointer"], anchor["case_depth"]))
        mode = case.get(anchor["mode_member"]) if "mode_member" in anchor else anchor["mode"]
        envelope = case.get(anchor["envelope"])
        if envelope is None or mode not in WIRE_MODES:
            return None
        return {WIRE_MODES[mode]: envelope}
    return None


def baseline_key(entry_id: str, pointer: str, mode: str) -> str:
    return hashlib.sha256(f"{entry_id}\0{pointer}\0{mode}".encode()).hexdigest()[:24]


# ----------------------------------------------------------------------------- solving
class Solver:
    def __init__(self, examples_dir: Path | None):
        self.examples_dir = examples_dir
        self.cache: dict[tuple, tuple] = {}

    def ensure(self) -> None:
        if self.examples_dir is None:
            cargo = os.environ.get("CARGO", "cargo")
            subprocess.run([cargo, "build", "--release", "--examples", "--manifest-path", str(ROOT / MANIFEST)],
                           check=True, cwd=ROOT)
            self.examples_dir = ROOT / "core/product_physics/target/release/examples"
        for name in EXAMPLES:
            if not (self.examples_dir / name).exists():
                raise SystemExit(f"missing example binary {self.examples_dir / name}")

    def dump(self, request: dict, mode: str) -> dict:
        """The test-only declared-problem dump of `request` (see the module docstring)."""
        if getattr(self, "dump_binary", None) is None:
            cargo = os.environ.get("CARGO", "cargo")
            built = subprocess.run([cargo, "test", "--lib", "--no-run", "--message-format=json",
                                    "--manifest-path", str(ROOT / MANIFEST)],
                                   capture_output=True, text=True, check=True, cwd=ROOT)
            binaries = [m["executable"] for m in map(json.loads, filter(None, built.stdout.splitlines()))
                        if m.get("reason") == "compiler-artifact" and m.get("executable")
                        and m["target"]["name"] == "open_pipe_stress_product_physics" and m["profile"]["test"]]
            if len(binaries) != 1:
                raise SystemExit(f"cannot find the product_physics lib test binary: {binaries}")
            self.dump_binary = binaries[0]
        with tempfile.TemporaryDirectory() as tmp:
            source, out = Path(tmp) / "request.json", Path(tmp) / "dump.json"
            source.write_text(json.dumps(request, ensure_ascii=False, allow_nan=False), encoding="utf-8")
            env = dict(os.environ, SWBPIPE_O4_SOURCE_DUMP_REQUEST=str(source), SWBPIPE_O4_SOURCE_DUMP_OUT=str(out),
                       SWBPIPE_O4_SOURCE_DUMP_MODE=MODES[mode])
            done = subprocess.run([self.dump_binary, DUMP_TEST, "--quiet"],
                                  capture_output=True, text=True, check=False, env=env,
                                  cwd=ROOT / Path(MANIFEST).parent)
            if done.returncode != 0 or not out.exists():
                raise SystemExit(f"source dump failed: {done.stdout[-400:]} {done.stderr[-400:]}")
            return json.loads(out.read_text(encoding="utf-8"))

    def solve(self, request: dict, mode: str, example: str) -> tuple[str, Any, str]:
        """(state, envelope or None, stderr): state is 'envelope', 'refused_member' or 'error'."""
        text = json.dumps(request, ensure_ascii=False, allow_nan=False)
        key = (hashlib.sha256(text.encode()).hexdigest(), mode, example)
        if key not in self.cache:
            with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False, encoding="utf-8") as handle:
                handle.write(text)
            try:
                done = subprocess.run([str(self.examples_dir / example), mode, handle.name],
                                      capture_output=True, text=True, check=False)
            finally:
                os.unlink(handle.name)
            stderr = done.stderr.strip()
            if done.stdout.strip():
                self.cache[key] = ("envelope", loads(done.stdout), stderr)
            elif re.search(r"unknown field `(pressure_elongation|model_settings)`", stderr):
                self.cache[key] = ("refused_member", None, stderr)
            else:
                self.cache[key] = ("error", None, stderr or f"exit {done.returncode} without output")
        return self.cache[key]


# ----------------------------------------------------------------------------- the run
def load_inventory() -> dict:
    return json.loads(INVENTORY.read_text(encoding="utf-8"))


def run(args) -> int:
    inventory = load_inventory()
    entries = [e for e in inventory["entries"] if not args.entry or e["id"] in args.entry]
    if args.entry and len(entries) != len(set(args.entry)):
        raise SystemExit(f"unknown entry: {sorted(set(args.entry) - {e['id'] for e in entries})}")
    modes = [m for m in args.modes.split(",") if m]
    if any(m not in MODES for m in modes):
        raise SystemExit("--modes takes sparse and/or dense")
    switch = "absent" if args.command == "capture" else args.switch
    baseline = Path(args.baseline).resolve() if args.baseline else None
    baseline_only = args.command == "check" and args.baseline_only
    if baseline_only and baseline is None:
        raise SystemExit("--baseline-only needs --baseline DIR")
    if args.command == "capture":
        baseline.mkdir(parents=True, exist_ok=True)
    solver = Solver(Path(args.examples_dir).resolve() if args.examples_dir else None)
    solver.ensure()
    report = {"switch": switch, "command": args.command, "modes": modes,
              "reference": ("baseline only" if baseline_only else
                            "committed envelope, else baseline") if args.command == "check" else None,
              "baseline": str(baseline) if baseline else None,
              "exclusions": {"keys": sorted(EXCLUDED_KEYS),
                             "identity_substitutions": [list(p) for p in IDENTITY_SUBSTITUTIONS],
                             "limitations_dropped": sorted(ADDED_LIMITATION_STATEMENTS),
                             "bend_clause_mapped": [new for new, _ in BEND_CLAUSE_SUBSTITUTIONS],
                             "chained_receipt_leaves": [p.pattern for p in CHAINED_RECEIPT_LEAVES],
                             "work_leaves_for_d3": [p.pattern for p in WORK_LEAVES],
                             "payload_chained": PAYLOAD_CHAINED},
              "entries": []}
    totals: dict[str, int] = {}
    for entry in entries:
        result = run_entry(entry, switch, modes, solver, baseline, args.command, baseline_only)
        report["entries"].append(result)
        totals[result["status"]] = totals.get(result["status"], 0) + 1
        line = f"{result['status']:<10} {entry['id']}"
        if result.get("detail"):
            line += f"  {result['detail']}"
        print(line, flush=True)
    report["totals"] = totals
    print("totals: " + ", ".join(f"{k} {v}" for k, v in sorted(totals.items())))
    if args.json:
        Path(args.json).write_text(json.dumps(report, indent=1, ensure_ascii=False) + "\n", encoding="utf-8")
    if totals.get("FAIL") or totals.get("ERROR"):
        return 1
    if totals.get("REFUSED"):
        print("REFUSED: the engine does not accept model_settings.pressure_elongation yet "
              "(expected until the engine change lands); nothing was compared for those entries")
        return 3
    return 0


def _matches(pointer: str, patterns: tuple) -> bool:
    return any(p.match(pointer) for p in patterns)


def _dump_bound(dump: dict, envelope: Any) -> str | None:
    """None when every dumped text hashes to the envelope's commitments, else the reason."""
    body = envelope["source_block_recovery"]["body"]
    cases = {c["basis_ref"]["ref_id"]: c for c in body["cases"]}
    if not dump.get("records"):
        return "dump has no source case"
    for record in dump["records"]:
        case = cases.get(record["case_id"])
        if case is None:
            return f"dumped case {record['case_id']} not in the envelope"
        for text, key in (("normalized_source_text", "normalized_source_sha256"),
                          ("functional_plan_text", "functional_plan_sha256")):
            digest = hashlib.sha256(record[text].encode("utf-8")).hexdigest()
            if not (digest == record[key] == case["source"][key]):
                return f"{record['case_id']} {key} does not hash its dumped text"
        if json.loads(record["normalized_source_text"])["payload"]["invocation_sha256"] != body["invocation"]["value"]:
            return f"{record['case_id']} payload invocation_sha256 is not the envelope's invocation"
    return None


def receipt_chain(reference: Any, envelope: Any, reference_dump: dict | None, actual_dump: dict) -> str | None:
    """None when the request-bound chain holds (module docstring), else the reason."""
    if reference_dump is None:
        return "no reference source dump (capture writes <key>.source-dump.json)"
    for name, dump, env in (("reference", reference_dump, reference), ("actual", actual_dump, envelope)):
        reason = _dump_bound(dump, env)
        if reason:
            return f"{name}: {reason}"
    expected = {r["case_id"]: r for r in reference_dump["records"]}
    actual = {r["case_id"]: r for r in actual_dump["records"]}
    if expected.keys() != actual.keys():
        return f"dumped cases {sorted(expected)} != {sorted(actual)}"
    for case_id, record in expected.items():
        for text, allowed in PAYLOAD_CHAINED.items():
            moved = [p for p, _ in differences(json.loads(record[text]), json.loads(actual[case_id][text]))
                     if p != allowed]
            if moved:
                return f"{case_id} {text} differs beyond {allowed}: {moved[:5]}"
    return None


def run_entry(entry: dict, switch: str, modes: list[str], solver: Solver, baseline: Path | None, command: str,
              baseline_only: bool = False) -> dict:
    """One entry's status. `baseline_only`: every reference from `baseline`, a missing one an ERROR."""
    anchor = entry["anchor"]
    if anchor["kind"] == "excluded":
        return {"id": entry["id"], "status": "EXCLUDED", "detail": anchor["reason"]}
    container = loads((ROOT / entry["path"]).read_text(encoding="utf-8"))
    example = anchor.get("example", EXAMPLES[0])
    compared = solved = 0
    uncompared, chained, work = [], [], []
    for document in entry["documents"]:
        pointer = document["pointer"]
        request = request_for(container, pointer, switch)
        committed = committed_envelopes(entry, document, container)
        for mode in modes:
            if committed is not None and mode not in committed:
                continue
            state, envelope, stderr = solver.solve(request, mode, example)
            where = f"{pointer or '/'} [{mode}]"
            last = stderr.splitlines()[-1] if stderr else ""
            if state == "refused_member":
                return {"id": entry["id"], "status": "REFUSED", "detail": f"{where}: {last}"}
            if state == "error":
                return {"id": entry["id"], "status": "ERROR", "detail": f"{where}: {last}"}
            solved += 1
            key = baseline_key(entry["id"], pointer, mode)
            if command == "capture":
                (baseline / f"{key}.json").write_text(json.dumps(envelope, ensure_ascii=False) + "\n", encoding="utf-8")
                if isinstance(envelope, dict) and envelope.get("source_block_recovery"):
                    (baseline / f"{key}.source-dump.json").write_text(
                        json.dumps(solver.dump(request, mode), ensure_ascii=False) + "\n", encoding="utf-8")
                continue
            if baseline_only:
                if baseline is None or not (baseline / f"{key}.json").exists():
                    return {"id": entry["id"], "status": "ERROR",
                            "detail": f"{where}: --baseline-only and no baseline envelope {key}.json"}
                reference = loads((baseline / f"{key}.json").read_text(encoding="utf-8"))
            elif committed is not None:
                reference = committed[mode]
            elif baseline is not None and (baseline / f"{key}.json").exists():
                reference = loads((baseline / f"{key}.json").read_text(encoding="utf-8"))
            else:
                uncompared.append(where)
                continue
            found = list(differences(reference, envelope))
            compared += 1
            other = [(p, r) for p, r in found if not _matches(p, CHAINED_RECEIPT_LEAVES + WORK_LEAVES)]
            if other:
                return {"id": entry["id"], "status": "FAIL",
                        "detail": f"{where}: {len(found)} differences; first {other[0][0]}: {other[0][1]}",
                        "differences": [f"{p}: {r}" for p, r in other[:20]]}
            if found:
                dump_path = baseline / f"{key}.source-dump.json" if baseline is not None else None
                reference_dump = (json.loads(dump_path.read_text(encoding="utf-8"))
                                  if dump_path is not None and dump_path.exists() else None)
                reason = receipt_chain(reference, envelope, reference_dump, solver.dump(request, mode))
                if reason:
                    return {"id": entry["id"], "status": "FAIL", "detail": f"{where}: receipt chain: {reason}",
                            "differences": [f"{p}: {r}" for p, r in found[:20]]}
                chained.append(where)
                work.extend(f"{where} {p}: {r}" for p, r in found if _matches(p, WORK_LEAVES))
    if command == "capture":
        return {"id": entry["id"], "status": "CAPTURED", "detail": f"{solved} envelopes"}
    if compared == 0 and baseline_only:
        return {"id": entry["id"], "status": "ERROR",
                "detail": f"--baseline-only and nothing compared ({solved} solved; no requested mode to compare)"}
    if compared == 0:
        return {"id": entry["id"], "status": "UNCOMPARED",
                "detail": f"{solved} solved; no committed envelope and no baseline (pass --baseline)"}
    detail = f"{compared} envelopes identical"
    if chained:
        detail = (f"{compared} envelopes identical but for the request-bound receipt chain in {len(chained)}"
                  f" (source payloads identical); work deltas for D3: {len(work)}")
    if uncompared:
        detail += f"; {len(uncompared)} without a reference"
    result = {"id": entry["id"], "status": "PASS_CHAINED" if chained else "PASS", "detail": detail}
    if work:
        result["work_deltas"] = work
    return result


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    sub = parser.add_subparsers(dest="command", required=True)
    for name in ("check", "capture"):
        p = sub.add_parser(name)
        if name == "check":
            p.add_argument("--switch", choices=PE.MODES, default="absent")
            p.add_argument("--baseline", help="directory of envelopes captured before the engine change")
            p.add_argument("--baseline-only", action="store_true",
                           help="take every reference from --baseline, never a committed envelope; "
                                "a missing baseline envelope is an ERROR")
        else:
            p.add_argument("--baseline", required=True, help="directory to write the captured envelopes into")
        p.add_argument("--examples-dir")
        p.add_argument("--entry", action="append")
        p.add_argument("--modes", default="sparse,dense")
        p.add_argument("--json")
    args = parser.parse_args(argv)
    return run(args)


if __name__ == "__main__":
    sys.exit(main())
