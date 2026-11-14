"""The before/after byte comparison tool (tools/byte_compare), against its threat model (README):

(a) agreement is never reported where the intended numerical work did not happen on both sides;
(b) only binaries the tool built from the stated revisions, clean and at the release profile, are run;
(c) no frozen request is modified on disk;
(d) coverage cannot be lost silently through the manifest.

No cargo build runs here. A stub cargo writes a stub driver whose embedded identity it reads from the
worktree it was asked to build, in a two-commit repository under pytest's temporary folder. The
maintained manifest itself is checked against the current tree.
"""
from __future__ import annotations

import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import textwrap

import pytest

PROJECT = Path(__file__).resolve().parents[1]
TOOL = PROJECT / "tools" / "byte_compare" / "byte_compare.py"
MANIFEST = PROJECT / "tools" / "byte_compare" / "manifest.json"
# The maintained manifest's roots and the version source the closure test reads in the real tree; the
# other scans are of the temporary repositories.
CI_READS = (
    "tools/byte_compare/**",
    "validation/references/kd5_nonlinear/**",
    "validation/references/w16_nonlinear_motion/requests/**",
    "fixtures/product_preview/invented_demo_model.json",
    "core/product_physics/tests/fixtures/preview_physics_invented_model.json",
    "core/product_physics/tests/fixtures/preview_physics_range_envelope_model.json",
    "fixtures/results/modal_state_friction_sliding.request.json",
    "fixtures/reference_system/reference_system.json",
    "core/product_physics/src/model_format.rs",
)

spec = importlib.util.spec_from_file_location("byte_compare", TOOL)
bc = importlib.util.module_from_spec(spec)
spec.loader.exec_module(bc)


# ------------------------------------------------------------------ envelopes and runs

def report(code=0, envelope=b"", status="MECHANICS_SOLVED", **extra):
    line = {"schema": "release-timing-v2", "invocation": "solve", "status": status, "written": code == 0,
            "output_sha256": bc.sha256_bytes(envelope), "output_bytes": len(envelope),
            "document": {"request_materials_ignored": 0}}
    line.update(extra)
    return json.dumps(line) + "\n"


def envelope(mechanics="MECHANICS_SOLVED", rows=3, blocking=(), quality="checks_passed"):
    return json.dumps({"status": {"mechanics": mechanics}, "results": [{"value": i} for i in range(rows)],
                       "diagnostics": [{"severity": "blocking", "code": c} for c in blocking],
                       "numerical_quality": {"status": quality}}).encode()


def classify(code, text, env):
    return bc.classify_run({"exit": code, "timed_out": False}, text, env)[0]


def test_a_solved_envelope_with_rows_is_numerical_work():
    env = envelope()
    assert classify(0, report(envelope=env), env) == bc.NUMERICAL


def test_refusals_and_failures_are_never_numerical_work():
    entry = json.dumps({"schema": "release-timing-v2", "invocation": "solve", "status": "entry_error",
                        "entry_error": {"stage": "entry", "message": "unknown field"}})
    assert classify(3, entry, None) == bc.ENTRY_REFUSAL
    retired = envelope("MODEL_INCOMPLETE", 0, ["MODEL_SCHEMA_VERSION_RETIRED"])
    assert classify(0, report(envelope=retired, status="MODEL_INCOMPLETE"), retired) == bc.ENTRY_REFUSAL
    contract = envelope("MODEL_INCOMPLETE", 0, ["PRESSURE_MODEL_REAUTHOR_REQUIRED"])
    assert classify(0, report(envelope=contract), contract) == bc.ENTRY_REFUSAL
    blocked = envelope("MODEL_INCOMPLETE", 0, ["NONLINEAR_SUPPORT_NONCONVERGENCE"])
    assert classify(0, report(envelope=blocked), blocked) == bc.MODEL_INCOMPLETE
    empty = envelope(rows=0)
    assert classify(0, report(envelope=empty), empty) == "empty_result"
    with_blocking = envelope(blocking=["X"])
    assert classify(0, report(envelope=with_blocking), with_blocking) == "solved_with_blocking"


