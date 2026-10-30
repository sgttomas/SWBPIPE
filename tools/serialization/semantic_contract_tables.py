#!/usr/bin/env python3
"""Mechanical regeneration of the current semantic-contract tables and their pins.

The pressure-2 table (`semantic_contract_v0_3_pressure_2.json`) is authored by hand:
its rows, vocabulary and limitation sentences are the reviewed content. Everything
derived from it is regenerated here, never edited by hand:

  apply-u4b  the T4-U4b contract (signatures numbered after each table's highest id,
          algebra marks, vocabularies, limitation texts, result-schema records) on
          pressure-2, preview-physics-2, the inheriting tables (preview-physics-retained-1
          taking preview-physics-2's rows, vocabulary and preview texts), pressure-source-1
          (its straight derived signatures and disclosure) and the result schemas,
          idempotently; `--check` reports a difference instead of writing.
  derive  pressure-2's `source_signature_count` and `source_kind_count` from its rows;
          physics-retained-1's inherited members (rows, vocabulary, counts, policies,
          hash vectors) copied from pressure-2, and its
          `inherited_semantic_contract_sha256` set to pressure-2's actual SHA-256.
          The Timoshenko preview table (preview-physics-timoshenko-1, DEL-05-03 Design
          pf-timoshenko.md §4) is derived from the committed preview-physics-2 table:
          its identity, its profile and limitation 0 replaced, every other member and
          byte preview-physics-2's. `--check` reports a difference instead of writing.
  repin   every tracked literal pin of an old table or result-schema SHA-256 (PINNED)
          replaced by the current file's SHA-256. The old values are read from the
          given revisions (`--from-rev`, repeatable; after a merge pass both parents).
          `--dry-run` lists the edits without writing. Frozen and historical trees
          (validation/references/**, execution/**) and the retained-pricing tool
          (tools/retained_pricing/**, whose basis pins S5's D3 registration owns) are
          never edited; an old value found only there is refused, since its pins are not this tool's to move.
          Carrier fixtures whose bytes embed a checksum are never edited here; their
          writers are printed instead. Protected records inside an editable file are
          never edited either (PROTECTED: the registered retained-memory profile's
          reviewed_inputs string in retained_memory.rs, which S5's D3 registration owns).
          Repin from a clean HEAD: the old values are read only from the named revisions
          and from this run's own writes, so a digest written by an earlier run is not
          recognized. After a partial or repeated edit of a pinned file, restore the
          consumers to HEAD and run repin once, after every pinned file is final.

  apply-w7  the W-7 global end-action signatures on pressure-2 and preview-physics-2 (before
          the U4b rows), their basis vocabulary, the preview combination policies' linear
          state kinds and the result schemas' closed basis enums;
          then apply-u4b and derive carry them to the inheriting tables. Idempotent;
          `--check` reports a difference instead of writing.
  apply-w16 the W-16 stage 2 (R-b) relative-displacement signatures on preview-physics-2
          (after the W-7 rows; apply-u4b keeps them there and carries them to
          preview-physics-retained-1). Never combined. Idempotent; `--check` reports a
          difference instead of writing.

Typical sequence after editing pressure-2 (or merging another edit of it):

  python3 tools/serialization/semantic_contract_tables.py apply-u4b
  python3 tools/serialization/semantic_contract_tables.py derive   # also after editing preview-physics-2
  python3 tools/serialization/semantic_contract_tables.py repin --from-rev HEAD [--from-rev MERGE_HEAD] [--dry-run]
  # then run the printed carrier writers and the audited-suffix and generation-record
  # refreshes the change requires (those bind source text, not these tables).

Standard library only. Run from the project root (projects/chirality-piping).
"""
from __future__ import annotations

