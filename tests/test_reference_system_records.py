"""The reference system's written records against the fixture (DEL-09-01 VER-005, AC-002, AC-010).

`fixtures/reference_system/SPECIFICATION.md` states the system and its pending ledger;
`fixtures/reference_system/RUN_SHEET.md` enters the fixture from a blank project and lists every
difference between the model it yields and the fixture. These checks fail when:

- the specification's ledger and the generator's ledger (and `authoring.json`'s pending flags)
  disagree on a family, its flag or its identities; or a row's flag, or a sub-feature it calls
  pending, disagrees with what the fixture holds, judged by a predicate per family;
- an identity the fixture holds is missing from either record, a record names an identity (in
  full or as a bare tag such as a member shorthand) that neither the fixture nor the ledger holds,
  or a self-weight primitive's identity does not follow the rule the specification states;
- a provenance, source or reference text the run sheet has the person enter is not the value of
  the field it is written to, and the run sheet does not list that field's value as a difference;
- either record cites a path or a file name that does not exist, or a path under `execution/`,
  which the product test job does not check out (cite a deliverable by its id instead).

Each check is a function of the records and the fixture, so the controls at the end alter the
coverage, the records or the fixture and show the check fails. `tests/test_reference_system_fixture.py`
checks the fixture itself: the generator's output, the schema, the solve and the references.
"""
from __future__ import annotations

import copy
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
# A bare tag (`RS-P150`, `RS-130`, `RS-B1`, `RS-SUS-RG-1`): what the records' tables and prose use.
TAG = re.compile(r"(?<![\w:-])RS-[A-Za-z0-9]+(?:-[A-Za-z0-9]+)*(?![\w-])")
NUMBERING_PREFIXES = {"pipe:RS-P"}  # the run sheet's Pipe prefix, not an identity


# -- reading the records --------------------------------------------------------------------------------------------
def level2_section(text: str, word: str) -> str:
    """The body of the one level-2 section whose heading contains `word` (case-insensitive)."""
    parts = re.split(r"(?m)^## ", text)
    found = [part for part in parts[1:] if word.lower() in part.splitlines()[0].lower()]
    assert len(found) == 1, f"expected one level-2 section about {word!r}, found {len(found)}"
    return found[0]


def step(text: str, name: str) -> str:
    """The body of the run sheet's `### Step <name>`."""
    found = re.findall(rf"(?ms)^### Step {re.escape(name)}\n(.*?)(?=^#{{2,3}} |\Z)", text)
    assert len(found) == 1, f"expected one step {name!r}, found {len(found)}"
    return found[0]


def cells_of(line: str) -> list[str]:
    """A Markdown table row's cells, split on pipes that are not escaped."""
    return [cell.strip() for cell in re.split(r"(?<!\\)\|", line.strip()[1:-1])]


def table(section: str, first_header: str, having: str | None = None) -> list[dict[str, str]]:
    """The Markdown table in `section` whose first header cell is `first_header` (and which has a column
    `having`), as rows keyed by header."""
    lines = section.splitlines()
    for index, line in enumerate(lines):
        cells = cells_of(line) if line.startswith("|") else []
        if cells and cells[0] == first_header and (having is None or having in cells):
            rows = []
            for row in lines[index + 2:]:
                if not row.startswith("|"):
                    break
                values = cells_of(row)
                assert len(values) == len(cells), f"row without {len(cells)} cells: {row}"
                rows.append(dict(zip(cells, values)))
            return rows
    raise AssertionError(f"no table headed {first_header!r}")


def spec_ledger(spec: str) -> dict[str, dict[str, object]]:
    ledger = {}
    for row in table(level2_section(spec, "ledger"), "Family or analysis"):
        family = row["Family or analysis"]
        assert family not in ledger, f"ledger family listed twice: {family}"
        ledger[family] = {"in_tranche_1": row["State"].startswith("in tranche 1"), "state": row["State"],
                          "entities": sorted(re.findall(r"`([a-z]+:[^`]+)`", row["Identities"]))}
    return ledger


def tag(identity: str) -> str:
    return identity.split(":", 1)[1]


def names(text: str, token: str) -> bool:
    return re.search(rf"(?<![\w-]){re.escape(token)}(?![\w-])", text) is not None


def by_id(model: dict, collection: str) -> dict[str, dict]:
    return {record["id"]: record for record in model.get(collection, [])}


