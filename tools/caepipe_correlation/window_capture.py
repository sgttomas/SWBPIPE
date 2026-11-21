"""The window print route of the benchmark comparison (DEL-17-05): a print made at the target program's window.

Some benchmark runs cannot be made by the batch round trip: a member is entered at the program's window, or a setting
is made there, so the committed sitting pack gives a window instruction (import the run's MBF, make the entries, save
the model under the run's name, analyse, print all results to `<stem>.print.csv`). This module stores the folder such
a run left, under the same protection as any stored print, and compares its print offline where the run's declaration
lets it be compared.

Store (`store_capture`). Before anything is written it checks that the run is declared with a window instruction;
that the folder is a directory holding only regular files (each opened without following a link and checked on the
open file), none named as the window record; that it holds the print, the saved model, the run line, the imported MBF
and the instruction; that the imported MBF is byte for byte the run's file as the writer produces it in this checkout
(else pack_file_differs, with both digests); and that the instruction is the manifest's text with its declared digest.
The folder itself is never changed. Every regular file in it is copied, never moved, into a directory built under a
temporary name beside `cases/<run id>/caepipe_out/window/<capture id>/` (the capture id the first 12 hex digits of the
print's SHA-256), with `window_record.json` written last, and the directory is then renamed into place: a capture
appears only complete, an interrupted one is never listed and never bars a retry, and an existing one is never written
into. The record states each file's SHA-256, size and role, the checkout the command ran in and whether the writer's
file was a commit's, the time, the instruction's digest, the print's parse status and the sections the parser did not
read. A print that does not parse is stored all the same; its comparison is refused by name.

Compare (`compare_capture`). Offline, at any commit, and only where the run's manifest declaration lets it be compared
(stated_bases.declared_compared: a dynamics capture, declared not screened, is a reference and is never compared). The
record's run identity, the stored files' digests, the run's file at this checkout and the instruction's digest are
checked again, and the print is parsed again by the current parser; SWBPIPE's side is solved afresh in its own solve
folder (for a hand-entry run, `benchmark_hand_entry.hand_entry_document`: the base with the member's support records,
its closed forms, its declared inexact inputs and screening classes); the plain comparison is created beside it,
naming the capture it read, and only then are the stated bases formed and written beside it. Several captures of one
run may be stored; each is compared on its own, and a batch capture of the same run is a separate record.

What a window capture cannot prove that a batch capture can (`WINDOW_LIMITS`), stated in the record and in every
comparison: nothing hashes the file the program opened, shows that the entries made were the instruction's, shows
that the print came from an analysis of the saved model, or shows the window's settings. The saved model and every
other file kept are not examined.
"""

from __future__ import annotations

import copy
import errno
import hashlib
import json
import os
import re
import shutil
import stat
import tempfile
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Callable, Mapping

from core.handoff.caepipe_external import parse_caepipe_print_bytes
from core.handoff.caepipe_external.benchmark import build_benchmark_record
from core.handoff.caepipe_external.harness import contained, contained_dir
from core.handoff.caepipe_mbf import render_caepipe_mbf_si
from tools.caepipe_correlation import benchmark_hand_entry, benchmark_models, stated_bases

WINDOW_RECORD = "window_record.json"
WINDOW_RECORD_KIND = "openpipestress.caepipe_benchmark.window_capture"
STATED_BASES_RECORD = "stated_bases.json"
STATED_BASES_KIND = "openpipestress.caepipe_benchmark.stated_bases"
WINDOW_ROUTE = "window_capture"
RUN_LINE = "run_line.txt"
INCOMING = ".incoming-"
CAPTURE_ID = re.compile(r"[0-9a-f]{12}\Z")
REQUIRED = ("print", "saved_model", "run_line", "imported_mbf", "instruction")
BINDING = ("window: the SHA-256 of the run's MBF in the capture folder, equal to the writer's file in the checkout the "
           "capture was stored and compared in, and of the instruction, the saved model and the print; not of the "
           "file the program opened")
WINDOW_LIMITS = ("a window capture binds the print to its run by the digests of the files the folder holds. Unlike a "
                 "batch capture, nothing hashes the file the program opened, shows that the entries made were the "
                 "instruction's, shows that the print came from an analysis of the saved model, or shows the "
                 "window's settings; the saved model and every other file kept are not examined")