import argparse
import re
import copy
import collections
import hashlib
import json
import subprocess
import sys
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(PROJECT / "tools"))
from repository_history import source_spec
PRESSURE = "fixtures/results/semantic_contract_v0_3_pressure_2.json"
RETAINED = "fixtures/results/semantic_contract_v0_3_physics_retained_1.json"
# DEL-05-03 Design pf-timoshenko.md §4: the pressure-free Timoshenko identity, preview-physics-2
# with its identity, profile and limitation 0 replaced. Never hand-edited.
PREVIEW = "fixtures/results/semantic_contract_v0_3_preview_physics_2.json"
TIMOSHENKO = "fixtures/results/semantic_contract_v0_3_preview_physics_timoshenko_1.json"
TIMOSHENKO_ID = "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1"
TIMOSHENKO_PROFILE = "product_preview_mechanics_timoshenko_v1"
PREVIEW_LIMITATION_0 = ("Small-displacement, linear-elastic Euler-Bernoulli frame preview; "
                        "numerical integrity does not establish physical correctness.")
TIMOSHENKO_LIMITATION_0 = (
    "Small-displacement, linear-elastic frame preview: straight members, including geometry-only bends, are "
    "Timoshenko with the energy-matched annular shear factor (timoshenko_annulus_energy_v1) and Poisson's ratio "
    "E/(2G) - 1, and realized arcs are Euler-Bernoulli; a straight member's published nodal rotations are section "
    "rotations, not slopes; numerical integrity does not establish physical correctness.")
SCHEMA = "schemas/results_pressure_2.v0.3.schema.json"
# The members physics-retained-1 inherits unchanged from pressure-2. The retained
# table's own members (identity, profile, limitations, receipt and formation
# definitions, accuracy classification, evidence policy) are authored there.
INHERITED = (
    "canonical_metadata_vocabulary",
    "hash_vectors",
    "metadata_policy",
    "reserved_inactive_successors",
    "rows",
    "schema_version",
    "source_basis_signature_policy",
    "source_kind_count",
    "source_producer",
    "source_schema_version",
    "source_signature_count",
)
# Frozen references and historical records: never edited by `repin`.
EXCLUDED = ("validation/references/", "execution/",
            # The retained-pricing tool's basis pins belong to S5's D3 registration.
            "tools/retained_pricing/")
# Carriers that embed checksums over their own bytes: regenerated by their writers.
CARRIER_WRITERS = {
    "fixtures/results/pressure_load_state_": (
        "LOAD_REFERENCE_WRITE_FIXTURES=1 pytest tests/test_load_reference_readers.py "
        "-k analysis_run_carrier_fixture_is_reproduced_byte_for_byte; "
        "LOAD_REFERENCE_WRITE_FIXTURES=1 cargo test (result_export) --test load_reference_contract; "
        "STRESS_NEUTRAL_WRITE_FIXTURES=1 pytest tests/test_stress_neutral_physics_source.py "
        "-k current_native_builder_matches_committed_package_file_bytes"
    ),
}
# The desktop run carriers embed checksums over records that carry the table digests. The
# TypeScript writers build the transient inputs and the S2 run; the Rust record path then emits
# the recorded-run witnesses from those inputs (read at compile time), so the TypeScript writers
# run first. Each variable takes the target file's path, written as given from the test's working
# directory, so give it absolutely.
PERSISTENCE_RUN_WRITERS = (
    "OPENPIPESTRESS_WRITE_TRANSIENT_RUN_INPUT=<abs fixtures/persistence/transient_run_input.json> "
    "OPENPIPESTRESS_WRITE_TRANSIENT_RUN_INPUT_RULE_PACKS=<abs fixtures/persistence/transient_run_input_rule_packs.json> "
    "OPENPIPESTRESS_WRITE_S2_UNRESOLVED_RULE_PACK_RUN=<abs fixtures/persistence/s2_unresolved_rule_pack_run.json> "
    "vitest run (apps/desktop) src/services/recordedRunParity.test.ts; then "
    "OPENPIPESTRESS_WRITE_RECORDED_RUN_WITNESS=<abs fixtures/persistence/recorded_run_witness.json> "
    "OPENPIPESTRESS_WRITE_RECORDED_RUN_WITNESS_RULE_PACKS=<abs fixtures/persistence/recorded_run_witness_rule_packs.json> "
    "cargo test (apps/desktop/src-tauri) analysis_runs_tests"
)
CARRIER_WRITERS.update({
    "fixtures/persistence/transient_run_input": PERSISTENCE_RUN_WRITERS,
    "fixtures/persistence/s2_unresolved_rule_pack_run": PERSISTENCE_RUN_WRITERS,
    "fixtures/persistence/recorded_run_witness": PERSISTENCE_RUN_WRITERS,
})


