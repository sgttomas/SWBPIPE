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
import posixpath
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
# Whole identity tokens: everything up to the next character an identity cannot hold, so that a suffix such
# as `pipe:RS-P010_old` is part of the token and not left behind.
FULL_ID = re.compile(r"\b(?:node|pipe|component|support|load|combination|material|section|point|law|rod|reference"
                     r"|region|spectrum):RS-[\w-]*[A-Za-z0-9_]")
# A bare tag (`RS-P150`, `RS-130`, `RS-B1`, `RS-SUS-RG-1`): what the records' tables and prose use.
TAG = re.compile(r"(?<![\w:-])RS-[\w]+(?:-[\w]+)*(?![\w-])")
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
        cells = cells_of(line) if line.lstrip().startswith("|") else []
        if cells and cells[0] == first_header and (having is None or having in cells):
            rows = []
            for row in lines[index + 2:]:
                if not row.lstrip().startswith("|"):
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
    "Realized bends with user k and SIFs": lambda m: bool(_components(m, "bend")) and all(
        c.get("mechanics_interface", {}).get("solver_consumption") == "curved_bend_macro_element"
        and {"flexibility_factor_user_value", "sif_in_plane_user_value", "sif_out_of_plane_user_value"}
        <= set(c.get("modifiers", {})) for c in _components(m, "bend")),
    "Valve and flanges, weight at CoG, stiffness scale": lambda m: all(_components(m, kind) for kind in (
        "valve", "flange")) and all(
        "weight" in c["geometry"] and "center_of_gravity" in c["geometry"]
        and "stiffness_scaling_user_value" in c.get("modifiers", {}) for c in _rigid(m)),
    "Expansion joint with tie rods, pressurized": _joint_in_region,
    "Pressure (five regions, shared terminals `transfers_to_wall`)": _regions_closed,
    "Anchors, guides, line stop, vertical supports": lambda m: {"anchor", "guide", "line_stop", "vertical_support"}
    <= {s.get("family") for s in m["supports"]},
    "Variable spring as a user-stiffness spring": lambda m: bool(_supports(m, "variable_spring_hanger")) and all(
        s.get("hanger", {}).get("stiffness") for s in _supports(m, "variable_spring_hanger")),
    "Tee RS-T1 with user SIFs": lambda m: bool(_components(m, "tee")) and all(
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


def ledger_problems(spec: str) -> list[str]:
    """The specification's §10 against the generator's LEDGER and authoring.json's pending rows, by
    family: its flag and its identities."""
    stated = {family: {"in_tranche_1": row["in_tranche_1"], "entities": row["entities"]}
              for family, row in spec_ledger(spec).items()}
    sources = {
        "the generator's LEDGER": {family: {"in_tranche_1": state.startswith("in tranche 1"),
                                            "entities": sorted(GENERATOR.LEDGER_ENTITIES.get(family, []))}
                                   for family, state, _lane, _expected in GENERATOR.LEDGER},
        "authoring.json": {row["family"]: {"in_tranche_1": row["in_tranche_1"], "entities": sorted(row["entities"])}
                           for row in AUTHORING["pending"]},
    }
    problems = []
    for source, rows in sources.items():
        for family in sorted(set(stated) | set(rows)):
            if stated.get(family) != rows.get(family):
                problems.append(f"{family}: the specification says {stated.get(family)}, {source} {rows.get(family)}")
    return problems


def test_the_specification_ledger_agrees_with_the_generator_and_the_authoring_flags():
    assert ledger_problems(SPEC) == []


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


COMPONENT_PRIMITIVE = re.compile(r"component:(\d+):(component:RS-[A-Za-z0-9-]+):(force|moment):([xyz])")


def primitive_identity_problems(model: dict, spec: str) -> list[str]:
    """Every self-weight primitive's id follows the specification's rule (§8.1) and names its own target:
    a line load `<n>:<case>:<m>:<pipe>` on every weighted member that no rigid component maps, and for
    each rigid component exactly one force and one moment `<n>:<case>:component:<m>:<component>:...`,
    each counted correctly and applied at that component's node."""
    problems = []
    rigid = {c["id"]: c for c in model["components"] if c.get("geometry", {}).get("rigid_pipe_ref")}
    mapped = {c["geometry"]["rigid_pipe_ref"] for c in rigid.values()}
    weighted = {p["id"] for p in model["pipe_segments"] if p["id"] != f"pipe:{GENERATOR.REPLACED_SPAN}"}
    for case in _cases(model):
        prefix = f"load:self-weight:{counted(case['id'])}:"
        lines, per_component = [], {cid: [] for cid in rigid}
        ids = [load["id"] for load in case["primitive_loads"]]
        for duplicate in sorted({i for i in ids if ids.count(i) > 1}):
            problems.append(f"{case['id']}: primitive id {duplicate} appears {ids.count(duplicate)} times")
        for load in case["primitive_loads"]:
            pid, target = load["id"], load.get("target", {})
            if not pid.startswith(prefix):
                problems.append(f"{pid}: does not start with its case {prefix}")
                continue
            rest = pid[len(prefix):]
            component = COMPONENT_PRIMITIVE.fullmatch(rest)
            if component:
                count, cid, kind, axis = component.groups()
                if int(count) != len(cid):
                    problems.append(f"{pid}: counts {count} characters for {cid}, which has {len(cid)}")
                if cid not in rigid:
                    problems.append(f"{pid}: names {cid}, which is not a rigid component")
                    continue
                if target != {"node": rigid[cid]["node"], "type": "node"}:
                    problems.append(f"{pid}: applied at {target}, not at {cid}'s node {rigid[cid]['node']}")
                if kind == "force" and axis != "z":
                    problems.append(f"{pid}: a component weight is a force along z")
                per_component[cid].append(kind)
            else:
                if rest != counted(target.get("pipe", "")):
                    problems.append(f"{pid}: names {rest}, its target is {target}")
                lines.append(target.get("pipe"))
        if sorted(lines) != sorted(weighted - mapped):
            extra = sorted(p for p in set(lines) if lines.count(p) > 1 or p not in weighted - mapped)
            missing = sorted((weighted - mapped) - set(lines))
            problems.append(f"{case['id']}: line loads differ from one per required member: "
                            f"repeated or unexpected {extra}, missing {missing}")
        for cid, kinds in per_component.items():
            if sorted(kinds) != ["force", "moment"]:
                problems.append(f"{case['id']}: {cid} has {sorted(kinds)}, not one force and one moment")
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


VALUE = r"(?:to\s+)?(?:⟨(\w+)⟩(?:`([^`]*)`)?|`([^`]+)`)"


def label_pattern(label: str) -> str:
    """A field label as the step writes it, bold or not, its words wrapped across lines or not."""
    return r"(?<![\w-])(?:\*\*)?" + r"\s+".join(map(re.escape, label.split())) + r"(?:\*\*)?\s+"


def instructed(text: str, step_name: str, label: str) -> list[str]:
    """Every value the step tells the person to enter in the field `label`: ⟨P⟩ or ⟨S5⟩, with any
    backticked text that follows, or a backticked literal."""
    texts = pasted_texts(text)
    found = re.findall(label_pattern(label) + VALUE, step(text, step_name))
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
    ("tee", "Connection type", lambda m: [c["geometry"]["branch_connection_type"] for c in _components(m, "tee")]),
    ("tee", "Reinforcement reference",
     lambda m: [c["geometry"]["branch_reinforcement_reference"] for c in _components(m, "tee")]),
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
    ("combinations", "Provenance", lambda m: [c["provenance"] for c in m["combinations"]]),
]


