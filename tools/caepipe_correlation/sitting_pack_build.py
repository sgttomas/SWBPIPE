"""The builder of the fixed CAEPIPE sitting pack (DEL-17-05): a development tool, and nothing more.

From the project root:

    python3 -m tools.caepipe_correlation.sitting_pack_build OUT_DIR

It reads the tracked benchmark manifest (`validation/caepipe_benchmark/manifest.json`), the tracked model documents it
names and one tracked plan (`sitting_pack_plan.json` beside this module), and writes into OUT_DIR, which must not
exist, two sibling folders: `pack/`, what the owner copies to Windows and nothing else (the instructions, the run sheet,
the command file, the qualification, the index, and per item its model file, its window instruction where there is one
and its run line), and `records/`, the Mac-side records a run holds or references (each written model file's identity
sidecar and writer's report; DEL-17-05 REQ-005 and REQ-006), never copied. Each model file in `pack/` has a
`<stem>.provenance.json` beside it, in the form the repository's data-boundary scan reads, saying only what is
established of that file. The same inputs give the same bytes. It
prints the pack's identity: the SHA-256 of `pack/pack_index.json`.

The pack the owner runs is not what this tool writes on some machine: it is the folder committed at
`validation/caepipe_sitting_pack/` and reviewed there, byte for byte. Regenerating that folder from the tracked inputs
is a consistency test of the committed bytes, not what makes them trustworthy; the reading of the committed bytes is.
Nothing here runs at the sitting, reads an exchange folder, takes results in or decodes anything returned.

What the pack holds, in the order the owner runs it (parts are struck from the last backwards; part 1 is never struck, only shortened from its highest strike numbers):

- part 0, the dry run: three small first-tranche cases by the batch route, and three samples, each a copy under its
  own item id and folder of one dynamics run file in the form that carries both added records (the smallest dynamics
  model's): one run by the batch command, one opened in the program's window for the checks (i) to (iv) the benchmark
  manifest's `dry_run` member lists, and the sacrificial duplicate its `export_trial` names, for check (v);
- part 1, the dynamics runs: every declared pair, in the manifest's strike order, both forms as their own folders,
  window only;
- part 2, the pressurised series: the screened runs, then the capture-only runs (sealed), batch route;
- part 3, the series S1, S2 and S3, batch route, each series' members together;
- part 4, the hand-entry cases in the manifest's rank order, window only;
- part 5, the first tranche's remaining models, batch route.

Every model file is the benchmark generator's document written by SWBPIPE's model-file writer and, for a declared run,
`benchmark_models.run_mbf_text` (the pressure options line substituted, or whole options records added); a run's file
must have the SHA-256 the manifest declares for that run, and a model's the `mbf_sha256` the manifest declares for it;
a file without a declared hash, or with another, refuses the build. A window-only case carries its window instruction as its own text file, whose
bytes are the manifest's `window_instruction` text and whose SHA-256 is the manifest's. A window-only case never has a
line in `BATCH_COMMANDS.txt`.

`QUALIFICATION.txt` carries the qualification of SWBPIPE's results; its SHA-256 is named in the index for every item
whose text names the product's gate ("checks passed"), and it is printed at the top of the instructions and of the run
sheet. `pack/pack_index.json` names every other file of both folders with its SHA-256, size and folder, so the pack's
identity, the SHA-256 of `pack/pack_index.json`, covers every byte. A dynamics pair whose spectrum table no dry-run
sample carries is said to be run in the typed form; a dynamics run need not have a pair. The pack contains only its specified permitted kinds of file (`PACK_KINDS`) and reviewed
contents.
"""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
import os
from pathlib import Path
import re
import sys
from typing import Any, Iterable, Mapping

PROJECT = Path(__file__).resolve().parents[2]
if str(PROJECT) not in sys.path:
    sys.path.insert(0, str(PROJECT))

from core.handoff.caepipe_external.correlation import canonical_model_hash  # noqa: E402
from core.handoff.caepipe_mbf import STANDARD_GRAVITY, render_caepipe_mbf_si  # noqa: E402
from tools.caepipe_correlation import benchmark_models  # noqa: E402

MANIFEST = PROJECT / "validation" / "caepipe_benchmark" / "manifest.json"
PLAN = Path(__file__).resolve().with_name("sitting_pack_plan.json")
PACK_FOLDER = "validation/caepipe_sitting_pack"  # the committed, reviewed pack
BUILDER = "tools/caepipe_correlation/sitting_pack_build.py"
PACK_VERSION = "4"
MANIFEST_KIND = "openpipestress.caepipe_benchmark.manifest"
MODEL_KIND = "openpipestress.caepipe_benchmark.model"
PLAN_KIND = "swbpipe.caepipe_sitting.fixed_pack_plan"
INDEX_KIND = "swbpipe.caepipe_sitting.pack_index"
INDEX = "pack_index.json"
INSTRUCTIONS = "INSTRUCTIONS.md"
RUN_SHEET = "RUN_SHEET.md"
BATCH_COMMANDS = "BATCH_COMMANDS.txt"
QUALIFICATION_FILE = "QUALIFICATION.txt"
TOP_LEVEL = (INDEX, INSTRUCTIONS, RUN_SHEET, BATCH_COMMANDS, QUALIFICATION_FILE)
# Two sibling folders: `pack/`, what the owner copies to Windows and nothing else; `records/`, the Mac-side records a
# run holds or references (DEL-17-05 REQ-005 and REQ-006: the writer's identity file and its report), never copied.
PACK, RECORDS = "pack", "records"
INDEX_PATH = f"{PACK}/{INDEX}"
# The kinds of file the pack holds, and the kinds it asks back: allow lists, by the last extension, compared without
# regard to letter case (`kind_problem`).
PACK_KINDS = frozenset({".mbf", ".json", ".md", ".txt"})
RETURN_KINDS = PACK_KINDS | {".csv", ".mod", ".res"}
BATCH, WINDOW = "manual_batch", "manual_print"
PART_KINDS = ("dry_run", "dynamics", "pressure", "static_series", "hand_entry", "tranche1_rest")
PRESSURE_SCREENED = ("pressure-default", "pressure-elongation-on")
PRESSURE_CAPTURE = "pressure-bourdon-expansion-capture"
GATE = "checks passed"

QUALIFICATION = ("Development comparison only. In any SWBPIPE result, 'checks passed', 'established' and certified bounds "
                 "rest on a premise the product does not verify, which is false on a constructed model (DEL-04-01; exit "
                 "criterion 18, not met). They are not established engineering results. The mechanics values are "
                 "compared with CAEPIPE on that understanding.")
QUALIFICATION_FOLLOW = ("Nothing in this pack is a SWBPIPE result. Where a case is held until the product publishes it "
                        "as checks passed, that phrase names the product's own gate and carries this qualification.")
# What is done with the returned folder on the Mac, as the Agent 0 settled it; one constant, which the storage
# script's second check may yet adjust as far as is needed to match that script.
SET_PASSAGE = ("On the Mac, the storage procedure copies the returned folder under one new name, records each file's path, "
               "size and hash, and verifies the copied contents against the source hashes. It reads file contents as "
               "bytes without interpreting prints or run lines, comparing models with the pack, or classifying files by "
               "kind. This procedure checks no print sections. Your dry-run comparison checks the example print you "
               "inspect; it does not check subsequent prints.")
SCREEN_WAITS = ("A result from this sitting is screened only after (a) the harness's stored-results change (pull "
                "request 262) has merged, (b) a rule for a returned file's origin is in place, and (c) the comparison "
                "record itself carries the qualification of SWBPIPE's results as a required member (pull request 263); "
                "no screen is run before all three.")
ONE_SESSION = ("A window instruction may end with a paragraph offering, on a condition it states, to make a run's "
               "setting in the model of an earlier run of the same case that is still open, instead of importing the "
               "run's file. Where its condition holds and you take it, save the model and print the results under that run's name in that run's folder, as the paragraph "
               "says, and write on its run line that the setting was made by hand. The files expected back in that "
               "folder are the same three.")
ON_THE_MAC = "No SWBPIPE command for the returned folder is part of this pack."
KEEP_EVERYTHING = ("A capture brought back without this evidence may remain unusable for a qualified comparison. Bring "
                   "back everything in each run's folder, and do not tidy, rename or delete anything.")
KINDS_CLAIM = "The pack contains only its specified permitted kinds of file and reviewed contents"
EXPORT_BINDING = ("An export, where one is made, is bound to that run's saved model and that run's instruction: the "
                  "program writes it from that run's saved model (`<run id>.mod`) as the program holds it after that "
                  "run's instruction, including what was entered or set by hand; it is not the pack's model file, "
                  "and it is not evidence of which model was run.")

