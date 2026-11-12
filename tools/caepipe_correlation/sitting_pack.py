"""The CAEPIPE sitting pack (DEL-17-05): prepare a pack a person can run, take in what comes back, seal captures.

From the project root:

    python3 -m tools.caepipe_correlation.sitting_pack prepare OUT_DIR [--authority DIR] [--manifest PATH]
        [--plan PATH] [--observations PATH] [--samples PATH] [--case ID ...]
    python3 -m tools.caepipe_correlation.sitting_pack ingest PACK_DIR --authority DIR [--item ID ...] [--check]

PREPARE reserves OUT_DIR (it must not exist; it is created, never reused) outside every Git work tree and writes a
self-contained pack into it, every file created exclusively. It also reserves the authority folder (default: the
sibling `<OUT_DIR>.authority`), which holds the host's own copy of the pack index and its SHA-256; it stays on the
host. The pack follows a plan (`--plan`, default `sitting_plan_default.json` beside this module): an ordered list of
parts, each with an id, a title, what it buys, an estimate of the person's time (printed as an estimate, not a
condition), a kind (`dry_run`, `benchmark`, `sealed_capture` or `observations`), its cases selected from the
benchmark manifest by model id, family, series, model member or all, its format samples and its variants. A case is
placed in the first part that selects it; a part that selects nothing is listed empty with its reason. Within a
part the order is by family, the members of one series next to each other, smaller models first. Per case the pack
holds the MBF exactly as the writer produces it for the offline screening (`run_pilot.render_benchmark_mbf`), its
identity file `<stem>.swbpipe-ids.json`, the writer's report (loss report, diagnostics, assumptions), `case.json`
(the case's index entry) and a run line to fill in. A variant is either an instruction (the same MBF, one stated
option changed by hand, the result saved under its own name) or a substitution: the one data line of an options
record named on an allow-list (`OPPRES`, `OPMISC`) replaced, giving a distinct MBF with its own SHA-256. The code
admits no other line, so a substitution changes no node, element, support, material, section or load line, and the
base case's identity file binds its rows; it has no identity file of its own. A format sample is a small invented,
hand-authored model file declared in the maintained format-sample declaration
(`validation/caepipe_correlation/format_samples/declaration.json`), bound there to its SHA-256, with an invented
provenance sidecar; prepare refuses a sample that is not declared, whose file hash differs, whose id is a benchmark
model's, or whose file equals the writer's MBF or the model file of any benchmark model. It is never screened or
compared. Observation items come from `--observations` and are copied in rank order. The pack also holds
`pack_index.json`, the run sheet `RUN_SHEET.md` generated from it, the instructions `INSTRUCTIONS.md`, the guest
script and a one-case driver. The same inputs give byte-identical files. A case the writer refuses is listed in
the index with its refusal codes.

INGEST takes a returned pack into the exchange folder named by SWBPIPE_CAEPIPE_EXCHANGE, resolved and checked as the
harness checks it. It needs the authority folder: the returned `pack_index.json` must hash equal to the
authoritative copy, every path and name in the index is validated, and the kind, capture status and destination of
every item come from the authoritative index, never from the returned folder or the command line. The exchange
folder is the owner's record and only grows: every directory this module adds is reserved exclusively and every file
created exclusively, each relative to directory handles opened without following links, with no `.git` on the way
(no write lands in a nested repository); a write onto an existing path is refused by name. Nothing there is
cleared, replaced, truncated, renamed or deleted, and no staging copy is made. Every intake is an attempt in a new
folder of its own; its record is written last, so a folder without its record is a partial attempt, which a later
intake leaves in place and names. Returned files are stored in a `payload/` or `returned/` subfolder; this module's
records sit beside it, never inside, so a returned file can never be taken for a record. For each item:

- an ordinary benchmark case: a new run folder `cases/<id>/sitting_runs/<UTC time>-<hash>/` holding every returned
  file under `returned/` (the MBF, its identity file and the writer's report included), `caepipe_in/model.mbf`,
  `caepipe_out/model.csv` (or `model.txt`) with `run_record.json`, and the intake record `sitting_intake.json`; a new
  file `cases/<id>/run_selection/<run>.json` names the run. The run record names the writer profile and carries each
  open question it carries. A result the guest script produced is bound as the harness binds one (host and
  guest-reported MBF hashes); a result obtained by hand is recorded under its own route with the binding class
  `person_attested`, which `run_pilot.bound_csv` does not admit. No existing reader consumes `run_selection/`: a
  result taken in at a sitting is stored and cannot be screened until a later change makes the offline screen read
  a selected run and write to new paths.
- a sealed capture (designated in the authoritative index): copied under the names it came back with to
  `cases/<id>/sealed/<attempt>/payload/`, each file read once by the copy that hashes it, with
  `capture_record.json` beside the payload (name, size from the file system, SHA-256 of each file; the model and
  MBF hashes; the complete request's hash). Nothing reads its content beyond that pass. An item that is sealed in
  the exchange folder (by id, or by the MBF hash of any capture record) never reaches a parser or a guest-report
  reader on any route. There is no unsealing command. Every route that parses or screens a stored result refuses a
  sealed capture before any read other than hashing: in this module ordinary intake, `--check`, `classify_case` and
  `check_case`; in run_pilot the live and offline benchmark runs and the pilot's case runs (`sealed_capture`).
  Unsealing is a later change with its own review.
- a format sample or an observation item: stored and hashed in one pass under `format_samples/<id>/<attempt>/payload/`
  or `observations/<id>/<attempt>/payload/`, with its record beside the payload (a sample's record keeps its
  declaration and model file hash); never parsed, recognised or interpreted.

`ingest --check` writes nothing and, like ingest, needs the exchange folder, which it only reads to recognise a sealed
case: it verifies hashes and, for an ordinary case that is not sealed, names its route and
binding and runs the parser on the returned print in place. Standard output carries operational status only: per item
its id, kind, status and route, and for a checked or placed print the number of sections the parser read and of its
error diagnostics. No result value is printed.

Two limits. Storing results append-only does not by itself enforce the sealed-capture reading restrictions: those are
the separate refusals this module and run_pilot make on every screening and parsing route, listed above. And the hand
route is an acceptable fallback for the sitting only because it keeps the same case identities and capture bindings
the harness would: the case id and designation come from the authoritative index, and the result is bound to the
model file's hash the index records; a hand-route intake lands under that case id, bound to that hash, with that
designation, for an ordinary case and a sealed one alike.
"""

from __future__ import annotations

import argparse
from datetime import datetime, timezone
import errno
import hashlib
import json
import os
from pathlib import Path
import re
import sys
from typing import Any, Callable, Iterable, Mapping


PROJECT = Path(__file__).resolve().parents[2]
if str(PROJECT) not in sys.path:
    sys.path.insert(0, str(PROJECT))

from core.handoff.caepipe_external import HarnessSkip, canonical_model_hash, resolve_harness_config  # noqa: E402
from core.handoff.caepipe_external import harness as _harness  # noqa: E402
from core.handoff.caepipe_external.harness import (  # noqa: E402
    ACKNOWLEDGEMENT_KEY,
    BATCH_COMMAND_SHAPE,
    BOUNDARY_NOTE,
    CASE_ID_PATTERN,
    CONFIG_NAME,
    CSV_ROUTES,
    ENV_VAR,
    GUEST_SCRIPT,
    GUEST_SCRIPT_REPOSITORY_PATH,
    RUN_RECORD_KIND,
    ContainmentError,
    last_json_object,
)
from core.handoff.caepipe_mbf.si_writer import MBF_SI_PROFILE_ID, STANDARD_GRAVITY  # noqa: E402
from tools.caepipe_correlation import run_pilot  # noqa: E402


PACK_VERSION = "2"
GENERATOR = "tools/caepipe_correlation/sitting_pack.py"
DEFAULT_MANIFEST = PROJECT / "validation" / "caepipe_benchmark" / "manifest.json"
DEFAULT_PLAN = Path(__file__).resolve().with_name("sitting_plan_default.json")
DEFAULT_SAMPLES = PROJECT / "validation" / "caepipe_correlation" / "format_samples" / "declaration.json"
DRIVER = Path(__file__).resolve().parent / "sitting" / "Run-SittingCase.ps1"
DRIVER_REPOSITORY_PATH = "tools/caepipe_correlation/sitting/Run-SittingCase.ps1"

PLAN_KIND = "swbpipe.caepipe_sitting.plan"
OBSERVATIONS_KIND = "swbpipe.caepipe_sitting.observations"
SAMPLES_KIND = "swbpipe.caepipe_sitting.format_sample_declaration"
INDEX_KIND = "swbpipe.caepipe_sitting.pack_index"
AUTHORITY_INDEX = "pack_index.json"
AUTHORITY_DIGEST = "pack_index.sha256"
# Private records, written only in the exchange folder.
INTAKE_KIND = "swbpipe.caepipe_sitting.intake"
CAPTURE_KIND = "swbpipe.caepipe_sitting.sealed_capture"
STORED_KIND = "swbpipe.caepipe_sitting.stored_output"
SELECTION_KIND = "swbpipe.caepipe_sitting.run_selection"
PRIVATE_RECORD_KINDS = (INTAKE_KIND, CAPTURE_KIND, STORED_KIND, SELECTION_KIND)
INTAKE_RECORD = "sitting_intake.json"
CAPTURE_RECORD = "capture_record.json"
STORED_RECORD = "stored_output.json"
PRIVATE_RECORD_NAMES = (INTAKE_RECORD, CAPTURE_RECORD, STORED_RECORD)
SITTING_RUNS = "sitting_runs"
RUN_SELECTION = "run_selection"
SEALED = "sealed"
PAYLOAD = "payload"
RETURNED = "returned"

ITEM_KINDS = ("benchmark_case", "format_sample", "format_observation")
CAPTURES = {"benchmark_case": ("screened", "sealed"), "format_sample": ("stored_only",),
            "format_observation": ("stored_only",)}
FOLDER_PREFIX = {"benchmark_case": "cases", "format_sample": "samples", "format_observation": "observations"}
PART_KINDS = ("dry_run", "benchmark", "sealed_capture", "observations")
SELECT_KEYS = ("model_ids", "families", "series", "model_members", "all")
VARIANT_KINDS = ("instruction", "substitution")
# The options records whose one data line a substitution variant may replace (Format of .MBF): the pressure options
# and the miscellaneous (dynamics) options. Nothing else is substitutable.
SUBSTITUTABLE_HEADINGS = ("OPPRES", "OPMISC")
ROUTES = ("guest_script", "manual_batch", "manual_print")
SITTING_ROUTES = {"manual_batch": "sitting_manual_batch", "manual_print": "sitting_manual_print"}
GUEST_BINDING = "guest_reported_hash"
ATTESTED_BINDING = "person_attested"
# The open questions the writer profile carries, as DEL-17-04's scope lists them ("Not decided"), and the one this
# run itself carries (DEL-17-05 REQ-003: the invocation profile; the pilot's documented batch pattern is used). The
# writer keeps no list of its own; this one is kept in step with those scopes by hand.
WRITER_PROFILE_OPEN_QUESTIONS = ("TBD-17-01-001", "TBD-17-01-002", "TBD-17-01-003", "TBD-17-04-005", "TBD-17-04-006",
                                 "TBD-17-04-007", "TBD-17-04-008", "TBD-17-04-009", "TBD-17-04-010")
RUN_OPEN_QUESTIONS = ("TBD-17-01-004",)
# The print sections the parser reads (core/handoff/caepipe_external/print_csv.py `_SPECS`), as printed.
PRINT_SECTIONS = (
    "<case>: Displacements",
    "Loads on Anchors in Global Coordinates: <case>",
    "Loads on Anchors in Local Coordinates: <case>",
    "Element forces in global coordinates: <case>",
    "<case>: Loads on restraints",
    "<case>: Pipe element forces (local)",
    "Min/Max Displacements: <case>",
)
# DEL-17-01 Source_Basis_Register.md: the vendor's public pages the instructions rest on.
SOURCES = {
    "CAEPIPE-BATCH": "https://docs.sstcae.com/users_manual/running_caepipe_caepipe_3d__in_batch_mode.htm",
    "CAEPIPE-IMPORT-MBF": "https://docs.sstcae.com/users_manual/import_mbf.htm",
    "CAEPIPE-PRINT-TO-FILE": "https://docs.sstcae.com/users_manual/print__ctrl_p__2.htm",
}
# run_pilot's own help text for the live benchmark options (tests check each against run_pilot.py).
HARNESS_OPTION_HELP = (
    ("--benchmark MANIFEST", "run the generated benchmark models of this manifest (two-way comparison, guarded "
                             "round trips, paced one at a time)"),
    ("--case ID", "run only this case (repeatable)"),
    ("--batch N", "with --benchmark, run only the N-th batch"),
    ("--freeze-file PATH", "with a live --benchmark run (required): a host file whose presence stops the run"),
    ("--exclusive-file PATH", "with a live --benchmark run (required): a host file naming the process id of a "
                              "pending exclusive job, which holds the run"),
    ("--pace-s S", "with a live --benchmark run, seconds between models (default 60)"),
)
# A name this module writes, reads or takes from an index: letters, digits, dot, hyphen and underscore, not starting
# with a dot (so never `.` or `..`), no separator.
NAME_PATTERN = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._-]{0,127}\Z")
_CHUNK = 1 << 20
_DIR_FLAGS = os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW
_NEW_FILE_FLAGS = os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW
_READ_FLAGS = os.O_RDONLY | os.O_NOFOLLOW


