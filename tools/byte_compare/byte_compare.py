#!/usr/bin/env python3
"""Before/after byte comparison of maintained solve requests between two revisions.

Standard library only. `README.md` beside this file states the use, the inputs and outputs, the exit
codes, the lift and the threat model this tool is reviewed against.

For each of revisions A and B it makes a clean detached worktree of exactly that commit, builds the
release timing driver (`core/solver/release_timing`, the desktop solve worker's engine stages at the
desktop's build) there with `cargo build -vv --message-format=json --release --locked` into a fresh
target directory, attests from that build's own output that every unit was compiled with the
manifest's build settings from the stated sources, checks the binary's embedded build identity
against the commit, and runs every manifest fixture through the driver in both solver modes.
A pair is COMPARED only where both sides reached numerical work; every other pair is reported by its
class, and a pair whose observed class differs from the manifest fails.
"""
from __future__ import annotations

import argparse
import copy
import hashlib
import io
import json
import os
from pathlib import Path, PurePosixPath
import re
import shlex
import shutil
import subprocess
import sys
import tarfile
import time
import tomllib
from urllib.parse import unquote

# Every directory this tool scans is a worktree or scratch directory its caller names, outside this
# checkout; a test that points it at project paths declares those paths itself.
CI_READS = ()

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

# The build attestation (threat (b); README "Build attestation"). The code-generation options every
# attested unit's settings are compared on (the manifest states each, null meaning cargo passes none),
# and the two that only name a unit and are never compared.
CODEGEN_KEYS = ("opt-level", "debug-assertions", "overflow-checks", "lto", "codegen-units", "panic", "debuginfo",
                "strip", "embed-bitcode")
UNIT_ID_KEYS = ("metadata", "extra-filename")
# The profile fields of cargo's JSON artifact message, its primary record of a unit's settings.
PROFILE_KEYS = ("opt_level", "debuginfo", "debug_assertions", "overflow_checks")
# Environment cargo itself gives a compiler invocation; anything else there must be a variable the
# package's build script declared (`cargo:rustc-env`), or the build is refused.
RUSTC_ENV = frozenset({"CARGO", "CARGO_CRATE_NAME", "CARGO_MANIFEST_DIR", "CARGO_MANIFEST_PATH",
                       "CARGO_PRIMARY_PACKAGE", "CARGO_BIN_NAME", "OUT_DIR"})
# Environment cargo gives a build-script run.
BUILD_SCRIPT_ENV = frozenset({"CARGO", "CARGO_ENCODED_RUSTFLAGS", "CARGO_MANIFEST_DIR", "CARGO_MANIFEST_PATH",
                              "DEBUG", "HOST", "NUM_JOBS", "OPT_LEVEL", "OUT_DIR", "PROFILE", "RUSTC", "RUSTDOC",
                              "TARGET"})
BUILD_SCRIPT_ENV_PREFIXES = ("CARGO_CFG_", "CARGO_FEATURE_", "CARGO_PKG_")
# The dynamic-library search path cargo sets for every process it starts (macOS, Linux).
LIBRARY_PATH_ENV = frozenset({"DYLD_FALLBACK_LIBRARY_PATH", "LD_LIBRARY_PATH"})
# Build-script output cargo acts on that this runner's closure may use; any other line refuses.
BUILD_SCRIPT_DIRECTIVES = frozenset({"rerun-if-changed", "rerun-if-env-changed", "rustc-check-cfg", "rustc-cfg",
                                     "rustc-env", "warning"})
BUILD_SCRIPT_LINE = re.compile(r"^\[(\S+) (\S+)\] (.*)$")
STDERR_STATUS = re.compile(r"^\s+(Compiling \S+ v\S+( \(.*\))?|Finished `release` profile .*|"
                           r"Blocking waiting for file lock on .*)$")
ENV_ASSIGNMENT = re.compile(r"^([A-Za-z_][A-Za-z0-9_]*)=(.*)$", re.S)
EMIT_KINDS = frozenset({"dep-info", "metadata", "link"})
REGISTRY_SRC = re.compile(r"^(?P<home>/.+)/registry/src/(?P<index>index\.crates\.io-[0-9a-f]{16})/"
                          r"(?P<dir>[^/]+)/(?P<rest>.+)$")

EXIT_OK, EXIT_FAILED, EXIT_SETUP = 0, 1, 2


class SetupError(Exception):
    """A condition that prevents a sound comparison: exit 2, nothing compared."""


class ManifestError(SetupError):
    pass


class BuildRefused(SetupError):
    """The build's own record does not establish the settings and sources it must have."""


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


# The run wrapper's own refusals (t3_slot.sh: 97, its guard is not running; 98, a host condition).
WRAPPER_REFUSALS = (97, 98)


def usable_wrapper(wrapper: list[str], what: str) -> None:
    """Refuses, before anything is built, a wrapper that cannot be started."""
    if not wrapper:
        return
    program = wrapper[0]
    found = program if os.sep in program else shutil.which(program)
    if not found or not os.path.isfile(found) or not os.access(found, os.X_OK):
        raise SetupError(f"{what} {program} cannot be used: it is not an executable file")


def run_wrapped(wrapper: list[str], command: list[str], what: str) -> subprocess.CompletedProcess:
    """Runs `command` under the run wrapper (directly only when none is given): every start of a
    built binary or of the build's rustc goes through it, as the run batch does. A wrapper that
    cannot be started, or that refuses, stops the comparison."""
    try:
        result = subprocess.run([*wrapper, *command], capture_output=True, text=True, env=clean_env())
    except OSError as error:
        raise SetupError(f"{what} could not be started{' under ' + wrapper[0] if wrapper else ''}: {error}") from error
    if wrapper and result.returncode in WRAPPER_REFUSALS:
        raise SetupError(f"the run wrapper {wrapper[0]} refused {what} (exit {result.returncode}, a host condition)")
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