def listed(differences: str, title: str, value: str | None) -> bool:
    """Whether the run sheet's differences list this field: the bullet titled exactly `title` (its bold
    title, `**<title>.**`) gives the fixture's value in backticks. A value under another title does not
    count."""
    for bullet in re.split(r"(?m)^- ", differences)[1:]:
        bullet = re.sub(r"\s+", " ", bullet)
        heading = re.match(r"\*\*(.+?)\.\*\*", bullet)
        if heading and heading.group(1).lower() == title.lower() and value is not None and f"`{value}`" in bullet:
            return True
    return False


def field_problems(text: str, model: dict) -> list[str]:
    differences = level2_section(text, "differences")
    problems = []
    # One value per field label, written to every record the binding names: each record is compared.
    for step_name, label, values in BINDINGS:
        entered = set(instructed(text, step_name, label))
        if len(entered) != 1:
            problems.append(f"step {step_name}, {label}: the sheet gives {len(entered)} values for one field")
            continue
        (value,) = entered
        for held in values(model):
            if held != value and not listed(differences, label, held):
                problems.append(f"step {step_name}, {label}: a record holds {held!r}, the sheet enters {value!r}")
    # Table-bound values, each compared with its own record: the bends' sources, the valve and flanges'
    # end references, and each case's self-weight provenance.
    components = by_id(model, "components")
    for column, field in (("End A reference", "connection_end_a_reference"),
                          ("End B reference", "connection_end_b_reference")):
        rows = table_values(text, "rows", "#", column, "Component id")
        for component in _rigid(model):
            value = rows.get(component["id"])
            if value != component["geometry"][field]:
                problems.append(f"step rows, {column} of {component['id']}: the sheet enters {value!r}")
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
    for key, title in (("measurement_restraints", "Joint stiffness measurement restraints"),
                       ("basis_transform_reference", "Joint stiffness basis transform reference")):
        value = connector["stiffness"]["provenance"][key]
        if value != connector["provenance"][key] and not listed(differences, title, value):
            problems.append(f"the joint's stiffness provenance {key} {value!r} differs from the connector's "
                            f"and is not listed under {title!r}")
    return problems + thermal_problems(text, model, differences)


def thermal_problems(text: str, model: dict, differences: str) -> list[str]:
    """Each element state's thermal provenance against what step states has the person enter for that
    case and that member: a sentence naming a member binds its value to that member, the case's other
    value is its template for every member; a case copied from another without marking its element
    states absent keeps that case's values."""
    body = step(text, "states")
    parts = re.split(r"(?m)^- \*\*(RS-[A-Z]+)\.\*\*", body)
    bullets = dict(zip(parts[1::2], parts[2::2]))
    texts = pasted_texts(text)
    expected: dict[str, dict[str, str | None]] = {}
    for case, bullet in bullets.items():
        copied = re.search(r"Copy analysis state from case\*\* (RS-[A-Z]+)", bullet)
        own = {"*": None}
        if copied and "Mark element_states absent" not in bullet:
            own = dict(expected[copied.group(1)])
        # The typed row is the fill's template: a value entered before the fill ("Fill.") is every
        # member's; a value entered after it is the member its sentence names.
        filled = re.search(r"(?<![\w*])Fill\.", bullet)
        for match in re.finditer(label_pattern("Thermal provenance") + VALUE, bullet):
            alias, suffix, literal = match.groups()
            value = texts[alias] + (suffix or "") if alias else literal
            if filled is None or match.start() < filled.start():
                own["*"] = value
                continue
            sentence = bullet[max(bullet.rfind(". ", 0, match.start()), 0):match.start()]
            pipes = re.findall(r"(?<![\w-])RS-P\d{3}(?![\w-])", sentence)
            if not pipes:
                return [f"step states, {case}: a thermal provenance after the fill names no member"]
            own[f"pipe:{pipes[-1]}"] = value
        expected[case] = own
    problems = []
    for case in _cases(model):
        own = expected.get(tag(case["id"]))
        if own is None:
            problems.append(f"step states does not enter {case['id']}")
            continue
        for row in case["analysis_state"]["element_states"]:
            want = own.get(row["pipe_ref"], own["*"])
            held = row.get("thermal_state", {}).get("provenance")
            if held != want and not listed(differences, "thermal provenance", held):
                problems.append(f"step states, {case['id']} {row['pipe_ref']}: thermal provenance {held!r}, "
                                f"the sheet enters {want!r}")
    return problems


def test_each_entered_source_and_provenance_is_the_value_of_its_field():
    assert field_problems(RUN_SHEET, MODEL) == []


def label_problems(text: str, model: dict) -> list[str]:
    """Each label the run sheet has the person type against its own record's label: node labels by row,
    support and component labels by id, and the cases' and combinations' labels by id."""
    problems = []
    rows = {row["#"]: row for row in table(step(text, "rows"), "#", having="Node id")}
    expected = {}
    for number, row in rows.items():
        node = re.match(r"(RS-\d{3})", row["Node id"])
        if node:
            expected[number] = f"node:{node.group(1)}"
    nodes = by_id(model, "nodes")
    for row in table(step(text, "rows"), "#", having="Label"):
        node = expected.get(row["#"])
        held = nodes.get(node, {}).get("label")
        if held != row["Label"].strip("`"):
            problems.append(f"step rows, label of row {row['#']} ({node}): the sheet enters {row['Label']!r}, "
                            f"the fixture holds {held!r}")
    for step_name, first, key, column, collection in (
            ("rows", "#", "Support id", "Support label", "supports"),
            ("rows", "#", "Component id", "Component label", "components"),
            ("weight", "Self-weight case ID", "Self-weight case ID", "Self-weight case label", "load_cases"),
            ("combinations", "ID", "ID", "Label", "combinations")):
        records = by_id(model, collection)
        for row in table(step(text, step_name), first, having=column):
            identity, value = row[key].strip("`"), row[column].strip("`")
            if records.get(identity, {}).get("label") != value:
                problems.append(f"step {step_name}, label of {identity}: the sheet enters {value!r}, "
                                f"the fixture holds {records.get(identity, {}).get('label')!r}")
    return problems


def number(text: str) -> float:
    return float(text.replace("−", "-").strip())


def same(entered: float, held: float) -> bool:
    return abs(entered - held) <= 1e-12 * max(1.0, abs(held))


def quantity(text: str) -> tuple[float, str]:
    value, _, unit = text.strip().partition(" ")
    return number(value), unit


SUPPORT_CODES = {"A": "anchor", "G": "guide", "L": "line_stop", "V": "vertical_support"}
ALL_SIX = ["UX", "UY", "UZ", "RX", "RY", "RZ"]