class PackError(ValueError):
    """The plan, a declaration, the manifest, an index or a folder cannot be used; the message says why."""


class ExistingPath(PackError):
    """A write would land on a path that already exists; nothing there is replaced."""


# -- small helpers ----------------------------------------------------------------------------------------------------
def _sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def stream_sha256(path: Path) -> tuple[int, str]:
    """(size, SHA-256) of a regular file, not following a link: the size from the file system, the hash in one
    streaming pass; the bytes are never decoded."""
    fd = os.open(path, _READ_FLAGS)
    try:
        digest = hashlib.sha256()
        while chunk := os.read(fd, _CHUNK):
            digest.update(chunk)
        return os.fstat(fd).st_size, digest.hexdigest()
    finally:
        os.close(fd)


def _json_bytes(value: Any) -> bytes:
    return (json.dumps(value, indent=2, sort_keys=True, ensure_ascii=True) + "\n").encode("ascii")


def _crlf(text: str) -> bytes:
    return text.replace("\r\n", "\n").replace("\n", "\r\n").encode("utf-8")


def _now() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


def check_name(name: Any, where: str) -> str:
    if not isinstance(name, str) or not NAME_PATTERN.match(name):
        raise PackError(f"{where}: {name!r} is not a plain name (letters, digits, . _ -; no separator, not "
                        "starting with a dot)")
    return name


def outside_work_trees(path: Path, what: str) -> Path:
    """`path` made absolute, refused when it (or its nearest existing ancestor) lies inside a Git work tree or Git
    directory, by the harness's own check."""
    path = Path(os.path.abspath(path))
    probe = path
    while not probe.exists():
        if probe.parent == probe:
            break
        probe = probe.parent
    inside = _harness._inside_git(probe, os.environ)
    if inside is None:
        raise PackError(f"{what}: git is not available to confirm {path} is outside every Git work tree")
    if inside:
        raise PackError(f"{what}: {path} lies inside a Git work tree or Git directory; the pack and the results "
                        "that come back into it must stay outside every repository")
    return path


# -- create-only writes below one base directory ----------------------------------------------------------------------
class Tree:
    """Reads and create-only writes below one base directory, each relative to directory handles.

    The base must lie outside every Git work tree. Every component on the way to a path is opened with O_NOFOLLOW and
    O_DIRECTORY relative to its parent's handle, so a component that is (or becomes) a symbolic link is refused rather
    than followed, and a parent swapped between two operations cannot redirect a write. A directory on the way that
    holds `.git` (a nested repository or work tree) is refused. A new directory is reserved with `mkdir` (which fails
    when it exists) and a file is created with O_CREAT | O_EXCL | O_NOFOLLOW: an existing path is refused by name and
    left untouched. Container directories (`cases`, `cases/<id>`, `sitting_runs`, ...) are created when absent and
    otherwise entered under the same rules."""

    def __init__(self, base: Path, what: str):
        self.base = outside_work_trees(Path(base), what)
        self.what = what
        try:
            self.fd = os.open(self.base, _DIR_FLAGS)
        except OSError as error:
            raise ContainmentError(f"{what}: {self.base} is not a directory that can be entered without following "
                                   f"a link ({error.strerror})") from None
        self._refuse_git(self.fd, "")

    def close(self) -> None:
        os.close(self.fd)

    def __enter__(self) -> "Tree":
        return self

    def __exit__(self, *exc: Any) -> None:
        self.close()

    def _refuse_git(self, fd: int, where: str) -> None:
        try:
            os.stat(".git", dir_fd=fd, follow_symlinks=False)
        except FileNotFoundError:
            return
        raise ContainmentError(f"{self.what}: {where or '.'} holds .git, a repository or work tree; nothing is "
                               "written inside a Git work tree")

    def _open_dir(self, parts: Iterable[str], *, create: bool) -> int:
        fd = os.dup(self.fd)
        walked: list[str] = []
        try:
            for part in parts:
                check_name(part, f"{self.what} path component")
                walked.append(part)
                try:
                    child = os.open(part, _DIR_FLAGS, dir_fd=fd)
                except FileNotFoundError:
                    if not create:
                        raise
                    try:
                        os.mkdir(part, 0o700, dir_fd=fd)
                    except FileExistsError:
                        pass
                    child = os.open(part, _DIR_FLAGS, dir_fd=fd)
                except OSError as error:
                    if error.errno in (errno.ELOOP, errno.ENOTDIR, errno.EMLINK):
                        raise ContainmentError(f"{self.what}: {'/'.join(walked)} is a symbolic link or not a "
                                               "directory; refused") from None
                    raise
                os.close(fd)
                fd = child
                self._refuse_git(fd, "/".join(walked))
            return fd
        except BaseException:
            os.close(fd)
            raise

    def exists(self, parts: tuple[str, ...]) -> bool:
        try:
            fd = self._open_dir(parts[:-1], create=False)
        except (FileNotFoundError, NotADirectoryError):
            return False
        try:
            os.stat(parts[-1], dir_fd=fd, follow_symlinks=False)
            return True
        except FileNotFoundError:
            return False
        finally:
            os.close(fd)

    def listdir(self, parts: tuple[str, ...]) -> list[str]:
        try:
            fd = self._open_dir(parts, create=False)
        except FileNotFoundError:
            return []
        try:
            return sorted(os.listdir(fd))
        finally:
            os.close(fd)

    def read_json(self, parts: tuple[str, ...]) -> Any:
        """One of this module's own records (never a payload file), or None when absent or unreadable."""
        try:
            parent = self._open_dir(parts[:-1], create=False)
        except FileNotFoundError:
            return None
        try:
            fd = os.open(parts[-1], _READ_FLAGS, dir_fd=parent)
        except (FileNotFoundError, OSError):
            return None
        finally:
            os.close(parent)
        try:
            data = b""
            while chunk := os.read(fd, _CHUNK):
                data += chunk
        finally:
            os.close(fd)
        try:
            return json.loads(data.decode("utf-8"))
        except (UnicodeDecodeError, json.JSONDecodeError):
            return None

    def ensure_dir(self, parts: tuple[str, ...]) -> None:
        """Enter a container directory (`cases/<id>/run_selection`, ...), creating it when absent; never a link."""
        os.close(self._open_dir(parts, create=True))

    def reserve_dir(self, parts: tuple[str, ...]) -> tuple[str, ...]:
        """Create a new directory; refused by name when anything is already at that path."""
        parent = self._open_dir(parts[:-1], create=True)
        try:
            check_name(parts[-1], f"{self.what} directory")
            os.mkdir(parts[-1], 0o700, dir_fd=parent)
        except FileExistsError:
            raise ExistingPath(f"{self.what}: {'/'.join(parts)} already exists; nothing there is replaced") from None
        finally:
            os.close(parent)
        return parts

    def _new_file(self, parts: tuple[str, ...]) -> int:
        parent = self._open_dir(parts[:-1], create=False)
        try:
            check_name(parts[-1], f"{self.what} file")
            return os.open(parts[-1], _NEW_FILE_FLAGS, 0o600, dir_fd=parent)
        except FileExistsError:
            raise ExistingPath(f"{self.what}: {'/'.join(parts)} already exists; nothing there is replaced") from None
        except OSError as error:
            if error.errno == errno.ELOOP:
                raise ExistingPath(f"{self.what}: {'/'.join(parts)} is a symbolic link; nothing is written through "
                                   "it") from None
            raise
        finally:
            os.close(parent)

    def create(self, parts: tuple[str, ...], data: bytes | str) -> dict[str, Any]:
        """Create a new file holding `data`: {bytes, sha256}."""
        payload = data.encode("utf-8") if isinstance(data, str) else data
        fd = self._new_file(parts)
        try:
            view = memoryview(payload)
            while view:
                view = view[os.write(fd, view):]
            return {"bytes": os.fstat(fd).st_size, "sha256": _sha256(payload)}
        finally:
            os.close(fd)

    def copy(self, parts: tuple[str, ...], source: Path) -> dict[str, Any]:
        """Copy a regular file (not followed if a link) into a new file in one pass that also hashes it: {bytes,
        sha256}, the size from the file system. The content is never decoded."""
        src = os.open(source, _READ_FLAGS)
        try:
            fd = self._new_file(parts)
            try:
                digest = hashlib.sha256()
                while chunk := os.read(src, _CHUNK):
                    digest.update(chunk)
                    view = memoryview(chunk)
                    while view:
                        view = view[os.write(fd, view):]
                size = os.fstat(fd).st_size
                if size != os.fstat(src).st_size:
                    raise PackError(f"{Path(source).name} changed size while it was copied")
                return {"bytes": size, "sha256": digest.hexdigest()}
            finally:
                os.close(fd)
        finally:
            os.close(src)

    def record(self, parts: tuple[str, ...], value: Any) -> None:
        self.create(parts, json.dumps(value, indent=2, sort_keys=True) + "\n")

    def path(self, parts: Iterable[str]) -> Path:
        return self.base.joinpath(*parts)


# -- plan and observations --------------------------------------------------------------------------------------------
def _text(value: Any, where: str, *, optional: bool = False) -> str | None:
    if value is None and optional:
        return None
    if not isinstance(value, str) or not value.strip():
        raise PackError(f"{where} must be a non-empty string")
    return value


def _ident(value: Any, where: str) -> str:
    if not isinstance(value, str) or not CASE_ID_PATTERN.match(value):
        raise PackError(f"{where} {value!r} must match {CASE_ID_PATTERN.pattern}")
    return value


def check_variant(variant: Any, where: str) -> dict[str, Any]:
    """A variant: `label`, `kind` and either `instruction` (by hand) or `heading` (one of SUBSTITUTABLE_HEADINGS) and
    `new_line` (substitution of that options record's one data line)."""
    if not isinstance(variant, dict):
        raise PackError(f"{where} must be an object")
    label = _ident(variant.get("label"), f"{where} label")
    kind = variant.get("kind")
    if kind not in VARIANT_KINDS:
        raise PackError(f"{where} kind must be one of {', '.join(VARIANT_KINDS)}")
    checked = {"label": label, "kind": kind}
    if kind == "instruction":
        checked["instruction"] = _text(variant.get("instruction"), f"{where} instruction")
        extra = set(variant) - {"label", "kind", "instruction"}
    else:
        heading = variant.get("heading")
        if heading not in SUBSTITUTABLE_HEADINGS:
            raise PackError(f"{where} heading must be one of {', '.join(SUBSTITUTABLE_HEADINGS)}: a substitution "
                            "replaces only the data line of an options record, never a layout, material, section or "
                            "load line")
        line = variant.get("new_line")
        if not isinstance(line, str) or not line.strip() or "\r" in line or "\n" in line or _is_keyword(line):
            raise PackError(f"{where} new_line must be one non-empty data line")
        checked.update(heading=heading, new_line=line,
                       note=_text(variant.get("note"), f"{where} note", optional=True))
        extra = set(variant) - {"label", "kind", "heading", "new_line", "note"}
    if extra:
        raise PackError(f"{where} has unknown member(s) {', '.join(sorted(extra))}")
    return checked


def _variants(value: Any, where: str) -> list[dict[str, Any]]:
    if value is None:
        return []
    if not isinstance(value, list):
        raise PackError(f"{where} must be a list")
    variants = [check_variant(item, f"{where}[{index}]") for index, item in enumerate(value)]
    labels = [item["label"] for item in variants]
    if len(set(labels)) != len(labels):
        raise PackError(f"{where} repeats a label")
    return variants


