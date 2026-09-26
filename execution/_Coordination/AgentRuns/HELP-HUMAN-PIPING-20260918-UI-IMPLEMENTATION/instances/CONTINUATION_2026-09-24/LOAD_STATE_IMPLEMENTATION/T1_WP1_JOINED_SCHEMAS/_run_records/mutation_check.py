"""Schema-weakening mutations against tests/test_load_reference_source_schema.py.

Run from the WORKING_ROOT of a scratch copy (git archive of HEAD plus the candidate
files), never in place:  <python> <this file> [extra pytest args]

The recorded run passes `-k "not pinned_identities and not reuse_the"`, which
deselects the two structural pin tests. Every mutation must then be caught by
an instance-level test (a refused relabel or an accepted instance that fails),
not merely by the pointer assertions.

Each mutation weakens one pointer of one carrier schema (a removed pin, receipt,
exact key or pairing). The new test file is then run with -x; a mutation is
killed when pytest exits non-zero. The original bytes are restored after each
run, and the restoration is checked by sha256. Exit status 1 if any survives.
"""
from __future__ import annotations

import hashlib
import json
import os
import subprocess
import sys
from pathlib import Path

PYTHON = sys.executable
TEST = "tests/test_load_reference_source_schema.py"
R, A, P = "schemas/results.v0.3.schema.yaml", "schemas/analysis_run.v0.3.schema.json", "schemas/stress_neutral_export.v0.3.schema.json"


def at(document, path):
    for part in path:
        document = document[part]
    return document


def present(parent, key):
    return 0 <= key < len(parent) if isinstance(parent, list) else key in parent


def delete(path):
    def apply(document):
        parent = at(document, path[:-1])
        assert present(parent, path[-1]), path
        del parent[path[-1]]
    return apply


def put(path, value):
    def apply(document):
        parent = at(document, path[:-1])
        assert present(parent, path[-1]), path
        parent[path[-1]] = value
    return apply


def combine(*steps):
    def apply(document):
        for step in steps:
            step(document)
    return apply


def remove_item(path, item):
    def apply(document):
        values = at(document, path)
        assert item in values, (path, item)
        values.remove(item)
    return apply