# DEL-17-01 Source_Basis_Register.md: the public pages the instructions quote.
SOURCES = {
    "CAEPIPE-BATCH": "https://docs.sstcae.com/users_manual/running_caepipe_caepipe_3d__in_batch_mode.htm",
    "CAEPIPE-IMPORT-MBF": "https://docs.sstcae.com/users_manual/import_mbf.htm",
    "CAEPIPE-EXPORT-MBF": "https://docs.sstcae.com/users_manual/export_mbf.htm",
    "CAEPIPE-PRINT-TO-FILE": "https://docs.sstcae.com/users_manual/print__ctrl_p__2.htm",
    "CAEPIPE-ANALYZE": "https://docs.sstcae.com/users_manual/analyze__f12_.htm",
    "CAEPIPE-SAVE": "https://docs.sstcae.com/users_manual/save__ctrl_s_.htm",
    "CAEPIPE-FILE-MENU": "https://docs.sstcae.com/users_manual/file_menu_2.htm",
    "MS-START": "https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/start",
}
# Each a contiguous excerpt of the saved public page it is attributed to.
IMPORT_OPPRES_FIELDS = "[Bourdon effect (0 = Off, 1 = ON)], [Pressure Correction for Bends (0 = Off, 1 = On)]"
IMPORT_OPPRES_BLANK = "Leaving a BLANK line following the Section heading \u201cOPPRES\u201d will set the values as 0, 1, 1, 1.00, 0."
IMPORT_COMMAND = "To import a model, select the menu command File > Import from the Main window."
BATCH_TWO_ARGUMENTS = ("Batch mode requires two arguments, separated by a semicolon (;). The first argument is the MBF "
                       "file name, and the second argument is batchmode=yes.")
BATCH_EXAMPLE_OUTPUT = ("The results will be output in CSV file format after analysis with name \u201cStrModel R0.csv\u201d "
                        "and will be stored in the folder where the MBF file is stored")
PRINT_RESULTS_TAB = "On this tab, check the different results items that you want to print."
PRINT_TO_FILE = "If you want to send all of your selected items (input and results) click on the To File button."
PRINT_FILE_KINDS = "You can export the data to a .txt file or to a .csv file."
ANALYZE_VIEW_RESULTS = ("At the end of analysis, you are asked whether you want to \u201cView results?\u201d Click on OK "
                        "to see results or Cancel to return to the main window.")
ANALYZE_INACTIVE = ("Command \u201cAnalyze\u201d in the \u201cFile\u201d menu will become inactive when the model is not "
                    "edited and \u201c.res\u201d file is available in the directory where the stress model \u201c.mod\u201d "
                    "is stored.")
EXPORT_OVERWRITE = ("But note that the existing .mod file will be overwritten. So, you must rename the edited text file "
                    "before importing it back into CAEPIPE.")
SAVE_WORDS = "This command saves model data to a .mod file format."
START_TITLE_WORDS = "Specifies the title to display in the Command Prompt window title bar."
START_WAIT_WORDS = "Starts an application and waits for it to end."

BATCH_LINE_SHAPE = 'start "" /wait "%CAEPIPE%" "%CD%\\<folder>\\<stem>.mbf";batchmode=yes'
BATCH_SETUP = ('set "CAEPIPE=<the full path of your CAEPIPE program>"', 'cd /d "<the full path of the pack folder>"')
START_LINE = re.compile(r'start "" /wait "%CAEPIPE%" "%CD%(\\[a-z0-9][a-z0-9._-]*)+\.mbf";batchmode=yes\Z')
CONTROL = re.compile(r"[\x00-\x1f\x7f]")
NAME = re.compile(r"^[a-z0-9][a-z0-9._-]{0,127}\Z")

PRINT_CHECK_LABEL = "section titles of the print (all present, or which are missing)"
FIRST_LINE_LABEL = ("A. the dry run's first case line, pasted alone: what the Command Prompt and the program did, and "
                    "the files the line left in its case's folder, by name")
BATCH_SAMPLE_LABEL = ("B. the batch sample (sample-dyn-batch), whose file carries the dynamics options record (OPMISC, "
                      "with a number of modes) and the spectrum table: whether its print lists modes, and the section "
                      "titles it holds")
IMPORT_MODEL_LABEL = ("C. after the import in the window sample's folder, whether the import wrote a model file, and "
                      "where (the export page, CAEPIPE-EXPORT-MBF, says \"" + EXPORT_OVERWRITE + "\" and does not say in "
                      "which folder)")
# The export: the label each run that offers one carries, and the labels for check (v)'s answers on the export
# trial's run line, each with the words of check (v)'s step it answers (refused at build if a step lacks them).
EXPORT_LABEL = "export (made, or not made and why)"
EXPORT_TRIAL_ANSWERS = (
    (3, "(v) step 3, before touching the dialog, the files the folder now holds, by name",
     ("Before touching the dialog", "the files the folder now holds, by name")),
    (5, "(v) step 5, whether the dialog lets you set the name sample-dyn-export-trial.export.mbf and this folder, and "
        "whether it displays them before anything is written",
     ("whether the dialog lets you set the name sample-dyn-export-trial.export.mbf and this folder, and whether it "
      "displays them before anything is written",)),
    (6, "(v) step 6, Export or Cancel (\"If it does, click Export; if not, click Cancel\")",
     ("If it does, click Export; if not, click Cancel",)),
    (7, "(v) step 7, the files the folder then holds, by name", ("the files the folder then holds, by name",)),
)


class BuildError(ValueError):
    """The pack cannot be built from these inputs; nothing is written."""


def _sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _json_bytes(value: Any) -> bytes:
    return (json.dumps(value, sort_keys=True, indent=2, ensure_ascii=False, allow_nan=False) + "\n").encode("utf-8")


def _crlf(text: str) -> bytes:
    return text.replace("\r\n", "\n").replace("\n", "\r\n").encode("utf-8")


def kind_problem(name: str, kinds: Iterable[str]) -> str | None:
    """Why `name` is not of one of `kinds`, or None. The kind is the last extension, compared without regard to letter
    case: `x.cmd.txt` is plain text, `x.txt.cmd` is not. A name with no extension, or ending in a dot or a space, is
    refused: Windows drops a trailing dot or space, so the kind it reads would not be the one the name shows."""
    if not name or name.endswith((".", " ")):
        return f"{name!r} ends in a dot or a space, or is empty"
    suffix = Path(name).suffix.lower()
    if not suffix:
        return f"{name!r} has no extension"
    if suffix not in kinds:
        return f"{name!r} is of the kind {suffix}, which is not one of {', '.join(sorted(kinds))}"
    return None


def section_titles() -> tuple[str, ...]:
    """The section titles the maintained print parser matches (core/handoff/caepipe_external/print_csv.py `_SPECS`),
    from its own patterns, `<case>` standing for a load case's name; the one section it skips is left out."""
    from core.handoff.caepipe_external import print_csv as _print_csv
    titles = []
    for spec in _print_csv._SPECS.values():
        if "headers" not in spec:
            continue
        text = spec["pattern"].pattern.strip("^$")
        text = text.replace("\\s*(?P<case>.+)", " <case>").replace("(?P<case>.+?)", "<case>")
        text = text.replace("\\s*", " ").replace("\\(", "(").replace("\\)", ")")
        titles.append(text)
    return tuple(titles)


def command_line_problem(line: str) -> str | None:
    """Why a line of BATCH_COMMANDS.txt is not one of its allowed forms, or None: blank, one of the two set-up lines
    exactly as the pack writes them, or `start "" /wait` with the documented batch command and a path of plain
    lower-case names below the current folder; no control character anywhere."""
    if CONTROL.search(line):
        return f"a control character: {line!r}"
    if line == "" or line in BATCH_SETUP or START_LINE.match(line):
        return None
    return f"not an allowed form: {line!r}"


def command_file_problems(data: bytes) -> list[str]:
    """Every problem of a BATCH_COMMANDS.txt: CRLF lines, each of an allowed form."""
    text = data.decode("ascii", errors="replace")
    if not text.endswith("\r\n") or "\r\n" not in text:
        return ["the file does not end its lines with CRLF"]
    lines = text[:-2].split("\r\n")
    return [problem for line in lines if (problem := command_line_problem(line))]


