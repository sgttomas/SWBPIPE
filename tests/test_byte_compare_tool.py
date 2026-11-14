"""The before/after byte comparison tool (tools/byte_compare), against its threat model (README):

(a) agreement is never reported where the intended numerical work did not happen on both sides;
(b) only binaries the tool built from the stated revisions, clean and at the release profile, are run;
(c) no frozen request is modified on disk;
(d) coverage cannot be lost silently through the manifest.

Most tests run no cargo build. A stub cargo writes a stub driver whose embedded identity it reads from
the worktree it was asked to build, in a two-commit repository under pytest's temporary folder, and
prints the -vv JSON record of that build. The build attestation is tested on a maintained captured
build of the real driver (tools/byte_compare/captured_build, paths cleaned), which pins the format.
The tests marked `runs_cargo` build a synthetic two-crate repository with real cargo, for the
inherited-configuration routes and the default-release control. The maintained manifest itself is
checked against the current tree.
"""
from __future__ import annotations

import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import subprocess
import sys
import tarfile
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

# The maintained manifest's build blocks, stated once there and used by every manifest made here.
MAINTAINED = json.loads(MANIFEST.read_text())
BUILD_BLOCKS = {key: MAINTAINED[key] for key in ("build_settings", "build_closure")}


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


def test_an_expected_refusal_that_solves_fails_in_judge_pair():
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
                       "entries": entries, **BUILD_BLOCKS})


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
    import json, os, shlex, subprocess, sys
    from pathlib import Path
    args = sys.argv[1:]
    LEAK_NAMES = {"RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "RUSTDOCFLAGS", "CARGO_INCREMENTAL", "RUSTC_WRAPPER",
                  "RUSTC_WORKSPACE_WRAPPER"}
    manifest = Path(args[args.index("--manifest-path") + 1])
    target = Path(args[args.index("--target-dir") + 1])
    tree = manifest.parent.parent.parent.parent
    def git(*a):
        return subprocess.run(["git", "-C", str(tree), *a], capture_output=True, text=True, check=True).stdout.strip()
    # Settings the tool must strip from the build's environment (CLEARED_ENV, CLEARED_PREFIXES).
    leaked = sorted(k for k in os.environ if k in LEAK_NAMES or k.startswith(("GIT_", "CARGO_PROFILE_", "CARGO_BUILD_", "CARGO_TARGET_")))
    if leaked:
        print("stub cargo: inherited build settings reached the build: " + ", ".join(leaked), file=sys.stderr)
        sys.exit(1)
    if os.environ.get("STUB_NO_LINK"):
        sys.exit(0)
    identity = {"source_commit": os.environ.get("STUB_COMMIT") or git("rev-parse", "HEAD"),
                "source_tree": os.environ.get("STUB_TREE") or git("rev-parse", "HEAD^{tree}"),
                "source_dirty": bool(git("status", "--porcelain")) or bool(os.environ.get("STUB_DIRTY")),
                "build_profile": os.environ.get("STUB_PROFILE") or ("release" if "--release" in args else "debug"),
                "rustc": ["rustc 0.0.0-stub"], "cargo": ["cargo 0.0.0-stub"]}
    perturb = (tree / "perturb").exists()
    side = target.name.split("-")[0]
    crate = manifest.parent
    deps = target / "release" / "deps"
    deps.mkdir(parents=True, exist_ok=True)
    extra = "-0123456789abcdef"
    body = ("#!" + sys.executable + "\\n" + "IDENTITY = " + repr(json.dumps(identity)) + "\\n"
            + "PERTURB = " + repr(perturb) + "\\nTREE = " + repr(str(tree)) + "\\n"
            + open(os.environ["STUB_DRIVER"]).read())
    written = deps / ("release_timing" + extra)   # what rustc writes
    binary = target / "release" / "release_timing"  # what cargo uplifts
    written.write_text(body)
    binary.write_text(body + ("# not the file rustc wrote\\n" if os.environ.get("STUB_UPLIFT_DIFFERS") else ""))
    for path in (written, binary):
        path.chmod(0o755)
    # A stand-in rustc in its own sysroot, answering -vV and --print sysroot.
    sysroot = Path(os.environ["STUB_SYSROOT"]) / side
    rustc = sysroot / "bin" / "rustc"
    rustc.parent.mkdir(parents=True, exist_ok=True)
    release = "0.0.1" if side == "B" and os.environ.get("STUB_RUSTC_B") else "0.0.0"
    version = f"rustc {release}-stub\\nbinary: rustc\\nhost: stub-host\\nrelease: {release}\\n"
    rustc.write_text("#!" + sys.executable + "\\nimport sys\\n"
                     + f"if sys.argv[1:] == ['-vV']: sys.stdout.write({version!r})\\n"
                     + f"elif sys.argv[1:] == ['--print', 'sysroot']: print({str(sysroot)!r})\\n"
                     + "else: sys.exit(1)\\n")
    rustc.chmod(0o755)
    # The build's record: one compiler-artifact message and the -vv invocation, at the manifest's
    # runtime settings (STUB_BUILD), with whatever a test adds.
    build = json.loads(os.environ["STUB_BUILD"])
    artifact = {"reason": "compiler-artifact", "manifest_path": str(manifest),
                "package_id": f"path+file://{crate}#open_pipe_stress_release_timing@0.1.0",
                "target": {"kind": ["bin"], "crate_types": ["bin"], "name": "release_timing",
                           "src_path": str(crate / "src" / "main.rs"), "edition": "2021"},
                "profile": dict(build["profile"], test=False), "features": [], "filenames": [str(binary)],
                "executable": str(binary), "fresh": bool(os.environ.get("STUB_FRESH"))}
    env = {"CARGO": "/stub/cargo", "CARGO_BIN_NAME": "release_timing", "CARGO_CRATE_NAME": "release_timing",
           "CARGO_MANIFEST_DIR": str(crate), "CARGO_MANIFEST_PATH": str(manifest),
           "CARGO_PKG_NAME": "open_pipe_stress_release_timing", "CARGO_PKG_VERSION": "0.1.0",
           "CARGO_PRIMARY_PACKAGE": "1"}
    env.update(dict(item.split("=", 1) for item in os.environ.get("STUB_RUSTC_ENV", "").split()))
    codegen = []
    for key, value in build["codegen"].items():
        if value is not None:
            codegen += ["-C", key if value is True else f"{key}={value}"]
    argv = ["--crate-name", "release_timing", "--edition=2021", "src/main.rs", "--error-format=json",
            "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link",
            *codegen, "-C", "metadata=0123", "-C", "extra-filename=" + extra, "--out-dir", str(deps),
            "-L", f"dependency={deps}", *shlex.split(os.environ.get("STUB_RUSTC_ARGS", ""))]
    command = [f"{k}={v}" for k, v in env.items()] + [str(rustc)] + argv
    print(json.dumps(artifact))
    print(json.dumps({"reason": "build-finished", "success": True}))
    sys.stderr.write(f"   Compiling open_pipe_stress_release_timing v0.1.0 ({crate})\\n"
                     f"     Running `{shlex.join(command)}`\\n"
                     "    Finished `release` profile [optimized] target(s) in 0.01s\\n")