KEPT_NOT_EXAMINED = "kept, not examined"
LIMIT_STOP_PAGE = ("https://docs.sstcae.com/tech_manual/limit_stop.htm (technical reference, \"Limit Stop\"), read "
                   "2026-11-21: \"This friction force is displayed in results under Limit stop support loads.\"")
UNREAD_MEMBER = {
    "nonlinear": ("the parser does not read the print's limit stop support loads (the section the Limit Stop page "
                  "names as where a limit stop's friction force is displayed: " + LIMIT_STOP_PAGE + ")"),
    "constant_effort_support": ("the parser does not read the print's hanger loads; the input "
                                "constant_support_load stays named"),
}
UNREAD_OTHER = "the parser does not read the printed load of a member entered at the window"
OUTSIDE_THE_MODEL = ("the source folder is opened without following a link at the folder itself and at each file in "
                     "it; a link among the folder's ancestors is outside what this route checks")


class CaptureRefused(ValueError):
    """The capture is not stored or not compared; `route` names why."""

    def __init__(self, route: str, reason: str):
        super().__init__(f"{route}: {reason}")
        self.route, self.reason = route, reason


def _sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _now() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds").replace("+00:00", "Z")


def is_window_run(entry: Mapping[str, Any]) -> bool:
    return bool(entry.get("capture_only") and entry.get("window_instruction"))


def run_mbf(entry: Mapping[str, Any]) -> tuple[Any, str]:
    """The writer's result for the run's base and the MBF the run carries, in this checkout."""
    mbf = render_caepipe_mbf_si(entry["model"], title=entry["model_id"],
                                reference_temperature_c=entry.get("reference_temperature_c", 20.0),
                                piping_code=entry["piping_code"])
    if mbf.status != "written":
        raise CaptureRefused("mbf_refused", "the writer refuses the run's base in this checkout")
    return mbf, benchmark_models.run_mbf_text(entry, mbf.text)


def _names(entry: Mapping[str, Any]) -> dict[str, str]:
    run = (entry.get("hand_entry") or {}).get("run") or {}
    run_id = entry["run_id"]
    return {"print": run.get("expected_print_file") or f"{run_id}.print.csv",
            "saved_model": run.get("expected_saved_file") or f"{run_id}.mod",
            "run_line": RUN_LINE, "imported_mbf": f"{run_id}.mbf",
            "instruction": f"{run_id}.window-instruction.txt"}


def checkout(identity: Mapping[str, Any]) -> dict[str, Any]:
    """The checkout a step ran in, and whose file the writer's file is: a commit's only when the tree was clean."""
    commit, clean = identity.get("source_commit"), identity.get("source_tree_clean")
    if commit and clean is True:
        writer = f"the writer's file at commit {commit} (the checkout was clean)"
    elif commit and clean is False:
        writer = (f"the writer's file of the working tree at commit {commit}, which was not clean: the working "
                  "tree's file, not that commit's")
    else:
        writer = "the writer's file of a checkout whose commit or cleanliness Git could not state"
    return {"source_commit": commit, "source_tree_clean": clean, "writer_file": writer}


def parse_status(print_bytes: bytes) -> tuple[dict[str, Any], dict[str, Any] | None]:
    """(status, parsed): `parsed` or `failed` with the parser's reason, and the sections it did not read by name."""
    try:
        parsed = parse_caepipe_print_bytes(print_bytes)
    except Exception as error:  # a parser that is behind the print never loses the capture
        return {"status": "failed", "reason": f"{type(error).__name__}: {error}", "sections_not_read": []}, None
    errors = [item for item in parsed.get("diagnostics") or [] if item.get("severity") == "error"]
    not_read = [{"title": item.get("title"), "reason": item.get("reason")} for item in parsed.get("sections_skipped") or []]
    if errors or not parsed.get("sections_parsed"):
        reason = "; ".join(str(item.get("message")) for item in errors) or "no section was parsed"
        return {"status": "failed", "reason": reason, "sections_not_read": not_read}, parsed
    return {"status": "parsed", "sections_parsed": [item.get("title") for item in parsed["sections_parsed"]],
            "sections_not_read": not_read}, parsed


def capture_dir(root: Path, run_id: str, capture_id: str) -> Path:
    return root / "cases" / run_id / "caepipe_out" / "window" / capture_id


