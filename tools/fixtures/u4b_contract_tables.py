"""T4-U4b result-semantics tables and schemas (DEL-04-07 Design ``u4b-u5b-implementation.md`` §2.2-§2.8, §5.1).

Applies the U4b contract to the committed tables and result schemas, idempotently, on top of whatever else they
already carry (for example another unit's connector rows):

* tables (``fixtures/results/semantic_contract_v0_3_*.json``):
  - pressure-2 and preview-physics-2 gain the U4b signatures: sigma+/sigma-, the transverse shear maximum (straight
    and arc tokens), the directional intensified station and maximum rows and, on pressure-2, the toroidal hoop and
    radial rows. Ids follow the tables' identity convention: one number per (kind, unit, component) after the
    table's highest; a pressure-2 stress signature adds ``-<token>-<basis>`` (its source_basis, hyphenated); a
    preview arc variant is ``preview-physics-arc-<number>``. Existing U4b rows are removed first, so a re-run on a
    table that gained other rows renumbers U4b after them.
  - every row of pressure-2, preview-physics-2 and the two inheriting tables carries ``combination_algebra`` (§2.7);
  - the canonical vocabularies gain the U4b components and basis tokens; the limitation texts take §2.5/§2.8;
  - the inheriting tables re-inherit their base by hash.
* schemas: ``results.v0.3`` (metadata enums, the preview limitation consts of the preview-physics-2 and
  preview-physics-retained-1 branches, preview extrema and intensified records), ``stress_neutral_export.v0.3``
  (preview extrema and intensified records), ``results_pressure_2.v0.3`` (region statement and bend radius, extrema
  and intensified records, enums) and ``component`` (the directional pair in the field catalog). A maximum record
  takes the straight or the arc token triple together (a ``oneOf`` of closed variants).

Consumers that pin the resulting bytes by hash are re-pinned separately from the printed hashes.

From ``projects/chirality-piping``::

    python3 tools/fixtures/u4b_contract_tables.py --write   # rewrite the files, print their sha256
    python3 tools/fixtures/u4b_contract_tables.py --check   # exit 1 unless every file is already current
"""
from __future__ import annotations

import argparse
import collections
import copy
import hashlib
import json
import re
import sys
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[2]
TABLE = "fixtures/results/semantic_contract_v0_3_%s.json"

EXT = "section_extremes_from_published_stress_rows_v1"
SVS = "saint_venant_annulus_flexure_v1"
SVA = "local_straight_annulus_on_arc"
HOOP = "bore_load_transferred_toroidal_membrane_v1"
DIR = "user_directional_sif_times_tangent_frame_bending_stress_v1"
BASES = ("nominal_section", "reduced_section")
SUFFIXES = ["; section_basis=" + b for b in BASES]
PREVIEW_COMPONENTS = ["sigma_plus", "sigma_minus", "tau_v_max", "intensified_bending", "intensified_bending_maximum"]
EXACT_COMPONENTS = PREVIEW_COMPONENTS + ["toroidal_intrados_hoop_stress", "toroidal_crown_hoop_stress",
                                        "toroidal_extrados_hoop_stress", "toroidal_inner_radial_stress",
                                        "toroidal_outer_radial_stress"]
PREVIEW_BASES = [EXT, SVS, SVA, DIR]
EXACT_BASES = [t + s for t in [EXT, SVS, SVA, DIR, HOOP] for s in SUFFIXES]
STRAIGHT = ["piecewise_quadratic_straight_section_statics", "j_side_section_equilibrium_binary64_enclosures",
            "supplied_binary64_polynomial_coefficient_enclosures; solution error is separate"]
ARC = ["arc_closed_form_section_statics", "arc_end_j_force_and_uniform_intensities_binary64_enclosures",
       "published_binary64_end_j_force_and_uniform_intensity_operand_enclosures; solution error is separate"]
RECORD_TOKENS = ["approximation", "coefficient_basis", "enclosure_scope"]
RECORD = ["pipe_id", "result_id", "station_fraction", "span_index", "local_fraction", "value_lower_pa", "value_upper_pa",
          "global_upper_bound_pa", "certified_gap_pa", "subdivisions"] + RECORD_TOKENS
U4B_KINDS = {"element_local_normal_stress_extreme", "element_local_transverse_shear_stress_maximum",
             "curved_bend_directional_intensified_bending_stress_v1",
             "curved_bend_directional_intensified_bending_maximum_v1",
             "pipe_toroidal_membrane_hoop_stress_v1", "pipe_toroidal_radial_surface_stress_v1"}