# ---------------------------------------------------------------- apply-u4b
# T4-U4b (DEL-04-07 Design u4b-u5b-implementation.md §2.2-§2.8, §5.1): the U4b contract, applied
# idempotently on top of whatever else the tables carry. pressure-2 and preview-physics-2 gain
# the U4b signatures (sigma+/sigma-, the transverse shear maximum on its straight and arc tokens,
# the directional intensified station and maximum rows and, on pressure-2, the toroidal hoop and
# radial rows), numbered after each table's highest id: one number per (kind, unit, component); a
# pressure-2 stress signature adds `-<token>-<basis>`, a preview arc variant is
# `preview-physics-arc-<number>`. Existing U4b rows are removed first, so a re-run on a table that
# gained other rows renumbers U4b after them. Every row of the four marked tables carries
# `combination_algebra` (§2.7); vocabularies, limitation texts (§2.5, §2.8) and the result
# schemas (closed arc and directional records, the extrema token triples as a oneOf, the region
# statement, the preview limitation consts, the component field catalog) follow.

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
    "connector_pressure_force_v1", "connector_effective_axial_force_v1", "connector_tie_rod_force_v1",
    "element_global_end_force_v1", "element_global_end_moment_v1"}
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
# T4-U4b §4.3 (S4): pressure-source-1 carries the straight-member derived kinds of its source
# family (no arc, no connector: D7, D1) on both bases, outside projection.quantity and REQ-002,
# with one disclosure sentence placed after the endpoint-maximum sentence (outside TBD-002).
SOURCE_DISCLOSURE = ("Section normal-stress extremes and transverse-shear maxima are published beside the qualified rows, "
                     "formed from them, and are not qualified.")
SOURCE_DISCLOSURE_AFTER = "Retained endpoint maxima bound source-action projection"
SOURCE_COMPONENTS = ["sigma_plus", "sigma_minus", "tau_v_max"]
SOURCE_BASES = [t + s for t in [EXT, SVS] for s in SUFFIXES]


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


def source_u4b_rows(first: int) -> list:
    """§4.3: pressure-source-1's straight derived signatures, unmarked (the table carries no
    algebra marks), numbered after its highest id with pressure-2's id rule."""
    out = []
    for number, (kind, component, token) in enumerate((
            ("element_local_normal_stress_extreme", "sigma_plus", EXT),
            ("element_local_normal_stress_extreme", "sigma_minus", EXT),
            ("element_local_transverse_shear_stress_maximum", "tau_v_max", SVS)), start=first):
        for basis in BASES:
            sid = "supported-source-%03d-%s-%s" % (number, token.removeprefix("recovered_from_"), basis)
            out.append(signature(sid.replace("_", "-"), kind, "MPa", component, token + "; section_basis=" + basis))
    return out


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


