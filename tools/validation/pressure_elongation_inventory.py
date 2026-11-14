"""Inventory of committed pressure model inputs for the pressure-elongation switch (DEL-04-07).

Scans every Git-tracked file under the scanned roots (`core`, `validation`, `tests`, `tools`, `fixtures`, `apps`;
`execution/` is outside them) whose bytes name `pressure_regions`, and requires each to be claimed exactly once:

- a data file (`.json`, or `.jsonl` read line by line) holding a model document under a `pressure_contract` with a
  pressurised load case (`pressure_elongation.pressurised_case_ids`), by one of the curated GROUPS: its
  maintained producer (and whether the producer takes the switch), the readers and tests that pin its outputs, and
  the anchor route the harness (`pressure_elongation_anchor.py`) uses. Documents are also read where a file holds
  them as JSON text in a string, or in the retained-precision DAG storage;
- a code file (any other type) that builds a pressurised document which is solved, by an IN_CODE entry stating what
  its assertions rest on and whether its pressurised cases are named;
- any other scanned file (code that builds no solved pressurised document, or data with no pressurised document),
  by an EXEMPT entry with a one-line reason.

The groups, in-code records, exemptions, derived consumers and pin registrations are data, held in
`pressure_elongation_inventory.declarations.json` beside this module; the README says how a claim is added. The module
loads that file as data and fails, naming the file and the problem, if it is missing or malformed.

What the scan does not read. It finds files by the literal bytes `pressure_regions` (not an escaped key, not UTF-16)
and evaluates no source code: a code file's claim rests on reading it. Of data it reads plain JSON, JSON Lines, JSON
text held in strings (recursively) and the retained-precision DAG storage; any other assembly is not reconstructed.
One such store exists: the CAEPIPE MBF writer parity vectors (`fixtures/caepipe_mbf/invented/parity/
writer_vectors.json`) keep many of their models as blobs assembled from a base blob and a prefix, middle and
suffix (`tools/fixtures/caepipe_mbf_import_vectors.py`), so the 58 writer models recorded below are those held
as full strings; the independent review of this change reconstructed 107 pressurised writer models, 49 of them
assembled. Every one is an MBF writer input compared between the Python and Rust writers; none is solved, as
source-reviewed at a6392aafb and 986096a03. What the committed test detects is narrower: it fails if a tracked code
file (`.rs`, `.py`, `.ts`, `.tsx`, `.js`, `.mjs`) that names the parity vectors or their harvest modules, or the
Cargo manifest of a Rust one's crate, names a mechanics solve entry. It excludes two files that name both, this
producer and its test module. It matches names, not reachability.

The result, `pressure_elongation_inventory.json`, is also the re-production checklist for the fixture rebuild.

    python3 tools/validation/pressure_elongation_inventory.py           # write the inventory
    python3 tools/validation/pressure_elongation_inventory.py --check   # compare only

An unclaimed file, a file claimed twice, a claim that matches no file still naming `pressure_regions` (stale), an
exemption without a reason, or an exemption of a data file that holds a pressurised document is an error: a new
pressure input must be classified here before the inventory is current.

What the committed check establishes. `--check` and `test_pressure_elongation_inventory.py` establish general
structural properties and particular data facts; they establish nothing about source behaviour. The structural
properties:
1. every scanned file is claimed exactly once, by a group, an IN_CODE record or an exemption;
2. no claim is stale: each group pattern matches a data file holding a pressurised document, each IN_CODE path and
   each exemption or PINNED_BY pattern a tracked file still naming `pressure_regions`;
3. the declarations have their required shape: an exemption has a reason and takes no data file holding a pressurised
   document; an IN_CODE record names code files only and states its expectations, its cases_named and a detail; a
   group whose documents are held only (JSON Lines, JSON text or DAG storage) has an excluded anchor;
4. the generated copy, `pressure_elongation_inventory.json`, equals what this producer writes;
5. registrations agree with the readers: for each of the fourteen tables named in the test module's PIN_TABLES, the
   inventoried paths its reader extracts (resolved to tracked repository files) list that table's consumer in their
   `pinned_by` (see "The pin registrations" below);
6. a literal-name guard over the MBF parity vectors: no tracked code file of the six suffixes above that names them
   or their harvest modules, nor a Rust one's Cargo manifest, names a mechanics solve entry, this producer and its
   test module excepted (both name the vectors and a solve entry). It does not establish that nothing reaches a
   solve.

The data facts, asserted directly on the parsed fixtures: the K-D5 M3-T frozen subset holds twelve pressurised
documents, one pressurised case each and twelve nonzero regions in all, none naming the setting; and the five files
whose documents are held only (the desktop native-replay requests, the desktop analysis-run v0.3 cases, the
pressure-source reader corpus and the two MBF parity vectors) hold them at the recorded locations, with the
recorded pressurised cases, each named in the setting except in the parity vectors. These are bounded facts about
those bytes, not about the source that reads them.

What it does not establish. The source classifications (an IN_CODE record's cases_named and expectations, a group's
producer) and the exemptions are curated assertions supported by review at identified revisions: an independent
reading at a6392aafb, whose findings (a false exemption and nine false cases_named) were repaired in accdaf407; a
second independent reading at 986096a03, which derived all 61 IN_CODE records and all 130 exempted code files from
source and found them correct; and a scratch probe, run at 4854700d0, that logged each case reaching pressure assembly
with its largest region pressure and whether the setting names it. The logger runs before mechanics: it can record
a case that is refused later or an assembly repeated for evidence, misses a case refused before assembly and ignores
its own write errors, so its records are assembly observations, not successful solves. The crates and Python sources the probe observed are
identical at 4854700d0 and 986096a03, and its observations agreed with 56 IN_CODE files and 28 exempted files and
disagreed with none. The probe is not committed and protects nothing. Regenerating the copy does not renew that
evidence, and a green check does not maintain the assertions' truth. An assertion must be re-read when a change
affects it: the case construction, the fixture inputs, the helpers, the callers, the naming or the dispatch of the
file or of what it relies on, not only a change to the setting's semantics. Before the inventory is used for a
rebuild, the affected assertions are reconciled with the source actually being used.

These pass the committed check and are its limits: a flipped cases_named, after regeneration; a false exemption,
after regeneration; an existing claimed source edited so that its classification becomes false; a pressurised solve
added to an existing exempted file.

The pin registrations. The registrations in PINNED_BY are curated, as the classifications are. The check compares
registrations with paths extracted by readers for fourteen named tables. Their current targets have been reconciled
by source review. Changes to table construction, path resolution or use can evade these readers or cause
unsupported-form refusals; those changes require source reconciliation. A passing check does not establish
completeness or executing hash enforcement. Known forms the readers do not see or refuse, by kind: an entry added
through an alias of a registered table; a property set after the TypeScript initialiser; a Rust constant shadowed in
a local scope; a digest written as an include_str! in a table read by the shared Rust array reader (refused as an
unsupported form). Each reader's check that the
consumer still contains its hashing code is a spelling check, not evidence that the hashing runs. A digest of an
inventoried file held anywhere other than these fourteen tables is not compared at all. An omitted registration can
leave the rebuild checklist incomplete. The existing consumer checks reject input bytes outside their admitted pins
when those checks run. This inventory neither schedules those checks nor establishes that their hash enforcement
executes.
"""
from __future__ import annotations