# -- inputs ---------------------------------------------------------------------------------------------------------
def load_plan(path: Path = PLAN) -> dict[str, Any]:
    plan = json.loads(Path(path).read_text(encoding="utf-8"))
    if plan.get("document_kind") != PLAN_KIND:
        raise BuildError(f"{path.name}: document_kind must be {PLAN_KIND}")
    kinds = [part.get("kind") for part in plan.get("parts") or []]
    if kinds != list(PART_KINDS):
        raise BuildError(f"{path.name}: the parts must be, in order, {', '.join(PART_KINDS)}")
    for part in plan["parts"]:
        for key in ("id", "title", "buys"):
            if not isinstance(part.get(key), str) or not part[key].strip():
                raise BuildError(f"{path.name}: part {part.get('id')!r} needs a {key}")
    return plan


def load_manifest(path: Path = MANIFEST) -> tuple[dict[str, Any], dict[str, dict[str, Any]]]:
    """(the manifest, {model id: its document}), each document checked against the manifest's file and model hashes."""
    manifest = json.loads(Path(path).read_text(encoding="utf-8"))
    if manifest.get("document_kind") != MANIFEST_KIND:
        raise BuildError(f"{path.name}: document_kind must be {MANIFEST_KIND}")
    documents: dict[str, dict[str, Any]] = {}
    for entry in manifest["models"]:
        model_id = entry["model_id"]
        if not NAME.match(model_id) or model_id in documents or entry.get("file") != f"models/{model_id}.json":
            raise BuildError(f"{path.name}: model {model_id!r} is invalid, repeated or names another file")
        data = (Path(path).parent / entry["file"]).read_bytes()
        if _sha256(data) != entry.get("file_sha256"):
            raise BuildError(f"{entry['file']} differs from the manifest's file_sha256")
        document = json.loads(data.decode("utf-8"))
        if (document.get("document_kind") != MODEL_KIND or document.get("model_id") != model_id
                or canonical_model_hash(document.get("model")) != entry.get("model_hash")):
            raise BuildError(f"{entry['file']} is not the model the manifest names")
        documents[model_id] = document
    return manifest, documents


# -- one case -------------------------------------------------------------------------------------------------------
def mbf_settings(text: str) -> dict[str, Any]:
    """The option records the writer writes (every record between VERSION and MATERIAL) and what they mean."""
    lines = text.split("\r\n")
    records: list[dict[str, str]] = []
    if "VERSION" in lines:
        at = lines.index("VERSION")
        while at + 1 < len(lines) and lines[at] != "MATERIAL":
            records.append({"record": lines[at], "value": lines[at + 1]})
            at += 2
    values = {record["record"]: record["value"] for record in records}
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


def _written(document: Mapping[str, Any]) -> Any:
    result = render_caepipe_mbf_si(document["model"], title=document["model_id"],
                                   reference_temperature_c=document.get("reference_temperature_c", 20.0),
                                   piping_code=document["piping_code"])
    if result.status != "written":
        codes = sorted({entry["code"] for entry in result.diagnostics if entry["severity"] == "error"})
        raise BuildError(f"{document['model_id']}: the writer refused it ({', '.join(codes)})")
    return result


def provenance(entry: Mapping[str, Any], run: Mapping[str, Any] | None, substituted: list[str],
               mbf_sha: str, generator: Mapping[str, Any]) -> bytes:
    """The provenance sidecar of one packed model file, in the form the repository's data-boundary scan reads
    (`produced_by_caepipe`, and `origin` naming it invented), saying only what is established of the file: it is the
    model-file writer's output from the generator's model document, through run_mbf_text for a declared run, with what
    was substituted or added by name, and its own SHA-256."""
    tranche = str(entry.get("tranche"))
    basis = (f"generator-made; drawn from seed {generator['seed']}" if tranche in ("1", "2")
             else "generator-made; designed in the generator's text, drawing nothing from the seed")
    return _json_bytes({
        "produced_by_caepipe": False,
        "origin": "invented",
        "origin_basis": basis,
        "generator": {"module": generator["module"], "version": generator["version"], "tranche": tranche},
        "written_by": "SWBPIPE's model-file writer (core/handoff/caepipe_mbf)",
        "model_document": {"path": f"validation/caepipe_benchmark/{entry['file']}", "file_sha256": entry["file_sha256"]},
        "run": None if run is None else {
            "run_id": run["run_id"], "series": run["series"], "through": "benchmark_models.run_mbf_text",
            "substituted_records": substituted,
            "added_records": [record["heading"] for record in (run.get("mbf") or {}).get("added_records") or []]},
        "mbf_sha256": mbf_sha,
    })


def run_line_text(item: Mapping[str, Any]) -> str:
    routes = " or ".join(item["routes"])
    lines = [f"SWBPIPE sitting run line: {item['item_id']}",
             "Fill in after each colon. Do not change the words before the colon.",
             "date: ",
             "run (yes, or not run and why): ",
             f"route used ({routes}): ",
             "CAEPIPE version as displayed: ",
             "anything unusual: "]
    if item.get("export"):
        lines.append(f"{EXPORT_LABEL}: ")
    if item.get("print_check"):
        lines.append(f"{PRINT_CHECK_LABEL}: ")
    if item.get("dry_run_checks"):
        lines += [f"{label}: " for label in item["dry_run_checks"]]
    return "\r\n".join(lines) + "\r\n"