''')

STUB_DRIVER = textwrap.dedent('''\
    import hashlib, json, os, sys
    if sys.argv[1:] == ["--build-identity"]:
        print(IDENTITY); sys.exit(0)
    path, mode, out = sys.argv[1], sys.argv[3], sys.argv[5]
    if os.environ.get("STUB_CREATE") and not os.path.lexists(os.environ["STUB_CREATE"]):
        with open(os.environ["STUB_CREATE"], "x") as handle:
            handle.write("earlier evidence")
    if os.environ.get("STUB_GIT_INIT"):
        import subprocess
        subprocess.run(["git", "init", "-q", os.environ["STUB_GIT_INIT"]], check=True)
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
                       "entries": entries + list(extra), **BUILD_BLOCKS})


@pytest.fixture()
def world(tmp_path):
    repo = tmp_path / "repo"
    repo.mkdir()
    git(repo, "init", "-q")
    write(repo, bc.VERSION_SOURCE, 'pub const MODEL_SCHEMA_VERSION: &str = "2.0.0";\n')
    write(repo, bc.DRIVER_CRATE + "/Cargo.toml", "[package]\n")
    write(repo, bc.DRIVER_CRATE + "/Cargo.lock",
          'version = 4\n\n[[package]]\nname = "open_pipe_stress_release_timing"\nversion = "0.1.0"\n')
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

    def run(a_rev=a, b_rev=b, env=None, manifest_path=manifest, scratch=None, work=None, table=None, keep=False):
        scratch = scratch or tmp_path / f"scratch-{len(list(tmp_path.glob('scratch-*')))}"
        table = table or tmp_path / f"table-{len(list(tmp_path.glob('table-*.md')))}.md"
        saved = dict(os.environ)
        os.environ.update({"STUB_DRIVER": str(tools / "driver.py"), "STUB_SYSROOT": str(tmp_path / "sysroot"),
                           "STUB_BUILD": json.dumps(BUILD_BLOCKS["build_settings"]["runtime"]), **(env or {})})
        try:
            code = bc.main(["compare", "--a", a_rev, "--b", b_rev, "--scratch", str(scratch), "--repo", str(repo),
                            "--manifest", str(manifest_path), "--cargo", f"{sys.executable} {tools / 'cargo.py'}",
                            "--table", str(table), "--timeout", "60"]
                           + (["--work", str(work)] if work else []) + (["--keep-worktrees"] if keep else []))
        finally:
            os.environ.clear()
            os.environ.update(saved)
        text = table.read_text() if table.is_file() else ""
        return code, text, scratch

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
    ({"STUB_TREE": "0" * 40}, "source_tree"),
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
    # Each build is into a fresh, empty target directory: one holding an earlier binary is refused
    # before anything is built (round 3, threat (b): every unit attested was compiled by this build).
    work = world["tmp"] / "work"
    target = work / "target" / f"A-{world['a']}" / "release"
    target.mkdir(parents=True)
    (target / "release_timing").write_text("stale")
    code, _, scratch = world["run"](env={"STUB_NO_LINK": "1"}, work=work)
    assert code == 2 and "is not an empty directory; each build is fresh" in capsys.readouterr().err
    assert not (scratch / "runs").exists()


def test_a_frozen_request_changed_during_the_run_fails(world):
    code, table, _ = world["run"](env={"STUB_TAMPER": "f/bare.json"})
    assert code == 1
    assert "f/bare.json changed on disk during the comparison" in table
    assert "the worktree changed during the runs" in table


def test_scratch_inside_a_git_work_tree_is_refused(world, capsys):
    code, _, _ = world["run"](scratch=world["repo"] / "scratch")
    assert code == 2 and "inside a git work tree" in capsys.readouterr().err


def test_inherited_build_settings_never_reach_the_build(world):
    assert bc.clean_env({"PATH": "p", "HOME": "h", "RUSTFLAGS": "-C opt-level=0", "CARGO_PROFILE_RELEASE_OPT_LEVEL": "0",
                         "CARGO_BUILD_RUSTFLAGS": "x", "CARGO_TARGET_DIR": "t", "RUSTC_WRAPPER": "w",
                         "GIT_DIR": "/nowhere"}) == {"PATH": "p", "HOME": "h"}
    # The stub cargo refuses to build if any of these reaches it; the tool's git calls would fail on GIT_DIR.
    inherited = {"RUSTFLAGS": "-C opt-level=0", "CARGO_ENCODED_RUSTFLAGS": "-Copt-level=0",
                 "CARGO_PROFILE_RELEASE_OPT_LEVEL": "0", "CARGO_BUILD_RUSTFLAGS": "-C opt-level=0",
                 "CARGO_TARGET_DIR": str(world["tmp"] / "elsewhere"), "CARGO_INCREMENTAL": "1",
                 "RUSTC_WRAPPER": "/usr/bin/false", "GIT_DIR": str(world["tmp"] / "nowhere")}
    code, table, _ = world["run"](env=inherited)
    assert code == 0, table
    assert "Compared with numerical work on both sides: 4 of 8 pairs**" in table