import argparse
import fnmatch
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
if str(ROOT) not in sys.path:
    sys.path.append(str(ROOT))
from tools.fixtures import retained_precision_corpus as CORPUS  # noqa: E402
from tools.validation import pressure_elongation as PE  # noqa: E402

OUTPUT = ROOT / "tools/validation/pressure_elongation_inventory.json"
SCAN_ROOTS = ("core", "validation", "tests", "tools", "fixtures", "apps")
TOKEN = "pressure_regions"
DATA_SUFFIXES = (".json", ".jsonl")
# What an IN_CODE entry's assertions rest on, and whether its pressurised cases are named.
EXPECTATIONS = ("closed_form", "reference_file", "product_output", "no_response_value")
CASES_NAMED = ("all", "some", "none")
BASELINE = {"kind": "baseline"}
DECLARATIONS = Path(__file__).with_name("pressure_elongation_inventory.declarations.json")
DECLARATIONS_KIND = "swbpipe.pressure_elongation_inventory.declarations"
TABLES = ("groups", "derived", "pinned_by", "in_code", "exempt")
NOTE_KINDS = ("section", "record")


class InventoryError(Exception):
    """The claims do not cover the scanned files exactly once."""


class DeclarationsError(InventoryError):
    """The declarations file is missing or malformed."""