def u4b_tables(files: dict) -> None:
    """Edits the four table documents in ``files`` (name -> text) in place."""
    hashes = {}
    for name, exact in (("pressure_2", True), ("preview_physics_2", False)):
        table = json.loads(files[TABLE % name], object_pairs_hook=collections.OrderedDict)
        # W-7's rows, then W-16's, follow the U4b rows and keep their numbers (apply-w7, apply-w16).
        w7 = [r for r in table["rows"] if r["kind"] in W7_KINDS + W16_KINDS]
        rows = [r for r in table["rows"] if r["kind"] not in U4B_KINDS and r["kind"] not in W7_KINDS + W16_KINDS]
        rows = marked(rows + u4b_rows(exact, highest(rows) + 1) + w7)
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
        if name == "preview_physics_retained_1":
            # T4-U4b §4.1 (S4): the preview-retained rows follow preview-physics-2's (the U4b
            # rows included), then its own rows (the requested-mode basis); its vocabulary takes
            # the base's tokens, and its preview limitations are the base's U4b texts.
            source = json.loads(files[TABLE % base], object_pairs_hook=collections.OrderedDict)
            inherited = {r["signature_id"] for r in source["rows"]}
            own = [r for r in table["rows"] if r["signature_id"] not in inherited and r["kind"] not in U4B_KINDS]
            table["rows"] = [collections.OrderedDict(r) for r in source["rows"]] + own
            table["source_signature_count"] = len(table["rows"])
            table["source_kind_count"] = len({r["kind"] for r in table["rows"]})
            for key, vocabulary in source["canonical_metadata_vocabulary"].items():
                if "enum" in vocabulary:
                    extend(table["canonical_metadata_vocabulary"][key]["enum"], vocabulary["enum"])
            table["supported_profile_limitations"][PREVIEW_LIMITATION_SLOTS] = PREVIEW_LIMITATIONS
        table["rows"] = marked(table["rows"])
        files[TABLE % name] = table_text(table)
    # §4.3: pressure-source-1 (its own table; not an inheriting one).
    table = json.loads(files[TABLE % "pressure_source_1"], object_pairs_hook=collections.OrderedDict)
    rows = [r for r in table["rows"] if r["kind"] not in U4B_KINDS]
    rows = rows + source_u4b_rows(highest(rows) + 1)
    table["rows"] = rows
    table["source_signature_count"] = len(rows)
    table["source_kind_count"] = len({r["kind"] for r in rows})
    vocabulary = table["canonical_metadata_vocabulary"]
    extend(vocabulary["component"]["enum"], SOURCE_COMPONENTS)
    extend(vocabulary["basis"]["enum"], SOURCE_BASES)
    limitations = table["supported_profile_limitations"]
    if SOURCE_DISCLOSURE not in limitations:
        at = next(i for i, text in enumerate(limitations) if text.startswith(SOURCE_DISCLOSURE_AFTER))
        limitations.insert(at + 1, SOURCE_DISCLOSURE)
    files[TABLE % "pressure_source_1"] = table_text(table)


# ---------------------------------------------------------------- apply-w7
# W-7 (DEL-04-01 Design global-end-actions.md): member end actions in global coordinates. Six
# signatures (Fx, Fy, Fz in N; Mx, My, Mz in N*m) on the two ordinary tables whose producers
# publish member rows (pressure-2, preview-physics-2), appended and numbered after each table's
# highest id (apply-u4b keeps them after its rows and numbers U4b without them). The
# inheriting tables take them through apply-u4b (preview-physics-retained-1) and derive
# (physics-retained-1). pressure-source-1 does not: a source-recovery publication omits the
# rows (`global_end_rows_published`). Signed linear; the combination policies of
# preview-physics-2 and preview-physics-retained-1 list both kinds among their linear state kinds. Idempotent; `--check` reports a difference.

W7_FORCE = "element_global_end_force_v1"
W7_MOMENT = "element_global_end_moment_v1"
W7_KINDS = (W7_FORCE, W7_MOMENT)
W7_BASIS = "rotated_from_published_element_local_end_actions_v1"
W7_WALL_BASIS = W7_BASIS + ";axial_is_pipe_wall_action"


def w7_rows(first: int) -> list:
    out = []
    for kind, unit, family, names in ((W7_FORCE, "N", "force", ("Fx", "Fy", "Fz")),
                                      (W7_MOMENT, "N*m", "moment", ("Mx", "My", "Mz"))):
        for name in names:
            out.append(collections.OrderedDict([
                ("signature_id", "supported-source-%03d" % (first + len(out))), ("kind", kind), ("unit", unit),
                ("component", name), ("source_physical_semantic_dimension", family), ("derivative_target_dimension", family),
                ("category", "physical_quantity"), ("family", family), ("canonical_disposition", "exported_quantity"),
                ("legacy_declared_dimension", None),
                ("legacy_run_creation_admission", "throws_ANALYSIS_RUN_RESULT_DIMENSION_UNDECLARED"),
                ("governing_ratio_eligible", False), ("combination_algebra", algebra(kind))]))
    return out


