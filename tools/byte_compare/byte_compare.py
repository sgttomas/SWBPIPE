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
import stat
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


# ------------------------------------------------------------------ writes (threat (c))
#
# Every path this tool writes is inside a reserved area: a directory the comparison creates itself,
# exclusively, under a resolved parent outside every git work tree, and holds open. Each file and
# directory below it is created exclusively, relative to an open directory descriptor and never by path,
# with no link followed, so neither a link at a destination nor a substituted parent can redirect a
# write. Where a subprocess must be given a path (cargo's target directory, a generated worktree, a run's
# input and envelope), the path is checked to still name the reserved directory immediately before the
# subprocess starts. README "Every sink" lists each write.

DIR_FLAGS = os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW
NEW_FILE_FLAGS = os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW


def open_directory(path: Path) -> int:
    """Opens an absolute, resolved directory one component at a time from the root, following no link."""
    if not path.is_absolute():
        raise SetupError(f"{path} is not an absolute path")
    fd = os.open("/", os.O_RDONLY | os.O_DIRECTORY)
    try:
        for part in path.parts[1:]:
            if part in (".", ".."):
                raise SetupError(f"{path} has a {part!r} component")
            child = os.open(part, DIR_FLAGS, dir_fd=fd)
            os.close(fd)
            fd = child
    except OSError as error:
        os.close(fd)
        raise SetupError(f"{path} is not a directory reached without following a link: {error}") from error
    except SetupError:
        os.close(fd)
        raise
    return fd


def same_directory(path: Path, fd: int) -> bool:
    """Whether the absolute `path`, reopened now one component at a time from the root with no link
    followed (a link at any ancestor or at the directory itself fails), is the directory open as `fd`.
    A directory reached through a link never matches, even one that leads back to the held directory."""
    try:
        reopened = open_directory(path)
    except SetupError:
        return False
    try:
        named, held = os.fstat(reopened), os.fstat(fd)
    finally:
        os.close(reopened)
    return (named.st_dev, named.st_ino) == (held.st_dev, held.st_ino)


def plain_name(name: str, what: str) -> str:
    if not isinstance(name, str) or not name or name in (".", "..") or "/" in name or "\0" in name:
        raise SetupError(f"{what} {name!r} is not a plain file name")
    return name