def _object_without_duplicates(pairs: list[tuple[str, object]]) -> dict:
    """A JSON object's members, refusing a key given twice instead of keeping only the last."""
    members: dict = {}
    for key, value in pairs:
        if key in members:
            raise DeclarationsError(f"the key {key!r} appears twice in one object")
        members[key] = value
    return members


def declaration_key(table: str, record: dict):
    """What a note names a declaration by: a group's id, a derived consumer's path, a pin registration's first
    consumer, an in-code record's first path, an exemption's first pattern. None when the record lacks it."""
    try:
        return {"groups": lambda: record["id"], "derived": lambda: record["path"],
                "pinned_by": lambda: record["pinned_by"][0], "in_code": lambda: record["paths"][0],
                "exempt": lambda: record["paths"][0]}[table]()
    except (KeyError, IndexError, TypeError):
        return None


def load_declarations(path: Path = DECLARATIONS) -> dict:
    """The declarations file, read as data. Every field is data: nothing in it is executed. A missing file, invalid
    JSON, a key given twice in one object, a top level other than the expected object of tables, or a note naming no
    single declaration raises DeclarationsError naming the file and the problem."""
    def refuse(problem: str):
        raise DeclarationsError(f"{path}: {problem}")
    try:
        text = path.read_text(encoding="utf-8")
    except FileNotFoundError:
        refuse("the declarations file is missing")
    except (OSError, UnicodeDecodeError) as error:
        refuse(f"cannot be read ({error})")
    try:
        value = json.loads(text, object_pairs_hook=_object_without_duplicates)
    except json.JSONDecodeError as error:
        refuse(f"not valid JSON ({error})")
    except DeclarationsError as error:
        refuse(str(error))
    members = ("document_kind", "version", *TABLES, "notes")
    if not isinstance(value, dict) or set(value) != set(members):
        found = sorted(value) if isinstance(value, dict) else type(value).__name__
        refuse(f"the top level must be an object with exactly the members {', '.join(members)}; found {found}")
    if value["document_kind"] != DECLARATIONS_KIND or value["version"] != 1:
        refuse(f"document_kind must be {DECLARATIONS_KIND!r} and version 1")
    for table in (*TABLES, "notes"):
        if not isinstance(value[table], list) or not all(isinstance(record, dict) for record in value[table]):
            refuse(f"{table} must be a list of objects")
    for index, note in enumerate(value["notes"]):
        if set(note) != {"table", "kind", "declarations", "comment"} or note["table"] not in TABLES \
                or note["kind"] not in NOTE_KINDS or not isinstance(note["declarations"], list) or not note["declarations"] \
                or not isinstance(note["comment"], list) or not all(isinstance(line, str) for line in note["comment"]):
            refuse(f"notes[{index}] must be an object of table, kind ({', '.join(NOTE_KINDS)}), declarations (a "
                   "non-empty list) and comment (a list of lines)")
        for key in note["declarations"]:
            matched = [record for record in value[note["table"]] if declaration_key(note["table"], record) == key]
            if len(matched) != 1:
                refuse(f"notes[{index}] names {key!r}, which is {len(matched)} declarations of {note['table']}, not one")
    return value


