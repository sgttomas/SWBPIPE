"""Test verifier of the base-route verdicts the desktop and Python product take from Rust (b7-plan R2).

Python's independent reading of `core/reporting/result_export/src/reader_verdict.rs`'s entries:
`dispatch` (the header route, its table binding, the header refusal and the standing), `validate` and
`transport` per reader, and `row_semantics`. It is ported from the TypeScript readers' logic that R2
retires (`numericalResultQuality.ts`'s `sourceContract` and `numericalResultStanding`,
`physicsResultEvidence.ts`'s model checks, `sourceBlockRecovery.ts`'s standing findings and
`resultSemantics.ts`'s table lookups), under T3's rulings (2026-10-11):

- G1: the standing's findings, the TypeScript standing's codes in its order, beside the
  `numerical_use_standing` token (here the verifier's own, `compatibility.numerical_use_standing`).
- G2: with a current model, the pressure reader runs the TypeScript model checks after the unchanged
  reader: every solved case is a model load case (`SOURCE_PHYSICS_MODEL_CASE_COVERAGE`), each case's
  members are the model's pipes but those an objective connector replaces
  (`SOURCE_PHYSICS_MODEL_MEMBER_COVERAGE`), each case section's captured D, t, c_m and c_a are the model's
  section of that pipe bit for bit (T4-U6: `SOURCE_PHYSICS_MODEL_SECTION_GEOMETRY`), each section entry states the
  beam theory the model's contract
  selects for its member (T4-U8: `SOURCE_PHYSICS_SHEAR_MODEL_CONTRACT`, `SOURCE_PHYSICS_ARC_BEAM_THEORY`), and a
  support row names a model support (`SOURCE_PHYSICS_SUPPORT_BINDING`); the pressure-source reader runs them
  against its invocation's model. Preview-physics-1's TypeScript reader ignores its model.
- G5: one header dispatch: the downgrade guard, a carrier namespace, the metadata header, then the
  TypeScript header's checks (the source route's receipt shape, the evidence object the pressure route
  needs, no identity outside these routes).
- pressure-2 (DEL-04-07 `pressure-contract.md`, "Reading pressure-2"): its own route, read here by the
  independent `pressure_2` verifier with G2's model checks and the ordinary standing; it has no transport
  reader, so a transported pressure-2 statement is refused by name.
- B7b (A4/A5): five routes (`pressure_source`, `preview_physics`, `pressure`, `retained_preview_physics`,
  `retained_physics`) and four readers (`preview-physics`, `pressure-source`, `source-recovery`,
  `pressure`), as Rust's `ROUTES` and `READERS`. A stored `2.0.0/exact_straight_pressure_v2` result is
  refused at the header by name (`SOURCE_EXACT_PRESSURE_V2_RETIRED`); its standing finding is
  `SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED`.

Outcomes have the Rust entries' shape: `{"ok": true, ...}` or `{"ok": false, "error": {"code", "detail"}}`.
The readers themselves are this package's verifiers. Tests only; no product import, no process.
"""
from __future__ import annotations

from collections.abc import Mapping
import json
import math
import re
from typing import Any

from . import compatibility as c
from . import physics_source, preview_physics_evidence, pressure_2, section_basis as sb, source_blocks

ROUTES = ("pressure_source", "preview_physics", "pressure", "retained_preview_physics", "retained_physics")
READERS = ("preview-physics", "pressure-source", "source-recovery", "pressure")
UNSUPPORTED = "unsupported"
SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED = "SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED"
SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED = "SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED"
LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE = "LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE_IN_T1"
RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE = "RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE"
PRESSURE_CONTRACT_ID = c.PRESSURE_CONTRACT_ID
PRESSURE_PROFILE = c.PRESSURE_PROFILE
PRESSURE_CONTRACT_SHA256 = c.PRESSURE_CONTRACT_SHA256
PRESSURE_TRANSPORT_UNSUPPORTED = c.PRESSURE_TRANSPORT_UNSUPPORTED
ROUTE_READERS = {"preview_physics": "preview-physics", "pressure_source": "pressure-source", "pressure": "pressure"}
# pf-timoshenko.md §4: the preview-physics route admits two identities; a source's binding is keyed by its
# `semantic_contract_id` (`binding_of`), and a route's default binding is its first identity here.
_IDENTITY_ROUTES = {c.PRESSURE_SOURCE_CONTRACT_ID: "pressure_source", c.PREVIEW_PHYSICS_CONTRACT_ID: "preview_physics",
                    c.PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID: "preview_physics",
                    PRESSURE_CONTRACT_ID: "pressure",
                    c.PREVIEW_PHYSICS_RETAINED_CONTRACT_ID: "retained_preview_physics", c.PHYSICS_RETAINED_CONTRACT_ID: "retained_physics"}