def walk_keys(value) -> set[str]:
    if isinstance(value, dict):
        found = set(value)
        for item in value.values():
            found |= walk_keys(item)
        return found
    if isinstance(value, list):
        found = set()
        for item in value:
            found |= walk_keys(item)
        return found
    return set()


# -- (1) the ledger against the fixture -----------------------------------------------------------------------------
def _supports(model, family=None):
    return [s for s in model["supports"] if family is None or s.get("family") == family]


def _components(model, kind):
    return [c for c in model["components"] if c.get("kind") == kind]


def _case(model, case_id):
    return by_id(model, "load_cases").get(case_id)


def _cases(model):
    return model["load_cases"]


def _combination(model, combination_id):
    return by_id(model, "combinations").get(combination_id)


def _ope_thermal(model):
    states = (_case(model, "load:RS-OPE") or {}).get("analysis_state", {}).get("element_states", [])
    return bool(_case(model, "load:RS-SUS")) and any(
        s.get("material_selection", {}).get("kind") == "temperature_interpolation"
        and s.get("thermal_state", {}).get("kind") == "free_length_state" for s in states)


def _regions_closed(model):
    cases = [c for c in _cases(model) if c.get("pressure_regions") is not None]
    return bool(cases) and all(len(c["pressure_regions"]) == 5 and all(
        t["closure_transfer"] == "transfers_to_wall" for r in c["pressure_regions"] for t in r["terminals"])
        for c in cases)


def _joint_in_region(model):
    joints = [c["id"] for c in _components(model, "expansion_joint")
              if c.get("objective_connector", {}).get("hardware", {}).get("rods")
              and c["objective_connector"].get("pressure_model")]
    return any(joint in r.get("member_connector_ids", []) for joint in joints
               for case in _cases(model) for r in case.get("pressure_regions", []))


def _arcs(model):
    return {c["geometry"]["bend_pipe_ref"] for c in _components(model, "bend")}


# Whether the fixture holds each ledger family (its subject, for a row whose subject is pending).
FAMILY_PRESENT = {
    "Straight members, sections, temperature-dependent material": lambda m: bool(m["sections"]) and any(
        len(x.get("temperature_points", [])) >= 2 for x in m["materials"]) and any(
        p["id"] not in _arcs(m) for p in m["pipe_segments"]),
    "Realized bends with user k and SIFs": lambda m: any(
        c.get("mechanics_interface", {}).get("solver_consumption") == "curved_bend_macro_element"
        and {"flexibility_factor_user_value", "sif_in_plane_user_value", "sif_out_of_plane_user_value"}
        <= set(c.get("modifiers", {})) for c in _components(m, "bend")),
    "Valve and flanges, weight at CoG, stiffness scale": lambda m: all(any(
        "weight" in c["geometry"] and "center_of_gravity" in c["geometry"]
        and "stiffness_scaling_user_value" in c.get("modifiers", {}) for c in _components(m, kind))
        for kind in ("valve", "flange")),
    "Expansion joint with tie rods, pressurized": _joint_in_region,
    "Pressure (five regions, shared terminals `transfers_to_wall`)": _regions_closed,
    "Anchors, guides, line stop, vertical supports": lambda m: {"anchor", "guide", "line_stop", "vertical_support"}
    <= {s.get("family") for s in m["supports"]},
    "Variable spring as a user-stiffness spring": lambda m: any(
        s.get("hanger", {}).get("stiffness") for s in _supports(m, "variable_spring_hanger")),
    "Tee RS-T1 with user SIFs": lambda m: any(
        {"branch_header_sif_user_value", "branch_branch_sif_user_value"} <= set(c.get("modifiers", {}))
        for c in _components(m, "tee")),
    "Eccentric reducer RS-R1 in region RG-R": lambda m: bool(_components(m, "reducer")),
    "Constant-effort support RS-CE1": lambda m: bool(_supports(m, "constant_effort_support")),
    "Lift-off, friction and gap (RS-LO1, RS-FR1, RS-G1)": lambda m: bool(_supports(m, "nonlinear")),
    "RS-FR1 with RS-FR1V: lift-off friction shoe pending RT-A3 S7": lambda m: any(
        s["id"] == "support:RS-FR1V" and s.get("family") != "vertical_support" for s in m["supports"]),
    "RS-SUS, RS-OPE (thermal, modulus by temperature)": _ope_thermal,
    "RS-EXP": lambda m: bool(_combination(m, "combination:RS-EXP")),
    "RS-WIN, RS-EQX, RS-EQY, RS-EQZ on the document that carries temperature": lambda m: any(
        "equivalent_static" in c or c["id"] in ("load:RS-WIN", "load:RS-EQX", "load:RS-EQY", "load:RS-EQZ")
        for c in _cases(m)),
    "RS-SAM (`boundary_motion` on rigid anchor RS-C)": lambda m: any(
        s["support_ref"] == "support:RS-C" and s.get("boundary_motion")
        for s in (_case(m, "load:RS-SAM") or {}).get("analysis_state", {}).get("support_states", [])),
    "RS-EQ-SRSS, RS-EQ-ABS, RS-OCC-SRSS, RS-OCC-ABS": lambda m: any(
        c.get("basis") in ("srss", "absolute_sum") for c in m["combinations"]),
    "RS-HYD": lambda m: any("test_condition" in c or c["id"] == "load:RS-HYD" for c in _cases(m)),
    "Operating condition set": lambda m: bool(m.get("condition_sets")) or any(
        "condition_set_ref" in p for p in m["pipe_segments"]),
    "Modal (RS-OPE linearization)": lambda m: any("modal" in key for key in walk_keys(m)),
    "Response spectrum, RS-SP1": lambda m: bool(m.get("spectra")),
    "Hanger sizing and table": lambda m: any("hanger_sizing" in key for key in walk_keys(m)),
    "Snubber RS-SN1 (RS-110, UX, rigid)": lambda m: bool(_supports(m, "snubber")),
}
# A sub-feature a row in tranche 1 calls pending (`<name>` pending): held exactly when the state does
# not call it pending.
SUB_FEATURES = {
    "RS-EXP": ("difference_basis", lambda m: "difference_basis" in (_combination(m, "combination:RS-EXP") or {})),
}
# An interim record a pending row says is in tranche 1 (`<tag> is in tranche 1`), and what the fixture
# must hold for it.
INTERIM = {
    "Eccentric reducer RS-R1 in region RG-R": ("RS-P150", lambda m: by_id(m, "pipe_segments").get(
        "pipe:RS-P150", {}).get("section_ref") == "section:RS-S270"),
    "RS-FR1 with RS-FR1V: lift-off friction shoe pending RT-A3 S7": ("RS-FR1V", lambda m: any(
        s["id"] == "support:RS-FR1V" and s.get("family") == "vertical_support" and s.get("restraints") == ["UZ"]
        for s in m["supports"])),
}