def test_a_crash_read_as_empty_is_a_crash():
    env = envelope()
    assert classify(101, "", None) == bc.CRASH                      # panic, nothing printed
    assert classify(0, report(envelope=env), None) == bc.CRASH      # no envelope written
    assert classify(0, report(envelope=env), env + b" ") == bc.CRASH  # not the bytes reported
    assert classify(0, report(envelope=env) * 2, env) == bc.CRASH   # not one line
    assert bc.classify_run({"exit": None, "timed_out": True, "seconds": 9}, "", None)[0] == bc.CRASH
    ignored = report(envelope=env, document={"request_materials_ignored": 2})
    assert classify(0, ignored, env) == bc.CRASH


def side(cls, expected, data=None, sha="s"):
    return {"cls": cls, "expected": expected, "envelope": data, "input_sha256": sha}


def test_two_refusals_where_numerical_work_is_expected_fail_and_are_not_compared():
    verdict = bc.judge_pair(side(bc.ENTRY_REFUSAL, bc.NUMERICAL), side(bc.ENTRY_REFUSAL, bc.NUMERICAL))
    assert verdict["failed"] and not verdict["compared"]


def test_expected_non_numerical_classes_pass_without_counting_as_compared():
    for cls in (bc.ENTRY_REFUSAL, bc.MODEL_INCOMPLETE, bc.OUTSIDE_RUNNER_INPUT):
        verdict = bc.judge_pair(side(cls, cls, b"x"), side(cls, cls, b"x"))
        assert not verdict["failed"] and not verdict["compared"], cls


def test_numerical_pairs_compare_their_bytes():
    same = bc.judge_pair(side(bc.NUMERICAL, bc.NUMERICAL, b"x"), side(bc.NUMERICAL, bc.NUMERICAL, b"x"))
    assert same["compared"] and same["verdict"] == "IDENTICAL" and not same["failed"]
    moved = bc.judge_pair(side(bc.NUMERICAL, bc.NUMERICAL, b"x"), side(bc.NUMERICAL, bc.NUMERICAL, b"y"))
    assert moved["compared"] and moved["failed"]


def test_an_expected_refusal_that_solves_fails():
    verdict = bc.judge_pair(side(bc.ENTRY_REFUSAL, bc.ENTRY_REFUSAL), side(bc.NUMERICAL, bc.ENTRY_REFUSAL, b"x"))
    assert verdict["failed"] and "B numerical_work where entry_refusal is expected" in verdict["why"]


def test_different_fixture_bytes_on_the_two_sides_fail():
    verdict = bc.judge_pair(side(bc.NUMERICAL, bc.NUMERICAL, b"x", "1"), side(bc.NUMERICAL, bc.NUMERICAL, b"x", "2"))
    assert verdict["failed"] and not verdict["compared"]


# ------------------------------------------------------------------ the lift

def test_the_lift_sets_only_the_version_and_wraps_a_bare_model():
    request = {"model": {"schema_version": "0.1.0", "x": 0.1}, "materials": []}
    lifted = bc.lift(json.dumps(request).encode(), "0.1.0", "9.9.9")
    assert lifted["lifted"] and lifted["outside"] is None
    assert lifted["request"] == {"model": {"schema_version": "9.9.9", "x": 0.1}, "materials": []}
    bare = bc.lift(json.dumps({"schema_version": "9.9.9", "nodes": []}).encode(), "9.9.9", "9.9.9")
    assert bare["request"] == {"model": {"schema_version": "9.9.9", "nodes": []}} and not bare["lifted"]
    assert bare["shape"] == "model_document"


def test_the_lift_refuses_a_version_the_manifest_does_not_name():
    with pytest.raises(bc.LiftError):
        bc.lift(json.dumps({"model": {"schema_version": "0.8.0"}}).encode(), "0.1.0", "0.9.0")
    with pytest.raises(bc.LiftError):
        bc.lift(b"[1]", "0.1.0", "0.9.0")