# ------------------------------------------------------------------ the table destination (threat (c))

def frozen(world):
    return world["repo"] / "f/old.json"


def caller_worktree(world):
    return frozen(world), "inside a git work tree"


def symlink_to_request(world):
    link = world["tmp"] / "link.md"
    link.symlink_to(frozen(world))
    return link, "inside a git work tree"


def hard_link_to_request(world):
    link = world["tmp"] / "hard.md"
    os.link(frozen(world), link)
    return link, "already exists"


def this_runs_generated_worktree(world):
    return world["tmp"] / "work" / f"wt-A-{world['a'][:12]}" / "f/old.json", "one of the tool's own paths"


def scratch_evidence(world):
    return world["tmp"] / "scratch-x" / "results.json", "one of the tool's own paths"


@pytest.mark.parametrize("destination", [caller_worktree, symlink_to_request, hard_link_to_request,
                                         this_runs_generated_worktree, scratch_evidence])
def test_a_table_destination_that_could_overwrite_a_request_or_evidence_is_refused(world, destination, capsys):
    before = frozen(world).read_bytes()
    table, needle = destination(world)
    links = os.stat(frozen(world)).st_nlink
    code, _, scratch = world["run"](table=table, work=world["tmp"] / "work", scratch=world["tmp"] / "scratch-x")
    assert code == 2 and needle in capsys.readouterr().err
    assert frozen(world).read_bytes() == before and os.stat(frozen(world)).st_nlink == links
    assert git(world["repo"], "status", "--porcelain") == ""
    assert not (scratch / "runs").exists() and not (scratch / "results.json").exists()
    assert "wt-" not in git(world["repo"], "worktree", "list")


def test_a_table_destination_in_a_worktree_kept_by_an_earlier_run_is_refused(world, capsys):
    work = world["tmp"] / "work"
    code, table, _ = world["run"](work=work, keep=True)
    assert code == 0, table
    kept = work / f"wt-A-{world['a'][:12]}"
    before = (kept / "f/old.json").read_bytes()
    try:
        code, _, _ = world["run"](table=kept / "f/old.json", work=world["tmp"] / "work-2")
        assert code == 2 and "inside a git work tree" in capsys.readouterr().err
        assert (kept / "f/old.json").read_bytes() == before
        assert git(kept, "status", "--porcelain") == ""
    finally:
        for tree in work.glob("wt-*"):
            git(world["repo"], "worktree", "remove", str(tree))


def test_a_table_destination_created_during_the_comparison_is_not_overwritten(world, capsys):
    table = world["tmp"] / "late.md"
    code, _, scratch = world["run"](table=table, env={"STUB_CREATE": str(table)})
    assert code == 2 and "came into existence during the comparison" in capsys.readouterr().err
    assert table.read_text() == "earlier evidence"
    assert (scratch / "results.json").is_file()


def test_a_table_destination_that_became_a_work_tree_during_the_comparison_is_refused(world, capsys):
    table = world["tmp"] / "becomes-a-repo" / "table.md"
    table.parent.mkdir()
    code, _, _ = world["run"](table=table, env={"STUB_GIT_INIT": str(table.parent)})
    assert code == 2 and "inside a git work tree" in capsys.readouterr().err
    assert not table.exists()


# ------------------------------------------------------------------ hosted selection

def test_hosted_selection_runs_this_test_when_the_tool_or_its_manifest_changes():
    spec = importlib.util.spec_from_file_location("python_ci_for_byte_compare", PROJECT / "tools/ci/python_ci.py")
    python_ci = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(python_ci)
    routing = json.loads((PROJECT / "tools/hosted-ci-routing.json").read_text())
    # Selected from the test's own reads, not from a rule naming it.
    assert not any(prefix.startswith("tools/byte_compare") for prefix in python_ci.PREFIX_TESTS)
    for path in ("tools/byte_compare/byte_compare.py", "tools/byte_compare/manifest.json"):
        assert "piping-python" in python_ci.hosted_ci.route_paths([path], routing)["checks"], path
        selected = python_ci.select(PROJECT, {"event": "pull_request", "paths": [python_ci.PREFIX + path]})
        assert "tests/test_byte_compare_tool.py" in selected, path


# ------------------------------------------------------------------ build attestation (threat (b)), stubbed

@pytest.mark.parametrize("env, needle", [
    ({"STUB_RUSTC_ARGS": "-Zshare-generics"}, "unknown rustc argument '-Zshare-generics'"),
    ({"STUB_RUSTC_ARGS": "--target aarch64-apple-darwin"}, "unknown rustc argument '--target'"),
    ({"STUB_RUSTC_ARGS": "-C linker=/usr/bin/cc"}, "not the runtime settings"),
    ({"STUB_RUSTC_ARGS": "--cfg stray"}, "that its build script did not declare"),
    ({"STUB_RUSTC_ENV": "SNEAKY=1"}, "unexpected environment variable SNEAKY"),
    ({"STUB_FRESH": "1"}, "(fresh)"),
    ({"STUB_UPLIFT_DIFFERS": "1"}, "is not the file rustc wrote"),
    ({"STUB_RUSTC_B": "1"}, "rustc -vV differs between the sides"),
])
def test_a_build_the_attestation_does_not_establish_is_refused_before_any_run(world, env, needle, capsys):
    code, table, scratch = world["run"](env=env)
    err = capsys.readouterr().err
    assert code == 2 and table == "" and needle in err, err
    assert not (scratch / "runs").exists() and not (scratch / "results.json").exists()
    assert "wt-" not in git(world["repo"], "worktree", "list")