# The declarations: data a lane edits to claim a file (the README says how). Each comment that qualified one
# declaration, or headed a run of them, is kept verbatim in the file's `notes`, keyed to the declarations it governs.
_DECLARATIONS = load_declarations()

# Each group: id, path patterns (fnmatch on the repository path), producer, pinned_by, anchor.
GROUPS = _DECLARATIONS["groups"]

# Consumers that hold no model document of their own but re-derive from the entries above.
DERIVED = _DECLARATIONS["derived"]

# Pins on individual scanned files beyond their group's `pinned_by`: each named file's group entry lists them after
# the group's pins, and an exemption's record lists them per file. A pattern that matches no scanned file is stale.
# Each is a fixed table in its consumer that requires the file's SHA-256 to match an admitted pin when that check runs;
# provenance text and historical endpoint hashes are not pins.
# The registrations are curated: they cover the fourteen tables `test_pressure_elongation_inventory.py` names
# (PIN_TABLES), whose current targets have been reconciled by source review, and are compared with what its readers
# extract (the module docstring, "The pin registrations"). They are not a census of every digest in the repository.
PINNED_BY = _DECLARATIONS["pinned_by"]

# Code files that build a pressurised document which is solved: the expectations their assertions rest on
# (EXPECTATIONS: an independent closed form, a reference file, the product's own output, or no response value at all)
# and whether their pressurised cases are named in model_settings.pressure_elongation (CASES_NAMED). Where cases are
# not named and an assertion rests on an independent value, the detail says why that value does not depend on the free
# pressure strain, or names the open matter.
IN_CODE = _DECLARATIONS["in_code"]

# Every other scanned file, with the reason it needs no group or IN_CODE entry. Patterns are fnmatch patterns over the
# repository path (`*` crosses `/`); an exemption may not take a data file that holds a pressurised document.
EXEMPT = _DECLARATIONS["exempt"]


def scanned_files(root: Path = ROOT) -> list[str]:
    """Every tracked file under SCAN_ROOTS whose bytes name TOKEN, sorted."""
    done = subprocess.run(["git", "grep", "-l", "-F", TOKEN, "--", *SCAN_ROOTS], cwd=root, capture_output=True, text=True)
    if done.returncode not in (0, 1):
        raise InventoryError(f"git grep failed: {done.stderr.strip()}")
    return sorted(path for path in done.stdout.splitlines() if path)


def is_data(path: str) -> bool:
    return path.endswith(DATA_SUFFIXES)


def _texts(value, pointer: str = ""):
    """(pointer, parsed) for each string value naming TOKEN that is itself a JSON object or array."""
    if isinstance(value, dict):
        for key, item in value.items():
            yield from _texts(item, f"{pointer}/{PE._escape(key)}")
    elif isinstance(value, list):
        for index, item in enumerate(value):
            yield from _texts(item, f"{pointer}/{index}")
    elif isinstance(value, str) and TOKEN in value:
        try:
            parsed = json.loads(value)
        except ValueError:
            return
        if isinstance(parsed, (dict, list)):
            yield pointer, parsed


def container_documents(container, texts: tuple = ()) -> tuple[list[dict], list[dict]]:
    """(plain, held): {pointer, schema_version, pressurised_load_cases} for each pressurised document in `container`, in
    document order, and for each held as JSON text in one of its strings, where `pointer` names the outermost string
    and `text_pointers` the path inside it (each inner string, then the document)."""
    plain, held = [], []
    for pointer, doc in PE.documents(container):
        cases = PE.pressurised_case_ids(doc)
        if cases:
            where = {"pointer": texts[0], "text_pointers": [*texts[1:], pointer]} if texts else {"pointer": pointer}
            (held if texts else plain).append({**where, "schema_version": doc.get("schema_version"),
                                               "pressurised_load_cases": cases})
    for pointer, parsed in _texts(container):
        inner, deeper = container_documents(parsed, (*texts, pointer))
        held.extend(inner + deeper)
    return plain, held