def w7_tables(files: dict) -> None:
    for name, bases in (("pressure_2", [W7_BASIS, W7_WALL_BASIS]), ("preview_physics_2", [W7_BASIS])):
        table = json.loads(files[TABLE % name], object_pairs_hook=collections.OrderedDict)
        # W-16's rows (apply-w16) follow W-7's and keep their numbers.
        w16 = [r for r in table["rows"] if r["kind"] in W16_KINDS]
        rows = [r for r in table["rows"] if r["kind"] not in W7_KINDS + W16_KINDS]
        rows = rows + w7_rows(highest(rows) + 1) + w16
        table["rows"] = rows
        table["source_signature_count"] = len(rows)
        table["source_kind_count"] = len({r["kind"] for r in rows})
        extend(table["canonical_metadata_vocabulary"]["basis"]["enum"], bases)
        policy = table.get("combination_policy")
        if policy is not None:
            extend(policy["linear_state_kinds"], W7_KINDS)
        files[TABLE % name] = table_text(table)
    # preview-physics-retained-1 states its own combination policy: the same linear state kinds.
    table = json.loads(files[TABLE % "preview_physics_retained_1"], object_pairs_hook=collections.OrderedDict)
    extend(table["combination_policy"]["linear_state_kinds"], W7_KINDS)
    files[TABLE % "preview_physics_retained_1"] = table_text(table)


def w7_schemas(files: dict) -> None:
    """The closed metadata basis enums of the result schemas: both W-7 bases where the exact
    vocabulary is closed (PressureResultMetadata, ResultMetadata), the plain basis on the
    preview branch (PreviewPhysicsResultMetadata)."""
    for path, edits in (("schemas/results_pressure_2.v0.3.schema.json", (("PressureResultMetadata", [W7_BASIS, W7_WALL_BASIS]),)),
                        ("schemas/results.v0.3.schema.yaml", (("ResultMetadata", [W7_BASIS, W7_WALL_BASIS]),
                                                              ("PreviewPhysicsResultMetadata", [W7_BASIS])))):
        schema = json.loads(files[path], object_pairs_hook=collections.OrderedDict)
        for name, bases in edits:
            extend(schema["$defs"][name]["properties"]["basis"]["enum"], bases)
        files[path] = schema_text(schema, files[path])


W7_PATHS = [TABLE % n for n in ("pressure_2", "preview_physics_2", "preview_physics_retained_1")] + [
    "schemas/results_pressure_2.v0.3.schema.json", "schemas/results.v0.3.schema.yaml"]


def apply_w7(check: bool) -> int:
    current = {p: (PROJECT / p).read_text(encoding="utf-8") for p in W7_PATHS}
    files = dict(current)
    w7_tables(files)
    w7_schemas(files)
    stale = [p for p in W7_PATHS if files[p] != current[p]]
    for path in W7_PATHS:
        state = "unchanged" if path not in stale else ("differs" if check else "written")
        if path in stale and not check:
            (PROJECT / path).write_text(files[path], encoding="utf-8")
        print(f"{path}: {state}; sha256 {hashlib.sha256(files[path].encode('utf-8')).hexdigest()}")
    return 1 if check and stale else 0


# ---------------------------------------------------------------- apply-w16
# W-16 stage 2, R-b (DEL-04-03 Design pf-nonlinear-imposed-motion.md §4): each support whose
# ground the case moves publishes `nonlinear_support_final_relative_displacement`, an observation
# row, in the two families of `nonlinear_support_final_displacement` (mm, length; rad, angle).
# Two signatures on preview-physics-2, appended after the W-7 rows and numbered after the table's
# highest id; preview-physics-retained-1 inherits them through apply-u4b. Never combined and not
# eligible for governing ratios. Idempotent; `--check` reports a difference.

W16_RELATIVE = "nonlinear_support_final_relative_displacement"
W16_KINDS = (W16_RELATIVE,)


def w16_rows(first: int) -> list:
    out = []
    for unit, dimension, family, admission in (
            ("mm", "length", "displacement", "accepts_with_legacy_dimension_equal_new_semantics"),
            ("rad", "angle", "rotation", "accepts_with_preserved_legacy_dimension_different_from_new_semantics")):
        out.append(collections.OrderedDict([
            ("signature_id", "supported-source-%03d" % (first + len(out))), ("kind", W16_RELATIVE), ("unit", unit),
            ("component", W16_RELATIVE), ("source_physical_semantic_dimension", dimension),
            ("derivative_target_dimension", dimension), ("category", "physical_quantity"), ("family", family),
            ("canonical_disposition", "exported_quantity"), ("legacy_declared_dimension", "length"),
            ("legacy_run_creation_admission", admission), ("governing_ratio_eligible", False),
            ("combination_algebra", algebra(W16_RELATIVE))]))
    return out