def fixture_identities(model: dict) -> set[str]:
    ids = {record["id"] for collection in COLLECTIONS for record in model[collection]}
    for material in model["materials"]:
        ids |= {point["id"] for point in material.get("temperature_points", [])}
        ids |= {law["id"] for law in material.get("expansion_laws", [])}
    for component in model["components"]:
        ids |= {rod["id"] for rod in component.get("objective_connector", {}).get("hardware", {}).get("rods", [])}
    for case in model["load_cases"]:
        ids |= {region["id"] for region in case.get("pressure_regions", [])}
    return ids


def coverage_problems(spec: str, model: dict) -> list[str]:
    problems = []
    held = fixture_identities(model)
    for family, row in spec_ledger(spec).items():
        if family not in FAMILY_PRESENT:
            problems.append(f"{family}: no fixture predicate; add one to FAMILY_PRESENT")
            continue
        present = FAMILY_PRESENT[family](model)
        if present != row["in_tranche_1"]:
            problems.append(f"{family}: the ledger says {'in tranche 1' if row['in_tranche_1'] else 'pending'}, "
                            f"the fixture {'holds' if present else 'does not hold'} it")
        if family in SUB_FEATURES:
            name, holds = SUB_FEATURES[family]
            stated_pending = f"`{name}` pending" in row["state"]
            if holds(model) == stated_pending:
                problems.append(f"{family}: the ledger says `{name}` is {'pending' if stated_pending else 'held'}, "
                                f"the fixture {'holds' if holds(model) else 'does not hold'} it")
        if family in INTERIM:
            subject, holds = INTERIM[family]
            if f"{subject} is in tranche 1" in row["state"] and not holds(model):
                problems.append(f"{family}: the ledger says {subject} is in tranche 1 as stated; the fixture differs")
        for entity in row["entities"]:
            if row["in_tranche_1"] and entity not in held:
                problems.append(f"{family}: in tranche 1, but the fixture has no {entity}")
            if not row["in_tranche_1"] and entity in held and f"{tag(entity)} is in tranche 1" not in row["state"]:
                problems.append(f"{family}: pending, but the fixture holds {entity} and the state does not say why")
    return problems