def case_item(document: Mapping[str, Any], entry: Mapping[str, Any], run: Mapping[str, Any] | None,
              part: Mapping[str, Any], files: dict[str, bytes], manifest: Mapping[str, Any]) -> dict[str, Any]:
    """The index entry of one case, its files added to `files`. A declared run's file must have the run's declared
    SHA-256; a model's, the manifest's `mbf_sha256`; a file without a declared hash refuses the build."""
    key = run["run_id"] if run else document["model_id"]
    if not NAME.match(key):
        raise BuildError(f"{key!r} is not a plain lower-case name")
    written = _written(document)
    text = benchmark_models.run_mbf_text(document, written.text)
    data = text.encode("utf-8")
    mbf_sha = _sha256(data)
    declared = (run.get("mbf") or {}).get("sha256") if run else entry.get("mbf_sha256")
    if run is not None and declared is None:
        raise BuildError(f"{key}: the manifest declares no SHA-256 for this run's file")
    if declared is None:
        raise BuildError(f"{key}: the manifest declares no model-file hash (mbf_sha256) for this model")
    if declared is not None and declared != mbf_sha:
        raise BuildError(f"{key}: the model file does not have the SHA-256 the manifest declares for it")
    writer_sha = _sha256(written.text.encode("utf-8"))
    window = run is not None and "window_instruction" in run
    folder, stem = f"cases/{key}", key
    item: dict[str, Any] = {
        "item_id": key, "item_kind": "benchmark_case", "part": part["id"], "folder": folder, "stem": stem,
        "model_id": document["model_id"], "family": entry.get("family"), "tranche": entry.get("tranche"),
        "series_run": run.get("series") if run else None, "run_kind": (run or {}).get("kind"),
        "routes": [WINDOW] if window else [BATCH, WINDOW],
        "source": {"file": f"validation/caepipe_benchmark/{entry['file']}", "file_sha256": entry["file_sha256"],
                   "model_hash": entry["model_hash"]},
        "mbf_declared_sha256": declared,
    }
    capture_only = bool(document.get("capture_only")) or (run is not None and not run.get("screened", True))
    item["capture"] = "sealed" if capture_only else "screened"
    # How the index designates the case's return: held (a window case: option addition or hand entry, held until
    # its screening's condition holds), sealed (a capture-only run), or screened (every other case).
    item["designation"] = ("held" if (run or {}).get("kind") in ("option_addition", "hand_entry")
                           else "sealed" if capture_only else "screened")
    if run is not None and run["series"] == PRESSURE_CAPTURE:
        item["sealed_reason"] = ("declared capture-only in the benchmark manifest: no SWBPIPE state of the model "
                                 "equals this run")
    holds: list[str] = []
    if run is not None and run.get("kind") == "hand_entry":
        holds += sorted(set((entry.get("held") or {}).values())) or [manifest["hand_entry"]["held"]]
        description = str(document.get("description", ""))
        if "NOT held equal" in description:  # the generator's own statement of an input not held equal, verbatim
            start = description.rfind(". ", 0, description.index("NOT held equal"))
            item["not_held_equal"] = description[start + 2 if start >= 0 else 0:].strip()
        if (document.get("hand_entry") or {}).get("one_way_case"):
            holds.append(document["hand_entry"]["one_way_case"])
    elif run is not None and run.get("screening_waits_for"):
        holds.append(run["screening_waits_for"])
    item["hold"] = holds
    # The identity file binds the writer's text where a run adds whole records (that file is not the bound file);
    # otherwise the case's own file, as before.
    bound_sha = writer_sha if (run or {}).get("kind") == "option_addition" else mbf_sha
    identity = (json.dumps(written.sidecar(bound_sha), sort_keys=True, indent=2, ensure_ascii=True, allow_nan=False)
                + "\n").encode("ascii")
    report = _json_bytes({"profile_id": written.to_dict()["profile_id"], "status": written.status,
                          "layout_row_count": written.layout_row_count, "piping_code": document["piping_code"],
                          "diagnostics": written.diagnostics, "loss_report": written.loss_report,
                          "assumptions": written.assumptions, "mbf_sha256": mbf_sha, "writer_sha256": writer_sha,
                          "oppres_line": document.get("oppres_line"),
                          "added_records": (run or {}).get("mbf", {}).get("added_records")})
    records = f"{RECORDS}/{folder}"
    files[f"{PACK}/{folder}/{stem}.mbf"] = data
    substituted = ["OPPRES"] if (run is not None and (run.get("kind") is None)
                                 and text != written.text) else []
    files[f"{PACK}/{folder}/{stem}.provenance.json"] = provenance(entry, run, substituted, mbf_sha,
                                                                   manifest["generator"])
    item["provenance"] = {"file": f"{stem}.provenance.json"}
    files[f"{records}/{stem}.swbpipe-ids.json"] = identity
    files[f"{records}/{stem}.writer-report.json"] = report
    item.update(layout_row_count=written.layout_row_count,
                mbf={"file": f"{stem}.mbf", "sha256": mbf_sha, "bytes": len(data), "writer_sha256": writer_sha},
                records={"identity_file": {"path": f"{records}/{stem}.swbpipe-ids.json",
                                           "binds": "writer_text" if bound_sha == writer_sha and bound_sha != mbf_sha
                                           else "case_file", "bound_sha256": bound_sha},
                         "writer_report": {"path": f"{records}/{stem}.writer-report.json"}},
                settings=mbf_settings(text))
    if window:
        instruction = run["window_instruction"]
        body = instruction["text"].encode("utf-8")
        if _sha256(body) != instruction["sha256"]:
            raise BuildError(f"{key}: the window instruction's text does not have the SHA-256 the manifest declares")
        files[f"{PACK}/{folder}/{stem}.window-instruction.txt"] = body
        item["window_instruction"] = {"file": f"{stem}.window-instruction.txt", "sha256": instruction["sha256"]}
        item["results"] = {WINDOW: [run["expected_print_file"], run["expected_saved_file"], f"{stem}.res"]}
        if run.get("form"):
            item["form"] = run["form"]
        if run.get("single_form"):
            item["single_form"] = True
        if run.get("strike_order") is not None:
            item["strike_order"] = run["strike_order"]
        if run.get("counterpart_run_id"):
            item["pair"] = {"form": run["form"], "counterpart": run["counterpart_run_id"],
                            "strike_order": run["strike_order"]}
        if run.get("export"):
            exported = run["export"]["file"]
            if exported != f"{stem}.export.mbf" or item["routes"] != [WINDOW]:
                raise BuildError(f"{stem}: an export is offered only to a window-only run, as <run id>.export.mbf")
            item["export"] = {"file": exported, "optional": run["export"]["optional"]}
            item["results"][WINDOW].append(exported)
            item["result_conditions"] = {exported: "if the export was made"}
        spectrum = [record["lines"] for record in (run.get("mbf") or {}).get("added_records") or []
                    if record["heading"] == "SPCMS"]
        if spectrum:
            item["spectrum_table"] = spectrum[0]
    else:
        if run is not None and run.get("export"):
            raise BuildError(f"{stem}: an export is offered only to a window-only run, as <run id>.export.mbf")
        item["results"] = {BATCH: [f"{stem}.csv (or {stem}.txt)", f"{stem}.mod", f"{stem}.res"],
                           WINDOW: [f"{stem}.print.csv", f"{stem}.mod", f"{stem}.res"]}
    return item


def export_trial_steps(manifest: Mapping[str, Any]) -> dict[int, str]:
    """Check (v)'s numbered steps, by number."""
    check_v = next(check for check in manifest["dry_run"]["checks"] if check.startswith("(v)"))
    steps = {}
    for line in check_v.split("\n")[1:]:
        number, _, text = line.partition(". ")
        steps[int(number)] = text
    return steps


def sample_items(run_entry: Mapping[str, Any], dyn_entries: list[Mapping[str, Any]], data: bytes,
                 samples: Mapping[str, str], part: Mapping[str, Any], files: dict[str, bytes],
                 manifest: Mapping[str, Any]) -> list[dict[str, Any]]:
    """The three dry-run samples (batch, window, and the export trial's sacrificial duplicate): each a copy, under its
    own id and folder, of one dynamics run file in the form that carries both added records, of the smallest dynamics
    model (fewest layout rows)."""
    entry, run = run_entry["entry"], run_entry["run"]
    smallest = min(dyn_entries, key=lambda e: (e["layout_row_count"], e["model_id"]))
    headings = [record["heading"] for record in run["mbf"]["added_records"]]
    if entry["model_id"] != smallest["model_id"] or headings != ["OPMISC", "SPCMS"]:
        raise BuildError("the dry-run samples must copy the smallest dynamics model's run file that adds both records")
    if _sha256(data) != run["mbf"]["sha256"]:
        raise BuildError("the dry-run samples' file does not have its run's declared SHA-256")
    records = {record["heading"]: record["lines"][1:] for record in run["mbf"]["added_records"]}
    opmisc = records["OPMISC"][0].split(",")
    table = [line.split(",") for line in records["SPCMS"][1:]]
    stated = {
        "spectrum": {"name": records["SPCMS"][0].split(",")[0], "entries": [[t[1], t[2]] for t in table],
                     "units_as_read": ("the file's unit codes are 0 for the abscissa and 3 for the ordinate, which "
                                       "the benchmark manifest reads, by inference, as frequency (Hz) and "
                                       "acceleration (mm/sec2)")},
        "cutoff_frequency": opmisc[0], "number_of_modes": opmisc[1],
        "cutoff_unit": ("The vendor's public import page gives the file's cutoff frequency value no unit; Hertz is "
                        "taken from the analysis page's description of the window's Cutoff Frequency field."),
    }
    checks = [check for check in manifest["dry_run"]["checks"] if not check.startswith("(v)")]
    trial = manifest["dry_run"]["export_trial"]
    trial_id = trial["file"][:-len(".mbf")]
    if (trial["copy_of_run"] != run["run_id"] or trial["folder"] != f"samples/{trial_id}"
            or trial["file"] != f"{trial_id}.mbf"):
        raise BuildError("the export trial must copy the dry-run samples' run file into samples/<its id>/")
    out = []
    for route, item_id in ((BATCH, samples["batch"]), (WINDOW, samples["window"]), (WINDOW, trial_id)):
        folder = f"samples/{item_id}"
        files[f"{PACK}/{folder}/{item_id}.mbf"] = data
        files[f"{PACK}/{folder}/{item_id}.provenance.json"] = provenance(entry, run, [], _sha256(data),
                                                                         manifest["generator"])
        item = {"item_id": item_id, "item_kind": "dry_run_sample", "part": part["id"], "folder": folder,
                "stem": item_id, "routes": [route], "capture": "stored_only", "designation": "stored_only", "hold": [],
                "provenance": {"file": f"{item_id}.provenance.json"},
                "copy_of": {"run_id": run["run_id"], "model_id": entry["model_id"], "sha256": run["mbf"]["sha256"]},
                "spectrum_table": [r["lines"] for r in run["mbf"]["added_records"] if r["heading"] == "SPCMS"][0],
                "mbf": {"file": f"{item_id}.mbf", "sha256": _sha256(data), "bytes": len(data)},
                "layout_row_count": entry["layout_row_count"]}
        if route == BATCH:
            item["results"] = {BATCH: [f"{item_id}.csv (or {item_id}.txt)", f"{item_id}.mod", f"{item_id}.res"]}
            item["observe"] = BATCH_SAMPLE_LABEL
        elif item_id == trial_id:
            check_v = export_trial_steps(manifest)
            for step, _label, words in EXPORT_TRIAL_ANSWERS:
                if any(w not in check_v.get(step, "") for w in words):
                    raise BuildError(f"check (v) step {step} no longer holds the words its answer's label quotes")
            exported = f"{item_id}.export.mbf"
            item["export_trial"] = {"untouched": list(trial["untouched"]), "made_by": trial["made_by"]}
            item["observe"] = "check (v) of the dry run, in this folder only (INSTRUCTIONS.md section 4, step 6)"
            item["results"] = {WINDOW: [exported]}
            item["results_first"] = "the model file the import writes, if it writes one (answer C)"
            item["result_conditions"] = {exported: "if check (v) allowed the export"}
            item["dry_run_checks"] = [label for _step, label, _words in EXPORT_TRIAL_ANSWERS]
        else:
            item["results"] = {WINDOW: [f"{item_id}.mod"]}
            item["stated_values"] = stated
            item["dry_run_checks"] = [IMPORT_MODEL_LABEL, *checks]
        out.append(item)
    return out


