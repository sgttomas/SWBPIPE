"""The window print route of the benchmark comparison (DEL-17-05): a print made at the target program's window.

Some benchmark runs cannot be made by the batch round trip: a member is entered at the program's window, so the
committed sitting pack gives a window instruction (import the run's MBF, enter the fields, save the model under the
run's name, analyse, print all results to `<stem>.print.csv`). This module takes the folder such a run left, stores
it under the same protection as any stored print, and compares its print like any print.

Store (`store_capture`). Before anything is written it checks that the run is declared with a window instruction, that
the folder's imported MBF is byte for byte the run's file as the writer produces it at this commit (else
pack_file_differs, with both digests), and that the folder's instruction text is the manifest's. The folder itself is
never changed. Every regular file in it is copied, never moved, into
`cases/<run id>/caepipe_out/window/<capture id>/`, the capture id the first 12 hex digits of the print's SHA-256, each
file created exclusively; a symbolic link or a directory in the folder is refused, and a capture already stored is
refused as such. Then `window_record.json` is created there once, never replaced: each file's SHA-256, size and role,
the commit the command ran at, the time, the instruction's digest, the print's parse status and the sections the
parser did not read. A print that does not parse is stored all the same; its comparison is refused by name.

Compare (`compare_capture`). Offline, at any commit: the print's bytes are checked against the record's digest
(print_digest_mismatch) and the run's file at this commit against the record's (pack_file_differs); SWBPIPE's side is
solved afresh in its own solve folder (for a hand-entry run, `benchmark_hand_entry.hand_entry_model`: the base with
the member's support records appended), and the comparison is created beside it, naming the capture it read. Several
captures of one run may be stored; each is compared on its own, and a batch capture of the same run is a separate
record.

What a window capture cannot prove that a batch capture can (`WINDOW_LIMITS`), stated in the record and in every
comparison: nothing hashes the file the program opened, shows that the entries made were the instruction's, shows
that the print came from an analysis of the saved model, or shows the window's settings. The saved model and every
other file kept are not examined.
"""

from __future__ import annotations

import hashlib
import json
import os
import stat
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
WINDOW_ROUTE = "window_capture"
RUN_LINE = "run_line.txt"
BINDING = ("window: the SHA-256 of the run's MBF in the capture folder, equal to the writer's file at the commit the "
           "capture was stored and compared at, and of the saved model and the print; not of the file the program "
           "opened")
WINDOW_LIMITS = ("a window capture binds the print to its run by the digests of the files the folder holds. Unlike a "
                 "batch capture, nothing hashes the file the program opened, shows that the entries made were the "
                 "instruction's, shows that the print came from an analysis of the saved model, or shows the "
                 "window's settings; the saved model and every other file kept are not examined")
KEPT_NOT_EXAMINED = "kept, not examined"
LIMIT_STOP_PAGE = ("https://docs.sstcae.com/tech_manual/limit_stop.htm (technical reference, \"Limit Stop\"), read "
                   "2026-11-21: \"This friction force is displayed in results under Limit stop support loads.\"")
UNREAD_MEMBER = {
    "nonlinear": ("the print's Limit stop support loads section is not read: its format is not publicly stated beyond "
                  "its name (" + LIMIT_STOP_PAGE + ")"),
    "constant_effort_support": ("the print's hanger loads are not read: their section is outside the parser's "
                                "coverage and its format is not publicly stated; constant_support_load stays named"),
}
UNREAD_OTHER = "the printed load of a member entered at the window is not read"


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
    """The writer's result for the run's base and the MBF the run carries, at this commit."""
    mbf = render_caepipe_mbf_si(entry["model"], title=entry["model_id"],
                                reference_temperature_c=entry.get("reference_temperature_c", 20.0),
                                piping_code=entry["piping_code"])
    if mbf.status != "written":
        raise CaptureRefused("mbf_refused", "the writer refuses the run's base at this commit")
    return mbf, benchmark_models.run_mbf_text(entry, mbf.text)


def _names(entry: Mapping[str, Any]) -> dict[str, str]:
    run = (entry.get("hand_entry") or {}).get("run") or {}
    run_id = entry["run_id"]
    return {"print": run.get("expected_print_file") or f"{run_id}.print.csv",
            "saved_model": run.get("expected_saved_file") or f"{run_id}.mod",
            "run_line": RUN_LINE, "imported_mbf": f"{run_id}.mbf",
            "instruction": f"{run_id}.window-instruction.txt"}


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