class Area:
    """A directory this comparison created exclusively and holds open; every write below it goes
    through its descriptor."""

    def __init__(self, path: Path, fd: int, what: str, root: "Area | None" = None):
        self.path, self.fd, self.what = path, fd, what
        self.root = root or self
        if root is None:
            self.members: list[Area] = []
        self.root.members.append(self)

    @classmethod
    def reserve(cls, given: str, what: str, root: "Area | None" = None) -> "Area":
        """Reserves `given` as a new area: its parent must resolve, strictly (a missing component, with
        or without a following "..", refuses), to an existing directory outside every git work tree; the
        area itself must not exist in any form, link included, and is created exclusively."""
        raw = Path(given)
        if not raw.is_absolute():
            raise SetupError(f"{what} must be an absolute path: {given}")
        plain_name(raw.name, what)
        try:
            parent = raw.parent.resolve(strict=True)
        except (OSError, RuntimeError) as error:
            raise SetupError(f"{what} {given}: its parent does not resolve to an existing directory ({error})") from error
        parent_fd = open_directory(parent)
        try:
            if inside_git_work_tree(parent) or not same_directory(parent, parent_fd):
                raise SetupError(f"{what} {given} resolves inside a git work tree ({parent}); "
                                 "it must be outside every work tree")
            try:
                os.mkdir(raw.name, 0o755, dir_fd=parent_fd)
            except FileExistsError as error:
                raise SetupError(f"{what} {parent / raw.name} already exists; each comparison reserves a new one") from error
            fd = os.open(raw.name, DIR_FLAGS, dir_fd=parent_fd)
        finally:
            os.close(parent_fd)
        return cls(parent / raw.name, fd, what, root)

    def directory(self, name: str) -> "Area":
        plain_name(name, f"a directory in {self.path}")
        try:
            os.mkdir(name, 0o755, dir_fd=self.fd)
        except FileExistsError as error:
            raise SetupError(f"{self.path / name} already exists; it was not made by this comparison") from error
        return Area(self.path / name, os.open(name, DIR_FLAGS, dir_fd=self.fd), self.what, self.root)

    def new_file(self, name: str, mode: int = 0o644) -> int:
        plain_name(name, f"a file in {self.path}")
        try:
            return os.open(name, NEW_FILE_FLAGS, mode, dir_fd=self.fd)
        except FileExistsError as error:
            raise SetupError(f"{self.path / name} already exists; it was not made by this comparison, "
                             "and nothing was written to it") from error

    def write(self, name: str, data: bytes, mode: int = 0o644) -> Path:
        with os.fdopen(self.new_file(name, mode), "wb") as handle:
            handle.write(data)
        return self.path / name

    def open_new(self, name: str):
        return os.fdopen(self.new_file(name), "wb")

    def read(self, name: str) -> bytes | None:
        """A file below this area, read without following a link; None when absent."""
        try:
            fd = os.open(name, os.O_RDONLY | os.O_NOFOLLOW, dir_fd=self.fd)
        except FileNotFoundError:
            return None
        except OSError as error:
            raise SetupError(f"{self.path / name} cannot be read as a plain file: {error}") from error
        with os.fdopen(fd, "rb") as handle:
            if not stat.S_ISREG(os.fstat(handle.fileno()).st_mode):
                raise SetupError(f"{self.path / name} is not a regular file")
            return handle.read()

    def identity_of(self, name: str) -> tuple[int, int]:
        """(device, inode) of `name` below this area, opened with no link followed; it must be a regular
        file with a single link, or this refuses."""
        try:
            fd = os.open(plain_name(name, f"a file in {self.path}"), os.O_RDONLY | os.O_NOFOLLOW, dir_fd=self.fd)
        except OSError as error:
            raise SetupError(f"{self.path / name} is no longer the file this comparison reserved ({error}); "
                             "nothing was written to it") from error
        try:
            info = os.fstat(fd)
        finally:
            os.close(fd)
        if not stat.S_ISREG(info.st_mode) or info.st_nlink != 1:
            raise SetupError(f"{self.path / name} was replaced or linked after it was reserved; nothing was written to it")
        return info.st_dev, info.st_ino

    def check_held(self, name: str, held: tuple[int, int]) -> None:
        """Refuses unless `name` is still the regular, singly linked file reserved as `held`."""
        if self.identity_of(name) != tuple(held):
            raise SetupError(f"{self.path / name} was replaced after it was reserved; nothing was written to it")

    def write_held(self, name: str, handle, data: bytes) -> None:
        """Writes a reserved file through its held handle, after checking that its name still names it."""
        try:
            held = os.fstat(handle.fileno())
            self.check_held(name, (held.st_dev, held.st_ino))
            handle.write(data)
        finally:
            handle.close()

    def read_held(self, name: str, held: tuple[int, int]) -> bytes:
        """A reserved file's bytes, read with no link followed, from the file reserved as `held` only."""
        try:
            fd = os.open(plain_name(name, f"a file in {self.path}"), os.O_RDONLY | os.O_NOFOLLOW, dir_fd=self.fd)
        except OSError as error:
            raise SetupError(f"{self.path / name} is no longer the file this comparison reserved ({error})") from error
        with os.fdopen(fd, "rb") as handle:
            info = os.fstat(handle.fileno())
            if not stat.S_ISREG(info.st_mode) or info.st_nlink != 1 or (info.st_dev, info.st_ino) != tuple(held):
                raise SetupError(f"{self.path / name} was replaced after it was reserved; it is not read")
            return handle.read()

    def verify(self) -> None:
        """Refuses unless every area of this reservation is still named by its path (no directory on the
        way replaced, by a link or otherwise)."""
        for area in self.root.members:
            if not same_directory(area.path, area.fd):
                raise SetupError(f"{area.path} no longer names the directory this comparison reserved "
                                 "(replaced or linked elsewhere); nothing more is run or written")

    def close(self) -> None:
        for area in self.root.members:
            try:
                os.close(area.fd)
            except OSError:
                pass


def read_plain(path: Path) -> bytes:
    """A file read with no link followed anywhere on its path (cargo's outputs, read back)."""
    parent = open_directory(path.parent)
    try:
        fd = os.open(path.name, os.O_RDONLY | os.O_NOFOLLOW, dir_fd=parent)
    except OSError as error:
        raise BuildRefused(f"{path} cannot be read as a plain file: {error}") from error
    finally:
        os.close(parent)
    with os.fdopen(fd, "rb") as handle:
        if not stat.S_ISREG(os.fstat(handle.fileno()).st_mode):
            raise BuildRefused(f"{path} is not a regular file")
        return handle.read()


def file_identity(path: Path) -> tuple[int, int] | None:
    """(device, inode) of the regular, singly linked file at the absolute `path`, reached with no link
    followed anywhere on the way; None otherwise."""
    try:
        parent = open_directory(path.parent)
    except SetupError:
        return None
    try:
        info = os.stat(path.name, dir_fd=parent, follow_symlinks=False)
    except OSError:
        return None
    finally:
        os.close(parent)
    if not stat.S_ISREG(info.st_mode) or info.st_nlink != 1:
        return None
    return info.st_dev, info.st_ino