def load_plan(path: Path | None = None) -> dict[str, Any]:
    """The plan, checked; `path` None gives the built-in default."""
    source = Path(path) if path is not None else DEFAULT_PLAN
    try:
        data = source.read_bytes()
        plan = json.loads(data.decode("utf-8"))
    except (OSError, UnicodeDecodeError, json.JSONDecodeError) as error:
        raise PackError(f"plan {source.name}: not readable JSON ({error})") from None
    if not isinstance(plan, dict) or plan.get("document_kind") != PLAN_KIND:
        raise PackError(f"plan {source.name}: document_kind must be {PLAN_KIND}")
    parts, seen, samples = [], set(), set()
    for index, part in enumerate(plan.get("parts") or []):
        where = f"plan part {index}"
        if not isinstance(part, dict):
            raise PackError(f"{where} must be an object")
        part_id = _ident(part.get("id"), f"{where} id")
        if part_id in seen:
            raise PackError(f"{where}: id {part_id} is repeated")
        seen.add(part_id)
        kind = part.get("kind")
        if kind not in PART_KINDS:
            raise PackError(f"{where} kind must be one of {', '.join(PART_KINDS)}")
        select = part.get("select") or {}
        if not isinstance(select, dict) or set(select) - set(SELECT_KEYS):
            raise PackError(f"{where} select takes only {', '.join(SELECT_KEYS)}")
        for key in ("model_ids", "families", "series", "model_members"):
            if key in select and not (isinstance(select[key], list)
                                      and all(isinstance(item, str) for item in select[key])):
                raise PackError(f"{where} select {key} must be a list of strings")
        if "all" in select and not isinstance(select["all"], bool):
            raise PackError(f"{where} select all must be true or false")
        overrides = part.get("case_overrides") or {}
        if not isinstance(overrides, dict):
            raise PackError(f"{where} case_overrides must be an object")
        checked_overrides = {}
        for model_id, override in sorted(overrides.items()):
            if not isinstance(override, dict) or set(override) - {"kind", "sealed_reason", "variants"}:
                raise PackError(f"{where} case_overrides {model_id} takes only kind, sealed_reason and variants")
            if override.get("kind") not in (None, "sealed_capture"):
                raise PackError(f"{where} case_overrides {model_id} kind may only be sealed_capture")
            checked_overrides[model_id] = {
                "sealed": override.get("kind") == "sealed_capture",
                "sealed_reason": _text(override.get("sealed_reason"), f"{where} case_overrides {model_id} sealed_reason",
                                       optional=True),
                "variants": _variants(override.get("variants"), f"{where} case_overrides {model_id} variants")}
            if checked_overrides[model_id]["sealed"] and not (checked_overrides[model_id]["sealed_reason"]
                                                              or part.get("sealed_reason")):
                raise PackError(f"{where} case_overrides {model_id}: a sealed capture needs a sealed_reason")
        format_samples = []
        for sample_index, sample in enumerate(part.get("format_samples") or []):
            swhere = f"{where} format_samples[{sample_index}]"
            if not isinstance(sample, dict):
                raise PackError(f"{swhere} must be an object")
            sample_id = _ident(sample.get("id"), f"{swhere} id")
            if sample_id in samples:
                raise PackError(f"{swhere}: id {sample_id} is repeated")
            samples.add(sample_id)
            if set(sample) != {"id"}:
                raise PackError(f"{swhere} names a declared format sample by its id only; its file, purpose and any "
                                "instruction come from the format-sample declaration")
            format_samples.append({"id": sample_id})
        sealed_reason = _text(part.get("sealed_reason"), f"{where} sealed_reason", optional=True)
        if kind == "sealed_capture" and not sealed_reason:
            raise PackError(f"{where}: a sealed_capture part needs a sealed_reason")
        parts.append({
            "id": part_id,
            "title": _text(part.get("title"), f"{where} title"),
            "buys": _text(part.get("buys"), f"{where} buys"),
            "estimate": _text(part.get("estimate"), f"{where} estimate", optional=True),
            "kind": kind,
            "select": {key: select[key] for key in SELECT_KEYS if key in select},
            "empty_reason": _text(part.get("empty_reason"), f"{where} empty_reason", optional=True),
            "sealed_reason": sealed_reason,
            "variants": _variants(part.get("variants"), f"{where} variants"),
            "case_overrides": checked_overrides,
            "format_samples": format_samples,
        })
    if not parts:
        raise PackError(f"plan {source.name}: no parts")
    return {"parts": parts, "sha256": _sha256(data),
            "source": "built-in default" if path is None else source.name}


def load_observations(path: Path | None) -> dict[str, Any] | None:
    """The observation items, checked and sorted by rank; each input file is read here (relative to the file)."""
    if path is None:
        return None
    source = Path(path)
    try:
        data = source.read_bytes()
        document = json.loads(data.decode("utf-8"))
    except (OSError, UnicodeDecodeError, json.JSONDecodeError) as error:
        raise PackError(f"observations {source.name}: not readable JSON ({error})") from None
    if not isinstance(document, dict) or document.get("document_kind") != OBSERVATIONS_KIND:
        raise PackError(f"observations {source.name}: document_kind must be {OBSERVATIONS_KIND}")
    items, ids, ranks = [], set(), set()
    base = source.resolve().parent
    for index, item in enumerate(document.get("items") or []):
        where = f"observations item {index}"
        if not isinstance(item, dict):
            raise PackError(f"{where} must be an object")
        item_id = _ident(item.get("id"), f"{where} id")
        rank = item.get("rank")
        if not isinstance(rank, int) or isinstance(rank, bool) or rank < 1 or rank in ranks or item_id in ids:
            raise PackError(f"{where}: rank must be a positive integer and rank and id unique")
        ids.add(item_id)
        ranks.add(rank)
        bring_back = item.get("bring_back")
        if not (isinstance(bring_back, list) and bring_back
                and all(isinstance(name, str) and re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9._-]{0,127}", name)
                        for name in bring_back)) or len(set(bring_back)) != len(bring_back):
            raise PackError(f"{where} bring_back must list plain file names")
        checked = {"id": item_id, "rank": rank, "title": _text(item.get("title"), f"{where} title"),
                   "action": _text(item.get("action"), f"{where} action"), "bring_back": list(bring_back),
                   "input": None}
        if item.get("input") is not None:
            relative = item["input"]
            if (not isinstance(relative, str) or Path(relative).is_absolute()
                    or any(part in ("", ".", "..") for part in Path(relative).parts)):
                raise PackError(f"{where} input must be a relative path below the observations file")
            input_path = base / relative
            if not input_path.is_file() or input_path.is_symlink():
                raise PackError(f"{where} input {relative} is not a regular file")
            name = Path(relative).name
            if name in bring_back:
                raise PackError(f"{where}: the input's name {name} is also a record to bring back")
            checked["input"] = {"file": name, "bytes_data": input_path.read_bytes()}
        items.append(checked)
    return {"items": sorted(items, key=lambda entry: entry["rank"]), "sha256": _sha256(data), "source": source.name}


# -- selection and order ----------------------------------------------------------------------------------------------
def _selects(select: Mapping[str, Any], model: Mapping[str, Any]) -> bool:
    return bool(select.get("all") or model["model_id"] in (select.get("model_ids") or ())
                or model.get("family") in (select.get("families") or ())
                or set(model.get("series") or ()) & set(select.get("series") or ())
                or set(model.get("model") or {}) & set(select.get("model_members") or ()))


def order_models(models: Iterable[Mapping[str, Any]], layout_rows: Mapping[str, int],
                 series_members: Mapping[str, list[str]] | None = None) -> list[Mapping[str, Any]]:
    """Run order within one part: by family; within a family the members of one series next to each other (in the
    series' member order), each series or lone model placed by its smallest layout row count, then its first id."""
    series_members = series_members or {}
    by_family: dict[str, list[Mapping[str, Any]]] = {}
    for model in models:
        by_family.setdefault(str(model.get("family") or ""), []).append(model)
    ordered: list[Mapping[str, Any]] = []
    for family in sorted(by_family):
        units: dict[str, list[Mapping[str, Any]]] = {}
        for model in by_family[family]:
            series = sorted(model.get("series") or [])
            units.setdefault(f"series:{series[0]}" if series else f"model:{model['model_id']}", []).append(model)
        keyed = []
        for key, members in units.items():
            order = series_members.get(key.split(":", 1)[1], []) if key.startswith("series:") else []
            members.sort(key=lambda m: (order.index(m["model_id"]) if m["model_id"] in order else len(order),
                                        m["model_id"]))
            keyed.append(((min(layout_rows.get(m["model_id"], 0) for m in members), members[0]["model_id"]), members))
        for _, members in sorted(keyed, key=lambda pair: pair[0]):
            ordered.extend(members)
    return ordered


def plan_parts(plan: Mapping[str, Any], models: list[Mapping[str, Any]], layout_rows: Mapping[str, int],
               series_members: Mapping[str, list[str]] | None = None, selected: Iterable[str] | None = None,
               observations: Mapping[str, Any] | None = None) -> list[dict[str, Any]]:
    """Each part with its ordered models (each in the first part that selects it), its format samples, observation
    items and, when it holds nothing, the reason."""
    chosen = set(selected) if selected else None
    placed: dict[str, str] = {}
    result = []
    for part in plan["parts"]:
        matched = [m for m in models if _selects(part["select"], m) and (chosen is None or m["model_id"] in chosen)]
        earlier = sorted(m["model_id"] for m in matched if m["model_id"] in placed)
        mine = [m for m in matched if m["model_id"] not in placed]
        for model in mine:
            placed[model["model_id"]] = part["id"]
        entry = {**part, "models": order_models(mine, layout_rows, series_members), "already_placed": {
            model_id: placed[model_id] for model_id in earlier}, "observations": []}
        if part["kind"] == "observations":
            entry["observations"] = list((observations or {}).get("items") or [])
        if not entry["models"] and not entry["format_samples"] and not entry["observations"]:
            if part["kind"] == "observations":
                entry["empty_because"] = "no observation items were supplied (--observations)"
            elif part["empty_reason"]:
                entry["empty_because"] = part["empty_reason"]
            elif not part["select"]:
                entry["empty_because"] = "the plan selects no case for this part"
            elif earlier:
                entry["empty_because"] = "every case it selects is already in an earlier part"
            else:
                entry["empty_because"] = ("no case of the manifest matches its selection "
                                          + json.dumps(part["select"], sort_keys=True))
        result.append(entry)
    return result


# -- pack contents ----------------------------------------------------------------------------------------------------
def mbf_settings(text: str) -> dict[str, Any]:
    """The settings that must be equal on both sides: the MBF's option records as the writer writes them (every
    record between VERSION and MATERIAL), and what they and the writer's conventions mean."""
    lines = text.split("\r\n")
    records: list[dict[str, str]] = []
    if "VERSION" in lines:
        index = lines.index("VERSION")
        while index + 1 < len(lines) and lines[index] != "MATERIAL":
            records.append({"record": lines[index], "value": lines[index + 1]})
            index += 2
    values = {item["record"]: item["value"] for item in records}
    options = values.get("OPTIONS", "").split(",")
    return {
        "mbf_option_records": records,
        "piping_code": options[1] if len(options) == 3 else None,
        "b31j": "off (OPCODE 0,0,0)" if values.get("OPCODE") == "0,0,0" else "no OPCODE record written",
        "reference_temperature_c": values.get("OPTEMP"),
        "units": options[-1] if options and options[-1] in ("SI", "ENGLISH") else values.get("OPTIONS"),
        "vertical_axis": "Y",
        "gravity_m_per_s2": STANDARD_GRAVITY,
        "pressure_options": values.get("OPPRES") or "no OPPRES record written",
    }


def _is_keyword(line: str) -> bool:
    return line.isalpha() and line.isupper()


def substitute_setting(text: str, heading: str, new_line: str) -> tuple[str, str]:
    """(`text` with the one data line of the options record `heading` replaced by `new_line`, the line replaced).

    Only an options record on SUBSTITUTABLE_HEADINGS, written before MATERIAL, may change: the heading must occur
    exactly once, be followed by one data line that occurs exactly once in the file, and the new line must be a data
    line. No layout, material, section or load line can change, so no node, element or support changes."""
    if heading not in SUBSTITUTABLE_HEADINGS:
        raise PackError(f"a substitution may change only the data line of {', '.join(SUBSTITUTABLE_HEADINGS)}")
    lines = text.split("\r\n")
    count = lines.count(heading)
    if count != 1:
        raise PackError(f"the options heading {heading} occurs {count} times in the MBF; it must occur once")
    at = lines.index(heading)
    if "MATERIAL" in lines and at > lines.index("MATERIAL"):
        raise PackError(f"{heading} is not in the options block (before MATERIAL)")
    if at + 1 >= len(lines) or not lines[at + 1].strip() or _is_keyword(lines[at + 1]):
        raise PackError(f"{heading} has no data line to replace")
    old = lines[at + 1]
    if lines.count(old) != 1:
        raise PackError(f"the data line under {heading} occurs {lines.count(old)} times in the MBF; it must occur once")
    if not new_line.strip() or "\r" in new_line or "\n" in new_line or _is_keyword(new_line) or new_line == old:
        raise PackError(f"the new line under {heading} must be one data line that differs from the old one")
    lines[at + 1] = new_line
    return "\r\n".join(lines), old


def _layout_rows_of(text: str) -> int:
    """The layout row count of an MBF text as the writer counts it: the title row plus the rows after LAYOUT."""
    lines = text.replace("\r\n", "\n").split("\n")
    if "LAYOUT" not in lines:
        return 0
    rows = [line for line in lines[lines.index("LAYOUT") + 1:] if line.strip()]
    return 1 + len(rows)


def _result_names(stem: str) -> dict[str, Any]:
    return {
        "guest_script": [f"{stem}.guest.json", f"{stem}.csv (or {stem}.txt; {stem}_results.csv with the GUI "
                                               "fallback)", f"{stem}.mod", f"{stem}.res"],
        "manual_batch": [f"{stem}.csv (or {stem}.txt)", f"{stem}.mod", f"{stem}.res"],
        "manual_print": [f"{stem}.print.csv", f"{stem}.mod", f"{stem}.res"],
        "run_line": "run_line.txt",
    }


def run_line_text(item: Mapping[str, Any]) -> str:
    lines = [f"SWBPIPE sitting run line: {item['item_id']}",
             "Fill in after each colon. Do not change the words before the colon.",
             "date: ",
             "route used (guest_script, manual_batch or manual_print): ",
             "CAEPIPE version as displayed: ",
             "anything unusual: "]
    for variant in item.get("variants") or []:
        prefix = f"variant {variant['label']}"
        lines += ["", f"{prefix} date: "]
        if variant["kind"] == "substitution":
            lines.append(f"{prefix} route used (guest_script, manual_batch or manual_print): ")
        lines += [f"{prefix} CAEPIPE version as displayed: ", f"{prefix} anything unusual: "]
    return "\r\n".join(lines) + "\r\n"


