"""LIVE CHECKS of the reference packages that staged inputs from live repository files.

Such a package's default check, run by tests/test_reference_packages_regeneration.py, reads only what the package
holds, so it establishes that the committed output follows from the staged inputs. Its separate live check
(--check-live) compares those staged inputs with the repository files they were staged from. The table module
names those files as constants (reference_packages.PREVIEW_MODEL, DEMO_MODEL, FRICTION_REVERSAL_TEST), so the CI
selector runs this test when any of them changes. A failure here means a source has moved on and a decision is
needed on re-staging; the committed output still stands for its staged inputs.

Each check runs as `python3 -I -B` from its package directory, in place, within reference_packages.CHECK_TIMEOUT_S,
and must leave every file under validation/references/ as it was. No product binary is built or run.
"""
from __future__ import annotations

import hashlib
import json
import shutil
import subprocess
import sys
from dataclasses import dataclass, replace
from pathlib import Path

import pytest

import reference_packages as R
from reference_packages import PACKAGES


@dataclass(frozen=True)
class LiveCheck:
    """A package's live check: the command, run from the package directory, and the repository files (repository-
    relative) its staged inputs came from. It is not one of the package's `checks`, which read only the package."""
    argv: tuple
    sources: tuple
    siblings: tuple = ()  # the sibling packages its generator imports or reads
    whole: tuple = ()  # ((source, sha256), ...): every source, held here by its whole bytes


LIVE_CHECKS = {
    "t4_i12_preload": LiveCheck(
        ("generators/u3_preload_reference.py", "--check-live"), (R.PREVIEW_MODEL,), siblings=("t4_i12",),
        whole=((R.PREVIEW_MODEL, "98e403feb5fd453526a84ef99790ec4b5a71cfd86c7b12e39e369aa30191468d"),)),
    "demo_friction_reversal_preload": LiveCheck(
        ("generators/demo_friction_preload_reference.py", "--check-live"), (R.DEMO_MODEL, R.FRICTION_REVERSAL_TEST),
        whole=((R.DEMO_MODEL, "2fd18681f688e60b03102eb65112d147871d3a59431685800d2bfa3baca7b672"),
               (R.FRICTION_REVERSAL_TEST, "a3d31f87f98f9fcdaca68d3430abaefe1eeb158b58084963b71c1c14e7f404ae"))),
}

# A live check compares part of its source: the leaves of a model document that the derivation reads, or the lines
# of the frozen table read as text. Neither establishes that the source still means what was staged: a member added
# beside the compared leaves can take precedence in the product, and conditional compilation or an import can change
# which table the compiled test reads. Every source is therefore also held here by the digest of its whole bytes:
# ANY change to it fails test_a_source_held_whole_is_unchanged until a person has confirmed it against the package.
WHOLE_CHANGED = "has changed since its digest was recorded in tests/test_reference_packages_live.py (LIVE_CHECKS)"
# What a person looks at in a changed source's diff before recording its new digest.
LOOK_AT = {
    R.PREVIEW_MODEL: "the whole record of support:SH-140, every member of it and not only its hanger",
    R.DEMO_MODEL: "the whole record of support:SH-140, and the records the derivation reads: the nodes, materials and "
                  "pipe segments, the other supports, and the load cases' primitive loads",
    R.FRICTION_REVERSAL_TEST: "the table EXPECTED, and anything that decides which table the compiled test reads: "
                              "conditional compilation, imports, macros",
}


def whole_changed(name: str, source: str, found: str) -> str:
    """What the failure asks of whoever changed the file, in three lines."""
    return (f"{source} {WHOLE_CHANGED}.\n"
            f"  1. The reference package validation/references/{name} staged its inputs from this file.\n"
            f"  2. In the file's diff, look at {LOOK_AT[source]}. The package's live check "
            f"(python3 -I {' '.join(LIVE_CHECKS[name].argv)}, from the package directory) reports the leaves and "
            f"lines it compares; it cannot see everything.\n"
            f"  3. Only after that look: if what the package staged still holds, record the new digest {found} for "
            f"this file in LIVE_CHECKS. If it does not, leave the digest: a decision is needed on re-staging.")

MOVED_ON = ("a source this package's inputs were staged from has moved on, and a decision is needed: re-stage the "
            "inputs and re-derive, or keep the staged inputs; the committed output still stands for its staged inputs")