def data_documents(path: str, text: str) -> tuple[list[dict], list[dict]]:
    """(plain, held) pressurised documents of one data file. Plain documents are those the anchor harness reads by
    JSON pointer. Held documents are in JSON text, in a `.jsonl` line (each record carries its `line`) or in a
    retained-precision DAG definition, read expanded (each record carries its `storage`)."""
    if path.endswith(".jsonl"):
        held = []
        for number, line in enumerate(text.splitlines(), 1):
            if line.strip():
                plain, inner = container_documents(json.loads(line))
                held.extend({"line": number, **record} for record in plain + inner)
        return [], held
    container = json.loads(text)
    if isinstance(container, dict) and container.get("format") == CORPUS.FORMAT:
        plain, inner = container_documents(CORPUS.expand(container))
        return [], [{**record, "storage": CORPUS.FORMAT} for record in plain + inner]
    return container_documents(container)


def scan(root: Path = ROOT) -> tuple[list[str], dict[str, tuple[list[dict], list[dict]]]]:
    """(every scanned file, {data file: (plain, held) pressurised documents}); a data file may hold none."""
    files = scanned_files(root)
    return files, {path: data_documents(path, (root / path).read_text(encoding="utf-8")) for path in files if is_data(path)}


def _raw_files(anchor: dict, path: str, container) -> tuple[dict | None, dict | None]:
    """(files, per_document) for a raw_files anchor templated by `files_from`."""
    pattern = anchor.get("files_from")
    if pattern is None:
        return anchor.get("files"), None
    if pattern == "raw_path":
        return {container.get("solver_mode", "sparse_interactive"): container["raw_path"]}, None
    folder, name = path.rsplit("/", 1)
    stem = name[: -len(".request.json")]
    files = {}
    for mode in ("sparse_interactive", "dense_scrutiny"):
        candidate = f"{folder}/" + pattern.format(stem=stem, mode=mode)
        if (ROOT / candidate).exists():
            files[mode] = candidate
    return files or None, None


def _suffix(path: str, paths: list[str]) -> str:
    """The path below the group's common directory, without `.json`: a unique, readable entry id."""
    common = Path(*Path(paths[0]).parts[:-1])
    for other in paths[1:]:
        while not str(Path(other)).startswith(str(common) + "/"):
            common = common.parent
    return str(Path(path).relative_to(common)).removesuffix(".json")


def _matches(path: str, patterns: list[str]) -> bool:
    return any(fnmatch.fnmatchcase(path, pattern) for pattern in patterns)