# §2.7: the algebra classes. Everything not signed or recomputed is never combined.
SIGNED = {
    "global_nodal_displacement_x", "global_nodal_displacement_y", "global_nodal_displacement_z",
    "global_nodal_rotation_x", "global_nodal_rotation_y", "global_nodal_rotation_z",
    "element_local_axial_force", "element_local_shear_force_y", "element_local_shear_force_z",
    "element_local_torsional_moment", "element_local_bending_moment_y", "element_local_bending_moment_z",
    "element_local_axial_normal_stress", "element_local_bending_normal_stress_y", "element_local_bending_normal_stress_z",
    "element_local_torsional_shear_stress", "pipe_section_pressure_hoop_stress", "pipe_section_pressure_longitudinal_stress",
    "pipe_wall_endpoint_action_v2", "pipe_wall_axial_force_v2", "pipe_effective_axial_force_v2", "pipe_axial_membrane_stress_v2",
    "pipe_lame_radial_stress_v2", "pipe_lame_hoop_stress_v2", "pipe_toroidal_membrane_hoop_stress_v1",
    "pipe_toroidal_radial_surface_stress_v1", "support_reaction_component_v2", "constant_effort_support_applied_load",
    "connector_generalized_translation_v1", "connector_generalized_rotation_v1", "connector_generalized_force_v1",
    "connector_generalized_moment_v1", "connector_endpoint_force_v1", "connector_endpoint_moment_v1",
    "connector_pressure_force_v1", "connector_effective_axial_force_v1", "connector_tie_rod_force_v1"}
RECOMPUTED = {"displacement_magnitude", "support_reaction_force_magnitude_v2", "support_reaction_moment_magnitude_v2"}

# §2.5 and §2.8: the limitation texts.
LIM2_OLD = "the straight Lame hoop and radial rows and the governing normal-stress maximum are withheld on arcs with the named reason."
LIM2_NEW = ("on arcs the hoop is the toroidal membrane cut average (bore_load_transferred_toroidal_membrane_v1) and the radial values are -p at the bore and 0 outside, published where R > r_o, "
            "the transverse shear is the straight-annulus Saint-Venant field driven by the tangent-frame shears (local_straight_annulus_on_arc), and the normal-stress maximum is bounded along the arc and enters the per-basis headline; "
            "on realized arcs with directional SIFs, intensified bending is hypot(i_ip*Mz, i_op*My)/Z in the tangent frame, with its along-arc maximum, never combined.")
LIM_EXTREMES = ("On every member the section normal-stress extremes are Nw/As ± hypot(My,Mz)/Z (N/A off the pressure family) at each station and basis, and the transverse shear maximum is the straight-annulus Saint-Venant value with the member's authored nu; "
                "neither is a code or equivalent stress.")
LIM_EXTREMES_AT = 7
PREVIEW_LIMITATIONS = [
    "On every member the section normal-stress extremes are N/A ± hypot(My,Mz)/Z at each station and the transverse shear maximum is the straight-annulus Saint-Venant value, with ν = E/(2G) − 1 assuming an isotropic material. On straight members the normal-stress maximum is |N/A| + hypot(My,Mz)/Z, bounded over all statics intervals. Torsional shear is separate. No equivalent stress or code stress.",
    "On arcs: signed tangent-frame resultants, nominal stress components and section extremes; endpoint force rows are node-on-element actions in the tangent frame at their end. The normal-stress maximum is bounded along the arc and enters the stress headline. The transverse shear uses the straight-annulus field without curvature correction.",
    "Intensified measures are i*hypot(My,Mz)/Z with the user's scalar SIF at member ends adjacent to markers and branches, using member Z. They are not code stresses and are never combined. A flexibility factor never multiplies stress. On realized arcs with directional SIFs, intensified bending is hypot(i_ip*Mz, i_op*My)/Z in the tangent frame, with its along-arc maximum.",
]
PREVIEW_LIMITATION_SLOTS = slice(2, 5)
PREVIEW_IDENTITIES = ("openpipestress.result_semantics/0.3.0/preview-physics-2",
                      "openpipestress.result_semantics/0.3.0/preview-physics-retained-1")


# ---------------------------------------------------------------- tables