def w16_tables(files: dict) -> None:
    table = json.loads(files[TABLE % "preview_physics_2"], object_pairs_hook=collections.OrderedDict)
    rows = [r for r in table["rows"] if r["kind"] not in W16_KINDS]
    rows = rows + w16_rows(highest(rows) + 1)
    table["rows"] = rows
    table["source_signature_count"] = len(rows)
    table["source_kind_count"] = len({r["kind"] for r in rows})
    files[TABLE % "preview_physics_2"] = table_text(table)


W16_PATHS = [TABLE % "preview_physics_2"]


def apply_w16(check: bool) -> int:
    current = {p: (PROJECT / p).read_text(encoding="utf-8") for p in W16_PATHS}
    files = dict(current)
    w16_tables(files)
    stale = [p for p in W16_PATHS if files[p] != current[p]]
    for path in W16_PATHS:
        state = "unchanged" if path not in stale else ("differs" if check else "written")
        if path in stale and not check:
            (PROJECT / path).write_text(files[path], encoding="utf-8")
        print(f"{path}: {state}; sha256 {hashlib.sha256(files[path].encode('utf-8')).hexdigest()}")
    return 1 if check and stale else 0


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
        elif identity == TIMOSHENKO_ID:
            # pf-timoshenko.md §4: preview-physics-2's texts with limitation 0 replaced, as the table.
            branch["properties"]["formulation_basis"]["properties"]["limitations"] = {
                "const": [TIMOSHENKO_LIMITATION_0] + list(preview_limitations[1:])}


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


def u4b_schemas(files: dict) -> None:
    preview = json.loads(files[TABLE % "preview_physics_2"])["supported_profile_limitations"]
    for path, edit in (("schemas/results.v0.3.schema.yaml", lambda s: results_schema(s, preview)),
                       ("schemas/stress_neutral_export.v0.3.schema.json", lambda s: preview_evidence(s["$defs"])),
                       ("schemas/results_pressure_2.v0.3.schema.json", pressure_schema),
                       ("schemas/component.schema.yaml", component_schema)):
        schema = json.loads(files[path], object_pairs_hook=collections.OrderedDict)
        edit(schema)
        files[path] = schema_text(schema, files[path])




U4B_PATHS = [TABLE % n for n in ("pressure_2", "preview_physics_2", "physics_retained_1", "preview_physics_retained_1",
                                  "pressure_source_1")] + [
    "schemas/results.v0.3.schema.yaml", "schemas/stress_neutral_export.v0.3.schema.json",
    "schemas/results_pressure_2.v0.3.schema.json", "schemas/component.schema.yaml"]


def apply_u4b(check: bool) -> int:
    current = {p: (PROJECT / p).read_text(encoding="utf-8") for p in U4B_PATHS}
    files = dict(current)
    u4b_tables(files)
    u4b_schemas(files)
    stale = [p for p in U4B_PATHS if files[p] != current[p]]
    for path in U4B_PATHS:
        state = "unchanged" if path not in stale else ("differs" if check else "written")
        if path in stale and not check:
            (PROJECT / path).write_text(files[path], encoding="utf-8")
        print(f"{path}: {state}; sha256 {hashlib.sha256(files[path].encode('utf-8')).hexdigest()}")
    return 1 if check and stale else 0


def dumps(value: object) -> bytes:
    """The tables' committed form: two-space indent, ASCII escapes, final newline."""
    return (json.dumps(value, indent=2, ensure_ascii=True) + "\n").encode("ascii")


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def derived_tables() -> dict[str, bytes]:
    pressure = json.loads((PROJECT / PRESSURE).read_bytes())
    rows = pressure["rows"]
    pressure["source_signature_count"] = len(rows)
    pressure["source_kind_count"] = len({row["kind"] for row in rows})
    pressure_bytes = dumps(pressure)
    retained = json.loads((PROJECT / RETAINED).read_bytes())
    for key in INHERITED:
        if key not in retained:
            raise SystemExit(f"{RETAINED}: inherited member {key} missing")
        retained[key] = pressure[key]
    retained["inherited_semantic_contract_sha256"] = sha256(pressure_bytes)
    return {PRESSURE: pressure_bytes, RETAINED: dumps(retained), TIMOSHENKO: timoshenko_table()}