def value_problems(text: str, model: dict) -> list[str]:
    """The numbers, codes and references the run sheet's tables have the person enter, each against its own
    record: sections, temperature points, the valve and flanges' body, weight and centre of gravity, the
    supports' codes and restraints, the bends' radius, k and SIFs, the regions and the combinations' operands."""
    problems = []

    def check(where: str, entered, held) -> None:
        ok = same(entered, held) if isinstance(entered, float) and isinstance(held, (int, float)) else entered == held
        if not ok:
            problems.append(f"{where}: the sheet enters {entered!r}, the fixture holds {held!r}")

    sections = by_id(model, "sections")
    for row in table(step(text, "sections"), "Section ID"):
        record = sections.get(row["Section ID"].strip("`"), {}).get("properties", {})
        check(f"step sections, outside diameter of {row['Section ID']}", number(row["Outside diameter"]),
              record.get("outside_diameter", {}).get("value"))
        check(f"step sections, wall thickness of {row['Section ID']}", number(row["Wall thickness"]),
              record.get("wall_thickness", {}).get("value"))
    points = {p["id"]: p for x in model["materials"] for p in x.get("temperature_points", [])}
    for row in table(step(text, "material"), "Point ID"):
        point = points.get(row["Point ID"].strip("`"), {})
        for column, key in (("Temperature", "temperature"), ("Elastic modulus", "elastic_modulus"),
                            ("Thermal expansion coefficient", "thermal_expansion_coefficient")):
            value, unit = quantity(row[column])
            held = point.get(key, {})
            check(f"step material, {column.lower()} of {row['Point ID']}", (value, unit),
                  (float(held.get("value", "nan")), held.get("unit")))
        check(f"step material, Poisson ratio of {row['Point ID']}", number(row["Poisson ratio"]),
              point.get("poisson_ratio", {}).get("value"))
    components = by_id(model, "components")
    for row in table(step(text, "rows"), "#", having="Body length"):
        geometry = components.get(row["Component id"].strip("`"), {}).get("geometry", {})
        where = f"step rows, {{}} of {row['Component id']}"
        check(where.format("body length"), number(row["Body length"]), geometry.get("rigid_body_length", {}).get("value"))
        check(where.format("weight"), number(row["Weight"]), geometry.get("weight", {}).get("value"))
        cog = geometry.get("center_of_gravity", {})
        check(where.format("COG"), [number(v) for v in row["COG X, Y, Z"].split(",")],
              [float(cog.get(axis, "nan")) for axis in "xyz"])
    supports = by_id(model, "supports")
    for row in table(step(text, "rows"), "#", having="Restraints"):
        record = supports.get(row["Support id"].strip("`"), {})
        check(f"step rows, family of {row['Support id']}", SUPPORT_CODES.get(row["Support"]), record.get("family"))
        restraints = ALL_SIX if row["Restraints"] == "*" else row["Restraints"].split()
        check(f"step rows, restraints of {row['Support id']}", sorted(restraints), sorted(record.get("restraints", [])))
    for row in table(step(text, "rows"), "#", having="Bend R"):
        modifiers = components.get(row["Bend component id"].strip("`"), {}).get("modifiers", {})
        geometry = components.get(row["Bend component id"].strip("`"), {}).get("geometry", {})
        where = f"step rows, {{}} of {row['Bend component id']}"
        radius = re.search(r"[−-]?\d+(?:\.\d+)?", row["Bend R"]).group(0)
        check(where.format("Bend R"), number(radius), geometry.get("bend_radius", {}).get("value"))
        check(where.format("k"), number(row["k"]), modifiers.get("flexibility_factor_user_value", {}).get("value"))
        check(where.format("in-plane SIF"), number(row["In-plane SIF i_ip"]),
              modifiers.get("sif_in_plane_user_value", {}).get("value"))
        check(where.format("out-of-plane SIF"), number(row["Out-of-plane SIF i_op"]),
              modifiers.get("sif_out_of_plane_user_value", {}).get("value"))
    regions = {r["id"]: r for case in _cases(model) for r in case.get("pressure_regions", [])}
    for row in table(step(text, "regions"), "Region ID"):
        region = regions.get(row["Region ID"].strip("`"), {})
        where = f"step regions, {{}} of {row['Region ID']}"
        check(where.format("member pipes"), sorted(f"pipe:{t}" for t in re.findall(r"RS-P\d{3}", row["Member pipes"])),
              sorted(region.get("member_pipe_ids", [])))
        check(where.format("member connectors"), re.findall(r"component:RS-[\w-]+", row["Member connectors"]),
              region.get("member_connector_ids", []))
        check(where.format("terminals"), [f"node:{t}" for t in re.findall(r"RS-\d{3}", row["Terminal 1, terminal 2"])],
              [t["node_ref"] for t in region.get("terminals", [])])
    combinations = by_id(model, "combinations")
    for row in table(step(text, "combinations"), "ID"):
        record = combinations.get(row["ID"].strip("`"), {})
        check(f"step combinations, minuend of {row['ID']}", row["Minuend"].strip("`"), record.get("minuend_id"))
        check(f"step combinations, subtrahend of {row['ID']}", row["Subtrahend"].strip("`"), record.get("subtrahend_id"))
    return problems


def _q(record: dict) -> tuple[float, str]:
    return float(record["value"]), record.get("unit")


def _gravity(m):
    return [(float(g["value"]), g["unit"]) for c in _cases(m) for p in c["primitive_loads"]
            for g in [json.loads(p["provenance"])["gravity"]]]


# (step, the label the step writes before the number, the (value, unit) of every record the entry is written to)
NUMBERS = [
    ("material", "Elastic modulus E", lambda m: [_q(x["elastic_modulus"]) for x in m["materials"]]),
    ("material", "Poisson ratio nu", lambda m: [(float(x["poisson_ratio"]["value"]), None) for x in m["materials"]]),
    ("material", "Thermal expansion coefficient", lambda m: [_q(x["thermal_expansion_coefficient"]) for x in m["materials"]]),
    ("material", "datum temperature", lambda m: [_q(l["datum_temperature"]) for x in m["materials"] for l in x["expansion_laws"]]),
    ("material", "constant coefficient", lambda m: [_q(l["data"]["coefficient"]) for x in m["materials"] for l in x["expansion_laws"]]),
    ("rs050", "value", lambda m: [(float(by_id(m, "nodes")["node:RS-050"]["position"]["x"]), None)]),
    ("joint", "Installed reference temperature", lambda m: [_q(_connector(m)["installed_reference_temperature"])]),
    ("joint", "Joint effective area", lambda m: [_q(_connector(m)["pressure_model"]["effective_area"])]),
    ("joint", "Rod stiffness", lambda m: [_q(r["stiffness"]) for r in _connector(m)["hardware"]["rods"]]),
    ("tee", "Connection angle", lambda m: [_q(c["geometry"]["branch_connection_angle"])[:1] + (None,) for c in _components(m, "tee")]),
    ("tee", "Run size", lambda m: [(float(c["geometry"]["branch_run_size"]["value"]), None) for c in _components(m, "tee")]),
    ("tee", "Header size", lambda m: [(float(c["geometry"]["branch_header_size"]["value"]), None) for c in _components(m, "tee")]),
    ("tee", "Header SIF user value,", lambda m: [(float(c["modifiers"]["branch_header_sif_user_value"]["value"]), None) for c in _components(m, "tee")]),
    ("tee", "Branch SIF user value,", lambda m: [(float(c["modifiers"]["branch_branch_sif_user_value"]["value"]), None) for c in _components(m, "tee")]),
    ("spring", "quantity", lambda m: [_q(s["hanger"]["stiffness"]["value"]) for s in _supports(m, "variable_spring_hanger")]),
    ("spring", "installed load", lambda m: [_q(s["hanger"]["installed_load"]) for s in _supports(m, "variable_spring_hanger")]),
    ("spring", "cold load", lambda m: [_q(s["hanger"]["cold_load"]) for s in _supports(m, "variable_spring_hanger")]),
    ("spring", "hot load", lambda m: [_q(s["hanger"]["hot_load"]) for s in _supports(m, "variable_spring_hanger")]),
    ("spring", "travel range", lambda m: [_q(s["hanger"]["travel_range"]) for s in _supports(m, "variable_spring_hanger")]),
    ("reference", "installation temperature", lambda m: [_q(r["basis"]["installation_temperature"])
                                                         for c in m["reference_configurations"] for r in c["member_references"]]),
    ("weight", "Gravity value", lambda m: [(v, None) for v, _u in _gravity(m)]),
    ("regions", "pressure", lambda m: [_q(r["pressure"]) for c in _cases(m) for r in c["pressure_regions"]]),
    ("states", "operating temperature", lambda m: [_q(s["operating_temperature"]) for c in _cases(m)
                                                   for s in c["analysis_state"]["element_states"] if "operating_temperature" in s]),
    ("states", "selection temperature", lambda m: [_q(s["material_selection"]["temperature"]) for c in _cases(m)
                                                   for s in c["analysis_state"]["element_states"]
                                                   if "temperature" in s["material_selection"]]),
]
NUMBER = r"(?:\*\*)?\s*([−-]?\d+(?:\.\d+)?(?:e[−-]?\d+)?)(?:\s+([A-Za-z^/0-9*]+(?:\^\d)?))?"


