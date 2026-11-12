"""The CAEPIPE sitting pack (DEL-17-05): prepare a pack a person can run, take its results in, unseal a capture.

From the project root:

    python3 -m tools.caepipe_correlation.sitting_pack prepare OUT_DIR [--manifest PATH] [--plan PATH]
        [--observations PATH] [--case ID ...]
    python3 -m tools.caepipe_correlation.sitting_pack ingest PACK_DIR [--item ID ...] [--check]
    python3 -m tools.caepipe_correlation.sitting_pack unseal --case ID [--tolerance-class CLASS]
        [--swbpipe-request FILE] [--swbpipe-result FILE]

PREPARE writes a self-contained pack to OUT_DIR, which must lie outside every Git work tree (as the harness refuses
an exchange folder inside one) and be absent or empty. The pack follows a plan (`--plan`, default
`sitting_plan_default.json` beside this module): an ordered list of parts, each with an id, a title, what it buys,
an estimate of the person's time (printed as an estimate, not a condition), a kind (`dry_run`, `benchmark`,
`sealed_capture` or `observations`), its cases selected from the benchmark manifest by model id, family, series,
model member or all, its format samples and its variants. A case is placed in the first part that selects it; a
part that selects nothing is listed empty with its reason. Within a part the order is by family, the members of one
series next to each other, smaller models first. Per case the pack holds the MBF exactly as the writer produces it
for the offline screening (`run_pilot.render_benchmark_mbf`), its identity file `<stem>.swbpipe-ids.json`, the
writer's report (loss report, diagnostics, assumptions), `case.json` (the case's index entry) and a run line to
fill in. A variant is either an instruction (the same MBF, one stated option changed by hand, the result saved
under its own name) or a substitution (exactly one whole line of the writer's MBF replaced, giving a distinct MBF
with its own SHA-256 and no identity file of its own; the base case's identity file binds its rows). A format
sample is an authored, invented MBF copied byte for byte, or a benchmark case with a variant; it is never screened
or compared. Observation items come from `--observations` and are copied in rank order. The pack also holds
`pack_index.json`, the run sheet `RUN_SHEET.md` generated from it, the instructions `INSTRUCTIONS.md`, the guest
script and a one-case driver. The same inputs give byte-identical files. A case the writer refuses is listed in
the index with its refusal codes.

INGEST takes a returned pack (the same folders with what the program wrote, the guest reports and the filled run
lines) into the exchange folder named by SWBPIPE_CAEPIPE_EXCHANGE, resolved and checked as the harness checks it
(outside every Git work tree, ignore-all rule, acknowledgement; every write path checks it again). Every record it
writes lives in the exchange folder, the private run directory, never in a repository: beside the case's run record,
or for a sealed capture in its own sealed folder. The kind of every item comes from the pack index, never from the
returned folder or the command line. The exchange folder is the owner's record: ingest and unseal only ever add new
files at new paths (each created exclusively), and a write that would land on an existing path is refused by name
with the existing file untouched. Nothing there is cleared, replaced, truncated, renamed or deleted, and no staging
copy is made in it. Per case it verifies that the MBF and identity file still have the SHA-256 the index records,
then:

- an ordinary case with no stored run: places the result where `run_pilot --benchmark MANIFEST --offline` finds it
  (`cases/<id>/caepipe_out/model.csv` or `model.txt`, with `run_record.json` and `sitting_intake.json`, and
  `caepipe_in/model.mbf`). A case that already has a stored run keeps it as it is: the result goes to a new run
  folder `cases/<id>/sitting_runs/<UTC time>-<hash of the returned files>/` with the same layout, and a new file
  `cases/<id>/run_selection/<that run>.json` names the run a later screen uses. Re-ingesting the same returned files
  is `already_placed`.
  A result the guest script produced is bound as the harness binds one: the run record holds the host's hash of the
  MBF and the hash the guest script reported for the MBF it solved. A result obtained by hand (`manual_batch`,
  `manual_print`) is recorded under its own route (`sitting_manual_batch`, `sitting_manual_print`) with no guest
  hash and the binding class `person_attested`; `bound_csv` does not admit it, so it is not compared.
- a sealed capture: copies every file of the case folder, under the names it came back with, to `sealed/<id>/`
  (outside `cases/`, so no path that walks stored results reaches it), hashing each file in that one pass, and
  records each file's name, size (from the file system) and SHA-256 in `capture_record.json`. Nothing reads its
  content beyond that pass: no recognition of the print file, no parser, no counts, no comparison. UNSEAL opens it
  only with a named tolerance class that has a value in the release target's benchmark method (every input held
  exactly, a named inexact input, nonlinear, modal; spectrum has none) and SWBPIPE's result for the case (the
  runner's request and result as run_pilot stores them, a solve of the request the index names). It keeps them
  beside the capture record and writes `unseal_record.json` before any byte of the capture is read for anything but
  hashing, and only then places the capture as an ordinary case. Both are written once and never replaced. The
  run's request is bound as well as its model: unseal is given SWBPIPE's request and result, refuses a result of
  the same model under another analysis request, and records the hashes of the model, the request and the result.
  A re-ingest of a sealed capture hashes the returned files once and writes nothing.
- a format sample (designated in the plan by an id of its own that no benchmark model has, and recorded in the
  index as outside the benchmark set) or an observation item: stores and hashes what came back
  (`format_samples/<id>/`, `observations/<id>/`) in one pass and does not parse, recognise or interpret it.
- a variant of an ordinary case: stores and hashes its result under `<run>/variants/<label>/` (the run directory
  or the new run folder the base result went to), bound to its MBF's hash; it is not screened.

Standard output carries operational status only: per item its id, kind, status and route, and for a placed result
the number of sections the parser read and of its error diagnostics. No result value is printed.
"""

from __future__ import annotations

import argparse
from datetime import datetime, timezone
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

from core.handoff.caepipe_external import HarnessSkip, canonical_model_hash, case_paths, resolve_harness_config  # noqa: E402,E501
from core.handoff.caepipe_external import harness as _harness  # noqa: E402
from core.handoff.caepipe_external.benchmark import EXACT, NAMED_INEXACT  # noqa: E402
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
    contained,
    contained_dir,
    last_json_object,
)
from core.handoff.caepipe_mbf.si_writer import STANDARD_GRAVITY  # noqa: E402
from tools.caepipe_correlation import run_pilot  # noqa: E402


PACK_VERSION = "1"
GENERATOR = "tools/caepipe_correlation/sitting_pack.py"
DEFAULT_MANIFEST = PROJECT / "validation" / "caepipe_benchmark" / "manifest.json"
DEFAULT_PLAN = Path(__file__).resolve().with_name("sitting_plan_default.json")
DRIVER = Path(__file__).resolve().parent / "sitting" / "Run-SittingCase.ps1"
DRIVER_REPOSITORY_PATH = "tools/caepipe_correlation/sitting/Run-SittingCase.ps1"
AUTHORED_REPOSITORY_DIR = "validation/caepipe_correlation/authored"