def timoshenko_table() -> bytes:
    """preview-physics-timoshenko-1 from the committed preview-physics-2 bytes: the same members in
    the same order, with the identity, the profile and limitation 0 replaced. A preview-physics-2
    whose limitation 0 is not the Euler-Bernoulli sentence is refused, so a later edit of that
    sentence is reviewed here rather than carried silently."""
    preview = json.loads((PROJECT / PREVIEW).read_bytes())
    if preview["semantic_contract_id"] != PREVIEW_IDENTITIES[0] or preview["formulation_profile_id"] != "product_preview_mechanics_v1":
        raise SystemExit(f"{PREVIEW}: unexpected identity or profile")
    if preview["supported_profile_limitations"][0] != PREVIEW_LIMITATION_0:
        raise SystemExit(f"{PREVIEW}: limitation 0 is not the Euler-Bernoulli sentence this derivation replaces")
    preview["semantic_contract_id"] = TIMOSHENKO_ID
    preview["formulation_profile_id"] = TIMOSHENKO_PROFILE
    preview["supported_profile_limitations"][0] = TIMOSHENKO_LIMITATION_0
    return dumps(preview)


def derive(check: bool) -> int:
    changed = 0
    for path, data in derived_tables().items():
        target = PROJECT / path
        current = target.read_bytes() if target.exists() else b""
        state = "unchanged" if current == data else ("differs" if check else "written")
        if current != data:
            changed += 1
            if not check:
                (PROJECT / path).write_bytes(data)
        print(f"{path}: {state}; sha256 {sha256(data)}")
    return 1 if check and changed else 0


def git(*args: str) -> str:
    return subprocess.run(["git", *args], cwd=PROJECT, check=True, capture_output=True, text=True).stdout


# Records inside editable files that repin never edits, by path. The registered retained-memory
# profile records the reviewed inputs it was registered on; S5's D3 re-registration owns it.
PROTECTED = {
    "core/product_physics/src/retained_memory.rs": re.compile(r'reviewed_inputs: "v1;[^"]*"'),
}


def replace_unprotected(consumer: str, text: str, old: str, new: str) -> tuple[str, int]:
    """``text`` with ``old`` replaced by ``new`` outside the consumer's protected records, and the
    number of occurrences left inside them."""
    pattern = PROTECTED.get(consumer)
    if pattern is None:
        return text.replace(old, new), 0
    parts, last, kept = [], 0, 0
    for match in pattern.finditer(text):
        parts.append(text[last:match.start()].replace(old, new))
        parts.append(match.group(0))
        kept += match.group(0).count(old)
        last = match.end()
    parts.append(text[last:].replace(old, new))
    return "".join(parts), kept


def classify(consumers: list[str]) -> tuple[list[str], list[str], list[str]]:
    """(editable, carriers, excluded) consumers of one old value, by path."""
    editable, carriers, excluded = [], [], []
    for consumer in consumers:
        if consumer.startswith(EXCLUDED):
            excluded.append(consumer)
        elif any(consumer.startswith(prefix) for prefix in CARRIER_WRITERS):
            carriers.append(consumer)
        else:
            editable.append(consumer)
    return editable, carriers, excluded


# The pinned files, tables before the schemas that embed their hashes: pressure-2, the
# inheriting and preview tables (T4-U4b), the result schemas and the analysis-run schema.
PINNED = (PRESSURE, RETAINED, TABLE % "preview_physics_2", TIMOSHENKO, TABLE % "preview_physics_retained_1", SCHEMA,
          "schemas/results.v0.3.schema.yaml", "schemas/stress_neutral_export.v0.3.schema.json",
          "schemas/component.schema.yaml", "schemas/analysis_run.v0.3.schema.json",
          # T4-U4b S4: the pressure-source-1 table, the source receipt schema (reviewed input 1)
          # and the three formation definitions DEF-O, DEF-C and DEF-E.
          TABLE % "pressure_source_1", "schemas/physics_source_recovery.schema.json",
          "fixtures/results/retained_precision_prepared_ordinary_v1.json",
          "fixtures/results/retained_precision_prepared_combination_v1.json",
          "fixtures/results/retained_precision_prepared_exact_v1.json")