def _variant_entry(variant: Mapping[str, Any], stem: str, base_text: str | None, base_sha: str | None,
                   files: dict[str, bytes], folder: str, identity_file: str | None) -> dict[str, Any]:
    label = variant["label"]
    vstem = f"{stem}.{label}"
    if variant["kind"] == "instruction":
        return {"label": label, "kind": "instruction", "instruction": variant["instruction"],
                "mbf": {"file": f"{stem}.mbf", "sha256": base_sha, "same_as_base": True},
                "result_file": f"{vstem}.print.csv", "model_file": f"{vstem}.mod",
                "results": {"manual_print": [f"{vstem}.print.csv", f"{vstem}.mod", f"{vstem}.res"]}}
    if base_text is None:
        raise PackError(f"{stem}: a substitution variant needs the writer's MBF")
    text, old_line = substitute_setting(base_text, variant["heading"], variant["new_line"])
    data = text.encode("utf-8")
    files[f"{folder}/{vstem}.mbf"] = data
    names = _result_names(vstem)
    names.pop("run_line")
    return {"label": label, "kind": "substitution", "heading": variant["heading"], "old_line": old_line,
            "new_line": variant["new_line"], "note": variant.get("note"),
            "mbf": {"file": f"{vstem}.mbf", "sha256": _sha256(data), "bytes": len(data), "same_as_base": False},
            "writer_output": False,
            "identity": (f"no identity file of its own: it is not the writer's output. The base case's identity file "
                         f"{identity_file} binds its rows: the code admits a substitution only of the data line of the "
                         f"options record {variant['heading']}, so no node, element or support line changed"),
            "identity_file_of_base": identity_file,
            "results": names}


def benchmark_request(model: Mapping[str, Any]) -> dict[str, Any]:
    """The complete analysis request run_pilot sends the runner for a benchmark model (`solve_swbpipe`'s body as
    `_run_benchmark_model` builds it); the index records its canonical hash."""
    model_id = model["model_id"]
    return {"request": run_pilot.runner_request(model_id, f"validation/caepipe_benchmark/models/{model_id}.json"),
            "solve": {"preview_model": {"model": model["model"], "materials": []}}}


def benchmark_item(model: Mapping[str, Any], manifest_entry: Mapping[str, Any], part: Mapping[str, Any], *,
                   sealed_reason: str | None, variants: list[Mapping[str, Any]],
                   files: dict[str, bytes]) -> dict[str, Any]:
    """The index entry of one benchmark case, adding its pack files to `files`; a refused render lists its codes."""
    model_id = model["model_id"]
    stem = model_id
    folder = f"cases/{model_id}"
    item: dict[str, Any] = {
        "item_id": model_id, "item_kind": "benchmark_case", "stem": stem, "folder": folder, "part": part["id"],
        "capture": "sealed" if sealed_reason else "screened", "sealed_reason": sealed_reason,
        "model_id": model_id, "family": model.get("family"), "series": list(model.get("series") or []),
        "source": {"file": f"validation/caepipe_benchmark/{manifest_entry['file']}",
                   "file_sha256": manifest_entry.get("file_sha256"), "model_hash": manifest_entry.get("model_hash")},
        "inexact_inputs": model.get("inexact_inputs") or {},
    }
    for member in ("model_settings",):
        if member in model:
            item[member] = model[member]
    if "pressure_contract" in (model.get("model") or {}):
        item["pressure_contract"] = model["model"]["pressure_contract"]
    item["swbpipe_request"] = {
        "canonical_sha256": canonical_model_hash(benchmark_request(model)),
        "basis": "the complete analysis request run_pilot sends the runner for this model (solve_swbpipe's body)"}
    mbf = run_pilot.render_benchmark_mbf(model)
    item["layout_row_count"] = mbf.layout_row_count
    if mbf.status != "written":
        item.update(status="refused", refusal_codes=sorted({entry["code"] for entry in mbf.diagnostics
                                                            if entry["severity"] == "error"}), variants=[])
        return item
    text = mbf.text
    data = text.encode("utf-8")
    mbf_sha = _sha256(data)
    identity = (json.dumps(mbf.sidecar(mbf_sha), sort_keys=True, indent=2, ensure_ascii=True, allow_nan=False)
                + "\n").encode("ascii")
    report = _json_bytes({"profile_id": mbf.to_dict()["profile_id"], "status": mbf.status,
                          "layout_row_count": mbf.layout_row_count, "piping_code": model["piping_code"],
                          "diagnostics": mbf.diagnostics, "loss_report": mbf.loss_report,
                          "assumptions": mbf.assumptions, "mbf_sha256": mbf_sha})
    files[f"{folder}/{stem}.mbf"] = data
    files[f"{folder}/{stem}.swbpipe-ids.json"] = identity
    files[f"{folder}/{stem}.writer-report.json"] = report
    item.update(status="written", refusal_codes=[],
                mbf={"file": f"{stem}.mbf", "sha256": mbf_sha, "bytes": len(data)},
                identity_file={"file": f"{stem}.swbpipe-ids.json", "sha256": _sha256(identity)},
                writer_report={"file": f"{stem}.writer-report.json", "sha256": _sha256(report)},
                settings=mbf_settings(text), results=_result_names(stem),
                variants=[_variant_entry(variant, stem, text, mbf_sha, files, folder,
                                         f"{stem}.swbpipe-ids.json") for variant in variants])
    return item


def load_samples(path: Path | None = None) -> dict[str, Any]:
    """The maintained format-sample declaration: each sample's id, its hand-authored invented model file (beside the
    declaration, with a `<stem>.provenance.json` declaring it invented and not produced by CAEPIPE), the file's
    SHA-256, a title, the sections it is meant to make the program print, and an optional instruction."""
    source = Path(path) if path is not None else DEFAULT_SAMPLES
    try:
        data = source.read_bytes()
        document = json.loads(data.decode("utf-8"))
    except (OSError, UnicodeDecodeError, json.JSONDecodeError) as error:
        raise PackError(f"format-sample declaration {source.name}: not readable JSON ({error})") from None
    if not isinstance(document, dict) or document.get("document_kind") != SAMPLES_KIND:
        raise PackError(f"format-sample declaration {source.name}: document_kind must be {SAMPLES_KIND}")
    samples = {}
    for index, entry in enumerate(document.get("samples") or []):
        where = f"format-sample declaration entry {index}"
        if not isinstance(entry, dict) or set(entry) - {"id", "file", "sha256", "title", "prints", "instruction"}:
            raise PackError(f"{where} takes only id, file, sha256, title, prints and instruction")
        sample_id = _ident(entry.get("id"), f"{where} id")
        if sample_id in samples:
            raise PackError(f"{where}: id {sample_id} is repeated")
        name = check_name(entry.get("file"), f"{where} file")
        if not name.endswith(".mbf"):
            raise PackError(f"{where}: file must be an .mbf")
        sha = entry.get("sha256")
        if not isinstance(sha, str) or not re.fullmatch(r"[0-9a-f]{64}", sha):
            raise PackError(f"{where}: sha256 must be 64 lowercase hex digits")
        samples[sample_id] = {"id": sample_id, "file": name, "sha256": sha,
                              "title": _text(entry.get("title"), f"{where} title"),
                              "prints": _text(entry.get("prints"), f"{where} prints"),
                              "instruction": _text(entry.get("instruction"), f"{where} instruction", optional=True)}
    return {"samples": samples, "dir": source.resolve().parent, "sha256": _sha256(data), "source": source.name}


def format_sample_item(sample: Mapping[str, Any], part: Mapping[str, Any], declaration: Mapping[str, Any],
                       benchmark_ids: set[str], benchmark_hashes: Mapping[str, str],
                       files: dict[str, bytes]) -> dict[str, Any]:
    """The index entry of one declared format sample, its model file copied byte for byte. Refused when the plan
    names a sample the declaration does not hold, when the id is a benchmark model's, when the file's hash is not the
    declared one, when the provenance sidecar does not declare it invented, or when the file is, byte for byte, the
    writer's MBF or the model file of any benchmark model (whatever it is called)."""
    sample_id = sample["id"]
    if sample_id in benchmark_ids:
        raise PackError(f"format sample {sample_id}: the id names a model of the benchmark manifest; a benchmark model "
                        "is never a format sample")
    entry = declaration["samples"].get(sample_id)
    if entry is None:
        raise PackError(f"format sample {sample_id}: not in the format-sample declaration {declaration['source']}")
    path = declaration["dir"] / entry["file"]
    if path.is_symlink() or not path.is_file():
        raise PackError(f"format sample {sample_id}: {entry['file']} is not a regular file beside the declaration")
    data = path.read_bytes()
    sha = _sha256(data)
    if sha != entry["sha256"]:
        raise PackError(f"format sample {sample_id}: {entry['file']} does not have the SHA-256 the declaration binds")
    if sha in benchmark_hashes:
        raise PackError(f"format sample {sample_id}: {entry['file']} is the {benchmark_hashes[sha]}; a benchmark model "
                        "is never a format sample")
    provenance = path.with_name(Path(entry["file"]).stem + ".provenance.json")
    try:
        declared = json.loads(provenance.read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, json.JSONDecodeError):
        declared = {}
    if declared.get("produced_by_caepipe") is not False or "invented" not in str(declared.get("origin", "")):
        raise PackError(f"format sample {sample_id}: {provenance.name} does not declare the file invented")
    folder = f"samples/{sample_id}"
    files[f"{folder}/{entry['file']}"] = data
    stem = Path(entry["file"]).stem
    return {"item_id": sample_id, "item_kind": "format_sample", "part": part["id"], "folder": folder, "stem": stem,
            "status": "written", "capture": "stored_only", "outside_benchmark_set": True,
            "title": entry["title"], "prints": entry["prints"], "instruction": entry["instruction"],
            "designation": {"declaration": declaration["source"], "declaration_sha256": declaration["sha256"],
                            "id": sample_id, "file": entry["file"], "sha256": sha},
            "source": {"provenance": provenance.name, "provenance_sha256": _sha256(provenance.read_bytes())},
            "mbf": {"file": entry["file"], "sha256": sha, "bytes": len(data), "copied_byte_for_byte": True},
            "identity_file": None, "layout_row_count": _layout_rows_of(data.decode("utf-8", errors="replace")),
            "results": _result_names(stem), "variants": [],
            "never_screened": "a format sample is never screened or compared; ingest stores and hashes what comes "
                              "back and does not parse it"}


def observation_item(observation: Mapping[str, Any], part: Mapping[str, Any], files: dict[str, bytes]) -> dict:
    folder = f"observations/{observation['id']}"
    item = {"item_id": observation["id"], "item_kind": "format_observation", "part": part["id"], "folder": folder,
            "rank": observation["rank"], "title": observation["title"], "action": observation["action"],
            "bring_back": observation["bring_back"], "input": None, "capture": "stored_only",
            "never_interpreted": "ingest checks which records came back and stores them; it does not interpret them"}
    if observation["input"] is not None:
        data = observation["input"]["bytes_data"]
        files[f"{folder}/{observation['input']['file']}"] = data
        item["input"] = {"file": observation["input"]["file"], "sha256": _sha256(data), "bytes": len(data)}
    return item