def algebra(kind: str) -> str:
    return "signed_linear" if kind in SIGNED else "recomputed_from_combined" if kind in RECOMPUTED else "never_combined"


def signature(sid, kind, unit, component, basis):
    row = {"signature_id": sid, "kind": kind, "unit": unit, "component": component,
           "source_physical_semantic_dimension": "stress", "derivative_target_dimension": "stress",
           "category": "physical_quantity", "family": "stress", "canonical_disposition": "exported_quantity",
           "legacy_declared_dimension": None,
           "legacy_run_creation_admission": "throws_ANALYSIS_RUN_RESULT_DIMENSION_UNDECLARED",
           "governing_ratio_eligible": False, "source_basis": basis}
    return row


def highest(rows) -> int:
    return max(int(m.group(1)) for r in rows for m in [re.search(r"-(\d{3})(?:-|$)", r["signature_id"])] if m)


def u4b_rows(exact: bool, first: int) -> list:
    out, number = [], [first]

    def each(kind, unit, component, tokens):
        num = "%03d" % number[0]
        number[0] += 1
        for token in tokens:
            if exact:
                for basis in BASES:
                    sid = "supported-source-%s-%s-%s" % (num, token.removeprefix("recovered_from_"), basis)
                    out.append(signature(sid.replace("_", "-"), kind, unit, component, token + "; section_basis=" + basis))
            else:
                sid = ("preview-physics-arc-%s" if token == SVA else "supported-source-%s") % num
                out.append(signature(sid, kind, unit, component, token))

    each("element_local_normal_stress_extreme", "MPa", "sigma_plus", [EXT])
    each("element_local_normal_stress_extreme", "MPa", "sigma_minus", [EXT])
    each("element_local_transverse_shear_stress_maximum", "MPa", "tau_v_max", [SVS, SVA])
    each("curved_bend_directional_intensified_bending_stress_v1", "Pa", "intensified_bending", [DIR])
    each("curved_bend_directional_intensified_bending_maximum_v1", "Pa", "intensified_bending_maximum", [DIR])
    if exact:
        for component in ("toroidal_intrados_hoop_stress", "toroidal_crown_hoop_stress", "toroidal_extrados_hoop_stress"):
            each("pipe_toroidal_membrane_hoop_stress_v1", "Pa", component, [HOOP])
        for component in ("toroidal_inner_radial_stress", "toroidal_outer_radial_stress"):
            each("pipe_toroidal_radial_surface_stress_v1", "Pa", component, [HOOP])
    return out


def marked(rows) -> list:
    out = []
    for row in rows:
        row = collections.OrderedDict(row)
        row["combination_algebra"] = algebra(row["kind"])
        out.append(row)
    return out


def extend(enum: list, items) -> None:
    for item in items:
        if item not in enum:
            enum.append(item)


def table_text(table) -> str:
    return json.dumps(table, indent=2, ensure_ascii=True) + "\n"


def tables(files: dict) -> None:
    """Edits the four table documents in ``files`` (name -> text) in place."""
    hashes = {}
    for name, exact in (("pressure_2", True), ("preview_physics_2", False)):
        table = json.loads(files[TABLE % name], object_pairs_hook=collections.OrderedDict)
        rows = [r for r in table["rows"] if r["kind"] not in U4B_KINDS]
        rows = marked(rows + u4b_rows(exact, highest(rows) + 1))
        table["rows"] = rows
        table["source_signature_count"] = len(rows)
        table["source_kind_count"] = len({r["kind"] for r in rows})
        vocabulary = table["canonical_metadata_vocabulary"]
        extend(vocabulary["component"]["enum"], EXACT_COMPONENTS if exact else PREVIEW_COMPONENTS)
        extend(vocabulary["basis"]["enum"], EXACT_BASES if exact else PREVIEW_BASES)
        limitations = table["supported_profile_limitations"]
        if exact:
            if LIM2_OLD in limitations[2]:
                limitations[2] = limitations[2].replace(LIM2_OLD, LIM2_NEW)
            if LIM_EXTREMES not in limitations:
                limitations.insert(LIM_EXTREMES_AT, LIM_EXTREMES)
        else:
            limitations[PREVIEW_LIMITATION_SLOTS] = PREVIEW_LIMITATIONS
        files[TABLE % name] = table_text(table)
        hashes[name] = hashlib.sha256(files[TABLE % name].encode()).hexdigest()
    for name, base in (("physics_retained_1", "pressure_2"), ("preview_physics_retained_1", "preview_physics_2")):
        table = json.loads(files[TABLE % name], object_pairs_hook=collections.OrderedDict)
        table["inherited_semantic_contract_sha256"] = hashes[base]
        table["rows"] = marked(table["rows"])
        files[TABLE % name] = table_text(table)