PLAN_KIND = "swbpipe.caepipe_sitting.plan"
OBSERVATIONS_KIND = "swbpipe.caepipe_sitting.observations"
INDEX_KIND = "swbpipe.caepipe_sitting.pack_index"
# Private records, written only in the exchange folder.
INTAKE_KIND = "swbpipe.caepipe_sitting.intake"
CAPTURE_KIND = "swbpipe.caepipe_sitting.sealed_capture"
UNSEAL_KIND = "swbpipe.caepipe_sitting.unseal"
STORED_KIND = "swbpipe.caepipe_sitting.stored_output"
SELECTION_KIND = "swbpipe.caepipe_sitting.run_selection"
PRIVATE_RECORD_KINDS = (INTAKE_KIND, CAPTURE_KIND, UNSEAL_KIND, STORED_KIND, SELECTION_KIND)
# Where a returned result goes when its case already has a stored run, and the files that name the run a later
# screen uses (one new file per selection; the stored run is never changed).
SITTING_RUNS = "sitting_runs"
RUN_SELECTION = "run_selection"
INTAKE_RECORD = "sitting_intake.json"
UNSEAL_PLACEMENT = "unseal_placement.json"
KEPT_REQUEST = "kept_swbpipe_request.json"
KEPT_RESULT = "kept_swbpipe_result.json"
CAPTURE_RECORD = "capture_record.json"
UNSEAL_RECORD = "unseal_record.json"
STORED_RECORD = "stored_output.json"
PRIVATE_RECORD_NAMES = (INTAKE_RECORD, CAPTURE_RECORD, UNSEAL_RECORD, STORED_RECORD, UNSEAL_PLACEMENT)
# The files unsealing adds beside a sealed capture; they are not part of the capture.
SEALED_SIDE_FILES = frozenset({CAPTURE_RECORD, UNSEAL_RECORD, UNSEAL_PLACEMENT, KEPT_REQUEST, KEPT_RESULT})

PART_KINDS = ("dry_run", "benchmark", "sealed_capture", "observations")
SELECT_KEYS = ("model_ids", "families", "series", "model_members", "all")
VARIANT_KINDS = ("instruction", "substitution")
ROUTES = ("guest_script", "manual_batch", "manual_print")
SITTING_ROUTES = {"manual_batch": "sitting_manual_batch", "manual_print": "sitting_manual_print"}
GUEST_BINDING = "guest_reported_hash"
ATTESTED_BINDING = "person_attested"
# The tolerance classes of the release target's benchmark method (docs/RELEASE_TARGET_v1.0.md, "Benchmark method"):
# those with a tolerance value today, and those without one. Only a class with a value admits an unseal.
TOLERANCE_CLASSES_WITH_VALUE = (EXACT, NAMED_INEXACT, "nonlinear", "modal")
TOLERANCE_CLASSES_WITHOUT_VALUE = ("spectrum",)
# Classes whose tolerance is set after the release target was accepted: an unseal under one also needs its report to the
# owner evidenced; how is left to DEL-17-05's scope, so for now such a class is always refused for that reason too.
TOLERANCE_CLASSES_SET_AFTER_ACCEPTANCE = ("spectrum",)
# Unseal refusals, each named.
CLASS_MISSING = "tolerance_class_missing"
CLASS_WITHOUT_VALUE = "tolerance_class_without_value"
CLASS_UNKNOWN = "tolerance_class_unknown"
RESULT_MISSING = "swbpipe_result_missing"
RESULT_MISMATCH = "swbpipe_result_mismatch"
REQUEST_DIFFERS = "swbpipe_request_differs"
SET_AFTER_ACCEPTANCE = "tolerance_set_after_acceptance"
REFUSAL_TEXT = {SET_AFTER_ACCEPTANCE: "tolerance set after acceptance: report not recorded",
                REQUEST_DIFFERS: "a result of this model under another analysis request"}
RESULT_NOT_SOLVED = "swbpipe_result_not_solved"
CAPTURE_MISSING = "capture_missing"
CAPTURE_CHANGED = "capture_changed"
ALREADY_UNSEALED = "already_unsealed"
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
_CHUNK = 1 << 20


class PackError(ValueError):
    """The plan, the observations, the manifest or the output directory cannot give a pack; the message says why."""


# -- small helpers ----------------------------------------------------------------------------------------------------
def _sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def stream_sha256(path: Path) -> tuple[int, str]:
    """(size, SHA-256) of a file: the size from the file system, the hash in one streaming pass; never decoded."""
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        while chunk := handle.read(_CHUNK):
            digest.update(chunk)
    return path.stat().st_size, digest.hexdigest()


def _json_bytes(value: Any) -> bytes:
    return (json.dumps(value, indent=2, sort_keys=True, ensure_ascii=True) + "\n").encode("ascii")


def _crlf(text: str) -> bytes:
    return text.replace("\r\n", "\n").replace("\n", "\r\n").encode("utf-8")


def _now() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


def outside_work_trees(path: Path, what: str) -> Path:
    """`path` made absolute, refused when it (or its nearest existing ancestor) lies inside a Git work tree or Git
    directory, by the harness's own check. Results that come back into it are private."""
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
    """A variant: `label`, `kind` and either `instruction` (by hand) or `old_line` and `new_line` (substitution)."""
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
        for key in ("old_line", "new_line"):
            line = variant.get(key)
            if not isinstance(line, str) or not line or "\r" in line or "\n" in line:
                raise PackError(f"{where} {key} must be one non-empty line")
            checked[key] = line
        if checked["old_line"] == checked["new_line"]:
            raise PackError(f"{where} new_line must differ from old_line")
        checked["note"] = _text(variant.get("note"), f"{where} note", optional=True)
        extra = set(variant) - {"label", "kind", "old_line", "new_line", "note"}
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
            if ("authored" in sample) == ("model_id" in sample):
                raise PackError(f"{swhere} names either an authored file or a benchmark model_id with a variant")
            checked = {"id": sample_id, "title": _text(sample.get("title"), f"{swhere} title"),
                       "purpose": _text(sample.get("purpose"), f"{swhere} purpose")}
            if "authored" in sample:
                checked["authored"] = _ident(sample["authored"], f"{swhere} authored")
                if set(sample) - {"id", "title", "purpose", "authored"}:
                    raise PackError(f"{swhere}: an authored sample takes no variant")
            else:
                checked["model_id"] = _ident(sample["model_id"], f"{swhere} model_id")
                checked["variant"] = check_variant(sample.get("variant"), f"{swhere} variant")
            format_samples.append(checked)
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


def substitute_line(text: str, old_line: str, new_line: str) -> str:
    """`text` with the one line equal to `old_line` replaced by `new_line` (CRLF lines, as the writer writes them);
    refused unless `old_line` occurs exactly once. The pilot's fault injection (`run_pilot.apply_fault_injection`)
    scales material rows by a factor and does not fit a declared one-line replacement, so it is not reused."""
    lines = text.split("\r\n")
    count = lines.count(old_line)
    if count != 1:
        raise PackError(f"the substitution's old line occurs {count} times in the writer's MBF; it must occur once")
    lines[lines.index(old_line)] = new_line
    return "\r\n".join(lines)


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
    text = substitute_line(base_text, variant["old_line"], variant["new_line"])
    data = text.encode("utf-8")
    files[f"{folder}/{vstem}.mbf"] = data
    names = _result_names(vstem)
    names.pop("run_line")
    return {"label": label, "kind": "substitution", "old_line": variant["old_line"], "new_line": variant["new_line"],
            "note": variant.get("note"),
            "mbf": {"file": f"{vstem}.mbf", "sha256": _sha256(data), "bytes": len(data), "same_as_base": False},
            "writer_output": False,
            "identity": (f"no identity file of its own: it is not the writer's output; the base case's identity file "
                         f"{identity_file} binds its rows, the substitution changing no node, element or support"),
            "identity_file_of_base": identity_file,
            "results": names}