def test_the_specification_ledger_agrees_with_the_generator_and_the_authoring_flags():
    stated = {family: {"in_tranche_1": row["in_tranche_1"], "entities": row["entities"]}
              for family, row in spec_ledger(SPEC).items()}
    generator = {family: {"in_tranche_1": state.startswith("in tranche 1"),
                          "entities": sorted(GENERATOR.LEDGER_ENTITIES.get(family, []))}
                 for family, state, _lane, _expected in GENERATOR.LEDGER}
    authored = {row["family"]: {"in_tranche_1": row["in_tranche_1"], "entities": sorted(row["entities"])}
                for row in AUTHORING["pending"]}
    assert stated == generator, "the specification's §10 and the generator's LEDGER differ"
    assert stated == authored, "the specification's §10 and authoring.json's pending rows differ"


def test_each_ledger_row_agrees_with_what_the_fixture_holds():
    assert coverage_problems(SPEC, MODEL) == []


# -- (2) identities -------------------------------------------------------------------------------------------------
LEDGER_IDS = {entity for row in AUTHORING["pending"] for entity in row["entities"]}
DRAFTING_IDS = {bend["corner"]["node_id_while_drafting"] for bend in AUTHORING["bends"]}


def missing_identities(record: str, text: str, model: dict) -> list[str]:
    missing = []
    for identity in sorted(fixture_identities(model)):
        if identity.startswith("region:") and record == "SPECIFICATION.md":
            # The specification states the regions once for every case: `region:<case>-<region>`.
            case, region = re.fullmatch(r"region:(RS-[A-Z]+)-(RG-\w+)", identity).groups()
            if not (names(text, case) and names(text, region)):
                missing.append(identity)
        elif not names(text, tag(identity)):
            missing.append(identity)
    return missing


def unknown_identities(text: str, model: dict) -> list[str]:
    known = fixture_identities(model) | LEDGER_IDS | DRAFTING_IDS
    unknown = set(FULL_ID.findall(text)) - known - NUMBERING_PREFIXES
    unknown |= set(TAG.findall(text)) - {tag(identity) for identity in known}
    return sorted(unknown)


@pytest.mark.parametrize("record", sorted(RECORDS))
def test_every_identity_the_fixture_holds_is_named(record):
    assert missing_identities(record, RECORDS[record][1], MODEL) == []


@pytest.mark.parametrize("record", sorted(RECORDS))
def test_every_identity_a_record_names_is_held_by_the_fixture_or_the_ledger(record):
    assert unknown_identities(RECORDS[record][1], MODEL) == []


def counted(identity: str) -> str:
    return f"{len(identity)}:{identity}"


def primitive_identity_problems(model: dict, spec: str) -> list[str]:
    """Every self-weight primitive's id follows the specification's rule (§8.1): a line load on every
    weighted member that no rigid component maps, and a force and a moment for each rigid component."""
    problems = []
    rigid = [c for c in model["components"] if c.get("geometry", {}).get("rigid_pipe_ref")]
    mapped = {c["geometry"]["rigid_pipe_ref"] for c in rigid}
    weighted = [p["id"] for p in model["pipe_segments"] if p["id"] != f"pipe:{GENERATOR.REPLACED_SPAN}"]
    for case in _cases(model):
        prefix = f"load:self-weight:{counted(case['id'])}:"
        expected = {prefix + counted(pipe) for pipe in weighted if pipe not in mapped}
        expected |= {prefix + f"component:{counted(c['id'])}:force:z" for c in rigid}
        actual = {p["id"] for p in case["primitive_loads"]}
        moments = {i for i in actual
                   if re.fullmatch(re.escape(prefix) + r"component:\d+:component:RS-\w+:moment:[xyz]", i)}
        if actual - moments != expected or len(moments) != len(rigid):
            problems.append(f"{case['id']}: primitive identities do not follow the rule: "
                            f"{sorted((actual - moments) ^ expected)}")
    examples = re.findall(r"`(load:self-weight:[^`<…]+)`", spec)
    held = {p["id"] for case in _cases(model) for p in case["primitive_loads"]}
    if not examples or not set(examples) <= held:
        problems.append(f"the specification's example primitive identity is not one the fixture holds: {examples}")
    return problems


def test_the_self_weight_primitive_identities_follow_the_stated_rule():
    assert primitive_identity_problems(MODEL, SPEC) == []