def test_the_build_log_is_kept_with_its_digest_beside_the_binarys(world):
    code, table, scratch = world["run"]()
    assert code == 0, table
    results = json.loads((scratch / "results.json").read_text())
    for side in ("A", "B"):
        build = results["sides"][side]["build"]
        assert bc.sha256_file(scratch / f"build-{side}.log") == build["log_sha256"]
        assert bc.sha256_file(scratch / f"build-{side}.jsonl") == build["messages_sha256"]
        assert build["rustc_output_sha256"] == build["binary_sha256"] == results["sides"][side]["binary_sha256"]
        assert build["attestation"]["runtime_units"] == ["open_pipe_stress_release_timing 0.1.0 release_timing"]
    assert "rustc output = uplifted binary = copy run" in table


def test_the_build_blocks_are_closed():
    good = json.loads(manifest_text([{"path": "f/a.json", "source_version": "1", "expect": NUMERICAL}]))
    bc.load_manifest(json.dumps(good))
    for mutate in (lambda m: m.pop("build_settings"),
                   lambda m: m["build_settings"]["runtime"]["codegen"].pop("lto"),
                   lambda m: m["build_settings"]["runtime"]["codegen"].update({"target-cpu": "native"}),
                   lambda m: m["build_settings"]["host"]["proc_macro_codegen"].update({"opt-level": "3"}),
                   lambda m: m["build_settings"]["host"]["profile"].update({"opt_level": 0}),
                   lambda m: m["build_closure"].update({"registry": "git+https://example.invalid/index"})):
        bad = json.loads(json.dumps(good))
        mutate(bad)
        with pytest.raises(bc.ManifestError):
            bc.load_manifest(json.dumps(bad))


# ------------------------------------------------------------------ the captured build of the real driver

CAPTURED = PROJECT / "tools" / "byte_compare" / "captured_build"


def edit(text, old, new, count=1):
    assert text.count(old) == count, (old, text.count(old))
    return text.replace(old, new)


def edit_line(text, marker, old, new):
    """Replaces `old` with `new` in the one line containing `marker`."""
    lines = text.split("\n")
    hits = [i for i, line in enumerate(lines) if marker in line]
    assert len(hits) == 1, (marker, len(hits))
    lines[hits[0]] = edit(lines[hits[0]], old, new)
    return "\n".join(lines)


def edit_message(text, package_id, kind, change):
    """Applies `change` to the one JSON message for `package_id` of the given target kind (or reason)."""
    lines, hits = text.split("\n"), 0
    for i, line in enumerate(lines):
        if not line.startswith("{"):
            continue
        message = json.loads(line)
        if message.get("package_id") == package_id and kind in (message.get("reason"), tuple(message.get("target", {}).get("kind", []))):
            change(message)
            lines[i], hits = json.dumps(message), hits + 1
    assert hits == 1, (package_id, kind, hits)
    return "\n".join(lines)


REGISTRY = "registry+https://github.com/rust-lang/crates.io-index"
BIN_LINE = "--crate-name release_timing --edition=2021 src/main.rs"


def attest_captured(stdout=lambda t: t, stderr=lambda t: t, lock=lambda t: t, observed=lambda o: o, calls=None):
    seen = json.loads((CAPTURED / "observed.json").read_text())
    seen = observed(seen)
    parsed = bc.parse_build_output(stdout((CAPTURED / "stdout.jsonl").read_text()),
                                   stderr((CAPTURED / "stderr.log").read_text()))
    rustc = {"program": "/SYSROOT/bin/rustc", "version": seen["rustc_version"], "sysroot": "/SYSROOT", "host": seen["host"]}
    manifest = bc.load_manifest(MANIFEST.read_text())
    record = calls if calls is not None else []
    return bc.attest_build(parsed, tree="/WT", target="/TARGET", lock_text=lock((CAPTURED / "Cargo.lock").read_text()),
                           settings=manifest["build_settings"], closure=manifest["build_closure"], rustc=rustc,
                           list_generated=lambda out_dir: seen["out_dirs"][out_dir],
                           verify_registry=lambda *args: record.append(args))


def test_the_captured_build_of_the_driver_is_attested():
    calls = []
    attestation = attest_captured(calls=calls)
    assert len(attestation["runtime_units"]) == 36 and len(attestation["host_units"]) == 16
    assert attestation["build_scripts"] == 10 and attestation["registry_packages"] == 22 == len(calls)
    assert "serde_json 1.0.149 serde_json" in attestation["runtime_units"]
    assert {"serde_derive 1.0.228 serde_derive", "syn 2.0.117 syn", "libc 0.2.186 build_script_build"} \
        <= set(attestation["host_units"])
    assert sorted(attestation["generated"]) == [f"{REGISTRY}#serde@1.0.228", f"{REGISTRY}#serde_core@1.0.228"]
    assert attestation["runner"] == {"rustc_output": "/TARGET/release/deps/release_timing-5593dae461ac0171",
                                     "executable": "/TARGET/release/release_timing"}
    # Each registry package is verified with the source, name, version and checksum Cargo.lock gives.
    assert ("/CARGO_HOME", "index.crates.io-1949cf8c6b5b557f", "serde_json", "1.0.149",
            "83fc039473c5595ace860d8c4fafa220ff474b3fc6bfdb4293327f1a37e94d86") in calls