def benchmark_item(model: Mapping[str, Any], manifest_entry: Mapping[str, Any], part: Mapping[str, Any], *,
                   sealed_reason: str | None, variants: list[Mapping[str, Any]],
                   files: dict[str, bytes], stem: str | None = None, folder: str | None = None) -> dict[str, Any]:
    """The index entry of one benchmark case, adding its pack files to `files`; a refused render lists its codes.
    `stem` and `folder` (a format sample's own id and folder) replace the model id in the file names."""
    model_id = model["model_id"]
    stem = stem or model_id
    folder = folder or f"cases/{model_id}"
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


def format_sample_item(sample: Mapping[str, Any], part: Mapping[str, Any], models: Mapping[str, Mapping[str, Any]],
                       manifest_entries: Mapping[str, Mapping[str, Any]], files: dict[str, bytes]) -> dict[str, Any]:
    folder = f"samples/{sample['id']}"
    if sample["id"] in models:
        raise PackError(f"format sample {sample['id']}: the id names a model of the benchmark manifest; a format "
                        "sample is designated by an id of its own and is recorded as outside the benchmark set")
    base = {"item_id": sample["id"], "item_kind": "format_sample", "part": part["id"], "folder": folder,
            "title": sample["title"], "purpose": sample["purpose"], "capture": "stored_only",
            "outside_benchmark_set": True,
            "never_screened": "a format sample is never screened or compared; ingest stores and hashes what comes "
                              "back and does not parse it"}
    if "authored" in sample:
        cases = {case["case_id"]: case for case in run_pilot.load_authored_cases()}
        if sample["authored"] not in cases:
            raise PackError(f"format sample {sample['id']}: {sample['authored']} is not an authored case")
        name = cases[sample["authored"]]["file"]
        data = run_pilot.authored_mbf_bytes(cases[sample["authored"]])
        provenance = run_pilot.AUTHORED_DIR / f"{sample['authored']}.provenance.json"
        declared = json.loads(provenance.read_text(encoding="utf-8")) if provenance.is_file() else {}
        if declared.get("produced_by_caepipe") is not False or "invented" not in str(declared.get("origin", "")):
            raise PackError(f"format sample {sample['id']}: {provenance.name} does not declare the file invented")
        stem = Path(name).stem
        files[f"{folder}/{name}"] = data
        base.update(stem=stem, status="written", source={
            "authored": f"{AUTHORED_REPOSITORY_DIR}/{name}",
            "provenance": f"{AUTHORED_REPOSITORY_DIR}/{provenance.name}",
            "provenance_sha256": _sha256(provenance.read_bytes())},
            mbf={"file": name, "sha256": _sha256(data), "bytes": len(data), "copied_byte_for_byte": True},
            identity_file=None, layout_row_count=_layout_rows_of(data.decode("utf-8", errors="replace")),
            results=_result_names(stem), variants=[])
        return base
    model_id = sample["model_id"]
    if model_id not in models:
        base.update(stem=model_id, status="not_in_manifest", refusal_codes=[], variants=[],
                    empty_because=f"{model_id} is not in the manifest")
        return base
    item = benchmark_item(models[model_id], manifest_entries[model_id], part, sealed_reason=None,
                          variants=[sample["variant"]], files=files, stem=sample["id"], folder=folder)
    item.update({key: base[key] for key in ("item_id", "item_kind", "part", "folder", "title", "purpose", "capture",
                                            "never_screened")})
    item.update(source_case=model_id, run_variant_only=True, outside_benchmark_set=True)
    return item


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
               selected: Iterable[str] | None = None) -> dict[str, bytes]:
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
    rows = {model["model_id"]: run_pilot.render_benchmark_mbf(model).layout_row_count for model in models}
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
            part_items.append(format_sample_item(sample, part, by_id, entries, files))
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


def write_pack(out_dir: Path, files: Mapping[str, bytes]) -> Path:
    """Write `files` below `out_dir`, which must lie outside every Git work tree and be absent or empty."""
    out = outside_work_trees(out_dir, "prepare")
    if out.is_symlink() or (out.exists() and (not out.is_dir() or any(out.iterdir()))):
        raise PackError(f"prepare: {out} exists and is not an empty directory; a pack is written only into a new "
                        "one, so no returned result is ever overwritten")
    out.mkdir(parents=True, exist_ok=True)
    for relative in sorted(files):
        target = out / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(files[relative])
    return out


# -- generated documents ----------------------------------------------------------------------------------------------
def _estimate(part: Mapping[str, Any]) -> str:
    return (f"Estimated time: {part['estimate']} (an estimate, not a condition)." if part.get("estimate")
            else "Estimated time: not given.")


def _kind_label(item: Mapping[str, Any]) -> str:
    if item["item_kind"] == "format_sample":
        return "format sample (run only its variant)" if item.get("run_variant_only") else "format sample"
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
share or upload it. The records SWBPIPE makes from it are written in the exchange folder, beside each case's run
record or, for a sealed capture, in its own sealed folder; never in a repository.

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
- **Substitution variant**: the pack already holds its own file `<stem>.<label>.mbf`, with one line of the base
  MBF replaced. Run it as any case, by any of the three ways, with `<stem>.<label>` as its stem.

### 2.4 Sealed captures and format samples

- A **sealed capture** is run exactly as any case and brought back. SWBPIPE stores it unopened until it has a
  tolerance class with a value and its own result for the case.
- A **format sample** makes the program print result sections that no public page describes. It is never screened
  or compared. Run it as any case; print all results to file as in 2.2.

### 2.5 The dry run comes first

Part 0 ({dry_ids}) is the dry run. Its cases already have stored results in the exchange folder, so they are run
by the hand route only: a dry-run case is never run through a route that clears a stored result. After part 0,
bring the pack back to the Mac and run

```
python3 -m tools.caepipe_correlation.sitting_pack ingest --check <the returned pack folder>
```

The check writes nothing. It verifies each case's MBF, names the route and binding it found, and reports how many
sections the parser read from each returned print. Go on to part 1 only when it reports each dry-run case with
sections read and no error diagnostic. After the sitting, `ingest` without `--check` takes the pack into the
exchange folder by adding new files only. A case that already has a stored run keeps it unchanged; the returned
result is stored beside it in a new run folder of its own. Ingest never clears, replaces, renames or deletes
anything already stored there.

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
            out += f"{item['title']}: the print should show {item['purpose']}. Never screened or compared.\n\n"
            if item.get("run_variant_only"):
                out += "Run only its variant below; the case itself is not run here.\n\n"
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
                        f"from the base MBF in one line, `{variant['old_line']}` replaced by `{variant['new_line']}`."
                        "\n\n")
        out += "Run line (`run_line.txt`):\n\n```\n" + run_line_text(item).replace("\r\n", "\n") + "```\n\n"
    out += "## Sources\n\n" + "\n".join(f"- {key}: {url}" for key, url in SOURCES.items()) + "\n"
    return out


