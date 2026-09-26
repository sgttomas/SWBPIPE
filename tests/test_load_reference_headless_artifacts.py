"""T1 WP4 consumers of the actual headless 0.4.0 artifacts.

The Rust lanes ``core/runner/headless/src/load_reference_route_tests.rs`` and
``core/runner/headless/tests/load_reference_cli.rs`` write the actual request,
invocation, raw, (for load-reference-1) canonical document and manifest files,
and the actual CLI input and stdout, when these variables are set:

- ``HEADLESS_LOAD_REFERENCE_OUTPUT_DIR``: load-reference-1, the two committed
  load-reference requests in both modes;
- ``HEADLESS_LOAD_REFERENCE_SOURCE_OUTPUT_DIR``: load-reference-source-1, the
  five committed joined witnesses in both modes.

In T1 the joined route is never numerically eligible (T1_WAVE1_RULINGS.md
section 7), so the headless binding builds no joined canonical document; its
artifacts carry no document file. These tests read the artifacts; they skip only
when the variable is unset and must be run with it set. All inputs are invented.
"""
from __future__ import annotations

import json
import os
from pathlib import Path

import pytest

from core.analysis_runs.compatibility import (
    LOAD_REFERENCE_CONTRACT_ID, LOAD_REFERENCE_CONTRACT_SHA256, LOAD_REFERENCE_SOURCE_CONTRACT_ID,
    LOAD_REFERENCE_SOURCE_CONTRACT_SHA256, _source_contract, build_analysis_run_v0_3,
    numerical_use_standing, validate_analysis_run_v0_3, verify_analysis_run_record,
)
from core.analysis_runs.load_reference_source import validate_load_reference_source_evidence
from core.analysis_runs.source_blocks import domain_hash
from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1
from schema_validation import validate_instance

PROJECT = Path(__file__).resolve().parents[1]
RESULTS_SCHEMA = PROJECT / "schemas/results.v0.3.schema.yaml"
MODES = {"sparse": "sparse_interactive", "dense": "dense_scrutiny"}
LR_STEMS = ["connected", "pressure"]
LRS_STEMS = ["n05", "n06", "fields", "mixed", "eigen_motion"]
LR_NAMES = [(stem, short) for stem in LR_STEMS for short in MODES]
LRS_NAMES = [(stem, short) for stem in LRS_STEMS for short in MODES]
LR_SUFFIXES = {"request", "invocation", "raw", "document", "manifest"}
LRS_SUFFIXES = {"request", "invocation", "raw", "manifest"}


def _folder(var: str) -> Path:
    folder = os.environ.get(var)
    if not folder:
        pytest.skip(f"set {var} and run the focused Rust lane to produce actual headless artifacts")
    return Path(folder)