def number_problems(text: str, model: dict) -> list[str]:
    """Each number the steps' prose has the person enter, with its unit where the record keeps one, against
    every record it is written to; and the joint's work matrix and RS-C's boundary motions."""
    problems = []
    for step_name, label, values in NUMBERS:
        found = re.search(label_pattern(label).removesuffix(r"(?:\*\*)?\s+") + NUMBER, step(text, step_name))
        if not found:
            problems.append(f"step {step_name}, {label}: the sheet no longer gives a number")
            continue
        entered, unit = number(found.group(1)), found.group(2)
        for held, held_unit in values(model):
            if not same(entered, held) or (held_unit is not None and unit != held_unit):
                problems.append(f"step {step_name}, {label}: the sheet enters {entered!r} {unit or ''}, "
                                f"a record holds {held!r} {held_unit or ''}")
    matrix = re.search(r"upper triangle \(N\\\*m\)\s+`([^`]+)`", step(text, "joint"))
    entered = [number(v) for v in matrix.group(1).split()] if matrix else None
    if entered != _connector(model)["stiffness"]["upper_triangle"]:
        problems.append(f"step joint, work matrix: the sheet enters {entered}, the fixture holds "
                        f"{_connector(model)['stiffness']['upper_triangle']}")
    sam = re.search(r"(?ms)^- \*\*RS-SAM\.\*\*.*", step(text, "states")).group(0)
    motions = [(dof, number(v), unit) for dof, v, unit in re.findall(r"\b(U[XYZ]) ([−-]?\d+(?:\.\d+)?) (mm)\b", sam)]
    held = [(b["dof"], float(b["value"]["value"]), b["value"]["unit"]) for c in _cases(model)
            for s in c["analysis_state"]["support_states"] for b in s.get("boundary_motion", [])]
    if sorted(motions) != sorted(held):
        problems.append(f"step states, RS-C's boundary motions: the sheet enters {motions}, the fixture holds {held}")
    return problems


def test_each_number_in_the_steps_is_its_record_s_value():
    assert number_problems(RUN_SHEET, MODEL) == []


def geometry_problems(text: str, model: dict) -> list[str]:
    """The rows table walked as the sheet builds it: each row's node lands at its fixture position (or its
    bend corner's), and each row's pipe takes the section in force on that row."""
    problems = []
    nodes = {n["id"]: n["position"] for n in model["nodes"]}
    corners = {b["corner"]["node_id_while_drafting"]: b["corner"]["position"] for b in AUTHORING["bends"]}
    pipes = by_id(model, "pipe_segments")
    at, section, orientation, placed = (0.0, 0.0, 0.0), None, None, {}
    for row in table(step(text, "rows"), "#", having="dX"):
        node_id = row["Node id"].split(" ")[0]
        if row["Section"]:
            section = row["Section"].strip("`")
        if row["Orientation"]:
            orientation = [number(v) for v in row["Orientation"].split()]
        if row["Kind"] == "F":
            at = placed.get(row["Node"].strip("`"), at)
            continue
        if row["Kind"] == "S":
            continue
        at = tuple(a + (number(row[c]) if row[c] else 0.0) for a, c in zip(at, ("dX", "dY", "dZ")))
        identity = f"node:{node_id}"
        placed[identity] = at
        want = corners.get(identity) or nodes.get(identity)
        if want is None or not all(abs(a - float(want[k])) <= 1e-9 for a, k in zip(at, "xyz")):
            problems.append(f"step rows, row {row['#']}: {identity} lands at {at}, the fixture holds {want}")
        pipe_id = row["Pipe id"].split(" ")[0]
        held_y = pipes.get(f"pipe:{pipe_id}", {}).get("y_reference", {})
        if pipe_id and [float(held_y.get(k, "nan")) for k in "xyz"] != orientation:
            problems.append(f"step rows, row {row['#']}: pipe:{pipe_id} takes orientation {orientation}, "
                            f"the fixture holds {held_y}")
        if pipe_id and pipes.get(f"pipe:{pipe_id}", {}).get("section_ref") != section:
            problems.append(f"step rows, row {row['#']}: pipe:{pipe_id} takes {section}, the fixture holds "
                            f"{pipes.get(f'pipe:{pipe_id}', {}).get('section_ref')}")
    return problems


def test_each_row_lands_its_node_and_sections_its_pipe_as_the_fixture_does():
    assert geometry_problems(RUN_SHEET, MODEL) == []


def spec_table_problems(spec: str, model: dict) -> list[str]:
    """The specification's node, member, section and support tables against the fixture, for every entity
    the fixture holds (a pending entity keeps its row and is not in the fixture)."""
    problems = []
    nodes = by_id(model, "nodes")
    for row in table(spec, "Node"):
        node = nodes.get(f"node:{row['Node']}")
        coordinates = [number(v) for v in row["X, Y, Z (m)"].split(",")]
        if node is None or [float(node["position"][k]) for k in "xyz"] != coordinates \
                or node["label"] != f"{row['Node']} {row['Role']}":
            problems.append(f"specification §3.1, {row['Node']}: states {coordinates} {row['Role']!r}, "
                            f"the fixture holds {node and (node['position'], node['label'])}")
    pipes = by_id(model, "pipe_segments")
    for row in table(spec, "Member"):
        start, end = (f"node:RS-{t.strip()}" for t in row["From → to"].split("→"))
        pipe = pipes.get(f"pipe:{row['Member']}", {})
        if (pipe.get("from"), pipe.get("to"), pipe.get("section_ref")) != (start, end, f"section:RS-{row['Section']}"):
            problems.append(f"specification §3.2, {row['Member']}: states {start} → {end} {row['Section']}, the fixture "
                            f"holds {pipe.get('from')} → {pipe.get('to')} {pipe.get('section_ref')}")
    for row in table(spec, "Section"):
        od, wall = (number(v) for v in row["OD × wall (mm)"].split("×"))
        insulation, insulation_density = (number(v) for v in row["Insulation (mm @ kg/m³)"].split("@"))
        stated = [od, wall, number(row["Mill tolerance (mm)"]), number(row["Corrosion allowance (mm)"]),
                  insulation, insulation_density, number(row["Contents (kg/m³)"])]
        for pipe in (p for p in model["pipe_segments"] if p["section_ref"] == f"section:{row['Section']}"):
            held = [float(pipe["section"][k]["value"]) for k in ("outside_diameter", "wall_thickness", "mill_tolerance",
                    "corrosion_allowance", "insulation_thickness", "insulation_density", "contents_density")]
            if held != stated:
                problems.append(f"specification §4.2, {row['Section']}: states {stated}, {pipe['id']} holds {held}")
    components = by_id(model, "components")
    for row in table(spec, "Component"):
        component = components.get(f"component:{row['Component']}")
        if component is None:
            continue  # pending
        data, modifiers, geometry = row["Data"], component.get("modifiers", {}), component.get("geometry", {})
        stated, held = {}, {}
        bend = re.search(r"R ([\d.]+) m, 90°; k ([\d.]+); SIF (?:in-plane )?([\d.]+)(?:, out-of-plane | / )([\d.]+)", data)
        if component["kind"] == "bend":
            stated = dict(zip(("radius", "k", "in", "out"), map(number, bend.groups()))) if bend else {}
            held = {"radius": geometry["bend_radius"]["value"], "k": modifiers["flexibility_factor_user_value"]["value"],
                    "in": modifiers["sif_in_plane_user_value"]["value"], "out": modifiers["sif_out_of_plane_user_value"]["value"]}
        elif component["kind"] == "tee":
            stated = {k: number(v) for k, v in re.findall(r"`(branch_(?:header|branch)_sif_user_value)` ([\d.]+)", data)}
            held = {k: modifiers[k]["value"] for k in ("branch_header_sif_user_value", "branch_branch_sif_user_value")}
        elif component["kind"] in ("valve", "flange"):
            weight, scale = re.search(r"weight (\d+) N", data), re.search(r"stiffness scale ([\d.]+)", data)
            stated = {"weight": number(weight.group(1)) if weight else None, "scale": number(scale.group(1)) if scale else None}
            held = {"weight": geometry["weight"]["value"], "scale": modifiers["stiffness_scaling_user_value"]["value"]}
        if held and {k: float(v) for k, v in held.items()} != stated:
            problems.append(f"specification §5, {row['Component']}: states {stated}, the fixture holds {held}")
    labels = {c["label"] for c in model["load_cases"]} | {c["label"] for c in model["combinations"]}
    for quoted in re.findall(r"`(RS-[A-Za-z-]+ [^`]*: [^`]*)`", spec):
        if quoted not in labels:
            problems.append(f"specification, label {quoted!r}: no case or combination of the fixture has it")
    numbers = {
        "effective area": [float(_connector(model)["pressure_model"]["effective_area"]["value"])],
        "Operating pressure": [float(r["pressure"]["value"]) for c in _cases(model) for r in c["pressure_regions"]],
        "installation temperature": [float(r["basis"]["installation_temperature"]["value"])
                                     for c in model["reference_configurations"] for r in c["member_references"]],
        "operating temperature": [float(s["operating_temperature"]["value"]) for c in _cases(model)
                                  for s in c["analysis_state"]["element_states"] if "operating_temperature" in s],
    }
    for label, held in numbers.items():
        found = re.search(label_pattern(label).removesuffix(r"(?:\*\*)?\s+") + NUMBER, spec)
        stated = number(found.group(1)) if found else None
        if stated is None or any(not same(stated, h) for h in held):
            problems.append(f"specification, {label}: states {stated}, the fixture holds {sorted(set(held))}")
    supports = by_id(model, "supports")
    for row in table(spec, "Support"):
        support = supports.get(f"support:{row['Support']}")
        if support is None or f"support:{row['Support']}" in LEDGER_IDS:
            continue  # pending, or an interim record the ledger states
        data = row["Data"]
        restraints = ALL_SIX if data.startswith("all six DOF") else [] if data.startswith("no restraint") else \
            re.findall(r"\b[UR][XYZ]\b", data.split(";")[0])
        stated = (f"node:{row['Node']}", row["Family"].strip("`"), sorted(restraints))
        held = (support["node"], support["family"], sorted(support["restraints"]))
        if stated != held:
            problems.append(f"specification §6, {row['Support']}: states {stated}, the fixture holds {held}")
    return problems


