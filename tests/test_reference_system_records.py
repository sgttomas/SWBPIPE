"""The reference system's written records against the fixture (DEL-09-01 VER-005, AC-002, AC-010).

`fixtures/reference_system/SPECIFICATION.md` states the system and its pending ledger;
`fixtures/reference_system/RUN_SHEET.md` enters the fixture from a blank project and lists every
difference between the model it yields and the fixture. These checks fail when:

- the specification's ledger and the generator's ledger (and `authoring.json`'s pending flags)
  disagree on a family, its flag or its identities, or a flag disagrees with what the fixture holds;
- an identity the fixture holds is missing from either record, or a record names an identity that
  neither the fixture nor the ledger holds;
- a provenance or source text the run sheet tells the person to type is not the fixture's and is
  not listed among its differences;
- either record links to a path that does not exist, or to a path under `execution/`, which the
  product test job does not check out (cite a deliverable by its id instead).

`tests/test_reference_system_fixture.py` checks the fixture itself: the generator's output, the
schema, the solve and the references.
"""
from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import re

import pytest

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "fixtures" / "reference_system"
SPEC_PATH = FIXTURE / "SPECIFICATION.md"
RUN_SHEET_PATH = FIXTURE / "RUN_SHEET.md"
SPEC = SPEC_PATH.read_text(encoding="utf-8")
RUN_SHEET = RUN_SHEET_PATH.read_text(encoding="utf-8")
MODEL = json.loads((FIXTURE / "reference_system.json").read_text(encoding="utf-8"))
AUTHORING = json.loads((FIXTURE / "authoring.json").read_text(encoding="utf-8"))
RECORDS = {"SPECIFICATION.md": (SPEC_PATH, SPEC), "RUN_SHEET.md": (RUN_SHEET_PATH, RUN_SHEET)}