def declaration_problems(name: str) -> list[str]:
    """The package has an entry and holds the generator; each source is a committed file, and the generator names
    every part of its path as a quoted string, so the declared sources are the ones it reads."""
    live = LIVE_CHECKS[name]
    if name not in PACKAGES:
        return [f"{name}: a live check with no package entry"]
    generator = R.REFS / name / live.argv[0]
    if not generator.is_file():
        return [f"{name}: the live check runs {live.argv[0]}, which the package does not hold"]
    text = generator.read_text(encoding="utf-8")
    problems = [f"{name}: live-check source {source} is not a committed file"
                for source in live.sources if not (R.ROOT / source).is_file()]
    problems += [f"{name}: {live.argv[0]} does not name {source} (part {part!r} absent)"
                 for source in live.sources for part in Path(source).parts if f'"{part}"' not in text]
    return problems


def live_problems(name: str) -> list[str]:
    """Runs the live check in place. It must leave validation/references/ as it was and exit 0; a nonzero exit
    means a source has moved on (MOVED_ON)."""
    live = LIVE_CHECKS[name]
    before = R.tree_state()
    label = f"python3 -I -B {' '.join(live.argv)} (in validation/references/{name})"
    try:
        run = subprocess.run([sys.executable, "-I", "-B", *live.argv], cwd=R.REFS / name,
                             capture_output=True, timeout=R.CHECK_TIMEOUT_S)
    except subprocess.TimeoutExpired:
        return [f"{label} did not finish within its {R.CHECK_TIMEOUT_S} s limit (reference_packages.CHECK_TIMEOUT_S)"]
    written = R._changes(before, R.tree_state())
    if written:
        return [f"{label} wrote under validation/references/:"] + written
    if run.returncode != 0:
        output = (run.stdout + run.stderr).decode("utf-8", "replace")
        return [f"{label} exited {run.returncode}: {MOVED_ON}. Sources compared: {', '.join(live.sources)}. "
                f"What it reported:\n{output[-3000:]}"]
    return []


def whole_problems(name: str, root: Path = R.ROOT) -> list[str]:
    """This package's live sources, by the digest of their whole bytes."""
    found = {source: hashlib.sha256((root / source).read_bytes()).hexdigest() for source, _ in LIVE_CHECKS[name].whole}
    return [whole_changed(name, source, found[source]) for source, digest in LIVE_CHECKS[name].whole
            if found[source] != digest]


@pytest.mark.parametrize("name", sorted(LIVE_CHECKS))
def test_a_source_held_whole_is_unchanged(name):
    problems = whole_problems(name)
    assert not problems, "\n".join(problems)


def test_every_live_source_is_held_whole():
    for name, live in LIVE_CHECKS.items():
        assert set(live.sources) == {source for source, _ in live.whole}, f"{name}: {sorted(live.sources)}"
        assert set(live.sources) <= set(LOOK_AT), f"{name}: a source with nothing said about what to look at"


@pytest.mark.parametrize("name", sorted(LIVE_CHECKS))
def test_the_live_check_is_declared_with_the_sources_its_generator_reads(name):
    problems = declaration_problems(name)
    assert not problems, "\n".join(problems)


@pytest.mark.parametrize("name", sorted(LIVE_CHECKS))
def test_the_live_check_is_kept_apart_from_the_default_check(name):
    """The default check reads only the package; the live check is not one of the package's checks."""
    assert PACKAGES[name].checks, f"{name}: no default check"
    assert all("--check-live" not in check.argv for check in PACKAGES[name].checks), name


@pytest.mark.parametrize("name", sorted(LIVE_CHECKS))
def test_the_staged_inputs_still_match_their_sources(name):
    problems = live_problems(name)
    assert not problems, f"{name}:\n" + "\n".join(problems)


def test_a_nonzero_exit_says_a_source_has_moved_on(monkeypatch):
    name = sorted(LIVE_CHECKS)[0]
    monkeypatch.setitem(LIVE_CHECKS, name, replace(LIVE_CHECKS[name], argv=("-c", "raise SystemExit(1)")))
    problems = live_problems(name)
    assert len(problems) == 1 and "exited 1" in problems[0] and MOVED_ON in problems[0]


def _copy_of_the_tree_the_live_check_reads(tmp_path: Path, name: str) -> Path:
    """A root holding the package, the sibling packages its generator imports and its live sources, at their
    repository paths, so that a source can be changed without touching the repository."""
    live = LIVE_CHECKS[name]
    for package in (name, *live.siblings):
        shutil.copytree(R.REFS / package, tmp_path / "validation" / "references" / package,
                        ignore=shutil.ignore_patterns("__pycache__"))
    for source in live.sources:
        (tmp_path / source).parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(R.ROOT / source, tmp_path / source)
    return tmp_path