def test_requests_the_runner_cannot_take_unchanged_are_outside_its_input():
    extra = bc.lift(json.dumps({"model": {"schema_version": "1"}, "materials": [], "mode_count": 3}).encode(), "1", "1")
    assert "mode_count" in extra["outside"]
    materials = bc.lift(json.dumps({"model": {"schema_version": "1"}, "materials": [{"id": "m"}]}).encode(), "1", "1")
    assert materials["outside"]


def test_serialized_numbers_round_trip_exactly():
    values = [5e-324, 0.1, 1e300, -0.0, 2.2250738585072014e-308, 0.30000000000000004, 123456789012345678901234567890]
    text = bc.serialize_request({"model": {"v": values}})
    back = json.loads(text)["model"]["v"]
    assert [repr(v) for v in back] == [repr(v) for v in values]


def test_the_target_version_comes_from_the_revision(tmp_path):
    source = tmp_path / bc.VERSION_SOURCE
    source.parent.mkdir(parents=True)
    source.write_text('pub const MODEL_SCHEMA_VERSION: &str = "4.5.6";\n')
    assert bc.target_version(tmp_path) == "4.5.6"
    source.write_text("// none\n")
    with pytest.raises(bc.SetupError):
        bc.target_version(tmp_path)


# ------------------------------------------------------------------ the manifest

def manifest_text(entries, roots=None):
    return json.dumps({"schema": bc.MANIFEST_SCHEMA, "roots": roots or [{"pattern": "f/*.json", "why": "w"}],
                       "entries": entries})


NUMERICAL = {"disposition": "numerical_work", "numerical": True}


def test_the_manifest_is_closed_and_consistent():
    good = {"path": "f/a.json", "source_version": "1", "expect": NUMERICAL}
    assert bc.load_manifest(manifest_text([good]))["entries"][0]["expect"]["B"]["disposition"] == bc.NUMERICAL
    per_side = dict(good, expect={"A": {"disposition": "entry_refusal", "numerical": False}, "B": NUMERICAL})
    assert bc.load_manifest(manifest_text([per_side]))["entries"][0]["expect"]["A"]["disposition"] == bc.ENTRY_REFUSAL
    for bad in (dict(good, expected=NUMERICAL),
                dict(good, expect={"disposition": "numerical_work", "numerical": False}),
                dict(good, expect={"disposition": "entry_refusal", "numerical": True}),
                dict(good, expect={"disposition": "solved", "numerical": True}),
                dict(good, path="/abs.json"), dict(good, path="../up.json")):
        with pytest.raises(bc.ManifestError):
            bc.load_manifest(manifest_text([bad]))
    with pytest.raises(bc.ManifestError):
        bc.load_manifest(manifest_text([good, good]))


def test_coverage_lists_unnamed_fixtures_and_missing_paths(tmp_path):
    (tmp_path / "f").mkdir()
    (tmp_path / "f" / "a.json").write_text("{}")
    (tmp_path / "f" / "new.json").write_text("{}")
    manifest = bc.load_manifest(manifest_text([{"path": "f/a.json", "source_version": "1", "expect": NUMERICAL},
                                               {"path": "f/gone.json", "source_version": "1", "expect": NUMERICAL}]))
    assert bc.coverage(tmp_path, manifest) == {"unnamed": ["f/new.json"], "missing": ["f/gone.json"]}


def test_the_maintained_manifest_covers_the_current_tree():
    manifest = bc.load_manifest(MANIFEST.read_text())
    assert bc.coverage(PROJECT, manifest) == {"unnamed": [], "missing": []}
    target = bc.target_version(PROJECT)
    for entry in manifest["entries"]:
        lifted = bc.lift((PROJECT / entry["path"]).read_bytes(), entry["source_version"], target)
        outside = entry["expect"]["A"]["disposition"] == bc.OUTSIDE_RUNNER_INPUT
        assert (lifted["outside"] is not None) == outside, entry["path"]


# ------------------------------------------------------------------ end to end, with stubs