def claims(files: list[str], data: dict[str, tuple[list[dict], list[dict]]], groups: list[dict], in_code: list[dict],
           exempt: list[dict]) -> tuple[dict[str, list[str]], list[str]]:
    """({file: the claims that take it}, problems). A group takes the data files with a pressurised document that
    its patterns match; an IN_CODE entry takes the code files it names exactly; an exemption takes the files its
    patterns match. A claim, or one of its patterns, that takes nothing is stale."""
    pressurised = {path for path, (plain, held) in data.items() if plain or held}
    scanned = set(files)
    taken: dict[str, list[str]] = {path: [] for path in files}
    problems = []
    for group in groups:
        for pattern in group["paths"]:
            if not any(fnmatch.fnmatchcase(path, pattern) for path in pressurised):
                problems.append(f"stale: group {group['id']} path {pattern} matches no data file with a pressurised document")
        for path in sorted(p for p in pressurised if _matches(p, group["paths"])):
            taken[path].append(f"group {group['id']}")
    for entry in in_code:
        label = f"IN_CODE {', '.join(entry.get('paths') or ['(no paths)'])}"
        if not entry.get("paths"):
            problems.append(f"{label}: names no file")
        if entry.get("expectations") not in EXPECTATIONS:
            problems.append(f"{label}: expectations must be one of {', '.join(EXPECTATIONS)}")
        if entry.get("cases_named") not in CASES_NAMED:
            problems.append(f"{label}: cases_named must be one of {', '.join(CASES_NAMED)}")
        if not str(entry.get("detail") or "").strip():
            problems.append(f"{label}: no detail of what its assertions rest on")
        for path in entry.get("paths") or []:
            if path not in scanned:
                problems.append(f"stale: IN_CODE path {path} is not a tracked file naming {TOKEN}")
            elif is_data(path):
                problems.append(f"IN_CODE path {path} is a data file; claim it with a group or an exemption")
            else:
                taken[path].append(f"IN_CODE {path}")
    for index, entry in enumerate(exempt):
        patterns = entry.get("paths") or []
        label = f"exemption #{index} ({patterns[0] if patterns else 'no paths'}{f' and {len(patterns) - 1} more' if len(patterns) > 1 else ''})"
        if not isinstance(entry.get("reason"), str) or not entry["reason"].strip():
            problems.append(f"{label}: no reason")
        for pattern in patterns:
            if not any(fnmatch.fnmatchcase(path, pattern) for path in files):
                problems.append(f"stale: exemption path {pattern} matches no tracked file naming {TOKEN}")
        for path in files:
            if _matches(path, patterns):
                if path in pressurised:
                    problems.append(f"{label}: {path} holds a pressurised document; claim it with a group")
                else:
                    taken[path].append(label)
    for path in files:
        if not taken[path]:
            problems.append(f"unclaimed: {path}")
        elif len(taken[path]) > 1:
            problems.append(f"claimed twice: {path} ({'; '.join(taken[path])})")
    return taken, problems


def _pins(path: str, pinned: list[dict]) -> list[str]:
    return [pin for entry in pinned if _matches(path, entry["paths"]) for pin in entry["pinned_by"]]


