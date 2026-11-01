"""Model document 0.9.0, S2: the wasm engine's operating-condition preview.

`preview_operating_conditions_json` (core/model_operations/operation_applier,
`--features wasm`) must return, for each committed parity input, exactly the
native preview recorded in `fixtures/operating_conditions/preview_parity.json`
(written and checked natively by core/product_physics/tests/model_document_0_9_0_s2.rs).
The engine is the one `npm run build:wasm --workspace apps/desktop` publishes,
which stamps it from its sources; recomputing that stamp needs `cargo metadata`,
which a pytest session does not run, so freshness is shown by behaviour: an
engine without this slice lacks the export, and the parity inputs include the
slice's newest refusal (an authored empty exposure list). The engine is not
built here: without it the test is skipped by name.
"""
import json
import os
import shutil
import subprocess
from pathlib import Path

import pytest

PROJECT = Path(__file__).resolve().parents[1]
DESKTOP = PROJECT / "apps" / "desktop"
ENGINE = Path(os.environ.get("OPENPIPESTRESS_WASM_ENGINE_DIR", DESKTOP / "public" / "wasm-engine"))
GLUE = ENGINE / "open_pipe_stress_operation_applier.js"
PARITY = PROJECT / "fixtures" / "operating_conditions" / "preview_parity.json"

DRIVER = r"""
import { readFileSync } from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";
const [desktop, engine, crate, parity] = process.argv.slice(-4);
const glue = await import(pathToFileURL(path.join(engine, "open_pipe_stress_operation_applier.js")).href);
glue.initSync({ module: readFileSync(path.join(engine, "open_pipe_stress_operation_applier_bg.wasm")) });
if (typeof glue.preview_operating_conditions_json !== "function") throw new Error("the engine has no preview_operating_conditions_json export: stale");
const rows = JSON.parse(readFileSync(parity, "utf8")).rows;
const inputs = rows.map((row) => JSON.stringify(row.model));
const outputs = inputs.map((text) => glue.preview_operating_conditions_json(text));
const again = inputs.map((text) => glue.preview_operating_conditions_json(text));
const invalid = glue.preview_operating_conditions_json("{not json");
console.log(JSON.stringify({ outputs, again, invalid }));
"""


def run_engine():
    node = shutil.which("node")
    if node is None or not GLUE.is_file():
        pytest.skip(f"WASM-ENGINE-ASSET-ABSENT: {GLUE} (build with npm run build:wasm --workspace apps/desktop)")
    crate = PROJECT / "core" / "model_operations" / "operation_applier"
    result = subprocess.run([node, "--input-type=module", "-e", DRIVER, "--", str(DESKTOP), str(ENGINE), str(crate), str(PARITY)],
                            capture_output=True, text=True, timeout=600, cwd=DESKTOP)
    assert result.returncode == 0, result.stderr
    return json.loads(result.stdout.splitlines()[-1])


def test_the_wasm_preview_equals_the_native_preview_on_every_parity_input():
    run = run_engine()
    rows = json.loads(PARITY.read_text())["rows"]
    assert [row["name"] for row in rows] == ["resolved", "set_missing", "authored_empty_spans", "unknown_field", "exact_route"]
    for row, text, again in zip(rows, run["outputs"], run["again"]):
        assert text == again, f"{row['name']}: deterministic"
        assert json.loads(text) == row["preview"], row["name"]
    resolved = json.loads(run["outputs"][0])
    assert resolved["resolved"] is True
    ids = [load["id"] for case in resolved["cases"] for load in case["generated_primitive_loads"]]
    assert ids[:3] == ["load:condition-temperature:3:OPE:2:P1", "load:condition-temperature:3:OPE:2:P2",
                       "load:condition-temperature:3:OPE:2:P3"]
    assert resolved["cases"][2]["wind_exposed_pipe_refs"] == ["P2", "P3"]


def test_the_wasm_preview_returns_the_input_error_envelope_for_text_that_is_not_json():
    invalid = json.loads(run_engine()["invalid"])
    assert invalid["document_kind"] == "openpipestress.desktop.wasm_engine_input_error"
    assert invalid["error"]["code"] == "WASM-ENGINE-INPUT-JSON-INVALID" and invalid["error"]["input"] == "model"