STUB_CARGO = textwrap.dedent('''\
    import json, os, subprocess, sys
    from pathlib import Path
    args = sys.argv[1:]
    manifest = Path(args[args.index("--manifest-path") + 1])
    target = Path(args[args.index("--target-dir") + 1])
    tree = manifest.parent.parent.parent.parent
    def git(*a):
        return subprocess.run(["git", "-C", str(tree), *a], capture_output=True, text=True, check=True).stdout.strip()
    if os.environ.get("STUB_NO_LINK"):
        sys.exit(0)
    identity = {"source_commit": os.environ.get("STUB_COMMIT") or git("rev-parse", "HEAD"),
                "source_tree": git("rev-parse", "HEAD^{tree}"),
                "source_dirty": bool(git("status", "--porcelain")) or bool(os.environ.get("STUB_DIRTY")),
                "build_profile": os.environ.get("STUB_PROFILE") or ("release" if "--release" in args else "debug"),
                "rustc": ["rustc 0.0.0-stub"], "cargo": ["cargo 0.0.0-stub"]}
    perturb = (tree / "perturb").exists()
    binary = target / "release" / "release_timing"
    binary.parent.mkdir(parents=True, exist_ok=True)
    binary.write_text("#!" + sys.executable + "\\n" + "IDENTITY = " + repr(json.dumps(identity)) + "\\n"
                      + "PERTURB = " + repr(perturb) + "\\nTREE = " + repr(str(tree)) + "\\n"
                      + open(os.environ["STUB_DRIVER"]).read())
    binary.chmod(0o755)
''')

STUB_DRIVER = textwrap.dedent('''\
    import hashlib, json, os, sys
    if sys.argv[1:] == ["--build-identity"]:
        print(IDENTITY); sys.exit(0)
    path, mode, out = sys.argv[1], sys.argv[3], sys.argv[5]
    if os.environ.get("STUB_TAMPER"):
        with open(os.path.join(TREE, os.environ["STUB_TAMPER"]), "a") as handle:
            handle.write(" ")
    doc = json.load(open(path))
    model = doc["model"]
    line = {"schema": "release-timing-v2", "invocation": "solve", "document": {"request_materials_ignored": 0}}
    behaviour = model.get("behaviour", "solve")
    if behaviour == "solve_in_b" and not PERTURB:
        behaviour = "refuse"
    if behaviour == "solve_in_b":
        behaviour = "solve"
    if behaviour == "crash":
        sys.exit(101)
    if behaviour == "refuse" or model.get("schema_version") != "2.0.0":
        line.update(status="entry_error", entry_error={"stage": "entry", "message": "refused"})
        print(json.dumps(line)); sys.exit(3)
    value = model.get("value", 1.0) + (1e-9 if PERTURB and model.get("sensitive") else 0.0)
    env = {"status": {"mechanics": "MODEL_INCOMPLETE" if behaviour == "incomplete" else "MECHANICS_SOLVED"},
           "results": [] if behaviour == "incomplete" else [{"value": value, "mode": mode}],
           "diagnostics": [], "numerical_quality": {"status": "checks_passed"}}
    text = json.dumps(env).encode()
    open(out, "wb").write(text)
    line.update(status=env["status"]["mechanics"], written=True, output_sha256=hashlib.sha256(text).hexdigest(),
                output_bytes=len(text))
    print(json.dumps(line))
''')

GIT_ENV = {"GIT_AUTHOR_NAME": "t", "GIT_AUTHOR_EMAIL": "t@example.invalid", "GIT_COMMITTER_NAME": "t",
           "GIT_COMMITTER_EMAIL": "t@example.invalid", "GIT_CONFIG_GLOBAL": os.devnull, "GIT_CONFIG_NOSYSTEM": "1"}


def git(repo, *args):
    env = {k: v for k, v in os.environ.items() if not k.startswith("GIT_")}
    env.update(GIT_ENV)
    return subprocess.run(["git", "-C", str(repo), *args], env=env, check=True, capture_output=True, text=True).stdout.strip()


def write(repo, rel, text):
    path = repo / rel
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text)


FIXTURES = {
    "f/old.json": {"model": {"schema_version": "1.0.0", "value": 2.0, "sensitive": True}, "materials": []},
    "f/bare.json": {"schema_version": "2.0.0", "value": 3.0},
    "f/refused.json": {"model": {"schema_version": "1.0.0", "behaviour": "refuse"}},
    "f/blocked.json": {"model": {"schema_version": "1.0.0", "behaviour": "incomplete"}},
}
EXPECT = {"f/old.json": "numerical_work", "f/bare.json": "numerical_work", "f/refused.json": "entry_refusal",
          "f/blocked.json": "model_incomplete"}