# What the tool itself writes directly in the scratch area; the table may not take one of these names.
SCRATCH_EVIDENCE = frozenset({"plan.json", "results.json", "build-A.log", "build-B.log", "build-A.jsonl",
                              "build-B.jsonl", "inputs", "runs", "build"})


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
    # Each entry's scratch input and run evidence are named by its derived ID; two entries with one ID
    # would share them, so that one request ran in place of the other.
    ids: dict[str, str] = {}
    for entry in entries:
        other = ids.setdefault(entry_id(entry["path"]), entry["path"])
        if other != entry["path"]:
            raise ManifestError(f"{other} and {entry['path']} derive the same entry ID {entry_id(entry['path'])!r}")
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


def make_worktree(repo: Path, area: "Area", sha: str) -> None:
    """Adds a detached worktree of `sha` into `area`, a directory this comparison reserved (empty)."""
    area.verify()
    git(repo, "worktree", "add", "--detach", str(area.path), sha)
    area.verify()
    head = git(area.path, "rev-parse", "HEAD").stdout.strip()
    if head != sha:
        raise SetupError(f"worktree {area.path} is at {head}, not {sha}")
    status = worktree_status(area.path)
    if status:
        raise SetupError(f"worktree {area.path} is not clean before the build:\n{status}")


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
        elif arg.startswith(("--allow=", "--warn=", "--deny=", "--forbid=", "--force-warn=")):
            pass  # lint levels: the packages' [lints] tables, and cargo's own (--force-warn=)
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


def verify_registry_package(home: str, index: str, name: str, version: str, checksum: str) -> dict:
    """The registry package compiled is exactly the one Cargo.lock names: its cached archive's SHA-256
    is the lock's checksum, and the extracted source cargo compiled holds exactly the archive's files
    and bytes (plus cargo's own `.cargo-ok` marker). Verified afresh on every call, so each side's build
    is attested from what was on disk for that side; nothing is remembered between sides. Returns the
    archive's digest, a digest over the extracted files and their digests, and the files."""
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
    return {"archive_sha256": sha256_bytes(data),
            "source_sha256": sha256_bytes(json.dumps(sorted(actual.items())).encode()),
            "root": str(root), "files": set(actual)}


def generated_files(out_dir: str) -> dict:
    """Every file a build script left in its OUT_DIR, with its SHA-256. The OUT_DIR leaf and each walked
    entry must not be a link or other non-regular entry, or this refuses; an ancestor link is not checked
    (README, "Also outside the model")."""
    root = Path(out_dir)
    if root.is_symlink() or not root.is_dir():
        raise BuildRefused(f"the build-script output directory {out_dir} is absent or a link")
    found = {}
    for directory, names, files in os.walk(root, followlinks=False):
        for name in names + files:
            path = Path(directory) / name
            mode = os.lstat(path).st_mode
            if stat.S_ISLNK(mode) or not (stat.S_ISDIR(mode) or stat.S_ISREG(mode)):
                raise BuildRefused(f"the build-script output {path} is a link or not a regular file")
            if stat.S_ISREG(mode):
                found[path.relative_to(root).as_posix()] = sha256_file(path)
    return dict(sorted(found.items()))


def committed_files(tree: Path) -> dict:
    """Each path the side's worktree has committed (the worktree is clean, so its index is the commit),
    with its mode: 100644 or 100755 for a file, 120000 for a link."""
    listing = subprocess.run(["git", "-c", "core.hooksPath=/dev/null", "-C", str(tree), "ls-files", "-s", "-z"],
                             capture_output=True, env=clean_env())
    if listing.returncode != 0:
        raise BuildRefused(f"git ls-files failed in {tree}")
    found = {}
    for record in filter(None, listing.stdout.decode("utf-8", "surrogateescape").split("\0")):
        meta, _, path = record.partition("\t")
        found[path] = meta.split(" ")[0]
    return found


def parse_dep_info(text: str, where: str) -> list[str]:
    """The files rustc reports a unit read, from its dep-info: every dependency of every rule. Comment
    lines other than `# env-dep:` refuse, and a phony line must name a file a rule names."""
    deps, phony = [], []
    for line in text.split("\n"):
        if not line or line.startswith("# env-dep:"):
            continue
        if line.startswith("#"):
            raise BuildRefused(f"{where}: an unknown dep-info line {line[:120]!r}")
        target, sep, rest = line.partition(": ")
        if sep:
            deps.extend(word.replace("\\ ", " ") for word in re.split(r"(?<!\\) ", rest) if word)
        elif line.endswith(":"):
            phony.append(line[:-1].replace("\\ ", " "))
        else:
            raise BuildRefused(f"{where}: an unparseable dep-info line {line[:120]!r}")
    if not deps or not set(phony) <= set(deps):
        raise BuildRefused(f"{where}: the dep-info names no source, or names a file no rule depends on")
    return list(dict.fromkeys(deps))


