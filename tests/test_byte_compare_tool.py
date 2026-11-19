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

# The mutation harness sets this so that no test can fail on a diagnostic's wording alone: every
# message check then passes, and a mutation counts as killed only where what the tool did changed.
BEHAVIOUR_ONLY = os.environ.get("BYTE_COMPARE_BEHAVIOUR_ONLY") == "1"


def says(text, needle):
    """Whether a diagnostic names its reason (always, in behaviour-only mode)."""
    return BEHAVIOUR_ONLY or needle in text


def nothing_ran(scratch):
    """No run output was written (the run directories are reserved empty first) and the reserved results
    file holds nothing."""
    runs, results = Path(scratch) / "runs", Path(scratch) / "results.json"
    return not (runs.is_dir() and any(p.is_file() for p in runs.rglob("*"))) \
        and not (results.is_file() and results.stat().st_size)


def refused_with(needle, call, *args, **kwargs):
    """`call` must raise BuildRefused (behaviour); its message must name `needle` (wording)."""
    with pytest.raises(bc.BuildRefused) as refused:
        call(*args, **kwargs)
    assert says(str(refused.value), needle), str(refused.value)


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
    with open(os.environ["STUB_CARGO_LOG"], "a") as handle:
        handle.write(json.dumps(args) + "\\n")
    side = target.name.split("-")[0]
    scratch = os.environ.get("STUB_SCRATCH")
    if os.environ.get("STUB_CARGO_HOOK"):   # a test's interleaving, run inside this build
        exec(open(os.environ["STUB_CARGO_HOOK"]).read())
    # During side A's build: plant a link where the tool will write later (STUB_PLANT = path:kind), or
    # swap a reserved directory for a link elsewhere (STUB_SWAP = path, STUB_SWAP_TO = destination).
    if side == "A" and os.environ.get("STUB_PLANT"):
        rel, kind = os.environ["STUB_PLANT"].rsplit(":", 1)
        where, frozen = os.path.join(scratch, rel), os.environ["STUB_FROZEN"]
        try:
            os.symlink(frozen, where) if kind == "symlink" else os.link(frozen, where)
        except FileExistsError:   # the tool already holds the name
            open(os.environ["STUB_CARGO_LOG"] + ".plant-refused", "w").close()
    if side == "A" and os.environ.get("STUB_REPLACE_BUILD"):
        replaced = os.path.join(scratch, os.environ["STUB_REPLACE_BUILD"])
        if os.environ.get("STUB_REPLACE_MODE") == "rename":
            os.rename(replaced, replaced + ".away")
            open(replaced, "w").write("a new file at the reserved name")
        else:
            os.unlink(replaced)
            os.link(os.environ["STUB_FROZEN"], replaced)
    if side == "A" and os.environ.get("STUB_SWAP"):
        where = os.path.normpath(os.path.join(scratch, os.environ["STUB_SWAP"]))
        os.rename(where, where + ".moved")
        os.symlink(os.environ["STUB_SWAP_TO"], where)
    if os.environ.get("STUB_NO_LINK"):
        sys.exit(0)
    identity = {"source_commit": os.environ.get("STUB_COMMIT") or git("rev-parse", "HEAD"),
                "source_tree": os.environ.get("STUB_TREE") or git("rev-parse", "HEAD^{tree}"),
                "source_dirty": bool(git("status", "--porcelain")) or bool(os.environ.get("STUB_DIRTY")),
                "build_profile": os.environ.get("STUB_PROFILE") or ("release" if "--release" in args else "debug"),
                "rustc": ["rustc 0.0.0-stub"], "cargo": ["cargo 0.0.0-stub"]}
    perturb = (tree / "perturb").exists()
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
    # rustc's dep-info for the unit: its crate root (relative, as rustc reports a primary package's
    # sources), and whatever a test adds (STUB_DEPS).
    deps_listed = " ".join(["src/main.rs", *os.environ.get("STUB_DEPS", "").split()])
    (deps / ("release_timing" + extra + ".d")).write_text(
        f"{deps}/release_timing{extra}.d: {deps_listed}\\n\\n{deps}/release_timing{extra}: {deps_listed}\\n\\n"
        + "".join(f"{d}:\\n" for d in deps_listed.split()) + "\\n# env-dep:CARGO_PKG_VERSION=0.1.0\\n")
    # A stand-in rustc in its own sysroot, answering -vV and --print sysroot.
    sysroot = Path(os.environ["STUB_SYSROOT"]) / side
    rustc = sysroot / "bin" / "rustc"
    rustc.parent.mkdir(parents=True, exist_ok=True)
    release = "0.0.1" if side == "B" and os.environ.get("STUB_RUSTC_B") else "0.0.0"
    version = f"rustc {release}-stub\\nbinary: rustc\\nhost: stub-host\\nrelease: {release}\\n"
    rustc.write_text("#!" + sys.executable + "\\nimport json, os, sys\\n"
                     + "if os.environ.get('STUB_OUTSIDE_LOG') and not os.environ.get('UNDER_RUN_WRAPPER'):\\n"
                     + "    open(os.environ['STUB_OUTSIDE_LOG'], 'a').write(json.dumps(sys.argv) + '\\\\n')\\n"
                     + "if os.environ.get('STUB_RUSTC_HOOK'): exec(open(os.environ['STUB_RUSTC_HOOK']).read())\\n"
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
    # After side A's whole record is written: rename a log away and put an identical copy at its name.
    if side == "A" and os.environ.get("STUB_REPLACE_BUILD_END"):
        sys.stdout.flush()
        sys.stderr.flush()
        replaced = os.path.join(scratch, os.environ["STUB_REPLACE_BUILD_END"])
        os.rename(replaced, replaced + ".away")
        with open(replaced + ".away", "rb") as old, open(replaced, "wb") as new:
            new.write(old.read())
''')

STUB_DRIVER = textwrap.dedent('''\
    import hashlib, json, os, sys
    with open(os.environ["STUB_RUN_LOG"], "a") as handle:
        handle.write(json.dumps(sys.argv[1:]) + "\\n")
    if os.environ.get("STUB_DRIVER_HOOK"):   # a test's interleaving, run inside this start of the driver
        exec(open(os.environ["STUB_DRIVER_HOOK"]).read())
    # At its first solve (or, with STUB_REPLACE_WRITTEN, the first once the file holds bytes), replace a
    # reserved file: by a hard link to a frozen request, or (STUB_REPLACE_MODE=rename) by renaming it away
    # and creating a new regular file at its name.
    target = os.environ.get("STUB_REPLACE")
    if target and sys.argv[1:] != ["--build-identity"] and not os.path.exists(target + ".done") \\
            and (not os.environ.get("STUB_REPLACE_WRITTEN") or os.path.getsize(target) > 0):
        open(target + ".done", "w").close()
        if os.environ.get("STUB_REPLACE_MODE") == "rename":
            os.rename(target, target + ".away")
            open(target, "w").write("a new file at the reserved name")
        else:
            os.unlink(target)
            os.link(os.environ["STUB_FROZEN"], target)
    if os.environ.get("STUB_OUTSIDE_LOG") and not os.environ.get("UNDER_RUN_WRAPPER"):
        with open(os.environ["STUB_OUTSIDE_LOG"], "a") as handle:
            handle.write(json.dumps(sys.argv) + "\\n")
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
    write(repo, bc.DRIVER_CRATE + "/src/main.rs", "fn main() {}\n")
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

    def run(a_rev=a, b_rev=b, env=None, manifest_path=manifest, scratch=None, work=None, table="table.md", keep=False,
            wrapper=None):
        scratch = scratch or tmp_path / f"scratch-{len(list(tmp_path.glob('scratch-*')))}"
        saved = dict(os.environ)
        os.environ.update({"STUB_DRIVER": str(tools / "driver.py"), "STUB_SYSROOT": str(tmp_path / "sysroot"),
                           "STUB_BUILD": json.dumps(BUILD_BLOCKS["build_settings"]["runtime"]),
                           "STUB_CARGO_LOG": str(tmp_path / "cargo.log"), "STUB_RUN_LOG": str(tmp_path / "runs.log"),
                           "STUB_SCRATCH": str(scratch), "STUB_FROZEN": str(repo / "f/old.json"), **(env or {})})
        try:
            code = bc.main(["compare", "--a", a_rev, "--b", b_rev, "--scratch", str(scratch), "--repo", str(repo),
                            "--manifest", str(manifest_path), "--cargo", f"{sys.executable} {tools / 'cargo.py'}",
                            "--timeout", "60"] + (["--table", table] if table else [])
                           + (["--work", str(work)] if work else []) + (["--keep-worktrees"] if keep else [])
                           + (["--run-wrapper", str(wrapper)] if wrapper else []))
        finally:
            os.environ.clear()
            os.environ.update(saved)
        written = scratch / table if table else None
        text = written.read_text() if written and code in (0, 1) and written.is_file() and not written.is_symlink() else ""
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


def frozen(world):
    return world["repo"] / "f/old.json"


def snapshot(world):
    """What must be preserved: the frozen request's bytes and link count, and the caller's clean status."""
    return frozen(world).read_bytes(), os.stat(frozen(world)).st_nlink, git(world["repo"], "status", "--porcelain")


def built(world):
    return (world["tmp"] / "cargo.log").exists()


def ran(world):
    return (world["tmp"] / "runs.log").exists()


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
    assert says(capsys.readouterr().err, needle)
    assert nothing_ran(scratch)
    lines = (world["tmp"] / "runs.log").read_text().splitlines() if ran(world) else []
    assert all(json.loads(line) == ["--build-identity"] for line in lines), lines
    assert "wt-" not in git(world["repo"], "worktree", "list")


def test_a_frozen_request_changed_during_the_run_fails(world):
    code, table, _ = world["run"](env={"STUB_TAMPER": "f/bare.json"})
    assert code == 1
    assert "f/bare.json changed on disk during the comparison" in table
    assert "the worktree changed during the runs" in table


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


# ------------------------------------------------------------------ writes (threat (c)): every sink

# Each path the tool writes, relative to the scratch area (README "Every sink"). `{a}` is side A's commit.
SINKS = ["", "plan.json", "results.json", "table.md", "build-A.log", "build-A.jsonl", "inputs", "inputs/A/f__old.json.json",
         "runs/A/f__old.json.sparse.stdout", "runs/A/f__old.json.sparse.envelope.json", "build", "build/bin/A-{a}/release_timing",
         "build/target/A-{a}", "build/wt-A-{a12}"]


def plant(world, where, kind):
    """A link at `where` to the frozen request: a symbolic link, or a hard link (a second name for its bytes)."""
    where.parent.mkdir(parents=True, exist_ok=True)
    where.symlink_to(frozen(world)) if kind == "symlink" else os.link(frozen(world), where)


@pytest.mark.parametrize("kind", ["symlink", "hardlink"])
@pytest.mark.parametrize("sink", SINKS)
def test_a_link_at_any_sink_of_an_existing_area_is_refused_before_anything_is_built(world, sink, kind, capsys):
    # The reading's reproduction was the binary copy through a pre-existing work area; every sink is tried.
    scratch = world["tmp"] / "scratch-old"
    if sink:
        scratch.mkdir()
    plant(world, scratch / sink.format(a=world["a"], a12=world["a"][:12]), kind)   # sink "": the area itself
    before = snapshot(world)
    code, _, _ = world["run"](scratch=scratch)
    assert code == 2 and says(capsys.readouterr().err, "already exists; each comparison reserves a new one")
    assert snapshot(world) == before and not built(world) and not ran(world)


def test_an_existing_empty_area_is_refused_before_anything_is_built(world, capsys):
    before = snapshot(world)
    (world["tmp"] / "scratch-empty").mkdir()
    code, _, _ = world["run"](scratch=world["tmp"] / "scratch-empty")
    assert code == 2 and says(capsys.readouterr().err, "already exists; each comparison reserves a new one")
    assert snapshot(world) == before and not built(world) and not ran(world)


@pytest.mark.parametrize("kind", ["symlink", "hardlink"])
def test_a_link_at_a_separate_work_area_is_refused_before_anything_is_built(world, kind, capsys):
    plant(world, world["tmp"] / "work-old", kind)
    before = snapshot(world)   # after the plant: a hard link adds its own name to the request
    code, _, _ = world["run"](work=world["tmp"] / "work-old")
    assert code == 2 and says(capsys.readouterr().err, "already exists; each comparison reserves a new one")
    assert snapshot(world) == before and not built(world) and not ran(world)


def test_an_area_inside_the_callers_work_tree_is_refused_before_anything_is_built(world, capsys):
    before = snapshot(world)
    code, _, _ = world["run"](scratch=world["repo"] / "scratch")
    assert code == 2 and says(capsys.readouterr().err, "inside a git work tree")
    assert snapshot(world) == before and not built(world) and not ran(world)
    assert not (world["repo"] / "scratch").exists()


def test_an_area_inside_a_worktree_kept_by_an_earlier_run_is_refused(world, capsys):
    work = world["tmp"] / "work"
    code, table, _ = world["run"](work=work, keep=True)
    assert code == 0, table
    kept = work / f"wt-A-{world['a'][:12]}"
    (world["tmp"] / "cargo.log").unlink()
    (world["tmp"] / "runs.log").unlink()
    try:
        code, _, _ = world["run"](scratch=kept / "f" / "scratch", work=world["tmp"] / "work-2")
        assert code == 2 and says(capsys.readouterr().err, "inside a git work tree")
        assert git(kept, "status", "--porcelain") == "" and not built(world) and not ran(world)
    finally:
        for tree in work.glob("wt-*"):
            git(world["repo"], "worktree", "remove", str(tree))


@pytest.mark.parametrize("via", ["into the work tree", "missing component then ..", "missing parent"])
def test_an_area_reached_through_an_alias_is_refused_before_anything_is_built(world, via, capsys):
    before = snapshot(world)
    if via == "into the work tree":   # a linked intermediate directory that resolves into the caller's repository
        (world["tmp"] / "alias").symlink_to(world["repo"], target_is_directory=True)
        given, needle = world["tmp"] / "alias" / "scratch", "inside a git work tree"
    elif via == "missing component then ..":   # checked through /outside while the component exists nowhere
        given, needle = world["tmp"] / "not-created-yet" / ".." / "repo" / "scratch", "does not resolve"
    else:
        given, needle = world["tmp"] / "not-created-yet" / "scratch", "does not resolve"
    code, _, _ = world["run"](scratch=given)
    assert code == 2 and says(capsys.readouterr().err, needle)
    assert snapshot(world) == before and not built(world) and not ran(world)
    assert not (world["repo"] / "scratch").exists() and not (world["tmp"] / "not-created-yet").exists()


def test_an_area_through_a_linked_directory_outside_every_work_tree_is_made_where_it_resolves(world):
    (world["tmp"] / "elsewhere").mkdir()
    (world["tmp"] / "alias").symlink_to(world["tmp"] / "elsewhere", target_is_directory=True)
    code, table, _ = world["run"](scratch=world["tmp"] / "alias" / "scratch")
    assert code == 0, table
    results = json.loads((world["tmp"] / "elsewhere" / "scratch" / "results.json").read_text())
    assert results["sides"]["A"]["inputs"] == str((world["tmp"] / "elsewhere" / "scratch" / "inputs" / "A").resolve())


@pytest.mark.parametrize("kind", ["symlink", "hardlink"])
@pytest.mark.parametrize("sink", ["build/bin/B-{b}/release_timing", "inputs/A/f__old.json.json", "inputs/B/f__bare.json.json",
                                  "runs/A/f__old.json.sparse.stdout", "runs/B/f__old.json.dense.status.json",
                                  "runs/A/f__old.json.sparse.envelope.json"])
def test_a_link_planted_at_a_sink_during_the_build_is_refused_and_nothing_runs(world, sink, kind, capsys):
    before = snapshot(world)
    code, _, _ = world["run"](env={"STUB_PLANT": f"{sink.format(b=world['b'])}:{kind}"})
    assert code == 2 and says(capsys.readouterr().err, "already exists")
    # The bytes are unchanged; a hard-link plant adds its own one name, and nothing else does.
    assert snapshot(world)[:2] == (before[0], before[1] + (kind == "hardlink")) and not ran_a_solve(world)


def ran_a_solve(world):
    lines = (world["tmp"] / "runs.log").read_text().splitlines() if ran(world) else []
    return any(json.loads(line) != ["--build-identity"] for line in lines)


@pytest.mark.parametrize("fixed", ["plan.json", "results.json", "table.md"])
def test_a_fixed_sink_is_reserved_before_the_build_so_a_later_link_cannot_take_it(world, fixed):
    before = snapshot(world)
    code, table, scratch = world["run"](env={"STUB_PLANT": f"{fixed}:hardlink"})
    assert (world["tmp"] / "cargo.log.plant-refused").exists()   # the name was already the tool's
    assert code == 0, table
    assert snapshot(world)[:2] == before[:2] and (scratch / fixed).stat().st_nlink == 1
    assert (scratch / fixed).read_bytes() != before[0]


@pytest.mark.parametrize("swapped", [".", "inputs", "runs", "build", "build/bin"])
def test_a_reserved_directory_swapped_for_a_link_during_the_build_is_refused_and_nothing_runs(world, swapped, capsys):
    before = snapshot(world)
    code, _, _ = world["run"](env={"STUB_SWAP": swapped, "STUB_SWAP_TO": str(world["repo"] / "f")})
    assert code == 2 and says(capsys.readouterr().err, "no longer names the directory")
    assert snapshot(world) == before and not ran_a_solve(world)
    assert sorted(p.name for p in (world["repo"] / "f").iterdir()) == sorted(Path(k).name for k in FIXTURES)


def test_a_run_output_replaced_after_its_reservation_is_refused(world, capsys):
    before = snapshot(world)
    scratch = world["tmp"] / "scratch-replace"
    later = scratch / "runs" / "B" / "f__old.json.dense.stdout"
    code, _, _ = world["run"](scratch=scratch, env={"STUB_REPLACE": str(later)})
    assert code == 2 and says(capsys.readouterr().err, "was replaced after it was reserved")
    assert frozen(world).read_bytes() == before[0]


@pytest.mark.parametrize("swapped", ["inputs", "runs/A", "build/bin/B-{b}"])
def test_a_reserved_directory_swapped_for_a_link_back_to_itself_is_refused(world, swapped, capsys):
    # The held directory is renamed and a link to it put at its name: the same inode is still reached by
    # the path, so only a no-link reopening from the root sees the substitution. For "inputs", the
    # deeper reserved inputs/A and inputs/B are also reached through the substituted parent.
    before = snapshot(world)
    scratch = world["tmp"] / "scratch-self-link"
    where = scratch / swapped.format(b=world["b"])
    code, _, _ = world["run"](scratch=scratch, env={"STUB_SWAP": str(where.relative_to(scratch)),
                                                    "STUB_SWAP_TO": str(where) + ".moved"})
    assert code == 2 and says(capsys.readouterr().err, "no longer names the directory")
    assert snapshot(world) == before and not ran_a_solve(world)
    moved = Path(str(where) + ".moved")
    assert where.is_symlink() and not [p for p in moved.rglob("*") if p.is_file()]   # nothing written through it


def test_a_deeper_reserved_directory_reached_through_a_substituted_parent_is_refused(tmp_path):
    area = bc.Area.reserve(str(tmp_path / "area"), "--scratch")
    try:
        inputs = area.directory("inputs")
        side = inputs.directory("A")
        os.rename(tmp_path / "area" / "inputs", tmp_path / "area" / "inputs.moved")
        (tmp_path / "area" / "inputs").symlink_to("inputs.moved", target_is_directory=True)
        # inputs/A itself is a real directory; only its parent has become a link back to the same place.
        assert not bc.same_directory(side.path, side.fd) and not bc.same_directory(inputs.path, inputs.fd)
        with pytest.raises(bc.SetupError):
            area.verify()
    finally:
        area.close()


@pytest.mark.parametrize("target, when, mode", [
    ("table.md", "solve", "hardlink"), ("results.json", "solve", "hardlink"), ("plan.json", "build", "hardlink"),
    ("table.md", "solve", "rename"), ("results.json", "solve", "rename"), ("plan.json", "build", "rename"),
    ("build-A.log", "build", "hardlink"), ("build-A.jsonl", "build", "rename"),
    ("build-A.log", "build-end", "copy"), ("build-A.jsonl", "build-end", "copy"),
    ("runs/A/f__old.json.sparse.stdout", "written", "hardlink"), ("runs/A/f__old.json.sparse.status.json", "written", "rename"),
    ("inputs/B/f__old.json.json", "solve", "rename"),
])
def test_a_reserved_file_replaced_before_it_is_written_or_read_is_refused(world, target, when, mode, capsys):
    # A reserved file's name is made to point elsewhere: a hard link to the frozen request, or a new
    # regular file after the reserved one is renamed away. The tool refuses rather than writing to (or
    # reading from) anything but the file it reserved, and the frozen request's bytes are unchanged.
    before = snapshot(world)
    scratch = world["tmp"] / "scratch-replace-file"
    env = {"STUB_REPLACE_MODE": mode}
    if when == "build":
        env["STUB_REPLACE_BUILD"] = target
    elif when == "build-end":   # an identical copy: only the identity check can tell it from the original
        env["STUB_REPLACE_BUILD_END"] = target
    else:
        env["STUB_REPLACE"] = str(scratch / target)
        if when == "written":
            env["STUB_REPLACE_WRITTEN"] = "1"
    code, _, _ = world["run"](scratch=scratch, env=env)
    needle = "is not the input the comparison wrote" if target.startswith("inputs/") else "after it was reserved"
    err = capsys.readouterr().err
    assert code == 2 and says(err, needle), err
    assert frozen(world).read_bytes() == before[0] and git(world["repo"], "status", "--porcelain") == ""


# A's copied binary renamed away and replaced by a new file with identical bytes and mode 0755: the same
# digest, a different inode (the reading's C1 case and two later points of the same substitution).
SWAP_A_BINARY = """
import glob
victim = glob.glob(os.path.join(os.environ["STUB_SCRATCH"], "build/bin/A-*/release_timing"))[0]
if not os.path.exists(victim + ".away"):
    os.rename(victim, victim + ".away")
    with open(victim + ".away", "rb") as old, open(victim, "wb") as new:
        new.write(old.read())
    os.chmod(victim, 0o755)
"""
BINARY_SWAP_HOOKS = {
    "during side B's cargo": ("STUB_CARGO_HOOK", "if side == 'B':\n" + textwrap.indent(SWAP_A_BINARY, "    ")),
    "during side B's identity check": ("STUB_DRIVER_HOOK", "if sys.argv[1:] == ['--build-identity'] and '/B-' in sys.argv[0]:\n"
                                       + textwrap.indent(SWAP_A_BINARY, "    ")),
    "during the last run": ("STUB_DRIVER_HOOK", "if '/B/f__blocked.json.json' in ' '.join(sys.argv) and 'dense' in sys.argv:\n"
                            + textwrap.indent(SWAP_A_BINARY, "    ")),
}


@pytest.mark.parametrize("when", sorted(BINARY_SWAP_HOOKS))
def test_the_copied_binary_replaced_by_identical_bytes_is_refused(world, when, capsys):
    before = snapshot(world)
    variable, code_text = BINARY_SWAP_HOOKS[when]
    hook = world["tmp"] / "hook.py"
    hook.write_text(code_text)
    code, _, scratch = world["run"](scratch=world["tmp"] / "scratch-binary", env={variable: str(hook)})
    victim = next((scratch / "build" / "bin").glob("A-*/release_timing"))
    assert Path(str(victim) + ".away").is_file() and victim.read_bytes() == Path(str(victim) + ".away").read_bytes()
    err = capsys.readouterr().err
    needle = "is not the binary the comparison wrote" if when == "during side B's identity check" else "after it was reserved"
    assert code == 2 and says(err, needle), err
    assert snapshot(world) == before
    if when == "during side B's cargo":   # refused before any binary is started at all
        assert not ran(world)
    if when == "during side B's identity check":   # refused by the plan's original identity, before any run
        assert not ran_a_solve(world)


def test_a_build_log_replaced_by_an_identical_copy_after_it_was_parsed_is_refused(world, capsys):
    # The reading's C2 case: after the parse read, during the rustc -vV probe, build-A.log is renamed away
    # and an identical copy put at its name; the later hash read must not take the copy.
    hook = world["tmp"] / "hook.py"
    hook.write_text("""
if sys.argv[1:] == ['-vV'] and '/A/bin/rustc' in sys.argv[0]:
    victim = os.path.join(os.environ['STUB_SCRATCH'], 'build-A.log')
    if not os.path.exists(victim + '.away'):
        os.rename(victim, victim + '.away')
        with open(victim + '.away', 'rb') as old, open(victim, 'wb') as new:
            new.write(old.read())
""")
    code, _, scratch = world["run"](scratch=world["tmp"] / "scratch-log", env={"STUB_RUSTC_HOOK": str(hook)})
    assert (scratch / "build-A.log.away").read_bytes() == (scratch / "build-A.log").read_bytes()
    err = capsys.readouterr().err
    assert code == 2 and says(err, "after it was reserved"), err
    assert not ran(world)


@pytest.mark.parametrize("target, needle", [("f__old.json.json", "is not the input the comparison wrote"),
                                            ("f__old.json.sparse.stdout", "after it was reserved")])
def test_a_file_replaced_right_after_the_tool_created_it_is_refused(world, monkeypatch, target, needle, capsys):
    # The identity recorded for an input or a run output is the one of the file the tool created: replace
    # the file at its name the moment it is created (the tool keeps writing through its descriptor) and the
    # replacement is never taken for it.
    original = bc.Area.new_file

    def new_file(self, name, mode=0o644):
        fd = original(self, name, mode)
        if name == target and self.path.name == "A":
            os.rename(self.path / name, self.path / (name + ".away"))
            (self.path / name).write_bytes(b"")
        return fd

    monkeypatch.setattr(bc.Area, "new_file", new_file)
    code, _, _ = world["run"]()
    err = capsys.readouterr().err
    assert code == 2 and says(err, needle), err
    assert not ran_a_solve(world)


@pytest.mark.parametrize("name", ["/elsewhere/table.md", "../escape.md", "sub/table.md", "results.json", "build", ".."])
def test_a_table_name_that_is_not_a_new_plain_name_in_the_area_is_refused(world, name, capsys):
    before = snapshot(world)
    code, _, scratch = world["run"](table=name)
    assert code == 2 and not built(world) and not ran(world) and snapshot(world) == before
    assert not (world["tmp"] / "escape.md").exists() and not Path("/elsewhere/table.md").exists()
    err = capsys.readouterr().err
    assert says(err, "is not a plain file name") or says(err, "is one of the tool's own evidence names"), err


# ------------------------------------------------------------------ the reservation primitives

def test_an_area_never_takes_a_name_it_did_not_make(tmp_path):
    area = bc.Area.reserve(str(tmp_path / "area"), "--scratch")
    try:
        (tmp_path / "area" / "made-elsewhere").mkdir()
        (tmp_path / "area" / "linked").symlink_to(tmp_path, target_is_directory=True)
        (tmp_path / "victim").write_text("keep")
        os.link(tmp_path / "victim", tmp_path / "area" / "hard")
        for name in ("made-elsewhere", "linked"):
            with pytest.raises(bc.SetupError):
                area.directory(name)
        for name in ("linked", "hard", "made-elsewhere"):
            with pytest.raises(bc.SetupError):
                area.write(name, b"overwritten")
        assert (tmp_path / "victim").read_text() == "keep" and not list(tmp_path.glob("overwritten"))
    finally:
        area.close()


def test_a_reserved_directory_swapped_for_a_link_is_refused_and_writes_stay_in_it(tmp_path):
    area = bc.Area.reserve(str(tmp_path / "area"), "--scratch")
    try:
        inner = area.directory("inner")
        os.rename(tmp_path / "area" / "inner", tmp_path / "area" / "inner.moved")
        (tmp_path / "area" / "inner").symlink_to(tmp_path, target_is_directory=True)
        with pytest.raises(bc.SetupError):
            area.verify()
        inner.write("evidence", b"kept")   # through the held descriptor, never through the link
        assert (tmp_path / "area" / "inner.moved" / "evidence").read_bytes() == b"kept"
        assert not (tmp_path / "evidence").exists()
    finally:
        area.close()


def test_a_directory_is_opened_without_following_a_link(tmp_path):
    root = tmp_path.resolve()
    (root / "real").mkdir()
    (root / "link").symlink_to(root / "real", target_is_directory=True)
    os.close(bc.open_directory(root / "real"))
    with pytest.raises(bc.SetupError):
        os.close(bc.open_directory(root / "link"))


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

def test_a_lint_level_cargo_itself_passes_is_accounted_for_and_changes_nothing(world):
    """Cargo on a newer toolchain passes `--force-warn=unused_crate_dependencies` to every unit. It is a lint
    level, which the attestation accounts for and does not compare, like the levels of a package's [lints]
    table. The comparison is the one `test_identical_revisions_compare_only_the_numerical_pairs` sees
    without the argument."""
    code, table, _ = world["run"](env={"STUB_RUSTC_ARGS": "--force-warn=unused_crate_dependencies"})
    assert code == 0, table
    assert "Compared with numerical work on both sides: 4 of 8 pairs** (4 identical, 0 different)" in table


@pytest.mark.parametrize("env, needle", [
    ({"STUB_RUSTC_ARGS": "-Zshare-generics"}, "unknown rustc argument '-Zshare-generics'"),
    ({"STUB_RUSTC_ARGS": "--target aarch64-apple-darwin"}, "unknown rustc argument '--target'"),
    # Only the `=` form cargo writes is a lint level here; the bare flag, whose value is the next word, refuses.
    ({"STUB_RUSTC_ARGS": "--force-warn unused_crate_dependencies"}, "unknown rustc argument '--force-warn'"),
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
    assert code == 2 and table == "" and says(err, needle), err
    assert nothing_ran(scratch)
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


def recording_wrapper(tmp_path, refuse_identity=False):
    """A run wrapper that logs each command it starts, marks the command as started under it, and (if
    asked) refuses the identity check as the slot wrapper refuses for a host condition (exit 98)."""
    wrapper = tmp_path / "wrapper.py"
    log = tmp_path / "wrapper.log"
    wrapper.write_text(f"#!{sys.executable}\nimport json, os, sys\n"
                       f"open({str(log)!r}, 'a').write(json.dumps(sys.argv[1:]) + '\\n')\n"
                       f"if {refuse_identity!r} and sys.argv[-1] == '--build-identity': sys.exit(98)\n"
                       "os.environ['UNDER_RUN_WRAPPER'] = '1'\nos.execvp(sys.argv[1], sys.argv[1:])\n")
    wrapper.chmod(0o755)
    return wrapper, log


def test_every_start_of_a_built_binary_or_its_rustc_goes_through_the_run_wrapper(world):
    wrapper, log = recording_wrapper(world["tmp"])
    outside = world["tmp"] / "outside.log"
    code, table, scratch = world["run"](wrapper=wrapper, env={"STUB_OUTSIDE_LOG": str(outside)})
    assert code == 0, table
    started = [json.loads(line) for line in log.read_text().splitlines()]
    results = json.loads((scratch / "results.json").read_text())
    for side in ("A", "B"):
        info = results["sides"][side]
        assert [info["binary"], "--build-identity"] in started
        program = info["build"]["attestation"]["rustc"]["program"]
        assert [program, "-vV"] in started and [program, "--print", "sysroot"] in started
    assert any(command[-2:] == ["run-batch", str(scratch / "plan.json")] for command in started)
    # Neither the driver (identity check and every run) nor the build's rustc started outside it.
    assert not outside.exists(), outside.read_text()
    assert "under " + str(wrapper) in table


def test_a_run_wrapper_that_cannot_be_used_is_refused_before_building(world, capsys):
    code, _, scratch = world["run"](wrapper=world["tmp"] / "no-such-wrapper")
    err = capsys.readouterr().err
    assert code == 2 and says(err, "--run-wrapper") and says(err, "cannot be used"), err
    assert not (scratch / "build-A.log").exists() and nothing_ran(scratch)


def test_a_run_wrapper_refusing_the_identity_check_stops_the_comparison(world, capsys):
    wrapper, _ = recording_wrapper(world["tmp"], refuse_identity=True)
    code, _, scratch = world["run"](wrapper=wrapper)
    err = capsys.readouterr().err
    assert code == 2 and says(err, "--build-identity (exit 98, a host condition)"), err
    assert nothing_ran(scratch)


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


def attest_captured(stdout=lambda t: t, stderr=lambda t: t, lock=lambda t: t, observed=lambda o: o,
                    dep_info=lambda d: d, calls=None):
    """Attests the captured build. The registry verifier stands in with the captured verification (the
    real one is tested below on a real archive); paths are the capture's placeholders, so the
    file-system resolution is the plain path (the link check is tested below on real links)."""
    seen = observed(json.loads((CAPTURED / "observed.json").read_text()))
    deps = dep_info(json.loads((CAPTURED / "dep_info.json").read_text()))
    parsed = bc.parse_build_output(stdout((CAPTURED / "stdout.log").read_text()),
                                   stderr((CAPTURED / "stderr.log").read_text()))
    rustc = {"program": "/SYSROOT/bin/rustc", "version": seen["rustc_version"], "sysroot": "/SYSROOT", "host": seen["host"]}
    manifest = bc.load_manifest(MANIFEST.read_text())
    record = calls if calls is not None else []

    def verify(home, index, name, version, checksum):
        record.append((home, index, name, version, checksum))
        root = f"{home}/registry/src/{index}/{name}-{version}"
        found = seen["registry"][root]
        return {"archive_sha256": found["archive_sha256"], "source_sha256": found["source_sha256"], "root": root,
                "files": set(found["files"])}

    return bc.attest_build(parsed, tree="/WT", target="/TARGET", lock_text=lock((CAPTURED / "Cargo.lock").read_text()),
                           settings=manifest["build_settings"], closure=manifest["build_closure"], rustc=rustc,
                           committed=seen["committed"], list_generated=lambda out_dir: seen["out_dirs"][out_dir],
                           verify_registry=verify, read_dep_info=lambda path: deps[path], find_link=lambda path: None)


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
    # Each registry package is verified with the source, name, version and checksum Cargo.lock gives, and
    # its archive and extracted-source digests are bound into this side's attestation.
    assert ("/CARGO_HOME", "index.crates.io-1949cf8c6b5b557f", "serde_json", "1.0.149",
            "83fc039473c5595ace860d8c4fafa220ff474b3fc6bfdb4293327f1a37e94d86") in calls
    seen = json.loads((CAPTURED / "observed.json").read_text())
    serde_json = seen["registry"]["/CARGO_HOME/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.149"]
    assert attestation["registry"][f"{REGISTRY}#serde_json@1.0.149"] == {
        "archive_sha256": serde_json["archive_sha256"], "source_sha256": serde_json["source_sha256"]}
    # The source closure: every file the units' dep-info names, the path manifests and the lock.
    assert attestation["source_closure_files"] == 540


def test_each_side_verifies_its_registry_packages_itself():
    # Two attestations (two sides) each verify every registry package: nothing is reused between them.
    calls_a, calls_b = [], []
    attest_captured(calls=calls_a)
    attest_captured(calls=calls_b)
    assert len(calls_a) == len(calls_b) == 22 and calls_a == calls_b


BIN_DEPS = "/TARGET/release/deps/release_timing-5593dae461ac0171.d"
ITOA_DEPS = next(path for path in json.loads((CAPTURED / "dep_info.json").read_text()) if "/deps/itoa-" in path)


def add_dep(path, extra):
    def change(deps):
        deps[path] = edit(deps[path], ": src/main.rs\n", f": src/main.rs {extra}\n", count=2)
        return deps
    return change


ITOA = f"{REGISTRY}#itoa@1.0.18"
PROC_MACRO2_RUN = "proc-macro2-5d24402fc79aac90/build-script-build`"


def test_a_build_script_run_told_that_no_path_is_trimmed_attests_as_before():
    # Cargo 1.100 and later give every build-script run the trim-paths setting, as `none` and an
    # empty remap when the profile trims nothing; earlier cargo gives neither. With both added to
    # every run of the captured build, the attestation is the one the capture gives without them.
    def told(text):
        lines = text.split("\n")
        runs = [i for i, line in enumerate(lines) if line.endswith("/build-script-build`")]
        assert len(runs) == 10, len(runs)
        for i in runs:
            lines[i] = edit(lines[i], "CARGO_ENCODED_RUSTFLAGS=''",
                            "CARGO_ENCODED_RUSTFLAGS='' CARGO_TRIM_PATHS_REMAP='' CARGO_TRIM_PATHS_SCOPE=none")
        return "\n".join(lines)
    assert attest_captured(stderr=told) == attest_captured()


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
    # Cargo 1.100 and later tell a build script the profile's trim-paths setting; only "trims nothing" attests.
    "a build script run under a trim-paths scope": (dict(stderr=lambda t: edit_line(
        t, PROC_MACRO2_RUN, "CARGO_ENCODED_RUSTFLAGS=''",
        "CARGO_ENCODED_RUSTFLAGS='' CARGO_TRIM_PATHS_REMAP='' CARGO_TRIM_PATHS_SCOPE=object")),
        "ran with CARGO_TRIM_PATHS_SCOPE='object', not 'none'"),
    "a build script run given a path remap": (dict(stderr=lambda t: edit_line(
        t, PROC_MACRO2_RUN, "CARGO_ENCODED_RUSTFLAGS=''",
        "CARGO_ENCODED_RUSTFLAGS='' CARGO_TRIM_PATHS_REMAP=/WT=. CARGO_TRIM_PATHS_SCOPE=none")),
        "ran with CARGO_TRIM_PATHS_REMAP='/WT=.', not ''"),
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
    # A linked dependency at exactly the build-time settings (profile and flags) is still a runtime unit.
    "a linked dependency at the build-time settings": (dict(
        stdout=lambda t: edit_message(t, ITOA, ("lib",), lambda m: m["profile"].update(opt_level="0")),
        stderr=lambda t: edit_line(t, "--crate-name itoa ", "-C opt-level=3 -C embed-bitcode=no",
                                   "-C embed-bitcode=no -C debug-assertions=off")),
        "itoa@1.0.18 (itoa, runtime) was built with profile"),
    # The source closure (rustc's dep-info).
    "a source outside the closure": (dict(dep_info=add_dep(BIN_DEPS, "/HOME/elsewhere/extra.rs")),
                                     "is neither committed, nor a verified registry file"),
    "an uncommitted worktree file": (dict(dep_info=add_dep(BIN_DEPS, "/WT/core/solver/release_timing/src/untracked.rs")),
                                     "is not a committed file of the side's worktree"),
    "a committed link in the closure": (dict(observed=lambda o: (o["committed"].update(
        {"core/solver/release_timing/src/main.rs": "120000"}), o)[1]), "is a committed link"),
    "a registry file outside its verified archive": (dict(dep_info=add_dep(
        BIN_DEPS, "/CARGO_HOME/registry/src/index.crates.io-1949cf8c6b5b557f/itoa-1.0.18/src/planted.rs")),
        "is neither committed, nor a verified registry file"),
    "a generated file the manifest does not allow": (dict(dep_info=add_dep(
        BIN_DEPS, "/TARGET/release/build/zmij-effecf68cb707a5f/out/gen.rs")), "nor an allowed generated file"),
    "a unit without dep-info": (dict(dep_info=lambda d: (d.pop(BIN_DEPS), d)[1]), "left no dep-info"),
    "an unknown dep-info line": (dict(dep_info=lambda d: dict(d, **{BIN_DEPS: d[BIN_DEPS] + "# checksum:abc\n"})),
                                 "an unknown dep-info line"),
    "a relative source for a unit compiled from an absolute path": (dict(dep_info=lambda d: dict(d, **{
        ITOA_DEPS: d[ITOA_DEPS].replace("\n\n", " src/relative.rs\n\n", 1)})), "names a relative file"),
}


@pytest.mark.parametrize("case", sorted(CAPTURED_REFUSALS))
def test_a_departure_in_the_captured_build_is_refused(case):
    edits, needle = CAPTURED_REFUSALS[case]
    refused_with(needle, attest_captured, **edits)


def test_a_wrapper_or_second_compiler_is_not_the_build_compiler():
    stdout = (CAPTURED / "stdout.log").read_text()
    stderr = (CAPTURED / "stderr.log").read_text()
    assert bc.compiler_program(bc.parse_build_output(stdout, stderr)) == "/SYSROOT/bin/rustc"
    for old, new, needle in (("/SYSROOT/bin/rustc --crate-name itoa ", "/HOME/bin/sccache /SYSROOT/bin/rustc --crate-name itoa ",
                              "compiled by 2 programs"),
                             ("/SYSROOT/bin/rustc", "/HOME/bin/wrap", "not rustc named by an absolute path")):
        changed = stderr.replace(old, new)
        assert changed != stderr
        refused_with(needle, lambda: bc.compiler_program(bc.parse_build_output(stdout, changed)))


def test_the_compiler_must_be_its_own_sysroots_rustc(tmp_path):
    def fake(sysroot_printed):
        rustc = tmp_path / "bin" / "rustc"
        rustc.parent.mkdir(parents=True, exist_ok=True)
        rustc.write_text(f"#!{sys.executable}\nimport sys\nif sys.argv[1:] == ['-vV']: print('rustc 9.9.9\\nhost: h')\n"
                         f"else: print({sysroot_printed!r})\n")
        rustc.chmod(0o755)
        return str(rustc)
    assert bc.rustc_identity(fake(str(tmp_path)))["host"] == "h"
    refused_with("is not the rustc of its own sysroot", bc.rustc_identity, fake(str(tmp_path / "elsewhere")))


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
    refused_with(needle, bc.verify_registry_package, str(home), index, "demo", "1.0.0", checksum)


def test_a_registry_source_edited_between_the_two_sides_verifications_is_refused(tmp_path):
    # Side A's verification accepts the package; its extracted source is then edited before side B's.
    home, index, checksum, root = registry_home(tmp_path)
    side_a = bc.verify_registry_package(str(home), index, "demo", "1.0.0", checksum)
    (root / "src" / "lib.rs").write_bytes(b"pub fn f() { changed_after_side_a() }\n")
    refused_with("is not the locked archive", bc.verify_registry_package, str(home), index, "demo", "1.0.0", checksum)
    assert side_a["archive_sha256"] == checksum and side_a["files"] == {"Cargo.toml", "src/lib.rs"}


# ------------------------------------------------------------------ the manifest's entry IDs

def test_entries_whose_derived_ids_collide_are_refused():
    colliding = [{"path": "f/a__b.json", "source_version": "1", "expect": NUMERICAL},
                 {"path": "f/a/b.json", "source_version": "1", "expect": NUMERICAL}]
    with pytest.raises(bc.ManifestError) as refused:
        bc.load_manifest(manifest_text(colliding, roots=[{"pattern": "f/**/*.json", "why": "w"}]))
    assert says(str(refused.value), "derive the same entry ID")


# ------------------------------------------------------------------ the source closure, on real links

@pytest.fixture()
def closure_tree(tmp_path):
    """A tree with committed files and committed links: `core/a/link` to a directory outside it,
    `core/a/self` to the sibling directory `sub` (so `self/..` lands where `a/..` would), and `top` at the
    first component under the tree, to `core`; and files outside it."""
    tree, outside = (tmp_path / "tree").resolve(), (tmp_path / "outside").resolve()
    (tree / "core" / "a" / "sub" / "sub2").mkdir(parents=True)
    (outside / "deep").mkdir(parents=True)
    (tree / "core" / "a" / "x.rs").write_text("committed\n")
    (outside / "x.rs").write_text("outside\n")
    (tree / "core" / "a" / "link").symlink_to(outside / "deep", target_is_directory=True)
    (tree / "core" / "a" / "self").symlink_to("sub", target_is_directory=True)
    (tree / "top").symlink_to("core", target_is_directory=True)
    committed = {"core/a/x.rs": "100644", "core/a/link": "120000", "core/a/self": "120000", "top": "120000"}
    return tree, outside, committed


def closure(tree, committed, paths, generated=()):
    bc.check_source_closure([str(p) for p in paths], tree=str(tree), committed=committed, registry={},
                            generated={str(g) for g in generated})


def test_committed_sources_reached_without_a_link_are_accepted(closure_tree):
    tree, _, committed = closure_tree
    # `..` through real directories is not a link: rustc reports such paths for include_str! sources.
    closure(tree, committed, [tree / "core" / "a" / "x.rs", tree / "core" / "a" / ".." / "a" / "x.rs",
                              tree / "core" / "a" / "sub" / "sub2" / ".." / ".." / "x.rs"])


@pytest.mark.parametrize("case", ["through a link and ..", "a link canceled by .., same endpoint",
                                  "a link canceled two levels down", "a link at the first component",
                                  "a committed link", "outside", "uncommitted"])
def test_a_source_the_closure_does_not_hold_is_refused(closure_tree, case):
    tree, outside, committed = closure_tree
    # "through a link and ..": written plainly it names committed core/a/x.rs, but the file system goes
    # through the committed link and reads outside/x.rs (the endpoints differ).
    # The next three resolve to exactly the committed core/a/x.rs that their plain form names (equal
    # endpoints): a link was traversed all the same, canceled by `..` at different depths.
    link = "a link anywhere on a source's path refuses"
    path, needle = {"through a link and ..": (tree / "core" / "a" / "link" / ".." / "x.rs", link),
                    "a link canceled by .., same endpoint": (tree / "core" / "a" / "self" / ".." / "x.rs", link),
                    "a link canceled two levels down": (tree / "core" / "a" / "self" / "sub2" / ".." / ".." / "x.rs", link),
                    "a link at the first component": (tree / "top" / ".." / "core" / "a" / "x.rs", link),
                    "a committed link": (tree / "core" / "a" / "link", link),
                    "outside": (outside / "x.rs", "is neither committed"),
                    "uncommitted": (tree / "core" / "a" / "y.rs", "is not a committed file")}[case]
    if case == "uncommitted":
        path.write_text("untracked\n")
    refused_with(needle, closure, tree, committed, [path])


def test_a_generated_output_that_is_a_link_is_refused(tmp_path):
    out_dir = tmp_path / "out"
    out_dir.mkdir()
    (out_dir / "private.rs").symlink_to(tmp_path / "elsewhere.rs")
    (tmp_path / "elsewhere.rs").write_text("pub fn f() {}\n")
    refused_with("is a link or not a regular file", bc.generated_files, str(out_dir))


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
    assert code == 2 and says(out.err, f"side {refused} (") and says(out.err, "runtime) was built with profile"), out.err
    assert nothing_ran(scratch)


@RUNS_CARGO
def test_a_cargo_home_config_adding_rustflags_is_refused(tmp_path, capsys):
    world = cargo_world(tmp_path)
    write(world["home"], "config.toml", '[build]\nrustflags = ["-C", "target-cpu=native"]\n')
    code, out, scratch = run_real(world, capsys)
    assert code == 2 and says(out.err, "side A (") and says(out.err, "'target-cpu': 'native'"), out.err
    assert nothing_ran(scratch)


@RUNS_CARGO
def test_a_dependency_specific_profile_override_is_refused(tmp_path, capsys):
    world = cargo_world(tmp_path)
    write(world["home"], "config.toml", "[profile.release.package.dep]\nopt-level = 0\n")
    code, out, scratch = run_real(world, capsys)
    assert code == 2 and says(out.err, "side A (") and says(out.err, "/core/solver/dep#0.1.0 (dep, runtime) was built with profile"), out.err
    assert nothing_ran(scratch)


@RUNS_CARGO
def test_a_rustc_wrapper_is_refused(tmp_path, capsys):
    world = cargo_world(tmp_path)
    wrapper = tmp_path / "wrap.sh"
    wrapper.write_text('#!/bin/sh\nexec "$@"\n')
    wrapper.chmod(0o755)
    write(world["home"], "config.toml", f'[build]\nrustc-wrapper = "{wrapper}"\n')
    code, out, scratch = run_real(world, capsys)
    assert code == 2 and says(out.err, "side A (") and says(out.err, f"the compiler {wrapper} is not rustc"), out.err
    assert nothing_ran(scratch)


@RUNS_CARGO
@pytest.mark.parametrize("side", ["A", "B"])
def test_a_build_override_that_changes_generated_source_is_refused(tmp_path, capsys, side):
    # The build script is compiled with debug assertions on, so the source it generates for the
    # dependency differs, while every runtime unit's own flags stay exactly the release settings.
    world = cargo_world(tmp_path, {side: {".cargo/config.toml": "[profile.release.build-override]\ndebug-assertions = true\n"}})
    code, out, scratch = run_real(world, capsys)
    assert code == 2 and says(out.err, f"side {side} (") and says(out.err, "build_script_build, host) was built with profile"), out.err
    assert nothing_ran(scratch)
    assert "= true;" in generated_source(world, side)


@RUNS_CARGO
@pytest.mark.parametrize("side", ["A", "B"])
def test_a_committed_link_in_the_source_closure_is_refused(tmp_path, capsys, side):
    # The reading's example: the dependency declares `mod inner;` and the committed src/inner.rs is a link
    # to code outside the repository. Cargo's crate root still names src/lib.rs.
    outside = tmp_path / "outside-inner.rs"
    outside.write_text("pub fn inner() {}\n")
    world = cargo_world(tmp_path, {side: {"core/solver/dep/src/lib.rs": SYNTHETIC["core/solver/dep/src/lib.rs"] + "mod inner;\n"}})
    repo = world["repo"]
    git(repo, "checkout", "-q", "--detach", world[side])
    (repo / "core/solver/dep/src/inner.rs").symlink_to(outside)
    git(repo, "add", "-A")
    git(repo, "commit", "-q", "-m", "link")
    world[side] = git(repo, "rev-parse", "HEAD")
    code, out, scratch = run_real(world, capsys)
    assert code == 2 and says(out.err, f"side {side} (") and says(out.err, "inner.rs, which is a link"), out.err
    assert nothing_ran(scratch)
