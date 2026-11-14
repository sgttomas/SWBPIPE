"""The pressure-elongation inventory's closure (DEL-04-07): every tracked file under the scanned roots that names
`pressure_regions` is claimed exactly once, and each way of breaking that fails by name.

The two misses this closure exists for are re-created by removing their claims: the stress benchmark builds its
documents in Rust (validation/benchmarks/stress/src/lib.rs), and the K-D5 M3-T frozen subset is JSON Lines
(validation/references/kd5_timoshenko/frozen/kd5t_frozen_subset.jsonl). The committed claims are used unchanged
otherwise, over one scan of the repository.
"""
from __future__ import annotations

import copy
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[2]
if str(ROOT) not in sys.path:
    sys.path.append(str(ROOT))
from tools.fixtures import retained_precision_corpus as CORPUS  # noqa: E402
from tools.validation import pressure_elongation as PE  # noqa: E402
from tools.validation import pressure_elongation_inventory as INV  # noqa: E402

STRESS = "validation/benchmarks/stress/src/lib.rs"
KD5 = "validation/references/kd5_timoshenko/frozen/kd5t_frozen_subset.jsonl"
NOT_NAMING = "tools/validation/pressure_elongation_anchor.py"


@pytest.fixture(scope="module")
def scanned():
    return INV.scan()


def problems(scanned, **claims) -> str:
    with pytest.raises(INV.InventoryError) as raised:
        INV.build(scanned=scanned, **claims)
    return str(raised.value)


def without(claims: list[dict], path: str) -> list[dict]:
    kept = [c for c in claims if path not in c["paths"]]
    assert len(kept) == len(claims) - 1, path
    return kept


# ----------------------------------------------------------------------------- the committed inventory
def test_the_inventory_is_current(scanned):
    assert INV.OUTPUT.read_text(encoding="utf-8") == INV.dumps(INV.build(scanned=scanned)), \
        "run python3 tools/validation/pressure_elongation_inventory.py and classify every new file naming pressure_regions"


def test_every_scanned_file_is_claimed_once_and_the_scan_reaches_every_type(scanned):
    files, data = scanned
    inventory = INV.build(scanned=scanned)
    assert inventory["scan"]["files"] == len(files) == sum(inventory["scan"]["claimed_by"].values())
    assert {Path(f).suffix for f in files} >= {".json", ".jsonl", ".rs", ".py", ".ts", ".tsx", ".md"}
    assert not any(f.startswith("execution/") for f in files)
    in_code = {p for entry in inventory["in_code_constructions"] for p in entry["paths"]}
    exempt = {p for entry in inventory["exemptions"] for p in entry["files"]}
    grouped = {entry["path"] for entry in inventory["entries"]}
    assert not (in_code & exempt) and not (in_code & grouped) and not (exempt & grouped)
    assert in_code | exempt | grouped == set(files)


# ----------------------------------------------------------------------------- the two known misses
def test_the_stress_benchmark_built_in_rust_is_caught_when_unclaimed(scanned):
    assert STRESS in scanned[0]
    found = problems(scanned, in_code=without(INV.IN_CODE, STRESS))
    assert f"  unclaimed: {STRESS}" in found


def test_the_kd5_json_lines_subset_is_caught_when_unclaimed(scanned):
    assert KD5 in scanned[0]
    groups = [g for g in INV.GROUPS if g["id"] != "kd5_timoshenko_frozen"]
    assert len(groups) == len(INV.GROUPS) - 1
    found = problems(scanned, groups=groups)
    assert f"  unclaimed: {KD5}" in found
    # Nor can it be exempted: it holds pressurised documents.
    found = problems(scanned, groups=groups, exempt=[*INV.EXEMPT, {"paths": [KD5], "reason": "an attempt"}])
    assert f"{KD5} holds a pressurised document; claim it with a group" in found