# -- (3) what the run sheet has the person enter, field by field ----------------------------------------------------
def pasted_texts(text: str) -> dict[str, str]:
    texts = dict(re.findall(r"\*\*⟨(\w+)⟩\*\* = `([^`]+)`", text))
    assert {"P", "S5"} <= set(texts), "the run sheet defines ⟨P⟩ and ⟨S5⟩"
    return texts


def label_pattern(label: str) -> str:
    """A field label as the step writes it, bold or not, its words wrapped across lines or not."""
    return r"(?<![\w-])(?:\*\*)?" + r"\s+".join(map(re.escape, label.split())) + r"(?:\*\*)?\s+"


def instructed(text: str, step_name: str, label: str) -> list[str]:
    """Every value the step tells the person to enter in the field `label`: ⟨P⟩ or ⟨S5⟩, with any
    backticked text that follows, or a backticked literal."""
    texts = pasted_texts(text)
    pattern = label_pattern(label) + r"(?:⟨(\w+)⟩(?:`([^`]*)`)?|`([^`]+)`)"
    found = re.findall(pattern, step(text, step_name))
    assert found, f"step {step_name} no longer tells the person what to enter in {label!r}"
    return [texts[alias] + (suffix or "") if alias else re.sub(r"\s+", " ", literal) for alias, suffix, literal in found]


def table_values(text: str, step_name: str, first_header: str, column: str, key: str) -> dict[str, str]:
    texts = pasted_texts(text)

    def resolve(cell: str) -> str:
        return re.sub(r"⟨(\w+)⟩(?:`([^`]*)`)?", lambda m: texts[m.group(1)] + (m.group(2) or ""), cell).strip("`")

    return {row[key].strip("`"): resolve(row[column])
            for row in table(step(text, step_name), first_header, having=column)}


def _rigid(m):
    return [c for c in m["components"] if c.get("kind") in ("valve", "flange")]


def _connector(m):
    return _components(m, "expansion_joint")[0]["objective_connector"]


def _sheet_records(m):
    """The records the run sheet's rows create, which take the header's Source as provenance."""
    bend_ids = set()
    for bend in AUTHORING["bends"]:
        bend_ids |= {bend["component"], bend["creates"]["arc_pipe"], *bend["creates"]["tangent_nodes"]}
    records = m["nodes"] + m["pipe_segments"] + [s for s in m["supports"] if s.get("family") != "variable_spring_hanger"]
    return records + [c for c in m["components"] if c["id"] in bend_ids]


# (step, the field's label as the step writes it, the values of the field the entry is written to)
BINDINGS = [
    ("material", "Constitutive provenance", lambda m: [x["provenance"] for x in m["materials"]]),
    ("material", "Point provenance", lambda m: [p["provenance"] for x in m["materials"] for p in x["temperature_points"]]),
    ("material", "Expansion law provenance", lambda m: [p["provenance"] for x in m["materials"] for p in x["expansion_laws"]]),
    ("sections", "Provenance", lambda m: [x["provenance"] for x in m["sections"]]),
    ("rows", "Source", lambda m: [x["provenance"] for x in _sheet_records(m)]),
    ("rows", "Stiffness behavior reference", lambda m: [c["geometry"]["stiffness_behavior_reference"] for c in _rigid(m)]),
    ("rows", "Stiffness scale source", lambda m: [c["modifiers"]["source_reference"] for c in _rigid(m)]),
    ("rows", "Component geometry source", lambda m: [c["geometry"]["rigid_component_source_reference"] for c in _rigid(m)]),
    ("rows", "Component provenance", lambda m: [c["provenance"] for c in _rigid(m)]),
    ("joint", "Rod source reference", lambda m: [r["source_reference"] for r in _connector(m)["hardware"]["rods"]]),
    ("joint", "Joint pressure area source reference", lambda m: [_connector(m)["pressure_model"]["source_reference"]]),
    ("joint", "Stiffness source reference", lambda m: [_connector(m)["provenance"]["source_reference"],
                                                       _connector(m)["stiffness"]["provenance"]["source_reference"]]),
    ("joint", "Measurement restraints", lambda m: [_connector(m)["provenance"]["measurement_restraints"]]),
    ("joint", "Basis transform reference", lambda m: [_connector(m)["provenance"]["basis_transform_reference"]]),
    ("joint", "Validity statement", lambda m: [_connector(m)["provenance"]["validity_statement"]]),
    ("joint", "New component provenance", lambda m: [c["provenance"] for c in _components(m, "expansion_joint")]),
    ("tee", "New component geometry source",
     lambda m: [c["geometry"]["branch_geometry_source_reference"] for c in _components(m, "tee")]),
    ("tee", "New component provenance", lambda m: [c["provenance"] for c in _components(m, "tee")]),
    ("tee", "Modifier source", lambda m: [c["modifiers"]["source_reference"] for c in _components(m, "tee")]),
    ("spring", "Support provenance", lambda m: [s["provenance"] for s in _supports(m, "variable_spring_hanger")]),
    ("spring", "Hanger source reference",
     lambda m: [s["hanger"]["source_reference"] for s in _supports(m, "variable_spring_hanger")]),
    ("reference", "Member provenance",
     lambda m: [r["provenance"] for c in m["reference_configurations"] for r in c["member_references"]]),
    ("reference", "Configuration provenance", lambda m: [c["provenance"] for c in m["reference_configurations"]]),
    ("regions", "Terminal provenance",
     lambda m: [t["provenance"] for c in _cases(m) for r in c["pressure_regions"] for t in r["terminals"]]),
    ("regions", "Region provenance", lambda m: [r["provenance"] for c in _cases(m) for r in c["pressure_regions"]]),
    ("states", "Analysis state provenance", lambda m: [c["analysis_state"]["provenance"] for c in _cases(m)]),
    ("states", "Thermal provenance", lambda m: [s["thermal_state"]["provenance"] for c in _cases(m)
                                                for s in c["analysis_state"]["element_states"]
                                                if "provenance" in s.get("thermal_state", {})]),
    ("combinations", "Provenance", lambda m: [c["provenance"] for c in m["combinations"]]),
]