def manifest_for(fixtures=FIXTURES, expect=EXPECT, extra=()):
    entries = [{"path": path, "source_version": (doc.get("model") or doc)["schema_version"],
                "expect": {"disposition": expect[path], "numerical": expect[path] == "numerical_work"}}
               for path, doc in fixtures.items()]
    return json.dumps({"schema": bc.MANIFEST_SCHEMA, "roots": [{"pattern": "f/*.json", "why": "stub"}],
                       "entries": entries + list(extra)})


@pytest.fixture()
def world(tmp_path):
    repo = tmp_path / "repo"
    repo.mkdir()
    git(repo, "init", "-q")
    write(repo, bc.VERSION_SOURCE, 'pub const MODEL_SCHEMA_VERSION: &str = "2.0.0";\n')
    write(repo, bc.DRIVER_CRATE + "/Cargo.toml", "[package]\n")
    for rel, doc in FIXTURES.items():
        write(repo, rel, json.dumps(doc, indent=1))
    git(repo, "add", "-A")
    git(repo, "commit", "-q", "-m", "a")
    a = git(repo, "rev-parse", "HEAD")
    write(repo, "README", "b\n")
    git(repo, "add", "-A")
    git(repo, "commit", "-q", "-m", "b")
    b = git(repo, "rev-parse", "HEAD")
    tools = tmp_path / "stubs"
    tools.mkdir()
    (tools / "cargo.py").write_text(STUB_CARGO)
    (tools / "driver.py").write_text(STUB_DRIVER)
    manifest = tmp_path / "manifest.json"
    manifest.write_text(manifest_for())

    def run(a_rev=a, b_rev=b, env=None, manifest_path=manifest, scratch=None, work=None):
        scratch = scratch or tmp_path / f"scratch-{len(list(tmp_path.glob('scratch-*')))}"
        saved = dict(os.environ)
        os.environ.update({"STUB_DRIVER": str(tools / "driver.py"), **(env or {})})
        try:
            code = bc.main(["compare", "--a", a_rev, "--b", b_rev, "--scratch", str(scratch), "--repo", str(repo),
                            "--manifest", str(manifest_path), "--cargo", f"{sys.executable} {tools / 'cargo.py'}",
                            "--table", str(tmp_path / "table.md"), "--timeout", "60"]
                           + (["--work", str(work)] if work else []))
        finally:
            os.environ.clear()
            os.environ.update(saved)
        table = (tmp_path / "table.md").read_text() if (tmp_path / "table.md").exists() else ""
        return code, table, scratch

    return {"repo": repo, "a": a, "b": b, "run": run, "tmp": tmp_path, "manifest": manifest}


def commit_b(world, rel, text):
    repo = world["repo"]
    git(repo, "checkout", "-q", "--detach", world["a"])
    write(repo, rel, text)
    git(repo, "add", "-A")
    git(repo, "commit", "-q", "-m", "b2")
    return git(repo, "rev-parse", "HEAD")


def test_identical_revisions_compare_only_the_numerical_pairs(world):
    code, table, scratch = world["run"]()
    assert code == 0, table
    assert "Compared with numerical work on both sides: 4 of 8 pairs** (4 identical, 0 different)" in table
    assert "entry_refusal 2, model_incomplete 2" in table
    assert "f/refused.json [sparse]: expected A entry_refusal, B entry_refusal; A did entry_refusal" in table
    # Raw outputs are kept; the worktrees are removed.
    assert list((scratch / "runs" / "A").glob("*.stdout")) and (scratch / "results.json").is_file()
    assert not list((scratch / "build").glob("wt-*"))
    assert "wt-" not in git(world["repo"], "worktree", "list")