def test_the_specification_s_tables_are_the_fixture_s():
    assert spec_table_problems(SPEC, MODEL) == []


def test_each_tabled_value_is_its_record_s_value():
    assert value_problems(RUN_SHEET, MODEL) == []


def test_each_typed_label_is_its_record_s_label():
    assert label_problems(RUN_SHEET, MODEL) == []


# -- (4) citations --------------------------------------------------------------------------------------------------
# The repository roots the check follows. A citation outside them (a directory the product test job does not
# check out, such as the declarative records) is reported without being resolved or read: cite a deliverable by
# its id instead.
FOLLOWED = ("apps", "core", "docs", "fixtures", "schemas", "tests", "tools", "validation")
LINK = re.compile(r"\]\(([^)\s]+)\)")
PLAIN_PATH = re.compile(r"(?<![\w./@\\-])((?:[A-Za-z_][\w.-]*/)+[\w.-]*)")
FILE_NAME = re.compile(r"(?<![\w./-])[\w-]+\.(?:json|md|py|rs|ts|tsx|mjs|yaml|yml)(?![\w/])")


def path_shaped(token: str) -> bool:
    """A repository path rather than a unit or a slash in prose: two segments or more, ending in a file
    name with an extension or in a slash."""
    segments = token.rstrip("/").split("/")
    return len(segments) >= 2 and (token.endswith("/") or re.search(r"\.[A-Za-z]\w*$", segments[-1]) is not None)


def cited_paths(path: Path, text: str) -> list[tuple[str, str]]:
    """(the citation as written, its repository-relative path), from links resolved against the record's
    directory and from plain or backticked repository paths, without touching the file system."""
    record_dir = path.parent.relative_to(ROOT).as_posix()
    cited = []
    for target in LINK.findall(text):
        if re.match(r"[a-z]+:", target) or target.startswith("#"):
            continue
        cited.append((target, posixpath.normpath(posixpath.join(record_dir, target.split("#", 1)[0]))))
    for token in PLAIN_PATH.findall(re.sub(r"\]\([^)]*\)", "]", text)):
        token = token.rstrip(".,;:")
        if token.split("/")[0] in FOLLOWED or path_shaped(token):
            cited.append((token, posixpath.normpath(token)))
    return cited


def citation_problems(path: Path, text: str) -> list[str]:
    cited = cited_paths(path, text)
    problems = [] if cited else ["cites no path"]
    followed = []
    for target, relative in cited:
        if relative.startswith("../") or relative == "..":
            problems.append(f"{target}: outside the repository")
        elif relative.split("/")[0] not in FOLLOWED:
            problems.append(f"{target}: not followed (outside {', '.join(FOLLOWED)}); cite the deliverable by its id")
        else:
            followed.append((target, ROOT / relative))
    problems += [f"{target}: does not exist" for target, location in followed if not location.exists()]
    # A bare file name resolves beside the record or in a followed directory the record cites.
    directories = [path.parent] + [location for _target, location in followed if location.is_dir()]
    for name in sorted(set(FILE_NAME.findall(re.sub(r"\]\([^)]*\)", "]", text)))):
        if not any((directory / name).exists() for directory in directories):
            problems.append(f"{name}: no such file beside the record or in a directory it cites")
    return problems


# A section or a `Work/` file of another deliverable: the product test job cannot check one (the
# declarative records are not checked out there), and such sections have been removed before. Cite the
# deliverable, or its requirement, criterion or check, by id.
CROSS_SECTION = re.compile(r"DEL-\d{2}-\d{2}\b[^.\n]{0,60}?(?:§\s*\d+(?:\.\d+)*|`?Work/[\w./-]*\w)")


def cross_citation_problems(text: str) -> list[str]:
    return [f"{match.group(0)!r}: cites another deliverable's section or Work file; cite it by id"
            for match in CROSS_SECTION.finditer(text)]


@pytest.mark.parametrize("record", sorted(RECORDS))
def test_no_record_cites_another_deliverable_s_section(record):
    assert cross_citation_problems(RECORDS[record][1]) == []


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
    "Realized bends with user k and SIFs":  # one bend only: every bend must hold its k and both SIFs
        lambda m: by_id(m, "components")["component:RS-B2"]["modifiers"].pop("sif_in_plane_user_value"),
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


def reports(problems: list[str], *fragments: str) -> bool:
    """Whether one diagnostic carries every fragment: the control's own diagnostic, not any problem."""
    return any(all(fragment in problem for fragment in fragments) for problem in problems)


@pytest.mark.parametrize("family", sorted(FAMILY_PRESENT))
def test_a_change_of_the_fixture_s_coverage_of_each_family_is_caught(family):
    assert coverage_problems(SPEC, MODEL) == []
    model = copy.deepcopy(MODEL)
    FLIP[family](model)
    assert FAMILY_PRESENT[family](model) != FAMILY_PRESENT[family](MODEL), f"{family}: the control does not flip it"
    stated = "in tranche 1" if spec_ledger(SPEC)[family]["in_tranche_1"] else "pending"
    assert reports(coverage_problems(SPEC, model), f"{family}: the ledger says {stated}")