def build_pack(manifest_path: Path, plan: Mapping[str, Any], observations: Mapping[str, Any] | None = None,
               selected: Iterable[str] | None = None, *, samples: Mapping[str, Any] | None = None) -> dict[str, bytes]:
    """Every file of the pack by its relative path: same inputs, same bytes."""
    manifest_path = Path(manifest_path)
    manifest_bytes = manifest_path.read_bytes()
    manifest = json.loads(manifest_bytes.decode("utf-8"))
    selected = sorted(set(selected)) if selected else None
    models = run_pilot.load_benchmark(manifest_path)
    by_id = {model["model_id"]: model for model in models}
    entries = {entry["model_id"]: entry for entry in manifest.get("models") or []}
    if selected:
        unknown = sorted(set(selected) - set(by_id))
        if unknown:
            raise PackError(f"unknown benchmark model id(s): {', '.join(unknown)}")
    rendered = {model["model_id"]: run_pilot.render_benchmark_mbf(model) for model in models}
    rows = {model_id: mbf.layout_row_count for model_id, mbf in rendered.items()}
    benchmark_hashes = {entry["file_sha256"]: f"model file of benchmark model {model_id}"
                        for model_id, entry in entries.items() if entry.get("file_sha256")}
    benchmark_hashes.update({_sha256(mbf.text.encode("utf-8")): f"writer's MBF of benchmark model {model_id}"
                             for model_id, mbf in rendered.items() if mbf.status == "written"})
    samples = samples if samples is not None else load_samples()
    series_members = {name: [member["model_id"] for member in definition.get("members") or []]
                      for name, definition in (manifest.get("series") or {}).items()}
    parts = plan_parts(plan, models, rows, series_members, selected, observations)
    files: dict[str, bytes] = {}
    items: list[dict[str, Any]] = []
    index_parts = []
    for part in parts:
        part_items = []
        for model in part["models"]:
            override = part["case_overrides"].get(model["model_id"], {})
            sealed = part["kind"] == "sealed_capture" or override.get("sealed")
            reason = (override.get("sealed_reason") or part["sealed_reason"]) if sealed else None
            variants = [*part["variants"], *override.get("variants", [])]
            if len({variant["label"] for variant in variants}) != len(variants):
                raise PackError(f"{model['model_id']}: a variant label is repeated across the part and its override")
            part_items.append(benchmark_item(model, entries[model["model_id"]], part, sealed_reason=reason,
                                             variants=variants, files=files))
        for sample in part["format_samples"]:
            part_items.append(format_sample_item(sample, part, samples, set(by_id), benchmark_hashes, files))
        for observation in part["observations"]:
            part_items.append(observation_item(observation, part, files))
        for item in part_items:
            item["order"] = len(items) + 1
            items.append(item)
            if item.get("status", "written") == "written" and item["item_kind"] != "format_observation":
                run_line = _crlf(run_line_text(item))
                files[f"{item['folder']}/run_line.txt"] = run_line
                item["run_line"] = {"file": "run_line.txt", "template_sha256": _sha256(run_line),
                                    "template_bytes": len(run_line)}
        index_parts.append({
            "id": part["id"], "title": part["title"], "buys": part["buys"], "estimate": part["estimate"],
            "kind": part["kind"], "select": part["select"], "items": [item["item_id"] for item in part_items],
            "already_placed": part["already_placed"], "empty_because": part.get("empty_because")})
    for item in items:
        if item.get("status", "written") == "written":
            files[f"{item['folder']}/case.json"] = _json_bytes(item)
    guest = GUEST_SCRIPT.read_bytes()
    driver = DRIVER.read_bytes()
    files["scripts/Invoke-CaepipeRoundTrip.ps1"] = guest
    files["scripts/Run-SittingCase.ps1"] = driver
    try:
        manifest_label = str(manifest_path.resolve().relative_to(PROJECT))
    except ValueError:
        manifest_label = manifest_path.name
    index = {
        "document_kind": INDEX_KIND, "pack_version": PACK_VERSION, "generator": GENERATOR,
        "inputs": {"manifest": {"path": manifest_label, "sha256": _sha256(manifest_bytes)},
                   "plan": {"source": plan["source"], "sha256": plan["sha256"]},
                   "observations": ({"source": observations["source"], "sha256": observations["sha256"]}
                                    if observations else None),
                   "format_samples": {"source": samples["source"], "sha256": samples["sha256"]},
                   "selected_cases": selected},
        "scripts": {"guest_script": {"file": "scripts/Invoke-CaepipeRoundTrip.ps1",
                                     "repository_path": GUEST_SCRIPT_REPOSITORY_PATH, "sha256": _sha256(guest)},
                    "driver": {"file": "scripts/Run-SittingCase.ps1", "repository_path": DRIVER_REPOSITORY_PATH,
                               "sha256": _sha256(driver)}},
        "batch_command_shape": BATCH_COMMAND_SHAPE,
        "parts": index_parts,
        "items": items,
        "privacy": ("The pack holds SWBPIPE's invented inputs only. Everything the program writes into it is private: "
                    "it stays in the owner's exchange folder or another private place outside every repository, "
                    "and is never committed, shared or uploaded."),
        "boundary_note": BOUNDARY_NOTE,
    }
    files["pack_index.json"] = _json_bytes(index)
    files["RUN_SHEET.md"] = _crlf(run_sheet_text(index))
    files["INSTRUCTIONS.md"] = _crlf(instructions_text(index))
    return files


def write_pack(out_dir: Path, files: Mapping[str, bytes], authority: Path) -> Path:
    """Reserve `out_dir` and `authority` (each created here; refused if it exists) outside every Git work tree, and
    create every pack file in `out_dir` and the index with its SHA-256 in `authority`, each file exclusively."""
    out = Path(os.path.abspath(out_dir))
    authority = Path(os.path.abspath(authority))
    for what, directory in (("prepare", out), ("the authority folder", authority)):
        outside_work_trees(directory, what)
        directory.parent.mkdir(parents=True, exist_ok=True)
        try:
            os.mkdir(directory, 0o700)
        except FileExistsError:
            raise ExistingPath(f"{what}: {directory} already exists; a pack is written only into a new folder, so "
                               "nothing there is ever replaced") from None
    with Tree(out, "prepare") as tree:
        for relative in sorted(files):
            parts = tuple(relative.split("/"))
            tree.ensure_dir(parts[:-1])
            tree.create(parts, files[relative])
    with Tree(authority, "the authority folder") as tree:
        index = files["pack_index.json"]
        tree.create((AUTHORITY_INDEX,), index)
        tree.create((AUTHORITY_DIGEST,), f"{_sha256(index)}  {AUTHORITY_INDEX}\n")
    return out


# -- generated documents ----------------------------------------------------------------------------------------------
def _estimate(part: Mapping[str, Any]) -> str:
    return (f"Estimated time: {part['estimate']} (an estimate, not a condition)." if part.get("estimate")
            else "Estimated time: not given.")


def _kind_label(item: Mapping[str, Any]) -> str:
    if item["item_kind"] == "format_sample":
        return "format sample"
    if item["item_kind"] == "format_observation":
        return "format observation"
    if item.get("status") == "refused":
        return "not to run (the writer refused it)"
    return "sealed capture" if item.get("capture") == "sealed" else "benchmark case"


def _harness_cases(index: Mapping[str, Any], part: Mapping[str, Any]) -> list[str]:
    by_id = {item["item_id"]: item for item in index["items"]}
    return [item_id for item_id in part["items"] if by_id[item_id]["item_kind"] == "benchmark_case"
            and by_id[item_id].get("status") == "written" and by_id[item_id]["capture"] == "screened"
            and not by_id[item_id]["variants"]]


def run_sheet_text(index: Mapping[str, Any]) -> str:
    by_id = {item["item_id"]: item for item in index["items"]}
    out = ["# SWBPIPE sitting run sheet", "",
           "Generated from `pack_index.json`. Run the parts in this order; you may stop at the end of any part and "
           "know what is left. Tick each line when its folder holds what it should.", ""]
    for part in index["parts"]:
        out += [f"## {part['id']}: {part['title']}", "", part["buys"], "", _estimate(part), ""]
        if part.get("empty_because"):
            out += [f"Nothing to run in this part: {part['empty_because']}", ""]
            continue
        cases = [] if part["kind"] == "dry_run" else _harness_cases(index, part)
        if cases:
            out += ["Harness route, this part's benchmark cases (not against the exchange folder until the harness "
                    "change named in INSTRUCTIONS.md section 1 is merged): `"
                    + " ".join(f"--case {c}" for c in cases) + "`", ""]
        out += ["| # | item | kind | layout rows | series | MBF SHA-256 (first 12) | folder | done |",
                "|---|---|---|---|---|---|---|---|"]
        for item_id in part["items"]:
            item = by_id[item_id]
            sha = (item.get("mbf") or {}).get("sha256") or ""
            rows = item.get("layout_row_count", "")
            if item["item_kind"] == "format_observation":
                rows = ""
            series = ", ".join(item.get("series") or [])
            out.append(f"| {item['order']} | {item_id} | {_kind_label(item)} | {rows} | {series} | {sha[:12]} | "
                       f"`{item['folder']}` | [ ] |")
            for variant in item.get("variants") or []:
                out.append(f"| {item['order']}.{variant['label']} | {item_id} variant {variant['label']} | "
                           f"{variant['kind']} variant | | | {variant['mbf']['sha256'][:12]} | `{item['folder']}` | [ ] |")
        if part["already_placed"]:
            out += ["", "Already placed in an earlier part: "
                    + ", ".join(f"{k} ({v})" for k, v in sorted(part["already_placed"].items()))]
        out.append("")
    return "\n".join(out)