def link_on_path(path: str, lstat=os.lstat) -> tuple[str, str] | None:
    """Walks the absolute `path` exactly as written, one component at a time and `..` included, without
    following any link: each prefix is examined with lstat, and `..` steps back to the parent of the
    prefix walked so far (which holds no link, so its parent is its lexical parent). Returns the first
    prefix that is a link or does not exist, with which; None when every component is a real entry. A
    link is found even where a later `..` cancels it."""
    current = "/"
    for part in PurePosixPath(path).parts[1:]:
        if part == ".":
            continue
        if part == "..":
            current = os.path.dirname(current)
            continue
        current = os.path.join(current, part)
        try:
            mode = lstat(current).st_mode
        except OSError:
            return current, "which does not exist"
        if stat.S_ISLNK(mode):
            return current, "which is a link"
    return None


def check_source_closure(paths: list[str], *, tree: str, committed: dict, registry: dict, generated: set,
                         find_link=link_on_path) -> None:
    """A list comparison over what rustc reported (README "Source closure"). Every path is first walked
    as written, component by component, with no link followed: a link at any component (even one a later
    `..` cancels) refuses. Each file must then be one of: a committed regular file of this side's
    worktree, a file of a registry package this side verified, or a generated file the manifest allows.
    Anything else refuses. The same operations apply to every reported path."""
    for path in paths:
        if not os.path.isabs(path):
            raise BuildRefused(f"the source {path} is not an absolute path")
        found = find_link(path)
        if found is not None:
            raise BuildRefused(f"the source {path} passes through {found[0]}, {found[1]}; "
                               "a link anywhere on a source's path refuses")
        plain = os.path.normpath(path)
        if plain in generated:
            continue
        if _inside(plain, tree):
            mode = committed.get(os.path.relpath(plain, tree))
            if mode == "120000":
                raise BuildRefused(f"the source {plain} is a committed link; a link in the source closure refuses")
            if mode not in ("100644", "100755"):
                raise BuildRefused(f"the source {plain} is not a committed file of the side's worktree")
            continue
        match = REGISTRY_SRC.match(plain)
        root = os.path.join(match["home"], "registry", "src", match["index"], match["dir"]) if match else None
        if root not in registry or match["rest"] not in registry[root]:
            raise BuildRefused(f"the source {plain} is neither committed, nor a verified registry file, nor an "
                               "allowed generated file")


