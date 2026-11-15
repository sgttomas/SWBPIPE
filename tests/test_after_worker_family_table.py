"""The after-worker family table (tools/after_worker_family, DEL-04-05), against its threat model (README):
drift between the committed table and the web sources it cites.

The check passes on the committed files. The controls run on a temporary copy of the cited files and the
judge, never on the repository: a changed anchor or cited text fails and names its row; lines inserted above
the cited passages pass; a removed cited file fails and says it is missing. The registered check's path
selection covers every file the table cites.
"""
from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import shutil
import subprocess
import sys

import pytest

PROJECT = Path(__file__).resolve().parents[1]
TOOL = PROJECT / "tools" / "after_worker_family" / "family_table.py"
WORKFLOW = PROJECT / "software-workflow.json"

_spec = importlib.util.spec_from_file_location("after_worker_family_table", TOOL)
family = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(family)

JUDGE = f"{family.WEB}/{family.JUDGE}"
CITED = [f"{family.WEB}/{rel}" for rel in family.cited_files()]


def run(root: Path):
    done = subprocess.run([sys.executable, "-I", str(TOOL), str(root)], capture_output=True, text=True, timeout=120)
    return done.returncode, done.stderr


def mismatches(stderr: str) -> list[str]:
    return [line for line in stderr.splitlines() if line.startswith("MISMATCH: ")]


@pytest.fixture
def copy(tmp_path: Path) -> Path:
    """The judge and every cited file, copied under a temporary root."""
    for rel in [JUDGE, *CITED]:
        target = tmp_path / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(PROJECT / rel, target)
    return tmp_path


def edit(root: Path, rel: str, old: str, new: str) -> None:
    path = root / family.WEB / rel
    text = path.read_text(encoding="utf-8")
    assert text.count(old) == 1, f"the control's text is not found once in {rel}"
    path.write_text(text.replace(old, new), encoding="utf-8")


def test_the_check_passes_on_the_committed_files():
    code, stderr = run(PROJECT)
    assert code == 0, stderr
    assert not mismatches(stderr)


def test_the_unchanged_copy_passes(copy: Path):
    code, stderr = run(copy)
    assert code == 0, stderr


def test_a_changed_cited_text_fails_and_names_its_row(copy: Path):
    # Inside the body of a restated declaration; its name, and so its anchor, is unchanged.
    edit(copy, family.PREVIEW, "function nativeEnvelopeRefusal(", "function nativeEnvelopeRefusal(/* edited */")
    code, stderr = run(copy)
    assert code == 1
    assert mismatches(stderr) == ["MISMATCH: nativeEnvelopeRefusal: its cited text changed (services/previewService.ts: "
                                  "nativeEnvelopeRefusal): re-read the source and revalidate this row's restatement"]


def test_a_changed_passage_inside_an_inline_row_fails_and_names_its_row(copy: Path):
    edit(copy, family.SESSION, "SOLVE-PRODUCER-CONTRACT-UNSUPPORTED: a fresh result", "SOLVE-PRODUCER-CONTRACT-UNSUPPORTED: an edited result")
    code, stderr = run(copy)
    assert code == 1
    found = mismatches(stderr)
    assert len(found) == 1 and found[0].startswith("MISMATCH: the session gates 1 and 2: its cited text changed"), stderr


def test_a_changed_anchor_fails_and_names_its_row(copy: Path):
    edit(copy, "features/workspace/resultsSessionState.ts", 'useState<PreviewSolverMode>("sparse_interactive")',
         'useState<PreviewSolverMode>("dense")')
    code, stderr = run(copy)
    assert code == 1
    found = mismatches(stderr)
    assert any(line.startswith("MISMATCH: the solver mode: its anchor") and "occurs 0 times" in line for line in found), stderr
    assert all("the solver mode" in line or "family-table.md" in line for line in found), stderr


def test_lines_inserted_above_the_cited_passages_pass(copy: Path):
    """A line shift alone: lines at the top of every cited file and directly above every cited passage that
    does not lie inside another (a line inserted there is a change to the outer passage's text)."""
    check = family.Check(copy)
    spans: dict[str, list[tuple[int, int]]] = {}
    for r in family.ROWS:
        if r["kind"] == "restated":
            found = check.declaration(r["predicate"], r["rel"], r["predicate"].split(" ")[0])
            span = found[:2] if found else None
        elif r["passage"]:
            span = check.passage(r["predicate"], r["rel"], r["passage"]) if r["kind"] == "inline" else (
                (lambda at: (at, at))(check.anchor_line(r["predicate"], r["rel"], r["passage"]["from"])))
        else:
            continue
        assert span is not None and span[0] is not None, check.problems
        spans.setdefault(r["rel"], []).append(span)
    inserted = 0
    for rel in family.cited_files():
        path = copy / family.WEB / rel
        lines = path.read_text(encoding="utf-8").split("\n")
        own = spans.get(rel, [])
        starts = {s for s, _ in own if not any(o < s <= e for o, e in own)}
        out = ["// shifted", "// shifted", ""]
        for i, line in enumerate(lines):
            if i in starts:
                out += ["// shifted above a cited passage", ""]
                inserted += 1
            out.append(line)
        path.write_text("\n".join(out), encoding="utf-8")
    assert inserted >= 15
    code, stderr = run(copy)
    assert code == 0, stderr


def test_a_removed_cited_file_fails_and_says_it_is_missing(copy: Path):
    (copy / family.WEB / "features/workspace/resultsSessionState.ts").unlink()
    code, stderr = run(copy)
    assert code == 1
    assert "MISMATCH: the solver mode: its cited file apps/desktop/src/features/workspace/resultsSessionState.ts is missing" in stderr


def test_the_registered_selection_covers_every_cited_file():
    """The table's files (and the digests file's) select the registered check, which runs this test."""
    workflow = json.loads(WORKFLOW.read_text(encoding="utf-8"))
    assert workflow["checks"]["after-worker-family-table"]["command"][-1] == "tests/test_after_worker_family_table.py"
    rules = [rule for rule in workflow["path_rules"] if "after-worker-family-table" in rule["checks"]]
    assert len(rules) == 1
    paths = set(rules[0]["paths"])
    digests = json.loads(family.DIGESTS.read_text(encoding="utf-8"))
    from_digests = {f"{family.WEB}/{entry['source'].split(':', 1)[0]}" for entry in digests.values()}
    assert from_digests <= set(CITED)
    assert set(CITED) <= paths
    assert {"tools/after_worker_family/**", "apps/desktop/src/test-support/after-worker/**",
            "tests/test_after_worker_family_table.py", "software-workflow.json"} <= paths