def field_problems(text: str, model: dict) -> list[str]:
    differences = re.sub(r"\s+", " ", level2_section(text, "differences"))
    problems = []
    for step_name, label, values in BINDINGS:
        entered = set(instructed(text, step_name, label))
        held = set(values(model))
        unlisted = sorted(value for value in held - entered if value not in differences)
        stray = sorted(entered - held)
        if unlisted or stray:
            problems.append(f"step {step_name}, {label}: the fixture holds {unlisted or 'nothing else'}, "
                            f"the sheet enters {stray or 'nothing else'}")
    # Table-bound values: the bends' sources and each case's self-weight provenance.
    components = by_id(model, "components")
    for column, field in (("k source", lambda c: c["modifiers"]["source_reference"]),
                          ("Bend geometry source", lambda c: c["geometry"]["bend_geometry_source_reference"])):
        for component, value in table_values(text, "rows", "#", column, "Bend component id").items():
            if field(components[component]) != value:
                problems.append(f"step rows, {column} of {component}: the sheet enters {value!r}")
    cases = by_id(model, "load_cases")
    for case, value in table_values(text, "weight", "Self-weight case ID", "Self-weight provenance",
                                    "Self-weight case ID").items():
        if cases[case]["provenance"] != value:
            problems.append(f"step weight, provenance of {case}: the sheet enters {value!r}")
    # A difference the run sheet can detect: the joint's stiffness provenance, which the form writes from
    # the connector's own references.
    connector = _connector(model)
    for key in ("measurement_restraints", "basis_transform_reference"):
        value = connector["stiffness"]["provenance"][key]
        if value != connector["provenance"][key] and value not in differences:
            problems.append(f"the joint's stiffness provenance {key} differs from the connector's and is not listed")
    return problems


def test_each_entered_source_and_provenance_is_the_value_of_its_field():
    assert field_problems(RUN_SHEET, MODEL) == []


# -- (4) citations --------------------------------------------------------------------------------------------------
TOP_LEVEL = ("apps", "core", "docs", "execution", "fixtures", "schemas", "tests", "tools", "validation")
REPO_PATH = re.compile(rf"`((?:{'|'.join(TOP_LEVEL)})/[^`\s]*)`")
LINK = re.compile(r"\]\(([^)\s]+)\)")
FILE_NAME = re.compile(r"(?<![\w./-])[\w-]+\.(?:json|md|py|rs|ts|tsx|mjs|yaml|yml)(?![\w/])")