# -- the pack -------------------------------------------------------------------------------------------------------
def _tranche1_order(items: list[dict[str, Any]], entries: Mapping[str, Mapping[str, Any]]) -> list[dict[str, Any]]:
    """By family; the members of one series together, each series or lone model placed by its smallest layout row
    count, then its first id."""
    ordered: list[dict[str, Any]] = []
    for family in sorted({item["family"] for item in items}):
        units: dict[str, list[dict[str, Any]]] = {}
        for item in (i for i in items if i["family"] == family):
            series = sorted(entries[item["model_id"]].get("series") or [])
            units.setdefault(f"series:{series[0]}" if series else f"model:{item['model_id']}", []).append(item)
        keyed = sorted(((min(i["layout_row_count"] for i in members), sorted(i["item_id"] for i in members)[0]),
                        sorted(members, key=lambda i: i["item_id"])) for members in units.values())
        for _, members in keyed:
            ordered.extend(members)
    return ordered


def build_pack(manifest_path: Path = MANIFEST, plan_path: Path = PLAN) -> dict[str, bytes]:
    """The pack's files, {relative path: bytes}. Same inputs, same bytes."""
    plan = load_plan(plan_path)
    manifest, documents = load_manifest(manifest_path)
    entries = {entry["model_id"]: entry for entry in manifest["models"]}
    runs = [{"entry": entry, "run": run} for entry in manifest["models"] for run in entry.get("runs") or []]
    files: dict[str, bytes] = {}
    parts_out: list[dict[str, Any]] = []
    items: list[dict[str, Any]] = []
    placed: set[str] = set()

    def run_case(pair: Mapping[str, Any], part: Mapping[str, Any]) -> dict[str, Any]:
        document = benchmark_models.series_document(copy.deepcopy(documents[pair["entry"]["model_id"]]),
                                                    pair["run"]["series"])
        if document["run_id"] != pair["run"]["run_id"]:
            raise BuildError(f"{pair['run']['run_id']}: the run's document names another run id")
        return case_item(document, pair["entry"], pair["run"], part, files, manifest)

    def model_case(model_id: str, part: Mapping[str, Any]) -> dict[str, Any]:
        entry = entries[model_id]
        if entry.get("runs"):
            raise BuildError(f"{model_id} declares runs; it is packed only as its runs")
        return case_item(documents[model_id], entry, None, part, files, manifest)

    for part in plan["parts"]:
        kind = part["kind"]
        chosen: list[dict[str, Any]] = []
        if kind == "dry_run":
            chosen = [model_case(model_id, part) for model_id in part["model_ids"]]
            chosen[0]["print_check"] = True
            sample_run = next(pair for pair in runs if pair["run"]["run_id"] == part["samples"]["run_id"])
            document = benchmark_models.series_document(copy.deepcopy(documents[sample_run["entry"]["model_id"]]),
                                                        sample_run["run"]["series"])
            data = benchmark_models.run_mbf_text(document, _written(document).text).encode("utf-8")
            dyn = [entry for entry in manifest["models"] if str(entry["family"]).startswith("DYN")]
            chosen += sample_items(sample_run, dyn, data, part["samples"], part, files, manifest)
        elif kind == "dynamics":
            pairs = [pair for pair in runs if pair["run"].get("kind") == "option_addition"]
            pairs.sort(key=lambda p: (p["run"].get("strike_order") or 0, p["run"].get("form") != "table_added",
                                      p["run"]["run_id"]))
            chosen = [run_case(pair, part) for pair in pairs]
        elif kind == "pressure":
            pressure = sorted((p for p in runs if p["run"]["series"] in (*PRESSURE_SCREENED, PRESSURE_CAPTURE)),
                              key=lambda p: (p["entry"]["family"], p["entry"]["model_id"]))
            screened = [p for p in pressure if p["run"]["series"] in PRESSURE_SCREENED]
            screened.sort(key=lambda p: (p["entry"]["family"], p["entry"]["model_id"],
                                         PRESSURE_SCREENED.index(p["run"]["series"])))
            chosen = [run_case(pair, part) for pair in screened]
            chosen += [run_case(pair, part) for pair in pressure if pair["run"]["series"] == PRESSURE_CAPTURE]
        elif kind == "static_series":
            for family in part["families"]:
                members = [entry for entry in manifest["models"] if entry["family"] == family]
                names = sorted({name for entry in members for name in entry.get("series") or []})
                if len(names) != 1:
                    raise BuildError(f"family {family} must form one series")
                order = [member["model_id"] for member in manifest["series"][names[0]]["members"]]
                if sorted(order) != sorted(entry["model_id"] for entry in members):
                    raise BuildError(f"series {names[0]} must name every model of family {family}")
                chosen += [model_case(model_id, part) for model_id in order]
        elif kind == "hand_entry":
            hand = sorted((p for p in runs if p["run"].get("kind") == "hand_entry"),
                          key=lambda p: p["entry"]["hand_entry_rank"])
            chosen = [run_case(pair, part) for pair in hand]
        elif kind == "tranche1_rest":
            rest = [model_case(entry["model_id"], part) for entry in manifest["models"]
                    if entry["tranche"] == "1" and entry["model_id"] not in placed]
            chosen = _tranche1_order(rest, entries)
        for item in chosen:
            if item["item_id"] in placed:
                raise BuildError(f"{item['item_id']} is placed twice")
            placed.add(item["item_id"])
        items += chosen
        parts_out.append({"id": part["id"], "kind": kind, "title": part["title"], "buys": part["buys"],
                          "items": [item["item_id"] for item in chosen]})
    expected = {entry["model_id"] for entry in manifest["models"] if not entry.get("runs")}
    expected |= {pair["run"]["run_id"] for pair in runs}
    cases = {item["item_id"] for item in items if item["item_kind"] == "benchmark_case"}
    if cases != expected:
        raise BuildError(f"cases not placed: {sorted(expected - cases)}; placed but not declared: "
                         f"{sorted(cases - expected)}")
    for order, item in enumerate(items, 1):
        item["order"] = order

    # The manifest's rule runs form A only if the dry run's check of the sample carrying the same spectrum table
    # agreed: a form-A run whose table no dry-run sample carries could never be run, so the build is refused.
    sample_tables = [item["spectrum_table"] for item in items if item["item_kind"] == "dry_run_sample"]
    for item in items:
        if item.get("form") == "table_added" and item.get("spectrum_table") not in sample_tables:
            raise BuildError(f"{item['item_id']}: no dry-run sample carries this run's spectrum table")
    qualification = QUALIFICATION.encode("utf-8")
    files[f"{PACK}/{QUALIFICATION_FILE}"] = qualification
    qualification_ref = {"file": QUALIFICATION_FILE, "sha256": _sha256(qualification)}
    for item in items:
        if any(GATE in sentence for sentence in item["hold"]):
            item["qualification"] = qualification_ref
    for item in items:
        files[f"{PACK}/{item['folder']}/run_line.txt"] = _crlf(run_line_text(item))

    index: dict[str, Any] = {
        "document_kind": INDEX_KIND, "pack_version": PACK_VERSION, "builder": BUILDER,
        "builder_note": ("a development tool: the reviewed pack is the committed folder " + PACK_FOLDER
                         + "; regenerating it is a consistency test, not the trust anchor"),
        "generator": manifest["generator"],
        "inputs": {"manifest": {"path": "validation/caepipe_benchmark/manifest.json",
                                "sha256": _sha256(Path(manifest_path).read_bytes())},
                   "plan": {"path": "tools/caepipe_correlation/sitting_pack_plan.json",
                            "sha256": _sha256(Path(plan_path).read_bytes())}},
        "kinds": sorted(PACK_KINDS), "return_kinds": sorted(RETURN_KINDS), "routes": [BATCH, WINDOW],
        "qualification": qualification_ref, "parts": parts_out, "items": items,
    }
    files[f"{PACK}/{BATCH_COMMANDS}"] = _crlf(batch_commands_text(index))
    files[f"{PACK}/{RUN_SHEET}"] = _crlf(run_sheet_text(index, manifest))
    files[f"{PACK}/{INSTRUCTIONS}"] = _crlf(instructions_text(index, manifest))
    problems = command_file_problems(files[f"{PACK}/{BATCH_COMMANDS}"])
    if problems:
        raise BuildError(f"{BATCH_COMMANDS}: {problems}")
    for name in files:
        problem = kind_problem(name.rsplit("/", 1)[-1], PACK_KINDS)
        if problem or any(not NAME.match(part.lower()) for part in name.split("/")):
            raise BuildError(f"{name}: {problem or 'not a plain name'}")
    if any(name.split("/", 1)[0] not in (PACK, RECORDS) for name in files):
        raise BuildError("every file is in pack/ or records/")
    index["files"] = {name: {"sha256": _sha256(data), "bytes": len(data), "folder": name.split("/", 1)[0]}
                      for name, data in sorted(files.items())}
    files[INDEX_PATH] = _json_bytes(index)
    return files