# What the tool itself writes directly in the scratch directory; the table may not take one of these names.
SCRATCH_EVIDENCE = frozenset({"plan.json", "results.json", "build-A.log", "build-B.log", "build-A.jsonl",
                              "build-B.jsonl", "inputs", "runs", "build"})


def table_destination(given: str, scratch: Path, work: Path) -> Path:
    """The resolved --table path, refused unless it is outside every git work tree, does not exist (so
    no file is truncated, including one hard-linked to a frozen request), and is none of the tool's own
    paths: nothing under --work, which holds the generated worktrees, and none of the scratch evidence."""
    raw = Path(given)
    path = raw.resolve()
    ensure_outside_work_trees(path, "--table")
    if os.path.lexists(raw) or os.path.lexists(path):
        raise SetupError(f"--table {given} already exists; the table is written only to a new file")
    for root in (work.resolve(), *([scratch.resolve()] if scratch.exists() else [])):
        if path == root or root in path.parents:
            inside = path.relative_to(root).parts
            if root == work.resolve() or not inside or inside[0] in SCRATCH_EVIDENCE:
                raise SetupError(f"--table {path} is one of the tool's own paths under {root}")
    return path


def write_table(path: Path, table: str) -> None:
    """Writes the table to `path`, created exclusively, after checking again that it is outside every
    git work tree (the comparison has made worktrees since the first check)."""
    ensure_outside_work_trees(path, "--table")
    try:
        with open(path, "x", encoding="utf-8") as handle:
            handle.write(table)
    except FileExistsError as error:
        raise SetupError(f"--table {path} came into existence during the comparison; nothing was written") from error


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
    _closed(raw, {"schema", "description", "roots", "entries", "build_settings", "build_closure"},
            {"schema", "roots", "entries", "build_settings", "build_closure"}, "the manifest")
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
    return {"roots": roots, "entries": entries, "build_settings": _build_settings(raw["build_settings"]),
            "build_closure": _build_closure(raw["build_closure"])}


def _codegen(record, what: str, keys: tuple[str, ...] | None) -> dict:
    if not isinstance(record, dict):
        raise ManifestError(f"{what} must be an object")
    if keys is not None and sorted(record) != sorted(keys):
        raise ManifestError(f"{what} must state exactly {list(keys)}")
    for key, value in record.items():
        if keys is None and key in CODEGEN_KEYS + UNIT_ID_KEYS:
            raise ManifestError(f"{what} may not restate {key}")
        if not (value is None or value is True or (isinstance(value, str) and value)):
            raise ManifestError(f"{what}.{key} must be a string, true (a flag with no value) or null (absent)")
    return dict(record)


def _profile(record, what: str) -> dict:
    _closed(record, set(PROFILE_KEYS), set(PROFILE_KEYS), what)
    if not isinstance(record["opt_level"], str) or not isinstance(record["debuginfo"], int) \
            or not all(isinstance(record[key], bool) for key in ("debug_assertions", "overflow_checks")):
        raise ManifestError(f"{what}: opt_level is a string, debuginfo an integer, the assertions booleans")
    return dict(record)


def _build_settings(raw) -> dict:
    """The build settings every attested unit must have been compiled with, stated once."""
    _closed(raw, {"ground", "runtime", "host"}, {"ground", "runtime", "host"}, "build_settings")
    blocks = {}
    for name, extra in (("runtime", set()), ("host", {"proc_macro_codegen"})):
        what = f"build_settings.{name}"
        _closed(raw[name], {"why", "profile", "codegen"} | extra, {"why", "profile", "codegen"} | extra, what)
        blocks[name] = {"profile": _profile(raw[name]["profile"], f"{what}.profile"),
                        "codegen": _codegen(raw[name]["codegen"], f"{what}.codegen", CODEGEN_KEYS)}
        if extra:
            blocks[name]["proc_macro_codegen"] = _codegen(raw[name]["proc_macro_codegen"],
                                                          f"{what}.proc_macro_codegen", None)
    return blocks


def _build_closure(raw) -> dict:
    """The runner's supported build closure: its one registry, and which build scripts may leave
    which generated files in their OUT_DIR."""
    _closed(raw, {"why", "registry", "generated_sources"}, {"registry", "generated_sources"}, "build_closure")
    if not isinstance(raw["registry"], str) or not raw["registry"].startswith("registry+"):
        raise ManifestError("build_closure.registry must be a registry source id")
    generated = raw["generated_sources"]
    if not isinstance(generated, dict):
        raise ManifestError("build_closure.generated_sources must be an object")
    for package, files in generated.items():
        if not isinstance(files, list) or not files:
            raise ManifestError(f"build_closure.generated_sources.{package} must list at least one file")
        for name in files:
            relative_posix(name, f"build_closure.generated_sources.{package} file")
    return {"registry": raw["registry"], "generated_sources": {k: sorted(v) for k, v in generated.items()}}


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


# ------------------------------------------------------------------ build attestation (threat (b))
#
# Everything here is read from the build cargo performed: its JSON messages (stdout), its -vv text
# (the compiler invocations on stderr; build-script output on stdout) and the target directory. The
# tree is never edited, so a historical revision is attested exactly as committed.