ITOA = f"{REGISTRY}#itoa@1.0.18"
CAPTURED_REFUSALS = {
    # Runtime profile checks: a unit the runner links (transitively, through serde_json) at another setting.
    "a linked dependency at opt-level 0": (dict(
        stdout=lambda t: edit_message(t, ITOA, ("lib",), lambda m: m["profile"].update(opt_level="0")),
        stderr=lambda t: edit_line(t, "--crate-name itoa ", "-C opt-level=3", "-C opt-level=0")),
        "itoa@1.0.18 (itoa, runtime) was built with profile"),
    "a linked dependency's -C flags alone": (dict(
        stderr=lambda t: edit_line(t, "--crate-name itoa ", "-C embed-bitcode=no", "-C embed-bitcode=no -C lto=fat")),
        "itoa@1.0.18 (itoa, runtime) was compiled with -C"),
    "a linker on the runner": (dict(stderr=lambda t: edit_line(t, BIN_LINE, "--out-dir", "-C linker=/usr/bin/cc --out-dir")),
                               "(release_timing, runtime) was compiled with -C"),
    # Build-time dependency checks.
    "a build script under a build-override": (dict(
        stdout=lambda t: edit_message(t, f"{REGISTRY}#proc-macro2@1.0.106", ("custom-build",),
                                      lambda m: m["profile"].update(debug_assertions=True)),
        stderr=lambda t: edit_line(t, "--crate-name build_script_build --edition=2021 /CARGO_HOME/registry/src/"
                                      "index.crates.io-1949cf8c6b5b557f/proc-macro2", "debug-assertions=off", "debug-assertions=on")),
        "proc-macro2@1.0.106 (build_script_build, host) was built with profile"),
    "a proc macro optimised": (dict(stderr=lambda t: edit_line(t, "--crate-name serde_derive ", "-C embed-bitcode=no",
                                                               "-C opt-level=3 -C embed-bitcode=no")),
                               "serde_derive@1.0.228 (serde_derive, host) was compiled with -C"),
    "a build script's link input": (dict(stdout=lambda t: edit_message(
        t, f"{REGISTRY}#serde@1.0.228", "build-script-executed", lambda m: m.update(linked_libs=["evil"]))),
        "adds link inputs"),
    "an unknown build-script output": (dict(stdout=lambda t: t + "[serde 1.0.228] cargo:rustc-link-lib=evil\n"),
                                       "unknown build-script output"),
    "a build-script line that is no directive": (dict(stdout=lambda t: t + "[serde 1.0.228] hello\n"),
                                                 "unknown build-script output"),
    "an unknown generated file": (dict(observed=lambda o: (o["out_dirs"][next(k for k in o["out_dirs"] if "/zmij-" in k)]
                                                            .update({"gen.rs": "0" * 64}), o)[1]),
                                  "zmij@1.0.21's build script generated ['gen.rs']"),
    "a build script run with rustflags": (dict(stderr=lambda t: edit_line(
        t, "proc-macro2-5d24402fc79aac90/build-script-build`", "CARGO_ENCODED_RUSTFLAGS=''",
        "CARGO_ENCODED_RUSTFLAGS=-Ctarget-cpu=native")), "ran with CARGO_ENCODED_RUSTFLAGS='-Ctarget-cpu=native'"),
    "a build script run beside a wrapper": (dict(stderr=lambda t: edit_line(
        t, "proc-macro2-5d24402fc79aac90/build-script-build`", "CARGO_ENCODED_RUSTFLAGS=''",
        "CARGO_ENCODED_RUSTFLAGS='' RUSTC_WRAPPER=/HOME/bin/wrap")), "unexpected environment variable RUSTC_WRAPPER"),
    # Flags and environment.
    "-Z": (dict(stderr=lambda t: edit_line(t, BIN_LINE, "--out-dir", "-Zshare-generics --out-dir")),
           "unknown rustc argument '-Zshare-generics'"),
    "--target": (dict(stderr=lambda t: edit_line(t, BIN_LINE, "--out-dir", "--target aarch64-apple-darwin --out-dir")),
                 "unknown rustc argument '--target'"),
    "a library": (dict(stderr=lambda t: edit_line(t, BIN_LINE, "--out-dir", "-l evil --out-dir")), "unknown rustc argument '-l'"),
    "-L native": (dict(stderr=lambda t: edit_line(t, BIN_LINE, "--out-dir", "-L native=/HOME/lib --out-dir")),
                  "searches -L native=/HOME/lib"),
    "a stray --cfg": (dict(stderr=lambda t: edit_line(t, BIN_LINE, "--out-dir", "--cfg stray --out-dir")),
                      "that its build script did not declare"),
    "a feature cargo did not resolve": (dict(stderr=lambda t: edit_line(t, "--crate-name itoa ", "--out-dir",
                                                                         "--cfg 'feature=\"extra\"' --out-dir")),
                                        "feature cfgs are not cargo's resolved features"),
    "an unexpected environment variable": (dict(stderr=lambda t: edit_line(t, BIN_LINE, "CARGO_BIN_NAME=", "SNEAKY=1 CARGO_BIN_NAME=")),
                                           "unexpected environment variable SNEAKY"),
    "a second compiler": (dict(stderr=lambda t: edit_line(t, "--crate-name itoa ", "/SYSROOT/bin/rustc", "/HOME/other/bin/rustc")),
                          "runs /HOME/other/bin/rustc, not the build's rustc"),
    "a prebuilt library linked": (dict(stderr=lambda t: edit_line(t, BIN_LINE, "/TARGET/release/deps/libsha2-071a591afd31b5ab.rlib",
                                                                  "/HOME/prebuilt/libsha2.rlib")),
                                  "links /HOME/prebuilt/libsha2.rlib, which this build did not produce"),
    # Provenance.
    "a registry package not in Cargo.lock": (dict(lock=lambda t: edit(t, 'name = "itoa"\nversion = "1.0.18"',
                                                                      'name = "itoa"\nversion = "1.0.17"')),
                                             "itoa@1.0.18 is not in Cargo.lock with that source"),
    "a registry package without a checksum": (dict(lock=lambda t: edit(
        t, 'name = "itoa"\nversion = "1.0.18"\nsource = "registry+https://github.com/rust-lang/crates.io-index"\nchecksum',
        'name = "itoa"\nversion = "1.0.18"\nsource = "registry+https://github.com/rust-lang/crates.io-index"\nnochecksum')),
        "itoa@1.0.18 has no checksum in Cargo.lock"),
    "source replacement": (dict(stdout=lambda t: edit_message(t, ITOA, ("lib",), lambda m: (
        m["target"].update(src_path="/HOME/vendor/itoa-1.0.18/src/lib.rs"),
        m.update(manifest_path="/HOME/vendor/itoa-1.0.18/Cargo.toml")))), "is not compiled from its registry source"),
    "a git source": (dict(stdout=lambda t: edit(t, ITOA, "git+https://example.invalid/itoa#itoa@1.0.18")),
                     "comes from neither the generated worktree nor"),
    "a path source outside the worktree": (dict(stdout=lambda t: edit(t, "path+file:///WT/core/units#",
                                                                      "path+file:///ELSEWHERE/core/units#")),
                                           "is a path source outside the generated worktree"),
    # The record itself.
    "a fresh unit": (dict(stdout=lambda t: edit_message(t, ITOA, ("lib",), lambda m: m.update(fresh=True))), "(fresh)"),
    "a unit with no invocation": (dict(stderr=lambda t: "\n".join(l for l in t.split("\n") if "--crate-name itoa " not in l)),
                                  "itoa@1.0.18 (itoa) has no compiler invocation"),
    "an unparseable stdout line": (dict(stdout=lambda t: t + "garbage\n"), "neither cargo JSON nor build-script output"),
    "an unknown message": (dict(stdout=lambda t: t + '{"reason": "something-new"}\n'), "unknown build message 'something-new'"),
    "an unknown stderr line": (dict(stderr=lambda t: t + "warning: unused manifest key\n"), "unknown build output on stderr"),
    "a build that never finished": (dict(stdout=lambda t: edit(t, '{"reason":"build-finished","success":true}', "")),
                                    "exactly one successful build-finished"),
}