_BINDINGS: dict[str, str] = {}
for _identity, _route in _IDENTITY_ROUTES.items():
    _BINDINGS.setdefault(_route, _identity)
_SHA256 = {c.PRESSURE_SOURCE_CONTRACT_ID: c.PRESSURE_SOURCE_CONTRACT_SHA256,
           **{identity: pinned[0] for identity, pinned in c.PREVIEW_PHYSICS_CONTRACTS.items()},
           PRESSURE_CONTRACT_ID: PRESSURE_CONTRACT_SHA256, **{identity: pinned[0] for identity, pinned in c.RETAINED_CONTRACTS.items()}}
_TABLE_PATHS = {c.PRESSURE_SOURCE_CONTRACT_ID: c._PRESSURE_SOURCE_CONTRACT_PATH,
                **{identity: pinned[1] for identity, pinned in c.PREVIEW_PHYSICS_CONTRACTS.items()},
                PRESSURE_CONTRACT_ID: c._PRESSURE_CONTRACT_PATH, **{identity: pinned[1] for identity, pinned in c.RETAINED_CONTRACTS.items()}}
SHEAR_MODEL_IDENTITY = "SOURCE_PREVIEW_PHYSICS_SHEAR_MODEL_IDENTITY"
# An absent `model_settings.shear_model`, distinct from an authored `null`.
_ABSENT = object()
_QUALITY_STATUSES = ("not_assessed", "checks_passed", "sensitive", "unresolved", "failed")
_SUPPORT_KINDS = {"support_reaction_component_v2", "support_reaction_force_magnitude_v2", "support_reaction_moment_magnitude_v2"}
_TABLES: dict[str, Any] = {}


class Refused(ValueError):
    """A header or reader refusal: its whole text."""


def error_json(text: str) -> dict[str, Any]:
    """A refusal's text as `{code, detail}`: the leading upper-case token, and the whole text when it says more."""
    match = re.match(r"[A-Z][A-Z0-9_]*", text)
    code = match.group(0) if match else ""
    return {"code": code, "detail": None if code == text else text}


def _refusal(text: str) -> dict[str, Any]:
    return {"ok": False, "error": error_json(text)}


def _table(identity: str | None) -> Any:
    if identity == c.PRESSURE_SOURCE_CONTRACT_ID: return physics_source.pressure_source_table()
    if identity == PRESSURE_CONTRACT_ID: return pressure_2.pressure_table()
    path = _TABLE_PATHS[identity]
    key = str(path)
    if key not in _TABLES:
        _TABLES[key] = json.loads(path.read_text(encoding="utf-8"))
    return _TABLES[key]


def _present(value: Any) -> Any:
    return None if value is None else value


def _keys(value: Any, keys: set[str]) -> bool:
    return isinstance(value, Mapping) and set(value) == keys


def _text(value: Any) -> bool:
    return isinstance(value, str) and bool(value)