def citation_problems(path: Path, text: str) -> list[str]:
    cited = []
    for target in LINK.findall(text):
        if re.match(r"[a-z]+:", target) or target.startswith("#"):
            continue
        cited.append((target, (path.parent / target.split("#", 1)[0]).resolve()))
    cited += [(target, (ROOT / target).resolve()) for target in REPO_PATH.findall(text)]
    problems = [] if cited else ["cites no path"]
    for target, resolved in cited:
        if not resolved.is_relative_to(ROOT):
            problems.append(f"{target}: outside the repository")
        elif resolved.relative_to(ROOT).parts[:1] == ("execution",):
            problems.append(f"{target}: under execution/, which the product test job cannot check; cite the id")
        elif not resolved.exists():
            problems.append(f"{target}: does not exist")
    # A bare file name resolves beside the record or in a directory the record cites.
    directories = [path.parent] + [resolved for _target, resolved in cited if resolved.is_dir()]
    for name in sorted(set(FILE_NAME.findall(re.sub(r"\]\([^)]*\)", "]", text)))):
        if not any((directory / name).exists() for directory in directories):
            problems.append(f"{name}: no such file beside the record or in a directory it cites")
    return problems


@pytest.mark.parametrize("record", sorted(RECORDS))
def test_every_path_and_file_a_record_cites_exists(record):
    path, text = RECORDS[record]
    assert citation_problems(path, text) == []


# -- the checks catch what they exist to catch ----------------------------------------------------------------------
def _add(collection, record):
    return lambda m: m.setdefault(collection, []).append(record)


def _set_case(case_id, key, value):
    return lambda m: _case(m, case_id).__setitem__(key, value)


def _keep(collection, keep):
    return lambda m: m.__setitem__(collection, [x for x in m[collection] if keep(x)])


# For each family, a change to the fixture that flips whether it holds the family.
FLIP = {
    "Straight members, sections, temperature-dependent material":
        lambda m: [x.pop("temperature_points") for x in m["materials"]],
    "Realized bends with user k and SIFs":
        lambda m: [c["modifiers"].pop("sif_in_plane_user_value") for c in _components(m, "bend")],
    "Valve and flanges, weight at CoG, stiffness scale": _keep("components", lambda c: c.get("kind") != "valve"),
    "Expansion joint with tie rods, pressurized":
        lambda m: [r.pop("member_connector_ids", None) for c in _cases(m) for r in c["pressure_regions"]],
    "Pressure (five regions, shared terminals `transfers_to_wall`)": lambda m: _cases(m)[0]["pressure_regions"][0][
        "terminals"][0].__setitem__("closure_transfer", "separately_supported_or_compensated"),
    "Anchors, guides, line stop, vertical supports": _keep("supports", lambda s: s.get("family") != "guide"),
    "Variable spring as a user-stiffness spring": _keep("supports", lambda s: s.get("family") != "variable_spring_hanger"),
    "Tee RS-T1 with user SIFs":
        lambda m: [c["modifiers"].pop("branch_header_sif_user_value") for c in _components(m, "tee")],
    "Eccentric reducer RS-R1 in region RG-R": _add("components", {"id": "component:RS-R1", "kind": "reducer"}),
    "Constant-effort support RS-CE1": _add("supports", {"id": "support:RS-CE1", "family": "constant_effort_support"}),
    "Lift-off, friction and gap (RS-LO1, RS-FR1, RS-G1)":
        _add("supports", {"id": "support:RS-LO1", "family": "nonlinear"}),
    "RS-FR1 with RS-FR1V: lift-off friction shoe pending RT-A3 S7":
        lambda m: by_id(m, "supports")["support:RS-FR1V"].__setitem__("family", "nonlinear"),
    "RS-SUS, RS-OPE (thermal, modulus by temperature)":
        lambda m: [s.__setitem__("thermal_state", {"kind": "unchanged_reference"})
                   for s in _case(m, "load:RS-OPE")["analysis_state"]["element_states"]],
    "RS-EXP": _keep("combinations", lambda c: c["id"] != "combination:RS-EXP"),
    "RS-WIN, RS-EQX, RS-EQY, RS-EQZ on the document that carries temperature":
        _set_case("load:RS-SUS", "equivalent_static", {}),
    "RS-SAM (`boundary_motion` on rigid anchor RS-C)":
        lambda m: [s.pop("boundary_motion", None) for s in _case(m, "load:RS-SAM")["analysis_state"]["support_states"]],
    "RS-EQ-SRSS, RS-EQ-ABS, RS-OCC-SRSS, RS-OCC-ABS":
        _add("combinations", {"id": "combination:RS-EQ-SRSS", "basis": "srss"}),
    "RS-HYD": _set_case("load:RS-SUS", "test_condition", {"kind": "hydrostatic"}),
    "Operating condition set": _add("condition_sets", {"id": "condition_set:RS-OPER"}),
    "Modal (RS-OPE linearization)": lambda m: m.__setitem__("modal_request", {}),
    "Response spectrum, RS-SP1": _add("spectra", {"id": "spectrum:RS-SP1"}),
    "Hanger sizing and table": lambda m: m.__setitem__("hanger_sizing_request", {}),
    "Snubber RS-SN1 (RS-110, UX, rigid)": _add("supports", {"id": "support:RS-SN1", "family": "snubber"}),
}


