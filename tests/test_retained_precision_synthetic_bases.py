"""The synthetic reader-control bases have a public repository writer that reproduces the committed corpus.

`tools/fixtures/retained_precision_synthetic_bases.py` (also `retained_precision_corpus.py synthetic-bases`)
forms each synthetic base's not-covered derived rows from its own rows and rehashes it by the corpus rule.
Its `--check` must report zero differences on the committed bytes. It imports only the standard library;
`test_verifier_independence.py` separately refuses any import of the verifiers from outside `tests/`.
"""
import ast
import copy
from decimal import Decimal, localcontext
import importlib.util
import io
import json
import math
from pathlib import Path
import random
import struct
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
_SPEC = importlib.util.spec_from_file_location("retained_precision_synthetic_bases", ROOT / "tools/fixtures/retained_precision_synthetic_bases.py")
writer = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(writer)


def _committed():
    storage = writer._storage()
    definition = json.loads((ROOT / writer.CORPUS).read_bytes())
    return storage, definition["layout"], storage.expand(definition)


def test_check_reproduces_the_committed_corpus_byte_for_byte():
    out = io.StringIO()
    assert writer.run(check=True, out=out) == 0, out.getvalue()
    assert "check: 0 differences" in out.getvalue()


def test_the_corpus_tool_subcommand_runs_the_same_check():
    completed = subprocess.run([sys.executable, "tools/fixtures/retained_precision_corpus.py", "synthetic-bases", "--check"],
                               cwd=ROOT, capture_output=True, text=True)
    assert completed.returncode == 0 and "check: 0 differences" in completed.stdout, completed.stdout + completed.stderr


def test_derived_rows_version_text_and_hashes_are_formed_from_each_bases_own_rows():
    """Strip every synthetic base back to its own rows (no derived rows or not_covered entries, projection
    indices on the stripped rows, model 0.6.0, stale text and hashes); the writer restores the committed bases."""
    storage, layout, committed = _committed()
    stripped = copy.deepcopy(committed)
    synthetic = [base for base in stripped["cases"] if base["provenance"] == writer.SYNTHETIC]
    assert len(synthetic) == 18
    for base in synthetic:
        source = base["source"]
        body = source["retained_precision"]["body"]
        before = writer.case_rows(source)
        derived = {row["id"] for row in source["results"] if row["kind"] in writer.DERIVED}
        assert derived
        source["results"] = [row for row in source["results"] if row["kind"] not in writer.DERIVED]
        after = writer.case_rows(source)
        for case in body["cases"]:
            selection = case.get("selection")
            if isinstance(selection, dict) and isinstance(selection.get("not_covered"), list):
                selection["not_covered"] = [i for i in selection["not_covered"] if i not in derived]
        for attempt in body["product_attempts"]:
            if isinstance(attempt, dict) and isinstance(attempt.get("proof"), dict) and (attempt.get("owner_ref") or {}).get("kind") == "case":
                case_id = body["cases"][attempt["owner_ref"]["index"]]["basis_ref"]["ref_id"]
                for outcome in attempt["proof"].get("projection_outcomes") or []:
                    if type(outcome.get("row_index")) is int and outcome["row_index"] < len(before[case_id]):
                        outcome["row_index"] = after[case_id].index(before[case_id][outcome["row_index"]])
        source["formulation_basis"] = {"stale": True}
        base["invocation"]["request"]["model"]["schema_version"] = "0.6.0"
        body["invocation"]["value"] = body["publication_sha256"] = source["retained_precision"]["receipt_sha256"] = "0" * 64
        for case in body["cases"]:
            if "source_identity_sha256" in case:
                case["source_identity_sha256"] = "0" * 64
    stripped_probes = {"shapes": []}
    regenerated, _, report = writer.regenerate(stripped, stripped_probes)
    assert sorted(report["row_sets_changed"]) == sorted(base["id"] for base in synthetic)
    assert storage._bytes(regenerated["cases"], layout) == storage._bytes(committed["cases"], layout)


def test_entry_paths_into_results_follow_their_rows():
    entry = {"id": "e", "edits": [{"path": ["results", 1, "value"], "op": "set", "value": 1.0}, {"path": ["producer"], "op": "remove"}],
             "after_rehash": [{"path": ["results", 0], "op": "remove"}]}
    assert writer.follow_rows(entry, ["a", "b"], ["a", "x", "y", "b"])
    assert [edit["path"] for edit in entry["edits"] + entry["after_rehash"]] == [["results", 3, "value"], ["producer"], ["results", 0]]


def test_canonical_text_matches_the_crates_committed_vectors():
    cases = json.loads((ROOT / "fixtures/canonical_hash/cases.json").read_text(encoding="utf-8"))["cases"]
    refused = []
    for case in cases:
        value = json.loads(case["input_json"])
        try:
            text = writer.canonical_text(value)
        except writer.WriterError:
            refused.append(case["case_id"])
            continue
        assert text == case["expected_canonical"], case["case_id"]
    assert len(cases) >= 20
    assert refused == ["number-large-exponent", "number-ecma-notation-boundaries", "number-beyond-2-53-integer-kept-exact"]


def _rounded_root(a, b):
    with localcontext() as context:
        context.prec = 120
        root = (Decimal(a) * Decimal(a) + Decimal(b) * Decimal(b)).sqrt()
    return float(root)


def test_norm2_is_the_correctly_rounded_root():
    generator = random.Random(20261019)
    pairs = [(3.0, 4.0), (0.0, 0.0), (-0.0, 5e-324), (1e-300, 1e-300), (1e300, -1e300), (1.0, 2 ** -27), (0.1, 0.2)]
    for _ in range(3000):
        pairs.append(tuple(math.ldexp(generator.uniform(-1.0, 1.0), generator.randint(-60, 60)) for _ in range(2)))
    for a, b in pairs:
        value = writer.norm2(a, b)
        assert struct.pack(">d", value) == struct.pack(">d", _rounded_root(a, b)), (a, b)


def test_writer_imports_only_the_standard_library():
    tree = ast.parse((ROOT / "tools/fixtures/retained_precision_synthetic_bases.py").read_text(encoding="utf-8"))
    imported = {alias.name.split(".")[0] for node in ast.walk(tree) if isinstance(node, ast.Import) for alias in node.names}
    imported |= {node.module.split(".")[0] for node in ast.walk(tree) if isinstance(node, ast.ImportFrom)}
    assert imported and imported <= set(sys.stdlib_module_names), imported