@pytest.mark.parametrize("case", sorted(CAPTURED_REFUSALS))
def test_a_departure_in_the_captured_build_is_refused(case):
    edits, needle = CAPTURED_REFUSALS[case]
    with pytest.raises(bc.BuildRefused) as refused:
        attest_captured(**edits)
    assert needle in str(refused.value), str(refused.value)


def test_a_wrapper_or_second_compiler_is_not_the_build_compiler():
    stdout = (CAPTURED / "stdout.jsonl").read_text()
    stderr = (CAPTURED / "stderr.log").read_text()
    assert bc.compiler_program(bc.parse_build_output(stdout, stderr)) == "/SYSROOT/bin/rustc"
    for old, new, needle in (("/SYSROOT/bin/rustc --crate-name itoa ", "/HOME/bin/sccache /SYSROOT/bin/rustc --crate-name itoa ",
                              "compiled by 2 programs"),
                             ("/SYSROOT/bin/rustc", "/HOME/bin/wrap", "not rustc named by an absolute path")):
        changed = stderr.replace(old, new)
        assert changed != stderr
        with pytest.raises(bc.BuildRefused, match=needle):
            bc.compiler_program(bc.parse_build_output(stdout, changed))


def test_the_compiler_must_be_its_own_sysroots_rustc(tmp_path):
    def fake(sysroot_printed):
        rustc = tmp_path / "bin" / "rustc"
        rustc.parent.mkdir(parents=True, exist_ok=True)
        rustc.write_text(f"#!{sys.executable}\nimport sys\nif sys.argv[1:] == ['-vV']: print('rustc 9.9.9\\nhost: h')\n"
                         f"else: print({sysroot_printed!r})\n")
        rustc.chmod(0o755)
        return str(rustc)
    assert bc.rustc_identity(fake(str(tmp_path)))["host"] == "h"
    with pytest.raises(bc.BuildRefused, match="is not the rustc of its own sysroot"):
        bc.rustc_identity(fake(str(tmp_path / "elsewhere")))


# ------------------------------------------------------------------ registry provenance

def registry_home(tmp_path, files=None):
    """A Cargo home holding demo 1.0.0's archive and its extracted source; returns (home, checksum)."""
    files = files or {"Cargo.toml": b"[package]\nname = \"demo\"\n", "src/lib.rs": b"pub fn f() {}\n"}
    index = "index.crates.io-0123456789abcdef"
    buffer = io.BytesIO()
    with tarfile.open(fileobj=buffer, mode="w:gz") as archive:
        for name, data in files.items():
            info = tarfile.TarInfo(f"demo-1.0.0/{name}")
            info.size = len(data)
            archive.addfile(info, io.BytesIO(data))
    home = tmp_path / "cargo-home"
    crate = home / "registry" / "cache" / index / "demo-1.0.0.crate"
    crate.parent.mkdir(parents=True)
    crate.write_bytes(buffer.getvalue())
    root = home / "registry" / "src" / index / "demo-1.0.0"
    for name, data in files.items():
        (root / name).parent.mkdir(parents=True, exist_ok=True)
        (root / name).write_bytes(data)
    (root / ".cargo-ok").write_text('{"v":1}')
    return home, index, hashlib.sha256(buffer.getvalue()).hexdigest(), root


def test_a_registry_package_matching_its_locked_archive_is_accepted(tmp_path):
    home, index, checksum, _ = registry_home(tmp_path)
    bc.verify_registry_package(str(home), index, "demo", "1.0.0", checksum)