def _inside(path: str, root: str) -> bool:
    path, root = os.path.normpath(path), os.path.normpath(root)
    return os.path.isabs(path) and os.path.commonpath([path, root]) == root


def parse_build_output(stdout: str, stderr: str) -> dict:
    """The build's messages and invocations. Anything this format does not account for refuses: a line
    that is neither cargo JSON nor build-script output, an unknown message, an unknown stderr line, a
    unit cargo reports as fresh, or a build that did not finish successfully exactly once."""
    artifacts, scripts, finished, directives = [], [], [], {}
    for number, line in enumerate(stdout.splitlines(), 1):
        if not line.strip():
            continue
        if line.startswith("{"):
            try:
                message = json.loads(line)
            except json.JSONDecodeError as error:
                raise BuildRefused(f"build stdout line {number} is not JSON: {error}") from error
            reason = message.get("reason") if isinstance(message, dict) else None
            if reason == "compiler-artifact":
                artifacts.append(message)
            elif reason == "build-script-executed":
                scripts.append(message)
            elif reason == "build-finished":
                finished.append(message)
            elif reason != "compiler-message":
                raise BuildRefused(f"unknown build message {reason!r} on stdout line {number}")
            continue
        match = BUILD_SCRIPT_LINE.match(line)
        if not match:
            raise BuildRefused(f"build stdout line {number} is neither cargo JSON nor build-script output: {line[:160]!r}")
        directives.setdefault((match[1], match[2]), []).append(match[3])
    if [message.get("success") for message in finished] != [True]:
        raise BuildRefused("the build did not report exactly one successful build-finished message")
    invocations, pending, start = [], None, 0
    for number, line in enumerate(stderr.split("\n"), 1):
        if pending is None:
            stripped = line.strip()
            if stripped.startswith("Running `"):
                pending, start = stripped[len("Running `"):], number
            elif stripped.startswith("Fresh "):
                raise BuildRefused(f"cargo reports a fresh unit, not compiled by this build: {stripped}")
            elif stripped and not STDERR_STATUS.match(line):
                raise BuildRefused(f"unknown build output on stderr line {number}: {stripped[:160]!r}")
        else:
            pending += "\n" + line
        if pending is not None and pending.endswith("`"):
            try:
                tokens = shlex.split(pending[:-1])
            except ValueError:
                continue  # a quoted value spans lines; the invocation continues
            invocations.append(_split_invocation(tokens, start))
            pending = None
    if pending is not None:
        raise BuildRefused(f"the invocation starting on stderr line {start} never ends")
    for artifact in artifacts:
        if artifact.get("fresh") is not False:
            raise BuildRefused(f"{artifact.get('package_id')} is not reported as compiled by this build (fresh)")
    return {"artifacts": artifacts, "scripts": scripts, "directives": directives, "invocations": invocations}


def _split_invocation(tokens: list[str], line: int) -> dict:
    env, index = {}, 0
    while index < len(tokens) and (match := ENV_ASSIGNMENT.match(tokens[index])):
        env[match[1]] = match[2]
        index += 1
    if index >= len(tokens):
        raise BuildRefused(f"the invocation on stderr line {line} names no program")
    return {"env": env, "program": tokens[index], "args": tokens[index + 1:], "line": line}


def parse_rustc_args(args: list[str], line: int) -> dict:
    """One compiler invocation's arguments. An argument this runner's build does not use refuses: -Z,
    --target, a library (-l), a sysroot, an output path, a print request and anything else unknown."""
    unit = {"crate_name": None, "crate_types": [], "sources": [], "codegen": {}, "cfgs": [], "out_dir": None,
            "lib_paths": [], "externs": []}
    index = 0

    def take() -> str:
        nonlocal index
        index += 1
        if index >= len(args):
            raise BuildRefused(f"rustc argument {args[index - 1]!r} on stderr line {line} has no value")
        return args[index]

    while index < len(args):
        arg = args[index]
        if arg == "--crate-name":
            unit["crate_name"] = take()
        elif arg == "--crate-type":
            unit["crate_types"].append(take())
        elif arg.startswith("--emit="):
            kinds = set(arg[len("--emit="):].split(","))
            if not kinds <= EMIT_KINDS:
                raise BuildRefused(f"rustc on stderr line {line} emits {sorted(kinds - EMIT_KINDS)}")
        elif arg.startswith("--edition=") or arg == "--error-format=json" or arg.startswith("--json="):
            pass
        elif arg == "-C" or (arg.startswith("-C") and len(arg) > 2):
            text = take() if arg == "-C" else arg[2:]
            key, has_value, value = text.partition("=")
            if key in unit["codegen"]:
                raise BuildRefused(f"rustc on stderr line {line} sets -C {key} twice")
            unit["codegen"][key] = value if has_value else True
        elif arg == "--cfg":
            unit["cfgs"].append(take())
        elif arg in ("--check-cfg", "--cap-lints"):
            take()
        elif arg.startswith(("--allow=", "--warn=", "--deny=", "--forbid=")):
            pass  # lint levels from the packages' [lints] tables
        elif arg == "--out-dir":
            unit["out_dir"] = take()
        elif arg == "-L":
            unit["lib_paths"].append(take())
        elif arg == "--extern":
            unit["externs"].append(take())
        elif not arg.startswith("-"):
            unit["sources"].append(arg)
        else:
            raise BuildRefused(f"unknown rustc argument {arg!r} on stderr line {line}")
        index += 1
    if not unit["crate_name"] or not unit["crate_types"] or len(unit["sources"]) != 1 or not unit["out_dir"]:
        raise BuildRefused(f"the compiler invocation on stderr line {line} does not name one crate, source and output")
    return unit