def build(groups: list[dict] | None = None, in_code: list[dict] | None = None, exempt: list[dict] | None = None,
          root: Path = ROOT, scanned: tuple | None = None, pinned: list[dict] | None = None) -> dict:
    """The inventory from the claims (default: this module's GROUPS, IN_CODE and EXEMPT) over `scanned` (default: a
    fresh `scan(root)`); InventoryError lists every problem."""
    groups = GROUPS if groups is None else groups
    in_code = IN_CODE if in_code is None else in_code
    exempt = EXEMPT if exempt is None else exempt
    pinned = PINNED_BY if pinned is None else pinned
    files, data = scan(root) if scanned is None else scanned
    taken, problems = claims(files, data, groups, in_code, exempt)
    for entry in pinned:
        for pattern in entry["paths"]:
            if not any(fnmatch.fnmatchcase(path, pattern) for path in files):
                problems.append(f"stale: PINNED_BY path {pattern} matches no tracked file naming {TOKEN}")
    entries = []
    for group in groups:
        paths = sorted(p for p, (plain, held) in data.items() if (plain or held) and _matches(p, group["paths"]))
        for path in paths:
            documents, held = data[path]
            if group["anchor"]["kind"] != "excluded" and not documents:
                problems.append(f"group {group['id']}: {path} holds no document the harness can read (only JSON Lines, "
                                "JSON text or DAG storage); its anchor must be excluded")
            anchor = {k: v for k, v in group["anchor"].items() if k != "files_from"}
            if group["anchor"]["kind"] == "raw_files":
                container = json.loads((root / path).read_text(encoding="utf-8"))
                files_for, _ = _raw_files(group["anchor"], path, container)
                if files_for is None:
                    anchor = dict(BASELINE)
                else:
                    anchor["files"] = files_for
            entry = {"id": group["id"] if len(paths) == 1 else f"{group['id']}/{_suffix(path, paths)}",
                     "group": group["id"], "path": path, "producer": group["producer"],
                     "pinned_by": group["pinned_by"] + _pins(path, pinned), "anchor": anchor, "documents": documents}
            if held:
                entry["held_documents"] = held
            entries.append(entry)
    if problems:
        raise InventoryError("inventory out of date:\n  " + "\n  ".join(problems))
    counts: dict[str, dict] = {}
    for entry in entries:
        c = counts.setdefault(entry["group"], {"files": 0, "documents": 0, "pressurised_load_cases": 0})
        c["files"] += 1
        c["documents"] += len(entry["documents"])
        c["pressurised_load_cases"] += sum(len(d["pressurised_load_cases"]) for d in entry["documents"])
        if entry.get("held_documents"):
            c["held_documents"] = c.get("held_documents", 0) + len(entry["held_documents"])
            c["held_pressurised_load_cases"] = (c.get("held_pressurised_load_cases", 0)
                                                + sum(len(d["pressurised_load_cases"]) for d in entry["held_documents"]))
    kinds = {"group": 0, "in_code": 0, "exemption": 0}
    for path in files:
        kind = taken[path][0].split(" ", 1)[0]
        kinds[{"group": "group", "IN_CODE": "in_code", "exemption": "exemption"}[kind]] += 1
    return {
        "document_kind": "swbpipe.pressure_elongation_inventory",
        "version": 2,
        "purpose": "Every tracked file under the scanned roots that names pressure_regions, each claimed once: a data file "
                   "with a pressurised document by its group (producer, pinning readers, anchor route), a code file "
                   "that builds a solved pressurised document by an in-code entry (what its assertions rest on, whether "
                   "its cases are named), any other by a reasoned exemption. The DEL-04-07 regression anchor's scope "
                   "and the fixture re-production checklist. Generated by tools/validation/pressure_elongation_inventory.py.",
        "setting": "model_settings.pressure_elongation.operating_load_cases",
        "switch": {"helper": "tools/validation/pressure_elongation.py", "values": list(PE.MODES), "default": PE.DEFAULT,
                   "environment": PE.ENV},
        "pressurised_case_rule": "a pressure region with a nonzero pressure, or condition_use.pressure resolving to a "
                                 "nonzero condition pressure on a pipe of one of the case's regions",
        "scan": {"roots": list(SCAN_ROOTS), "token": TOKEN, "data_suffixes": list(DATA_SUFFIXES), "files": len(files),
                 "claimed_by": kinds},
        "counts": counts,
        "entries": entries,
        "derived_consumers": DERIVED,
        "in_code_constructions": in_code,
        "exemptions": [_exemption_record(entry, files, pinned) for entry in exempt],
    }


def _exemption_record(entry: dict, files: list[str], pinned: list[dict]) -> dict:
    matched = [p for p in files if _matches(p, entry["paths"])]
    record = {"reason": entry["reason"], "paths": entry["paths"], "files": matched}
    pins = {p: _pins(p, pinned) for p in matched if _pins(p, pinned)}
    if pins:
        record["pinned_by"] = pins
    return record


def dumps(value) -> str:
    return json.dumps(value, indent=1, ensure_ascii=False) + "\n"


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="compare with the committed inventory; write nothing")
    args = parser.parse_args(argv)
    try:
        text = dumps(build())
    except InventoryError as error:
        print(error, file=sys.stderr)
        return 1
    if args.check:
        current = OUTPUT.read_text(encoding="utf-8") if OUTPUT.exists() else ""
        if current != text:
            print(f"{OUTPUT.relative_to(ROOT)} is not current; run without --check")
            return 1
        print(f"{OUTPUT.relative_to(ROOT)} is current")
        return 0
    OUTPUT.write_text(text, encoding="utf-8")
    print(f"wrote {OUTPUT.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
