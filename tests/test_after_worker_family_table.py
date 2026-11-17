"""The after-worker family table (tools/after_worker_family, DEL-04-05), against its threat model (README):
drift between the committed table and the web sources it cites.

The check passes on the committed files. The controls run on a temporary copy of the cited files and the
judge, never on the repository: a changed anchor or cited text fails and names its row; lines inserted above
the cited passages pass; a removed cited file fails and says it is missing. The registered check's path
selection covers every file the table cites.
"""
from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import shutil
import subprocess
import sys

import pytest

PROJECT = Path(__file__).resolve().parents[1]
TOOL = PROJECT / "tools" / "after_worker_family" / "family_table.py"
WORKFLOW = PROJECT / "software-workflow.json"

_spec = importlib.util.spec_from_file_location("after_worker_family_table", TOOL)
family = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(family)

WEB = "apps/desktop/src"
JUDGE = "apps/desktop/src/test-support/after-worker/afterWorkerReplies.ts"
CITED = [f"apps/desktop/src/{rel}" for rel in family.cited_files()]
assert (WEB, JUDGE) == (family.WEB, f"{family.WEB}/{family.JUDGE}")


def run(root: Path):
    done = subprocess.run([sys.executable, "-I", str(TOOL), str(root)], capture_output=True, text=True, timeout=120)
    return done.returncode, done.stderr


def mismatches(stderr: str) -> list[str]:
    return [line for line in stderr.splitlines() if line.startswith("MISMATCH: ")]


@pytest.fixture
def copy(tmp_path: Path) -> Path:
    """The judge and every cited file, copied under a temporary root."""
    for rel in [JUDGE, *CITED]:
        target = tmp_path / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(PROJECT / rel, target)
    return tmp_path


def edit(root: Path, rel: str, old: str, new: str) -> None:
    path = root / WEB / rel
    text = path.read_text(encoding="utf-8")
    assert text.count(old) == 1, f"the control's text is not found once in {rel}"
    path.write_text(text.replace(old, new), encoding="utf-8")


def test_the_check_passes_on_the_committed_files():
    code, stderr = run(PROJECT)
    assert code == 0, stderr
    assert not mismatches(stderr)


def test_the_unchanged_copy_passes(copy: Path):
    code, stderr = run(copy)
    assert code == 0, stderr


def test_a_changed_cited_text_fails_and_names_its_row(copy: Path):
    # Inside the body of a restated declaration; its name, and so its anchor, is unchanged.
    edit(copy, family.PREVIEW, "function nativeEnvelopeRefusal(", "function nativeEnvelopeRefusal(/* edited */")
    code, stderr = run(copy)
    assert code == 1
    assert mismatches(stderr) == ["MISMATCH: nativeEnvelopeRefusal: its cited text changed (services/previewService.ts: "
                                  "nativeEnvelopeRefusal): re-read the source and revalidate this row's restatement"]


def test_a_changed_passage_inside_an_inline_row_fails_and_names_its_row(copy: Path):
    edit(copy, family.SESSION, "SOLVE-PRODUCER-CONTRACT-UNSUPPORTED: a fresh result", "SOLVE-PRODUCER-CONTRACT-UNSUPPORTED: an edited result")
    code, stderr = run(copy)
    assert code == 1
    found = mismatches(stderr)
    assert len(found) == 1 and found[0].startswith("MISMATCH: the session gates 1 and 2: its cited text changed"), stderr


def test_a_changed_anchor_fails_and_names_its_row(copy: Path):
    edit(copy, "features/workspace/resultsSessionState.ts", 'useState<PreviewSolverMode>("sparse_interactive")',
         'useState<PreviewSolverMode>("dense")')
    code, stderr = run(copy)
    assert code == 1
    found = mismatches(stderr)
    assert any(line.startswith("MISMATCH: the solver mode: its anchor") and "occurs 0 times" in line for line in found), stderr
    assert all("the solver mode" in line or "family-table.md" in line for line in found), stderr