def _ref(source_id: str) -> str:
    return f"{source_id}, {SOURCES[source_id]}"


# -- intake -----------------------------------------------------------------------------------------------------------
def parse_run_line(path: Path) -> dict[str, Any]:
    """The person's run line: each `key: value` line, with the route normalised. Never a CAEPIPE output."""
    try:
        text = path.read_bytes().decode("utf-8-sig", errors="replace")
    except OSError:
        return {"present": False}
    fields: dict[str, str] = {}
    for line in text.splitlines():
        key, sep, value = line.partition(":")
        if sep:
            fields[key.strip().lower()] = value.strip()
    route_key = "route used (guest_script, manual_batch or manual_print)"
    route = fields.get(route_key, "").strip().lower().replace(" ", "_")
    return {"present": True, "text": text, "fields": fields, "route": route if route in ROUTES else None,
            "route_as_written": fields.get(route_key) or None}


def _regular_files(folder: Path, skip_names: Iterable[str] = ()) -> dict[str, Path]:
    """The regular files of a returned folder by name (less `skip_names`); a link or a subfolder is refused."""
    found = {}
    skip = set(skip_names)
    for entry in sorted(folder.iterdir()):
        if entry.name in skip:
            continue
        if entry.is_symlink():
            raise PackError(f"{folder.name}/{entry.name} is a symbolic link; refused")
        if not entry.is_file():
            raise PackError(f"{folder.name}/{entry.name} is not a regular file; refused")
        found[entry.name] = entry
    return found


class ExistingPath(PackError):
    """A write would land on a path that already exists in the exchange folder; nothing there is replaced."""


def _relative(root: Path, path: Path) -> str:
    try:
        return str(Path(path).relative_to(Path(root).resolve()))
    except ValueError:
        return str(path)


def _create(root: Path, path: Path, data: bytes | str) -> None:
    """Create `path` in the exchange folder as a new file. Anything already at that path is refused by name and left
    untouched: the exchange folder is the owner's record, to which a run only adds."""
    target = contained(root, path)
    if target.exists() or target.is_symlink():
        raise ExistingPath(f"{_relative(root, target)} already exists in the exchange folder; nothing there is "
                           "replaced")
    with target.open("xb") as handle:
        handle.write(data.encode("utf-8") if isinstance(data, str) else data)


def _new_dir(root: Path, path: Path) -> Path:
    """Create a directory that must not exist yet (its parents are created as needed)."""
    if contained(root, path).exists():
        raise ExistingPath(f"{_relative(root, path)} already exists in the exchange folder; nothing there is replaced")
    return contained_dir(root, path)


def _copy_hashed(root: Path, source: Path, target: Path) -> dict[str, Any]:
    """Copy `source` to a new file `target` in the exchange folder in one streaming pass that also hashes it; the size
    comes from the file system. The content is never decoded, and an existing target is refused by name."""
    destination = contained(root, target)
    if destination.exists() or destination.is_symlink():
        raise ExistingPath(f"{_relative(root, destination)} already exists in the exchange folder; nothing there is "
                           "replaced")
    digest = hashlib.sha256()
    with source.open("rb") as reader, destination.open("xb") as writer:
        while chunk := reader.read(_CHUNK):
            digest.update(chunk)
            writer.write(chunk)
    size = destination.stat().st_size
    if size != source.stat().st_size:
        raise PackError(f"{source.name} changed size while it was copied")
    return {"bytes": size, "sha256": digest.hexdigest()}


def _hashes(files: Mapping[str, Path]) -> dict[str, dict[str, Any]]:
    """One hashing pass over each file, nothing written: {name: {bytes, sha256}}."""
    return {name: dict(zip(("bytes", "sha256"), stream_sha256(path))) for name, path in sorted(files.items())}


def _write_record(root: Path, path: Path, value: Any) -> None:
    _create(root, path, json.dumps(value, indent=2, sort_keys=True) + "\n")


def _verify_inputs(folder: Path, item: Mapping[str, Any], files: Mapping[str, Path]) -> str | None:
    """Why the case's MBF or identity file is not the one the index records, or None."""
    checks = [("mbf", item.get("mbf"))]
    if item.get("identity_file"):
        checks.append(("identity file", item["identity_file"]))
    for variant in item.get("variants") or []:
        if not variant["mbf"].get("same_as_base"):
            checks.append((f"variant {variant['label']} MBF", variant["mbf"]))
    for what, entry in checks:
        name = entry["file"]
        if name not in files:
            return f"{what}_missing: {name} is not in the returned folder"
        if stream_sha256(files[name])[1] != entry["sha256"]:
            return f"{what.replace(' ', '_')}_changed: {name} no longer has the SHA-256 the index records"
    return None


def pack_inputs(item: Mapping[str, Any]) -> set[str]:
    """The names of the files the pack put in an item's folder (run_line.txt excepted: it comes back filled)."""
    names = {"case.json"}
    for key in ("mbf", "identity_file", "writer_report"):
        if item.get(key):
            names.add(item[key]["file"])
    for variant in item.get("variants") or []:
        names.add(variant["mbf"]["file"])
    return names


def returned(folder: Path, item: Mapping[str, Any], files: Mapping[str, Path]) -> bool:
    """Whether anything came back for the item, from names and file-system sizes alone (nothing is read): a file the
    pack did not put there, or a run line whose size differs from the template's (filling it in adds text)."""
    if set(files) - pack_inputs(item) - {"run_line.txt"}:
        return True
    run_line = files.get("run_line.txt")
    template = (item.get("run_line") or {}).get("template_bytes")
    return run_line is not None and template is not None and run_line.stat().st_size != template


def sealed_dir(root: Path, item_id: str) -> Path:
    """The sealed location of one case: `sealed/<id>/` in the exchange folder, outside `cases/`, where no path that
    walks stored results (the offline screening, the private regression checks, the export step) looks."""
    return Path(root) / "sealed" / item_id