def compiler_program(parsed: dict) -> str:
    """The one compiler every unit was compiled with: every invocation that is not a build-script run
    names the same program, and that program is rustc itself (not a wrapper)."""
    scripts = {name for artifact in parsed["artifacts"] if artifact["target"]["kind"] == ["custom-build"]
               for name in artifact.get("filenames", [])}
    programs = {inv["program"] for inv in parsed["invocations"] if inv["program"] not in scripts}
    if len(programs) != 1:
        raise BuildRefused(f"the units were compiled by {len(programs)} programs, not one rustc: {sorted(programs)}")
    program = programs.pop()
    if os.path.basename(program) not in ("rustc", "rustc.exe") or not os.path.isabs(program):
        raise BuildRefused(f"the compiler {program} is not rustc named by an absolute path (a wrapper?)")
    return program


def rustc_identity(program: str, wrapper: list[str] = ()) -> dict:
    """`rustc -vV` and the sysroot of the compiler the build used. The prebuilt standard library in that
    sysroot is the trusted part of the toolchain (cargo compiles it nowhere in the build); what identifies
    it is this output and the sysroot path, and the compiler must be that sysroot's own `bin/rustc`."""
    version = run_wrapped(list(wrapper), [program, "-vV"], f"{program} -vV")
    sysroot = run_wrapped(list(wrapper), [program, "--print", "sysroot"], f"{program} --print sysroot")
    if version.returncode != 0 or sysroot.returncode != 0 or not version.stdout.startswith("rustc "):
        raise BuildRefused(f"{program} gives no rustc -vV and sysroot")
    root = sysroot.stdout.strip()
    if os.path.normpath(program) != os.path.join(os.path.normpath(root), "bin", os.path.basename(program)):
        raise BuildRefused(f"the compiler {program} is not the rustc of its own sysroot {root}")
    hosts = [line[len("host: "):] for line in version.stdout.splitlines() if line.startswith("host: ")]
    if len(hosts) != 1:
        raise BuildRefused(f"{program} -vV names no single host")
    return {"program": program, "version": version.stdout, "sysroot": root, "host": hosts[0]}


def _package(package_id: str, closure: dict, tree: str) -> dict:
    source, _, tail = package_id.partition("#")
    if "@" in tail:
        name, version = tail.rsplit("@", 1)
    else:
        name, version = source.rstrip("/").rsplit("/", 1)[-1], tail
    if source.startswith("path+file://"):
        path = unquote(source[len("path+file://"):])
        if not _inside(path, tree):
            raise BuildRefused(f"{package_id} is a path source outside the generated worktree")
        return {"id": package_id, "name": name, "version": version, "kind": "path", "source": None}
    if source == closure["registry"]:
        return {"id": package_id, "name": name, "version": version, "kind": "registry", "source": source}
    raise BuildRefused(f"{package_id} comes from neither the generated worktree nor {closure['registry']}")


# Registry packages already verified in this process (each archive is read once).
_REGISTRY_VERIFIED: dict = {}


def verify_registry_package(home: str, index: str, name: str, version: str, checksum: str) -> None:
    """The registry package compiled is exactly the one Cargo.lock names: its cached archive's SHA-256
    is the lock's checksum, and the extracted source cargo compiled holds exactly the archive's files
    and bytes (plus cargo's own `.cargo-ok` marker)."""
    key = (home, index, name, version, checksum)
    if key in _REGISTRY_VERIFIED:
        return
    stem = f"{name}-{version}"
    archive_path = Path(home) / "registry" / "cache" / index / f"{stem}.crate"
    root = Path(home) / "registry" / "src" / index / stem
    if not archive_path.is_file() or not root.is_dir():
        raise BuildRefused(f"{stem}: no cached archive {archive_path} beside its source")
    data = archive_path.read_bytes()
    if sha256_bytes(data) != checksum:
        raise BuildRefused(f"{stem}: the cached archive's SHA-256 is not Cargo.lock's checksum")
    expected = {}
    with tarfile.open(fileobj=io.BytesIO(data), mode="r:gz") as archive:
        for member in archive.getmembers():
            if member.isdir():
                continue
            parts = PurePosixPath(member.name).parts
            if not member.isfile() or len(parts) < 2 or parts[0] != stem or ".." in parts:
                raise BuildRefused(f"{stem}: the archive holds an entry this tool does not accept: {member.name}")
            expected["/".join(parts[1:])] = sha256_bytes(archive.extractfile(member).read())
    actual = {}
    for path in root.rglob("*"):
        if path.is_symlink():
            raise BuildRefused(f"{stem}: the extracted source holds a symbolic link {path}")
        if path.is_file():
            actual[path.relative_to(root).as_posix()] = sha256_file(path)
    actual.pop(".cargo-ok", None)
    if actual != expected:
        changed = sorted(k for k in set(actual) | set(expected) if actual.get(k) != expected.get(k))
        raise BuildRefused(f"{stem}: the extracted source is not the locked archive: {changed[:5]}")
    _REGISTRY_VERIFIED[key] = True


def generated_files(out_dir: str) -> dict:
    """Every file a build script left in its OUT_DIR, with its SHA-256."""
    root = Path(out_dir)
    if not root.is_dir():
        raise BuildRefused(f"the build-script output directory {out_dir} is absent")
    return {path.relative_to(root).as_posix(): sha256_file(path) for path in sorted(root.rglob("*"))
            if path.is_file() or path.is_symlink()}