def test_a_changed_sub_feature_or_interim_record_is_caught():
    for family, (name, _holds) in SUB_FEATURES.items():
        model = copy.deepcopy(MODEL)
        _combination(model, "combination:RS-EXP")[name] = {}
        assert reports(coverage_problems(SPEC, model), f"{family}: the ledger says `{name}` is pending, the fixture holds")
        state = spec_ledger(SPEC)[family]["state"]
        recorded = SPEC.replace(state, state.replace(f"`{name}` pending", f"`{name}` recorded"), 1)
        assert recorded != SPEC
        assert reports(coverage_problems(recorded, MODEL), f"{family}: the ledger says `{name}` is held")
    for family, (subject, _holds) in INTERIM.items():
        model = copy.deepcopy(MODEL)
        if subject == "RS-P150":
            by_id(model, "pipe_segments")["pipe:RS-P150"]["section_ref"] = "section:RS-S220"
        else:
            by_id(model, "supports")[f"support:{subject}"]["restraints"] = ["UX"]
        assert reports(coverage_problems(SPEC, model), f"{family}: the ledger says {subject} is in tranche 1"), family


def test_a_ledger_row_that_disagrees_with_the_generator_is_caught():
    assert ledger_problems(SPEC) == [] and coverage_problems(SPEC, MODEL) == []
    ledger = spec_ledger(SPEC)
    family, row = next((f, r) for f, r in ledger.items() if not r["in_tranche_1"] and r["entities"])
    line = next(l for l in SPEC.splitlines() if l.startswith(f"| {family} |"))
    flipped = line.replace(f"| {row['state']} |", f"| in tranche 1; {row['state']} |", 1)
    problems = ledger_problems(SPEC.replace(line, flipped, 1))
    assert reports(problems, f"{family}: the specification says {{'in_tranche_1': True", "the generator's LEDGER")
    assert reports(problems, f"{family}: the specification says {{'in_tranche_1': True", "authoring.json")
    dropped = line.replace(f"`{row['entities'][0]}`", "`support:RS-A`", 1)
    assert reports(ledger_problems(SPEC.replace(line, dropped, 1)), f"{family}: the specification says", "support:RS-A")
    renamed = SPEC.replace(f"| {family} |", f"| {family} (renamed) |", 1)
    assert reports(ledger_problems(renamed), f"{family}: the specification says None")
    assert reports(ledger_problems(renamed), f"{family} (renamed): the specification says")
    assert reports(coverage_problems(renamed, MODEL), f"{family} (renamed): no fixture predicate")


def test_a_ledger_identity_the_fixture_lacks_or_holds_wrongly_is_caught():
    model = copy.deepcopy(MODEL)
    model["components"] = [c for c in model["components"] if c["id"] != "component:RS-T1"]
    assert reports(coverage_problems(SPEC, model), "Tee RS-T1 with user SIFs: in tranche 1, but the fixture has no component:RS-T1")
    model = copy.deepcopy(MODEL)
    model["supports"].append({"id": "support:RS-CE1", "family": "anchor"})
    assert reports(coverage_problems(SPEC, model), "Constant-effort support RS-CE1: pending, but the fixture holds support:RS-CE1")


def test_a_missing_or_unknown_identity_is_caught():
    assert all(missing_identities(r, t, MODEL) == [] and unknown_identities(t, MODEL) == [] for r, (_p, t) in RECORDS.items())
    for record, (_path, text) in RECORDS.items():
        dropped = sorted(fixture_identities(MODEL))[0]
        without = re.sub(rf"(?<![\w-]){re.escape(tag(dropped))}(?![\w-])", "", text)
        assert missing_identities(record, without, MODEL) == [dropped], record
        assert unknown_identities(text + "\n| RS-P999 | 999 → 998 | S270 | |\n", MODEL) == ["RS-P999"], record
        assert unknown_identities(text + "\n`component:RS-ZZ9`\n", MODEL) == ["component:RS-ZZ9"], record
        assert unknown_identities(text + "\n`pipe:RS-P010_old`\n", MODEL) == ["pipe:RS-P010_old"], record
        assert unknown_identities(text + "\n| RS-P010_old |\n", MODEL) == ["RS-P010_old"], record
    without_region = re.sub(r"(?<![\w-])RG-4(?![\w-])", "", SPEC)
    assert missing_identities("SPECIFICATION.md", without_region, MODEL) == [
        "region:RS-OPE-RG-4", "region:RS-SAM-RG-4", "region:RS-SUS-RG-4"]


def _primitive(model, case_index, contains):
    return next(p for p in _cases(model)[case_index]["primitive_loads"] if contains in p["id"])


def test_a_wrong_primitive_identity_count_component_or_target_is_caught():
    assert primitive_identity_problems(MODEL, SPEC) == []
    moment = "load:self-weight:11:load:RS-SUS:component:15:component:RS-F1:moment:y"
    line = "load:self-weight:11:load:RS-SUS:12:pipe:RS-P010"
    controls = {
        "a renamed line load": (lambda m: _primitive(m, 0, line).__setitem__("id", line + "-x"),
                                (f"{line}-x: names 12:pipe:RS-P010-x",)),
        "a wrong count in a moment": (lambda m: _primitive(m, 0, moment).__setitem__("id", moment.replace("component:15:", "component:999:")),
                                      ("counts 999 characters for component:RS-F1, which has 15",)),
        "a moment naming another component": (lambda m: _primitive(m, 0, moment).__setitem__("id", moment.replace("RS-F1", "RS-T1")),
                                              ("names component:RS-T1, which is not a rigid component",)),
        "a moment at the wrong node": (lambda m: _primitive(m, 0, moment)["target"].__setitem__("node", "node:RS-010"),
                                       (f"{moment}: applied at", "not at component:RS-F1's node node:RS-030")),
        "one moment too many": (lambda m: _cases(m)[0]["primitive_loads"].append({**_primitive(m, 0, moment), "id": moment[:-1] + "x"}),
                                ("load:RS-SUS: component:RS-F1 has ['force', 'moment', 'moment']",)),
        "one moment too few": (lambda m: _cases(m)[0]["primitive_loads"].remove(_primitive(m, 0, "RS-V1:moment")),
                               ("load:RS-SUS: component:RS-V1 has ['force'], not one force and one moment",)),
        "a duplicated line load": (lambda m: _cases(m)[0]["primitive_loads"].append(copy.deepcopy(_primitive(m, 0, line))),
                                   (f"load:RS-SUS: primitive id {line} appears 2 times",)),
    }
    for name, (change, fragments) in controls.items():
        model = copy.deepcopy(MODEL)
        change(model)
        assert reports(primitive_identity_problems(model, SPEC), *fragments), name
    model = copy.deepcopy(MODEL)
    _cases(model)[0]["primitive_loads"].append(copy.deepcopy(_primitive(model, 0, line)))
    assert reports(primitive_identity_problems(model, SPEC), "load:RS-SUS: line loads differ", "repeated or unexpected ['pipe:RS-P010']")
    assert reports(primitive_identity_problems(MODEL, SPEC.replace(line, line.replace("RS-P010", "RS-P999"))),
                   "example primitive identity is not one the fixture holds")


