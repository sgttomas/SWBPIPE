"""The SI MBF writer against its committed parity vectors (DEL-17-04 Design `desktop-mbf-export-import.md` §2, §6 slice 2).

`fixtures/caepipe_mbf/invented/parity/writer_vectors.json` records what `core/handoff/caepipe_mbf/si_writer.py` returns
for every call the test suites that call it make, for the committed invented corpus and for the generator's coverage
inputs (written by `tools/fixtures/caepipe_mbf_writer_vectors.py harvest`). The Rust port replays the same vectors in
`core/handoff/caepipe_mbf_engine/tests/writer_parity.rs`. These tests replay them through the Python, so neither side can
change alone: a change to the writer's text, maps, sidecar, refusals or number formatting fails here until the
vectors are harvested again, and then fails the Rust test until the port follows.
"""

from __future__ import annotations

import importlib.util
import json
from pathlib import Path

import pytest


ROOT = Path(__file__).resolve().parents[1]
PARTS = 2


def _load(name: str, path: Path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


VECTORS = _load("caepipe_mbf_writer_vectors", ROOT / "tools" / "fixtures" / "caepipe_mbf_writer_vectors.py")


@pytest.fixture(scope="module")
def document() -> dict:
    return VECTORS.load()


@pytest.mark.parametrize("part", range(PARTS))
def test_every_recorded_writer_call_replays_to_the_recorded_bytes(document, part):
    failures, _ = VECTORS.check(document, part=part, parts=PARTS)
    assert failures == [], "\n".join(failures)


def test_every_writer_code_is_emitted_by_a_vector(document):
    """Every diagnostic code si_writer.py names is emitted by at least one vector, as recorded and as replayed."""
    codes = VECTORS.writer_codes()
    assert len(codes) > 40
    assert set(codes) <= set(document["codes"])
    _, replayed = VECTORS.check(document)
    assert replayed == set(document["codes"])


def test_the_vectors_cover_the_written_back_corpus_and_every_correlation_case(document):
    sources = set(document["sources"])
    for path in sorted((ROOT / "validation" / "caepipe_correlation" / "cases").glob("*.json")):
        case = json.loads(path.read_text(encoding="utf-8"))
        assert f"corpus:cases/{case['case_id']}" in sources, path
    imports = json.loads((ROOT / "fixtures/caepipe_mbf/invented/parity/import_vectors.json").read_text(encoding="utf-8"))
    written_back = [source for source in imports["sources"]
                    if source.startswith("corpus:") and source.endswith("written back")]
    assert len(written_back) >= 10
    assert set(written_back) <= sources
    full = [row for row in document["vectors"] if len(row[VECTORS.ROW["expect"]]) > 1]
    assert len(full) >= len(written_back)


def test_the_vectors_hold_renders_regenerations_and_no_skipped_call(document):
    counts = document["counts"]
    assert counts["renders"] > 300 and counts["regenerations"] > 200
    assert counts["vectors"] == len(document["vectors"])
    assert counts["skipped"] == {}


def test_the_vectors_are_declared_invented():
    provenance = json.loads(VECTORS.VECTORS.with_name("writer_vectors.provenance.json").read_text(encoding="utf-8"))
    assert provenance == VECTORS.PROVENANCE
    assert provenance["produced_by_caepipe"] is False and "invented" in provenance["origin"]