BYTES = "bytesInvoke (the byte responses' delivery guard: bytes or text)"
CACHE = "cached (the reading cache)"
READERS = "validateNativeReaders"
GATE = "nativeRegistrationHolds (gate 1, through hasNativeMechanicsInvocation)"
CAPTURE = "captureNativeInvocation (the fresh capture)"
ORDER = "readNativeFreshResult (its order of reads)"
KEYS = "readNativeVerdict and readNativeSummary (their cache keys)"
GATES = "the session gates 1 and 2"
PAGE = "the panel's first page and the empty-page re-read"
TABLE = "the semantics' recorded-table agreement"
# Each source a restated row's claim relies on beyond its own passage, mutated: (file, old, new, the rows it must name).
COVERED = [
    (family.NATIVE_RESULT, "  return transport(command, args);", "  return transport(command, { ...args });", {BYTES}),
    (family.NATIVE_RESULT, "  if (existing) return existing as Promise<T>;\n", "", {CACHE}),
    (family.NATIVE_RESULT, "  if (job.disposed) throw new Error(`${NATIVE_RESULT_DISPOSED}: ${jobId}'s native custody has ended.`);",
     "  if (job.disposed && false) throw new Error(`${NATIVE_RESULT_DISPOSED}: ${jobId}'s native custody has ended.`);", {CACHE}),
    (family.NATIVE_RESULT, "    job = { disposed: false, readings: new Map(), fullCopy: null };",
     "    job = { disposed: false, readings: new Map<string, Promise<unknown>>(), fullCopy: null };", {CACHE}),
    # readNativeValidation lies inside the cache keys' passage as well.
    (family.NATIVE_RESULT, 'return cached(jobId, keyOf("validate", reader', 'return cached(jobId, keyOf("validation", reader', {READERS, KEYS}),
    (family.PREVIEW, "  if (!source || !model) return false;", "  if (!source || !model || !solverMode) return false;", {GATE}),
    (family.NATIVE_RESULT, "NATIVE in (value as object)", "NATIVE in Object(value)", {GATE}),
    (family.PREVIEW, "const capture = captureNativeInvocation(model, solverMode);",
     "const capture = captureNativeInvocation(structuredClone(model), solverMode);", {CAPTURE}),
    (family.PREVIEW, "if (capture) jobInvocations.set(receipt.job_id, capture);",
     "if (capture) jobInvocations.set(String(receipt.job_id), capture);", {CAPTURE}),
    (family.NATIVE_RESULT, 'operation:"route_header"', 'operation:"route_header", raw: false', {ORDER}),
    (family.PREVIEW, "  return presentedStanding(await readNativeVerdict(jobId, inputs));",
     "  return presentedStanding(await readNativeVerdict(jobId, { ...inputs, invocationText: null }));", {ORDER}),
    (family.PREVIEW, "return { contract: standing.route, status: standing.status, eligible: standing.eligible",
     "return { contract: standing.route, status: standing.status, eligible: !!standing.eligible", {ORDER}),
    (family.NATIVE_RESULT, "limit: number, raw = false)", "limit: number, raw = true)", {ORDER}),
    (family.SESSION, "readNativeRunPreview(nativeCustody(output).jobId, {", "readNativeRunPreview(nativeCustody(output).jobId, { ", {GATES}),
    (family.PANEL, "const [pageSize, setPageSize] = useState<number>(DEFAULT_RESULT_PAGE_SIZE);",
     "const [pageSize, setPageSize] = useState<number>(DEFAULT_RESULT_PAGE_SIZE * 2);", {PAGE}),
    (family.NATIVE_RESULT, "  return page(jobId, () => queryResultRows(detailInvoke, jobId, filter, offset, limit));",
     "  return page(jobId, () => queryResultRows(detailInvoke, jobId, filter, offset, limit + 1));", {PAGE}),
    ("features/results/ResultsPanel.tsx", 'export const ALL_FRAMES = "all";', 'export const ALL_FRAMES = "any";', {PAGE}),
    ("features/results/resultsNavigator.ts", 'export const ALL_CASES = "all";', 'export const ALL_CASES = "any";', {PAGE}),
    (family.PANEL, "  recordedTableRead = null\n", "  recordedTableRead = undefined\n", {TABLE}),
    ("features/results/resultReader.ts", "return (a ?? null) === null ? (b ?? null) === null :", "return (a ?? null) === null ? true :", {TABLE}),
    (family.NATIVE_RESULT, "  return page(jobId, () => readResultRowSemantics(detailInvoke, jobId, ordinals));",
     "  return page(jobId, () => readResultRowSemantics(detailInvoke, jobId, [...ordinals]));", {TABLE}),
    # Shared implementations the judge calls with other inputs than the web: their handling of the difference.
    # panelDetailFilter: the judge omits kind and selectionEntity, the panel passes null; an undefined-only default
    # kind would filter the judge's first page while the panel's stays unfiltered.
    ("services/resultDetail.ts", "    ...(state.kind ? { kinds: [state.kind] } : {}),",
     '    ...(state.kind !== undefined ? (state.kind ? { kinds: [state.kind] } : {}) : { kinds: ["force"] }),', {PAGE}),
    # The web's ingress maps a rejection with nativeInvokeError (and resends a headroom refusal).
    ("services/nativeIngress.ts", "      const read = nativeInvokeError(error);", "      const read = nativeInvokeError(error ?? null);",
     {READERS, ORDER}),
    # nativeStandingInputs gets the judge's capture with empty fingerprints.
    (family.PREVIEW, '  if (route !== "pressure_source" || !registered || !capture || capture.invalidated) return { modelText, invocationText: null };',
     '  if (route !== "pressure_source" || !registered || !capture || capture.invalidated || !capture.fingerprint) return { modelText, invocationText: null };',
     {CAPTURE}),
    # The judge omits queryResultSourceRecovery's raw; the web passes false.
    ("services/resultDetail.ts", "limit: number, raw = false): Promise<SourceRecovery>", "limit: number, raw = true): Promise<SourceRecovery>", {ORDER}),
    # The judge's handle carries resultBytes 0: createNativeResult and gate 2's path for a native handle.
    (family.NATIVE_RESULT, "  Object.defineProperty(handle, NATIVE, { enumerable: false, value: custody });",
     "  Object.defineProperty(handle, NATIVE, { enumerable: false, value: Object.freeze(custody) });", {GATES}),
    ("features/results/resultReader.ts", "  return currentSemanticContract(source) !== null;", "  return currentSemanticContract(source) != null;", {GATES}),
    ("features/results/resultReader.ts", "  try { return sourceSemanticBinding(source); } catch { return null; }",
     '  try { return sourceSemanticBinding(source); } catch { return { id: "", sha256: "" }; }', {GATES}),
    ("features/results/resultReader.ts", '  if (!binding) throw new Error("SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED");',
     '  if (!binding || !binding.id) throw new Error("SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED");', {GATES}),
    ("features/results/resultReader.ts", "  if (isNativeResult(source)) return nativeDispatch(source, model, invocation);",
     "  if (isNativeResult(source)) return nativeDispatch(source, model ?? null, invocation);", {GATES}),
    ("features/results/resultReader.ts", "  if (model === undefined && invocation === undefined) return verdict;",
     "  if (model == null && invocation == null) return verdict;", {GATES}),
]