@pytest.mark.parametrize("tamper, needle", [
    ("checksum", "is not Cargo.lock's checksum"),
    ("edited", "is not the locked archive"),
    ("added", "is not the locked archive"),
    ("no archive", "no cached archive"),
])
def test_a_registry_package_that_is_not_its_locked_archive_is_refused(tmp_path, tamper, needle):
    home, index, checksum, root = registry_home(tmp_path)
    if tamper == "checksum":
        checksum = "0" * 64
    elif tamper == "edited":
        (root / "src" / "lib.rs").write_bytes(b"pub fn f() { evil() }\n")
    elif tamper == "added":
        (root / "build.rs").write_bytes(b"fn main() {}\n")
    else:
        next((home / "registry" / "cache").rglob("*.crate")).unlink()
    with pytest.raises(bc.BuildRefused, match=needle):
        bc.verify_registry_package(str(home), index, "demo", "1.0.0", checksum)


# ------------------------------------------------------------------ real cargo: inherited configuration

RUNS_CARGO = pytest.mark.runs_cargo("cargo builds of a synthetic two-crate repository through the tool")

SYNTHETIC = {
    bc.VERSION_SOURCE: 'pub const MODEL_SCHEMA_VERSION: &str = "2.0.0";\n',
    "f/one.json": json.dumps({"model": {"schema_version": "2.0.0"}}),
    f"{bc.DRIVER_CRATE}/Cargo.toml": textwrap.dedent('''\
        [package]
        name = "open_pipe_stress_release_timing"
        version = "0.1.0"
        edition = "2021"
        publish = false

        [workspace]

        [[bin]]
        name = "release_timing"
        path = "src/main.rs"

        [dependencies]
        dep = { path = "../dep" }
        '''),
    f"{bc.DRIVER_CRATE}/Cargo.lock": textwrap.dedent('''\
        # This file is automatically @generated by Cargo.
        # It is not intended for manual editing.
        version = 4

        [[package]]
        name = "dep"
        version = "0.1.0"

        [[package]]
        name = "open_pipe_stress_release_timing"
        version = "0.1.0"
        dependencies = [
         "dep",
        ]
        '''),
    # The driver's identity, embedded by its build script from the tree it is built in.
    f"{bc.DRIVER_CRATE}/build.rs": textwrap.dedent('''\
        use std::process::Command;
        fn git(args: &[&str]) -> String {
            let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
            let out = Command::new("git").arg("-C").arg(&dir).args(args).output().expect("git");
            String::from_utf8(out.stdout).unwrap().trim().to_string()
        }
        fn main() {
            println!("cargo:rerun-if-changed=identity-is-read-on-every-build.absent");
            println!("cargo:rustc-env=SYN_COMMIT={}", git(&["rev-parse", "HEAD"]));
            println!("cargo:rustc-env=SYN_TREE={}", git(&["rev-parse", "HEAD^{tree}"]));
            println!("cargo:rustc-env=SYN_DIRTY={}", !git(&["status", "--porcelain"]).is_empty());
            println!("cargo:rustc-env=SYN_PROFILE={}", std::env::var("PROFILE").unwrap());
        }
        '''),
    f"{bc.DRIVER_CRATE}/src/main.rs": textwrap.dedent('''\
        fn main() {
            let args: Vec<String> = std::env::args().skip(1).collect();
            if args == ["--build-identity"] {
                println!("{{\\"source_commit\\": \\"{}\\", \\"source_tree\\": \\"{}\\", \\"source_dirty\\": {}, \\"build_profile\\": \\"{}\\", \\"generated\\": {}}}",
                         env!("SYN_COMMIT"), env!("SYN_TREE"), env!("SYN_DIRTY"), env!("SYN_PROFILE"), dep::built_with_debug_assertions());
                return;
            }
            println!("{{\\"schema\\": \\"release-timing-v2\\", \\"invocation\\": \\"solve\\", \\"status\\": \\"entry_error\\", \\"entry_error\\": {{\\"stage\\": \\"entry\\", \\"message\\": \\"synthetic\\"}}}}");
            std::process::exit(3);
        }
        '''),
    # The path dependency: its build script generates source the library includes, and what it
    # generates depends on the settings the build script itself was compiled with.
    "core/solver/dep/Cargo.toml": '[package]\nname = "dep"\nversion = "0.1.0"\nedition = "2021"\npublish = false\n',
    "core/solver/dep/build.rs": textwrap.dedent('''\
        fn main() {
            let out = std::env::var("OUT_DIR").unwrap();
            std::fs::write(format!("{}/gen.rs", out),
                           format!("pub const BUILT_WITH_DEBUG_ASSERTIONS: bool = {};\\n", cfg!(debug_assertions))).unwrap();
            println!("cargo:rerun-if-changed=build.rs");
        }
        '''),
    "core/solver/dep/src/lib.rs": textwrap.dedent('''\
        include!(concat!(env!("OUT_DIR"), "/gen.rs"));
        pub fn built_with_debug_assertions() -> bool { BUILT_WITH_DEBUG_ASSERTIONS }
        '''),
}


def cargo_world(tmp_path, side_files=None):
    """The synthetic repository: a base commit, and A and B each the base plus their own files."""
    repo = tmp_path / "repo"
    repo.mkdir()
    git(repo, "init", "-q")
    for rel, text in SYNTHETIC.items():
        write(repo, rel, text)
    git(repo, "add", "-A")
    git(repo, "commit", "-q", "-m", "base")
    base = git(repo, "rev-parse", "HEAD")
    revisions = {}
    for side in ("A", "B"):
        files = (side_files or {}).get(side, {})
        git(repo, "checkout", "-q", "--detach", base)
        if files:
            for rel, text in files.items():
                write(repo, rel, text)
            git(repo, "add", "-A")
            git(repo, "commit", "-q", "-m", side)
        revisions[side] = git(repo, "rev-parse", "HEAD")
    manifest = tmp_path / "manifest.json"
    manifest.write_text(json.dumps({
        "schema": bc.MANIFEST_SCHEMA, "roots": [{"pattern": "f/*.json", "why": "synthetic"}],
        "entries": [{"path": "f/one.json", "source_version": "2.0.0",
                     "expect": {"disposition": "entry_refusal", "numerical": False}}],
        "build_settings": BUILD_BLOCKS["build_settings"],
        "build_closure": dict(BUILD_BLOCKS["build_closure"], generated_sources={"dep": ["gen.rs"]})}))
    home = tmp_path / "cargo-home"
    home.mkdir()
    return {"repo": repo, "manifest": manifest, "home": home, "tmp": tmp_path, **revisions}