def _live_exit(root: Path, name: str) -> tuple[int, str]:
    run = subprocess.run([sys.executable, "-I", "-B", *LIVE_CHECKS[name].argv],
                         cwd=root / "validation" / "references" / name, capture_output=True, timeout=R.CHECK_TIMEOUT_S)
    return run.returncode, (run.stdout + run.stderr).decode("utf-8", "replace")


def _another_cold_load(text: str) -> str:
    """The hanger's entered cold load, one newton more."""
    document = json.loads(text)
    (hanger,) = [s for s in document["supports"] if s["id"] == "support:SH-140"]
    hanger["hanger"]["cold_load"]["value"] += 1
    return json.dumps(document, indent=2) + "\n"


def _another_number_in_a_row(text: str) -> str:
    assert text.count("stop_reaction_n: -297.6267180126413") == 1
    return text.replace("stop_reaction_n: -297.6267180126413", "stop_reaction_n: -297.6267180126414")


def _a_seventh_row(text: str) -> str:
    """One more row in the frozen table, written without the space the six rows have after `Expected`."""
    lines = text.splitlines(keepends=True)
    (start,) = [i for i, line in enumerate(lines) if line.startswith("const EXPECTED: [Expected; 6] = [")]
    end = next(i for i in range(start, len(lines)) if lines[i].startswith("];"))
    lines[start] = lines[start].replace("[Expected; 6]", "[Expected; 7]")
    lines.insert(end, lines[end - 1].replace("Expected {", "Expected{", 1))
    return "".join(lines)


def _a_comment_in_the_table(text: str) -> str:
    lines = text.splitlines(keepends=True)
    (start,) = [i for i, line in enumerate(lines) if line.startswith("const EXPECTED: [Expected; 6] = [")]
    lines.insert(start + 1, "    // a line this package did not stage\n")
    return "".join(lines)


def _a_commented_copy_and_a_changed_table(text: str) -> str:
    """The table as staged kept inside a block comment, above the table the test reads, which has another number."""
    lines = text.splitlines(keepends=True)
    (start,) = [i for i, line in enumerate(lines) if line.startswith("const EXPECTED: [Expected; 6] = [")]
    end = next(i for i in range(start, len(lines)) if lines[i].startswith("];"))
    kept = ["/*\n", *lines[start:end + 1], "*/\n"]
    return "".join(lines[:start] + kept) + _another_number_in_a_row("".join(lines[start:]))


def _a_second_declaration_behind_an_attribute(text: str) -> str:
    """A second table of the same name declared after an attribute on its own line's start."""
    lines = text.splitlines(keepends=True)
    (start,) = [i for i, line in enumerate(lines) if line.startswith("const EXPECTED: [Expected; 6] = [")]
    end = next(i for i in range(start, len(lines)) if lines[i].startswith("];"))
    second = ["#[cfg(any())] " + lines[start], *lines[start + 1:end + 1]]
    return "".join(lines[:end + 1] + second + lines[end + 1:])


def _a_closing_line_with_more_on_it(text: str) -> str:
    lines = text.splitlines(keepends=True)
    (start,) = [i for i, line in enumerate(lines) if line.startswith("const EXPECTED: [Expected; 6] = [")]
    end = next(i for i in range(start, len(lines)) if lines[i].startswith("];"))
    lines[end] = "/* a note inside the array */ ];\n"
    return "".join(lines)


def _a_boolean_where_a_number_was(text: str) -> str:
    """A coordinate of 0 written as false, which Python equates with 0 and the model document does not."""
    document = json.loads(text)
    (node,) = [n for n in document["nodes"] if n["id"] == "node:N-100"]
    assert node["position"]["x"] == 0 and node["position"]["x"] is not False
    node["position"]["x"] = False
    return json.dumps(document, indent=2) + "\n"


def _a_changed_copy_behind_a_condition_and_an_import(text: str) -> str:
    """The table the test reads replaced, on this host, by a changed copy under another name, imported as EXPECTED.
    The live check reads text and cannot see it; the whole-file digest must."""
    lines = text.splitlines(keepends=True)
    (start,) = [i for i, line in enumerate(lines) if line.startswith("const EXPECTED: [Expected; 6] = [")]
    end = next(i for i in range(start, len(lines)) if lines[i].startswith("];"))
    copy = _another_number_in_a_row("".join(lines[start:end + 1])).replace("const EXPECTED:", "const EXPECTED_MACOS:")
    condition = '#[cfg(target_os = "macos")]\n'
    return "".join(lines[:start] + ['#[cfg(not(target_os = "macos"))]\n'] + lines[start:end + 1]
                   + [condition, copy, condition, "use self::EXPECTED_MACOS as EXPECTED;\n"] + lines[end + 1:])