def instructions_text(index: Mapping[str, Any]) -> str:
    src = SOURCES
    items = index["items"]
    sections = "\n".join(f"   - `{title}`" for title in PRINT_SECTIONS)
    options = "\n".join(f"- `{flag}`: {text}" for flag, text in HARNESS_OPTION_HELP)
    dry_run = [part for part in index["parts"] if part["kind"] == "dry_run"]
    dry_ids = ", ".join(dry_run[0]["items"]) if dry_run else "none"
    out = f"""# SWBPIPE sitting: instructions

This pack holds the cases SWBPIPE asks you to run in your own licensed CAEPIPE, in the order of `RUN_SHEET.md`.
Every input in it is invented. The pack installs nothing and needs nothing installed: you use your own CAEPIPE,
and the quicker way below uses Windows PowerShell, which Windows includes.

**The results are private.** Everything CAEPIPE writes into this pack stays private. Keep the returned pack in your
exchange folder (the folder named by `{ENV_VAR}`) or another private place outside every repository. Never commit,
share or upload it. The records SWBPIPE makes from it are written in the exchange folder, in each case's run
directory; never in a repository.

**What cannot be done yet.** A result taken in at this sitting is stored as a new run of its case and cannot be
screened yet: no existing reader uses the run that intake names, and the offline screen reads only a case's own
stored run. A later change makes it read a selected run and write to new paths. There is no unsealing command: every
route that parses or screens a result refuses a sealed capture, and unsealing is a later change with its own review.

## 1. The harness route (run_pilot's live benchmark mode)

This route runs the benchmark cases from this Mac through the existing harness, one at a time, against your guest.
It needs no copy of this pack on Windows. The command, from the SWBPIPE project root, is:

```
python3 -m tools.caepipe_correlation.run_pilot --benchmark validation/caepipe_benchmark/manifest.json \\
    --case ID [--case ID ...] [--batch N] [--runner PATH] \\
    --freeze-file PATH --exclusive-file PATH [--pace-s S]
```

Its options, in its own help text:

{options}

`RUN_SHEET.md` gives, per part, the `--case` list of that part's benchmark cases, except the dry run's.

It runs only when the exchange folder is configured as the harness checks it: `{ENV_VAR}` names an existing
folder outside every Git work tree, holding a `.gitignore` whose content is exactly `*` and a `{CONFIG_NAME}` that
sets `{ACKNOWLEDGEMENT_KEY}` to true, names `vm_name`, `guest_executable`, `guest_exchange_root` and
`guest_work_root` (absolute Windows paths of letters, digits, space and `\\ . - _ ( )` only) and, for a live
benchmark run, `settings_guard`. `prlctl` must be on the Mac's PATH, and the runner is named by `--runner` or
`OPENPIPESTRESS_RUNNER_BIN`.

The guest must be in this state before the command starts, and you bring it there: the virtual machine running and
awake, the Windows user logged in, and no instance of CAEPIPE running. Before every case the command checks that
the guest answers and that no instance of CAEPIPE is running, and stops otherwise.

Rules of this route:

1. After the guest is resumed, wait a fixed 5 minutes before the first harness command (measured 12 November 2026).
2. Before any live run, confirm which drive of the guest's session is the exchange folder. In the guest, the Windows
   command `net use` lists the session's drives; the exchange folder appears there as a shared folder of the
   virtualisation tool, under its own name. Its letter can differ from the one `guest_exchange_root` in
   `{CONFIG_NAME}` names (measured 12 November 2026). If it differs, do not start the run.

> **Until the harness change that resolves the exchange folder's drive in the guest and verifies it by its content
> is merged, start no live run that clears an output folder against the exchange folder at all.** The live benchmark
> mode clears each case's `caepipe_out` before it runs the case, so this route is not used against the exchange
> folder until then. Even after that, never run with this command a sealed capture, a format sample, a variant, or
> any case whose result is already stored, such as the dry run's cases: it has no capture-only path, and it clears
> what it replaces. Run those by the hand route below.

## 2. The hand route (the fallback, and the route for sealed captures, format samples and variants)

### 2.1 Before you start

> **Never leave the pressure options line blank.** When you import or edit a model by hand, the line under the
> `OPPRES` heading (the pressure options) must never be blank. The vendor's public import page
> ({_ref('CAEPIPE-IMPORT-MBF')}, "Format of .MBF") says a blank line there sets the Bourdon effect and the pressure
> correction for bends on. SWBPIPE's writer writes that line in full whenever it writes the heading: leave it exactly
> as written, and never clear it.

1. Copy the whole pack folder to the Windows machine at a short path of letters, digits and hyphens only, for
   example `C:\\swbpipe-sitting`. The batch page notes constraints on the argument and its path ({_ref('CAEPIPE-BATCH')}).
2. Do not rename any folder or file the pack holds.
3. Work down `RUN_SHEET.md` part by part. Each case has its own folder; everything for that case is done in it.

### 2.2 Run one case: three ways

**Quicker: the guest script.** In PowerShell, from the pack folder:

```
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\\Run-SittingCase.ps1 -Exe "<your CAEPIPE executable>" -Mbf "<case folder>\\<stem>.mbf"
```

It runs the documented batch pattern (below) through SWBPIPE's guest script and saves the script's report as
`<stem>.guest.json` beside the MBF. That report carries the SHA-256 of the MBF the program solved, which is what
lets SWBPIPE bind the result to the case. It stops only the processes it started. If the batch run leaves no print
file, it reports so and stops; then use the third way for that case.

**By hand, batch.** In a Command Prompt opened in the case folder, run the documented batch pattern
({_ref('CAEPIPE-BATCH')}, {_ref('CAEPIPE-IMPORT-MBF')}):

```
"<your CAEPIPE executable>" "<stem>.mbf";batchmode=yes
```

It imports the MBF and solves it. These files appear beside the MBF: `<stem>.mod`, `<stem>.res`, and the printed
results `<stem>.csv` (or `<stem>.txt` when the print-file extension saved in your CAEPIPE preferences is `.txt`).

**By hand, in the program.**

1. Import `<stem>.mbf` with the program's Import MBF command ({_ref('CAEPIPE-IMPORT-MBF')}) and save the model in
   the case folder as `<stem>.mod`.
2. Before analysing, check the settings the import set against the case's settings (in `case.json` and in the list
   at the end of this document): piping code, B31J off, reference temperature, SI units, vertical axis Y, gravity
   9.80665 m/s2 and the pressure options. Do not edit the MBF. If the program shows a different value, write the
   value it shows on the run line.
3. Analyse.
4. Print the results to a file ({_ref('CAEPIPE-PRINT-TO-FILE')}): Print, select All, To File, choose the CSV type,
   and save as `<stem>.print.csv` in the case folder. The print must hold these sections, for every load case:

{sections}

### 2.3 Variants

A variant is run after its base case, in the same folder.

- **Instruction variant**: import the same `<stem>.mbf`, make exactly the one change the case's variant states
  (listed at the end of this document), save the model as `<stem>.<label>.mod`, analyse, and print to file as
  `<stem>.<label>.print.csv`. Change nothing else.
- **Substitution variant**: the pack already holds its own file `<stem>.<label>.mbf`, in which only the data line
  of one options record (named at the end of this document) differs from the base MBF. Run it as any case, by any of
  the three ways, with `<stem>.<label>` as its stem.

### 2.4 Sealed captures and format samples

- A **sealed capture** is run exactly as any case and brought back. SWBPIPE stores it unopened; there is no
  unsealing command (unsealing is a later change).
- A **format sample** is a small invented model whose purpose is to see how the program prints the sections named
  for it at the end of this document. It is never screened or compared. Run it as any case, making the one change
  named for it if any; print all results to file as in 2.2.

### 2.5 The dry run comes first

Part 0 ({dry_ids}) is the dry run. Its cases already have stored results in the exchange folder, so they are run
by the hand route only: a dry-run case is never run through a route that clears a stored result. After part 0,
bring the pack back to the Mac and run

```
python3 -m tools.caepipe_correlation.sitting_pack ingest --check <the returned pack folder> --authority <the authority folder>
```

The authority folder is the one `prepare` wrote beside the pack on this Mac (`<pack folder>.authority` unless named
otherwise); it never goes to Windows. Ingest refuses a returned pack whose `pack_index.json` differs from it.

The check writes nothing. Like ingest it needs the exchange folder configured (section 1), which it only reads, to
recognise a case sealed there and leave it unread. It verifies each case's MBF, names the route and binding it found, and reports how many
sections the parser read from each returned print. Go on to part 1 only when it reports each dry-run case with
sections read and no error diagnostic. After the sitting, `ingest` without `--check` takes the pack into the
exchange folder by adding new files only: every returned result is stored in a new run folder of its own, beside
whatever the case already holds, which stays unchanged. Ingest never clears, replaces, renames or deletes anything
already stored there.

### 2.6 Fill in the run line

Each case folder holds `run_line.txt`. Fill it in for every case you run: the date, the route you used
(`guest_script`, `manual_batch` or `manual_print`), the CAEPIPE version as the program displays it, and anything
unusual. A variant has its own lines in the same file.

### 2.7 What never to edit

Never edit, rename, re-save or delete: any `.mbf` the pack holds, any `.swbpipe-ids.json`, any
`.writer-report.json`, any `case.json`, `pack_index.json`, or anything in `scripts`. Ingest refuses a case whose MBF
or identity file no longer has the SHA-256 the index records.

### 2.8 What to bring back

Bring the whole pack folder back, unchanged except for the files the program wrote, the guest reports and the
filled run lines. On the Mac, put it in your exchange folder or another private place outside every repository and
run ingest as in 2.5.

### 2.9 Rehearsing the hand route

The hand route can be rehearsed on its own, end to end, without CAEPIPE and without the harness, on one case with
invented result files:

1. Make a separate rehearsal exchange folder outside every repository, configured as section 1 describes (its own
   `.gitignore` whose content is `*` and its own `{CONFIG_NAME}`), and point `{ENV_VAR}` at it for the rehearsal
   only. Invented files never go into the real exchange folder.
2. Prepare a pack: `python3 -m tools.caepipe_correlation.sitting_pack prepare <new folder>`.
3. In one case folder, a person, or a script standing in for the program, places invented files under the names
   section 2.2 gives for "by hand, in the program": `<stem>.print.csv`, `<stem>.mod` and `<stem>.res`. Fill in
   `run_line.txt` with the route `manual_print`.
4. Run `ingest --check` and then `ingest` as in 2.5, with `--authority`, against the rehearsal exchange folder.
5. Ingest reports the case with the route `sitting_manual_print` and the binding `person_attested`. The files are in
   the case's new run folder under `returned/`, with their SHA-256 in its `sitting_intake.json`.

## 3. Per item

"""
    for item in items:
        out += f"### {item['order']}. {item['item_id']} ({_kind_label(item)})\n\n"
        if item["item_kind"] == "format_observation":
            out += (f"Folder `{item['folder']}`. {item['title']}.\n\nAction: {item['action']}\n\n"
                    f"Bring back, in that folder: {', '.join(f'`{name}`' for name in item['bring_back'])}.\n\n")
            continue
        if item.get("status") != "written":
            reason = item.get("empty_because") or ("the writer refused it: " + ", ".join(item.get("refusal_codes")
                                                                                         or []))
            out += f"Not to run: {reason}.\n\n"
            continue
        if item["item_kind"] == "format_sample":
            out += f"{item['title']}. Run to see {item['prints']}. Never screened or compared.\n\n"
            if item.get("instruction"):
                out += f"Before analysing: {item['instruction']}\n\n"
        if item.get("capture") == "sealed":
            out += f"Sealed capture: {item['sealed_reason']}\n\n"
        out += (f"Folder `{item['folder']}`, MBF `{item['mbf']['file']}`, layout rows {item['layout_row_count']}.\n\n")
        settings = item.get("settings")
        if settings:
            out += ("Settings to check: piping code {pc}; B31J {b}; reference temperature {t} C; units {u}; "
                    "vertical axis {v}; gravity {g} m/s2; pressure options {p}.\n\n").format(
                pc=settings["piping_code"], b=settings["b31j"], t=settings["reference_temperature_c"],
                u=settings["units"], v=settings["vertical_axis"], g=settings["gravity_m_per_s2"],
                p=settings["pressure_options"])
        for variant in item.get("variants") or []:
            if variant["kind"] == "instruction":
                out += (f"Variant `{variant['label']}` (instruction): {variant['instruction']} Save the print as "
                        f"`{variant['result_file']}`.\n\n")
            else:
                out += (f"Variant `{variant['label']}` (substitution): run `{variant['mbf']['file']}`; it differs "
                        f"from the base MBF only in the data line of `{variant['heading']}`, `{variant['old_line']}` "
                        f"replaced by `{variant['new_line']}`.\n\n")
        out += "Run line (`run_line.txt`):\n\n```\n" + run_line_text(item).replace("\r\n", "\n") + "```\n\n"
    out += "## Sources\n\n" + "\n".join(f"- {key}: {url}" for key, url in SOURCES.items()) + "\n"
    return out


def _ref(source_id: str) -> str:
    return f"{source_id}, {SOURCES[source_id]}"


# -- intake -----------------------------------------------------------------------------------------------------------
def load_authority(authority: Path) -> tuple[dict[str, Any], str]:
    """The host's own copy of a prepared pack's index and its SHA-256, checked against each other and validated."""
    directory = Path(authority)
    try:
        data = (directory / AUTHORITY_INDEX).read_bytes()
        recorded = (directory / AUTHORITY_DIGEST).read_text(encoding="ascii").split()[0]
    except (OSError, UnicodeDecodeError, IndexError) as error:
        raise PackError(f"the authority folder {directory} holds no readable {AUTHORITY_INDEX} and "
                        f"{AUTHORITY_DIGEST} ({error})") from None
    if _sha256(data) != recorded:
        raise PackError(f"the authority folder's {AUTHORITY_INDEX} does not have the SHA-256 its {AUTHORITY_DIGEST} "
                        "records; nothing is taken in")
    index = json.loads(data.decode("utf-8"))
    validate_index(index)
    return index, recorded


def validate_index(index: Any) -> None:
    """Every id, folder and file name an index gives is a plain name in the place its kind assigns; every kind and
    capture status is one this module knows. Anything else refuses the whole index."""
    if not isinstance(index, dict) or index.get("document_kind") != INDEX_KIND:
        raise PackError(f"not a sitting pack index ({INDEX_KIND})")
    seen = set()
    for item in index.get("items") or []:
        item_id = item.get("item_id") if isinstance(item, dict) else None
        if not isinstance(item_id, str) or not CASE_ID_PATTERN.match(item_id) or item_id in seen:
            raise PackError(f"index item id {item_id!r} is invalid or repeated")
        seen.add(item_id)
        kind = item.get("item_kind")
        if kind not in ITEM_KINDS or item.get("capture") not in CAPTURES[kind]:
            raise PackError(f"index item {item_id}: kind {kind!r} with capture {item.get('capture')!r} is not known")
        if item.get("folder") != f"{FOLDER_PREFIX[kind]}/{item_id}":
            raise PackError(f"index item {item_id}: folder {item.get('folder')!r} must be "
                            f"{FOLDER_PREFIX[kind]}/{item_id}")
        names = [entry["file"] for key in ("mbf", "identity_file", "writer_report", "run_line", "input")
                 if isinstance(entry := item.get(key), dict) and "file" in entry]
        names += [variant["mbf"]["file"] for variant in item.get("variants") or []]
        names += list(item.get("bring_back") or [])
        if "stem" in item:
            names.append(item["stem"])
        for name in names:
            check_name(name, f"index item {item_id}")


def pack_inputs(item: Mapping[str, Any]) -> set[str]:
    """The names of the files the pack put in an item's folder (run_line.txt excepted: it comes back filled)."""
    names = {"case.json"}
    for key in ("mbf", "identity_file", "writer_report", "input"):
        if isinstance(item.get(key), dict):
            names.add(item[key]["file"])
    for variant in item.get("variants") or []:
        names.add(variant["mbf"]["file"])
    return names


def _regular_files(folder: Path) -> dict[str, Path]:
    """The regular files of a returned folder by name, from the directory listing alone (nothing is opened); a link,
    a subfolder or a name that is not plain is refused."""
    found = {}
    for entry in sorted(os.scandir(folder), key=lambda e: e.name):
        if entry.is_symlink() or not entry.is_file(follow_symlinks=False):
            raise PackError(f"{folder.name}/{entry.name} is a symbolic link or not a regular file; refused")
        check_name(entry.name, f"{folder.name}")
        found[entry.name] = Path(entry.path)
    return found


def returned(item: Mapping[str, Any], files: Mapping[str, Path]) -> bool:
    """Whether anything came back, from names and file-system sizes alone (nothing is read): a file the pack did not
    put there, or a run line whose size differs from the template's (filling it in adds text)."""
    if set(files) - pack_inputs(item) - {"run_line.txt"}:
        return True
    run_line = files.get("run_line.txt")
    template = (item.get("run_line") or {}).get("template_bytes")
    return run_line is not None and template is not None and os.lstat(run_line).st_size != template


def _verify_inputs(item: Mapping[str, Any], files: Mapping[str, Path]) -> str | None:
    """Why a file the pack provided (the MBF, the identity file, a substituted MBF) is not the one the index records,
    or None. These are SWBPIPE's own files; nothing that came back from the program is read here."""
    checks = [("mbf", item.get("mbf"))]
    if item.get("identity_file"):
        checks.append(("identity file", item["identity_file"]))
    for variant in item.get("variants") or []:
        if not variant["mbf"].get("same_as_base"):
            checks.append((f"variant {variant['label']} MBF", variant["mbf"]))
    for what, entry in checks:
        if not entry:
            continue
        name = entry["file"]
        if name not in files:
            return f"{what.replace(' ', '_')}_missing: {name} is not in the returned folder"
        if stream_sha256(files[name])[1] != entry["sha256"]:
            return f"{what.replace(' ', '_')}_changed: {name} no longer has the SHA-256 the index records"
    return None


def sealed_in_exchange(tree: Tree, item: Mapping[str, Any]) -> str | None:
    """Why an item is sealed in the exchange folder, or None: its id has a sealed folder, or any capture record there
    holds its MBF's hash. Only this module's own capture records are read, never a payload."""
    item_id = item["item_id"]
    if tree.exists(("cases", item_id, SEALED)):
        return f"cases/{item_id}/{SEALED} exists"
    mbf = (item.get("mbf") or {}).get("sha256")
    for case_id in tree.listdir(("cases",)):
        if not CASE_ID_PATTERN.match(case_id):
            continue
        for attempt in tree.listdir(("cases", case_id, SEALED)):
            if not NAME_PATTERN.match(attempt):
                continue
            record = tree.read_json(("cases", case_id, SEALED, attempt, CAPTURE_RECORD))
            if isinstance(record, dict) and mbf and record.get("mbf_sha256") == mbf:
                return f"a sealed capture of the same MBF is recorded at cases/{case_id}/{SEALED}/{attempt}"
    return None


def _listing_sha(listing: Mapping[str, Any]) -> str:
    return _sha256(json.dumps(listing, sort_keys=True).encode("utf-8"))


def _attempts(tree: Tree, base: tuple[str, ...], record_name: str) -> tuple[list[tuple[str, dict]], list[str]]:
    """The complete attempts below `base` (each with its record) and the partial ones (a folder without its record)."""
    complete, partial = [], []
    for name in tree.listdir(base):
        if not NAME_PATTERN.match(name):
            continue
        record = tree.read_json(base + (name, record_name))
        if isinstance(record, dict):
            complete.append((name, record))
        else:
            partial.append(name)
    return complete, partial