def seal_capture(folder: Path, item: Mapping[str, Any], root: Path, *,
                 pack_index_sha256: str | None = None, source_label: str = "returned pack folder") -> dict[str, Any]:
    """Seal one capture: copy every file of `folder` to a new `sealed/<id>/`, under the names it came back with,
    hashing each in that one pass, and record each name, size (from the file system) and SHA-256. Every returned file,
    the run line included, is read exactly once, by that copy; nothing recognises, parses or counts it. On a re-ingest
    (a capture record exists) the one pass is a hash with nothing written: the same capture is `already_sealed`, a
    different one is refused. Nothing in the exchange folder is ever replaced or removed."""
    root = outside_work_trees(Path(root), "the exchange folder")
    item_id = item["item_id"]
    files = _regular_files(folder)
    problem = _verify_inputs(folder, item, files)
    if problem:
        return {"status": "refused", "reason": problem}
    inputs = pack_inputs(item)
    target = sealed_dir(root, item_id)
    record_path = contained(root, target / CAPTURE_RECORD)

    def listing_of(stored: Mapping[str, Mapping[str, Any]]) -> dict[str, dict[str, Any]]:
        return {name: dict(entry) | {"role": "pack_input" if name in inputs else "returned"}
                for name, entry in stored.items()}

    if record_path.is_file():
        listing = listing_of(_hashes(files))
        capture_sha = _sha256(json.dumps(listing, sort_keys=True).encode("utf-8"))
        previous = json.loads(record_path.read_text(encoding="utf-8"))
        if previous.get("capture_sha256") == capture_sha:
            return {"status": "already_sealed", "capture_sha256": capture_sha}
        return {"status": "refused", "reason": f"a different capture is already sealed at sealed/{item_id}; nothing "
                                               "was written, and a sealed capture is never replaced"}
    if contained(root, target).exists():
        return {"status": "refused", "reason": f"sealed/{item_id} exists without a capture record; nothing was "
                                               "written"}
    contained_dir(root, target.parent)
    _new_dir(root, target)
    listing = listing_of({name: _copy_hashed(root, path, target / name) for name, path in sorted(files.items())})
    capture_sha = _sha256(json.dumps(listing, sort_keys=True).encode("utf-8"))
    _write_record(root, record_path, {
        "document_kind": CAPTURE_KIND, "item_id": item_id, "status": "sealed",
        "reason": item.get("sealed_reason"), "sealed_at": _now(), "source": source_label,
        "pack_index_sha256": pack_index_sha256, "mbf_sha256": item["mbf"]["sha256"],
        "model_hash": (item.get("source") or {}).get("model_hash"),
        "swbpipe_request": item.get("swbpipe_request"),
        "files": listing, "capture_sha256": capture_sha,
        "rule": ("Sealed: no agent opens, parses, decodes, prints, summarises or compares these files until "
                 "sitting_pack unseal admits a tolerance class with a value and SWBPIPE's result for this case's "
                 "complete analysis request."),
        "boundary_note": BOUNDARY_NOTE})
    return {"status": "sealed", "capture_sha256": capture_sha, "files": len(listing)}


def store_outputs(folder: Path, item: Mapping[str, Any], root: Path, target: Path, *,
                  names: Iterable[str] | None = None, skip_names: Iterable[str] = ()) -> dict[str, Any]:
    """Store and hash what came back for a format sample, an observation item or a variant in a new `target`, one
    pass per file under the names it came back with; never parse or recognise it. When `target` already holds a
    stored set, the one pass is a hash with nothing written: the same set is `already_stored`, a different one is
    refused by name."""
    root = outside_work_trees(Path(root), "the exchange folder")
    files = _regular_files(folder, skip_names)
    wanted = sorted(set(names) if names is not None else set(files) - pack_inputs(item))
    present = {name: files[name] for name in wanted if name in files}
    missing = [name for name in wanted if name not in files]
    record_path = contained(root, target / STORED_RECORD)
    if record_path.is_file():
        previous = json.loads(record_path.read_text(encoding="utf-8"))
        if previous.get("files") == _hashes(present):
            return {"status": "already_stored", "files": len(present), "missing": missing}
        return {"status": "refused", "reason": f"{_relative(root, target)} already holds different stored files; "
                                               "nothing was written"}
    if contained(root, target).exists():
        return {"status": "refused", "reason": f"{_relative(root, target)} exists without a stored record; nothing "
                                               "was written"}
    contained_dir(root, target.parent)
    _new_dir(root, target)
    stored = {name: _copy_hashed(root, path, target / name) for name, path in present.items()}
    _write_record(root, record_path, {
        "document_kind": STORED_KIND, "item_id": item["item_id"], "item_kind": item["item_kind"],
        "stored_at": _now(), "files": stored, "missing": missing,
        "rule": "stored and hashed only; not parsed, recognised, screened, compared or interpreted",
        "boundary_note": BOUNDARY_NOTE})
    return {"status": "stored", "files": len(present), "missing": missing}


def _guest_result(stem: str, files: Mapping[str, Path], mbf_sha: str) -> dict[str, Any]:
    """Read the guest script's report `<stem>.guest.json`: the MBF hash it reports, its status and print file."""
    path = files.get(f"{stem}.guest.json")
    if path is None:
        return {"present": False}
    log = last_json_object(path.read_bytes().decode("utf-8-sig", errors="replace"))
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


def classify_case(folder: Path, item: Mapping[str, Any], skip_names: Iterable[str] = ()) -> dict[str, Any]:
    """How an ordinary case came back, from `folder` alone (it may be a returned pack folder, an unsealed capture or
    any directory holding the same names): the inputs' hashes, the route, the binding class and the result file.
    Writes nothing and does not read the result's content."""
    stem, mbf_sha = item["stem"], item["mbf"]["sha256"]
    files = _regular_files(folder, skip_names)
    problem = _verify_inputs(folder, item, files)
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
    return {"status": "classified", "files": files, "run_line": run_line, "guest": guest, "notes": notes,
            "sitting_route": sitting_route, "binding": binding, "route": route, "result_name": result_name}


def check_case(folder: Path, item: Mapping[str, Any]) -> dict[str, Any]:
    """The dry run's check of one ordinary case, in place and writing nothing: hashes, route, binding, and the
    parser's section and error counts on the returned print."""
    found = classify_case(folder, item)
    if found["status"] == "refused":
        return found
    parser = None
    name = found["result_name"]
    if name is not None:
        path = found["files"][name]
        if name.endswith(".txt") and (problem := _harness._print_layout_problem(path)):
            return {"status": "checked", "route": "print_file_unrecognised", "binding": found["binding"],
                    "reason": problem}
        from core.handoff.caepipe_external.print_csv import read_caepipe_print_csv
        parsed = read_caepipe_print_csv(path)
        parser = {"sections_parsed": len(parsed["sections_parsed"]),
                  "error_diagnostics": sum(1 for d in parsed["diagnostics"] if d["severity"] == "error")}
    return {"status": "checked", "route": found["route"], "binding": found["binding"], "parser": parser,
            "reason": "; ".join(found["notes"]) or None}


def _has_stored_run(root: Path, case_root: Path) -> bool:
    """Whether the case's run directory already holds a run: any file in caepipe_out/, or caepipe_in/model.mbf."""
    out = contained(root, case_root / "caepipe_out")
    return (out.is_dir() and any(out.iterdir())) or contained(root, case_root / "caepipe_in" / "model.mbf").exists()


def _intake_records(root: Path, case_root: Path) -> list[Path]:
    """The case's sitting intake records: in its run directory and in each of its sitting run folders."""
    found = [case_root / "caepipe_out" / INTAKE_RECORD]
    runs = contained(root, case_root / SITTING_RUNS)
    if runs.is_dir():
        found += [run / "caepipe_out" / INTAKE_RECORD for run in sorted(runs.iterdir()) if run.is_dir()]
    return [contained(root, path) for path in found if contained(root, path).is_file()]