def store_capture(entry: Mapping[str, Any], root: Path, folder: Path, *,
                  identity: Mapping[str, Any]) -> tuple[str, dict[str, Any]]:
    """Store one capture folder of a window run; (capture id, record). Refuses (CaptureRefused) before anything is
    written unless the run is declared with a window instruction, the folder holds the required files as regular
    files, its imported MBF is the writer's file at this commit and its instruction is the manifest's."""
    if not is_window_run(entry):
        raise CaptureRefused("not_a_window_run", f"{entry.get('run_id') or entry.get('model_id')} is not declared "
                                                 "with a window instruction")
    root = Path(root).resolve()
    folder = Path(folder)
    if folder.is_symlink() or not folder.is_dir():
        raise CaptureRefused("capture_folder", "the capture folder is not a directory (or is a symbolic link)")
    files: dict[str, bytes] = {}
    for path in sorted(folder.iterdir()):
        mode = path.lstat().st_mode
        if stat.S_ISLNK(mode) or not stat.S_ISREG(mode):
            raise CaptureRefused("capture_folder", f"{path.name} is a symbolic link or not a regular file; only "
                                                   "regular files are stored")
        files[path.name] = path.read_bytes()
    names = _names(entry)
    missing = [names[role] for role in ("print", "saved_model", "run_line", "imported_mbf") if names[role] not in files]
    if missing:
        raise CaptureRefused("capture_incomplete", f"the folder lacks {', '.join(missing)}")
    _, mbf_text = run_mbf(entry)
    writer_sha = _sha256(mbf_text.encode("utf-8"))
    imported_sha = _sha256(files[names["imported_mbf"]])
    if imported_sha != writer_sha:
        raise CaptureRefused("pack_file_differs", f"the imported file's SHA-256 is {imported_sha}; the writer's file "
                                                  f"for this run at commit {identity.get('source_commit')} is "
                                                  f"{writer_sha}")
    instruction = entry["window_instruction"]
    if names["instruction"] in files and files[names["instruction"]] != instruction["text"].encode("utf-8"):
        raise CaptureRefused("instruction_differs", "the folder's window instruction is not the manifest's text")
    if _sha256(instruction["text"].encode("utf-8")) != instruction["sha256"]:
        raise CaptureRefused("instruction_differs", "the manifest's instruction text does not have its declared digest")
    print_bytes = files[names["print"]]
    capture_id = _sha256(print_bytes)[:12]
    target = capture_dir(root, entry["run_id"], capture_id)
    if os.path.lexists(target):
        raise CaptureRefused("capture_stored", f"capture {capture_id} of {entry['run_id']} is already stored")
    status, _ = parse_status(print_bytes)
    roles = {names[role]: role for role in ("print", "saved_model", "run_line", "imported_mbf", "instruction")}
    contained_dir(root, target.parent)
    contained(root, target).mkdir()  # fails if it appeared meanwhile: a capture is never written into
    for name, data in files.items():
        descriptor = os.open(contained(root, target / name), os.O_CREAT | os.O_EXCL | os.O_WRONLY, 0o600)
        with os.fdopen(descriptor, "wb") as handle:
            handle.write(data)
    record = {
        "document_kind": WINDOW_RECORD_KIND, "deliverable": "DEL-17-05", "route": WINDOW_ROUTE,
        "run_id": entry["run_id"], "model_id": entry["model_id"], "series": entry.get("series_run"),
        "capture_id": capture_id, "stored": _now(), "stored_at": dict(identity),
        "source_folder_name": folder.name,
        "files": {name: {"sha256": _sha256(data), "bytes": len(data), "role": roles.get(name, KEPT_NOT_EXAMINED)}
                  for name, data in sorted(files.items())},
        "print": names["print"], "saved_model": names["saved_model"], "run_line_text":
            files[names["run_line"]].decode("utf-8", "replace"),
        "mbf_sha256": writer_sha, "window_instruction_sha256": instruction["sha256"],
        "parse": status, "binding": BINDING, "limits": WINDOW_LIMITS,
        "examined": "the print is parsed and compared; the saved model and every other file are kept, not examined",
    }
    descriptor = os.open(contained(root, target / WINDOW_RECORD), os.O_CREAT | os.O_EXCL | os.O_WRONLY, 0o600)
    with os.fdopen(descriptor, "wb") as handle:
        handle.write((json.dumps(record, indent=2, sort_keys=True) + "\n").encode("utf-8"))
    return capture_id, record


def stored_captures(root: Path, run_id: str) -> list[str]:
    folder = Path(root).resolve() / "cases" / run_id / "caepipe_out" / "window"
    if folder.is_symlink() or not folder.is_dir():
        return []
    return sorted(path.name for path in folder.iterdir() if path.is_dir() and not path.is_symlink())


def read_record(root: Path, run_id: str, capture_id: str) -> dict[str, Any]:
    path = contained(Path(root).resolve(), capture_dir(Path(root).resolve(), run_id, capture_id) / WINDOW_RECORD)
    if path.is_symlink() or not path.is_file():
        raise CaptureRefused("no_window_record", f"capture {capture_id} of {run_id} has no window record")
    record = json.loads(path.read_text(encoding="utf-8"))
    if record.get("document_kind") != WINDOW_RECORD_KIND or record.get("capture_id") != capture_id:
        raise CaptureRefused("no_window_record", f"capture {capture_id} of {run_id} has no valid window record")
    return record