def attest_build(parsed: dict, *, tree: str, target: str, lock_text: str, settings: dict, closure: dict,
                 rustc: dict, list_generated=generated_files, verify_registry=verify_registry_package) -> dict:
    """Attests a parsed build (README "Build attestation"); raises BuildRefused on any departure.

    Runtime profile checks: the runner and every unit it links, followed through --extern, have the
    manifest's runtime settings. Build-time dependency checks: build scripts, proc macros and what they
    link have its host settings. Every unit's source is in the generated worktree or is the exact
    Cargo.lock registry package; every flag and environment variable is accounted for."""
    try:
        lock = tomllib.loads(lock_text)
    except tomllib.TOMLDecodeError as error:
        raise BuildRefused(f"Cargo.lock is not TOML: {error}") from error
    locked = {}
    for entry in lock.get("package", []):
        key = (entry.get("name"), entry.get("version"))
        if key in locked:
            raise BuildRefused(f"Cargo.lock names {key[0]} {key[1]} twice")
        locked[key] = entry
    program = rustc["program"]
    # Units: each compiler invocation is exactly one artifact message, and the reverse.
    by_key, by_file, packages = {}, {}, {}
    registry_checked = set()
    for artifact in parsed["artifacts"]:
        package = packages.setdefault(artifact["package_id"], _package(artifact["package_id"], closure, tree))
        entry = locked.get((package["name"], package["version"]))
        if entry is None or entry.get("source") != package["source"]:
            raise BuildRefused(f"{package['id']} is not in Cargo.lock with that source")
        target_info = artifact["target"]
        if package["kind"] == "path":
            if not _inside(target_info["src_path"], tree) or not _inside(artifact["manifest_path"], tree):
                raise BuildRefused(f"{package['id']} compiles a source outside the generated worktree")
        else:
            match = REGISTRY_SRC.match(target_info["src_path"])
            if not match or match["dir"] != f"{package['name']}-{package['version']}" or artifact["manifest_path"] != \
                    os.path.join(match["home"], "registry", "src", match["index"], match["dir"], "Cargo.toml"):
                raise BuildRefused(f"{package['id']} is not compiled from its registry source (source replacement?)")
            checksum = entry.get("checksum")
            if not isinstance(checksum, str) or not re.fullmatch(r"[0-9a-f]{64}", checksum):
                raise BuildRefused(f"{package['id']} has no checksum in Cargo.lock")
            if package["id"] not in registry_checked:
                verify_registry(match["home"], match["index"], package["name"], package["version"], checksum)
                registry_checked.add(package["id"])
        crate = "build_script_build" if target_info["kind"] == ["custom-build"] else target_info["name"].replace("-", "_")
        key = (artifact["manifest_path"], crate, tuple(target_info["crate_types"]))
        if key in by_key:
            raise BuildRefused(f"{package['id']} compiles {crate} twice; outside this runner's supported closure")
        unit = {"artifact": artifact, "package": package, "key": key, "invocation": None,
                "kind": target_info["kind"], "crate_types": target_info["crate_types"]}
        by_key[key] = unit
        for name in artifact.get("filenames", []):
            by_file[name] = unit
    scripts_by_program = {name: unit for unit in by_key.values() if unit["kind"] == ["custom-build"]
                          for name in unit["artifact"].get("filenames", [])}
    runs = []
    for invocation in parsed["invocations"]:
        if invocation["program"] in scripts_by_program:
            runs.append(invocation)
            continue
        if invocation["program"] != program:
            raise BuildRefused(f"stderr line {invocation['line']} runs {invocation['program']}, not the build's rustc")
        args = parse_rustc_args(invocation["args"], invocation["line"])
        key = (invocation["env"].get("CARGO_MANIFEST_PATH"), args["crate_name"], tuple(args["crate_types"]))
        unit = by_key.get(key)
        if unit is None or unit["invocation"] is not None:
            raise BuildRefused(f"the compiler invocation on stderr line {invocation['line']} ({key[1]}) is not "
                               "exactly one artifact of this build")
        unit["invocation"], unit["args"] = invocation, args
    for unit in by_key.values():
        if unit["invocation"] is None:
            raise BuildRefused(f"{unit['package']['id']} ({unit['key'][1]}) has no compiler invocation in the -vv output")
    # The attested sets, from the link graph.
    manifest_path = os.path.join(tree, DRIVER_CRATE, "Cargo.toml")
    runners = [u for u in by_key.values() if u["kind"] == ["bin"] and u["artifact"]["target"]["name"] == DRIVER_BIN
               and u["artifact"]["manifest_path"] == manifest_path]
    if len(runners) != 1:
        raise BuildRefused(f"the build has {len(runners)} {DRIVER_BIN} binaries from {manifest_path}, not one")
    runner = runners[0]

    def links(unit: dict) -> list[dict]:
        found = []
        for extern in unit["args"]["externs"]:
            name, has_path, path = extern.partition("=")
            if not has_path:
                if name != "proc_macro" or unit["crate_types"] != ["proc-macro"]:
                    raise BuildRefused(f"{unit['key'][1]} names --extern {extern} with no file")
                continue
            if path not in by_file:
                raise BuildRefused(f"{unit['key'][1]} links {path}, which this build did not produce")
            found.append(by_file[path])
        return found

    runtime, queue = {id(runner): runner}, [runner]
    while queue:
        for linked in links(queue.pop()):
            if linked["crate_types"] != ["proc-macro"] and id(linked) not in runtime:
                runtime[id(linked)] = linked
                queue.append(linked)
    host = {id(u): u for u in by_key.values() if u["kind"] == ["custom-build"] or u["crate_types"] == ["proc-macro"]}
    queue = list(host.values())
    while queue:
        for linked in links(queue.pop()):
            if id(linked) not in host:
                host[id(linked)] = linked
                queue.append(linked)
    both = sorted(u["key"][1] for k, u in runtime.items() if k in host)
    if both:
        raise BuildRefused(f"units linked both at runtime and at build time: {both}")
    outside = sorted(u["key"][1] for k, u in ((id(u), u) for u in by_key.values()) if k not in runtime and k not in host)
    if outside:
        raise BuildRefused(f"units neither linked by the runner nor used at build time: {outside}")
    # Build scripts: what they told cargo, what they generated, and how they ran.
    scripts = {}
    for message in parsed["scripts"]:
        package = packages.get(message.get("package_id"))
        if package is None or package["id"] in scripts:
            raise BuildRefused(f"build-script output for {message.get('package_id')} is not one script of this build")
        if message.get("linked_libs") or message.get("linked_paths"):
            raise BuildRefused(f"{package['id']}'s build script adds link inputs {message.get('linked_libs')} "
                               f"{message.get('linked_paths')}; outside this runner's supported closure")
        if not _inside(message.get("out_dir", ""), target):
            raise BuildRefused(f"{package['id']}'s build-script output directory is outside the target directory")
        cfgs, env = [], []
        for text in parsed["directives"].pop((package["name"], package["version"]), []):
            match = re.fullmatch(r"cargo::?([a-z-]+)=(.*)", text, re.S)
            if not match or match[1] not in BUILD_SCRIPT_DIRECTIVES:
                raise BuildRefused(f"unknown build-script output from {package['id']}: {text[:160]!r}")
            if match[1] == "rustc-cfg":
                cfgs.append(match[2])
            elif match[1] == "rustc-env":
                name, has_value, value = match[2].partition("=")
                if not has_value:
                    raise BuildRefused(f"{package['id']} declares rustc-env {match[2]!r} with no value")
                env.append([name, value])
        if cfgs != message.get("cfgs") or env != message.get("env"):
            raise BuildRefused(f"{package['id']}'s build-script output and cargo's record of it disagree")
        files = list_generated(message["out_dir"])
        allowed = set(closure["generated_sources"].get(package["name"], []))
        unknown = sorted(set(files) - allowed)
        if unknown:
            raise BuildRefused(f"{package['id']}'s build script generated {unknown}, an arrangement outside this "
                               "runner's supported closure")
        scripts[package["id"]] = {"message": message, "env": dict(env), "generated": files, "run": None}
    if parsed["directives"]:
        raise BuildRefused(f"build-script output from {sorted(parsed['directives'])} that cargo did not record")
    # Each attested unit: its settings, its flags, its environment.
    blocks = {"runtime": settings["runtime"], "host": settings["host"]}
    served: dict[str, set] = {}
    for unit in by_key.values():
        cls = "runtime" if id(unit) in runtime else "host"
        block, artifact, args, env = blocks[cls], unit["artifact"], unit["args"], unit["invocation"]["env"]
        what = f"{unit['package']['id']} ({unit['key'][1]}, {cls})"
        check_settings(what, cls, block, unit)
        for path in args["lib_paths"]:
            kind, _, where = path.partition("=")
            if kind != "dependency" or not _inside(where, target):
                raise BuildRefused(f"{what} searches -L {path}; only this build's own dependency directory is accepted")
        if not _inside(args["out_dir"], target):
            raise BuildRefused(f"{what} writes outside the target directory")
        source = args["sources"][0]
        if source != artifact["target"]["src_path"] and \
                os.path.join(env.get("CARGO_MANIFEST_DIR", ""), source) != artifact["target"]["src_path"]:
            raise BuildRefused(f"{what} compiles {source}, not {artifact['target']['src_path']}")
        if env.get("CARGO_MANIFEST_PATH") != artifact["manifest_path"] or env.get("CARGO_PKG_NAME") != \
                unit["package"]["name"] or env.get("CARGO_PKG_VERSION") != unit["package"]["version"]:
            raise BuildRefused(f"{what}'s compiler environment names another package")
        features = sorted(c for c in args["cfgs"] if c.startswith("feature="))
        if features != sorted(f'feature="{f}"' for f in artifact["features"]):
            raise BuildRefused(f"{what}'s feature cfgs are not cargo's resolved features {artifact['features']}")
        other_cfgs = sorted(c for c in args["cfgs"] if not c.startswith("feature="))
        script = None
        if "OUT_DIR" in env:
            script = scripts.get(unit["package"]["id"])
            if script is None or script["message"]["out_dir"] != env["OUT_DIR"] or unit["kind"] == ["custom-build"]:
                raise BuildRefused(f"{what}'s OUT_DIR is not its package's build-script output")
            served.setdefault(unit["package"]["id"], set()).add(cls)
        if other_cfgs != sorted(script["message"]["cfgs"] if script else []):
            raise BuildRefused(f"{what} has --cfg {other_cfgs} that its build script did not declare")
        declared = script["env"] if script else {}
        for name, value in env.items():
            if name in RUSTC_ENV or name in LIBRARY_PATH_ENV or name.startswith("CARGO_PKG_"):
                continue
            if declared.get(name) != value:
                raise BuildRefused(f"{what} was compiled with an unexpected environment variable {name}")
        unit["class"] = cls
    for package_id, script in scripts.items():
        if len(served.get(package_id, ())) != 1:
            raise BuildRefused(f"{package_id}'s build script serves {sorted(served.get(package_id, ()))} units, not one class")
    for run in runs:
        compile_unit = scripts_by_program[run["program"]]
        env = run["env"]
        script = scripts.get(compile_unit["package"]["id"])
        if script is None or script["run"] is not None or env.get("OUT_DIR") != script["message"]["out_dir"]:
            raise BuildRefused(f"the build-script run on stderr line {run['line']} is not one script's run")
        script["run"] = run
        cls = served[compile_unit["package"]["id"]].copy().pop()
        block = blocks[cls]["profile"]
        wanted = {"CARGO_ENCODED_RUSTFLAGS": "", "RUSTC": program, "HOST": rustc["host"], "TARGET": rustc["host"],
                  "PROFILE": PROFILE, "OPT_LEVEL": block["opt_level"], "DEBUG": "true" if block["debuginfo"] else "false"}
        for name, value in wanted.items():
            if env.get(name) != value:
                raise BuildRefused(f"{compile_unit['package']['id']}'s build script ran with {name}={env.get(name)!r}, "
                                   f"not {value!r}")
        for name in env:
            if name not in BUILD_SCRIPT_ENV and name not in LIBRARY_PATH_ENV and not name.startswith(BUILD_SCRIPT_ENV_PREFIXES):
                raise BuildRefused(f"{compile_unit['package']['id']}'s build script ran with an unexpected environment "
                                   f"variable {name}")
        if run["args"]:
            raise BuildRefused(f"the build-script run on stderr line {run['line']} has arguments")
    for package_id, script in scripts.items():
        if script["run"] is None:
            raise BuildRefused(f"{package_id}'s build script has no run in the -vv output")
    rustc_output = os.path.join(runner["args"]["out_dir"],
                                runner["args"]["crate_name"] + str(runner["args"]["codegen"].get("extra-filename", "")))
    return {
        "rustc": rustc,
        "runtime_units": sorted(f"{u['package']['name']} {u['package']['version']} {u['key'][1]}" for u in runtime.values()),
        "host_units": sorted(f"{u['package']['name']} {u['package']['version']} {u['key'][1]}" for u in host.values()),
        "build_scripts": len(scripts),
        "registry_packages": len(registry_checked),
        "generated": {pid: s["generated"] for pid, s in sorted(scripts.items()) if s["generated"]},
        "runner": {"rustc_output": rustc_output, "executable": runner["artifact"].get("executable")},
    }


