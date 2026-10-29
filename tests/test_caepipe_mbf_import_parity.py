"""The MBF importer against its committed parity vectors (DEL-17-04 Design `desktop-mbf-export-import.md` §2, §6 slice 3).

`fixtures/caepipe_mbf/invented/parity/import_vectors.json` records what `core/handoff/caepipe_mbf/importer.py` returns
for every call its own test suites make, for the committed invented MBF corpus and for two coverage inputs (written by
`tools/fixtures/caepipe_mbf_import_vectors.py harvest`). The Rust port replays the same vectors in
`core/handoff/caepipe_mbf_engine/tests/parity.rs`. These tests replay them through the Python, so neither side can
change alone: a change to the importer's outputs, to the writer's regenerated sidecars or to the numerical boundary
vectors fails here until the vectors are harvested again, and then fails the Rust test until the port follows.
"""

from __future__ import annotations

import hashlib
import importlib.util
import json
from pathlib import Path

import pytest


ROOT = Path(__file__).resolve().parents[1]
PARTS = 4


def _load(name: str, path: Path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


VECTORS = _load("caepipe_mbf_import_vectors", ROOT / "tools" / "fixtures" / "caepipe_mbf_import_vectors.py")
UNICODE = _load("caepipe_mbf_engine_unicode", ROOT / "tools" / "fixtures" / "caepipe_mbf_engine_unicode.py")


@pytest.fixture(scope="module")
def document() -> dict:
    return VECTORS.load()


@pytest.mark.parametrize("part", range(PARTS))
def test_every_recorded_import_replays_to_the_recorded_bytes(document, part):
    failures = VECTORS.check(document, part=part, parts=PARTS)
    assert failures == [], "\n".join(failures)


def test_the_recorded_regenerations_are_the_writers(document):
    assert VECTORS.check_regenerations(document) == []


def test_the_vectors_cover_the_committed_mbf_corpus(document):
    """Every tracked invented MBF (the import fixtures and the authored cases) is a corpus input with its full outputs."""
    rows = document["imports"]
    corpus = {row[VECTORS.ROW["data"]] for row in rows
              if document["sources"][row[VECTORS.ROW["src"]]].startswith("corpus:")}
    files = sorted([*(ROOT / "fixtures" / "caepipe_mbf").rglob("*.mbf"),
                    *(ROOT / "validation" / "caepipe_correlation" / "authored").glob("*.mbf")])
    assert len(files) >= 25
    for path in files:
        assert hashlib.sha256(path.read_bytes()).hexdigest()[:16] in corpus, path
    full = [row for row in rows if "report" in row[VECTORS.ROW["expect"]]]
    assert len(full) >= len(files)


def test_the_round_trip_sets_are_the_suites(document):
    assert document["sets"] == VECTORS.round_trip_sets()


def test_the_vectors_name_every_importer_code_they_exercise_and_skip_only_what_has_no_json_form(document):
    assert document["counts"]["imports"] == len(document["imports"]) > 9000
    assert set(document["counts"]["skipped"]) <= {"checked_json_domain patched by the test",
                                                  "input not representable in JSON: a lone surrogate"}


def test_the_engine_unicode_tables_are_the_pythons():
    assert UNICODE.main(["--check"]) == 0


def test_the_vectors_are_declared_invented():
    provenance = json.loads(VECTORS.VECTORS.with_name("import_vectors.provenance.json").read_text(encoding="utf-8"))
    assert provenance["produced_by_caepipe"] is False and "invented" in provenance["origin"]
