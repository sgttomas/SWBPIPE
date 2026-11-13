"""The pressure-elongation inventory's closure (DEL-04-07): every tracked file under the scanned roots that names
`pressure_regions` is claimed exactly once, and each way of breaking that fails by name.

The two misses this closure exists for are re-created by removing their claims: the stress benchmark builds its
documents in Rust (validation/benchmarks/stress/src/lib.rs), and the K-D5 M3-T frozen subset is JSON Lines
(validation/references/kd5_timoshenko/frozen/kd5t_frozen_subset.jsonl). The committed claims are used unchanged
otherwise, over one scan of the repository.
"""
from __future__ import annotations

import copy
import json
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