def attest_build(parsed: dict, *, tree: str, target: str, lock_text: str, settings: dict, closure: dict,
                 rustc: dict, committed: dict, list_generated=generated_files, verify_registry=verify_registry_package,
                 read_dep_info=lambda path: Path(path).read_text(encoding="utf-8"), find_link=link_on_path) -> dict:
    """Attests a parsed build (README "Build attestation"); raises BuildRefused on any departure.

    Runtime profile checks: the runner and every unit it links, followed through --extern, have the
    manifest's runtime settings. Build-time dependency checks: build scripts, proc macros and what they
    link have its host settings. Every unit's source is in the generated worktree or is the exact
    Cargo.lock registry package, verified for this side; every file rustc reports reading is committed,
    verified or an allowed generated file; every flag and environment variable is accounted for."""
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
    registry = {}
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
            if package["id"] not in registry:
                registry[package["id"]] = verify_registry(match["home"], match["index"], package["name"],
                                                          package["version"], checksum)
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
    # The source closure: every file rustc reports each unit read (its dep-info), the path packages'
    # manifests and the driver's lock, compared with this side's committed files, verified registry
    # files and allowed generated files.
    sources = [os.path.join(tree, DRIVER_CRATE, "Cargo.lock")]
    sources += sorted({u["artifact"]["manifest_path"] for u in by_key.values() if u["package"]["kind"] == "path"})
    for unit in by_key.values():
        args, env = unit["args"], unit["invocation"]["env"]
        dep_info = os.path.join(args["out_dir"], args["crate_name"] + str(args["codegen"].get("extra-filename", "")) + ".d")
        try:
            text = read_dep_info(dep_info)
        except (OSError, KeyError) as error:
            raise BuildRefused(f"{unit['package']['id']} ({unit['key'][1]}) left no dep-info {dep_info}") from error
        relative_base = env.get("CARGO_MANIFEST_DIR") if not os.path.isabs(args["sources"][0]) else None
        for path in parse_dep_info(text, dep_info):
            if not os.path.isabs(path):
                if relative_base is None:
                    raise BuildRefused(f"{dep_info} names a relative file {path} for a unit compiled from an absolute path")
                path = os.path.join(relative_base, path)
            sources.append(path)
    generated = {os.path.join(s["message"]["out_dir"], name) for s in scripts.values() for name in s["generated"]}
    check_source_closure(list(dict.fromkeys(sources)), tree=tree, committed=committed,
                         registry={r["root"]: r["files"] for r in registry.values()}, generated=generated,
                         find_link=find_link)
    rustc_output = os.path.join(runner["args"]["out_dir"],
                                runner["args"]["crate_name"] + str(runner["args"]["codegen"].get("extra-filename", "")))
    return {
        "rustc": rustc,
        "runtime_units": sorted(f"{u['package']['name']} {u['package']['version']} {u['key'][1]}" for u in runtime.values()),
        "host_units": sorted(f"{u['package']['name']} {u['package']['version']} {u['key'][1]}" for u in host.values()),
        "build_scripts": len(scripts),
        "registry_packages": len(registry),
        "registry": {pid: {"archive_sha256": r["archive_sha256"], "source_sha256": r["source_sha256"]}
                     for pid, r in sorted(registry.items())},
        "source_closure_files": len(dict.fromkeys(sources)),
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


def build_driver(info: dict, cargo: list[str], jobs: int, scratch: "Area", side: str, manifest: dict,
                 wrapper: list[str] = ()) -> dict:
    """Builds the driver from the side's worktree into its reserved, empty target directory and attests
    the build. Returns the build record; raises SetupError (exit 2, before any run) on any refusal."""
    tree, target, sha = info["worktree"].path, info["target"].path, info["sha"]
    binary = target / PROFILE / DRIVER_BIN
    command = [*cargo, "build", "-vv", "--message-format=json", "--release", "--locked", "--manifest-path",
               str(tree / DRIVER_CRATE / "Cargo.toml"), "--target-dir", str(target), "--bin", DRIVER_BIN, "-j", str(jobs)]
    started = time.time()
    with scratch.open_new(f"build-{side}.jsonl") as out, scratch.open_new(f"build-{side}.log") as err:
        held = {f"build-{side}.jsonl": os.fstat(out.fileno()), f"build-{side}.log": os.fstat(err.fileno())}
        scratch.verify()
        try:
            result = subprocess.run(command, cwd=tree / DRIVER_CRATE, stdout=out, stderr=err, env=clean_env())
        except OSError as error:
            raise SetupError(f"the cargo command {cargo[0]} could not be started: {error}") from error
    scratch.verify()

    def read_log(name):
        # Every read of a build log (the parse reads and the hash reads) is of the file cargo wrote into,
        # by the identity taken from the descriptor that created it, before cargo started.
        original = held[name]
        return scratch.read_held(name, (original.st_dev, original.st_ino))

    messages, log = scratch.path / f"build-{side}.jsonl", scratch.path / f"build-{side}.log"
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
        parsed = parse_build_output(read_log(messages.name).decode("utf-8"), read_log(log.name).decode("utf-8"))
        rustc = rustc_identity(compiler_program(parsed), wrapper)
        attestation = attest_build(parsed, tree=str(tree), target=str(target),
                                   lock_text=read_plain(tree / DRIVER_CRATE / "Cargo.lock").decode("utf-8"),
                                   settings=manifest["build_settings"], closure=manifest["build_closure"], rustc=rustc,
                                   committed=info["committed"],
                                   read_dep_info=lambda path: read_plain(Path(path)).decode("utf-8"))
    except (OSError, UnicodeDecodeError, KeyError, TypeError, AttributeError) as error:
        raise BuildRefused(f"the build of {sha} could not be attested: {type(error).__name__}: {error}") from error
    except BuildRefused as error:
        raise BuildRefused(f"side {side} ({sha}): {error}") from error
    # Binding: what rustc wrote, what cargo uplifted, and (by the caller) the copy that runs.
    runner = attestation["runner"]
    if runner["executable"] is None or Path(runner["executable"]) != binary:
        raise BuildRefused(f"side {side}: cargo's executable {runner['executable']} is not {binary}")
    data = read_plain(binary)
    output_sha256, binary_sha256 = sha256_bytes(read_plain(Path(runner["rustc_output"]))), sha256_bytes(data)
    if output_sha256 != binary_sha256:
        raise BuildRefused(f"side {side}: the binary {binary} is not the file rustc wrote, {runner['rustc_output']}")
    return {"command": command, "binary": str(binary), "seconds": round(time.time() - started, 1),
            "log": str(log), "log_sha256": sha256_bytes(read_log(log.name)), "messages": str(messages),
            "messages_sha256": sha256_bytes(read_log(messages.name)), "rustc_output_sha256": output_sha256,
            "binary_sha256": binary_sha256, "attestation": attestation, "data": data}


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

RUN_OUTPUTS = (".stdout", ".stderr", ".status.json")


def run_batch(plan: dict) -> int:
    """Executes a plan's runs one at a time, keeping stdout, stderr, exit status and envelope. It reopens
    the scratch area from the root with no link followed and requires it to be the directory the plan
    names. Every run's outputs were reserved by the comparison before the batch; each is reopened with no
    link followed and must be the reserved file with a single link, and no envelope may exist yet, before
    the first run starts. Before each run it checks that the area and the run's directory are still the
    held ones, and that the run's input and binary are still the files the comparison wrote."""
    root = Path(plan["root"])
    fd = open_directory(root)
    dirs: dict[str, int] = {}
    try:
        held = os.fstat(fd)
        if [held.st_dev, held.st_ino] != plan["root_id"]:
            raise SetupError(f"{root} is not the scratch area the comparison reserved")
        outputs = {key: tuple(value) for key, value in plan["outputs"].items()}

        def reopen(run: dict, suffix: str, flags: int) -> int:
            name = run["name"] + suffix
            try:
                out = os.open(name, flags | os.O_NOFOLLOW, dir_fd=dirs[run["dir"]])
            except OSError as error:
                raise SetupError(f"{root / run['dir'] / name} is not the reserved output ({error})") from error
            info = os.fstat(out)
            if not stat.S_ISREG(info.st_mode) or info.st_nlink != 1 \
                    or (info.st_dev, info.st_ino) != outputs[f"{run['dir']}/{name}"]:
                os.close(out)
                raise SetupError(f"{root / run['dir'] / name} was replaced after it was reserved")
            return out

        for run in plan["runs"]:
            if run["dir"] not in dirs:
                parent = fd
                for part in Path(run["dir"]).parts:
                    child = os.open(plain_name(part, "a run directory"), DIR_FLAGS, dir_fd=parent)
                    if parent != fd:
                        os.close(parent)
                    parent = child
                dirs[run["dir"]] = parent
            for suffix in RUN_OUTPUTS:
                os.close(reopen(run, suffix, os.O_RDONLY))
            try:
                os.stat(run["name"] + ".envelope.json", dir_fd=dirs[run["dir"]], follow_symlinks=False)
                raise SetupError(f"{root / run['dir'] / (run['name'] + '.envelope.json')} already exists; nothing was run")
            except FileNotFoundError:
                pass

        for run in plan["runs"]:
            if not same_directory(root, fd) or not same_directory(root / run["dir"], dirs[run["dir"]]):
                raise SetupError(f"{root / run['dir']} no longer names the directory reserved for the runs")
            for key in ("input", "binary"):
                if file_identity(Path(run[key])) != tuple(plan["files"][run[key]]):
                    raise SetupError(f"{run[key]} is not the {key} the comparison wrote; nothing more is run")
            envelope = root / run["dir"] / (run["name"] + ".envelope.json")
            started = time.monotonic()
            with os.fdopen(reopen(run, ".stdout", os.O_WRONLY), "wb") as out, \
                    os.fdopen(reopen(run, ".stderr", os.O_WRONLY), "wb") as err:
                process = subprocess.Popen([run["binary"], run["input"], "--mode", run["mode"], "--write", str(envelope)],
                                           stdout=out, stderr=err, cwd=str(root), env=clean_env())
                try:
                    code, timed_out = process.wait(timeout=plan["timeout"]), False
                except subprocess.TimeoutExpired:
                    process.kill()
                    code, timed_out = process.wait(), True
            status = {"exit": code, "timed_out": timed_out, "seconds": round(time.monotonic() - started, 3)}
            with os.fdopen(reopen(run, ".status.json", os.O_WRONLY), "wb") as handle:
                handle.write((json.dumps(status) + "\n").encode())
    finally:
        for where in dirs.values():
            os.close(where)
        os.close(fd)
    return 0


def execute_plan(plan: dict, plan_handle, scratch: "Area", wrapper: list[str]) -> None:
    """Writes the plan into its reserved file (checked to be still the reserved file) and runs the batch,
    under the run wrapper when given."""
    held = os.fstat(plan_handle.fileno())
    scratch.write_held("plan.json", plan_handle, (json.dumps(plan, indent=1) + "\n").encode())
    scratch.verify()
    if not wrapper:
        run_batch(plan)
        return
    scratch.check_held("plan.json", (held.st_dev, held.st_ino))   # the batch reads it by name
    command = [*wrapper, sys.executable, "-I", str(Path(__file__).resolve()), "run-batch", str(scratch.path / "plan.json")]
    result = subprocess.run(command, env=clean_env())
    if result.returncode != 0:
        raise SetupError(f"the run batch exited {result.returncode} under {wrapper[0]}"
                         + (" (a host condition: the slot wrapper refused)" if result.returncode in (97, 98) else ""))


def read_run(runs: "Area", name: str, outputs: dict) -> tuple[dict, str, bytes | None]:
    """A run's saved status, stdout and envelope, read below the held runs directory with no link
    followed. The status and stdout must be the files reserved for this run. The envelope is the driver's
    own file, and counts only where its digest and length are the ones the run's stdout report gives."""
    prefix = runs.path.relative_to(runs.root.path).as_posix()
    status_bytes = runs.read_held(name + ".status.json", outputs[f"{prefix}/{name}.status.json"])
    status = json.loads(status_bytes) if status_bytes else {"exit": None, "missing": True}
    stdout = runs.read_held(name + ".stdout", outputs[f"{prefix}/{name}.stdout"]).decode("utf-8", "replace")
    return status, stdout, runs.read(name + ".envelope.json")


# ------------------------------------------------------------------ the comparison

def compare(args: argparse.Namespace) -> int:
    started = time.monotonic()
    repo = Path(git(Path(args.repo).resolve(), "rev-parse", "--show-toplevel").stdout.strip())
    manifest_path = Path(args.manifest).resolve()
    manifest = load_manifest(manifest_path.read_text(encoding="utf-8"))
    cargo = shlex.split(args.cargo)
    wrapper = shlex.split(args.run_wrapper) if args.run_wrapper else []
    usable_wrapper(cargo, "--cargo")
    usable_wrapper(wrapper, "--run-wrapper")
    if args.table is not None and plain_name(args.table, "--table") in SCRATCH_EVIDENCE:
        raise SetupError(f"--table {args.table} is one of the tool's own evidence names")
    sides = {}
    for side, rev in (("A", args.a), ("B", args.b)):
        sha = resolve_commit(repo, rev)
        sides[side] = {"rev": rev, "sha": sha, "tree_sha": git(repo, "rev-parse", f"{sha}^{{tree}}").stdout.strip()}
    # The reservation: the scratch area (and a separate work area, if given) is created here, fresh, and
    # every directory and fixed file the comparison will write is reserved in it before anything runs.
    scratch = Area.reserve(args.scratch, "--scratch")
    made, reserved = [], {}
    try:
        work = Area.reserve(args.work, "--work", root=scratch) if args.work else scratch.directory("build")
        for name in ("plan.json", "results.json", *([args.table] if args.table else [])):
            reserved[name] = scratch.open_new(name)
        inputs, runs = scratch.directory("inputs"), scratch.directory("runs")
        targets, bins = work.directory("target"), work.directory("bin")
        for side, info in sides.items():
            sha = info["sha"]
            info.update(worktree=work.directory(f"wt-{side}-{sha[:12]}"), target=targets.directory(f"{side}-{sha}"),
                        bin_dir=bins.directory(f"{side}-{sha}"), inputs=inputs.directory(side), runs=runs.directory(side))
        for side, info in sides.items():
            make_worktree(repo, info["worktree"], info["sha"])
            made.append(info["worktree"].path)
        return _compare_in_worktrees(args, repo, scratch, manifest, manifest_path, cargo, wrapper, sides, started,
                                     reserved)
    finally:
        for handle in reserved.values():
            handle.close()
        if not args.keep_worktrees:
            for path in made:
                error = remove_worktree(repo, path)
                if error:
                    print(f"byte_compare: could not remove worktree {path}: {error}", file=sys.stderr)
        scratch.close()


def _compare_in_worktrees(args, repo, scratch, manifest, manifest_path, cargo, wrapper, sides, started,
                          reserved) -> int:
    failures: list[str] = []
    for side, info in sides.items():
        tree = info["worktree"].path
        info["committed"] = committed_files(tree)
        info["target_version"] = target_version(tree)
        info["coverage"] = coverage(tree, manifest)
        for path in info["coverage"]["unnamed"]:
            failures.append(f"{side}: {path} is under a stated root and not in the manifest")
        for path in info["coverage"]["missing"]:
            failures.append(f"{side}: manifest path {path} does not exist")
    # Build, attest and check both sides before anything runs.
    for side, info in sides.items():
        info["build"] = build_driver(info, cargo, args.jobs, scratch, side, manifest, wrapper)
        data = info["build"].pop("data")
        with os.fdopen(info["bin_dir"].new_file(DRIVER_BIN, 0o755), "wb") as handle:
            handle.write(data)
            created = os.fstat(handle.fileno())
            info["binary_id"] = (created.st_dev, created.st_ino)
        binary = info["bin_dir"].path / DRIVER_BIN
        info["binary"] = binary
        info["binary_sha256"] = sha256_bytes(info["bin_dir"].read_held(DRIVER_BIN, info["binary_id"]))
        if info["binary_sha256"] != info["build"]["binary_sha256"]:
            raise BuildRefused(f"side {side}: the copy that runs, {binary}, is not the binary the build attested")
    versions = {side: info["build"]["attestation"]["rustc"]["version"] for side, info in sides.items()}
    if versions["A"] != versions["B"]:
        raise BuildRefused(f"rustc -vV differs between the sides:\nA:\n{versions['A']}B:\n{versions['B']}")
    for side, info in sides.items():
        scratch.verify()
        info["bin_dir"].check_held(DRIVER_BIN, info["binary_id"])
        info["identity"] = check_identity(info["binary"], info["sha"], info["tree_sha"], wrapper)
    # Inputs: the lift applied to temporary copies under the scratch directory only.
    records = {}
    runs, outputs, files = [], {}, {}
    for entry in manifest["entries"]:
        record = records[entry["path"]] = {"entry": entry, "sides": {}}
        for side, info in sides.items():
            source = info["worktree"].path / entry["path"]
            facts = {"expected": entry["expect"][side]["disposition"], "source": str(source)}
            record["sides"][side] = facts
            if info["committed"].get(entry["path"]) not in ("100644", "100755") or \
                    link_on_path(str(source)) is not None:
                facts.update(pre_class=MISSING, pre_detail="not a committed regular file reached without a link")
                continue
            data = read_plain(source)
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
            temp = info["inputs"].path / f"{entry_id(entry['path'])}.json"
            with os.fdopen(info["inputs"].new_file(temp.name), "wb") as handle:
                handle.write(serialize_request(lifted["request"]))
                created = os.fstat(handle.fileno())
            facts["input"] = str(temp)
            files[str(temp)] = (created.st_dev, created.st_ino)
            files[str(info["binary"])] = info["binary_id"]
            for mode in MODES:
                name = f"{entry_id(entry['path'])}.{mode}"
                # Every output of the run is reserved here, by the comparison, before the batch starts.
                for suffix in RUN_OUTPUTS:
                    reserved_fd = info["runs"].new_file(name + suffix)
                    created = os.fstat(reserved_fd)
                    os.close(reserved_fd)
                    outputs[f"runs/{side}/{name}{suffix}"] = (created.st_dev, created.st_ino)
                runs.append({"binary": str(info["binary"]), "input": str(temp), "mode": mode, "dir": f"runs/{side}",
                             "name": name})
    held = os.fstat(scratch.fd)
    plan = {"root": str(scratch.path), "root_id": [held.st_dev, held.st_ino], "timeout": args.timeout, "runs": runs,
            "outputs": outputs, "files": files}
    execute_plan(plan, reserved.pop("plan.json"), scratch, wrapper)
    scratch.verify()
    # Integrity after the runs: binaries, sources and worktrees as they were.
    for side, info in sides.items():
        if sha256_bytes(info["bin_dir"].read_held(DRIVER_BIN, info["binary_id"])) != info["binary_sha256"]:
            failures.append(f"{side}: the binary changed during the runs")
        status = worktree_status(info["worktree"].path)
        if status:
            failures.append(f"{side}: the worktree changed during the runs:\n{status}")
    for record in records.values():
        for side, facts in record["sides"].items():
            source = Path(facts["source"])
            if "input_sha256" in facts and (not source.is_file() or source.is_symlink()
                                            or sha256_file(source) != facts["input_sha256"]):
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
                    status, stdout, envelope = read_run(sides[side]["runs"], f"{entry_id(path)}.{mode}", outputs)
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
               "sides": {side: {k: (str(v.path) if isinstance(v, Area) else str(v) if isinstance(v, Path) else v)
                                for k, v in info.items() if k != "committed"}
                         for side, info in sides.items()},
               "pairs": pairs, "failures": failures}
    scratch.verify()
    scratch.write_held("results.json", reserved.pop("results.json"),
                       (json.dumps(results, indent=1, default=str) + "\n").encode())
    if args.table:
        scratch.write_held(args.table, reserved.pop(args.table), table.encode())
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
    run.add_argument("--scratch", required=True,
                     help="a new directory for raw outputs, created by the tool, whose parent exists outside every git work tree")
    run.add_argument("--work", help="a new directory for the worktrees, target directories and binaries, created as "
                                    "--scratch is (default <scratch>/build)")
    run.add_argument("--manifest", default=str(DEFAULT_MANIFEST))
    run.add_argument("--repo", default=str(TOOL_DIR), help="any path in the repository whose revisions are compared")
    run.add_argument("--cargo", default="cargo", help="the cargo command, e.g. a host wrapper (shell-split)")
    run.add_argument("--jobs", type=int, default=4, help="cargo build jobs (default 4)")
    run.add_argument("--run-wrapper", default="", help="command prefix for the run batch, e.g. a host slot wrapper")
    run.add_argument("--timeout", type=float, default=1800.0, help="seconds per driver run (default 1800)")
    run.add_argument("--table", help="also write the table to this new file name in the scratch area, instead of stdout")
    run.add_argument("--keep-worktrees", action="store_true", help="leave the worktrees for inspection")
    batch = sub.add_parser("run-batch", help=argparse.SUPPRESS)
    batch.add_argument("plan")
    args = parser.parse_args(argv)
    if args.command == "run-batch":
        try:
            return run_batch(json.loads(Path(args.plan).read_text()))
        except SetupError as error:
            print(f"byte_compare run-batch: {error}", file=sys.stderr)
            return EXIT_SETUP
    try:
        return compare(args)
    except SetupError as error:
        print(f"byte_compare: {error}", file=sys.stderr)
        return EXIT_SETUP


if __name__ == "__main__":
    sys.exit(main())