def test_the_kd5_subset_is_twelve_pressurised_documents_naming_no_setting(scanned):
    [entry] = [e for e in INV.build(scanned=scanned)["entries"] if e["path"] == KD5]
    assert entry["group"] == "kd5_timoshenko_frozen" and entry["documents"] == []
    assert entry["anchor"]["kind"] == "excluded" and entry["producer"]["status"] == "awaiting_m3_restoration"
    held = entry["held_documents"]
    assert len(held) == 12 and all(len(d["pressurised_load_cases"]) == 1 for d in held)
    lines = (ROOT / KD5).read_text(encoding="utf-8").splitlines()
    nonzero = 0
    for record in held:
        doc = json.loads(lines[record["line"] - 1])
        for part in record["pointer"].split("/")[1:]:
            doc = doc[part]
        assert "pressure_elongation" not in (doc.get("model_settings") or {}), record
        nonzero += sum(1 for case in doc["load_cases"] for region in case.get("pressure_regions") or []
                       if PE._nonzero(region.get("pressure")))
    assert nonzero == 12


# ----------------------------------------------------------------------------- each way of breaking the closure
def test_a_stale_claim_fails(scanned):
    assert NOT_NAMING not in scanned[0]
    entry = {"paths": [NOT_NAMING], "expectations": "closed_form", "cases_named": "all", "detail": "a stale claim"}
    assert f"stale: IN_CODE path {NOT_NAMING} is not a tracked file naming pressure_regions" in \
        problems(scanned, in_code=[*INV.IN_CODE, entry])
    assert "stale: exemption path tools/no_such_dir/* matches no tracked file naming pressure_regions" in \
        problems(scanned, exempt=[*INV.EXEMPT, {"paths": ["tools/no_such_dir/*"], "reason": "a stale pattern"}])
    groups = copy.deepcopy(INV.GROUPS)
    groups[0]["paths"].append("validation/references/no_such_reference/*.json")
    assert f"stale: group {groups[0]['id']} path validation/references/no_such_reference/*.json" in \
        problems(scanned, groups=groups)


def test_an_exemption_without_a_reason_fails(scanned):
    exempt = copy.deepcopy(INV.EXEMPT)
    for blank in ("", "  ", None):
        exempt[0]["reason"] = blank
        assert "exemption #0 (" in problems(scanned, exempt=exempt) and ": no reason" in problems(scanned, exempt=exempt)


def test_an_in_code_entry_must_say_what_it_rests_on(scanned):
    in_code = copy.deepcopy(INV.IN_CODE)
    [stress] = [e for e in in_code if STRESS in e["paths"]]
    stress.update(expectations="trust", cases_named="most", detail=" ")
    found = problems(scanned, in_code=in_code)
    assert "expectations must be one of" in found and "cases_named must be one of" in found and "no detail" in found


def test_a_file_claimed_twice_fails(scanned):
    found = problems(scanned, exempt=[*INV.EXEMPT, {"paths": [STRESS], "reason": "a second claim"}])
    assert f"  claimed twice: {STRESS} (IN_CODE {STRESS}; exemption #" in found


def test_an_in_code_entry_cannot_claim_a_data_file(scanned):
    data_file = "validation/references/t4_u5a/requests/J3-0.request.json"
    entry = {"paths": [data_file], "expectations": "reference_file", "cases_named": "all", "detail": "wrong kind"}
    assert f"IN_CODE path {data_file} is a data file" in problems(scanned, in_code=[*INV.IN_CODE, entry])


# ----------------------------------------------------------------------------- documents the rule reads
def test_documents_are_read_in_json_text_json_lines_and_dag_storage():
    doc = {"schema_version": "0.9.0", "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3"},
           "load_cases": [{"id": "case:p", "pressure_regions": [{"id": "r", "pressure": {"value": 1.0, "unit": "Pa"}}]}]}
    record = {"schema_version": "0.9.0", "pressurised_load_cases": ["case:p"]}
    assert INV.data_documents("a.json", json.dumps({"model": doc})) == ([{"pointer": "/model", **record}], [])
    text = json.dumps({"requests": [{"args": {"modelJson": json.dumps({"wrapped": json.dumps(doc)})}}]})
    assert INV.data_documents("a.json", text) == ([], [{"pointer": "/requests/0/args/modelJson",
                                                        "text_pointers": ["/wrapped", ""], **record}])
    lines = "\n".join([json.dumps({"tag": 1, "request": {"model": {**doc, "load_cases": []}}}), "",
                       json.dumps({"tag": 2, "request": {"model": doc}})])
    assert INV.data_documents("a.jsonl", lines) == ([], [{"line": 3, "pointer": "/request/model", **record}])
    dag = CORPUS.compact((json.dumps({"entries": [{"request": {"model": doc}}]}, indent=2) + "\n").encode())
    assert INV.data_documents("a.json", json.dumps(dag)) == \
        ([], [{"pointer": "/entries/0/request/model", **record, "storage": CORPUS.FORMAT}])