def intake_case(folder: Path, item: Mapping[str, Any], root: Path, *, pack_index_sha256: str | None = None,
                skip_names: Iterable[str] = ()) -> dict[str, Any]:
    """Take one ordinary (or unsealed) case in from `folder`, adding new files only.

    Where the case has no stored run, the result is placed where `run_pilot --benchmark ... --offline` finds it
    (`cases/<id>/caepipe_out/`, with its run record, and `caepipe_in/model.mbf`). Where it has one, the stored run
    is left as it is and the result goes to a new run folder `cases/<id>/sitting_runs/<UTC time>-<hash>/` with the
    same layout, and a new file `cases/<id>/run_selection/<run>.json` names that run for a later screen. A
    guest-script result is bound as the harness binds one (host and guest MBF hashes); a result obtained by hand gets
    its own route and the binding class `person_attested`, which `run_pilot.bound_csv` does not admit."""
    item_id, stem, mbf_sha = item["item_id"], item["stem"], item["mbf"]["sha256"]
    if item.get("item_kind") != "benchmark_case":
        return {"status": "refused", "reason": f"the index lists {item.get('item_id')} as {item.get('item_kind')}, "
                                               "not a benchmark case"}
    found = classify_case(folder, item, skip_names)
    if found["status"] == "refused":
        return found
    files, run_line, guest, notes = found["files"], found["run_line"], found["guest"], found["notes"]
    sitting_route, binding, route, result_name = (found["sitting_route"], found["binding"], found["route"],
                                                  found["result_name"])
    returned_files = _hashes({name: path for name, path in files.items() if name not in PRIVATE_RECORD_NAMES})
    root = outside_work_trees(Path(root), "the exchange folder")
    case_root = contained_dir(root, Path(root) / "cases" / item_id)
    for path in _intake_records(root, case_root):
        try:
            previous = json.loads(path.read_text(encoding="utf-8"))
        except (OSError, UnicodeDecodeError, json.JSONDecodeError):
            continue
        if previous.get("returned_files") == returned_files:
            return {"status": "already_placed", "route": previous.get("route"), "binding": previous.get("binding"),
                    "run": previous.get("run")}
    if result_name is not None and result_name.endswith(".txt"):
        problem = _harness._print_layout_problem(files[result_name])
        if problem:
            route, result_name = "print_file_unrecognised", None
            notes.append(problem)
    run_id = None
    if _has_stored_run(root, case_root):
        run_id = (datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ") + "-"
                  + _sha256(json.dumps(returned_files, sort_keys=True).encode("utf-8"))[:12])
        contained_dir(root, case_root / SITTING_RUNS)
        run_root = _new_dir(root, case_root / SITTING_RUNS / run_id)
    else:
        run_root = case_root
    run = _relative(root, run_root)
    out = contained_dir(root, run_root / "caepipe_out")
    in_dir = contained_dir(root, run_root / "caepipe_in")
    _copy_hashed(root, files[item["mbf"]["file"]], in_dir / "model.mbf")

    collected: dict[str, dict[str, Any]] = {}
    observed = None
    if result_name is not None:
        observed = "model.txt" if result_name.endswith(".txt") else "model.csv"
        collected[observed] = _copy_hashed(root, files[result_name], out / observed)
    if sitting_route == "guest_script":
        for suffix in ("mod", "res"):
            name = f"{stem}.{suffix}"
            if name in files:
                collected[f"model.{suffix}"] = _copy_hashed(root, files[name], out / f"model.{suffix}")
        if guest.get("log") is not None:
            text = json.dumps(guest["log"], indent=2, sort_keys=True) + "\n"
            _create(root, out / "guest_log.json", text)
            collected["guest_log.json"] = {"sha256": _sha256(text.encode("utf-8"))}
    capture_check = None
    if observed is not None:
        from core.handoff.caepipe_external.print_csv import read_caepipe_print_csv
        parsed = read_caepipe_print_csv(contained(root, out / observed))
        capture_check = {"sections_parsed": len(parsed["sections_parsed"]),
                         "error_diagnostics": sum(1 for d in parsed["diagnostics"] if d["severity"] == "error")}
    record = {
        "document_kind": RUN_RECORD_KIND, "schema_version": "1", "deliverable": "DEL-17-05", "case_id": item_id,
        "configuration_surface": {"environment_variable": ENV_VAR, "config_file": CONFIG_NAME,
                                  "checked": ["sitting pack intake"], "warnings": []},
        "executable_path_as_configured": None,
        ACKNOWLEDGEMENT_KEY: True,
        "mbf": {"path": "caepipe_in/model.mbf", "guest_path": None, "sha256": mbf_sha,
                "bytes": item["mbf"]["bytes"]},
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
        "parser": capture_check,
        "sitting": {"binding": binding, "pack_index_sha256": pack_index_sha256, "part": item["part"], "run": run,
                    "returned_file": result_name, "run_line": run_line.get("fields"), "notes": notes},
        "timestamps": {"ingested": _now()},
        "boundary_note": BOUNDARY_NOTE,
    }
    _write_record(root, out / "run_record.json", record)
    variants = {}
    for variant in item.get("variants") or []:
        variants[variant["label"]] = intake_variant(folder, item, variant, root, run_root=run_root, files=files,
                                                    skip_names=skip_names)
    bound = binding == GUEST_BINDING and route in CSV_ROUTES and observed is not None
    _write_record(root, out / INTAKE_RECORD, {
        "document_kind": INTAKE_KIND, "item_id": item_id, "ingested_at": _now(), "route": route,
        "sitting_route": sitting_route, "binding": binding, "bound": bound, "run": run,
        "returned_files": returned_files,
        "placed": collected, "parser": capture_check, "run_line": run_line.get("text"), "notes": notes,
        "variants": variants, "pack_index_sha256": pack_index_sha256, "boundary_note": BOUNDARY_NOTE})
    if run_id is not None:
        contained_dir(root, case_root / RUN_SELECTION)
        _write_record(root, case_root / RUN_SELECTION / f"{run_id}.json", {
            "document_kind": SELECTION_KIND, "case_id": item_id, "run": f"{SITTING_RUNS}/{run_id}",
            "selected_at": _now(), "bound": bound, "route": route,
            "rule": ("A later screen of this case uses the run named by the last file of run_selection/ in name "
                     "order, read as a case root (its caepipe_out/run_record.json and the print it names); the "
                     "stored run in the case's own caepipe_out/ is never changed."),
            "boundary_note": BOUNDARY_NOTE})
    return {"status": "placed" if observed else "recorded", "route": route, "binding": binding, "run": run,
            "parser": capture_check, "variants": {k: v["status"] for k, v in variants.items()}}


def intake_variant(folder: Path, item: Mapping[str, Any], variant: Mapping[str, Any], root: Path, *,
                   run_root: Path | None = None, files: Mapping[str, Path] | None = None,
                   skip_names: Iterable[str] = ()) -> dict[str, Any]:
    """Store a variant's result under `<run>/variants/<label>/`, bound to its MBF's hash; not screened."""
    files = files if files is not None else _regular_files(folder, skip_names)
    vstem = f"{item['stem']}.{variant['label']}"
    names = [name for name in files if name.startswith(vstem + ".") and name != variant["mbf"]["file"]]
    if not names:
        return {"status": "not_returned"}
    guest = _guest_result(vstem, files, variant["mbf"]["sha256"]) if variant["kind"] == "substitution" else {
        "present": False}
    base = run_root if run_root is not None else Path(root) / "cases" / item["item_id"]
    stored = store_outputs(folder, item, root, base / "variants" / variant["label"], names=names,
                           skip_names=skip_names)
    stored.update(mbf_sha256=variant["mbf"]["sha256"], variant_kind=variant["kind"],
                  binding=GUEST_BINDING if guest.get("route") in CSV_ROUTES else ATTESTED_BINDING,
                  screened=False)
    return stored