def _read_regular(dir_fd: int, name: str, route: str) -> bytes:
    """A regular file in an open directory, opened without following a link and checked on the open file."""
    try:
        descriptor = os.open(name, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK, dir_fd=dir_fd)
    except OSError as error:
        raise CaptureRefused(route, f"{name} cannot be opened as a regular file ({error.strerror or error}); only "
                                    "regular files are read, never through a symbolic link") from None
    try:
        if not stat.S_ISREG(os.fstat(descriptor).st_mode):
            raise CaptureRefused(route, f"{name} is not a regular file; only regular files are read")
        chunks = []
        while True:
            chunk = os.read(descriptor, 1 << 20)
            if not chunk:
                return b"".join(chunks)
            chunks.append(chunk)
    finally:
        os.close(descriptor)


def _open_dir(path: Path, route: str, what: str) -> int:
    try:
        return os.open(path, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
    except OSError as error:
        raise CaptureRefused(route, f"{what} is not a directory, or is a symbolic link ({error.strerror or error})") \
            from None


def _read_folder(folder: Path) -> dict[str, bytes]:
    """Every file of a source folder: regular files only, none named as the window record."""
    dir_fd = _open_dir(Path(folder), "capture_folder", "the capture folder")
    try:
        files: dict[str, bytes] = {}
        for item in sorted(os.scandir(dir_fd), key=lambda found: found.name):
            if item.name == WINDOW_RECORD:
                raise CaptureRefused("reserved_name", f"the folder holds a file named {WINDOW_RECORD}, the name of "
                                                      "the record this route writes; it is not stored")
            files[item.name] = _read_regular(dir_fd, item.name, "capture_folder")
        return files
    finally:
        os.close(dir_fd)


def _stored_print_sha(root: Path, target: Path) -> str | None:
    try:
        dir_fd = _open_dir(contained(root, target), "capture_stored", "the stored capture")
    except (CaptureRefused, OSError, ValueError):
        return None
    try:
        record = json.loads(_read_regular(dir_fd, WINDOW_RECORD, "capture_stored").decode("utf-8"))
        return str(record["files"][record["print"]]["sha256"])
    except (CaptureRefused, OSError, ValueError, KeyError, TypeError):
        return None
    finally:
        os.close(dir_fd)


def _already_stored(root: Path, target: Path, run_id: str, capture_id: str, print_sha: str) -> CaptureRefused:
    stored = _stored_print_sha(root, target)
    if stored == print_sha:
        return CaptureRefused("capture_stored", f"capture {capture_id} of {run_id} is already stored")
    return CaptureRefused("capture_id_collision", f"a capture {capture_id} of {run_id} is stored for another print "
                                                  f"(its print's SHA-256 is {stored or 'not readable'}; this print's "
                                                  f"is {print_sha}); nothing is written")


def store_capture(entry: Mapping[str, Any], root: Path, folder: Path, *,
                  identity: Mapping[str, Any]) -> tuple[str, dict[str, Any]]:
    """Store one capture folder of a window run; (capture id, record). Refuses (CaptureRefused) before anything is
    written unless the run is declared with a window instruction, the folder holds the required files as regular
    files, its imported MBF is the writer's file in this checkout and its instruction is the manifest's."""
    if not is_window_run(entry):
        raise CaptureRefused("not_a_window_run", f"{entry.get('run_id') or entry.get('model_id')} is not declared "
                                                 "with a window instruction")
    root = Path(root).resolve()
    files = _read_folder(Path(folder))
    names = _names(entry)
    missing = [names[role] for role in REQUIRED if names[role] not in files]
    if missing:
        raise CaptureRefused("capture_incomplete", f"the folder lacks {', '.join(missing)}")
    _, mbf_text = run_mbf(entry)
    writer_sha = _sha256(mbf_text.encode("utf-8"))
    imported_sha = _sha256(files[names["imported_mbf"]])
    if imported_sha != writer_sha:
        raise CaptureRefused("pack_file_differs", f"the imported file's SHA-256 is {imported_sha}; the writer's file "
                                                  f"for this run is {writer_sha}, "
                                                  f"{checkout(identity)['writer_file']}")
    instruction = entry["window_instruction"]
    if _sha256(instruction["text"].encode("utf-8")) != instruction["sha256"]:
        raise CaptureRefused("instruction_differs", "the manifest's instruction text does not have its declared digest")
    instruction_sha = _sha256(files[names["instruction"]])
    if instruction_sha != instruction["sha256"]:
        raise CaptureRefused("instruction_differs", f"the folder's window instruction has SHA-256 {instruction_sha}; "
                                                    f"the manifest's is {instruction['sha256']}")
    print_bytes = files[names["print"]]
    print_sha = _sha256(print_bytes)
    capture_id = print_sha[:12]
    target = capture_dir(root, entry["run_id"], capture_id)
    parent = contained_dir(root, target.parent)
    if os.path.lexists(target):
        raise _already_stored(root, target, entry["run_id"], capture_id, print_sha)
    status, _ = parse_status(print_bytes)
    roles = {names[role]: role for role in REQUIRED}
    record = {
        "document_kind": WINDOW_RECORD_KIND, "deliverable": "DEL-17-05", "route": WINDOW_ROUTE,
        "run_id": entry["run_id"], "model_id": entry["model_id"], "series": entry.get("series_run"),
        "capture_id": capture_id, "stored": _now(), "stored_at": dict(identity), "checkout": checkout(identity),
        "source_folder_name": Path(folder).name,
        "files": {name: {"sha256": _sha256(data), "bytes": len(data), "role": roles.get(name, KEPT_NOT_EXAMINED)}
                  for name, data in sorted(files.items())},
        "print": names["print"], "saved_model": names["saved_model"], "instruction": names["instruction"],
        "imported_mbf": names["imported_mbf"],
        "run_line_text": files[names["run_line"]].decode("utf-8", "replace"),
        "mbf_sha256": writer_sha, "window_instruction_sha256": instruction_sha,
        "parse": status, "binding": BINDING, "limits": WINDOW_LIMITS,
        "examined": "the print is parsed and compared; the saved model and every other file are kept, not examined",
    }
    incoming = Path(tempfile.mkdtemp(prefix=f"{INCOMING}{capture_id}-", dir=contained(root, parent)))
    try:
        incoming_fd = _open_dir(contained(root, incoming), "capture_folder", "the incoming capture")
        try:
            for name, data in [*sorted(files.items()), (WINDOW_RECORD, (json.dumps(record, indent=2, sort_keys=True)
                                                                        + "\n").encode("utf-8"))]:
                descriptor = os.open(name, os.O_CREAT | os.O_EXCL | os.O_WRONLY | os.O_NOFOLLOW, 0o600,
                                     dir_fd=incoming_fd)
                with os.fdopen(descriptor, "wb") as handle:
                    handle.write(data)
                    handle.flush()
                    os.fsync(handle.fileno())
        finally:
            os.close(incoming_fd)
        try:
            # Renamed into place complete; a directory already there (made by another store) is never written into.
            os.rename(contained(root, incoming), contained(root, target))
        except OSError as error:
            if error.errno in (errno.EEXIST, errno.ENOTEMPTY, errno.EISDIR):
                raise _already_stored(root, target, entry["run_id"], capture_id, print_sha) from None
            raise
    finally:
        if os.path.lexists(incoming):
            shutil.rmtree(incoming)  # this store's own incomplete copy; never a stored capture
    return capture_id, record


def stored_captures(root: Path, run_id: str) -> list[str]:
    """The complete captures of a run: directories named by a capture id holding a window record. A directory being
    built (or left by an interrupted store) has another name and is not listed."""
    folder = Path(root).resolve() / "cases" / run_id / "caepipe_out" / "window"
    if folder.is_symlink() or not folder.is_dir():
        return []
    return sorted(path.name for path in folder.iterdir()
                  if CAPTURE_ID.match(path.name) and not path.is_symlink() and path.is_dir()
                  and not (path / WINDOW_RECORD).is_symlink() and (path / WINDOW_RECORD).is_file())


def _stored(root: Path, run_id: str, capture_id: str) -> tuple[int, dict[str, Any]]:
    """The open capture directory and its record, checked for this run; the caller closes the directory."""
    if not CAPTURE_ID.match(str(capture_id)):
        raise CaptureRefused("no_window_record", f"{capture_id!r} is not a capture id")
    try:
        path = contained(root, capture_dir(root, run_id, capture_id))
    except Exception as error:  # a link inside the exchange folder, or a name that leaves it
        raise CaptureRefused("no_window_record", f"capture {capture_id} of {run_id}: {error}") from None
    dir_fd = _open_dir(path, "no_window_record", f"capture {capture_id} of {run_id}")
    try:
        try:
            record = json.loads(_read_regular(dir_fd, WINDOW_RECORD, "no_window_record").decode("utf-8"))
        except ValueError:
            raise CaptureRefused("no_window_record", f"capture {capture_id} of {run_id} has no readable window "
                                                     "record") from None
        if not isinstance(record, dict) or record.get("document_kind") != WINDOW_RECORD_KIND \
                or record.get("capture_id") != capture_id:
            raise CaptureRefused("no_window_record", f"capture {capture_id} of {run_id} has no valid window record")
        if record.get("run_id") != run_id:
            raise CaptureRefused("record_run_differs", f"capture {capture_id}'s record names run "
                                                       f"{record.get('run_id')!r}, not {run_id}")
    except BaseException:
        os.close(dir_fd)
        raise
    return dir_fd, record


def read_record(root: Path, run_id: str, capture_id: str) -> dict[str, Any]:
    dir_fd, record = _stored(Path(root).resolve(), run_id, capture_id)
    os.close(dir_fd)
    return record


def solved_document(entry: Mapping[str, Any]) -> dict[str, Any]:
    """The document SWBPIPE solves for a compared window run, with what the run declares: for a hand-entry run the
    model with the member and its third values as closed forms; otherwise the run's own model with its third values;
    in both, the declared inexact inputs in place of the emptied ones."""
    if entry.get("run_kind") == "hand_entry":
        document = benchmark_hand_entry.hand_entry_document(dict(entry))
    else:
        document = copy.deepcopy(dict(entry))
        document["closed_form"] = copy.deepcopy(entry.get("third_values") or [])
    if "declared_inexact_inputs" in entry:
        document["inexact_inputs"] = copy.deepcopy(entry["declared_inexact_inputs"])
    return {**document, "capture_only": False}


def declared_classes(entry: Mapping[str, Any]) -> dict[str, str] | None:
    static = (entry.get("screening_classes") or {}).get("static")
    return dict(static) if isinstance(static, Mapping) else None


def expected_support_states(entry: Mapping[str, Any]) -> dict[str, dict[str, str]]:
    return {str(support): dict(states) for support, states in
            ((entry.get("hand_entry") or {}).get("expected_status") or {}).items()}


def unread_members(entry: Mapping[str, Any]) -> dict[str, str]:
    """{support id: why} for each member entered at the window, whose printed load the comparison does not read."""
    members = ((entry.get("hand_entry") or {}).get("member") or []) if entry.get("run_kind") == "hand_entry" else []
    return {str(member["id"]): UNREAD_MEMBER.get(str(member.get("family")), UNREAD_OTHER) for member in members}


def compare_capture(entry: Mapping[str, Any], root: Path, capture_id: str, *,
                    solve: Callable[[Mapping[str, Any]], tuple[Any, str, dict[str, Any]]],
                    write: Callable[[Path, Any], str], declaration: Mapping[str, Any] | None,
                    identity: Mapping[str, Any] | None = None) -> tuple[Path, dict[str, Any]]:
    """Compare one stored capture of a window run with SWBPIPE's solve in this checkout; (comparison path, record).

    `solve(document)` solves SWBPIPE's side in a new solve folder: (payload or None, error, solve record);
    `write(path, value)` creates a file, never replacing one. `declaration` is the run's manifest declaration. Refuses
    (CaptureRefused), with nothing solved or written, a run the declaration does not let be compared, a capture whose
    record is not this run's, whose stored files no longer have the record's digests, whose print does not parse (when
    stored or now), whose instruction or run file differs from the run's now. The plain comparison is written first;
    the stated bases are then written beside it (STATED_BASES_RECORD) and never cost it."""
    root = Path(root).resolve()
    if declaration is None:
        raise CaptureRefused("not_declared", f"{entry['run_id']} has no run declaration in the manifest")
    compared, why = stated_bases.declared_compared(declaration)
    if not compared:
        raise CaptureRefused("not_screened", f"{entry['run_id']}: {why}; the capture is kept as a reference and "
                                             "compared by no route")
    names = _names(entry)
    dir_fd, record = _stored(root, entry["run_id"], capture_id)
    try:
        if (record.get("parse") or {}).get("status") != "parsed":
            raise CaptureRefused("print_not_parsed", f"the print did not parse when stored: "
                                                     f"{(record.get('parse') or {}).get('reason')}")
        for role in ("print", "instruction", "imported_mbf"):
            if record.get(role) != names[role] or names[role] not in (record.get("files") or {}):
                raise CaptureRefused("no_window_record", f"the record does not name this run's {role} file "
                                                         f"{names[role]}")
        stored = {role: _read_regular(dir_fd, names[role], "stored_file_changed")
                  for role in ("print", "instruction", "imported_mbf")}
    finally:
        os.close(dir_fd)
    if _sha256(stored["print"]) != record["files"][names["print"]]["sha256"]:
        raise CaptureRefused("print_digest_mismatch", "the print's bytes do not match the window record's digest")
    for role in ("instruction", "imported_mbf"):
        if _sha256(stored[role]) != record["files"][names[role]]["sha256"]:
            raise CaptureRefused("stored_file_changed", f"the stored {names[role]} does not match the window "
                                                        "record's digest")
    declared_instruction = (entry.get("window_instruction") or {}).get("sha256")
    if record.get("window_instruction_sha256") != declared_instruction:
        raise CaptureRefused("instruction_differs", f"the capture was made under the instruction of SHA-256 "
                                                    f"{record.get('window_instruction_sha256')}; the run now declares "
                                                    f"{declared_instruction}")
    mbf, mbf_text = run_mbf(entry)
    writer_sha = _sha256(mbf_text.encode("utf-8"))
    if writer_sha != record["mbf_sha256"]:
        raise CaptureRefused("pack_file_differs", f"the capture was made from a file of SHA-256 {record['mbf_sha256']} "
                                                  f"({(record.get('checkout') or {}).get('writer_file')}); the "
                                                  f"writer's file for this run now is {writer_sha}")
    status, parsed = parse_status(stored["print"])
    if status["status"] != "parsed":
        raise CaptureRefused("print_not_parsed", f"the print does not parse with the current parser: "
                                                 f"{status.get('reason')}")
    document = solved_document(entry)
    payload, error, solve_record = solve(document)
    if error or payload is None:
        raise CaptureRefused("not_solved", f"SWBPIPE not solved ({error or 'no result'})")
    unread = unread_members(entry)
    options = {"unread_supports": unread, "declared_classes": declared_classes(entry),
               "expected_support_states": expected_support_states(entry)}
    comparison = build_benchmark_record(document, swbpipe_result_payload=payload, mbf_result=mbf, mbf_text=mbf_text,
                                        parsed_caepipe=parsed, **options)
    case_root = root / "cases" / entry["run_id"]
    folder = capture_dir(root, entry["run_id"], capture_id)
    comparison["swbpipe_solve"] = dict(solve_record)
    comparison["caepipe_print"] = {"csv": str((folder / names["print"]).relative_to(case_root)),
                                   "route": WINDOW_ROUTE, "sha256": _sha256(stored["print"]), "capture_id": capture_id}
    comparison["window_capture"] = {
        "record": str((folder / WINDOW_RECORD).relative_to(case_root)),
        "binding": BINDING, "limits": WINDOW_LIMITS, "sections_not_read": status["sections_not_read"],
        "members_not_compared": unread,
        "kept_not_examined": sorted(name for name, item in record["files"].items()
                                    if item["role"] in (KEPT_NOT_EXAMINED, "saved_model")),
        "stored_at": record.get("checkout") or record.get("stored_at"),
        "compared_at": checkout(identity or {}),
        "declaration": {"screened": declaration.get("screened"),
                        "screening_waits_for": declaration.get("screening_waits_for"),
                        "counted": ("compared under its declaration; a run declared screened: false is not counted "
                                    "among the screened runs of exit criterion 11")},
        "run_line_text": record.get("run_line_text")}
    comparison["stated_bases"] = {"rule": stated_bases.RULE, "record": STATED_BASES_RECORD,
                                  "note": "written after this record, beside it"}
    path = contained_dir(root, case_root / "comparison" / solve_record["solve_id"]) / "benchmark_comparison.json"
    plain_sha = write(path, comparison)

    def basis_solve(basis_document: Mapping[str, Any], basis_names: tuple[str, ...]) -> tuple[Any, str, dict]:
        basis_payload, basis_error, made = solve(basis_document)
        made["stated_bases"] = list(basis_names)
        return basis_payload, basis_error, made

    bases = stated_bases.compare_bases(
        document, plain_payload=payload, plain_record=comparison, mbf_result=mbf, mbf_text=mbf_text,
        parsed_caepipe=parsed, solve=basis_solve, record_options=options, declaration=declaration)
    write(path.parent / STATED_BASES_RECORD, {"document_kind": STATED_BASES_KIND, "comparison": path.name,
                                              "comparison_sha256": plain_sha, **bases})
    return path, comparison