def _load(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def _bases(raw):
    return [case["basis_ref"] for case in raw["numerical_quality"]["cases"]]


def _results_branches():
    document = json.loads(RESULTS_SCHEMA.read_text(encoding="utf-8"))
    return document, document["$defs"]["ResultEnvelope"]["oneOf"]


def _matching_result_branches(instance):
    document, branches = _results_branches()
    found = []
    for index, branch in enumerate(branches):
        alone = {"$schema": document["$schema"], "$id": document["$id"], "$defs": document["$defs"], **branch}
        try:
            validate_instance(alone, instance["result_envelope"], instance_label=f"branch {index}")
        except AssertionError:
            continue
        found.append(index)
    return found


def _branch_of(contract_id):
    _, branches = _results_branches()
    ids = [branch["properties"]["producer"]["properties"]["semantic_contract_id"]["const"] for branch in branches]
    return ids.index(contract_id)


def _files(folder: Path, name: str):
    return {path.name[len(name) + 1:-len(".json")] for path in folder.glob(f"{name}.*.json")}


def _common(folder: Path, stem: str, short: str, request_path: str, raw_path: str, suffixes: set[str]):
    mode = MODES[short]
    name = f"{stem}-{mode}"
    assert _files(folder, name) == suffixes, f"{name}: {sorted(_files(folder, name))}"
    request = _load(folder / f"{name}.request.json")
    invocation = _load(folder / f"{name}.invocation.json")
    raw = _load(folder / f"{name}.raw.json")
    manifest = _load(folder / f"{name}.manifest.json")
    # The actual invocation is the committed request, unchanged, and its mode.
    assert request == _load(PROJECT / request_path.format(stem=stem))
    assert invocation == {"request": request, "solver_mode": mode}
    # The actual producer output is the committed producer envelope.
    assert raw == _load(PROJECT / raw_path.format(stem=stem, mode=mode))
    for suffix in suffixes - {"manifest"}:
        assert manifest[f"{suffix}_digest"] == canonical_sha256_checked_v1(_load(folder / f"{name}.{suffix}.json")), suffix
    assert manifest["source"] == "actual headless Value invocation; no native UI claim"
    # The actual CLI process returned the same raw inside the ControlledExport wrapper.
    cli_input = _load(folder / f"cli-{name}.input.json")
    assert cli_input["solve"]["preview_model"] == request
    controlled = _load(folder / f"cli-{name}.output.json")
    assert controlled["blocked"] is False and controlled["summary"]["blocking_count"] == 0
    output = controlled["payload"]
    assert output["command"] == "solve" and output["diagnostics"] == []
    assert output["mechanics_envelope"] == raw
    assert "result_envelope_document" not in output and "qualified_preview_evidence" not in output
    return name, request, invocation, raw


def _committed_carrier(path: str):
    """The committed carrier as a JSON value. The artifacts are written through
    a serde Value, so their key order is not the producer's; values compare."""
    return _load(PROJECT / path)


@pytest.mark.parametrize(("stem", "short"), LR_NAMES, ids=[f"{s}-{m}" for s, m in LR_NAMES])
def test_actual_load_reference_one_document_selects_its_branch_and_analysis_run_matches(stem, short):
    folder = _folder("HEADLESS_LOAD_REFERENCE_OUTPUT_DIR")
    name, request, invocation, raw = _common(
        folder, stem, short,
        "fixtures/product_preview/load_reference/{stem}.request.json",
        "fixtures/product_preview/load_reference/{stem}-{mode}.raw.json",
        LR_SUFFIXES,
    )
    document = _load(folder / f"{name}.document.json")
    # Schema: the whole results 0.3 carrier, on the load-reference-1 branch only.
    validate_instance(json.loads(RESULTS_SCHEMA.read_text(encoding="utf-8")), document, instance_label=f"actual headless {name}")
    assert _matching_result_branches(document) == [_branch_of(LOAD_REFERENCE_CONTRACT_ID)]
    envelope = document["result_envelope"]
    assert envelope["semantic_contract_ref"] == {"ref_type": "semantic_contract", "ref_id": LOAD_REFERENCE_CONTRACT_ID}
    for key in ("producer", "numerical_quality", "formulation_basis", "contract_evidence"):
        assert envelope[key] == raw[key], key
    assert "source_block_recovery" not in envelope and "source_block_recovery" not in raw
    assert len(envelope["row_accounting"]) == len(raw["results"])
    # Reader and standing.
    assert _source_contract(raw)[0] == LOAD_REFERENCE_CONTRACT_ID
    assert numerical_use_standing(raw, _bases(raw)) == "numerically_eligible"
    # AnalysisRun: verified, bound to the same received bytes as the document,
    # and equal to the committed carrier for the same producer bytes.
    carrier = f"{stem}-{short}"
    record = build_analysis_run_v0_3(raw, input_manifest_ref={"object_type": "InputManifest", "ref": f"manifest:load-reference-{carrier}"}, input_manifest_hash="1" * 64)
    validate_analysis_run_v0_3(record, raw)
    assert verify_analysis_run_record(record) == "match"
    run = record["analysis_run"]
    assert run["reproducibility"]["semantic_contract"] == {"id": LOAD_REFERENCE_CONTRACT_ID, "sha256": LOAD_REFERENCE_CONTRACT_SHA256}
    received = next(item for item in run["hashes"] if item["payload_scope"] == "received_result")
    origin = envelope["reproducibility"]["source_origin_bindings"][0]
    assert received["value"] == origin["received_carrier_checksum"]["value"] == canonical_sha256_checked_v1(raw)
    assert origin["origin_class"] == "attested_headless_producer"
    assert record == _committed_carrier(f"fixtures/results/load_reference_{stem}_{short}.analysis_run.json")


@pytest.mark.parametrize(("stem", "short"), LRS_NAMES, ids=[f"{s}-{m}" for s, m in LRS_NAMES])
def test_actual_joined_receipt_is_invocation_bound_and_never_eligible_without_a_document(stem, short):
    folder = _folder("HEADLESS_LOAD_REFERENCE_SOURCE_OUTPUT_DIR")
    name, request, invocation, raw = _common(
        folder, stem, short,
        "fixtures/product_preview/load_reference_source/{stem}.request.json",
        "fixtures/product_preview/load_reference_source/{stem}-{mode}.raw.json",
        LRS_SUFFIXES,
    )
    assert not (folder / f"{name}.document.json").exists(), "T1 builds no joined headless document"
    # The retained receipt binds the actual invocation, not the other mode's.
    receipt = raw["source_block_recovery"]
    assert receipt["body"]["invocation"]["value"] == domain_hash("source_blocks_invocation_v1", invocation)
    other = {"request": request, "solver_mode": next(m for m in MODES.values() if m != invocation["solver_mode"])}
    assert receipt["body"]["invocation"]["value"] != domain_hash("source_blocks_invocation_v1", other)
    # The joined reader admits the received bytes and never grants eligibility.
    assert validate_load_reference_source_evidence(raw) is False
    assert _source_contract(raw)[0] == LOAD_REFERENCE_SOURCE_CONTRACT_ID
    assert numerical_use_standing(raw, _bases(raw)) == "needs_recompute"
    carrier = f"{stem}-{short}"
    record = build_analysis_run_v0_3(raw, input_manifest_ref={"object_type": "InputManifest", "ref": f"manifest:load-reference-source-{carrier}"}, input_manifest_hash="1" * 64)
    validate_analysis_run_v0_3(record, raw)
    assert verify_analysis_run_record(record) == "match"
    run = record["analysis_run"]
    assert run["reproducibility"]["semantic_contract"] == {"id": LOAD_REFERENCE_SOURCE_CONTRACT_ID, "sha256": LOAD_REFERENCE_SOURCE_CONTRACT_SHA256}
    assert run["source_block_recovery"] == receipt and run["contract_evidence"] == raw["contract_evidence"]
    assert record == _committed_carrier(f"fixtures/results/load_reference_source_{stem}_{short}.analysis_run.json")


def test_artifact_folders_hold_exactly_the_expected_cases():
    lr = _folder("HEADLESS_LOAD_REFERENCE_OUTPUT_DIR")
    lrs = _folder("HEADLESS_LOAD_REFERENCE_SOURCE_OUTPUT_DIR")
    expected_lr = {f"{stem}-{MODES[short]}" for stem, short in LR_NAMES}
    expected_lrs = {f"{stem}-{MODES[short]}" for stem, short in LRS_NAMES}
    assert {p.name.split(".")[0] for p in lr.glob("*.raw.json")} == expected_lr
    assert {p.name.split(".")[0] for p in lrs.glob("*.raw.json")} == expected_lrs
    assert {p.name[4:].split(".")[0] for p in lr.glob("cli-*.output.json")} == expected_lr
    assert {p.name[4:].split(".")[0] for p in lrs.glob("cli-*.output.json")} == expected_lrs
    assert not list(lrs.glob("*.document.json"))