def _metadata_header(source: Any) -> str:
    """The metadata header (Rust `for_source_metadata`'s checks, as the Python base header reads them in Rust's
    order): the identity's route, or the refusal raised. A successor takes its own transport checks."""
    if not isinstance(source, Mapping):
        raise Refused("SOURCE_SCHEMA_VERSION_UNSUPPORTED")
    producer = source.get("producer")
    identity = producer.get("semantic_contract_id") if isinstance(producer, Mapping) else None
    if identity in c.RETAINED_CONTRACTS:
        try:
            c._retained_transport(source)
        except ValueError as error:
            raise Refused(str(error)) from error
        return _IDENTITY_ROUTES[identity]
    if "retained_precision" in source:
        raise Refused(c.RETAINED_PRECISION_DOWNGRADE_FORBIDDEN)
    version = source.get("schema_version")
    if version == "0.1.0":
        raise Refused(c.MECHANICS_RESULT_VERSION_RETIRED)
    if version != "0.2.0":
        raise Refused("SOURCE_SCHEMA_VERSION_UNSUPPORTED")
    known = set(_IDENTITY_ROUTES) - set(c.RETAINED_CONTRACTS)
    if (not _keys(producer, {"component_name", "component_version", "semantic_contract_id"}) or producer["component_name"] != "open_pipe_stress_product_physics"
            or producer["component_version"] != "0.2.0"):
        raise Refused("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
    if identity == c.RETIRED_PRECISION_CONTRACT_ID:
        raise Refused(f"{c.SOURCE_PRECISION_1_RETIRED}: precision-1 results are no longer read; re-run the analysis")
    if identity == c.RETIRED_PRESSURE_1_CONTRACT_ID:
        raise Refused(f"{c.SOURCE_PRESSURE_1_RETIRED}: pressure-1 results are no longer read; re-run the analysis")
    if identity == c.RETIRED_PREVIEW_PHYSICS_1_CONTRACT_ID:
        raise Refused(f"{c.SOURCE_PREVIEW_PHYSICS_1_RETIRED}: preview-physics-1 results are no longer read; re-run the analysis")
    if identity == c.RETIRED_SOURCE_BLOCKS_CONTRACT_ID:
        raise Refused(f"{c.SOURCE_BLOCKS_1_RETIRED}: source-blocks-1 results are no longer read; re-run the analysis")
    if identity in c.RETIRED_EXACT_V2_IDS:
        raise Refused(c.SOURCE_EXACT_PRESSURE_V2_RETIRED_TEXT)
    if not isinstance(identity, str) or identity not in known:
        raise Refused("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
    # Rust `current_header`: the load/reference-state group owns no carrier namespace.
    if isinstance(source.get("contract_evidence"), Mapping) and "load_reference_states" in source["contract_evidence"] and "carrier_evidence" in source:
        raise Refused("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
    if identity != c.PRESSURE_SOURCE_CONTRACT_ID and "source_block_recovery" in source:
        raise Refused("SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN")
    if identity in c.PREVIEW_PHYSICS_CONTRACTS and not isinstance(source.get("contract_evidence"), Mapping):
        raise Refused("SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED")
    if identity == c.PRESSURE_SOURCE_CONTRACT_ID:
        try: physics_source.SourceContract.PRESSURE_V3.check_received(source)
        except ValueError as error: raise Refused(str(error)) from error
    quality = source.get("numerical_quality")
    if (not _keys(quality, {"value_representation", "publication_quantization", "integrity_policy", "status", "cases"})
            or quality["value_representation"] != "finite_binary64" or quality["publication_quantization"] != "none"
            or quality["integrity_policy"] != "M03-INTEGRITY-v1" or not isinstance(quality["status"], str)
            or quality["status"] not in _QUALITY_STATUSES or not isinstance(quality["cases"], list)):
        raise Refused("SOURCE_NUMERICAL_QUALITY_INVALID")
    for case in quality["cases"]:
        basis = case.get("basis_ref") if isinstance(case, Mapping) else None
        if (not _keys(case, {"basis_ref", "structural_status", "solve_quality", "model_matrix_fidelity", "accuracy_evidence", "evidence_refs"})
                or not isinstance(case["solve_quality"], str) or case["solve_quality"] not in _QUALITY_STATUSES
                or case["structural_status"] not in ("passive_model_basis", "physical_mechanism_witnessed", "negative_energy_witnessed", "numerically_unresolved")
                or case["model_matrix_fidelity"] not in ("represented_equations_retained", "assembly_loss_detected", "assembly_uncertainty", "not_assessed")
                or case["accuracy_evidence"] not in ("not_claimed", "reference_verified", "unresolved")
                or not isinstance(case["evidence_refs"], list) or not all(_text(item) for item in case["evidence_refs"])
                or not _keys(basis, {"ref_type", "ref_id"}) or not _text(basis["ref_type"]) or not _text(basis["ref_id"])):
            raise Refused("SOURCE_NUMERICAL_CASE_INVALID")
    profile = {PRESSURE_CONTRACT_ID: PRESSURE_PROFILE, c.PRESSURE_SOURCE_CONTRACT_ID: PRESSURE_PROFILE,
               **{i: pinned[2] for i, pinned in c.PREVIEW_PHYSICS_CONTRACTS.items()}}.get(identity, c.PREVIEW_PHYSICS_PROFILE)
    formulation = source.get("formulation_basis")
    if (not _keys(formulation, {"profile_id", "limitations"}) or formulation["profile_id"] != profile
            or not isinstance(formulation["limitations"], list) or not formulation["limitations"] or not all(_text(item) for item in formulation["limitations"])):
        raise Refused("SOURCE_FORMULATION_BASIS_UNSUPPORTED")
    return _IDENTITY_ROUTES[identity]


def route(source: Any) -> str:
    """G5: the header route, or the refusal raised (Refused, its text)."""
    if isinstance(source, Mapping) and not c._is_retained(source):
        if "retained_precision" in source or c._has_retained_rows(source):
            raise Refused(c.RETAINED_PRECISION_DOWNGRADE_FORBIDDEN)
    if isinstance(source, Mapping) and "carrier_evidence" in source:
        raise Refused(SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED)
    name = _metadata_header(source)
    evidence_object = isinstance(source.get("contract_evidence"), Mapping) if isinstance(source, Mapping) else False
    receipt = source.get("source_block_recovery") if isinstance(source, Mapping) else None
    try:
        # Rust `route`: as the retired load-reference-source-1 route did, the composite needs its evidence
        # object before its receipt is read.
        if name == "pressure_source" and not evidence_object:
            raise Refused(SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED)
        if name == "pressure_source":
            physics_source.validate_receipt_shape(receipt, source_contract=physics_source.SourceContract.PRESSURE_V3)
        if name == "pressure" and not evidence_object:
            raise Refused(SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED)
    except Refused:
        raise
    except ValueError as error:
        raise Refused(str(error)) from error
    return name


def binding(name: str) -> dict[str, str] | None:
    identity = _BINDINGS.get(name)
    return None if identity is None else {"id": identity, "sha256": _SHA256[identity]}


def binding_of(name: str, source: Any) -> dict[str, str] | None:
    """A routed source's binding, keyed by its `semantic_contract_id` within its route (Rust `binding_of`)."""
    producer = source.get("producer") if isinstance(source, Mapping) else None
    identity = producer.get("semantic_contract_id") if isinstance(producer, Mapping) else None
    if not isinstance(identity, str) or _IDENTITY_ROUTES.get(identity) != name:
        return None
    return {"id": identity, "sha256": _SHA256[identity]}


def bind_preview_shear_model(source: Mapping[str, Any], model: Mapping[str, Any]) -> None:
    """pf-timoshenko.md §4 (model-bound; Rust `bind_preview_shear_model`): the identity the model's
    `model_settings.shear_model` selects. An absent member (or absent `model_settings`) or `euler_bernoulli`
    selects preview-physics-2, `timoshenko_annulus_energy_v1` the Timoshenko identity; a crossed pair is
    refused. A setting the producer and validator refuse is refused naming their code: a `model_settings`
    that is not an object, `null` included (MODEL-SLOT-SHAPE), and a `shear_model` that is not one of the
    two names, `null` and non-strings included (MODEL-SETTING-SHEAR-MODEL-UNKNOWN)."""
    selected: Any = _ABSENT
    if isinstance(model, Mapping) and "model_settings" in model:
        settings = model["model_settings"]
        if not isinstance(settings, Mapping):
            raise Refused(f"{SHEAR_MODEL_IDENTITY}: MODEL-SLOT-SHAPE: model_settings is not an object, so it selects no beam theory")
        selected = settings.get("shear_model", _ABSENT)
    if selected is _ABSENT or selected == "euler_bernoulli":
        expected = c.PREVIEW_PHYSICS_CONTRACT_ID
    elif selected == "timoshenko_annulus_energy_v1":
        expected = c.PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID
    else:
        raise Refused(f"{SHEAR_MODEL_IDENTITY}: MODEL-SETTING-SHEAR-MODEL-UNKNOWN: model_settings.shear_model must be "
                      "`euler_bernoulli` or `timoshenko_annulus_energy_v1`")
    producer = source.get("producer") if isinstance(source, Mapping) else None
    if not isinstance(producer, Mapping) or producer.get("semantic_contract_id") != expected:
        raise Refused(f"{SHEAR_MODEL_IDENTITY}: the result's semantic contract is not the one the model's shear model selects ({expected})")


def _model_section_bound(section: Any, segments: list[Any]) -> bool:
    """Rust `reader_verdict::model_section_bound` (T4-U6, D-5 §1, §2.5, §5): one ``pipe_sections`` entry's captured
    authored values are the model's section of the same pipe, bit for bit, as pressure-source-1 binds them against its
    invocation (``physics_source._actual_materials``): ``outside_diameter_m``, ``wall_thickness_m``,
    ``mill_tolerance_m`` (0.0 where the model's section has none) and ``corrosion_allowance_m``, each the model's
    quantity in metres by the verifier's unit catalog (``units.to_canonical``). A signed zero is not its other zero. A
    pipe the model lacks, a quantity it lacks (the allowance is required) or one that does not normalize to a finite
    length is unbound."""
    from .units import to_canonical

    if not isinstance(section, Mapping):
        return False
    pipe = next((p for p in segments if isinstance(p, Mapping) and p.get("id") == section.get("pipe_id")), None)
    authored = pipe.get("section") if isinstance(pipe, Mapping) else None
    if not isinstance(authored, Mapping):
        return False

    def length(quantity: Any) -> float | None:
        if not isinstance(quantity, Mapping):
            return None
        value, unit = quantity.get("value"), quantity.get("unit")
        if type(value) not in (int, float) or not math.isfinite(value) or not isinstance(unit, str) or not unit:
            return None
        try:
            return to_canonical(value, unit, "length")
        except (ValueError, OverflowError):
            return None

    mill_tolerance = 0.0 if authored.get("mill_tolerance") is None else length(authored["mill_tolerance"])
    expected = (("outside_diameter_m", length(authored.get("outside_diameter"))), ("wall_thickness_m", length(authored.get("wall_thickness"))),
                ("mill_tolerance_m", mill_tolerance), ("corrosion_allowance_m", length(authored.get("corrosion_allowance"))))
    return all(model is not None and type(section.get(key)) in (int, float) and math.isfinite(section[key]) and sb.bits(section[key]) == sb.bits(model)
               for key, model in expected)


def physics_model_checks(source: Mapping[str, Any], model: Mapping[str, Any]) -> None:
    """G2: TS `validateKnownPhysicsEvidence`'s model checks, after the unchanged reader admitted the evidence."""
    same = lambda a, b: len(a) == len(b) and all(item in b for item in a)
    ids = lambda items, key: [item.get(key) if isinstance(item, Mapping) else None for item in items] if isinstance(items, list) else []
    evidence = source.get("contract_evidence") or {}
    cases = evidence.get("exact_cases") if isinstance(evidence.get("exact_cases"), list) else []
    if (source.get("status") or {}).get("mechanics") == "MECHANICS_SOLVED":
        if not same([case.get("load_case_id") for case in cases], ids(model.get("load_cases"), "id")):
            raise Refused("SOURCE_PHYSICS_MODEL_CASE_COVERAGE")
    if model.get("pipe_segments") is not None:
        replaced = ids(evidence.get("connector"), "replaced_pipe_id")
        pipes = [pipe for pipe in ids(model["pipe_segments"], "id") if pipe not in replaced]
        for case in cases:
            if not same(ids(case.get("pipe_materials"), "pipe_id"), pipes):
                raise Refused("SOURCE_PHYSICS_MODEL_MEMBER_COVERAGE")
        # T4-U6 (§1, §5; FINAL-B): the evidence reader rederived each case section's three geometries from its captured
        # (D, t, c_m, c_a); those are the model's, bit for bit.
        segments = model["pipe_segments"] if isinstance(model["pipe_segments"], list) else []
        for case in cases:
            sections = case.get("pipe_sections") if isinstance(case, Mapping) else None
            for section in sections if isinstance(sections, list) else []:
                if not _model_section_bound(section, segments):
                    raise Refused("SOURCE_PHYSICS_MODEL_SECTION_GEOMETRY")
    # T4-U8 (§3.6, §5): every section entry states the beam theory the model's contract selects for its member (a
    # realized arc, Euler-Bernoulli).
    if model.get("pressure_contract") is not None:
        try:
            sb.bind_shear_model(evidence, model)
        except sb.BasisError as error:
            raise Refused(str(error)) from error
    if model.get("supports") is not None:
        supports = ids(model["supports"], "id")
        for row in source.get("results") or []:
            if row.get("kind") in _SUPPORT_KINDS and row.get("entity_ref") not in supports:
                raise Refused("SOURCE_PHYSICS_SUPPORT_BINDING")


def _source_validate(source: Any, invocation: Any, context=physics_source.SourceContract.PRESSURE_V3) -> bool:
    eligible = physics_source.validate_physics_source(source, invocation, source_contract=context)
    if invocation is not None:
        physics_model_checks(source, invocation["request"]["model"])
    return eligible


def source_recovery(source: Any) -> None:
    """Rust `reader_verdict::source_recovery`: the portable receipt/publication consistency the record and export
    builders read: the header route (pressure-source-1 only), its transport metadata and the publication
    commitment. No raw rows, invocation or eligibility."""
    try:
        name = route(source)
    except Refused:
        name = None
    if name != "pressure_source":
        raise Refused("SOURCE_RECOVERY_METADATA_CONTRADICTION")
    physics_source.validate_transport_metadata(source, source_contract=physics_source.SourceContract.PRESSURE_V3)
    publication = {key: value for key, value in source.items() if key != "source_block_recovery"}
    if source["source_block_recovery"]["body"]["publication_sha256"] != source_blocks.domain_hash("source_blocks_publication_v1", publication):
        raise Refused("SOURCE_BLOCKS_PUBLICATION_HASH")


def validate(reader: str, source: Any, model: Any = None, invocation: Any = None) -> dict[str, Any]:
    """One reader's raw validation: `{"ok": true}` (with `eligible`, and the source reader's G1 `findings`) or the refusal."""
    model, invocation = _present(model), _present(invocation)
    try:
        if reader == "source-recovery":
            source_recovery(source)
            return {"ok": True}
        if reader == "preview-physics":
            preview_physics_evidence.validate_preview_physics_evidence(source)
            if model is not None:
                # pf-timoshenko.md §4: with the model, a crossed identity is refused first.
                bind_preview_shear_model(source, model)
                # T4-U4b (§5.1): τ bit for bit with the captured model (G2).
                from . import u4b_model
                u4b_model.preview_model_checks(source, model)
                # W-15 slice d (DEL-03-05 Design §5): the model's bodies and their records.
                from . import component_body
                component_body.model_checks(source, model, preview_physics_evidence.TABLE)
                # Operating-case slice 1a (§3): the operand records bound to the model's combinations.
                from . import preview_combination_operands
                preview_combination_operands.model_checks(source, model)
            return {"ok": True}
        if reader == "pressure":
            pressure_2.validate_pressure_evidence(source)
            if model is not None:
                physics_model_checks(source, model)
                # T4-U4b review item 1: the directional binding (the pressure-2 reader only).
                from . import u4b
                u4b.directional_model_checks(source, model, True)
                # W-15 slice d (DEL-03-05 Design §5): the model's bodies and their records.
                from . import component_body
                component_body.model_checks(source, model, pressure_2.pressure_table())
                # RT-A3 increment 1: the branch rows bound to the model's tee associations (Rust `exact_model_checks`).
                u4b.branch_model_checks(source, model)
                # W-16 slice b (§6): the combination records bound to the model.
                from . import exact_combinations
                exact_combinations.model_checks(source, model)
            return {"ok": True}
        if reader == "pressure-source":
            eligible = _source_validate(source, invocation)
            return {"ok": True, "eligible": eligible, "findings": [] if invocation is None else source_findings(source, invocation)}
    except ValueError as error:
        return _refusal(str(error))
    return _refusal(f"READER_UNKNOWN: {reader}")


def transport(reader: str, source: Any) -> dict[str, Any]:
    """One reader's transport (metadata-only) checks; never eligible."""
    checks = {"preview-physics": preview_physics_evidence.validate_transport_metadata,
              "pressure-source": lambda source: physics_source.validate_transport_metadata(source, source_contract=physics_source.SourceContract.PRESSURE_V3)}
    if reader == "pressure":
        return _refusal(PRESSURE_TRANSPORT_UNSUPPORTED)
    if reader not in checks:
        return _refusal(f"READER_UNKNOWN: {reader}")
    try:
        checks[reader](source)
    except ValueError as error:
        return _refusal(str(error))
    return {"ok": True}


def _semantic(table: Mapping[str, Any], row: Mapping[str, Any]) -> Mapping[str, Any] | None:
    """TS `sourceBlockRecovery.ts`'s `semantic`: the first table row of this kind and unit whose component is null
    or the row's, and whose source_basis (if any) is the row's."""
    metadata = row.get("metadata") if isinstance(row.get("metadata"), Mapping) else {}
    for signature in table["rows"]:
        if (signature["kind"] == row.get("kind") and signature["unit"] == row.get("unit")
                and (signature["component"] is None or signature["component"] == metadata.get("component"))
                and ("source_basis" not in signature or signature["source_basis"] == metadata.get("basis"))):
            return signature
    return None


def source_findings(source: Mapping[str, Any], invocation: Mapping[str, Any]) -> list[str]:
    """G1: TS `validateKnownSourceRecovery`'s findings: per case in order, an unqualified case and each
    inspection-only row of a physical (or unknown) kind; then an envelope not qualified, or whose model has
    combinations; then an unsolved envelope. Each code once, first place kept."""
    table = _table(c.PRESSURE_SOURCE_CONTRACT_ID)
    rows = {row["id"]: row for row in source.get("results") or [] if isinstance(row, Mapping) and isinstance(row.get("id"), str)}
    def physical(row: Mapping[str, Any]) -> bool:
        semantic = _semantic(table, row)
        return semantic is None or semantic["category"] == "physical_quantity"
    ordinary = "ordinary_physics_checked"
    findings: list[str] = []
    body = source["source_block_recovery"]["body"]
    for case in body.get("cases") or []:
        if case.get("outcome") != "qualified":
            findings.append("SOURCE_BLOCKS_CASE_UNQUALIFIED")
        for treatment in case.get("rows") or []:
            if treatment.get("treatment") in ("qualified_projection", ordinary, "checked_derived"):
                continue
            row = rows.get(treatment.get("result_id"))
            # T4-U4b §4.3: `derived_not_qualified` is neutral for the two derived kinds only.
            if treatment.get("treatment") == "derived_not_qualified" and row is not None and row.get("kind") in (
                    "element_local_normal_stress_extreme", "element_local_transverse_shear_stress_maximum"):
                continue
            if row is not None and physical(row):
                findings.append("SOURCE_BLOCKS_PHYSICAL_ROW_INSPECTION_ONLY")
    combinations = invocation["request"]["model"].get("combinations")
    if body.get("status") != "qualified" or (isinstance(combinations, list) and combinations):
        findings.append("SOURCE_BLOCKS_ENVELOPE_UNQUALIFIED")
    if (source.get("status") or {}).get("mechanics") != "MECHANICS_SOLVED":
        findings.append("MECHANICS_NOT_SOLVED")
    return list(dict.fromkeys(findings))


def _ordinary_findings(source: Mapping[str, Any], requested: list[Any]) -> list[str]:
    """TS `numericalResultStanding`'s generic branch over `numerical_quality`, in its order."""
    quality = source["numerical_quality"]
    cases = quality["cases"]
    findings = []
    if quality["status"] != "checks_passed":
        findings.append("NUMERICAL_INTEGRITY_NOT_QUALIFIED")
    results, diagnostics = source.get("results") or [], source.get("diagnostics") or []
    def truthy(value: Any) -> bool:
        return not (value is None or value is False or value == "" or (type(value) in (int, float) and value == 0))
    emitted_ids = [row.get("id") for row in results] + [d.get("id") for d in diagnostics if truthy(d.get("id"))]
    emitted = []
    for item in emitted_ids:
        if item not in emitted:
            emitted.append(item)
    if any(not _text(row.get("id")) for row in results) or any(not _text(d.get("id")) for d in diagnostics) or len(emitted) != len(emitted_ids):
        findings.append("NUMERICAL_EVIDENCE_ID_AMBIGUOUS")
    distinct = []
    for item in requested:
        if item not in distinct:
            distinct.append(item)
    if not requested or len(distinct) != len(requested):
        findings.append("REQUESTED_NUMERICAL_BASIS_UNAVAILABLE")
    def matching(identifier: Any) -> int:
        return sum(1 for case in cases if (case.get("basis_ref") or {}).get("ref_type") == "load_case" and case["basis_ref"].get("ref_id") == identifier)
    if len(cases) != len(requested) or any(matching(identifier) != 1 for identifier in requested):
        findings.append("NUMERICAL_CASE_COVERAGE_INCOMPLETE")
    def eligible(case: Any) -> bool:
        refs = case.get("evidence_refs") if isinstance(case, Mapping) else None
        return (isinstance(case, Mapping) and case.get("structural_status") == "passive_model_basis" and case.get("solve_quality") == "checks_passed"
                and case.get("model_matrix_fidelity") == "represented_equations_retained" and case.get("accuracy_evidence") in ("not_claimed", "reference_verified")
                and isinstance(refs, list) and bool(refs) and all(ref in emitted for ref in refs))
    if any(not eligible(case) for case in cases):
        findings.append("NUMERICAL_CASE_EVIDENCE_INCOMPLETE")
    order = ("checks_passed", "sensitive", "not_assessed", "unresolved", "failed")
    aggregate = order[max([order.index(case["solve_quality"]) for case in cases if case.get("solve_quality") in order] + [0])] if cases else "not_assessed"
    if quality["status"] != aggregate:
        findings.append("NUMERICAL_AGGREGATE_CONTRADICTION")
    return findings


def standing(source: Any, model: Any = None, invocation: Any = None) -> dict[str, Any]:
    """G1: `{route, token, status, eligible, findings}`. `token` is the verifier's numerical-use standing with the
    model's load cases as the requested refs and the caller's invocation."""
    model, invocation = _present(model), _present(invocation)
    requested_ids = [case.get("id") for case in model["load_cases"]] if isinstance(model, Mapping) and isinstance(model.get("load_cases"), list) else []
    token = c.numerical_use_standing(source, [{"ref_type": "load_case", "ref_id": identifier} for identifier in requested_ids], invocation)
    findings: list[str] = []
    try:
        name = route(source)
    except Refused as error:
        code = str(error)
        findings.append(code if code in {c.RETAINED_PRECISION_DOWNGRADE_FORBIDDEN, "SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN", c.MECHANICS_RESULT_VERSION_RETIRED} else SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED)
        name = UNSUPPORTED
    def done(findings: list[str]) -> dict[str, Any]:
        eligible = not findings
        return {"route": name, "token": token, "status": "integrity_checked" if eligible else "needs_recompute", "eligible": eligible, "findings": findings}
    if name == "pressure_source":
        if invocation is None:
            findings.append(SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED)
        else:
            try:
                _source_validate(source, invocation)
            except ValueError as error:
                findings.append(str(error))
            else:
                findings += source_findings(source, invocation)
                if "load_reference_states" in source["contract_evidence"]:
                    findings.append(LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE)
    elif name in ("retained_preview_physics", "retained_physics"):
        if token != "numerically_eligible":
            from .retained_precision import RetainedPrecisionError, validate_retained_precision
            try:
                validate_retained_precision(source, invocation)
                findings.append(RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE)
            except RetainedPrecisionError as error:
                findings.append(error.code)
        return done(findings)
    elif name != UNSUPPORTED:
        reader = {"preview_physics": "preview-physics", "pressure": "pressure"}.get(name)
        if reader is not None:
            outcome = validate(reader, source, model, None)
            if outcome["ok"] is not True:
                findings.append(outcome["error"]["detail"] or outcome["error"]["code"])
        findings += _ordinary_findings(source, requested_ids)
    if ((source.get("status") or {}) if isinstance(source, Mapping) else {}).get("mechanics") != "MECHANICS_SOLVED":
        findings.append("MECHANICS_NOT_SOLVED")
    return done(findings)


def dispatch(source: Any, model: Any = None, invocation: Any = None) -> dict[str, Any]:
    """The dispatch entry: the route, its table binding, the header refusal (or None) and the standing."""
    try:
        name, refusal = route(source), None
    except Refused as error:
        name, refusal = UNSUPPORTED, error_json(str(error))
    return {"ok": refusal is None, "route": name, "binding": binding_of(name, source), "error": refusal, "standing": standing(source, model, invocation)}


def signature(table: Mapping[str, Any], row: Mapping[str, Any]) -> Mapping[str, Any] | None:
    """TS `resultSemantics`: the row's table signature, None for an unknown kind; a contradiction raises."""
    kind = row.get("kind")
    if not isinstance(kind, str):
        raise Refused("SOURCE_KIND_MISSING")
    known = [entry for entry in table["rows"] if entry["kind"] == kind]
    if not known:
        return None
    units = [entry for entry in known if entry["unit"] == row.get("unit")]
    if not units:
        raise Refused(f"SOURCE_UNIT_CONTRADICTION: {kind}")
    observed = (row.get("metadata") or {}).get("component") if isinstance(row.get("metadata"), Mapping) else None
    component = observed if isinstance(observed, str) and observed else None
    variants = [entry for entry in units if entry["component"] is None or entry["component"] == component]
    basis = (row.get("metadata") or {}).get("basis") if isinstance(row.get("metadata"), Mapping) else None
    exact = next((entry for entry in variants if "source_basis" not in entry or entry["source_basis"] == basis), None)
    if exact is not None:
        return exact
    if variants:
        raise Refused(f"SOURCE_BASIS_CONTRADICTION: {kind}")
    if component:
        raise Refused(f"SOURCE_COMPONENT_CONTRADICTION: {kind}")
    return units[0]


def complete_metadata(row: Mapping[str, Any]) -> bool:
    metadata = row.get("metadata")
    return isinstance(metadata, Mapping) and all(_text(metadata.get(key)) for key in ("component", "coordinate_system", "location", "basis", "sign_convention"))


def canonical_metadata(table: Mapping[str, Any], row: Mapping[str, Any]) -> dict[str, Any] | None:
    if not complete_metadata(row):
        return None
    projection = {}
    for key, rule in table["canonical_metadata_vocabulary"].items():
        value = row["metadata"].get(key)
        if "enum" in rule and value not in rule["enum"]:
            return None
        projection[key] = value
    return projection


def row_semantics(row: Any, source: Any = None) -> dict[str, Any]:
    """One row's semantics under its source's table; no source is refused."""
    if source is None:
        return _refusal("SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED")
    try:
        name = route(source)
    except Refused:
        return _refusal("SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED")
    table = _table((binding_of(name, source) or {"id": _BINDINGS.get(name)})["id"])
    try:
        found = signature(table, row)
    except Refused as error:
        return _refusal(str(error))
    return {"ok": True, "signature": found, "complete_metadata": complete_metadata(row), "canonical_metadata": canonical_metadata(table, row)}