def test_a_stale_source_or_a_changed_fixture_field_is_caught():
    assert field_problems(RUN_SHEET, MODEL) == []
    texts = pasted_texts(RUN_SHEET)
    stale = field_problems(RUN_SHEET.replace(f"`{texts['P']}`", f"`{texts['P']} (stale)`", 1), MODEL)
    assert reports(stale, "step material, Constitutive provenance", f"the sheet enters '{texts['P']} (stale)'")
    # A literal entered in place of each bound entry, one at a time, reported for that field.
    for step_name, label, _values in BINDINGS:
        body = step(RUN_SHEET, step_name)
        literal = re.sub("(" + label_pattern(label) + r")(?:⟨\w+⟩|`[^`]+`)", r"\1`a stale text`", body, count=1)
        assert literal != body, (step_name, label)
        assert reports(field_problems(RUN_SHEET.replace(body, literal, 1), MODEL),
                       f"step {step_name}, {label}:", "the sheet enters 'a stale text'"), (step_name, label)
    # Each fixture field the run sheet writes, altered in one record, reported for that field and record.
    rigid = [c["id"] for c in _rigid(MODEL)]
    fixture_changes = {
        "the hanger source": (lambda m: _supports(m, "variable_spring_hanger")[0]["hanger"].__setitem__("source_reference", "x"),
                              "step spring, Hanger source reference: a record holds 'x'"),
        "the tee's reinforcement reference": (lambda m: _components(m, "tee")[0]["geometry"].__setitem__(
            "branch_reinforcement_reference", "x"), "step tee, Reinforcement reference: a record holds 'x'"),
        "the tee's connection type": (lambda m: _components(m, "tee")[0]["geometry"].__setitem__("branch_connection_type", "x"),
                                      "step tee, Connection type: a record holds 'x'"),
        "the first rigid component's End A reference": (lambda m: _rigid(m)[0]["geometry"].__setitem__(
            "connection_end_a_reference", "x"), f"step rows, End A reference of {rigid[0]}"),
        "the last rigid component's End B reference": (lambda m: _rigid(m)[-1]["geometry"].__setitem__(
            "connection_end_b_reference", "x"), f"step rows, End B reference of {rigid[-1]}"),
        "RS-B2's k source": (lambda m: by_id(m, "components")["component:RS-B2"]["modifiers"].__setitem__("source_reference", "x"),
                             "step rows, k source of component:RS-B2"),
        "RS-SAM's self-weight provenance": (lambda m: _case(m, "load:RS-SAM").__setitem__("provenance", "x"),
                                            "step weight, provenance of load:RS-SAM"),
        "the joint stiffness's measurement restraints, unlisted": (
            lambda m: _connector(m)["stiffness"]["provenance"].__setitem__("measurement_restraints", "an unlisted text"),
            "the joint's stiffness provenance measurement_restraints 'an unlisted text'"),
        "the connector's measurement restraints set to a value listed under the stiffness's title": (
            lambda m: _connector(m)["provenance"].__setitem__("measurement_restraints", "invented diagonal work matrix; no measurement"),
            "step joint, Measurement restraints: a record holds 'invented diagonal work matrix; no measurement'"),
    }
    for name, (change, fragment) in fixture_changes.items():
        model = copy.deepcopy(MODEL)
        change(model)
        assert reports(field_problems(RUN_SHEET, model), fragment), name
    # The joint stiffness's two listed values swapped in the fixture: both fields reported.
    model = copy.deepcopy(MODEL)
    stiffness = _connector(model)["stiffness"]["provenance"]
    stiffness["measurement_restraints"], stiffness["basis_transform_reference"] = (
        stiffness["basis_transform_reference"], stiffness["measurement_restraints"])
    swapped_stiffness = field_problems(RUN_SHEET, model)
    assert reports(swapped_stiffness, "stiffness provenance measurement_restraints", "Joint stiffness measurement restraints")
    assert reports(swapped_stiffness, "stiffness provenance basis_transform_reference", "Joint stiffness basis transform reference")
    # Two components' End A references swapped in the sheet's table: both records reported.
    a = [c["geometry"]["connection_end_a_reference"] for c in _rigid(MODEL)[:2]]
    swapped = RUN_SHEET.replace(f"`{a[0]}`", "@@", 1).replace(f"`{a[1]}`", f"`{a[0]}`", 1).replace("@@", f"`{a[1]}`", 1)
    assert swapped != RUN_SHEET
    assert reports(field_problems(swapped, MODEL), f"End A reference of {rigid[0]}", repr(a[1]))
    assert reports(field_problems(swapped, MODEL), f"End A reference of {rigid[1]}", repr(a[0]))
    # RS-SUS's two thermal provenance assignments swapped, in the sheet: each member reported, per case.
    bullet = re.search(r"(?ms)^- \*\*RS-SUS\.\*\*.*?(?=^- \*\*)", step(RUN_SHEET, "states")).group(0)
    values = re.findall(r"⟨P⟩(`[^`]*`)", bullet)
    assert len(set(values)) == 2
    thermal_swap = bullet.replace(values[0], "@@", 1).replace(values[1], values[0], 1).replace("@@", values[1], 1)
    problems = field_problems(RUN_SHEET.replace(bullet, thermal_swap, 1), MODEL)
    assert reports(problems, "step states, load:RS-SUS pipe:RS-P010: thermal provenance")
    assert reports(problems, "step states, load:RS-SUS pipe:RS-P180: thermal provenance")
    # The same swap in the fixture alone.
    model = copy.deepcopy(MODEL)
    rows = {r["pipe_ref"]: r for r in _case(model, "load:RS-SUS")["analysis_state"]["element_states"]}
    rows["pipe:RS-P010"]["thermal_state"]["provenance"], rows["pipe:RS-P180"]["thermal_state"]["provenance"] = (
        rows["pipe:RS-P180"]["thermal_state"]["provenance"], rows["pipe:RS-P010"]["thermal_state"]["provenance"])
    problems = field_problems(RUN_SHEET, model)
    assert reports(problems, "step states, load:RS-SUS pipe:RS-P010: thermal provenance")
    assert reports(problems, "step states, load:RS-SUS pipe:RS-P180: thermal provenance")


def test_a_wrong_typed_label_is_caught():
    assert label_problems(RUN_SHEET, MODEL) == []
    for old, new, fragment in (
            ("`RS-010 Anchor A (nozzle A)`", "`RS-010 Anchor A (nozzle B)`", "label of row 1 (node:RS-010)"),
            ("| `support:RS-G2` | Guide G2 |", "| `support:RS-G2` | Guide G9 |", "label of support:RS-G2"),
            ("| `component:RS-V1` | Valve V1 |", "| `component:RS-V1` | Valve V2 |", "label of component:RS-V1"),
            ("`RS-OPE operating: W + P + T", "`RS-OPE operating: W + P", "label of load:RS-OPE"),
            ("`RS-dSAM anchor movement term: RS-SAM - RS-SUS`", "`RS-dSAM anchor movement: RS-SAM - RS-SUS`",
             "label of combination:RS-dSAM")):
        changed = RUN_SHEET.replace(old, new, 1)
        assert changed != RUN_SHEET, old
        assert reports(label_problems(changed, MODEL), fragment), old


def test_a_dead_link_a_dead_file_name_an_unfollowed_path_and_an_outside_path_are_caught():
    for record, (path, text) in RECORDS.items():
        assert citation_problems(path, text) == [], record
        assert reports(citation_problems(path, text + "\n[gone](no-such-file.md)\n"), "no-such-file.md: does not exist"), record
        assert reports(citation_problems(path, text + "\nsee no_such_file.json\n"),
                       "no_such_file.json: no such file beside the record"), record
        plain = "fixtures/reference_system/no_such_file.json"
        assert reports(citation_problems(path, text + f"\nsee {plain} here\n"), f"{plain}: does not exist"), record
        # Any root outside FOLLOWED takes this branch, the declarative records' among them. The guard forbids a
        # literal naming that root from reaching this function, which reads files, so another root stands in.
        unfollowed = "specs/PKG-09/DEL-09-01/ScopeOfWork.md"
        assert reports(citation_problems(path, text + f"\n`{unfollowed}`\n"), f"{unfollowed}: not followed"), record
        assert reports(citation_problems(path, text + f"\nsee {unfollowed}.\n"), f"{unfollowed}: not followed"), record
        assert reports(citation_problems(path, text + "\n[outside](../../../../../outside.md)\n"),
                       "../../../../../outside.md: outside the repository"), record
    assert citation_problems(SPEC_PATH, "no citation at all") == ["cites no path"]