def solved_document(entry: Mapping[str, Any]) -> dict[str, Any]:
    """The document SWBPIPE solves for a window run: for a hand-entry run the model with the member (and its third
    values as closed forms); otherwise the run's own."""
    if entry.get("run_kind") == "hand_entry":
        document = benchmark_hand_entry.hand_entry_document(dict(entry))
        return {**document, "capture_only": False}
    return dict(entry)


def unread_members(entry: Mapping[str, Any]) -> dict[str, str]:
    """{support id: why} for each member entered at the window, whose printed load the comparison does not read."""
    members = ((entry.get("hand_entry") or {}).get("member") or []) if entry.get("run_kind") == "hand_entry" else []
    return {str(member["id"]): UNREAD_MEMBER.get(str(member.get("family")), UNREAD_OTHER) for member in members}


def compare_capture(entry: Mapping[str, Any], root: Path, capture_id: str, *,
                    solve: Callable[[Mapping[str, Any]], tuple[Any, str, dict[str, Any]]],
                    write: Callable[[Path, Any], str]) -> tuple[Path, dict[str, Any]]:
    """Compare one stored capture of a window run with SWBPIPE's solve at this commit; (comparison path, record).

    `solve(document)` solves SWBPIPE's side in a new solve folder: (payload or None, error, solve record);
    `write(path, value)` creates the comparison, never replacing one. Refuses (CaptureRefused) a capture whose print
    did not parse, whose bytes no longer have the record's digest, or whose run file differs at this commit."""
    root = Path(root).resolve()
    record = read_record(root, entry["run_id"], capture_id)
    folder = capture_dir(root, entry["run_id"], capture_id)
    if (record.get("parse") or {}).get("status") != "parsed":
        raise CaptureRefused("print_not_parsed", f"the print did not parse when stored: "
                                                 f"{(record.get('parse') or {}).get('reason')}")
    print_path = contained(root, folder / record["print"])
    print_bytes = print_path.read_bytes()
    if _sha256(print_bytes) != record["files"][record["print"]]["sha256"]:
        raise CaptureRefused("print_digest_mismatch", "the print's bytes do not match the window record's digest")
    mbf, mbf_text = run_mbf(entry)
    writer_sha = _sha256(mbf_text.encode("utf-8"))
    if writer_sha != record["mbf_sha256"]:
        raise CaptureRefused("pack_file_differs", f"the capture was made from a file of SHA-256 {record['mbf_sha256']} "
                                                  f"(stored at commit {record['stored_at'].get('source_commit')}); "
                                                  f"the writer's file for this run now is {writer_sha}")
    status, parsed = parse_status(print_bytes)
    document = solved_document(entry)
    payload, error, solve_record = solve(document)
    if error or payload is None:
        raise CaptureRefused("not_solved", f"SWBPIPE not solved ({error or 'no result'})")
    unread = unread_members(entry)
    comparison = build_benchmark_record(document, swbpipe_result_payload=payload, mbf_result=mbf, mbf_text=mbf_text,
                                        parsed_caepipe=parsed, unread_supports=unread)
    comparison["swbpipe_solve"] = dict(solve_record)
    comparison["caepipe_print"] = {"csv": str(print_path.relative_to(root / "cases" / entry["run_id"])),
                                   "route": WINDOW_ROUTE, "sha256": _sha256(print_bytes), "capture_id": capture_id}
    comparison["window_capture"] = {
        "record": str((folder / WINDOW_RECORD).relative_to(root / "cases" / entry["run_id"])),
        "binding": BINDING, "limits": WINDOW_LIMITS, "sections_not_read": status["sections_not_read"],
        "members_not_compared": unread,
        "kept_not_examined": sorted(name for name, item in record["files"].items()
                                    if item["role"] in (KEPT_NOT_EXAMINED, "saved_model")),
        "stored_at": record["stored_at"], "run_line_text": record.get("run_line_text")}

    def basis_solve(basis_document: Mapping[str, Any], names: tuple[str, ...]) -> tuple[Any, str, dict[str, Any]]:
        basis_payload, basis_error, made = solve(basis_document)
        made["stated_bases"] = list(names)
        return basis_payload, basis_error, made

    comparison["stated_bases"] = stated_bases.compare_bases(
        document, plain_payload=payload, plain_record=comparison, mbf_result=mbf, mbf_text=mbf_text,
        parsed_caepipe=parsed, solve=basis_solve, record_options={"unread_supports": unread})
    path = contained_dir(root, root / "cases" / entry["run_id"] / "comparison" / solve_record["solve_id"]) \
        / "benchmark_comparison.json"
    write(path, comparison)
    return path, comparison