def _reserve_attempt(tree: Tree, base: tuple[str, ...], listing_sha: str) -> tuple[str, ...]:
    """A new attempt folder below `base`, named by the UTC time (to the microsecond) and the returned files' hash."""
    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%S%fZ")
    for suffix in [""] + [f"-{n}" for n in range(1, 100)]:
        try:
            return tree.reserve_dir(base + (f"{stamp}-{listing_sha[:12]}{suffix}",))
        except ExistingPath:
            continue
    raise ExistingPath(f"no free attempt name below {'/'.join(base)}")


def _hash_listing(files: Mapping[str, Path], inputs: set[str]) -> dict[str, dict[str, Any]]:
    """One hashing pass over each file, nothing written."""
    return {name: dict(zip(("bytes", "sha256"), stream_sha256(path)))
            | {"role": "pack_input" if name in inputs else "returned"} for name, path in sorted(files.items())}


def _store_attempt(tree: Tree, base: tuple[str, ...], record_name: str, files: Mapping[str, Path],
                   inputs: set[str], record: Mapping[str, Any], *, refuse_different: bool,
                   same_status: str) -> dict[str, Any]:
    """Store `files` as a new attempt below `base` (payload under `payload/`, the record beside it, written last),
    each returned file read once. When a complete attempt exists, the one pass is a hash with nothing written: the
    same files give `same_status`; different files are refused (`refuse_different`) or stored as a new attempt."""
    complete, partial = _attempts(tree, base, record_name)
    if complete:
        listing = _hash_listing(files, inputs)
        for name, previous in complete:
            if previous.get("files") == listing:
                return {"status": same_status, "attempt": "/".join(base + (name,))}
        if refuse_different:
            return {"status": "refused", "reason": f"{'/'.join(base)} already holds a complete attempt with "
                                                   "different files; nothing was written"}
    attempt = _reserve_attempt(tree, base, _sha256(json.dumps(sorted(files)).encode("utf-8")))
    tree.reserve_dir(attempt + (PAYLOAD,))
    listing = {name: tree.copy(attempt + (PAYLOAD, name), path)
               | {"role": "pack_input" if name in inputs else "returned"} for name, path in sorted(files.items())}
    tree.record(attempt + (record_name,), {
        **record, "attempt": "/".join(attempt), "stored_at": _now(), "files": listing,
        "payload": f"{'/'.join(attempt)}/{PAYLOAD}", "listing_sha256": _listing_sha(listing),
        "earlier_partial_attempts": ["/".join(base + (name,)) for name in partial],
        "boundary_note": BOUNDARY_NOTE})
    return {"status": "stored", "attempt": "/".join(attempt), "files": len(listing),
            "earlier_partial_attempts": len(partial)}


def seal_capture(folder: Path, item: Mapping[str, Any], tree: Tree, *, pack_index_sha256: str) -> dict[str, Any]:
    """Seal one capture: every file of `folder` copied under the name it came back with to
    `cases/<id>/sealed/<attempt>/payload/`, each read once by the copy that hashes it, and `capture_record.json`
    written beside the payload, last. Nothing reads the content beyond that pass; no parser is reachable from here.
    A re-intake hashes once and writes nothing; a different capture is refused."""
    files = _regular_files(folder)
    problem = _verify_inputs(item, files)
    if problem:
        return {"status": "refused", "reason": problem}
    outcome = _store_attempt(
        tree, ("cases", item["item_id"], SEALED), CAPTURE_RECORD, files, pack_inputs(item), {
            "document_kind": CAPTURE_KIND, "item_id": item["item_id"], "status": "sealed",
            "item_kind": item["item_kind"], "capture": item["capture"],
            "reason": item.get("sealed_reason"), "pack_index_sha256": pack_index_sha256,
            "mbf_sha256": item["mbf"]["sha256"], "model_hash": (item.get("source") or {}).get("model_hash"),
            "swbpipe_request": item.get("swbpipe_request"),
            "rule": ("Sealed: no agent opens, parses, decodes, prints, summarises or compares the payload. This "
                     "change has no unsealing command, and every route that parses or screens a result refuses a "
                     "sealed capture; unsealing is a later change with its own review.")},
        refuse_different=True, same_status="already_sealed")
    if outcome["status"] == "refused":
        outcome["reason"] += "; a sealed capture is never replaced"
    return outcome


def store_outputs(folder: Path, item: Mapping[str, Any], tree: Tree, base: tuple[str, ...], *,
                  names: Iterable[str] | None = None, designation: Mapping[str, Any] | None = None) -> dict[str, Any]:
    """Store and hash what came back for a format sample or an observation item, in one pass per file, under a new
    attempt's `payload/`; never parse, recognise or interpret it. A sample's record keeps its designation and model
    file hash."""
    files = _regular_files(folder)
    wanted = sorted(set(names) if names is not None else set(files))
    present = {name: files[name] for name in wanted if name in files}
    missing = [name for name in wanted if name not in files]
    outcome = _store_attempt(tree, base, STORED_RECORD, present, pack_inputs(item), {
        "document_kind": STORED_KIND, "item_id": item["item_id"], "item_kind": item["item_kind"],
        "designation": designation, "missing": missing,
        "rule": "stored and hashed only; not parsed, recognised, screened, compared or interpreted"},
        refuse_different=True, same_status="already_stored")
    return outcome | {"missing": missing}


def parse_run_line(path: Path) -> dict[str, Any]:
    """The person's run line: each `key: value` line, with the route normalised."""
    fd = os.open(path, _READ_FLAGS)
    try:
        data = b""
        while chunk := os.read(fd, _CHUNK):
            data += chunk
    finally:
        os.close(fd)
    text = data.decode("utf-8-sig", errors="replace")
    fields: dict[str, str] = {}
    for line in text.splitlines():
        key, sep, value = line.partition(":")
        if sep:
            fields[key.strip().lower()] = value.strip()
    route_key = "route used (guest_script, manual_batch or manual_print)"
    route = fields.get(route_key, "").strip().lower().replace(" ", "_")
    return {"present": True, "text": text, "fields": fields, "route": route if route in ROUTES else None,
            "route_as_written": fields.get(route_key) or None}


def _guest_result(stem: str, files: Mapping[str, Path], mbf_sha: str) -> dict[str, Any]:
    """Read the guest script's report `<stem>.guest.json`: the MBF hash it reports, its status and print file."""
    path = files.get(f"{stem}.guest.json")
    if path is None:
        return {"present": False}
    fd = os.open(path, _READ_FLAGS)
    try:
        data = b""
        while chunk := os.read(fd, _CHUNK):
            data += chunk
    finally:
        os.close(fd)
    log = last_json_object(data.decode("utf-8-sig", errors="replace"))
    if log is None:
        return {"present": True, "log": None, "route": "no_guest_json"}
    reported = log.get("mbf_sha256")
    reported = reported.lower() if isinstance(reported, str) else None
    status = log.get("status")
    csv_path = log.get("csv_path")
    name = csv_path.replace("/", "\\").split("\\")[-1] if isinstance(csv_path, str) else ""
    result = {"present": True, "log": log, "guest_mbf_sha256": reported, "csv_path": csv_path}
    if reported != mbf_sha:
        return result | {"route": "input_hash_mismatch"}
    if status in CSV_ROUTES:
        allowed = (f"{stem}.csv", f"{stem}.txt", f"{stem}_results.csv")
        if name in allowed and name in files:
            return result | {"route": status, "file": name}
        return result | {"route": "csv_not_collected"}
    return result | {"route": str(status or "unknown_guest_status")}


def _manual_result(stem: str, route: str, files: Mapping[str, Path]) -> tuple[str | None, str]:
    if route == "manual_print":
        name = f"{stem}.print.csv"
        return (name, "") if name in files else (None, f"{name} is not in the returned folder")
    candidates = [name for name in (f"{stem}.csv", f"{stem}.txt") if name in files]
    if len(candidates) == 2:
        return None, "print_file_ambiguous: both a .csv and a .txt print came back"
    return (candidates[0], "") if candidates else (None, f"no {stem}.csv or {stem}.txt in the returned folder")


def classify_case(folder: Path, item: Mapping[str, Any], tree: Tree) -> dict[str, Any]:
    """How an ordinary (screened, not sealed) case came back: the inputs' hashes, the route, the binding class and
    the result file. Writes nothing. Reads the run line and the guest report, so it is never called for a sealed
    capture or a format sample."""
    if item.get("item_kind") != "benchmark_case" or item.get("capture") != "screened":
        raise PackError(f"{item.get('item_id')}: only a screened benchmark case is classified")
    if (sealed := sealed_in_exchange(tree, item)):
        return {"status": "refused", "reason": f"sealed in the exchange folder ({sealed}); it is not opened"}
    stem, mbf_sha = item["stem"], item["mbf"]["sha256"]
    files = _regular_files(folder)
    problem = _verify_inputs(item, files)
    if problem:
        return {"status": "refused", "reason": problem}
    run_line = parse_run_line(files["run_line.txt"]) if "run_line.txt" in files else {"present": False}
    guest = _guest_result(stem, files, mbf_sha)
    notes = []
    if guest["present"]:
        sitting_route, binding = "guest_script", GUEST_BINDING
        route, result_name = guest["route"], guest.get("file")
        if run_line.get("route") and run_line["route"] != "guest_script":
            notes.append(f"the run line names {run_line['route']}, but a guest report came back; the guest report "
                         "decides")
    elif run_line.get("route") in SITTING_ROUTES:
        sitting_route, binding = run_line["route"], ATTESTED_BINDING
        result_name, reason = _manual_result(stem, sitting_route, files)
        route = SITTING_ROUTES[sitting_route] if result_name else (reason.split(":")[0] if reason.startswith(
            "print_file_ambiguous") else "result_missing")
        if reason:
            notes.append(reason)
    else:
        sitting_route, binding, route, result_name = None, None, "route_not_stated", None
        notes.append("no guest report came back and the run line names no route")
    if result_name is not None and result_name.endswith(".txt"):
        problem = _harness._print_layout_problem(files[result_name])
        if problem:
            route, result_name = "print_file_unrecognised", None
            notes.append(problem)
    return {"status": "classified", "files": files, "run_line": run_line, "guest": guest, "notes": notes,
            "sitting_route": sitting_route, "binding": binding, "route": route, "result_name": result_name}


def _parser_counts(path: Path) -> dict[str, int]:
    from core.handoff.caepipe_external.print_csv import read_caepipe_print_csv
    parsed = read_caepipe_print_csv(path)
    return {"sections_parsed": len(parsed["sections_parsed"]),
            "error_diagnostics": sum(1 for d in parsed["diagnostics"] if d["severity"] == "error")}


def check_case(folder: Path, item: Mapping[str, Any], tree: Tree) -> dict[str, Any]:
    """The dry run's check of one ordinary case, in place and writing nothing; a case sealed in the exchange folder
    `tree` names is refused before anything is read."""
    found = classify_case(folder, item, tree)
    if found["status"] == "refused":
        return found
    name = found["result_name"]
    parser = _parser_counts(found["files"][name]) if name is not None else None
    return {"status": "checked", "route": found["route"], "binding": found["binding"], "parser": parser,
            "reason": "; ".join(found["notes"]) or None}


