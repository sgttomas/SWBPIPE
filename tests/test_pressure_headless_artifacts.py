"""T4-U2 phase 2: consumers of the actual headless pressure-2 export artifacts.

The Rust lane ``core/runner/headless/tests/pressure_export.rs`` writes each
case's raw producer envelope and canonical export document when
``HEADLESS_PRESSURE_OUTPUT_DIR`` is set. These tests read them: the document
validates against the results 0.3 schema on the pressure-2 branch only; the
test-only verifier admits the raw envelope and its transported statements;
and Python product paths take the Rust reader's pressure route: the raw
envelope is read, a transported statement is refused by name and no 0.3
AnalysisRun record is built from pressure-2. They skip only when the variable
is unset and must be run with it set. All inputs are invented.
"""
from __future__ import annotations

import json
import os
from pathlib import Path

import pytest

from core.analysis_runs import rust_reader
from core.analysis_runs.compatibility import PRESSURE_CONTRACT_ID as PRODUCT_PRESSURE_ID, _source_contract, build_analysis_run_v0_3, numerical_use_standing
from schema_validation import envelope_branches, validate_instance
from tests.verifiers.pressure_2 import CONTRACT_ID as PRESSURE_CONTRACT_ID, verify_source

# b7-plan R2: Python product paths read through the Rust verdict's pressure
# route, which has no transport reader.
PRESSURE_TRANSPORT_REFUSAL = "SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED"

PROJECT = Path(__file__).resolve().parents[1]
STEMS = ["arc-u2-l-anch-ptw-k2-0-3-0", "arc-u2-l-anch-ptw-k2-0-4-0", "arc-u2-l-kink-anch-p-k2-0-3-0",
         "arc-l-line-with-connector-0-3-0", "arc-replaced-span-only-0-3-0",
         "connector-u3-sys-demo-connector-001", "connector-u3-sys-demo-connector-002-lr1", "authored-l-line"]
NAMES = [f"{stem}-{mode}" for stem in STEMS for mode in ("sparse_interactive", "dense_scrutiny")]


def _folder() -> Path:
    folder = os.environ.get("HEADLESS_PRESSURE_OUTPUT_DIR")
    if not folder:
        pytest.skip("set HEADLESS_PRESSURE_OUTPUT_DIR and run core/runner/headless tests/pressure_export.rs first")
    return Path(folder)


def _schema(name):
    return json.loads((PROJECT / "schemas" / name).read_text(encoding="utf-8"))


def _branches(instance):
    document = _schema("results.v0.3.schema.yaml")
    found = []
    # pressure-2's branch is a reference into results_pressure_2.v0.3 (validated
    # through it); its identity is read from the referenced definition.
    for index, (branch, resolved) in enumerate(zip(document["$defs"]["ResultEnvelope"]["oneOf"], envelope_branches(document))):
        alone = {"$schema": document["$schema"], "$id": document["$id"], "$defs": document["$defs"], **branch}
        try:
            validate_instance(alone, instance["result_envelope"], instance_label=f"branch {index}")
        except AssertionError:
            continue
        found.append(resolved["properties"]["producer"]["properties"]["semantic_contract_id"]["const"])
    return found


@pytest.mark.parametrize("name", NAMES)
def test_actual_pressure_1_document_selects_its_branch_and_the_verifier_admits_it(name):
    folder = _folder()
    raw = json.loads((folder / f"{name}.raw.json").read_text())
    document = json.loads((folder / f"{name}.document.json").read_text())
    validate_instance(_schema("results.v0.3.schema.yaml"), document, instance_label=f"actual headless {name}")
    assert _branches(document) == [PRESSURE_CONTRACT_ID]
    envelope = document["result_envelope"]
    for key in ("producer", "numerical_quality", "formulation_basis", "contract_evidence"):
        assert envelope[key] == raw[key], key
    verify_source(raw)
    verify_source(raw, raw=False)
    assert _source_contract(raw)[0] == PRODUCT_PRESSURE_ID
    with pytest.raises(ValueError, match=PRESSURE_TRANSPORT_REFUSAL):
        _source_contract(raw, check_receipt=False)
    with pytest.raises(ValueError, match="ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH"):
        build_analysis_run_v0_3(raw, input_manifest_ref={"object_type": "InputManifest", "ref": f"manifest:{name}"}, input_manifest_hash="1" * 64)
    bases = [case["basis_ref"] for case in raw["numerical_quality"]["cases"]]
    # The standing is the Rust reader's own verdict (b7-plan R2), which reads pressure-2.
    assert numerical_use_standing(raw, bases) == rust_reader.standing(raw, bases, None)["token"]


def test_artifact_folder_holds_exactly_the_expected_cases():
    folder = _folder()
    assert {p.name[: -len(".raw.json")] for p in folder.glob("*.raw.json")} == set(NAMES)
    assert {p.name[: -len(".document.json")] for p in folder.glob("*.document.json")} == set(NAMES)