def test_the_lift_reaches_the_runner_and_the_frozen_request_is_unchanged(world):
    before = (world["repo"] / "f/old.json").read_bytes()
    code, table, scratch = world["run"]()
    assert code == 0, table
    lifted = json.loads((scratch / "inputs" / "A" / "f__old.json.json").read_text())
    assert lifted["model"]["schema_version"] == "2.0.0"
    assert (world["repo"] / "f/old.json").read_bytes() == before
    assert json.loads(before)["model"]["schema_version"] == "1.0.0"


def test_a_moved_result_fails(world):
    b = commit_b(world, "perturb", "")
    code, table, _ = world["run"](b_rev=b)
    assert code == 1
    assert "| f/old.json | sparse |" in table and "DIFFERENT" in table
    assert "(3 identical, 1 different)" not in table and "(2 identical, 2 different)" in table


def test_an_expected_refusal_that_solves_fails(world):
    # A refuses the fixture; B (the same fixture bytes, with the stub's perturbation) solves it.
    a = commit_b(world, "f/refused.json", json.dumps({"model": {"schema_version": "1.0.0", "behaviour": "solve_in_b"}}))
    write(world["repo"], "perturb", "")
    git(world["repo"], "add", "-A")
    git(world["repo"], "commit", "-q", "-m", "b")
    b = git(world["repo"], "rev-parse", "HEAD")
    code, table, _ = world["run"](a_rev=a, b_rev=b)
    assert code == 1
    assert "f/refused.json [sparse]: expected A entry_refusal, B entry_refusal; A did entry_refusal" in table
    assert "B numerical_work where entry_refusal is expected" in table


def test_a_crash_where_numerical_work_is_expected_fails(world):
    b = commit_b(world, "f/bare.json", json.dumps({"schema_version": "2.0.0", "behaviour": "crash"}))
    code, table, _ = world["run"](b_rev=b)
    assert code == 1
    assert "B crash where numerical_work is expected" in table or "the fixture's bytes differ" in table


def test_an_unnamed_fixture_under_a_root_fails(world):
    b = commit_b(world, "f/new.json", json.dumps({"model": {"schema_version": "2.0.0"}}))
    code, table, _ = world["run"](b_rev=b)
    assert code == 1
    assert "B: f/new.json is under a stated root and not in the manifest" in table


def test_a_manifest_path_that_does_not_exist_fails(world):
    extra = [{"path": "f/gone.json", "source_version": "1.0.0", "expect": NUMERICAL}]
    world["manifest"].write_text(manifest_for(extra=extra))
    code, table, _ = world["run"]()
    assert code == 1
    assert "manifest path f/gone.json does not exist" in table


@pytest.mark.parametrize("env, needle", [
    ({"STUB_COMMIT": "0" * 40}, "source_commit"),
    ({"STUB_DIRTY": "1"}, "source_dirty"),
    ({"STUB_PROFILE": "debug"}, "build_profile"),
    ({"STUB_NO_LINK": "1"}, "did not link"),
])
def test_a_binary_not_built_as_stated_is_never_run(world, env, needle, capsys):
    code, table, scratch = world["run"](env=env)
    assert code == 2 and table == ""
    assert needle in capsys.readouterr().err
    assert not (scratch / "runs").exists()
    assert "wt-" not in git(world["repo"], "worktree", "list")


def test_a_stale_binary_in_the_target_directory_is_not_used(world, capsys):
    work = world["tmp"] / "work"
    target = work / "target" / f"A-{world['a']}" / "release"
    target.mkdir(parents=True)
    (target / "release_timing").write_text("stale")
    code, _, _ = world["run"](env={"STUB_NO_LINK": "1"}, work=work)
    assert code == 2 and "did not link" in capsys.readouterr().err


def test_a_frozen_request_changed_during_the_run_fails(world):
    code, table, _ = world["run"](env={"STUB_TAMPER": "f/bare.json"})
    assert code == 1
    assert "f/bare.json changed on disk during the comparison" in table
    assert "the worktree changed during the runs" in table


def test_scratch_inside_a_git_work_tree_is_refused(world, capsys):
    code, _, _ = world["run"](scratch=world["repo"] / "scratch")
    assert code == 2 and "inside a git work tree" in capsys.readouterr().err