def pack_identity(files: Mapping[str, bytes]) -> str:
    return _sha256(files[INDEX_PATH])


# -- the texts ------------------------------------------------------------------------------------------------------
def batch_line(folder: str, mbf_file: str) -> str:
    return f'start "" /wait "%CAEPIPE%" "%CD%\\{folder.replace("/", chr(92))}\\{mbf_file}";batchmode=yes'


def batch_items(index: Mapping[str, Any], part: Mapping[str, Any]) -> list[Mapping[str, Any]]:
    by_id = {item["item_id"]: item for item in index["items"]}
    return [by_id[item_id] for item_id in part["items"] if BATCH in by_id[item_id]["routes"]]


def batch_commands_text(index: Mapping[str, Any]) -> str:
    """The two set-up lines, then one line per batch-route case in the run sheet's order, each part's lines together
    and one blank line between parts. A window-only case has no line."""
    groups = ["\n".join(BATCH_SETUP)]
    for part in index["parts"]:
        lines = [batch_line(item["folder"], item["mbf"]["file"]) for item in batch_items(index, part)]
        if lines:
            groups.append("\n".join(lines))
    return "\n\n".join(groups) + "\n"


def _kind_label(item: Mapping[str, Any]) -> str:
    if item["item_kind"] == "dry_run_sample":
        if item.get("export_trial"):
            return "dry-run sample, export trial"
        return "dry-run sample, batch" if item["routes"] == [BATCH] else "dry-run sample, window"
    label = {"held": "held capture", "sealed": "sealed capture"}.get(item.get("designation"), "benchmark case")
    return label + (", window only" if item["routes"] == [WINDOW] else "")


def _sentence_starting(text: str, start: str) -> str:
    at = text.index(start)
    end = text.find(". ", at)
    return text[at:end + 1] if end >= 0 else text[at:]


def part1_rule(index: Mapping[str, Any]) -> str:
    """Part 1 is never struck for a later part; within it, a complete strike number is the unit that is cut, highest
    first (the three highest numbers named from the index)."""
    highest = max(item.get("strike_order") or 0 for item in index["items"])
    return ("This part is never struck to make room for a later part. If time runs out within it, stop after a complete "
            "strike number: the lowest numbers are run first, and strike numbers "
            f"{highest - 2} to {highest} are the first to go.")


def shortening(index: Mapping[str, Any]) -> str:
    rule = part1_rule(index).replace("This part is never struck", "Part 1 is never struck", 1)
    return ("If the sitting must be shortened, strike parts from the end: part 5 first, then part 4, part 3 and part 2. "
            + rule)


def run_sheet_text(index: Mapping[str, Any], manifest: Mapping[str, Any]) -> str:
    by_id = {item["item_id"]: item for item in index["items"]}
    pair_rule = _sentence_starting(manifest["option_addition"]["pairs"], "Exactly one of a pair")
    out = [QUALIFICATION, "", QUALIFICATION_FOLLOW, "", "# SWBPIPE sitting run sheet", "",
           "Run the parts in this order. " + shortening(index), "",
           "Tick each line when you have run its case, or written on its run line why not.", ""]
    group = 0
    for number, part in enumerate(index["parts"]):
        out += [f"## Part {number}: {part['title']}", "", part["buys"], "", evidence_line(index, part, number), ""]
        batch = batch_items(index, part)
        if batch:
            group += 1
            out += [f"Batch lines: group {group} of `{BATCH_COMMANDS}` after the two set-up lines, {len(batch)} "
                    "line(s), pasted one line at a time in this order.", ""]
        if part["kind"] == "dynamics":
            out += ["Window only: no case of this part has a line in `BATCH_COMMANDS.txt`.",
                    "", f"From the benchmark manifest: \"{pair_rule}\"", ""]
            out += [f"From the benchmark manifest: \"{manifest['option_addition']['single_form']}\"", ""] \
                if any(by_id[i].get("single_form") for i in part["items"]) else []
            out += [ONE_SESSION, ""]
        if part["kind"] == "hand_entry":
            out += ["Window only: no case of this part has a line in `BATCH_COMMANDS.txt`.", ""]
        if part["kind"] == "dry_run":
            window = next(by_id[i] for i in part["items"] if by_id[i]["item_kind"] == "dry_run_sample"
                          and by_id[i]["routes"] == [WINDOW] and not by_id[i].get("export_trial"))
            trial = next(by_id[i] for i in part["items"] if by_id[i].get("export_trial"))
            consequences = manifest["dry_run"]["consequences"]
            numbered = [c[:c.index(")") + 1] for c in consequences]
            out += ["Write down what you observe, after each colon. Nothing here says what you will see. Write the "
                    f"answers to C and to checks (i) to (iv) also on the run line of `{window['item_id']}`, and the "
                    f"answers to check (v) on the run line of `{trial['item_id']}`.", "",
                    f"- {FIRST_LINE_LABEL}: ", f"- {BATCH_SAMPLE_LABEL}: ",
                    *[f"- {label}: " for label in window["dry_run_checks"]], "",
                    f"Check (v) is made in `{trial['folder']}` alone. From the benchmark manifest:", "",
                    check_v_text(manifest), "",
                    *[f"- {label}: " for label in trial["dry_run_checks"]], "",
                    f"The benchmark manifest's consequences of the checks {_and(numbered)}:", "",
                    *[f"- {consequence}" for consequence in consequences], "",
                    "- the consequences that apply (write their numbers, or none): ", ""]
        if part["kind"] == "dynamics":
            out += ["Tick the form you ran, and write \"not run: other form\" on the other form's run line. "
                    + part1_rule(index), "",
                    "| # | strike order | form | item | layout rows | MBF SHA-256 (first 12) | folder | done |",
                    "|---|---|---|---|---|---|---|---|"]
            forms = {"table_added": "A, table added", "table_typed": "B, table typed"}
            for item_id in part["items"]:
                item = by_id[item_id]
                form = "only form" if item.get("single_form") else forms.get(item.get("form"), "")
                out.append(f"| {item['order']} | {item.get('strike_order')} | {form} | {item_id} | "
                           f"{item['layout_row_count']} | {item['mbf']['sha256'][:12]} | `{item['folder']}` | [ ] |")
            out.append("")
            continue
        out += ["| # | item | kind | layout rows | MBF SHA-256 (first 12) | folder | done |",
                "|---|---|---|---|---|---|---|"]
        for item_id in part["items"]:
            item = by_id[item_id]
            out.append(f"| {item['order']} | {item_id} | {_kind_label(item)} | {item['layout_row_count']} | "
                       f"{item['mbf']['sha256'][:12]} | `{item['folder']}` | [ ] |")
        out.append("")
    return "\n".join(out)


def check_v_text(manifest: Mapping[str, Any]) -> str:
    return next(check for check in manifest["dry_run"]["checks"] if check.startswith("(v)"))


def evidence_parts(index: Mapping[str, Any]) -> tuple[list[int], list[int]]:
    """(the parts whose every case is window only, the parts whose every case is offered the batch line), by number;
    the dry run, which holds both, is named apart."""
    window, batch = [], []
    by_id = {item["item_id"]: item for item in index["items"]}
    for number, part in enumerate(index["parts"]):
        routes = [by_id[i]["routes"] for i in part["items"]]
        if part["kind"] == "dry_run" or not routes:
            continue
        if all(r == [WINDOW] for r in routes):
            window.append(number)
        elif all(BATCH in r for r in routes):
            batch.append(number)
    return window, batch


def _and(numbers: list[int]) -> str:
    names = [str(n) for n in numbers]
    return names[0] if len(names) == 1 else ", ".join(names[:-1]) + " and " + names[-1]


def evidence_line(index: Mapping[str, Any], part: Mapping[str, Any], number: int) -> str:
    window, batch = evidence_parts(index)
    if number in window:
        return ("Evidence of the model run: the model saved as `<run id>.mod` from inside the program before the "
                "analysis, brought back in the run's folder with its print (INSTRUCTIONS.md section 2).")
    if number in batch:
        return ("Evidence of the model run: whatever the program writes beside the model file its batch line names, "
                "brought back in that folder with the print; which files those are is the dry run's answer A "
                "(INSTRUCTIONS.md section 2).")
    return ("Evidence of the model run: for the batch lines, whatever the program writes beside each model file, with "
            "the print; for the window sample, the model saved as `sample-dyn-window.mod` (INSTRUCTIONS.md section 2).")