def _a_stiffness_on_the_record_beside_its_hanger(text: str) -> str:
    """A stiffness written on the support record itself, which the product reads before the hanger's. The live
    checks compare the hanger's leaves and cannot see it; the whole-file digest must."""
    document = json.loads(text)
    (hanger,) = [s for s in document["supports"] if s["id"] == "support:SH-140"]
    assert "stiffness" not in hanger
    hanger["stiffness"] = {"dof": "UZ", "value": {"value": 43000, "unit": "N/m"}}
    return json.dumps(document, indent=2) + "\n"


def _a_repeated_primitive_load(text: str) -> str:
    """One primitive load written twice under the same id, which the product refuses."""
    document = json.loads(text)
    loads = document["load_cases"][0]["primitive_loads"]
    loads.append(dict(loads[0]))
    return json.dumps(document, indent=2) + "\n"


# Each live source with the changes its package's live check must report. A source with no entry here fails
# test_every_live_source_has_a_drift_control, so a source the generator stopped reading cannot stay declared.
DRIFTS = {
    ("t4_i12_preload", R.PREVIEW_MODEL): (_another_cold_load,),
    ("demo_friction_reversal_preload", R.DEMO_MODEL): (_another_cold_load, _a_repeated_primitive_load,
                                                       _a_boolean_where_a_number_was),
    ("demo_friction_reversal_preload", R.FRICTION_REVERSAL_TEST): (
        _another_number_in_a_row, _a_seventh_row, _a_comment_in_the_table, _a_commented_copy_and_a_changed_table,
        _a_second_declaration_behind_an_attribute, _a_closing_line_with_more_on_it),
}


def test_every_live_source_has_a_drift_control():
    declared = {(name, source) for name, live in LIVE_CHECKS.items() for source in live.sources}
    assert declared == set(DRIFTS), f"live sources and drift controls differ: {sorted(declared ^ set(DRIFTS))}"
    assert all(DRIFTS.values()), f"a live source with no change to report: {sorted(k for k, v in DRIFTS.items() if not v)}"


@pytest.mark.parametrize("name", sorted(LIVE_CHECKS))
def test_the_live_check_passes_on_a_copy_of_what_it_reads(tmp_path, name):
    """The control for the drift tests below: unchanged copies pass, so their failures come from the change."""
    code, output = _live_exit(_copy_of_the_tree_the_live_check_reads(tmp_path, name), name)
    assert code == 0, output[-3000:]


@pytest.mark.parametrize("name, source, change", [(name, source, change) for (name, source), changes in sorted(DRIFTS.items())
                                                  for change in changes],
                         ids=lambda v: v.__name__.strip("_") if callable(v) else Path(v).name)
def test_a_changed_source_fails_the_live_check(tmp_path, name, source, change):
    root = _copy_of_the_tree_the_live_check_reads(tmp_path, name)
    before = (root / source).read_text(encoding="utf-8")
    after = change(before)
    assert after != before
    (root / source).write_text(after, encoding="utf-8")
    code, output = _live_exit(root, name)
    assert code != 0 and "SOURCE MOVED ON" in output, f"{change.__name__} in {source} was not reported:\n{output[-3000:]}"


@pytest.mark.parametrize("name, source, change", [
    ("demo_friction_reversal_preload", R.FRICTION_REVERSAL_TEST, _a_changed_copy_behind_a_condition_and_an_import),
    ("demo_friction_reversal_preload", R.DEMO_MODEL, _a_stiffness_on_the_record_beside_its_hanger),
    ("t4_i12_preload", R.PREVIEW_MODEL, _a_stiffness_on_the_record_beside_its_hanger),
], ids=lambda v: v.__name__.strip("_") if callable(v) else Path(v).name)
def test_a_change_the_partial_comparison_cannot_see_fails_the_whole_file_digest(tmp_path, name, source, change):
    root = _copy_of_the_tree_the_live_check_reads(tmp_path, name)
    assert not whole_problems(name, root)
    before = (root / source).read_text(encoding="utf-8")
    (root / source).write_text(change(before), encoding="utf-8")
    problems = whole_problems(name, root)
    assert len(problems) == 1 and problems[0].startswith(f"{source} {WHOLE_CHANGED}"), problems
    assert f"validation/references/{name}" in problems[0] and LOOK_AT[source] in problems[0], problems


def test_a_source_the_generator_does_not_name_fails_its_declaration(monkeypatch):
    name = sorted(LIVE_CHECKS)[0]
    monkeypatch.setitem(LIVE_CHECKS, name, replace(LIVE_CHECKS[name], sources=("unnamed_source.json",)))
    problems = declaration_problems(name)
    assert any("does not name unnamed_source.json" in p for p in problems), problems