@pytest.mark.parametrize("rel, old, new, rows", COVERED, ids=[f"{c[0].rsplit('/', 1)[-1]}:{i}" for i, c in enumerate(COVERED)])
def test_a_changed_covered_source_fails_and_names_its_rows(copy: Path, rel, old, new, rows):
    """A source a restated row's claim relies on, changed while the row's own passage is not: the check fails
    and names exactly the rows that cover it (the table may also differ, when an anchor moved)."""
    edit(copy, rel, old, new)
    code, stderr = run(copy)
    assert code == 1
    found = [line for line in mismatches(stderr) if "family-table.md" not in line]
    named = {r["predicate"] for r in family.ROWS if any(line.startswith(f"MISMATCH: {r['predicate']}: ") for line in found)}
    assert named == rows and len(found) == len(rows), stderr


def test_an_inline_row_whose_name_the_judge_imports_fails_and_names_its_row(copy: Path):
    """An inline restated row that names a web declaration: the judge importing it fails, naming the row."""
    path = copy / JUDGE
    text = path.read_text(encoding="utf-8")
    old = 'import { createNativeResult, decodeNativeRouteHeader,'
    assert text.count(old) == 1
    path.write_text(text.replace(old, 'import { createNativeResult, readNativeVerdict, decodeNativeRouteHeader,'), encoding="utf-8")
    code, stderr = run(copy)
    assert code == 1
    assert mismatches(stderr) == ["MISMATCH: readNativeVerdict and readNativeSummary (their cache keys): the row says "
                                  "readNativeVerdict is restated, but the judge imports it"], stderr


