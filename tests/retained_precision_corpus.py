"""The current shared retained-precision corpus, stored losslessly as one DAG/v1 document.

Expansion preserves independent containers, scalar JSON forms and the exact
current bytes; reading the fixtures establishes no new native execution.
"""
import importlib.util
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CORPUS_FILES = ("fixtures/results/retained_precision_cases.json",)
LIST_MEMBERS = ("cases", "mutations", "must_pass")

_STORAGE_SPEC = importlib.util.spec_from_file_location(
    "retained_precision_corpus_storage", ROOT / "tools/fixtures/retained_precision_corpus.py"
)
_STORAGE = importlib.util.module_from_spec(_STORAGE_SPEC)
_STORAGE_SPEC.loader.exec_module(_STORAGE)


def expanded_snapshot_bytes(name):
    """Historical full bytes, independent of the lossless storage definition."""
    raw = (ROOT / name).read_bytes()
    document = json.loads(raw)
    return _STORAGE.expanded_bytes(document)


def load_corpus(files=CORPUS_FILES):
    merged = {}
    for name in files:
        for key, value in _STORAGE.expand(json.loads((ROOT / name).read_bytes())).items():
            if key in LIST_MEMBERS:
                assert isinstance(value, list), f"{name}: {key} is not an array"
                merged.setdefault(key, []).extend(value)
            elif key in merged:
                assert merged[key] == value, f"{name}: corpus member {key} differs from an earlier file"
            else:
                merged[key] = value
    return merged