# ---------------------------------------------------------------- schemas

def token_variants(items):
    """§2.3: a maximum record takes the straight or the arc token triple together."""
    if "oneOf" in items:
        return items
    variants = []
    for triple in (STRAIGHT, ARC):
        variant = copy.deepcopy(items)
        for key, token in zip(RECORD_TOKENS, triple):
            variant["properties"][key] = {"const": token}
        variants.append(variant)
    return collections.OrderedDict([("oneOf", variants)])


def directional_records(marker_items, with_basis: bool) -> list:
    """§2.6: the marker record, or a directional station or maximum record."""
    basis = {"section_basis": {"enum": list(BASES)}} if with_basis else {}
    factors = {"i_in_plane": {"type": "number", "exclusiveMinimum": 0}, "i_out_of_plane": {"type": "number", "exclusiveMinimum": 0}}
    station = {"type": "object", "additionalProperties": False,
               "required": ["pipe_id", "result_id", "location", "i_in_plane", "i_out_of_plane"] + list(basis),
               "properties": dict({"pipe_id": {"type": "string", "minLength": 1}, "result_id": {"type": "string", "minLength": 1},
                                   "location": {"enum": ["end_i", "end_j", "quarter_1", "midspan", "quarter_3"]}}, **factors, **basis)}
    maximum = {"type": "object", "additionalProperties": False,
               "required": RECORD + ["i_in_plane", "i_out_of_plane"] + list(basis),
               "properties": dict({"pipe_id": {"type": "string", "minLength": 1}, "result_id": {"type": "string", "minLength": 1},
                                   "station_fraction": {"type": "number", "minimum": 0, "maximum": 1}, "span_index": {"const": 0},
                                   "local_fraction": {"type": "number", "minimum": 0, "maximum": 1},
                                   "value_lower_pa": {"type": "number"}, "value_upper_pa": {"type": "number"},
                                   "global_upper_bound_pa": {"type": "number"}, "certified_gap_pa": {"type": "number"},
                                   "subdivisions": {"type": "integer", "minimum": 0, "maximum": 131072},
                                   "approximation": {"const": "arc_closed_form_directional_intensified"},
                                   "coefficient_basis": {"const": ARC[1]}, "enclosure_scope": {"const": ARC[2]}}, **factors, **basis)}
    return ([marker_items] if marker_items is not None else []) + [station, maximum]


def preview_evidence(defs) -> None:
    case = defs["PreviewPhysicsContractEvidence"]["properties"]["preview_cases"]["items"]["properties"]
    case["pipe_stress_extrema"]["items"] = token_variants(case["pipe_stress_extrema"]["items"])
    items = case["intensified_measures"]["items"]
    if "oneOf" not in items:
        case["intensified_measures"]["items"] = {"oneOf": directional_records(items, False)}


def walk(node, visit) -> None:
    if isinstance(node, dict):
        visit(node)
        for value in node.values():
            walk(value, visit)
    elif isinstance(node, list):
        for value in node:
            walk(value, visit)


def results_schema(schema, preview_limitations: list) -> None:
    defs = schema["$defs"]
    extend(defs["ResultMetadata"]["properties"]["component"]["enum"], EXACT_COMPONENTS)
    extend(defs["ResultMetadata"]["properties"]["basis"]["enum"], PREVIEW_BASES + EXACT_BASES)
    extend(defs["PreviewPhysicsResultMetadata"]["properties"]["component"]["enum"], PREVIEW_COMPONENTS)
    extend(defs["PreviewPhysicsResultMetadata"]["properties"]["basis"]["enum"], PREVIEW_BASES)
    preview_evidence(defs)
    # §2.8: the preview branches state the producer's limitations (the retained successor carries its source's).
    for branch in defs["ResultEnvelope"]["oneOf"]:
        identity = branch.get("properties", {}).get("producer", {}).get("properties", {}).get("semantic_contract_id", {}).get("const")
        if identity in PREVIEW_IDENTITIES:
            branch["properties"]["formulation_basis"]["properties"]["limitations"] = {"const": list(preview_limitations)}