def _class_refusals(tolerance_class: str | None) -> list[str]:
    """Every reason a tolerance class does not admit an unseal (empty when it does)."""
    if not tolerance_class:
        return [CLASS_MISSING]
    reasons = []
    if tolerance_class in TOLERANCE_CLASSES_WITHOUT_VALUE:
        reasons.append(CLASS_WITHOUT_VALUE)
    if tolerance_class in TOLERANCE_CLASSES_SET_AFTER_ACCEPTANCE:
        reasons.append(SET_AFTER_ACCEPTANCE)
    if not reasons and tolerance_class not in TOLERANCE_CLASSES_WITH_VALUE:
        reasons.append(CLASS_UNKNOWN)
    return reasons


def benchmark_request(model: Mapping[str, Any]) -> dict[str, Any]:
    """The complete analysis request run_pilot sends the runner for a benchmark model (`solve_swbpipe`'s body as
    `_run_benchmark_model` builds it)."""
    model_id = model["model_id"]
    return {"request": run_pilot.runner_request(model_id, f"validation/caepipe_benchmark/models/{model_id}.json"),
            "solve": {"preview_model": {"model": model["model"], "materials": []}}}


def unseal_capture(root: Path, item_id: str, *, tolerance_class: str | None, swbpipe_request: Path | None,
                   swbpipe_result: Path | None, intake: Callable[..., dict[str, Any]] = intake_case) -> dict[str, Any]:
    """Unseal one sealed capture, or refuse with named reasons.

    Both conditions are required. `tolerance_class` names a tolerance class with a value in the release target's
    benchmark method: every input held exactly, a named inexact input, nonlinear, modal. Spectrum has none and is
    refused, and as a class whose tolerance is set after the release target's acceptance it is also refused until
    its report to the owner is recorded. `swbpipe_request` and `swbpipe_result` are the runner's request and result
    as run_pilot stores them (`request.json`, `result.json`): a solve of this case's complete analysis request, whose
    model and whole request hash equal what the index recorded. In this order, before any byte of the capture is
    read for anything but hashing: the capture's files are checked against their sealed hashes, the request and the
    result are kept beside the capture record as new files, and `unseal_record.json` is written with the tolerance
    class and the hashes of the model, the request, the result and the capture. Only then is the capture placed as
    an ordinary case (new files only), its outcome recorded in `unseal_placement.json`. Nothing is ever replaced: a
    capture is unsealed once, and a later solve is a separate result that never replaces the kept one."""
    reasons = _class_refusals(tolerance_class)
    if reasons:
        return {"status": "refused", "reason": reasons[0], "reasons": reasons}
    root = outside_work_trees(Path(root), "the exchange folder")
    sealed = contained(root, sealed_dir(root, item_id))
    record_path = contained(root, sealed / CAPTURE_RECORD)
    if not record_path.is_file():
        return {"status": "refused", "reason": CAPTURE_MISSING}
    if any(contained(root, sealed / name).exists() for name in (UNSEAL_RECORD, KEPT_REQUEST, KEPT_RESULT)):
        return {"status": "refused", "reason": ALREADY_UNSEALED}
    capture = json.loads(record_path.read_text(encoding="utf-8"))
    if swbpipe_request is None or swbpipe_result is None or not (Path(swbpipe_request).is_file()
                                                                 and Path(swbpipe_result).is_file()):
        return {"status": "refused", "reason": RESULT_MISSING}
    request_bytes, result_bytes = Path(swbpipe_request).read_bytes(), Path(swbpipe_result).read_bytes()
    try:
        request = json.loads(request_bytes.decode("utf-8"))
        result = json.loads(result_bytes.decode("utf-8"))
        model = request["solve"]["preview_model"]["model"]
        ref = request["request"]["model_ref"]["ref_id"]
    except (UnicodeDecodeError, json.JSONDecodeError, KeyError, TypeError):
        return {"status": "refused", "reason": RESULT_MISMATCH}
    model_hash = canonical_model_hash(model)
    if ref != item_id or model_hash != capture.get("model_hash"):
        return {"status": "refused", "reason": RESULT_MISMATCH}
    request_canonical = canonical_model_hash(request)
    if request_canonical != (capture.get("swbpipe_request") or {}).get("canonical_sha256"):
        return {"status": "refused", "reason": REQUEST_DIFFERS}
    mechanics = ((((result.get("payload") or {}).get("mechanics_envelope") or {}).get("status") or {})
                 .get("mechanics")) if isinstance(result, dict) else None
    if mechanics != "MECHANICS_SOLVED":
        return {"status": "refused", "reason": RESULT_NOT_SOLVED}
    for name, entry in (capture.get("files") or {}).items():
        path = contained(root, sealed / name)
        if not path.is_file() or path.is_symlink() or stream_sha256(path) != (entry["bytes"], entry["sha256"]):
            return {"status": "refused", "reason": CAPTURE_CHANGED}
    _create(root, sealed / KEPT_REQUEST, request_bytes)
    _create(root, sealed / KEPT_RESULT, result_bytes)
    _write_record(root, sealed / UNSEAL_RECORD, {
        "document_kind": UNSEAL_KIND, "item_id": item_id, "unsealed_at": _now(), "tolerance_class": tolerance_class,
        "model_hash": model_hash,
        "swbpipe_request": {"given": str(Path(swbpipe_request).resolve()), "kept": KEPT_REQUEST,
                            "sha256": _sha256(request_bytes), "canonical_sha256": request_canonical,
                            "request_id": (request.get("request") or {}).get("request_id")},
        "swbpipe_result": {"given": str(Path(swbpipe_result).resolve()), "kept": KEPT_RESULT,
                           "sha256": _sha256(result_bytes), "mechanics": mechanics},
        "capture_sha256": capture.get("capture_sha256"), "mbf_sha256": capture.get("mbf_sha256"),
        "boundary_note": BOUNDARY_NOTE})
    # From here on the capture's content may be read: the record exists.
    item = json.loads(contained(root, sealed / "case.json").read_text(encoding="utf-8"))
    placement = intake(sealed, item, root, pack_index_sha256=capture.get("pack_index_sha256"),
                       skip_names=SEALED_SIDE_FILES)
    _write_record(root, sealed / UNSEAL_PLACEMENT, {
        "document_kind": UNSEAL_KIND, "item_id": item_id, "placement": placement, "placed_at": _now(),
        "boundary_note": BOUNDARY_NOTE})
    return {"status": "unsealed", "placement": placement}