def test_a_field_the_sheet_stops_naming_or_an_undefined_pasted_text_is_refused():
    body = step(RUN_SHEET, "spring")
    unnamed = RUN_SHEET.replace(body, body.replace("Hanger source reference", "Hanger source", 1), 1)
    with pytest.raises(AssertionError, match="Hanger source reference"):
        field_problems(unnamed, MODEL)
    with pytest.raises(AssertionError, match="defines"):
        pasted_texts(RUN_SHEET.replace("**⟨P⟩** =", "**⟨Q⟩** =", 1))


def test_a_citation_of_another_deliverable_s_section_is_caught():
    for record, (_path, text) in RECORDS.items():
        assert cross_citation_problems(text) == [], record
        for cited in ("DEL-17-04 §1.4", "DEL-17-04 import Design §20.4", "DEL-07-02 `Work/run-sheet-entry.md`"):
            assert reports(cross_citation_problems(text + f"\nas {cited} states.\n"), cited.rstrip("`"),
                           "cite it by id"), (record, cited)
        assert cross_citation_problems(text + "\nas DEL-17-04 AC-003 states.\n") == [], record


def test_a_wrong_tabled_value_is_caught():
    assert value_problems(RUN_SHEET, MODEL) == []
    for old, new, fragment in (
            ("| `section:RS-S220` | RS-S220 | 220 | 8.0 |", "| `section:RS-S220` | RS-S220 | 220 | 8.5 |",
             "wall thickness of `section:RS-S220`"),
            ("| `point:RS-M1-T200` | 200 degC | 189 GPa |", "| `point:RS-M1-T200` | 200 degC | 198 GPa |",
             "elastic modulus of `point:RS-M1-T200`"),
            ("| 4 | `component:RS-V1` | Valve V1 | 0.6 | 2600 |", "| 4 | `component:RS-V1` | Valve V1 | 0.6 | 2500 |",
             "weight of `component:RS-V1`"),
            ("| −0.3, 0, 0.3 |", "| −0.3, 0, 0.35 |", "COG of `component:RS-V1`"),
            ("| 18 | G | UX UZ |", "| 18 | G | UX UY |", "restraints of `support:RS-G2`"),
            ("| 6 | L | UX |", "| 6 | V | UX |", "family of `support:RS-LS1`"),
            ("| (inherits 0.4) | 4.2 |", "| (inherits 0.5) | 4.2 |", "Bend R of `component:RS-B2`"),
            ("| 0.25 | 3.6 | ⟨S5⟩ | 2.3 | 1.9 |", "| 0.25 | 3.6 | ⟨S5⟩ | 2.3 | 1.8 |", "out-of-plane SIF of `component:RS-B3`"),
            ("| `region:RS-SUS-RG-3` | RS-P130, RS-P140 |", "| `region:RS-SUS-RG-3` | RS-P130 |", "member pipes of `region:RS-SUS-RG-3`"),
            ("| RS-160, RS-210 |", "| RS-210, RS-160 |", "terminals of `region:RS-SUS-RG-4`"),
            ("| `combination:RS-dSAM` | `RS-dSAM anchor movement term: RS-SAM - RS-SUS` | `load:RS-SAM` |",
             "| `combination:RS-dSAM` | `RS-dSAM anchor movement term: RS-SAM - RS-SUS` | `load:RS-OPE` |",
             "minuend of `combination:RS-dSAM`")):
        changed = RUN_SHEET.replace(old, new, 1)
        assert changed != RUN_SHEET, old
        assert reports(value_problems(changed, MODEL), fragment), old


def test_a_wrong_number_row_or_specification_value_is_caught():
    assert number_problems(RUN_SHEET, MODEL) == [] and geometry_problems(RUN_SHEET, MODEL) == []
    assert spec_table_problems(SPEC, MODEL) == []
    sheet = {
        "hot load 7400 N": ("hot load 7500 N", number_problems, "step spring, hot load: the sheet enters 7500.0 N"),
        "7400 N, travel": ("7400 kN, travel", number_problems, "step spring, hot load: the sheet enters 7400.0 kN"),
        "pressure 1600 kPa": ("pressure 1500 kPa", number_problems, "step regions, pressure: the sheet enters 1500.0"),
        "Rod stiffness 1.2e8 N/m": ("Rod stiffness of each rod", number_problems,
                                    "step joint, Rod stiffness: the sheet no longer gives a number"),
        "150000 0 0 0 0 0 400000": ("160000 0 0 0 0 0 400000", number_problems, "step joint, work matrix"),
        "UX 15 mm and UY -10 mm": ("UX 10 mm and UY -10 mm", number_problems, "step states, RS-C's boundary motions"),
        "| 18 | 170 | | 2.6 |": ("| 18 | 170 | | 2.5 |", geometry_problems, "step rows, row 18: node:RS-170 lands at"),
        "| 1 0 0 | | RS-080 |": ("| 0 1 0 | | RS-080 |", geometry_problems, "step rows, row 9: pipe:RS-P070 takes orientation"),
        "| section:RS-S220 |": ("| section:RS-S270 |", geometry_problems,
                                "step rows, row 18: pipe:RS-P160 takes section:RS-S270"),
        "| F | node:RS-130 |": ("| F | node:RS-140 |", geometry_problems, "step rows, row 24: node:RS-215 lands at"),
    }
    for old, (new, predicate, fragment) in sheet.items():
        changed = RUN_SHEET.replace(old, new, 1)
        assert changed != RUN_SHEET, old
        assert reports(predicate(changed, MODEL), fragment), old
    spec = {
        "| RS-170 | 6.000, 23.000, 6.975 | Guide G2 |": ("| RS-170 | 6.000, 23.500, 6.975 | Guide G2 |", "specification §3.1, RS-170"),
        "| RS-P160 | 160 → 170 | S220 |": ("| RS-P160 | 160 → 170 | S270 |", "specification §3.2, RS-P160"),
        "| RS-S170 | 170 × 7.0 | 0.8 | 1.5 | 50 @ 140 | 820 |": (
            "| RS-S170 | 170 × 7.0 | 0.8 | 1.5 | 50 @ 140 | 900 |", "specification §4.2, RS-S170"),
        "| RS-G2 | RS-170 | `guide` | UX, UZ |": ("| RS-G2 | RS-170 | `guide` | UX, UY |", "specification §6, RS-G2"),
        "R 0.250 m, 90°; k 3.6": ("R 0.250 m, 90°; k 3.7", "specification §5, RS-B3"),
        "weight 2600 N": ("weight 2500 N", "specification §5, RS-V1"),
        "`RS-OPE operating: W + P + T (210 degC, modulus at 210 degC)`": (
            "`RS-OPE operating: W + P + T (200 degC, modulus at 200 degC)`", "specification, label 'RS-OPE operating"),
        "effective area 0.0490 m²": ("effective area 0.0500 m²", "specification, effective area: states 0.05"),
        "Operating pressure 1600 kPa": ("Operating pressure 1500 kPa", "specification, Operating pressure: states 1500.0"),
    }
    for old, (new, fragment) in spec.items():
        changed = SPEC.replace(old, new, 1)
        assert changed != SPEC, old
        assert reports(spec_table_problems(changed, MODEL), fragment), old