def check_settings(what: str, cls: str, block: dict, unit: dict) -> None:
    """A unit's settings, from its JSON profile and its -C options (all but the two that name it), must be
    exactly its class's block; a proc macro also carries the block's proc_macro_codegen."""
    artifact, args = unit["artifact"], unit["args"]
    profile = {key: artifact["profile"].get(key) for key in PROFILE_KEYS}
    if profile != block["profile"] or artifact["profile"].get("test") is not False:
        raise BuildRefused(f"{what} was built with profile {artifact['profile']}, not the {cls} settings {block['profile']}")
    expected = {k: v for k, v in block["codegen"].items() if v is not None}
    if cls == "host" and unit["crate_types"] == ["proc-macro"]:
        expected.update({k: v for k, v in block["proc_macro_codegen"].items() if v is not None})
    actual = {k: v for k, v in args["codegen"].items() if k not in UNIT_ID_KEYS}
    if actual != expected:
        raise BuildRefused(f"{what} was compiled with -C {actual}, not the {cls} settings {expected}")


def build_driver(tree: Path, sha: str, target: Path, cargo: list[str], jobs: int, scratch: Path, side: str,
                 manifest: dict, wrapper: list[str] = ()) -> dict:
    """Builds the driver from `tree` into the fresh target directory `target` and attests the build.
    Returns the build record; raises SetupError (exit 2, before any run) on any refusal."""
    if target.exists() and (not target.is_dir() or any(target.iterdir())):
        raise SetupError(f"target directory {target} exists and is not an empty directory; each build is fresh")
    target.mkdir(parents=True, exist_ok=True)
    tree, target = tree.resolve(), target.resolve()
    binary = target / PROFILE / DRIVER_BIN
    command = [*cargo, "build", "-vv", "--message-format=json", "--release", "--locked", "--manifest-path",
               str(tree / DRIVER_CRATE / "Cargo.toml"), "--target-dir", str(target), "--bin", DRIVER_BIN, "-j", str(jobs)]
    messages, log = scratch / f"build-{side}.jsonl", scratch / f"build-{side}.log"
    started = time.time()
    with open(messages, "wb") as out, open(log, "wb") as err:
        try:
            result = subprocess.run(command, cwd=tree / DRIVER_CRATE, stdout=out, stderr=err, env=clean_env())
        except OSError as error:
            raise SetupError(f"the cargo command {cargo[0]} could not be started: {error}") from error
    if result.returncode in WRAPPER_REFUSALS and cargo and os.path.basename(cargo[0]) != "cargo":
        raise SetupError(f"the cargo wrapper {cargo[0]} refused the build of {sha} (exit {result.returncode}, "
                         "a host condition)")
    if result.returncode != 0:
        raise SetupError(f"the build of {sha} exited {result.returncode}; see {log}")
    if not binary.is_file() or binary.stat().st_mtime < started - 1:
        raise SetupError(f"the build of {sha} did not link {binary}")
    status = worktree_status(tree)
    if status:
        raise SetupError(f"worktree {tree} is not clean after the build:\n{status}")
    try:
        parsed = parse_build_output(messages.read_text(encoding="utf-8"), log.read_text(encoding="utf-8"))
        rustc = rustc_identity(compiler_program(parsed), wrapper)
        lock = tree / DRIVER_CRATE / "Cargo.lock"
        attestation = attest_build(parsed, tree=str(tree), target=str(target), lock_text=lock.read_text(encoding="utf-8"),
                                   settings=manifest["build_settings"], closure=manifest["build_closure"], rustc=rustc)
    except (OSError, UnicodeDecodeError, KeyError, TypeError, AttributeError) as error:
        raise BuildRefused(f"the build of {sha} could not be attested: {type(error).__name__}: {error}") from error
    except BuildRefused as error:
        raise BuildRefused(f"side {side} ({sha}): {error}") from error
    # Binding: what rustc wrote, what cargo uplifted, and (by the caller) the copy that runs.
    runner = attestation["runner"]
    if runner["executable"] is None or Path(runner["executable"]).resolve() != binary:
        raise BuildRefused(f"side {side}: cargo's executable {runner['executable']} is not {binary}")
    output_sha256, binary_sha256 = sha256_file(Path(runner["rustc_output"])), sha256_file(binary)
    if output_sha256 != binary_sha256:
        raise BuildRefused(f"side {side}: the binary {binary} is not the file rustc wrote, {runner['rustc_output']}")
    return {"command": command, "binary": str(binary), "seconds": round(time.time() - started, 1),
            "log": str(log), "log_sha256": sha256_file(log), "messages": str(messages),
            "messages_sha256": sha256_file(messages), "rustc_output_sha256": output_sha256,
            "binary_sha256": binary_sha256, "attestation": attestation}