def resolve(value, pointer: str):
    for part in pointer.split("/")[1:] if pointer else []:
        part = part.replace("~1", "/").replace("~0", "~")
        value = value[int(part)] if isinstance(value, list) else value[part]
    return value


def held_document(path: str, record: dict) -> dict:
    """The document a held record locates."""
    text = (ROOT / path).read_text(encoding="utf-8")
    container = json.loads(text.splitlines()[record["line"] - 1] if "line" in record else text)
    if "storage" in record:
        container = CORPUS.expand(container)
    value = resolve(container, record["pointer"])
    for pointer in record.get("text_pointers", []):
        value = resolve(json.loads(value), pointer)
    return value


def test_held_documents_found_in_the_repository_are_recorded_where_they_are(scanned):
    """The files whose pressurised documents only this scan reads, beside the JSON Lines subset. Each record locates a
    document with exactly the recorded pressurised cases; each case is named, except the MBF parity vectors', which
    are never solved."""
    entries = {e["path"]: e for e in INV.build(scanned=scanned)["entries"]}
    for path, named in (("apps/desktop/src/test-support/native-replay/native_replay_requests.json", True),
                        ("apps/desktop/src/test-support/analysis-run-v03/analysis_run_v03_cases.json", True),
                        ("fixtures/results/pressure_source_reader_corpus.json", True),
                        ("fixtures/caepipe_mbf/invented/parity/writer_vectors.json", False),
                        ("fixtures/caepipe_mbf/invented/parity/import_vectors.json", False)):
        entry = entries[path]
        assert entry["held_documents"] and entry["anchor"]["kind"] == "excluded", path
        for record in entry["held_documents"]:
            doc = held_document(path, record)
            assert PE.pressurised_case_ids(doc) == record["pressurised_load_cases"], (path, record)
            listed = ((doc.get("model_settings") or {}).get("pressure_elongation") or {}).get("operating_load_cases")
            assert (set(record["pressurised_load_cases"]) <= set(listed or [])) is named, (path, record)


# ----------------------------------------------------------------------------- what the scan does not reconstruct
PARITY_READERS = r"parity/(writer|import)_vectors|caepipe_mbf_(writer|import)_vectors"
SOLVE_ENTRIES = re.compile(r"run_linear_static_preview|open_pipe_stress_product_physics|open_pipe_stress_headless_runner"
                           r"|openpipestress-runner|OPENPIPESTRESS_RUNNER_BIN")
CODE = ("*.rs", "*.py", "*.ts", "*.tsx", "*.js", "*.mjs")


