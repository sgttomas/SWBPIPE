#!/usr/bin/env python3
"""Before/after byte comparison of maintained solve requests between two revisions.

Standard library only. `README.md` beside this file states the use, the inputs and outputs, the exit
codes, the lift and the threat model this tool is reviewed against.

For each of revisions A and B it makes a clean detached worktree of exactly that commit, builds the
release timing driver (`core/solver/release_timing`, the desktop solve worker's engine stages at the
desktop's build) there with `cargo build --release --locked`, checks the binary's embedded build
identity against the commit, and runs every manifest fixture through the driver in both solver modes.
A pair is COMPARED only where both sides reached numerical work; every other pair is reported by its
class, and a pair whose observed class differs from the manifest fails.
"""
from __future__ import annotations

import argparse
import copy
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import shlex
import shutil
import subprocess
import sys
import time

TOOL_DIR = Path(__file__).resolve().parent
DEFAULT_MANIFEST = TOOL_DIR / "manifest.json"
MANIFEST_SCHEMA = "swbpipe-byte-compare-manifest/v1"
RESULTS_SCHEMA = "swbpipe-byte-compare-results/v1"

# The runner: the release timing driver (DEL-04-05). Its solve invocation reads a model document or a
# `{"model", "materials"}` request, calls `run_linear_static_preview_value_with_mode` on
# `{"model": model, "materials": []}` (the desktop's `native_request`) and `--write` keeps
# `serde_json::to_string(&envelope)`, the worker's exact result bytes.
DRIVER_CRATE = "core/solver/release_timing"
DRIVER_BIN = "release_timing"
PROFILE = "release"
DRIVER_SCHEMA = "release-timing-v2"
DRIVER_ENTRY_ERROR_EXIT = 3
# The driver's `--mode` values, in the order run.
MODES = ("sparse", "dense")

# Where the revision under test states its model document version (the lift's target).
VERSION_SOURCE = "core/product_physics/src/model_format.rs"
VERSION_CONST = re.compile(r'^pub const MODEL_SCHEMA_VERSION: &str = "([^"\n]+)";$', re.M)

# Observed and expected classes. A class outside DISPOSITIONS (crash, empty_result, ...) is never
# expected, so it always fails.
NUMERICAL = "numerical_work"
ENTRY_REFUSAL = "entry_refusal"
MODEL_INCOMPLETE = "model_incomplete"
OUTSIDE_RUNNER_INPUT = "outside_runner_input"
DISPOSITIONS = (NUMERICAL, ENTRY_REFUSAL, MODEL_INCOMPLETE, OUTSIDE_RUNNER_INPUT)
CRASH = "crash"
LIFT_REFUSED = "lift_refused"
MISSING = "missing"
# Blocking codes by which the product refuses the document's format rather than solving it: an
# envelope carrying one is an entry refusal, never a model-incomplete result.
DOCUMENT_REFUSAL_CODES = frozenset({
    "MODEL_SCHEMA_VERSION_RETIRED",
    "PREVIEW_SCHEMA_VERSION_UNSUPPORTED",
    "PRESSURE_MODEL_REAUTHOR_REQUIRED",
    "PRESSURE_CONTRACT_UNSUPPORTED",
})
SOLVED = "MECHANICS_SOLVED"
INCOMPLETE = "MODEL_INCOMPLETE"

# Environment that could change what cargo builds (flags, profile, target directory) is removed
# from the build; so is every GIT_* variable, from every git call and build.
CLEARED_ENV = frozenset({
    "RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "RUSTDOCFLAGS", "CARGO_TARGET_DIR", "CARGO_INCREMENTAL",
    "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER",
})
CLEARED_PREFIXES = ("GIT_", "CARGO_PROFILE_", "CARGO_BUILD_", "CARGO_TARGET_")

EXIT_OK, EXIT_FAILED, EXIT_SETUP = 0, 1, 2


class SetupError(Exception):
    """A condition that prevents a sound comparison: exit 2, nothing compared."""


class ManifestError(SetupError):
    pass


class LiftError(Exception):
    """A fixture the lift cannot take; the pair is reported LIFT_REFUSED and fails."""


# ------------------------------------------------------------------ helpers