def check_identity(binary: Path, sha: str, tree_sha: str, wrapper: list[str] = ()) -> dict:
    """The driver's embedded identity (`--build-identity`) must name `sha`, its tree, a clean source
    and the release profile. The binary runs under the run wrapper, like every other run of it."""
    result = run_wrapped(list(wrapper), [str(binary), "--build-identity"], f"{binary} --build-identity")
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
    table_path = table_destination(args.table, scratch, work) if args.table else None
    manifest_path = Path(args.manifest).resolve()
    manifest = load_manifest(manifest_path.read_text(encoding="utf-8"))
    cargo = shlex.split(args.cargo)
    wrapper = shlex.split(args.run_wrapper) if args.run_wrapper else []
    usable_wrapper(cargo, "--cargo")
    usable_wrapper(wrapper, "--run-wrapper")
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
        return _compare_in_worktrees(args, repo, scratch, manifest, manifest_path, cargo, wrapper, sides, started,
                                     table_path)
    finally:
        if not args.keep_worktrees:
            for path in made:
                error = remove_worktree(repo, path)
                if error:
                    print(f"byte_compare: could not remove worktree {path}: {error}", file=sys.stderr)


def _compare_in_worktrees(args, repo, scratch, manifest, manifest_path, cargo, wrapper, sides, started,
                          table_path) -> int:
    failures: list[str] = []
    for side, info in sides.items():
        tree = info["worktree"]
        info["target_version"] = target_version(tree)
        info["coverage"] = coverage(tree, manifest)
        for path in info["coverage"]["unnamed"]:
            failures.append(f"{side}: {path} is under a stated root and not in the manifest")
        for path in info["coverage"]["missing"]:
            failures.append(f"{side}: manifest path {path} does not exist")
    # Build, attest and check both sides before anything runs.
    for side, info in sides.items():
        info["build"] = build_driver(info["worktree"], info["sha"], info["target"], cargo, args.jobs, scratch, side,
                                     manifest, wrapper)
        info["bin_dir"].mkdir(parents=True, exist_ok=True)
        binary = info["bin_dir"] / DRIVER_BIN
        shutil.copy2(info["build"]["binary"], binary)
        info["binary"] = binary
        info["binary_sha256"] = sha256_file(binary)
        if info["binary_sha256"] != info["build"]["binary_sha256"]:
            raise BuildRefused(f"side {side}: the copy that runs, {binary}, is not the binary the build attested")
    versions = {side: info["build"]["attestation"]["rustc"]["version"] for side, info in sides.items()}
    if versions["A"] != versions["B"]:
        raise BuildRefused(f"rustc -vV differs between the sides:\nA:\n{versions['A']}B:\n{versions['B']}")
    for side, info in sides.items():
        info["identity"] = check_identity(info["binary"], info["sha"], info["tree_sha"], wrapper)
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
    if table_path is not None:
        write_table(table_path, table)
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
    out.append("")
    out.append("Build attestation (every unit compiled by this build, from its JSON messages and -vv output):")
    for side, info in sides.items():
        build = info.get("build", {})
        attestation = build.get("attestation", {})
        rustc = attestation.get("rustc", {})
        out.append(f"- {side}: {len(attestation.get('runtime_units', []))} runtime units at the runtime settings, "
                   f"{len(attestation.get('host_units', []))} build-time units at the host settings, "
                   f"{attestation.get('build_scripts', 0)} build scripts, "
                   f"{attestation.get('registry_packages', 0)} registry packages matched to Cargo.lock; "
                   f"rustc output = uplifted binary = copy run ({build.get('rustc_output_sha256', '-')}); "
                   f"-vv log {build.get('log_sha256', '-')}, JSON messages {build.get('messages_sha256', '-')}; "
                   f"compiler {rustc.get('program', '-')}, sysroot {rustc.get('sysroot', '-')} (trusted)")
    version = sides["A"].get("build", {}).get("attestation", {}).get("rustc", {}).get("version", "")
    if version:
        out.append(f"- rustc -vV, identical on both sides: {' | '.join(version.strip().splitlines())}")
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