def test_nothing_that_reads_the_mbf_parity_vectors_can_solve_them():
    """The writer parity vectors hold many pressurised models as delta-assembled blobs the scan does not reconstruct
    (the module docstring). They are claimed as MBF writer inputs that nothing solves. This keeps that true: every
    tracked code file naming the vectors or their harvest modules, and the Cargo manifest of a Rust one's crate, names
    no mechanics solve entry (the product's preview entry, the product or runner crate, or the runner binary)."""
    done = subprocess.run(["git", "grep", "-l", "-E", PARITY_READERS, "--", *CODE], cwd=ROOT, capture_output=True,
                          text=True, check=True)
    own = {"tools/validation/pressure_elongation_inventory.py", "tools/validation/test_pressure_elongation_inventory.py"}
    readers = sorted(set(done.stdout.split()) - own)
    assert "tools/fixtures/caepipe_mbf_writer_vectors.py" in readers and \
        "core/handoff/caepipe_mbf_engine/tests/writer_parity.rs" in readers, readers
    for path in readers:
        checked = [path]
        if path.endswith(".rs"):
            crate = (ROOT / path).parent
            while not (crate / "Cargo.toml").exists():
                crate = crate.parent
            checked.append(str((crate / "Cargo.toml").relative_to(ROOT)))
        for name in checked:
            found = SOLVE_ENTRIES.search((ROOT / name).read_text(encoding="utf-8"))
            assert found is None, f"{name} (read for {path}) names {found.group(0)}: the parity vectors may now be solved"


# ----------------------------------------------------------------------------- pins on individual files
# A fixed table in a consumer that requires the current bytes of the files it names to have a recorded SHA-256. Each
# entry reads the paths the table names from the consumer's text: which files, never whether the digests agree. The
# registration a pinned file needs therefore stays required while the consumer's digest is stale (the failure this
# exists for: the three mixed source requests changed and the O4 twins' table did not), and only moving the table or
# the file out of it removes it. The list is the tables the independent review found on inventoried files; a digest
# held anywhere else is not covered.
def _block(text: str, start: str) -> str:
    """The text from the line starting `start` to the next line that closes it at the left margin."""
    found = re.search(rf"^{start}.*?^[\]\}}]", text, re.M | re.S)
    assert found, start
    return found.group(0)


def _relative(consumer: str, path: str) -> str:
    return str((ROOT / consumer).parent.joinpath(path).resolve().relative_to(ROOT))


def _twins(text):
    tables = re.findall(r"^PINS(?:_ADMITTED)? = \{(.*?)^\}", text, re.M | re.S)
    return {path for table in tables for path in re.findall(r'"([^"]+)":\s*"[0-9a-f]{64}"', table)}


def _source_basis(text):
    family = {"Physics": "physics_source", "LoadReference": "load_reference_source"}
    rows = re.findall(r'\(Family::(\w+), "([^"]+)", "[0-9a-f]{64}", PreviewSolverMode::(\w+),',
                      _block(text, "const CURRENT_SOURCE_BASIS"))
    mode = {"SparseInteractive": "sparse_interactive", "DenseScrutiny": "dense_scrutiny"}
    base = "fixtures/product_preview/pressure_source"
    return ({f"{base}/{family[f]}/{name}.request.json" for f, name, _ in rows}
            | {f"{base}/{family[f]}/{name}-{mode[m]}.raw.json" for f, name, m in rows})


def _u4a(text):
    assert re.search(r"^U4A_REQUESTS_SHA256 = \"[0-9a-f]{64}\"", text, re.M)
    return {f"validation/references/t4_u4a/{name}"
            for name in re.findall(r'os\.path\.join\(U4A_DIR, "([^"]+)"\)', _block(text, "def u4a_cross_check"))}


def _u5a(text):
    return {f"validation/references/t4_u5a/{name}"
            for name in re.findall(r'"([^"]+)":\s*"[0-9a-f]{64}"', _block(text, "U5A_PINNED = "))}


def _included(consumer, start, macro="include_str"):
    return lambda text: {_relative(consumer, p) for p in re.findall(rf'{macro}!\("([^"]+)"\)', _block(text, start))}


def _census(text):
    assert 'pin["inputs_sha256"], sha(INPUTS.as_bytes())' in text
    return {_relative(J7_CENSUS, p) for p in re.findall(r'^const INPUTS: &str = include_str!\("([^"]+)"\)', text, re.M)}


def _eigen_motion(text):
    assert re.search(r'hex, EIGEN_MOTION_SHA256', text)
    return {_relative("core/product_physics/Cargo.toml", p.lstrip("/"))  # relative to CARGO_MANIFEST_DIR
            for p in re.findall(r'const EIGEN_MOTION_REQUEST: &str = concat!\(\s*env!\("CARGO_MANIFEST_DIR"\),\s*"([^"]+)"',
                                text)}