@pytest.mark.parametrize("family", sorted(FAMILY_PRESENT))
def test_a_change_of_the_fixture_s_coverage_of_each_family_is_caught(family):
    model = copy.deepcopy(MODEL)
    FLIP[family](model)
    assert FAMILY_PRESENT[family](model) != FAMILY_PRESENT[family](MODEL), f"{family}: the control does not flip it"
    assert any(problem.startswith(family + ":") for problem in coverage_problems(SPEC, model))


def test_a_changed_sub_feature_or_interim_record_is_caught():
    for family, (name, _holds) in SUB_FEATURES.items():
        model = copy.deepcopy(MODEL)
        _combination(model, "combination:RS-EXP")[name] = {}
        assert any(problem.startswith(family + ":") for problem in coverage_problems(SPEC, model))
        state = spec_ledger(SPEC)[family]["state"]
        recorded = SPEC.replace(state, state.replace(f"`{name}` pending", f"`{name}` recorded"), 1)
        assert recorded != SPEC
        assert any(problem.startswith(family + ":") for problem in coverage_problems(recorded, MODEL))
    for family, (subject, _holds) in INTERIM.items():
        model = copy.deepcopy(MODEL)
        if subject == "RS-P150":
            by_id(model, "pipe_segments")["pipe:RS-P150"]["section_ref"] = "section:RS-S220"
        else:
            by_id(model, "supports")[f"support:{subject}"]["restraints"] = ["UX"]
        assert any(problem.startswith(family + ":") for problem in coverage_problems(SPEC, model)), family


def test_a_missing_or_unknown_identity_is_caught():
    for record, (_path, text) in RECORDS.items():
        dropped = sorted(fixture_identities(MODEL))[0]
        without = re.sub(rf"(?<![\w-]){re.escape(tag(dropped))}(?![\w-])", "", text)
        assert dropped in missing_identities(record, without, MODEL), record
        assert "RS-P999" in unknown_identities(text + "\n| RS-P999 | 999 → 998 | S270 | |\n", MODEL), record
        assert "component:RS-ZZ9" in unknown_identities(text + "\n`component:RS-ZZ9`\n", MODEL), record
    model = copy.deepcopy(MODEL)
    _cases(model)[0]["primitive_loads"][0]["id"] += "-renamed"
    assert primitive_identity_problems(model, SPEC)


def test_a_stale_source_or_a_changed_fixture_field_is_caught():
    texts = pasted_texts(RUN_SHEET)
    # A stale ⟨P⟩: every field it is written to now differs.
    assert field_problems(RUN_SHEET.replace(f"`{texts['P']}`", f"`{texts['P']} (stale)`", 1), MODEL)
    # A literal entered in place of the pasted source, in each bound field in turn.
    for step_name, label, _values in BINDINGS:
        body = step(RUN_SHEET, step_name)
        literal = re.sub("(" + label_pattern(label) + r")(?:⟨\w+⟩|`[^`]+`)",
                         r"\1`a stale text`", body, count=1)
        assert literal != body, (step_name, label)
        assert field_problems(RUN_SHEET.replace(body, literal, 1), MODEL), (step_name, label)
    # Only the fixture's hanger source changes.
    model = copy.deepcopy(MODEL)
    _supports(model, "variable_spring_hanger")[0]["hanger"]["source_reference"] = "another source"
    assert field_problems(RUN_SHEET, model)


def test_a_dead_link_a_dead_file_name_and_an_execution_path_are_caught():
    for record, (path, text) in RECORDS.items():
        assert citation_problems(path, text + "\n[gone](no-such-file.md)\n"), record
        assert citation_problems(path, text + "\nsee no_such_file.json\n"), record
        assert citation_problems(path, text + "\n`execution/PKG-09/DEL-09-01/ScopeOfWork.md`\n"), record