def _generator():
    spec = importlib.util.spec_from_file_location("generate_reference_system",
                                                  FIXTURE / "generate_reference_system.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


GENERATOR = _generator()
COLLECTIONS = ("materials", "sections", "nodes", "pipe_segments", "components", "supports", "load_cases",
               "combinations", "reference_configurations")
FULL_ID = re.compile(r"\b(?:node|pipe|component|support|load|combination|material|section|point|law|rod|reference"
                     r"|region|spectrum):RS-[A-Za-z0-9](?:[A-Za-z0-9-]*[A-Za-z0-9])?")


# -- reading the records --------------------------------------------------------------------------------------------
def level2_section(text: str, word: str) -> str:
    """The body of the one level-2 section whose heading contains `word` (case-insensitive)."""
    parts = re.split(r"(?m)^## ", text)
    found = [part for part in parts[1:] if word.lower() in part.splitlines()[0].lower()]
    assert len(found) == 1, f"expected one level-2 section about {word!r}, found {len(found)}"
    return found[0]


def table(section: str, first_header: str) -> list[dict[str, str]]:
    """The Markdown table in `section` whose first header cell is `first_header`, as rows keyed by header."""
    lines = section.splitlines()
    for index, line in enumerate(lines):
        cells = [cell.strip() for cell in line.strip().strip("|").split(" | ")]
        if line.startswith("|") and cells[0] == first_header:
            header = cells
            rows = []
            for row in lines[index + 2:]:
                if not row.startswith("|"):
                    break
                values = [cell.strip() for cell in row.strip().strip("|").split(" | ")]
                assert len(values) == len(header), f"row without {len(header)} cells: {row}"
                rows.append(dict(zip(header, values)))
            return rows
    raise AssertionError(f"no table headed {first_header!r}")


def spec_ledger() -> dict[str, dict[str, object]]:
    rows = table(level2_section(SPEC, "ledger"), "Family or analysis")
    ledger = {}
    for row in rows:
        family = row["Family or analysis"]
        assert family not in ledger, f"ledger family listed twice: {family}"
        ledger[family] = {"in_tranche_1": row["State"].startswith("in tranche 1"), "state": row["State"],
                          "entities": sorted(re.findall(r"`([a-z]+:[^`]+)`", row["Identities"]))}
    return ledger


def tag(identity: str) -> str:
    return identity.split(":", 1)[1]


def names(text: str, token: str) -> bool:
    return re.search(rf"(?<![\w-]){re.escape(token)}(?![\w-])", text) is not None


# -- what the fixture holds -----------------------------------------------------------------------------------------
def fixture_identities() -> set[str]:
    ids = {record["id"] for collection in COLLECTIONS for record in MODEL[collection]}
    for material in MODEL["materials"]:
        ids |= {point["id"] for point in material.get("temperature_points", [])}
        ids |= {law["id"] for law in material.get("expansion_laws", [])}
    for component in MODEL["components"]:
        hardware = component.get("objective_connector", {}).get("hardware", {})
        ids |= {rod["id"] for rod in hardware.get("rods", [])}
    for case in MODEL["load_cases"]:
        ids |= {region["id"] for region in case.get("pressure_regions", [])}
    return ids


FIXTURE_IDS = fixture_identities()
LEDGER_IDS = {entity for row in AUTHORING["pending"] for entity in row["entities"]}
DRAFTING_IDS = {bend["corner"]["node_id_while_drafting"] for bend in AUTHORING["bends"]}
NUMBERING_PREFIXES = {"pipe:RS-P"}  # the run sheet's Pipe prefix, not an identity


def provenance_texts(value) -> set[str]:
    """Every `provenance` and `source_reference` string anywhere in the fixture."""
    found = set()
    if isinstance(value, dict):
        for key, item in value.items():
            if key in ("provenance", "source_reference") and isinstance(item, str):
                found.add(item)
            found |= provenance_texts(item)
    elif isinstance(value, list):
        for item in value:
            found |= provenance_texts(item)
    return found


# -- (1) the ledger -------------------------------------------------------------------------------------------------
def test_the_specification_ledger_agrees_with_the_generator_and_the_authoring_flags():
    ledger = spec_ledger()
    generator = {family: {"in_tranche_1": state.startswith("in tranche 1"),
                          "entities": sorted(GENERATOR.LEDGER_ENTITIES.get(family, []))}
                 for family, state, _lane, _expected in GENERATOR.LEDGER}
    authored = {row["family"]: {"in_tranche_1": row["in_tranche_1"], "entities": sorted(row["entities"])}
                for row in AUTHORING["pending"]}
    stated = {family: {"in_tranche_1": row["in_tranche_1"], "entities": row["entities"]}
              for family, row in ledger.items()}
    assert stated == generator, "the specification's §10 and the generator's LEDGER differ"
    assert stated == authored, "the specification's §10 and authoring.json's pending rows differ"


def test_each_ledger_flag_agrees_with_what_the_fixture_holds():
    for family, row in spec_ledger().items():
        for entity in row["entities"]:
            if row["in_tranche_1"]:
                assert entity in FIXTURE_IDS, f"{family}: in tranche 1, but the fixture has no {entity}"
            elif entity in FIXTURE_IDS:
                # A pending row may keep an interim record in the fixture only where its state says so.
                assert f"{tag(entity)} is in tranche 1" in row["state"], \
                    f"{family}: pending, but the fixture holds {entity} and the state does not say why"


# -- (2) identities -------------------------------------------------------------------------------------------------
@pytest.mark.parametrize("record", sorted(RECORDS))
def test_every_identity_the_fixture_holds_is_named(record):
    _path, text = RECORDS[record]
    missing = []
    for identity in sorted(FIXTURE_IDS):
        if identity.startswith("region:") and record == "SPECIFICATION.md":
            # The specification states the regions once for every case: `region:<case>-<region>`.
            case, region = re.fullmatch(r"region:(RS-[A-Z]+)-(RG-\w+)", identity).groups()
            if not (names(text, case) and names(text, region)):
                missing.append(identity)
        elif not names(text, tag(identity)):
            missing.append(identity)
    assert not missing, f"{record} does not name {missing}"


@pytest.mark.parametrize("record", sorted(RECORDS))
def test_every_identity_a_record_names_is_held_by_the_fixture_or_the_ledger(record):
    _path, text = RECORDS[record]
    unknown = sorted(set(FULL_ID.findall(text)) - FIXTURE_IDS - LEDGER_IDS - DRAFTING_IDS - NUMBERING_PREFIXES)
    assert not unknown, f"{record} names identities that neither the fixture nor the ledger holds: {unknown}"


# -- (3) the run sheet's texts and its differences ------------------------------------------------------------------
def pasted_texts() -> dict[str, str]:
    texts = dict(re.findall(r"\*\*⟨(\w+)⟩\*\* = `([^`]+)`", RUN_SHEET))
    assert {"P", "S5"} <= set(texts), "the run sheet defines ⟨P⟩ and ⟨S5⟩"
    return texts


def test_the_provenance_and_source_texts_are_the_fixture_s_or_listed_as_differences():
    """Every provenance or source the run sheet has the person type, ⟨P⟩ and ⟨S5⟩ and each text
    composed from ⟨P⟩, is one the fixture holds, or the run sheet lists it as a difference."""
    held = provenance_texts(MODEL)
    differences = level2_section(RUN_SHEET, "differences")
    texts = pasted_texts()
    typed = {texts["P"], texts["S5"]}
    typed |= {texts["P"] + suffix for suffix in re.findall(r"⟨P⟩`([^`]+)`", RUN_SHEET)}
    unlisted = sorted(text for text in typed if text not in held and text not in differences)
    assert not unlisted, f"the run sheet has the person type texts the fixture does not hold, unlisted: {unlisted}"


def test_the_pasted_texts_are_the_fixture_s_record_provenance_and_user_source():
    """⟨P⟩ is every record's provenance and ⟨S5⟩ every user value's source; a run sheet whose texts
    are not these enters different records."""
    texts = pasted_texts()
    record_provenance = {record["provenance"] for collection in COLLECTIONS if collection != "load_cases"
                         for record in MODEL[collection]}
    assert record_provenance == {texts["P"]} or texts["P"] in level2_section(RUN_SHEET, "differences")
    bend_sources = {component["modifiers"]["source_reference"] for component in MODEL["components"]
                    if component["kind"] in ("bend", "tee")}
    assert bend_sources == {texts["S5"]} or texts["S5"] in level2_section(RUN_SHEET, "differences")


# -- (4) links ------------------------------------------------------------------------------------------------------
DECLARATIVE = "execution"  # the declarative records' directory: a citation of it is refused, nothing under it is read
TOP_LEVEL = ("apps", "core", "docs", DECLARATIVE, "fixtures", "schemas", "tests", "tools", "validation")
REPO_PATH = re.compile(rf"`((?:{'|'.join(TOP_LEVEL)})/[^`\s]*)`")
LINK = re.compile(r"\]\(([^)\s]+)\)")


def referenced_paths(path: Path, text: str) -> list[tuple[str, Path]]:
    found = []
    for target in LINK.findall(text):
        if re.match(r"[a-z]+:", target) or target.startswith("#"):
            continue
        found.append((target, (path.parent / target.split("#", 1)[0]).resolve()))
    for target in REPO_PATH.findall(text):
        found.append((target, (ROOT / target).resolve()))
    return found


@pytest.mark.parametrize("record", sorted(RECORDS))
def test_every_path_a_record_cites_exists(record):
    path, text = RECORDS[record]
    paths = referenced_paths(path, text)
    assert paths, f"{record} cites no path"
    under_execution = [target for target, resolved in paths
                       if resolved.is_relative_to(ROOT) and resolved.relative_to(ROOT).parts[:1] == (DECLARATIVE,)]
    assert not under_execution, f"{record} cites execution/ paths, which the product test job cannot check: " \
                                f"{under_execution}; cite the deliverable by its id"
    missing = [target for target, resolved in paths if not resolved.exists()]
    assert not missing, f"{record} cites paths that do not exist: {missing}"
    outside = [target for target, resolved in paths if not resolved.is_relative_to(ROOT)]
    assert not outside, f"{record} cites paths outside the repository: {outside}"


# -- the check catches what it exists to catch ----------------------------------------------------------------------
def test_the_checks_detect_a_changed_flag_a_missing_identity_a_stale_text_and_a_dead_link(monkeypatch):
    global SPEC, RUN_SHEET
    original_spec, original_run_sheet = SPEC, RUN_SHEET
    try:
        SPEC = original_spec.replace("| Constant-effort support RS-CE1 | pending |",
                                     "| Constant-effort support RS-CE1 | in tranche 1 |")
        with pytest.raises(AssertionError):
            test_the_specification_ledger_agrees_with_the_generator_and_the_authoring_flags()
        SPEC = original_spec
        RUN_SHEET = original_run_sheet.replace("`support:RS-G3`", "`support:RS-G9`")
        monkeypatch.setitem(RECORDS, "RUN_SHEET.md", (RUN_SHEET_PATH, RUN_SHEET))
        with pytest.raises(AssertionError):
            test_every_identity_a_record_names_is_held_by_the_fixture_or_the_ledger("RUN_SHEET.md")
        RUN_SHEET = original_run_sheet.replace(
            "invented reference-system fixture input (DEL-09-01); not standard",
            "invented reference-system fixture input (DEL-09-01, a stale text); not standard", 1)
        with pytest.raises(AssertionError):
            test_the_provenance_and_source_texts_are_the_fixture_s_or_listed_as_differences()
        monkeypatch.setitem(RECORDS, "RUN_SHEET.md",
                            (RUN_SHEET_PATH, original_run_sheet + "\n[gone](reference-system-run-sheet.md)\n"))
        with pytest.raises(AssertionError):
            test_every_path_a_record_cites_exists("RUN_SHEET.md")
    finally:
        SPEC, RUN_SHEET = original_spec, original_run_sheet