def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with open(path, "rb") as handle:
        for block in iter(lambda: handle.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def clean_env(base: dict[str, str] | None = None) -> dict[str, str]:
    env = dict(os.environ if base is None else base)
    return {k: v for k, v in env.items() if k not in CLEARED_ENV and not k.startswith(CLEARED_PREFIXES)}


def git(cwd: Path, *args: str, check: bool = True) -> subprocess.CompletedProcess:
    command = ["git", "-c", "core.hooksPath=/dev/null", "-c", "core.fsmonitor=false", "-C", str(cwd), *args]
    result = subprocess.run(command, capture_output=True, text=True, env=clean_env())
    if check and result.returncode != 0:
        raise SetupError(f"git {' '.join(args)} failed in {cwd}: {result.stderr.strip()}")
    return result


def inside_git_work_tree(path: Path) -> bool:
    """Whether `path` (or, if absent, its nearest existing ancestor) lies inside any git work tree."""
    probe = path
    while not probe.exists():
        probe = probe.parent
    result = git(probe if probe.is_dir() else probe.parent, "rev-parse", "--is-inside-work-tree", check=False)
    return result.returncode == 0 and result.stdout.strip() == "true"


def ensure_outside_work_trees(path: Path, what: str) -> None:
    if not path.is_absolute():
        raise SetupError(f"{what} must be an absolute path: {path}")
    if inside_git_work_tree(path):
        raise SetupError(f"{what} {path} is inside a git work tree; it must be outside every work tree")


def ensure_new_or_empty(path: Path, what: str) -> None:
    if path.exists() and (not path.is_dir() or any(path.iterdir())):
        raise SetupError(f"{what} {path} exists and is not an empty directory")
    path.mkdir(parents=True, exist_ok=True)


def relative_posix(text: str, what: str) -> str:
    if not isinstance(text, str) or not text:
        raise ManifestError(f"{what} must be a non-empty string")
    pure = PurePosixPath(text)
    if pure.is_absolute() or ".." in pure.parts or "\\" in text:
        raise ManifestError(f"{what} {text!r} must be a relative path inside the repository")
    return pure.as_posix()


def entry_id(path: str) -> str:
    return path.replace("/", "__")


# ------------------------------------------------------------------ manifest

def _closed(record: dict, allowed: set[str], required: set[str], what: str) -> None:
    if not isinstance(record, dict):
        raise ManifestError(f"{what} must be an object")
    unknown = sorted(set(record) - allowed)
    if unknown:
        raise ManifestError(f"{what} has unknown keys {unknown}")
    missing = sorted(required - set(record))
    if missing:
        raise ManifestError(f"{what} lacks {missing}")


def _expectation(record: dict, what: str) -> dict:
    _closed(record, {"disposition", "numerical"}, {"disposition", "numerical"}, what)
    disposition, numerical = record["disposition"], record["numerical"]
    if disposition not in DISPOSITIONS:
        raise ManifestError(f"{what}: disposition {disposition!r} is not one of {list(DISPOSITIONS)}")
    if not isinstance(numerical, bool) or numerical != (disposition == NUMERICAL):
        raise ManifestError(f"{what}: numerical must be {disposition == NUMERICAL} for disposition {disposition}")
    return {"disposition": disposition, "numerical": numerical}


def load_manifest(text: str) -> dict:
    """The validated manifest: closed records, unique relative paths, consistent expectations."""
    try:
        raw = json.loads(text)
    except json.JSONDecodeError as error:
        raise ManifestError(f"the manifest is not JSON: {error}") from error
    _closed(raw, {"schema", "description", "roots", "entries"}, {"schema", "roots", "entries"}, "the manifest")
    if raw["schema"] != MANIFEST_SCHEMA:
        raise ManifestError(f"manifest schema {raw['schema']!r} is not {MANIFEST_SCHEMA!r}")
    if not isinstance(raw["roots"], list) or not raw["roots"]:
        raise ManifestError("the manifest must state at least one root")
    roots = []
    for index, root in enumerate(raw["roots"]):
        _closed(root, {"pattern", "why"}, {"pattern", "why"}, f"root {index}")
        roots.append({"pattern": relative_posix(root["pattern"], f"root {index} pattern"), "why": root["why"]})
    if not isinstance(raw["entries"], list) or not raw["entries"]:
        raise ManifestError("the manifest must name at least one entry")
    entries, seen = [], set()
    for index, entry in enumerate(raw["entries"]):
        what = f"entry {index}"
        _closed(entry, {"path", "source_version", "expect", "why"}, {"path", "source_version", "expect"}, what)
        path = relative_posix(entry["path"], f"{what} path")
        if path in seen:
            raise ManifestError(f"{what}: {path} is named twice")
        seen.add(path)
        if not isinstance(entry["source_version"], str) or not entry["source_version"]:
            raise ManifestError(f"{what}: source_version must be a non-empty string")
        expect = entry["expect"]
        if isinstance(expect, dict) and set(expect) == {"A", "B"}:
            sides = {side: _expectation(expect[side], f"{what} ({path}) expect.{side}") for side in ("A", "B")}
        else:
            both = _expectation(expect, f"{what} ({path}) expect")
            sides = {"A": dict(both), "B": dict(both)}
        entries.append({"path": path, "source_version": entry["source_version"], "expect": sides,
                        "why": entry.get("why", "")})
    return {"roots": roots, "entries": entries}


def coverage(tree: Path, manifest: dict) -> dict:
    """Files under the stated roots the manifest does not name, and manifest paths absent from `tree`."""
    named = {entry["path"] for entry in manifest["entries"]}
    found = set()
    for root in manifest["roots"]:
        found.update(path.relative_to(tree).as_posix() for path in tree.glob(root["pattern"]) if path.is_file())
    return {
        "unnamed": sorted(found - named),
        "missing": sorted(path for path in named if not (tree / path).is_file()),
    }


# ------------------------------------------------------------------ the lift

def target_version(tree: Path) -> str:
    """MODEL_SCHEMA_VERSION as the revision under test states it."""
    source = tree / VERSION_SOURCE
    if not source.is_file():
        raise SetupError(f"{VERSION_SOURCE} is absent at {tree}")
    found = VERSION_CONST.findall(source.read_text(encoding="utf-8"))
    if len(found) != 1:
        raise SetupError(f"{VERSION_SOURCE} at {tree} states MODEL_SCHEMA_VERSION {len(found)} times, not once")
    return found[0]


def lift(document: bytes, source_version: str, target: str) -> dict:
    """The reviewed lift, in memory: a bare model document is wrapped as {"model": doc}; the model's
    schema_version must equal `source_version` and is then set to `target`. Nothing else changes.

    Returns {"request", "lifted", "shape", "outside"}; `outside` names why the runner cannot take
    the request unchanged (top-level keys beyond model and materials, or materials it would drop)."""
    try:
        doc = json.loads(document)
    except (json.JSONDecodeError, UnicodeDecodeError) as error:
        raise LiftError(f"not JSON: {error}") from error
    if not isinstance(doc, dict):
        raise LiftError("not a JSON object")
    if isinstance(doc.get("model"), dict):
        request, shape = copy.deepcopy(doc), "request"
    else:
        request, shape = {"model": copy.deepcopy(doc)}, "model_document"
    original = copy.deepcopy(request)
    actual = request["model"].get("schema_version")
    if actual != source_version:
        raise LiftError(f"model.schema_version is {actual!r}; the manifest names {source_version!r}")
    lifted = actual != target
    if lifted:
        request["model"]["schema_version"] = target
    # Nothing but model.schema_version differs from the (wrapped) document.
    check = copy.deepcopy(request)
    check["model"]["schema_version"] = original["model"]["schema_version"]
    if check != original:
        raise LiftError("the lift changed more than model.schema_version")
    extra = sorted(set(request) - {"model", "materials"})
    materials = request.get("materials", [])
    outside = None
    if extra:
        outside = f"request keys {extra} beyond model and materials, which the runner does not pass to the entry"
    elif materials != []:
        outside = "materials the runner does not pass to the entry"
    return {"request": request, "lifted": lifted, "shape": shape, "outside": outside}


def serialize_request(request: dict) -> bytes:
    # Python's float repr is the shortest text that round-trips the binary64 value, and the driver's
    # serde_json parses with float_roundtrip, so every number reaches the entry unchanged.
    return json.dumps(request, ensure_ascii=False, allow_nan=False).encode("utf-8")


# ------------------------------------------------------------------ classification

def classify_run(status: dict, stdout: str, envelope: bytes | None) -> tuple[str, str, dict]:
    """(class, detail, facts) of one driver run, from its exit status, stdout and written envelope."""
    if status.get("timed_out"):
        return CRASH, f"timed out after {status.get('seconds', 0):.0f} s", {}
    code = status.get("exit")
    lines = [line for line in stdout.splitlines() if line.strip()]
    report = None
    if len(lines) == 1:
        try:
            report = json.loads(lines[0])
        except json.JSONDecodeError:
            report = None
    if not isinstance(report, dict) or report.get("schema") != DRIVER_SCHEMA or report.get("invocation") != "solve":
        return CRASH, f"exit {code}; stdout is not one {DRIVER_SCHEMA} solve line", {}
    if code == DRIVER_ENTRY_ERROR_EXIT and report.get("status") == "entry_error":
        error = report.get("entry_error") or {}
        message = str(error.get("message", ""))
        return ENTRY_REFUSAL, f"entry error at {error.get('stage')}: {message[:160]}", {}
    if code != 0:
        return CRASH, f"exit {code}, status {report.get('status')!r}", {}
    if report.get("written") is not True or envelope is None or not envelope:
        return CRASH, "exit 0 but no envelope was written", {}
    if sha256_bytes(envelope) != report.get("output_sha256") or len(envelope) != report.get("output_bytes"):
        return CRASH, "the written envelope is not the bytes the driver reported", {}
    if (report.get("document") or {}).get("request_materials_ignored", 0):
        return CRASH, "the driver ignored request materials", {}
    try:
        doc = json.loads(envelope)
    except (json.JSONDecodeError, UnicodeDecodeError):
        return CRASH, "the envelope is not JSON", {}
    if not isinstance(doc, dict):
        return CRASH, "the envelope is not an object", {}
    mechanics = (doc.get("status") or {}).get("mechanics")
    results = doc.get("results")
    diagnostics = doc.get("diagnostics") if isinstance(doc.get("diagnostics"), list) else []
    blocking = sorted({d.get("code") for d in diagnostics if isinstance(d, dict) and d.get("severity") == "blocking"})
    rows = len(results) if isinstance(results, list) else None
    quality = (doc.get("numerical_quality") or {}).get("status")
    facts = {"mechanics": mechanics, "rows": rows, "blocking": blocking, "quality": quality}
    refused = sorted(DOCUMENT_REFUSAL_CODES.intersection(blocking))
    if refused:
        return ENTRY_REFUSAL, f"document refused: {', '.join(refused)}", facts
    if mechanics == SOLVED:
        if not rows:
            return "empty_result", "MECHANICS_SOLVED with no result rows", facts
        if blocking:
            return "solved_with_blocking", f"MECHANICS_SOLVED with blocking {blocking}", facts
        return NUMERICAL, f"{rows} rows, {quality}", facts
    if mechanics == INCOMPLETE:
        return MODEL_INCOMPLETE, f"blocking {', '.join(blocking) or 'none'}", facts
    return f"status:{mechanics}", f"mechanics status {mechanics!r}", facts


def judge_pair(a: dict, b: dict) -> dict:
    """The verdict of one fixture in one mode. `a`/`b`: {"cls", "expected", "input_sha256",
    "envelope" (bytes or None)}. COMPARED only where both sides reached numerical work as expected."""
    failures = []
    for side, run in (("A", a), ("B", b)):
        if run["cls"] != run["expected"]:
            failures.append(f"{side} {run['cls']} where {run['expected']} is expected")
    if a.get("input_sha256") and b.get("input_sha256") and a["input_sha256"] != b["input_sha256"]:
        failures.append("the fixture's bytes differ between A and B")
    same = a.get("envelope") is not None and a.get("envelope") == b.get("envelope")
    if failures:
        return {"verdict": "FAIL", "compared": False, "failed": True, "why": "; ".join(failures), "same_bytes": same}
    if a["cls"] == NUMERICAL and b["cls"] == NUMERICAL:
        if same:
            return {"verdict": "IDENTICAL", "compared": True, "failed": False, "why": "", "same_bytes": True}
        return {"verdict": "DIFFERENT", "compared": True, "failed": True, "why": "result bytes differ", "same_bytes": False}
    return {"verdict": f"not compared: {a['cls']}", "compared": False, "failed": False, "why": "", "same_bytes": same}


# ------------------------------------------------------------------ build and identity

def resolve_commit(repo: Path, rev: str) -> str:
    sha = git(repo, "rev-parse", "--verify", "--quiet", f"{rev}^{{commit}}", check=False).stdout.strip()
    if not re.fullmatch(r"[0-9a-f]{40}", sha):
        raise SetupError(f"{rev!r} names no commit in {repo}")
    return sha


def worktree_status(tree: Path) -> str:
    return git(tree, "status", "--porcelain", "--untracked-files=all", "--ignore-submodules=none").stdout


def make_worktree(repo: Path, path: Path, sha: str) -> None:
    if path.exists():
        raise SetupError(f"worktree path {path} already exists")
    git(repo, "worktree", "add", "--detach", str(path), sha)
    head = git(path, "rev-parse", "HEAD").stdout.strip()
    if head != sha:
        raise SetupError(f"worktree {path} is at {head}, not {sha}")
    status = worktree_status(path)
    if status:
        raise SetupError(f"worktree {path} is not clean before the build:\n{status}")


def remove_worktree(repo: Path, path: Path) -> str | None:
    """`git worktree remove` (never forced); returns an error message, or None."""
    if not path.exists():
        return None
    result = git(repo, "worktree", "remove", str(path), check=False)
    return None if result.returncode == 0 else result.stderr.strip()


def build_driver(tree: Path, sha: str, target: Path, cargo: list[str], jobs: int, log: Path) -> dict:
    """Builds the driver from `tree` and checks the binary's identity. Returns the build record."""
    binary = target / PROFILE / DRIVER_BIN
    # A binary left by an earlier build is removed first: the binary used is one this build linked.
    if binary.exists():
        binary.unlink()
    command = [*cargo, "build", "--release", "--locked", "--manifest-path", str(tree / DRIVER_CRATE / "Cargo.toml"),
               "--target-dir", str(target), "--bin", DRIVER_BIN, "-j", str(jobs)]
    started = time.time()
    with open(log, "wb") as handle:
        result = subprocess.run(command, cwd=tree / DRIVER_CRATE, stdout=handle, stderr=subprocess.STDOUT,
                                env=clean_env())
    if result.returncode != 0:
        raise SetupError(f"the build of {sha} exited {result.returncode}; see {log}")
    if not binary.is_file() or binary.stat().st_mtime < started - 1:
        raise SetupError(f"the build of {sha} did not link {binary}")
    status = worktree_status(tree)
    if status:
        raise SetupError(f"worktree {tree} is not clean after the build:\n{status}")
    return {"command": command, "binary": str(binary), "seconds": round(time.time() - started, 1)}


def check_identity(binary: Path, sha: str, tree_sha: str) -> dict:
    """The driver's embedded identity (`--build-identity`) must name `sha`, its tree, a clean source
    and the release profile."""
    result = subprocess.run([str(binary), "--build-identity"], capture_output=True, text=True, env=clean_env())
    try:
        identity = json.loads(result.stdout)
    except json.JSONDecodeError:
        identity = None
    if result.returncode != 0 or not isinstance(identity, dict):
        raise SetupError(f"{binary} --build-identity gave no identity (exit {result.returncode})")
    problems = []
    if identity.get("source_commit") != sha:
        problems.append(f"source_commit {identity.get('source_commit')!r} is not {sha}")
    if identity.get("source_tree") != tree_sha:
        problems.append(f"source_tree {identity.get('source_tree')!r} is not {tree_sha}")
    if identity.get("source_dirty") is not False:
        problems.append(f"source_dirty is {identity.get('source_dirty')!r}, not false")
    if identity.get("build_profile") != PROFILE:
        problems.append(f"build_profile {identity.get('build_profile')!r} is not {PROFILE}")
    if problems:
        raise SetupError(f"{binary} was not built as stated: " + "; ".join(problems))
    return identity


# ------------------------------------------------------------------ runs

def run_batch(plan: dict) -> int:
    """Executes a plan's runs one at a time, keeping stdout, stderr, exit status and envelope."""
    timeout = plan["timeout"]
    for run in plan["runs"]:
        prefix = Path(run["prefix"])
        envelope = Path(str(prefix) + ".envelope.json")
        if envelope.exists():
            envelope.unlink()
        started = time.monotonic()
        with open(str(prefix) + ".stdout", "wb") as out, open(str(prefix) + ".stderr", "wb") as err:
            process = subprocess.Popen([run["binary"], run["input"], "--mode", run["mode"], "--write", str(envelope)],
                                       stdout=out, stderr=err, cwd=plan["cwd"], env=clean_env())
            try:
                code, timed_out = process.wait(timeout=timeout), False
            except subprocess.TimeoutExpired:
                process.kill()
                code, timed_out = process.wait(), True
        status = {"exit": code, "timed_out": timed_out, "seconds": round(time.monotonic() - started, 3)}
        Path(str(prefix) + ".status.json").write_text(json.dumps(status) + "\n")
    return 0


def execute_plan(plan: dict, plan_path: Path, wrapper: list[str]) -> None:
    plan_path.write_text(json.dumps(plan, indent=1) + "\n")
    if not wrapper:
        run_batch(plan)
        return
    command = [*wrapper, sys.executable, "-I", str(Path(__file__).resolve()), "run-batch", str(plan_path)]
    result = subprocess.run(command, env=clean_env())
    if result.returncode != 0:
        raise SetupError(f"the run batch exited {result.returncode} under {wrapper[0]}"
                         + (" (a host condition: the slot wrapper refused)" if result.returncode in (97, 98) else ""))


def read_run(prefix: Path) -> tuple[dict, str, bytes | None]:
    status_path = Path(str(prefix) + ".status.json")
    status = json.loads(status_path.read_text()) if status_path.is_file() else {"exit": None, "missing": True}
    stdout_path, envelope_path = Path(str(prefix) + ".stdout"), Path(str(prefix) + ".envelope.json")
    stdout = stdout_path.read_text(errors="replace") if stdout_path.is_file() else ""
    envelope = envelope_path.read_bytes() if envelope_path.is_file() else None
    return status, stdout, envelope


# ------------------------------------------------------------------ the comparison

def compare(args: argparse.Namespace) -> int:
    started = time.monotonic()
    repo = Path(git(Path(args.repo).resolve(), "rev-parse", "--show-toplevel").stdout.strip())
    scratch, work = Path(args.scratch), Path(args.work) if args.work else Path(args.scratch) / "build"
    for path, what in ((scratch, "--scratch"), (work, "--work")):
        ensure_outside_work_trees(path, what)
    ensure_new_or_empty(scratch, "--scratch")
    work.mkdir(parents=True, exist_ok=True)
    manifest_path = Path(args.manifest).resolve()
    manifest = load_manifest(manifest_path.read_text(encoding="utf-8"))
    cargo = shlex.split(args.cargo)
    wrapper = shlex.split(args.run_wrapper) if args.run_wrapper else []
    sides = {}
    for side, rev in (("A", args.a), ("B", args.b)):
        sha = resolve_commit(repo, rev)
        sides[side] = {"rev": rev, "sha": sha, "tree_sha": git(repo, "rev-parse", f"{sha}^{{tree}}").stdout.strip(),
                       "worktree": work / f"wt-{side}-{sha[:12]}", "target": work / "target" / f"{side}-{sha}",
                       "bin_dir": work / "bin" / f"{side}-{sha}"}
    made = []
    try:
        for side, info in sides.items():
            make_worktree(repo, info["worktree"], info["sha"])
            made.append(info["worktree"])
        return _compare_in_worktrees(args, repo, scratch, manifest, manifest_path, cargo, wrapper, sides, started)
    finally:
        if not args.keep_worktrees:
            for path in made:
                error = remove_worktree(repo, path)
                if error:
                    print(f"byte_compare: could not remove worktree {path}: {error}", file=sys.stderr)


def _compare_in_worktrees(args, repo, scratch, manifest, manifest_path, cargo, wrapper, sides, started) -> int:
    failures: list[str] = []
    for side, info in sides.items():
        tree = info["worktree"]
        info["target_version"] = target_version(tree)
        info["coverage"] = coverage(tree, manifest)
        for path in info["coverage"]["unnamed"]:
            failures.append(f"{side}: {path} is under a stated root and not in the manifest")
        for path in info["coverage"]["missing"]:
            failures.append(f"{side}: manifest path {path} does not exist")
    # Build and check both sides before anything runs.
    for side, info in sides.items():
        info["target"].mkdir(parents=True, exist_ok=True)
        info["build"] = build_driver(info["worktree"], info["sha"], info["target"], cargo, args.jobs,
                                     scratch / f"build-{side}.log")
        info["bin_dir"].mkdir(parents=True, exist_ok=True)
        binary = info["bin_dir"] / DRIVER_BIN
        shutil.copy2(info["build"]["binary"], binary)
        info["binary"] = binary
        info["binary_sha256"] = sha256_file(binary)
        info["identity"] = check_identity(binary, info["sha"], info["tree_sha"])
    # Inputs: the lift applied to temporary copies under the scratch directory only.
    records = {}
    runs = []
    for entry in manifest["entries"]:
        record = records[entry["path"]] = {"entry": entry, "sides": {}}
        for side, info in sides.items():
            source = info["worktree"] / entry["path"]
            facts = {"expected": entry["expect"][side]["disposition"], "source": str(source)}
            record["sides"][side] = facts
            if not source.is_file():
                facts.update(pre_class=MISSING, pre_detail="the manifest path does not exist")
                continue
            data = source.read_bytes()
            facts["input_sha256"] = sha256_bytes(data)
            try:
                lifted = lift(data, entry["source_version"], info["target_version"])
            except LiftError as error:
                facts.update(pre_class=LIFT_REFUSED, pre_detail=str(error))
                continue
            facts["lifted"] = lifted["lifted"]
            facts["shape"] = lifted["shape"]
            if lifted["outside"]:
                facts.update(pre_class=OUTSIDE_RUNNER_INPUT, pre_detail=lifted["outside"])
                continue
            input_dir = scratch / "inputs" / side
            input_dir.mkdir(parents=True, exist_ok=True)
            temp = input_dir / f"{entry_id(entry['path'])}.json"
            temp.write_bytes(serialize_request(lifted["request"]))
            facts["input"] = str(temp)
            for mode in MODES:
                prefix = scratch / "runs" / side / f"{entry_id(entry['path'])}.{mode}"
                prefix.parent.mkdir(parents=True, exist_ok=True)
                runs.append({"binary": str(info["binary"]), "input": str(temp), "mode": mode, "prefix": str(prefix)})
    plan = {"cwd": str(scratch), "timeout": args.timeout, "runs": runs}
    execute_plan(plan, scratch / "plan.json", wrapper)
    # Integrity after the runs: binaries, sources and worktrees as they were.
    for side, info in sides.items():
        if sha256_file(info["binary"]) != info["binary_sha256"]:
            failures.append(f"{side}: the binary changed during the runs")
        status = worktree_status(info["worktree"])
        if status:
            failures.append(f"{side}: the worktree changed during the runs:\n{status}")
    for record in records.values():
        for side, facts in record["sides"].items():
            source = Path(facts["source"])
            if "input_sha256" in facts and (not source.is_file() or sha256_file(source) != facts["input_sha256"]):
                failures.append(f"{side}: {record['entry']['path']} changed on disk during the comparison")
    # Classification and verdicts.
    pairs = []
    for path, record in records.items():
        for mode in MODES:
            observed = {}
            for side, facts in record["sides"].items():
                if "pre_class" in facts:
                    observed[side] = {"cls": facts["pre_class"], "detail": facts["pre_detail"], "envelope": None,
                                      "facts": {}}
                else:
                    prefix = scratch / "runs" / side / f"{entry_id(path)}.{mode}"
                    status, stdout, envelope = read_run(prefix)
                    cls, detail, run_facts = classify_run(status, stdout, envelope)
                    observed[side] = {"cls": cls, "detail": detail, "envelope": envelope, "facts": run_facts,
                                      "seconds": status.get("seconds"), "exit": status.get("exit")}
                observed[side]["expected"] = facts["expected"]
                observed[side]["input_sha256"] = facts.get("input_sha256")
            verdict = judge_pair(observed["A"], observed["B"])
            if verdict["failed"]:
                failures.append(f"{path} [{mode}]: {verdict['verdict']}: {verdict['why']}")
            pairs.append({"path": path, "mode": mode, **verdict,
                          "A": {k: v for k, v in observed["A"].items() if k != "envelope"},
                          "B": {k: v for k, v in observed["B"].items() if k != "envelope"},
                          "envelope_sha256": {s: (sha256_bytes(o["envelope"]) if o["envelope"] is not None else None)
                                              for s, o in observed.items()}})
    elapsed = time.monotonic() - started
    table = render_table(sides, manifest, manifest_path, records, pairs, failures, cargo, wrapper, elapsed)
    results = {"schema": RESULTS_SCHEMA, "manifest": str(manifest_path), "elapsed_seconds": round(elapsed, 1),
               "sides": {side: {k: (str(v) if isinstance(v, Path) else v) for k, v in info.items()}
                         for side, info in sides.items()},
               "pairs": pairs, "failures": failures}
    (scratch / "results.json").write_text(json.dumps(results, indent=1, default=str) + "\n")
    if args.table:
        Path(args.table).write_text(table)
    else:
        sys.stdout.write(table)
    return EXIT_FAILED if failures else EXIT_OK


def render_table(sides, manifest, manifest_path, records, pairs, failures, cargo, wrapper, elapsed) -> str:
    out = ["# Byte comparison", ""]
    out.append(f"Manifest: {manifest_path} ({len(manifest['entries'])} entries, {len(manifest['roots'])} roots)")
    out.append(f"Cargo: {' '.join(cargo)}; profile {PROFILE}; runner {DRIVER_CRATE} ({DRIVER_BIN}); "
               f"runs {'under ' + ' '.join(wrapper) if wrapper else 'direct'}")
    out.append("")
    out.append("| side | revision | commit | binary sha256 | identity (commit, dirty, profile) | target version |")
    out.append("|---|---|---|---|---|---|")
    for side, info in sides.items():
        identity = info.get("identity", {})
        out.append(f"| {side} | {info['rev']} | {info['sha']} | {info.get('binary_sha256', '-')} | "
                   f"{str(identity.get('source_commit', '-'))[:12]}, {identity.get('source_dirty')}, "
                   f"{identity.get('build_profile')} | {info.get('target_version', '-')} |")
    rustc = next((line for line in sides["A"].get("identity", {}).get("rustc", []) if line.startswith("rustc ")), "")
    if rustc:
        out.append(f"\nToolchain (A): {rustc}")
    out.append("")
    out.append("| fixture | mode | A | B | verdict |")
    out.append("|---|---|---|---|---|")
    for pair in pairs:
        cell = lambda o: o["cls"] if o["cls"] != NUMERICAL else f"numerical ({o['facts'].get('rows')} rows)"
        note = pair["verdict"] + (f": {pair['why']}" if pair["why"] else "")
        if not pair["compared"] and pair["envelope_sha256"]["A"] is not None:
            note += " (same bytes)" if pair["same_bytes"] else " (bytes differ)"
        out.append(f"| {pair['path']} | {pair['mode']} | {cell(pair['A'])} | {cell(pair['B'])} | {note} |")
    out.append("")
    compared = [p for p in pairs if p["compared"]]
    identical = sum(1 for p in compared if p["verdict"] == "IDENTICAL")
    by_class: dict[str, int] = {}
    for pair in pairs:
        if not pair["compared"] and not pair["failed"]:
            by_class[pair["A"]["cls"]] = by_class.get(pair["A"]["cls"], 0) + 1
    out.append(f"**Compared with numerical work on both sides: {len(compared)} of {len(pairs)} pairs** "
               f"({identical} identical, {len(compared) - identical} different).")
    out.append("Not compared, as the manifest expects: "
               + (", ".join(f"{cls} {n}" for cls, n in sorted(by_class.items())) or "none") + ".")
    out.append(f"Failed pairs: {sum(1 for p in pairs if p['failed'])}.")
    out.append("")
    out.append("Coverage under the stated roots:")
    for side, info in sides.items():
        cov = info.get("coverage", {})
        out.append(f"- {side}: not in the manifest: {', '.join(cov.get('unnamed', [])) or 'none'}; "
                   f"manifest paths that do not exist: {', '.join(cov.get('missing', [])) or 'none'}")
    out.append("")
    out.append("Entries not expected to reach numerical work, and what each side did:")
    listed = False
    for path, record in records.items():
        expectations = {side: record["entry"]["expect"][side]["disposition"] for side in ("A", "B")}
        if all(value == NUMERICAL for value in expectations.values()):
            continue
        listed = True
        for pair in (p for p in pairs if p["path"] == path):
            out.append(f"- {path} [{pair['mode']}]: expected A {expectations['A']}, B {expectations['B']}; "
                       f"A did {pair['A']['cls']} ({pair['A']['detail']}); B did {pair['B']['cls']} ({pair['B']['detail']})")
    if not listed:
        out.append("- none")
    out.append("")
    out.append(f"Failures: {len(failures)}")
    out.extend(f"- {failure}" for failure in failures)
    out.append("")
    out.append(f"Result: {'FAIL' if failures else 'PASS'}; elapsed {elapsed:.0f} s.")
    return "\n".join(out) + "\n"


# ------------------------------------------------------------------ command line

def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    sub = parser.add_subparsers(dest="command", required=True)
    run = sub.add_parser("compare", help="build both revisions and compare every manifest fixture")
    run.add_argument("--a", required=True, help="revision A (before)")
    run.add_argument("--b", required=True, help="revision B (after)")
    run.add_argument("--scratch", required=True, help="new or empty directory for raw outputs, outside every git work tree")
    run.add_argument("--work", help="directory for the worktrees, target directories and binaries (default <scratch>/build)")
    run.add_argument("--manifest", default=str(DEFAULT_MANIFEST))
    run.add_argument("--repo", default=str(TOOL_DIR), help="any path in the repository whose revisions are compared")
    run.add_argument("--cargo", default="cargo", help="the cargo command, e.g. a host wrapper (shell-split)")
    run.add_argument("--jobs", type=int, default=4, help="cargo build jobs (default 4)")
    run.add_argument("--run-wrapper", default="", help="command prefix for the run batch, e.g. a host slot wrapper")
    run.add_argument("--timeout", type=float, default=1800.0, help="seconds per driver run (default 1800)")
    run.add_argument("--table", help="write the table here instead of stdout")
    run.add_argument("--keep-worktrees", action="store_true", help="leave the worktrees for inspection")
    batch = sub.add_parser("run-batch", help=argparse.SUPPRESS)
    batch.add_argument("plan")
    args = parser.parse_args(argv)
    if args.command == "run-batch":
        return run_batch(json.loads(Path(args.plan).read_text()))
    try:
        return compare(args)
    except SetupError as error:
        print(f"byte_compare: {error}", file=sys.stderr)
        return EXIT_SETUP


if __name__ == "__main__":
    sys.exit(main())