def test_lines_inserted_above_the_cited_passages_pass(copy: Path):
    """A line shift alone: lines at the top of every cited file and directly above every cited passage that
    does not lie inside another (a line inserted there is a change to the outer passage's text)."""
    check = family.Check(copy)
    spans: dict[str, list[tuple[int, int]]] = {}
    for r in family.ROWS:
        if r["kind"] == "restated":
            found = check.declaration(r["predicate"], r["rel"], r["predicate"].split(" ")[0])
            span = found[:2] if found else None
        elif r["passage"]:
            span = check.passage(r["predicate"], r["rel"], r["passage"]) if r["kind"] == "inline" else (
                (lambda at: (at, at))(check.anchor_line(r["predicate"], r["rel"], r["passage"]["from"])))
        else:
            continue
        assert span is not None and span[0] is not None, check.problems
        spans.setdefault(r["rel"], []).append(span)
        for spec in r["also"]:
            widening = check.passage(r["predicate"], spec["rel"], spec)
            assert widening is not None, check.problems
            spans.setdefault(spec["rel"], []).append(widening)
    inserted = 0
    for rel in family.cited_files():
        path = copy / WEB / rel
        lines = path.read_text(encoding="utf-8").split("\n")
        own = spans.get(rel, [])
        starts = {s for s, _ in own if not any(o < s <= e for o, e in own)}
        out = ["// shifted", "// shifted", ""]
        for i, line in enumerate(lines):
            if i in starts:
                out += ["// shifted above a cited passage", ""]
                inserted += 1
            out.append(line)
        path.write_text("\n".join(out), encoding="utf-8")
    assert inserted >= 15
    code, stderr = run(copy)
    assert code == 0, stderr


def test_a_removed_cited_file_fails_and_says_it_is_missing(copy: Path):
    (copy / WEB / "features/workspace/resultsSessionState.ts").unlink()
    code, stderr = run(copy)
    assert code == 1
    assert "MISMATCH: the solver mode: its cited file apps/desktop/src/features/workspace/resultsSessionState.ts is missing" in stderr


def test_the_registered_selection_covers_every_cited_file():
    """The table's files (and the digests file's) select the registered check, which runs this test."""
    workflow = json.loads(WORKFLOW.read_text(encoding="utf-8"))
    assert workflow["checks"]["after-worker-family-table"]["command"][-1] == "tests/test_after_worker_family_table.py"
    rules = [rule for rule in workflow["path_rules"] if "after-worker-family-table" in rule["checks"]]
    assert len(rules) == 1
    paths = set(rules[0]["paths"])
    digests = json.loads(family.DIGESTS.read_text(encoding="utf-8"))
    from_digests = {f"{WEB}/{part.split(':', 1)[0]}" for entry in digests.values() for part in entry["source"].split("; with ")}
    assert from_digests <= set(CITED)
    assert set(CITED) <= paths
    assert {"tools/after_worker_family/**", "apps/desktop/src/test-support/after-worker/**",
            "tests/test_after_worker_family_table.py", "software-workflow.json"} <= paths