RB = ["$defs", "ResultEnvelope", "oneOf", 5, "properties"]
AB = ["$defs", "AnalysisRun", "oneOf", 5]
PB = ["oneOf", 5]
MUTATIONS = [
    ("M01 results: producer pin removed", R, delete(RB + ["producer"])),
    ("M02 results: profile pin removed", R, delete(RB + ["formulation_basis"])),
    ("M03 results: semantic_contract_ref pin removed", R, delete(RB + ["semantic_contract_ref"])),
    ("M04 results: receipt no longer required", R, remove_item(["$defs", "ResultEnvelope", "oneOf", 5, "required"], "source_block_recovery")),
    ("M05 results: branch evidence override removed", R, delete(RB + ["contract_evidence"])),
    ("M06 results: branch receipt override removed", R, delete(RB + ["source_block_recovery"])),
    ("M07 receipt: policy pin loosened", R, put(["$defs", "LoadReferenceSourceReceiptBody", "properties", "policy"], {"type": "string"})),
    ("M08 receipt: body is the physics-source body", R, put(["$defs", "LoadReferenceSourceRecovery", "properties", "body"],
                                                            {"$ref": "physics_source_recovery.schema.json#/$defs/receipt_body"})),
    ("M09 evidence: load_reference_states not required", R, remove_item(["$defs", "LoadReferenceSourceContractEvidence", "required"], "load_reference_states")),
    ("M10 evidence: namespace opened", R, delete(["$defs", "LoadReferenceSourceContractEvidence", "additionalProperties"])),
    ("M11 evidence: connector pin removed", R, put(["$defs", "LoadReferenceSourceContractEvidence", "properties", "connector"], {"type": "array"})),
    ("M12 record: selected/retained pairing removed", R, delete(["$defs", "LoadReferenceSourceStateRecord", "oneOf"])),
    ("M13 record: selected method pin removed", R, put(["$defs", "LoadReferenceSourceStateRecord", "properties", "source_recovery", "oneOf", 1,
                                                          "properties", "method"], {"type": "string"})),
    ("M14 exact case: material_basis pin removed", R, put(["$defs", "LoadReferenceSourceExactCase", "properties", "material_basis"],
                                                          {"type": "string", "minLength": 1})),
    ("M15 run: table hash pin removed", A, delete(AB + ["properties", "reproducibility", "properties", "semantic_contract", "properties", "sha256"])),
    ("M16 run: row contract pin removed", A, delete(AB + ["properties", "result_refs"])),
    ("M17 run: receipt no longer required", A, remove_item(AB + ["required"], "source_block_recovery")),
    ("M18 run: evidence no longer required", A, remove_item(AB + ["required"], "contract_evidence")),
    ("M19 run: branch evidence override removed", A, delete(AB + ["properties", "contract_evidence"])),
    ("M20 run: SemanticContract pairing removed for the joined ID", A, put(["$defs", "SemanticContract", "oneOf", 5],
                                                                          {"properties": {"id": {"const": "openpipestress.result_semantics/0.3.0/load-reference-source-1"}}})),
    ("M21 package: table hash pin removed", P, delete(PB + ["properties", "semantic_contract", "properties", "sha256"])),
    ("M22 package: profile pin removed", P, delete(PB + ["properties", "formulation_basis"])),
    ("M23 package: receipt no longer required", P, remove_item(PB + ["required"], "source_block_recovery")),
    ("M24 package: branch evidence override removed", P, delete(PB + ["properties", "contract_evidence"])),
    ("M25 package: branch receipt override removed", P, delete(PB + ["properties", "source_block_recovery"])),
    # M15 alone is expected to be equivalent: SemanticContract.oneOf[5] already pairs the
    # joined ID with its table hash. M26 removes both pins together and must be killed.
    ("M26 run: branch table hash pin and SemanticContract pairing both removed", A, combine(
        delete(AB + ["properties", "reproducibility", "properties", "semantic_contract", "properties", "sha256"]),
        put(["$defs", "SemanticContract", "oneOf", 5],
            {"properties": {"id": {"const": "openpipestress.result_semantics/0.3.0/load-reference-source-1"}}}))),
]
FROM = os.environ.get("MUTATIONS_FROM")  # resume a run at a label prefix, e.g. "M20"
if FROM:
    MUTATIONS = MUTATIONS[next(i for i, m in enumerate(MUTATIONS) if m[0].startswith(FROM)):]


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


survivors = []
env = dict(os.environ, PYTHONDONTWRITEBYTECODE="1")
for label, name, mutate in MUTATIONS:
    original = Path(name).read_bytes()
    before = sha(name)
    document = json.loads(original)
    mutate(document)
    Path(name).write_text(json.dumps(document, indent=2) + "\n", encoding="utf-8")
    try:
        run = subprocess.run([PYTHON, "-m", "pytest", "-q", "-x", "-p", "no:cacheprovider", TEST, *sys.argv[1:]],
                             capture_output=True, text=True, env=env)
    finally:
        Path(name).write_bytes(original)
    assert sha(name) == before
    failed = [line for line in run.stdout.splitlines() if line.startswith("FAILED")]
    killed = run.returncode != 0
    print(f"{'KILLED ' if killed else 'SURVIVED'} {label} :: exit {run.returncode} :: {failed[0] if failed else run.stdout.strip().splitlines()[-1]}")
    sys.stdout.flush()
    if not killed and not label.startswith("M15 "):
        survivors.append(label)
print("NOTE M15 alone is an expected equivalent mutant (see M26); it is not counted as a survivor.")
print("OVERALL", "PASS: all killed" if not survivors else f"FAIL: survivors {survivors}")
sys.exit(1 if survivors else 0)