def intake_case(folder: Path, item: Mapping[str, Any], tree: Tree, *, pack_index_sha256: str) -> dict[str, Any]:
    """Take one ordinary case in as a new run folder `cases/<id>/sitting_runs/<attempt>/`, adding new files only:
    every returned file under `returned/`, `caepipe_in/model.mbf`, `caepipe_out/` (the print as `model.csv` or
    `model.txt`, the solved model and results, the guest report and `run_record.json`), and `sitting_intake.json`
    written last; then a new `cases/<id>/run_selection/<attempt>.json`."""
    item_id, stem, mbf_sha = item["item_id"], item["stem"], item["mbf"]["sha256"]
    found = classify_case(folder, item, tree)
    if found["status"] == "refused":
        return found
    files, run_line, guest, notes = found["files"], found["run_line"], found["guest"], found["notes"]
    sitting_route, binding, route, result_name = (found["sitting_route"], found["binding"], found["route"],
                                                  found["result_name"])
    inputs = pack_inputs(item)
    listing = _hash_listing(files, inputs)
    base = ("cases", item_id, SITTING_RUNS)
    complete, partial = _attempts(tree, base, INTAKE_RECORD)
    for name, previous in complete:
        if previous.get("returned_files") == listing:
            # An intake interrupted after its record and before its selection: the selection is added, new.
            if not tree.exists(("cases", item_id, RUN_SELECTION, f"{name}.json")):
                _select_run(tree, item_id, name, bound=bool(previous.get("bound")), route=previous.get("route"))
            return {"status": "already_placed", "route": previous.get("route"), "binding": previous.get("binding"),
                    "run": "/".join(base + (name,))}
    attempt = _reserve_attempt(tree, base, _listing_sha(listing))
    run = "/".join(attempt)
    tree.reserve_dir(attempt + (RETURNED,))
    kept = {name: tree.copy(attempt + (RETURNED, name), path) for name, path in sorted(files.items())}
    tree.reserve_dir(attempt + ("caepipe_in",))
    tree.copy(attempt + ("caepipe_in", "model.mbf"), files[item["mbf"]["file"]])
    tree.reserve_dir(attempt + ("caepipe_out",))
    out = attempt + ("caepipe_out",)
    collected: dict[str, dict[str, Any]] = {}
    observed = None
    if result_name is not None:
        observed = "model.txt" if result_name.endswith(".txt") else "model.csv"
        collected[observed] = tree.copy(out + (observed,), files[result_name])
    if sitting_route == "guest_script":
        for suffix in ("mod", "res"):
            name = f"{stem}.{suffix}"
            if name in files:
                collected[f"model.{suffix}"] = tree.copy(out + (f"model.{suffix}",), files[name])
        if guest.get("log") is not None:
            collected["guest_log.json"] = tree.create(out + ("guest_log.json",),
                                                      json.dumps(guest["log"], indent=2, sort_keys=True) + "\n")
    parser = _parser_counts(tree.path(out + (observed,))) if observed is not None else None
    retained = {key: {"path": f"{RETURNED}/{item[key]['file']}", "sha256": kept[item[key]["file"]]["sha256"]}
                for key in ("mbf", "identity_file", "writer_report") if item.get(key) and item[key]["file"] in kept}
    record = {
        "document_kind": RUN_RECORD_KIND, "schema_version": "1", "deliverable": "DEL-17-05", "case_id": item_id,
        "configuration_surface": {"environment_variable": ENV_VAR, "config_file": CONFIG_NAME,
                                  "checked": ["sitting pack intake"], "warnings": []},
        "executable_path_as_configured": None,
        ACKNOWLEDGEMENT_KEY: True,
        "mbf": {"path": "caepipe_in/model.mbf", "guest_path": None, "sha256": mbf_sha,
                "bytes": item["mbf"]["bytes"]},
        "mbf_profile": {"profile_id": MBF_SI_PROFILE_ID, "open_questions": list(WRITER_PROFILE_OPEN_QUESTIONS),
                        "run_open_questions": list(RUN_OPEN_QUESTIONS),
                        "basis": "DEL-17-04 ScopeOfWork 'Not decided'; DEL-17-05 REQ-003"},
        "retained_inputs": retained,
        "guest_mbf_sha256": guest.get("guest_mbf_sha256") if sitting_route == "guest_script" else None,
        "input_hash_verified": bool(sitting_route == "guest_script" and guest.get("guest_mbf_sha256") == mbf_sha),
        "guest_script": {"repository_path": GUEST_SCRIPT_REPOSITORY_PATH, "run_from": "sitting pack scripts/"},
        "command_shape": {"batch": BATCH_COMMAND_SHAPE if sitting_route in ("guest_script", "manual_batch") else None,
                          "host_invocation": "none: run by a person in the sitting",
                          "sitting_route": sitting_route},
        "working_directory": f"the person's copy of {item['folder']}",
        "expected_batch_csv_path": f"{stem}.csv",
        "observed_csv_path": {"guest": guest.get("csv_path") or result_name,
                              "host": f"caepipe_out/{observed}" if observed else None},
        "route": route,
        "exit_status": ((guest.get("log") or {}).get("batch") or {}).get("exit_code"),
        "guest_log": guest.get("log"),
        "outputs_collected": sorted(name for name in collected if name.startswith("model")),
        "outputs_sha256": {name: entry["sha256"] for name, entry in sorted(collected.items())
                           if name.startswith("model")},
        "parser": parser,
        "sitting": {"binding": binding, "pack_index_sha256": pack_index_sha256, "part": item["part"], "run": run,
                    "returned_file": result_name, "run_line": run_line.get("fields"), "notes": notes},
        "timestamps": {"ingested": _now()},
        "boundary_note": BOUNDARY_NOTE,
    }
    tree.record(out + ("run_record.json",), record)
    variants = {}
    for variant in item.get("variants") or []:
        vstem = f"{stem}.{variant['label']}"
        names = sorted(name for name in files if name.startswith(vstem + ".") and name != variant["mbf"]["file"])
        vguest = (_guest_result(vstem, files, variant["mbf"]["sha256"]) if variant["kind"] == "substitution"
                  else {"present": False})
        variants[variant["label"]] = {
            "kind": variant["kind"], "returned": [f"{RETURNED}/{name}" for name in names],
            "status": "returned" if names else "not_returned", "mbf_sha256": variant["mbf"]["sha256"],
            "mbf": f"{RETURNED}/{variant['mbf']['file']}", "declaration": {
                key: variant.get(key) for key in ("instruction", "heading", "old_line", "new_line", "note")
                if variant.get(key) is not None},
            "identity_file_of_base": variant.get("identity_file_of_base"),
            "binding": GUEST_BINDING if vguest.get("route") in CSV_ROUTES else ATTESTED_BINDING, "screened": False}
    bound = binding == GUEST_BINDING and route in CSV_ROUTES and observed is not None
    tree.record(attempt + (INTAKE_RECORD,), {
        "document_kind": INTAKE_KIND, "item_id": item_id, "ingested_at": _now(), "route": route,
        "item_kind": item["item_kind"], "capture": item["capture"], "mbf_sha256": mbf_sha,
        "sitting_route": sitting_route, "binding": binding, "bound": bound, "run": run,
        "returned_files": listing, "kept": f"{run}/{RETURNED}", "placed": collected, "parser": parser,
        "run_line": run_line.get("text"), "notes": notes, "variants": variants,
        "earlier_partial_attempts": ["/".join(base + (name,)) for name in partial],
        "pack_index_sha256": pack_index_sha256, "boundary_note": BOUNDARY_NOTE})
    _select_run(tree, item_id, attempt[-1], bound=bound, route=route)
    return {"status": "placed" if observed else "recorded", "route": route, "binding": binding, "run": run,
            "parser": parser, "variants": {k: v["status"] for k, v in variants.items()},
            "earlier_partial_attempts": len(partial)}


def _select_run(tree: Tree, item_id: str, run: str, *, bound: bool, route: Any) -> None:
    """A new file `cases/<id>/run_selection/<run>.json` naming the run a later screen of the case should use."""
    tree.ensure_dir(("cases", item_id, RUN_SELECTION))
    tree.record(("cases", item_id, RUN_SELECTION, f"{run}.json"), {
        "document_kind": SELECTION_KIND, "case_id": item_id, "run": f"{SITTING_RUNS}/{run}",
        "selected_at": _now(), "bound": bound, "route": route,
        "rule": ("Names the run a later screen of this case should use. No existing reader consumes this file yet: "
                 "the offline screen reads only the case's own caepipe_out/, so this run cannot be screened until a "
                 "later change makes it read a selected run and write to new paths."),
        "boundary_note": BOUNDARY_NOTE})


def ingest_pack(pack_dir: Path, root: Path | None, authority: Path, *, selected: Iterable[str] | None = None,
                check: bool = False, emit: Callable[[str], None] = print) -> bool:
    """Take a returned pack in, item by item, in run order. True when no returned item was refused.

    The returned `pack_index.json` must hash equal to the authoritative one, and every designation is taken from the
    authoritative one. With `check` nothing is written; the exchange folder is still needed, and only read, so that a
    case sealed there is never read on this route either."""
    if root is None:
        raise PackError("ingest needs the exchange folder, for --check too: without it a case sealed there could not "
                        "be recognised, and nothing is read")
    pack = outside_work_trees(pack_dir, "ingest")
    index, index_sha = load_authority(authority)
    try:
        returned_sha = stream_sha256(pack / "pack_index.json")[1]
    except OSError as error:
        raise PackError(f"the returned pack has no readable pack_index.json ({error.strerror})") from None
    if returned_sha != index_sha:
        raise PackError("the returned pack_index.json differs from the authoritative index; nothing is taken in")
    tree = Tree(root, "the exchange folder")
    chosen = set(selected) if selected else None
    ok = True
    try:
        for item in index["items"]:
            item_id = item["item_id"]
            if chosen is not None and item_id not in chosen:
                continue
            if item.get("status", "written") != "written":
                emit(f"{item_id}: {_kind_label(item)}; skipped")
                continue
            folder = pack.joinpath(*item["folder"].split("/"))
            try:
                if not folder.is_dir() or folder.is_symlink():
                    raise PackError(f"folder {item['folder']} is missing or a link")
                files = _regular_files(folder)
                outcome = _take_in(folder, item, files, tree, index_sha, check=check)
            except (ContainmentError, PackError) as error:
                outcome = {"status": "refused", "reason": str(error)}
            if outcome["status"] == "refused":
                ok = False
            emit(_status_line(item, outcome))
    finally:
        if tree is not None:
            tree.close()
    return ok


def _take_in(folder: Path, item: Mapping[str, Any], files: Mapping[str, Path], tree: Tree | None, index_sha: str, *,
             check: bool) -> dict[str, Any]:
    """Dispatch one item by the authoritative designation; a sealed state in the exchange folder overrides any
    screened designation and keeps the item from every reader."""
    kind, capture, item_id = item["item_kind"], item["capture"], item["item_id"]
    if kind == "format_observation":
        if not set(item["bring_back"]) & set(files):
            return {"status": "not_returned"}
        if check:
            return {"status": "present; not opened",
                    "missing": [name for name in item["bring_back"] if name not in files]}
        return store_outputs(folder, item, tree, ("observations", item_id), names=item["bring_back"])
    if not returned(item, files):
        return {"status": "not_returned"}
    sealed = sealed_in_exchange(tree, item) if tree is not None else None
    if kind == "format_sample" or capture == "sealed" or sealed:
        problem = _verify_inputs(item, files)
        if problem:
            return {"status": "refused", "reason": problem}
        if check:
            return {"status": "present; not opened" + (f" (sealed: {sealed})" if sealed else "")}
        if kind == "format_sample":
            return store_outputs(folder, item, tree, ("format_samples", item_id),
                                 designation={**(item.get("designation") or {}), "mbf": item["mbf"]})
        if capture == "sealed":
            return seal_capture(folder, item, tree, pack_index_sha256=index_sha)
        return {"status": "refused", "reason": f"sealed in the exchange folder ({sealed}); it is not opened"}
    if check:
        return check_case(folder, item, tree)
    return intake_case(folder, item, tree, pack_index_sha256=index_sha)


def _status_line(item: Mapping[str, Any], outcome: Mapping[str, Any]) -> str:
    line = f"{item['item_id']}: {_kind_label(item)}; {outcome['status']}"
    if outcome.get("route"):
        line += f"; route={outcome['route']}"
    if outcome.get("binding"):
        line += f"; binding={outcome['binding']}"
    if outcome.get("parser"):
        line += (f"; parser read {outcome['parser']['sections_parsed']} section(s), "
                 f"{outcome['parser']['error_diagnostics']} error diagnostic(s)")
    if outcome.get("earlier_partial_attempts"):
        line += f"; earlier partial attempts left in place: {outcome['earlier_partial_attempts']}"
    if outcome.get("missing"):
        line += f"; not returned: {', '.join(outcome['missing'])}"
    if outcome.get("reason"):
        line += f"; {outcome['reason']}"
    return line


# -- command line -----------------------------------------------------------------------------------------------------
def main(argv: list[str] | None = None, environ: Mapping[str, str] = os.environ) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    commands = parser.add_subparsers(dest="command", required=True)
    prepare = commands.add_parser("prepare", help="write a sitting pack outside every Git work tree")
    prepare.add_argument("out_dir", metavar="OUT_DIR")
    prepare.add_argument("--authority", metavar="DIR",
                         help="where the host keeps its copy of the index (default: OUT_DIR.authority beside it)")
    prepare.add_argument("--manifest", default=str(DEFAULT_MANIFEST), help="benchmark manifest (default: the tracked "
                                                                            "validation/caepipe_benchmark/manifest.json)")
    prepare.add_argument("--plan", help="pack plan JSON (default: the built-in sitting_plan_default.json)")
    prepare.add_argument("--observations", help="observation items JSON for the observations part")
    prepare.add_argument("--samples", help="format-sample declaration (default: the tracked declaration)")
    prepare.add_argument("--case", action="append", dest="cases", metavar="ID",
                         help="only this benchmark case (repeatable); format samples and observations stay")
    ingest = commands.add_parser("ingest", help="take a returned pack into the exchange folder")
    ingest.add_argument("pack_dir", metavar="PACK_DIR")
    ingest.add_argument("--authority", required=True, metavar="DIR",
                        help="the authority folder prepare wrote for this pack (the host's own copy of the index)")
    ingest.add_argument("--item", action="append", dest="items", metavar="ID", help="only this item (repeatable)")
    ingest.add_argument("--check", action="store_true",
                        help="write nothing: verify hashes, name each ordinary case's route and binding, and run the "
                             "parser on each ordinary, unsealed case's returned print in place (the dry run's check)")
    args = parser.parse_args(argv)
    try:
        if args.command == "prepare":
            out = Path(os.path.abspath(args.out_dir))
            authority = Path(os.path.abspath(args.authority)) if args.authority else out.with_name(
                out.name + ".authority")
            files = build_pack(Path(args.manifest), load_plan(Path(args.plan) if args.plan else None),
                               load_observations(Path(args.observations) if args.observations else None), args.cases,
                               samples=load_samples(Path(args.samples) if args.samples else None))
            write_pack(out, files, authority)
            index = json.loads(files["pack_index.json"])
            print(f"pack written: {len(files)} files, {len(index['items'])} items; authority: {authority}")
            return 0
        config = resolve_harness_config(environ, require_vm_tools=False)
        if isinstance(config, HarnessSkip):
            print(f"CAEPIPE harness skipped: {config.reason}; nothing taken in or checked (the exchange folder is "
                  "needed to recognise a sealed case)")
            return 0
        root = config.exchange_root.resolve()
        return 0 if ingest_pack(Path(args.pack_dir), root, Path(args.authority), selected=args.items,
                                check=args.check) else 1
    except (OSError, KeyError, ValueError, ContainmentError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