def pressure_schema(schema) -> None:
    defs = schema["$defs"]
    region = defs["PressureRegion"]
    geometry = region["properties"]["geometry"]["items"]
    geometry["properties"]["bend_radius_m"] = {"type": "number", "exclusiveMinimum": 0}
    geometry["dependentRequired"]["member_kind"] = ["arc_end_tangents_global", "bend_radius_m"]
    geometry["dependentRequired"]["bend_radius_m"] = ["member_kind"]
    if "withheld_on_arcs" in region["properties"]:
        del region["properties"]["withheld_on_arcs"]
        region["properties"]["arc_stress_definitions"] = {"type": "array", "minItems": 2, "maxItems": 2, "items": {
            "type": "object", "additionalProperties": False, "required": ["definition", "statement"],
            "properties": {"definition": {"enum": [HOOP, SVA]}, "statement": {"type": "string", "minLength": 1}}}}

    def rename(node):
        if isinstance(node.get("required"), list):
            node["required"] = ["arc_stress_definitions" if x == "withheld_on_arcs" else x for x in node["required"]]
    walk(defs, rename)
    case = defs["PressureExactCase"]
    case["properties"]["pipe_stress_extrema"]["items"] = token_variants(case["properties"]["pipe_stress_extrema"]["items"])
    case["properties"]["intensified_measures"] = {"type": "array", "minItems": 1, "items": {"oneOf": directional_records(None, True)}}

    def enums(node):
        values = node.get("enum")
        if isinstance(values, list) and "maximum_absolute_normal_stress" in values:
            extend(values, EXACT_COMPONENTS)
        elif isinstance(values, list) and "recovered_from_open_mechanics_stress_components; section_basis=nominal_section" in values:
            extend(values, EXACT_BASES)
    walk(defs, enums)


def component_schema(schema) -> None:
    def fields(node):
        values = node.get("enum")
        if isinstance(values, list) and "sif_user_value" in values and "sif_in_plane_user_value" not in values:
            i = values.index("sif_user_value") + 1
            values[i:i] = ["sif_in_plane_user_value", "sif_out_of_plane_user_value"]
    walk(schema, fields)


def schema_text(schema, original: str) -> str:
    # Keep each file's own escaping: an ASCII-escaped file stays escaped.
    escaped = "\\u" in original and all(ord(c) < 128 for c in original)
    return json.dumps(schema, indent=2, ensure_ascii=escaped) + "\n"


def schemas(files: dict) -> None:
    preview = json.loads(files[TABLE % "preview_physics_2"])["supported_profile_limitations"]
    for path, edit in (("schemas/results.v0.3.schema.yaml", lambda s: results_schema(s, preview)),
                       ("schemas/stress_neutral_export.v0.3.schema.json", lambda s: preview_evidence(s["$defs"])),
                       ("schemas/results_pressure_2.v0.3.schema.json", pressure_schema),
                       ("schemas/component.schema.yaml", component_schema)):
        schema = json.loads(files[path], object_pairs_hook=collections.OrderedDict)
        edit(schema)
        files[path] = schema_text(schema, files[path])


# ---------------------------------------------------------------- entry

PATHS = [TABLE % n for n in ("pressure_2", "preview_physics_2", "physics_retained_1", "preview_physics_retained_1")] + [
    "schemas/results.v0.3.schema.yaml", "schemas/stress_neutral_export.v0.3.schema.json",
    "schemas/results_pressure_2.v0.3.schema.json", "schemas/component.schema.yaml"]


def generate() -> tuple[dict, dict]:
    current = {p: (PROJECT / p).read_text(encoding="utf-8") for p in PATHS}
    files = dict(current)
    tables(files)
    schemas(files)
    return current, files


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--write", action="store_true", help="rewrite the tables and schemas and print their sha256")
    mode.add_argument("--check", action="store_true", help="exit 1 unless every file is already current")
    args = parser.parse_args(argv)
    current, files = generate()
    stale = [p for p in PATHS if files[p] != current[p]]
    if args.check:
        for path in stale:
            print(f"stale: {path}", file=sys.stderr)
        return 1 if stale else 0
    for path in stale:
        (PROJECT / path).write_text(files[path], encoding="utf-8")
    for path in PATHS:
        print(f"{hashlib.sha256(files[path].encode('utf-8')).hexdigest()}  {path}{'  (rewritten)' if path in stale else ''}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