def _witness_inputs(text):
    names = re.findall(r'\("(\w+)", (\w+_EXACT), "[0-9a-f]{64}"\)', _block(text, "fn b3b_witness_inputs_and_routes"))
    return {_relative(FACADE, path) for name, const in names
            for path in re.findall(rf'^const {const}: &str = include_str!\("([^"]+)"\)', text, re.M)}


def _b3_successors(text):
    assert re.search(r"hashlib\.sha256\(raw\)\.hexdigest\(\) == EXACT_PINNED\[mode\]\[0\]", text)
    template = re.search(r'ROOT / f"(fixtures/results/retained_precision_exact_successor_\{mode\}\.json)"', text).group(1)
    return {template.replace("{mode}", mode) for mode in re.findall(r'"(\w+)": \("[0-9a-f]{64}"', _block(text, "EXACT_PINNED = "))}


def _golden(text):
    assert "expect(sha256(bytes)).toBe(golden.successorSha256)" in text and "expect(sha256(goldenText)).toBe(golden.sha256)" in text
    return set(re.findall(r'(?:successor|path): "([^"]+)"', _block(text, "const EXACT_GOLDEN")))


def _frozen(text):
    return set(re.findall(r'"([^"]+)":\s*"[0-9a-f]{64}"', _block(text, "FROZEN = ")))


TWINS = "validation/references/o4_retained_source/generators/o4_source_twins.py"
J7_CENSUS = "core/product_physics/src/source_recovery/j7_rf_census.rs"
FACADE = "core/product_physics/src/retained_facade_tests.rs"
J7_I12 = "core/product_physics/src/source_recovery/j7_t4_consequences.rs"
CONTRACT = "core/reporting/result_export/tests/retained_precision_contract.rs"
DERIVATIVE = "core/reporting/result_export/tests/retained_precision_derivative_golden.rs"
# (consumer, table, the paths it names, how many it names, how many of those the inventory holds)
PIN_TABLES = [
    # tools/fixtures/retained_precision_corpus.py names no pressure_regions
    (TWINS, "PINS, PINS_ADMITTED", _twins, 14, 13),
    # the 24 committed raw outputs name no pressure_regions
    ("core/product_physics/tests/pressure_source_join.rs", "CURRENT_SOURCE_BASIS", _source_basis, 36, 12),
    (FACADE, "b3b_witness_inputs_and_routes", _witness_inputs, 3, 3),
    ("core/product_physics/tests/load_reference_state_runtime_extension.rs", "EIGEN_MOTION_SHA256", _eigen_motion, 1, 1),
    # oracle.json names no pressure_regions
    ("validation/references/t4_u4b/generators/u4b_reference.py", "U4A_REQUESTS_SHA256", _u4a, 2, 1),
    ("validation/references/t4_u5b/generators/u5b_reference.py", "U5A_PINNED", _u5a, 10, 10),
    (J7_CENSUS, "inputs_sha256", _census, 1, 1),
    (J7_I12, "request", _included(J7_I12, "fn request", "include_bytes"), 1, 1),
    # the four raw outputs name no pressure_regions
    ("tests/test_load_reference_readers.py", "FROZEN", _frozen, 6, 2),
    (FACADE, "EXACT_PINNED", _included(FACADE, "fn b3b_exact_successor_fixtures_are_the_live_successors"), 2, 2),
    (CONTRACT, "M3X", _included(CONTRACT, "const M3X"), 2, 2),
    (DERIVATIVE, "EXACT_PINNED", _included(DERIVATIVE, "const EXACT_PINNED"), 2, 2),
    ("tests/test_retained_precision_b3.py", "EXACT_PINNED", _b3_successors, 2, 2),
    # the two derivative goldens name no pressure_regions
    ("apps/desktop/src/features/result-export/retainedPrecisionResultExport.test.tsx", "EXACT_GOLDEN", _golden, 4, 2),
]