def expected_back(item: Mapping[str, Any]) -> str:
    """What comes back in an item's folder, one line per way, no file named twice."""
    named: list[str] = []
    lines = []
    for route in item["routes"]:
        new = [name for name in item["results"].get(route, []) if name not in named]
        named += new
        conditions = item.get("result_conditions") or {}
        files = ", ".join(f"`{name}`" + (f" ({conditions[name]})" if name in conditions else "") for name in new)
        if item.get("results_first"):
            files = f"{item['results_first']}, and {files}"
        if route == BATCH:
            lines.append(f"- by the batch line: {files}. Which of these the program writes is the dry run's answer A.")
        elif BATCH in item["routes"]:
            lines.append(f"- by the window way, only if the batch line left no results: {files}, besides the model and "
                         "results files named above")
        else:
            lines.append(f"- by the window way: {files}")
    text = "\n".join(lines) + "\n"
    if item["routes"] == [WINDOW] and item["item_kind"] == "benchmark_case":
        text += "\n" + KEEP_EVERYTHING + "\n"
    return text


def _ref(source_id: str) -> str:
    return f"{source_id}, {SOURCES[source_id]}"


def instructions_text(index: Mapping[str, Any], manifest: Mapping[str, Any]) -> str:
    items = index["items"]
    by_id = {item["item_id"]: item for item in items}
    generator = manifest["generator"]
    sections = "\n".join(f"   - `{title}`" for title in section_titles())
    dry = index["parts"][0]
    first = next(item for item in items if item.get("print_check"))
    batch_sample = next(by_id[i] for i in dry["items"] if by_id[i]["routes"] == [BATCH]
                        and by_id[i]["item_kind"] == "dry_run_sample")
    window_sample = next(by_id[i] for i in dry["items"] if by_id[i]["routes"] == [WINDOW]
                         and by_id[i]["item_kind"] == "dry_run_sample" and not by_id[i].get("export_trial"))
    window_parts, batch_parts = evidence_parts(index)
    window_parts, batch_parts = _and(window_parts), _and(batch_parts)
    check_iii = next(c for c in manifest["dry_run"]["checks"] if c.startswith("(iii)"))
    check_iii = "(iii): " + check_iii[len("(iii) "):]
    export = manifest["export"]
    trial = next(by_id[i] for i in dry["items"] if by_id[i].get("export_trial"))
    consequence_v = next(c for c in manifest["dry_run"]["consequences"] if c.startswith("(v)"))
    out = f"""{QUALIFICATION}

{QUALIFICATION_FOLLOW}

# SWBPIPE sitting: instructions

This pack is not a SWBPIPE result and holds none. It carries no driver, no script that runs at the sitting and no part of SWBPIPE's harness: you run every case by hand, in your own licensed CAEPIPE.

The pack holds the cases SWBPIPE asks you to run, in the order of `{RUN_SHEET}`. Every model file in it is the output of SWBPIPE's benchmark generator (`{generator['module']}` with its tranche modules) through SWBPIPE's model-file writer. The first and second tranches' members are drawn from seed {generator['seed']}; the third tranche's members are designed, each stated in the generator's own text, and draw nothing from the seed. For a run the benchmark manifest declares, the file is that run's: the written file, with its pressure options line substituted or whole options records added where the manifest declares that. The same inputs give the same bytes. {KINDS_CLAIM}: model files (`.mbf`), records (`.json`), these instructions and the run sheet (`.md`), and plain text (`.txt`). Its `{INDEX}` names every file of this folder, and of the `{RECORDS}` folder that stays on the Mac, with its SHA-256 and size.

**The results are private.** Every file that comes back in this pack is private. Keep the returned pack in your exchange folder or another private place outside every repository. Never commit, share or upload it.

**On the Mac.** {ON_THE_MAC} {SET_PASSAGE}

**What a result waits on.** {SCREEN_WAITS} A sealed capture is run like any other case and brought back; nothing in this pack unseals it.

## 1. Before you start

> **Never leave the pressure options line blank.** When you import or edit a model by hand, leave the line under the `OPPRES` heading (the pressure options) exactly as it is in the pack's model file, and never clear it. The vendor's public import page ({_ref('CAEPIPE-IMPORT-MBF')}, "Format of .MBF") lists among that line's fields "{IMPORT_OPPRES_FIELDS}" and says: "{IMPORT_OPPRES_BLANK}"

1. Copy the pack folder (`{PACK}`, the folder that holds these instructions) and nothing else to a fresh working folder on the Windows machine, at a short path of letters, digits and hyphens only, for example `C:\\swbpipe-sitting`. Choose a working folder that is not the exchange folder and is not inside it. Run nothing from the exchange folder. The `{RECORDS}` folder beside the pack folder stays on the Mac; never copy it.
2. Do not rename any folder or file the pack holds. Each case and sample folder holds one small provenance file, `<stem>.provenance.json`, beside its model file. Do not open it in the program; you need not bring it back.
3. Work down `{RUN_SHEET}` part by part, in order. Each case has its own folder; do everything for that case in it.
4. {shortening(index)}

## 2. What binds a result to the model that was run

A matching model hash shows which bytes were kept, not which model CAEPIPE actually ran, and evidence that is not made at the sitting cannot be made afterwards. What each kind of run makes, and what to bring back:

- **A window run** (parts {window_parts}, and a batch case run the window way): save the model under the run's name, `<run id>.mod`, from inside the program before the analysis, in the run's folder, and bring it back there with its print. That saved model is the evidence.
- **A batch run** (parts {batch_parts}, and the dry run's cases): the command line names the model file; bring back whatever the program writes beside it, in the same folder, with the print. Which files those are is the dry run's answer A. If answer A shows that a batch run leaves a model file of the program's own beside the print, that file is the evidence. If it shows none, a batch run's only evidence is that its print came back in the folder of the model file its command line named, and that is weaker evidence.
- **A run made by the one-session alternative:** the model saved under that run's name after the setting was changed, with the note on its run line, is the evidence.

{KEEP_EVERYTHING}

{EXPORT_BINDING}

**Exports.** The runs that offer an export, from the benchmark manifest: {export['runs']}. Of an export, the benchmark manifest says: "{export['statement']}." Each export's steps are optional, and you may drop them; if you do, write "export not made" on the run line. A run whose export is dropped is still run.

## 3. Two ways to run a case

Each case is offered one or both of two ways (`{BATCH}`, `{WINDOW}`), named for it in section 5. A window-only case is offered the program's window alone and has no line in `{BATCH_COMMANDS}`. Where a case is offered both ways, run it by its batch line; use the window way for that case only if its batch line left no results, and note that on its run line.

### 3.1 By hand, batch

The batch page ({_ref('CAEPIPE-BATCH')}) says: "{BATCH_TWO_ARGUMENTS}" Of its example it says: "{BATCH_EXAMPLE_OUTPUT}". Which files appear beside the model file is observed at the dry run (answer A).

`{BATCH_COMMANDS}` holds two set-up lines, then one line for every case this way can run, in the order of `{RUN_SHEET}`, each part's lines together and a blank line between parts. Open a Command Prompt. Type the first set-up line, with the full path of your CAEPIPE program in place of the text in angle brackets:

```
{BATCH_SETUP[0]}
```

Then type the second set-up line, with the full path of the pack folder in place of the text in angle brackets:

```
{BATCH_SETUP[1]}
```

Then paste the case lines one line at a time, in the file's order. Every case line has this form and nothing else:

```
{BATCH_LINE_SHAPE}
```

Microsoft's page for `start` ({_ref('MS-START')}) gives the words for its two parts used here: for `"title"` (the empty `""`), "{START_TITLE_WORDS}"; and for `/wait`, "{START_WAIT_WORDS}" Paste the next line only when the program has ended and the case's results are in its folder.

If a window of the program asks something or shows a message, answering or closing it is yours to do, by hand; nothing in the pack answers it. If a case ends with no results in its folder, note it on its run line, then run that case the window way (3.2) before pasting the next line. The import may write over the model file the batch line left in that folder (the export page's words in 3.2, step 1); note on the run line that the case was imported after its batch line.

### 3.2 By hand, in the program's window

1. Import `<stem>.mbf` with the program's import command (the import page, {_ref('CAEPIPE-IMPORT-MBF')}: "{IMPORT_COMMAND}"). The export page ({_ref('CAEPIPE-EXPORT-MBF')}) says: "{EXPORT_OVERWRITE}" The page does not say in which folder. Each run has its own folder for that reason. Then save the model as `<stem>.mod` in this folder by the command you noted at the dry run (check (iii)). Do not save over the imported file `<stem>.mbf`.
2. Before analysing, compare the model's settings as the window shows them with the case's settings in section 5: piping code, B31J, reference temperature, units, vertical axis, gravity and the pressure options. Do not edit the model file. If the window shows a different value, write the value it shows on the run line.
3. Analyse the model by File > Analyze (F12). The Analyze page ({_ref('CAEPIPE-ANALYZE')}, read 2026-11-13) says: "{ANALYZE_VIEW_RESULTS}" It also notes: "{ANALYZE_INACTIVE}" If Analyze is inactive, note it on the run line and go on to the next case.
4. Print the results to a file. The print page ({_ref('CAEPIPE-PRINT-TO-FILE')}) says of the Results tab: "{PRINT_RESULTS_TAB}" and of the To File button: "{PRINT_TO_FILE}" and "{PRINT_FILE_KINDS}" Choose the .csv file and save it as `<stem>.print.csv` in the case's folder. Ask for these sections, for every load case (`<case>` stands for the load case's name):

{sections}

A window-only case's folder holds `<stem>.window-instruction.txt`, the benchmark manifest's text for that run, byte for byte. Follow it step by step in place of the four steps above; this document does not restate it. Its print and its saved model take the names section 5 gives.

{ONE_SESSION}

## 4. The dry run comes first

Part 0 is the dry run. Write what you observe on `{RUN_SHEET}` under part 0, after each colon; nothing in this pack says what you will see.

1. Open a Command Prompt, type the two set-up lines (3.1) and paste the dry run's first case line ({first['item_id']}) alone. Write down answer A.
2. Open that case's print (`<stem>.csv`, or `<stem>.txt`) and compare its section titles with the list in 3.2, step 4. On that case's run line, after "{PRINT_CHECK_LABEL}", write `all present` or the titles that are missing.
3. Paste each other case line of part 0 one line at a time.
4. Paste the batch sample's line ({batch_sample['item_id']}). Write down answer B.
5. In the window sample's folder (`{window_sample['folder']}`), import `{window_sample['mbf']['file']}` in the program's window with the import command in 3.2, step 1. Write down answer C. Make checks (i) and (ii) against the stated values for this sample in section 5, and check (iv), and write down each answer. Then save the model as `{window_sample['stem']}.mod` in that folder, as a real window run saves its model as `<run id>.mod` in its own folder. The Save page ({_ref('CAEPIPE-SAVE')}, read 2026-11-13) says: "{SAVE_WORDS}" The File Menu page ({_ref('CAEPIPE-FILE-MENU')}, read 2026-11-13) lists no command for saving under another name. Then make check {check_iii}. Do not save over the imported file. Do not analyse or print. This sample has no line in `{BATCH_COMMANDS}`.
6. Then go to the export trial's folder (`{trial['folder']}`) and make check (v) there, in that folder only, as the benchmark manifest gives it below this list. Write each answer on that folder's run line. The window sample's folder (`{window_sample['folder']}`) and every case folder are not opened or touched by this trial.
7. Read the consequences listed under part 0 of `{RUN_SHEET}` and write down which apply.

8. After part 0, copy the working folder to a place of your choosing on the Mac, outside every repository, leaving the working folder on Windows as it is. Say when the copy has finished and where it is; say it only once your copying tool says the copy has finished, because the copy must not change while it is being stored. It is stored there under one new name and listed by name, size and hash. Wait until you are told it is stored, whether or not a title was missing, and then go on to part 1.
9. If a section title is missing from the first case's print, write which on that case's run line, and say so when the folder is brought back: what to do about it is decided then, before part 1.

Check (v), from the benchmark manifest:

{check_v_text(manifest)}

Its consequence, from the benchmark manifest:

{consequence_v}

Use the same working folder for the rest of the sitting. At the end, copy it to the Mac whole in the same way, and say when the copy has finished and where it is.

## 5. Per item

"""
    for item in items:
        out += f"### {item['order']}. {item['item_id']} ({_kind_label(item)})\n\n"
        ways = " and ".join(f"`{route}`" for route in item["routes"])
        out += (f"Folder `{item['folder']}`; model file `{item['mbf']['file']}`, layout rows "
                f"{item['layout_row_count']}; offered: {ways}.\n\n")
        if item["item_kind"] == "dry_run_sample":
            copy_of = item["copy_of"]
            out += (f"A copy of the model file of the run {copy_of['run_id']} (SHA-256 {copy_of['sha256'][:12]}), "
                    "which carries both added records. It is never screened or compared.\n\n")
            if item.get("stated_values"):
                stated = item["stated_values"]
                entries = "; ".join(f"{a}, {o}" for a, o in stated["spectrum"]["entries"])
                out += (f"Stated values: the spectrum {stated['spectrum']['name']}, "
                        f"{len(stated['spectrum']['entries'])} entries: {entries}; "
                        f"{stated['spectrum']['units_as_read']}. Cutoff Frequency {stated['cutoff_frequency']} Hz and "
                        f"Number of Modes {stated['number_of_modes']}. {stated['cutoff_unit']}\n\n")
            else:
                out += f"Observe: {item['observe']}.\n\n"
        if item.get("window_instruction"):
            out += (f"Window only: follow `{item['window_instruction']['file']}`. Print to "
                    f"`{item['results'][WINDOW][0]}`; saved model `{item['results'][WINDOW][1]}`.\n\n")
        if item.get("form"):
            form = "form A, spectrum table added" if item["form"] == "table_added" else "form B, spectrum table typed"
            counterpart = (f"; its counterpart is {item['pair']['counterpart']}" if item.get("pair") else
                           "; declared in this form only")
            out += f"Strike order {item.get('strike_order')}, {form}{counterpart}.\n\n"
        if item.get("single_form"):
            out += f"From the benchmark manifest: \"{manifest['option_addition']['single_form']}\"\n\n"
        if item.get("not_held_equal"):
            out += (f"From its model document (`{item['source']['file']}`): \"{item['not_held_equal']}\"\n\n")
        for sentence in item["hold"]:
            label = ("on its one-way support" if sentence == manifest["hand_entry"]["one_way_cases"]
                     else "how this run's capture is held" if item.get("run_kind") == "hand_entry"
                     else "what the screening of this run waits for")
            out += f"From the benchmark manifest, {label}: \"{sentence}\"\n\n"
        if item.get("qualification"):
            out += f"Its hold names the product's gate; `{QUALIFICATION_FILE}` applies.\n\n"
        if item.get("sealed_reason"):
            out += f"Sealed capture: {item['sealed_reason']}.\n\n"
        settings = item.get("settings")
        if settings and BATCH in item["routes"]:
            out += ("Settings to check in the window (3.2, step 2): piping code {pc}; B31J {b}; reference temperature "
                    "{t} C; units {u}; vertical axis {v}; gravity {g} m/s2; pressure options {p}.\n\n").format(
                pc=settings["piping_code"], b=settings["b31j"], t=settings["reference_temperature_c"],
                u=settings["units"], v=settings["vertical_axis"], g=settings["gravity_m_per_s2"],
                p=settings["pressure_options"])
        out += f"Expected back in this folder (`{item['folder']}` of the pack folder):\n\n" + expected_back(item) + "\n"
        out += "Run line (`run_line.txt`):\n\n```\n" + run_line_text(item).replace("\r\n", "\n") + "```\n\n"
    out += "## Sources\n\n" + "\n".join(f"- {key}: {url}" for key, url in SOURCES.items()) + "\n"
    return out


# -- writing --------------------------------------------------------------------------------------------------------
def write_pack(out_dir: Path, files: Mapping[str, bytes]) -> Path:
    """Write the files into `out_dir`, which must not exist; every file is created exclusively."""
    out = Path(out_dir)
    out.mkdir(parents=False, exist_ok=False)
    for name, data in sorted(files.items()):
        path = out / name
        path.parent.mkdir(parents=True, exist_ok=True)
        with open(path, "xb") as handle:
            handle.write(data)
    return out


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("out_dir", metavar="OUT_DIR", help="the folder to write; it must not exist")
    args = parser.parse_args(argv)
    try:
        files = build_pack()
        write_pack(Path(args.out_dir), files)
    except (BuildError, FileExistsError, FileNotFoundError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 2
    print(f"pack written: {len(files)} files, {sum(len(d) for d in files.values())} bytes; "
          f"identity (SHA-256 of {INDEX}): {pack_identity(files)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