def reachable_cargo_configs(world):
    """Cargo configuration files cargo could read for builds under the synthetic world: in any ancestor
    of its folder (where the repository, worktrees and target directories are) and in its Cargo home."""
    found = [world["home"] / name for name in ("config", "config.toml")]
    for place in [world["tmp"], *world["tmp"].resolve().parents]:
        found += [place / ".cargo" / name for name in ("config", "config.toml")]
    return [path for path in found if path.is_file()]


def run_real(world, capsys):
    saved = dict(os.environ)
    os.environ.update(CARGO_HOME=str(world["home"]), CARGO_NET_OFFLINE="true")
    scratch = world["tmp"] / "scratch"
    try:
        code = bc.main(["compare", "--a", world["A"], "--b", world["B"], "--scratch", str(scratch),
                        "--repo", str(world["repo"]), "--manifest", str(world["manifest"]), "--cargo", "cargo",
                        "--jobs", "2", "--timeout", "60"])
    finally:
        os.environ.clear()
        os.environ.update(saved)
    return code, capsys.readouterr(), scratch


def generated_source(world, side):
    found = list((world["tmp"] / "scratch" / "build" / "target").glob(f"{side}-*/release/build/dep-*/out/gen.rs"))
    assert len(found) == 1, found
    return found[0].read_text()


@RUNS_CARGO
def test_the_default_release_build_attests_under_controlled_configuration(tmp_path, capsys):
    # The control for the manifest's build_settings block: cargo's default release build of the
    # synthetic repository, with the tool's clean environment, an empty Cargo home and no cargo
    # configuration in any ancestor, must attest with exactly that block.
    world = cargo_world(tmp_path)
    assert reachable_cargo_configs(world) == [], "a cargo configuration is reachable; the control is not controlled"
    code, out, scratch = run_real(world, capsys)
    assert code == 0, out.err + out.out
    results = json.loads((scratch / "results.json").read_text())
    for side in ("A", "B"):
        attestation = results["sides"][side]["build"]["attestation"]
        assert attestation["runtime_units"] == ["dep 0.1.0 dep", "open_pipe_stress_release_timing 0.1.0 release_timing"]
        assert attestation["host_units"] == ["dep 0.1.0 build_script_build",
                                             "open_pipe_stress_release_timing 0.1.0 build_script_build"]
        assert list(attestation["generated"].values()) == [{"gen.rs": bc.sha256_bytes(generated_source(world, side).encode())}]
        assert "= false;" in generated_source(world, side)


@RUNS_CARGO
@pytest.mark.parametrize("where", ["A", "B", "outside"])
def test_an_ancestor_cargo_config_with_opt_level_0_is_refused(tmp_path, capsys, where):
    config = "[profile.release]\nopt-level = 0\n"
    if where == "outside":
        write(tmp_path, ".cargo/config.toml", config)   # an ancestor of every worktree: side A meets it first
        world = cargo_world(tmp_path)
    else:
        world = cargo_world(tmp_path, {where: {".cargo/config.toml": config}})  # an ancestor of that side's crate
    code, out, scratch = run_real(world, capsys)
    refused = "B" if where == "B" else "A"
    assert code == 2 and f"side {refused} (" in out.err and "runtime) was built with profile" in out.err, out.err
    assert not (scratch / "runs").exists()


@RUNS_CARGO
def test_a_cargo_home_config_adding_rustflags_is_refused(tmp_path, capsys):
    world = cargo_world(tmp_path)
    write(world["home"], "config.toml", '[build]\nrustflags = ["-C", "target-cpu=native"]\n')
    code, out, scratch = run_real(world, capsys)
    assert code == 2 and "side A (" in out.err and "'target-cpu': 'native'" in out.err, out.err
    assert not (scratch / "runs").exists()


@RUNS_CARGO
def test_a_dependency_specific_profile_override_is_refused(tmp_path, capsys):
    world = cargo_world(tmp_path)
    write(world["home"], "config.toml", "[profile.release.package.dep]\nopt-level = 0\n")
    code, out, scratch = run_real(world, capsys)
    assert code == 2 and "side A (" in out.err and "/core/solver/dep#0.1.0 (dep, runtime) was built with profile" in out.err, out.err
    assert not (scratch / "runs").exists()


@RUNS_CARGO
def test_a_rustc_wrapper_is_refused(tmp_path, capsys):
    world = cargo_world(tmp_path)
    wrapper = tmp_path / "wrap.sh"
    wrapper.write_text('#!/bin/sh\nexec "$@"\n')
    wrapper.chmod(0o755)
    write(world["home"], "config.toml", f'[build]\nrustc-wrapper = "{wrapper}"\n')
    code, out, scratch = run_real(world, capsys)
    assert code == 2 and "side A (" in out.err and f"the compiler {wrapper} is not rustc" in out.err, out.err
    assert not (scratch / "runs").exists()


@RUNS_CARGO
@pytest.mark.parametrize("side", ["A", "B"])
def test_a_build_override_that_changes_generated_source_is_refused(tmp_path, capsys, side):
    # The build script is compiled with debug assertions on, so the source it generates for the
    # dependency differs, while every runtime unit's own flags stay exactly the release settings.
    world = cargo_world(tmp_path, {side: {".cargo/config.toml": "[profile.release.build-override]\ndebug-assertions = true\n"}})
    code, out, scratch = run_real(world, capsys)
    assert code == 2 and f"side {side} (" in out.err and "build_script_build, host) was built with profile" in out.err, out.err
    assert not (scratch / "runs").exists()
    assert "= true;" in generated_source(world, side)