def ingest_pack(pack_dir: Path, root: Path | None, *, selected: Iterable[str] | None = None,
                check: bool = False, emit: Callable[[str], None] = print) -> bool:
    """Take a returned pack in, item by item, in run order. True when no returned item was refused.

    With `check` nothing is written (and `root` is not needed): each ordinary case is classified and its print read
    by the parser in place; a sealed capture, format sample or observation is only checked for its inputs' hashes."""
    pack = outside_work_trees(pack_dir, "ingest")
    if not check:
        root = outside_work_trees(Path(root), "the exchange folder")
    index_path = pack / "pack_index.json"
    index_bytes = index_path.read_bytes()
    index = json.loads(index_bytes.decode("utf-8"))
    if index.get("document_kind") != INDEX_KIND:
        raise PackError(f"{index_path} is not a sitting pack index")
    index_sha = _sha256(index_bytes)
    chosen = set(selected) if selected else None
    benchmark_ids = {item["item_id"] for item in index["items"] if item["item_kind"] == "benchmark_case"}
    ok = True
    for item in index["items"]:
        item_id = item["item_id"]
        if chosen is not None and item_id not in chosen:
            continue
        folder = pack / item["folder"]
        if item.get("status", "written") != "written":
            emit(f"{item_id}: {_kind_label(item)}; skipped")
            continue
        if not folder.is_dir() or folder.is_symlink():
            emit(f"{item_id}: folder {item['folder']} is missing; refused")
            ok = False
            continue
        try:
            if item["item_kind"] == "format_sample" and (
                    item_id in benchmark_ids or item.get("capture") != "stored_only"
                    or item["folder"] != f"samples/{item_id}" or not item.get("outside_benchmark_set")):
                raise PackError("the index does not designate this item a format sample outside the benchmark set; "
                                "it is not treated as one")
            files = _regular_files(folder)
            if check:
                if item["item_kind"] == "format_observation":
                    missing = [name for name in item["bring_back"] if name not in files]
                    outcome = {"status": "not_returned" if len(missing) == len(item["bring_back"]) else "present",
                               "missing": missing}
                elif not returned(folder, item, files):
                    outcome = {"status": "not_returned"}
                elif item["item_kind"] == "benchmark_case" and item.get("capture") == "screened":
                    outcome = check_case(folder, item)
                else:
                    problem = _verify_inputs(folder, item, files)
                    outcome = ({"status": "refused", "reason": problem} if problem
                               else {"status": "present; not opened"})
            elif item["item_kind"] == "format_observation":
                if not set(item["bring_back"]) & set(files):
                    outcome = {"status": "not_returned"}
                else:
                    outcome = store_outputs(folder, item, root, Path(root) / "observations" / item_id,
                                            names=item["bring_back"])
            elif not returned(folder, item, files):
                outcome = {"status": "not_returned"}
            elif item["item_kind"] == "format_sample":
                problem = _verify_inputs(folder, item, files)
                outcome = {"status": "refused", "reason": problem} if problem else store_outputs(
                    folder, item, root, Path(root) / "format_samples" / item_id)
            elif item.get("capture") == "sealed":
                outcome = seal_capture(folder, item, root, pack_index_sha256=index_sha)
            else:
                outcome = intake_case(folder, item, root, pack_index_sha256=index_sha)
        except (ContainmentError, PackError) as error:
            outcome = {"status": "refused", "reason": str(error)}
        if outcome["status"] == "refused":
            ok = False
        line = f"{item_id}: {_kind_label(item)}; {outcome['status']}"
        if outcome.get("route"):
            line += f"; route={outcome['route']}"
        if outcome.get("binding"):
            line += f"; binding={outcome['binding']}"
        if outcome.get("parser"):
            line += (f"; parser read {outcome['parser']['sections_parsed']} section(s), "
                     f"{outcome['parser']['error_diagnostics']} error diagnostic(s)")
        if outcome.get("missing"):
            line += f"; not returned: {', '.join(outcome['missing'])}"
        if outcome.get("reason"):
            line += f"; {outcome['reason']}"
        emit(line)
    return ok


# -- command line -----------------------------------------------------------------------------------------------------
def main(argv: list[str] | None = None, environ: Mapping[str, str] = os.environ) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    commands = parser.add_subparsers(dest="command", required=True)
    prepare = commands.add_parser("prepare", help="write a sitting pack outside every Git work tree")
    prepare.add_argument("out_dir", metavar="OUT_DIR")
    prepare.add_argument("--manifest", default=str(DEFAULT_MANIFEST), help="benchmark manifest (default: the tracked "
                                                                            "validation/caepipe_benchmark/manifest.json)")
    prepare.add_argument("--plan", help="pack plan JSON (default: the built-in sitting_plan_default.json)")
    prepare.add_argument("--observations", help="observation items JSON for the observations part")
    prepare.add_argument("--case", action="append", dest="cases", metavar="ID",
                         help="only this benchmark case (repeatable); format samples and observations stay")
    ingest = commands.add_parser("ingest", help="take a returned pack into the exchange folder")
    ingest.add_argument("pack_dir", metavar="PACK_DIR")
    ingest.add_argument("--item", action="append", dest="items", metavar="ID", help="only this item (repeatable)")
    ingest.add_argument("--check", action="store_true",
                        help="write nothing: verify hashes, name each case's route and binding, and run the parser on "
                             "each ordinary case's returned print in place (the dry run's check)")
    unseal = commands.add_parser("unseal", help="unseal one sealed capture in the exchange folder")
    unseal.add_argument("--case", required=True, metavar="ID")
    unseal.add_argument("--tolerance-class", metavar="CLASS",
                        help=f"one of {', '.join(TOLERANCE_CLASSES_WITH_VALUE)} (a class with a value)")
    unseal.add_argument("--swbpipe-request", metavar="FILE",
                        help="SWBPIPE's request for the case, as run_pilot stores it (swbpipe/request.json)")
    unseal.add_argument("--swbpipe-result", metavar="FILE",
                        help="SWBPIPE's result for that request, as run_pilot stores it (swbpipe/result.json)")
    args = parser.parse_args(argv)
    try:
        if args.command == "prepare":
            out = outside_work_trees(Path(args.out_dir), "prepare")
            files = build_pack(Path(args.manifest), load_plan(Path(args.plan) if args.plan else None),
                               load_observations(Path(args.observations) if args.observations else None), args.cases)
            write_pack(out, files)
            index = json.loads(files["pack_index.json"])
            print(f"pack written: {len(files)} files, {len(index['items'])} items")
            return 0
        if args.command == "ingest" and args.check:
            return 0 if ingest_pack(Path(args.pack_dir), None, selected=args.items, check=True) else 1
        config = resolve_harness_config(environ, require_vm_tools=False)
        if isinstance(config, HarnessSkip):
            print(f"CAEPIPE harness skipped: {config.reason}; nothing taken in")
            return 0
        root = config.exchange_root.resolve()
        if args.command == "ingest":
            return 0 if ingest_pack(Path(args.pack_dir), root, selected=args.items) else 1
        if not CASE_ID_PATTERN.match(args.case):
            print(f"error: case id {args.case!r} must match {CASE_ID_PATTERN.pattern}", file=sys.stderr)
            return 2
        outcome = unseal_capture(root, args.case, tolerance_class=args.tolerance_class,
                                 swbpipe_request=Path(args.swbpipe_request) if args.swbpipe_request else None,
                                 swbpipe_result=Path(args.swbpipe_result) if args.swbpipe_result else None)
        placement = outcome.get("placement") or {}
        reasons = "; ".join(f"{name} ({REFUSAL_TEXT[name]})" if name in REFUSAL_TEXT else name
                            for name in outcome.get("reasons") or ([outcome["reason"]] if outcome.get("reason") else []))
        print(f"{args.case}: {outcome['status']}" + (f"; {reasons}" if reasons else "")
              + (f"; placement {placement.get('status')}; route={placement.get('route')}" if placement else ""))
        return 0 if outcome["status"] == "unsealed" else 1
    except (OSError, KeyError, ValueError, ContainmentError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