def _names(pin: str, consumer: str) -> bool:
    """A registered pin names `consumer`: the bare path (a group's) or the path and its table."""
    return pin == consumer or pin.startswith(consumer + " ")


def registrations(inventory: dict) -> dict[str, list[str]]:
    """Every inventoried file's registered pins, whether its claim is a group or an exemption."""
    named = {e["path"]: e["pinned_by"] for e in inventory["entries"]}
    for record in inventory["exemptions"]:
        named.update({path: record.get("pinned_by", {}).get(path, []) for path in record["files"]})
    return named


def unregistered(inventory: dict, root: Path = ROOT) -> list[tuple[str, str]]:
    """(file, consumer) for every inventoried file a PIN_TABLES table names whose registration does not name that
    consumer, reading the tables' consumers under `root`."""
    named = registrations(inventory)
    missing = []
    for consumer, _, paths, _, _ in PIN_TABLES:
        for path in sorted(paths((root / consumer).read_text(encoding="utf-8")) & set(named)):
            if not any(_names(pin, consumer) for pin in named[path]):
                missing.append((path, consumer))
    return missing


@pytest.mark.parametrize("consumer, table, paths, total, held", PIN_TABLES, ids=[f"{c.rsplit('/', 1)[1]}:{t}" for c, t, *_ in PIN_TABLES])
def test_every_inventoried_file_a_fixed_pin_table_names_names_that_table(scanned, consumer, table, paths, total, held):
    """A change to a pinned file without moving its pin fails the consumer's check, so every pinned file the inventory
    holds must say which consumer pins it, whether its claim is a group or an exemption. The counts keep the reading
    of each table honest: a table whose paths can no longer be read fails here, not silently."""
    pinned = paths((ROOT / consumer).read_text(encoding="utf-8"))
    assert len(pinned) == total and all((ROOT / p).exists() for p in pinned), sorted(pinned)
    named = registrations(INV.build(scanned=scanned))
    inventoried = sorted(pinned & set(named))
    assert len(inventoried) == held, inventoried
    for path in inventoried:
        assert any(_names(pin, consumer) for pin in named[path]), (path, consumer, table)


def test_a_pinned_file_changed_under_a_stale_table_still_needs_its_registration(scanned, tmp_path):
    """The failure that happened, in a scratch copy: an inventoried input changes and its consumer's table does not, so
    the consumer's digest no longer matches. Its registration is still demanded, and only the registration decides."""
    changed = "fixtures/product_preview/pressure_source/physics_source/mixed.request.json"
    consumer = "core/product_physics/tests/pressure_source_join.rs"
    for path in {c for c, *_ in PIN_TABLES} | {changed}:
        (tmp_path / path).parent.mkdir(parents=True, exist_ok=True)
        (tmp_path / path).write_bytes((ROOT / path).read_bytes())
    document = json.loads((tmp_path / changed).read_text(encoding="utf-8"))
    del document["model"]["model_settings"]["pressure_elongation"]  # the change that moved it, in reverse
    (tmp_path / changed).write_text(json.dumps(document, indent=1) + "\n", encoding="utf-8")
    recorded = re.search(r'\(Family::Physics, "mixed", "([0-9a-f]{64})"', (tmp_path / consumer).read_text()).group(1)
    assert hashlib.sha256((tmp_path / changed).read_bytes()).hexdigest() != recorded  # the table is now stale
    assert unregistered(INV.build(scanned=scanned), tmp_path) == []
    dropped = [e for e in INV.PINNED_BY if not any(p.startswith(consumer) for p in e["pinned_by"])]
    assert len(dropped) == len(INV.PINNED_BY) - 1
    assert (changed, consumer) in unregistered(INV.build(scanned=scanned, pinned=dropped), tmp_path)


def test_a_stale_pin_fails(scanned):
    stale = [*INV.PINNED_BY, {"paths": ["fixtures/no_such_fixture.json"], "pinned_by": ["somewhere"]}]
    assert "stale: PINNED_BY path fixtures/no_such_fixture.json matches no tracked file naming pressure_regions" in \
        problems(scanned, pinned=stale)