def repin(revisions: list[str], dry_run: bool = False) -> int:
    """Repins to a fixed point: a file whose pins were rewritten may itself be pinned (a schema
    embedding a table hash), so passes repeat until none edits, and every value written for a
    path in an earlier pass counts as an old value of that path. A dry run makes one pass and
    writes nothing, so it lists the first pass's edits only (a cascade appears once they are
    written). Frozen and historical trees are never edited; an old value found only there is
    refused."""
    edits = 0
    carriers: set[str] = set()
    refused: list[str] = []
    written: dict[str, set[str]] = {path: set() for path in PINNED}
    while True:
        pass_edits = 0
        for path in PINNED:
            new = sha256((PROJECT / path).read_bytes())
            olds = set(written[path])
            for revision in revisions:
                try:
                    olds.add(sha256(subprocess.run(["git", "show", source_spec(PROJECT, revision, path)], cwd=PROJECT, check=True,
                                                   capture_output=True).stdout))
                except (subprocess.CalledProcessError, ValueError):
                    continue
            for old in sorted(olds - {new}):
                listed = subprocess.run(["git", "grep", "-l", old, "--", "."], cwd=PROJECT, capture_output=True, text=True)
                editable, found_carriers, excluded = classify(list(filter(None, listed.stdout.splitlines())))
                if excluded and not editable and not found_carriers:
                    reason = f"{path} {old[:12]}...: found only in {', '.join(excluded)}"
                    if reason not in refused:
                        refused.append(reason)
                    continue
                for consumer in excluded:
                    print(f"{consumer}: {path} {old[:12]}... left unchanged (frozen or historical tree)")
                carriers.update(found_carriers)
                for consumer in editable:
                    file = PROJECT / consumer
                    text = file.read_text(encoding="utf-8")
                    updated, kept = replace_unprotected(consumer, text, old, new)
                    if kept:
                        print(f"{consumer}: {path} {old[:12]}... left unchanged in its protected record")
                    if updated == text:
                        continue
                    if not dry_run:
                        file.write_text(updated, encoding="utf-8")
                    pass_edits += 1
                    print(f"{consumer}: {path} {old[:12]}... -> {new[:12]}...{' (dry run)' if dry_run else ''}")
            written[path].add(new)
        edits += pass_edits
        if dry_run or not pass_edits:
            break
    for consumer in sorted(carriers):
        print(f"{consumer}: carrier; regenerate through its writer")
    for prefix, writer in CARRIER_WRITERS.items():
        if any(c.startswith(prefix) for c in carriers):
            print(f"writers for {prefix}*: {writer}")
    for reason in refused:
        print(f"refused: {reason}")
    print(f"{edits} file edit(s){' listed, none written' if dry_run else ''}")
    return 1 if refused else 0


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = parser.add_subparsers(dest="command", required=True)
    apply_parser = sub.add_parser("apply-u4b")
    apply_parser.add_argument("--check", action="store_true")
    w7_parser = sub.add_parser("apply-w7")
    w7_parser.add_argument("--check", action="store_true")
    w16_parser = sub.add_parser("apply-w16")
    w16_parser.add_argument("--check", action="store_true")
    derive_parser = sub.add_parser("derive")
    derive_parser.add_argument("--check", action="store_true")
    repin_parser = sub.add_parser("repin", description=(
        "Replace literal pins of old table and schema SHA-256s (read from the --from-rev revisions) by the "
        "current ones. Repin from a clean HEAD: run it once, after every pinned file is final, on consumers "
        "that no earlier repin run has edited (restore them to HEAD first). The registered retained-memory "
        "profile's reviewed_inputs record and tools/retained_pricing/** are never edited."))
    repin_parser.add_argument("--from-rev", action="append", required=True)
    repin_parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args(argv)
    if args.command == "apply-u4b":
        return apply_u4b(args.check)
    if args.command == "apply-w7":
        return apply_w7(args.check)
    if args.command == "apply-w16":
        return apply_w16(args.check)
    if args.command == "derive":
        return derive(args.check)
    return repin(args.from_rev, args.dry_run)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
