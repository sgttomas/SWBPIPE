"""SI CAEPIPE MBF text writer for the pilot subset (DEL-17-04).

`render_caepipe_mbf_si` turns the SWBPIPE headless preview model (the
`solve.preview_model.model` object, model document 0.7.0) into MBF text in the
SI unit set (mm, N, N*m, MPa, C), following the public CAEPIPE User's Manual
appendix "Import MBF". Only a narrow pilot subset is written; everything else
is refused by name and listed in the loss report, never silently dropped.

Units are never converted between unit systems: a project or quantity unit
outside the SI set whose scaling to the MBF unit is an exact power of ten is
refused. The only non-decimal conversions are those stated in the result's
`assumptions` (radians to degrees, weight per length to weight density by
standard gravity, Poisson ratio from E and G).

The writer does not run CAEPIPE and makes no claim about how CAEPIPE reads or
analyses the text.
"""

from __future__ import annotations

from dataclasses import dataclass, field
import math
from typing import Any, Mapping


MBF_SI_PROFILE_ID = "caepipe-mbf-si-pilot-1"
STANDARD_GRAVITY = 9.80665  # m/s^2; converts SWBPIPE weight per length to CAEPIPE weight density only.

MBF_VERSION = "15.10"
# The target tool's labels for the load cases the writer requests.
LOAD_CASE_LABELS = {"sustained": "Sustained (W+P)", "expansion": "Expansion (T1)"}
DOFS = ("UX", "UY", "UZ", "RX", "RY", "RZ")
_DOF_TOKENS = {
    **{dof: dof for dof in DOFS},
    **{dof[0] + dof[1].lower(): dof for dof in DOFS},
    **{dof.lower(): dof for dof in DOFS},
}
_FORCE_DIRECTIONS = {"global_x": 0, "GLOBAL_X": 0, "global_y": 1, "GLOBAL_Y": 1, "global_z": 2, "GLOBAL_Z": 2}
_MOMENT_DIRECTIONS = {
    **{token: DOFS.index(dof) - 3 for token, dof in _DOF_TOKENS.items() if dof.startswith("R")},
    "rotation_x": 0,
    "rotation_y": 1,
    "rotation_z": 2,
}
_RIGID_FAMILIES = {None, "anchor", "guide", "line_stop", "vertical_support"}
_MECHANICAL_CATEGORIES = {"concentrated_force", "concentrated_moment", "weight"}
_THERMAL_CATEGORIES = {"thermal", "imposed_displacement"}

# Accepted SWBPIPE unit symbols (core/units/src/lib.rs) and the power of ten
# that scales each to the MBF SI unit. "C" is the units crate's legacy alias
# of degC for temperature dimensions (unit_by_symbol). The rotation table's
# "rad" entry is the one non-decimal conversion (radians to degrees).
_LENGTH_TO_MM = {"m": 3, "mm": 0}
_STRESS_TO_MPA = {"Pa": -6, "kPa": -3, "MPa": 0, "GPa": 3}
_FORCE_TO_N = {"N": 0, "kN": 3}
_MOMENT_TO_NM = {"N*m": 0, "kN*m": 3}
_FORCE_PER_LENGTH_TO_N_PER_M = {"N/m": 0, "N/mm": 3, "kN/m": 3, "kN/mm": 6}
_EXPANSION_TO_PER_C = {"1/K": 0, "1/degC": 0}
_TEMPERATURE_INTERVAL_TO_C = {"K": 0, "degC": 0, "C": 0}
_ROTATION_TO_DEG = {"rad": "rad", "deg": 0}
_DIMENSIONLESS = {"1": 0, "none": 0}
_BEND_SOLVER_CONSUMPTION = "curved_bend_macro_element"
_BEND_RELATIVE_TOLERANCE = 1e-9  # tangent lengths, line intersection, bend angle and bend plane
_PROJECT_UNITS = {"length": ("m", "mm"), "force": ("N",), "temperature": ("degC",)}

_TITLE_MAX = 70
_DESCRIPTION_MAX = 30
_MAX_NODES = 9999  # node numbers 10, 20, ... stay below 99999


@dataclass(frozen=True)
class MbfSiResult:
    status: str
    text: str | None
    diagnostics: list[dict[str, Any]] = field(default_factory=list)
    node_map: dict[str, str] = field(default_factory=dict)
    element_map: dict[str, dict[str, Any]] = field(default_factory=dict)
    support_map: dict[str, dict[str, Any]] = field(default_factory=dict)
    load_case_map: dict[str, str] = field(default_factory=dict)
    material_map: dict[str, dict[str, Any]] = field(default_factory=dict)
    section_map: dict[str, dict[str, Any]] = field(default_factory=dict)
    load_map: dict[str, dict[str, Any]] = field(default_factory=dict)
    layout_row_count: int = 0
    loss_report: list[dict[str, Any]] = field(default_factory=list)
    assumptions: list[str] = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        return {
            "profile_id": MBF_SI_PROFILE_ID,
            "status": self.status,
            "text": self.text,
            "diagnostics": [dict(item) for item in self.diagnostics],
            "node_map": dict(self.node_map),
            "element_map": {key: dict(value) for key, value in self.element_map.items()},
            "support_map": {key: dict(value) for key, value in self.support_map.items()},
            "load_case_map": dict(self.load_case_map),
            "material_map": {key: dict(value) for key, value in self.material_map.items()},
            "section_map": {key: dict(value) for key, value in self.section_map.items()},
            "load_map": {key: dict(value) for key, value in self.load_map.items()},
            "layout_row_count": self.layout_row_count,
            "loss_report": [dict(item) for item in self.loss_report],
            "assumptions": list(self.assumptions),
        }


def render_caepipe_mbf_si(
    model: Mapping[str, Any],
    *,
    title: str,
    reference_temperature_c: float = 20.0,
    max_layout_rows: int | None = 20,
) -> MbfSiResult:
    """Render the pilot subset of a SWBPIPE preview model as SI MBF text, or refuse by name."""

    return _Writer(model, title, reference_temperature_c, max_layout_rows).run()


class _Writer:
    def __init__(self, model: Any, title: str, t_ref: Any, max_layout_rows: int | None) -> None:
        self.model = model
        self.title = title
        self.t_ref = t_ref
        self.max_layout_rows = max_layout_rows
        self.diagnostics: list[dict[str, Any]] = []
        self.loss: list[dict[str, Any]] = []
        self.assumptions: list[str] = []
        self.length_exp = 0
        self.nodes: dict[str, tuple[float, float, float]] = {}
        self.materials: dict[str, dict[str, Any]] = {}
        self.pipes: dict[str, dict[str, Any]] = {}
        self.node_restraints: dict[str, set[str]] = {}
        self.node_supports: dict[str, list[str]] = {}
        self.supports: dict[str, dict[str, Any]] = {}
        self.case_labels: dict[str, str] = {}
        self.forces: dict[str, list[float]] = {}
        self.moments: dict[str, list[float]] = {}
        self.weight: dict[str, float] = {}
        self.delta_t: dict[str, float] = {}
        self.anchor_disp: dict[str, list[float]] = {}
        self.anchor_rot: dict[str, list[float]] = {}
        self.load_records: dict[str, dict[str, Any]] = {}
        self.bends: dict[str, dict[str, Any]] = {}
        self.tangent_points: dict[str, str] = {}
        self.rotation_converted = False

    # -- diagnostics -----------------------------------------------------
    def diag(self, code: str, severity: str, entity: str, message: str) -> None:
        self.diagnostics.append({"code": code, "severity": severity, "entity": entity, "message": message})

    def refuse(self, code: str, entity: str, message: str) -> None:
        self.diag(code, "error", entity, message)
        self.loss.append({"entity": entity, "category": "unsupported", "code": code, "reason": message})

    def omit(self, code: str, entity: str, message: str) -> None:
        self.diag(code, "warning", entity, message)
        self.loss.append({"entity": entity, "category": "omitted", "code": code, "reason": message})

    def invalid(self, entity: str, message: str) -> None:
        self.diag("MBF-SI-INPUT-INVALID", "error", entity, message)

    def has_errors(self) -> bool:
        return any(item["severity"] == "error" for item in self.diagnostics)

    # -- value helpers ---------------------------------------------------
    def number(self, value: Any, entity: str, name: str) -> float | None:
        if isinstance(value, bool) or not isinstance(value, (int, float)):
            self.invalid(entity, f"{entity} {name} must be a number (got {value!r})")
            return None
        if not math.isfinite(value):
            self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", entity, f"{entity} {name} is not finite ({value!r})")
            return None
        return float(value)

    def quantity(self, record: Any, table: Mapping[str, Any], target: str, entity: str, name: str) -> float | None:
        if not isinstance(record, Mapping) or "value" not in record or "unit" not in record:
            self.invalid(entity, f"{entity} {name} must be a {{value, unit}} quantity")
            return None
        unit = record["unit"]
        value = self.number(record["value"], entity, name)
        if value is None:
            return None
        if not isinstance(unit, str) or unit not in table:
            self.refuse(
                "MBF-SI-UNIT-UNSUPPORTED",
                entity,
                f"{entity} {name} unit {unit!r} is refused: only SI units with an exact power-of-ten "
                f"scaling to the MBF unit {target} are written ({', '.join(table)}); no conversion is made",
            )
            return None
        scaling = table[unit]
        if scaling == "rad":
            self.rotation_converted = True
            return math.degrees(value)
        return _scale(value, scaling)

    # -- run -------------------------------------------------------------
    def run(self) -> MbfSiResult:
        if not isinstance(self.model, Mapping):
            self.invalid("model", "model must be an object")
            return self.refused()
        if isinstance(self.t_ref, bool) or not isinstance(self.t_ref, (int, float)) or not math.isfinite(self.t_ref):
            self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", "reference_temperature_c",
                      f"reference temperature must be a finite number (got {self.t_ref!r})")
            return self.refused()
        self.t_ref = float(self.t_ref)
        self.check_document()
        self.check_project_units()
        if self.has_errors():
            return self.refused()
        self.read_nodes()
        self.read_materials()
        self.read_pipes()
        self.read_components()
        self.read_supports()
        self.read_load_cases()
        self.read_other_members()
        if self.has_errors():
            return self.refused()
        self.resolve_bends()
        if self.has_errors():
            return self.refused()
        rows = self.layout()
        if self.has_errors():
            return self.refused()
        return self.render(rows)

    def refused(self) -> MbfSiResult:
        return MbfSiResult(
            status="refused",
            text=None,
            diagnostics=self.diagnostics,
            loss_report=self.loss,
            assumptions=self.assumptions,
        )

    def check_document(self) -> None:
        version = self.model.get("schema_version")
        if version != "0.7.0":
            self.diag("MBF-SI-SCHEMA-VERSION", "warning", "model.schema_version",
                      f"model schema_version {version!r} is not 0.7.0, the version this writer reads")
        if self.model.get("pressure_contract") is not None:
            self.refuse("MBF-SI-PRESSURE-UNSUPPORTED", "model.pressure_contract",
                        "pressure_contract is present; pressure is not exported by the pilot writer")
        if self.model.get("shear_model") is not None:
            self.refuse("MBF-SI-SHEAR-MODEL-UNSUPPORTED", "model.shear_model", "shear_model is not exported")
        if self.model.get("reference_configurations"):
            self.refuse("MBF-SI-REFERENCE-CONFIGURATION-UNSUPPORTED", "model.reference_configurations",
                        "reference configurations are not exported by the pilot writer")

    def check_project_units(self) -> None:
        project = self.model.get("project")
        units = project.get("units") if isinstance(project, Mapping) else None
        if not isinstance(units, Mapping):
            units = {}
        for key, accepted in _PROJECT_UNITS.items():
            unit = units.get(key)
            if unit not in accepted:
                self.refuse(
                    "MBF-SI-PROJECT-UNIT-UNSUPPORTED",
                    f"project.units.{key}",
                    f"project.units.{key} is {unit!r}; the SI MBF writer accepts only {' or '.join(accepted)} "
                    "and does not convert other unit systems",
                )
        if units.get("length") in _LENGTH_TO_MM:
            self.length_exp = _LENGTH_TO_MM[units["length"]]

    # -- model members ----------------------------------------------------
    def read_nodes(self) -> None:
        for node in _list(self.model.get("nodes"), self, "model.nodes"):
            if not isinstance(node, Mapping) or not isinstance(node.get("id"), str):
                self.invalid("model.nodes", "each node needs a string id")
                continue
            entity = f"node:{node['id']}"
            if node["id"] in self.nodes:
                self.invalid(entity, f"node id {node['id']!r} is repeated")
                continue
            position = node.get("position")
            if not isinstance(position, Mapping):
                self.invalid(entity, f"{entity} position must be an object with x, y, z")
                continue
            coords = [self.number(position.get(axis), entity, f"position.{axis}") for axis in "xyz"]
            if any(value is None for value in coords):
                continue
            self.nodes[node["id"]] = tuple(_scale(value, self.length_exp) for value in coords)  # type: ignore[misc]

    def read_materials(self) -> None:
        for material in _list(self.model.get("materials"), self, "model.materials"):
            if not isinstance(material, Mapping) or not isinstance(material.get("id"), str):
                self.invalid("model.materials", "each material needs a string id")
                continue
            entity = f"material:{material['id']}"
            if material["id"] in self.materials:
                self.invalid(entity, f"material id {material['id']!r} is repeated")
                continue
            ok = True
            if material.get("temperature_points"):
                self.refuse("MBF-SI-TEMPERATURE-DEPENDENT-MATERIAL-UNSUPPORTED", entity,
                            f"{entity} has temperature_points; temperature-dependent properties are not exported")
                ok = False
            if "expansion_laws" in material:
                self.refuse("MBF-SI-TEMPERATURE-DEPENDENT-MATERIAL-UNSUPPORTED", entity,
                            f"{entity} has expansion_laws; material expansion laws are not exported")
                ok = False
            e_mpa = self.quantity(material.get("elastic_modulus"), _STRESS_TO_MPA, "N/mm2", entity, "elastic_modulus")
            if e_mpa is not None and e_mpa <= 0:
                self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", entity, f"{entity} elastic_modulus must be positive")
                e_mpa = None
            g_mpa = None
            if material.get("shear_modulus") is not None:
                g_mpa = self.quantity(material["shear_modulus"], _STRESS_TO_MPA, "N/mm2", entity, "shear_modulus")
            nu = None
            if material.get("poisson_ratio") is not None:
                nu = self.quantity(material["poisson_ratio"], _DIMENSIONLESS, "1", entity, "poisson_ratio")
                if nu is not None and e_mpa is not None and g_mpa is not None and g_mpa > 0:
                    implied = e_mpa / (2.0 * (1.0 + nu))
                    if abs(implied - g_mpa) > 1e-9 * g_mpa:
                        self.diag("MBF-SI-SHEAR-MODULUS-NOT-EXPORTED", "warning", entity,
                                  f"{entity} gives both poisson_ratio and shear_modulus; the MBF carries the Poisson "
                                  f"ratio {_num(nu)}, whose implied G = {_num(implied)} MPa differs from the authored "
                                  f"G = {_num(g_mpa)} MPa")
            elif e_mpa is not None and g_mpa is not None:
                if g_mpa <= 0:
                    self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", entity, f"{entity} shear_modulus must be positive")
                else:
                    nu = e_mpa / (2.0 * g_mpa) - 1.0
                    self.assumptions.append(
                        f"{entity}: Poisson ratio derived as E/(2G) - 1 = {_num(nu)} "
                        f"(E = {_num(e_mpa)} MPa, G = {_num(g_mpa)} MPa)"
                    )
            elif e_mpa is not None:
                self.refuse("MBF-SI-MATERIAL-POISSON-MISSING", entity,
                            f"{entity} has neither poisson_ratio nor shear_modulus; the MBF needs a Poisson ratio")
            if nu is not None and not 0.0 < nu < 0.5:
                self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", entity,
                          f"{entity} Poisson ratio {_num(nu)} is outside (0, 0.5)")
                nu = None
            alpha = 0.0
            has_alpha = material.get("thermal_expansion_coefficient") is not None
            if has_alpha:
                alpha = self.quantity(material["thermal_expansion_coefficient"], _EXPANSION_TO_PER_C, "mm/mm/C",
                                      entity, "thermal_expansion_coefficient")
            if not ok or e_mpa is None or nu is None or alpha is None:
                continue
            self.materials[material["id"]] = {"E": e_mpa, "nu": nu, "alpha": alpha, "has_alpha": has_alpha}

    def read_pipes(self) -> None:
        for pipe in _list(self.model.get("pipe_segments"), self, "model.pipe_segments"):
            if not isinstance(pipe, Mapping) or not isinstance(pipe.get("id"), str):
                self.invalid("model.pipe_segments", "each pipe segment needs a string id")
                continue
            entity = f"pipe:{pipe['id']}"
            if pipe["id"] in self.pipes:
                self.invalid(entity, f"pipe segment id {pipe['id']!r} is repeated")
                continue
            start, end = pipe.get("from"), pipe.get("to")
            if start not in self.nodes or end not in self.nodes:
                self.diag("MBF-SI-REFERENCE-UNRESOLVED", "error", entity,
                          f"{entity} connects {start!r} to {end!r}; both must be model nodes")
                continue
            if start == end:
                self.refuse("MBF-SI-GEOMETRY-UNSUPPORTED", entity, f"{entity} starts and ends at the same node")
                continue
            material_id = pipe.get("material")
            if material_id not in self.materials:
                if not any(item["entity"] == f"material:{material_id}" for item in self.diagnostics):
                    self.diag("MBF-SI-REFERENCE-UNRESOLVED", "error", entity,
                              f"{entity} names material {material_id!r}, which is not in model.materials")
                continue
            y_reference = pipe.get("y_reference")
            section = self.read_section(pipe.get("section"), entity)
            if section is None:
                continue
            self.pipes[pipe["id"]] = {"from": start, "to": end, "material": material_id, "y_reference": y_reference,
                                      **section}

    def omit_orientation(self) -> None:
        orientation = [pipe_id for pipe_id, pipe in self.pipes.items()
                       if pipe["y_reference"] is not None and pipe_id not in self.bends]
        if orientation:
            self.omit("MBF-SI-ORIENTATION-NOT-EXPORTED", "model.pipe_segments.y_reference",
                      f"y_reference of {', '.join(orientation)} is not exported; element local axes follow the "
                      "target program's own convention")

    def read_section(self, section: Any, entity: str) -> dict[str, float] | None:
        if not isinstance(section, Mapping):
            self.invalid(entity, f"{entity} section must be an object")
            return None
        od = self.quantity(section.get("outside_diameter"), _LENGTH_TO_MM, "mm", entity, "section.outside_diameter")
        thk = self.quantity(section.get("wall_thickness"), _LENGTH_TO_MM, "mm", entity, "section.wall_thickness")
        ok = od is not None and thk is not None
        if ok and not (od > 0 and thk > 0 and 2 * thk < od):
            self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", entity,
                      f"{entity} needs 0 < wall_thickness < outside_diameter / 2 (got {_num(od)} mm, {_num(thk)} mm)")
            ok = False
        for name in ("corrosion_allowance", "mill_tolerance", "insulation_thickness"):
            if section.get(name) is None:
                continue
            value = self.quantity(section[name], _LENGTH_TO_MM, "mm", entity, f"section.{name}")
            if value is None:
                ok = False
            elif value != 0:
                self.refuse("MBF-SI-SECTION-UNSUPPORTED", entity,
                            f"{entity} section.{name} is nonzero; the pilot writer exports bare pipe only")
                ok = False
        for name in ("material_density", "contents_density", "insulation_density"):
            if section.get(name) is not None:
                self.omit("MBF-SI-SECTION-FIELD-NOT-EXPORTED", entity,
                          f"{entity} section.{name} is not exported; weight is exported only from weight primitives")
        if not ok:
            return None
        return {"od": od, "thk": thk}  # type: ignore[dict-item]

    def read_components(self) -> None:
        for component in _list(self.model.get("components"), self, "model.components"):
            identity = component.get("id") if isinstance(component, Mapping) else None
            kind = component.get("kind") if isinstance(component, Mapping) else None
            interface = component.get("mechanics_interface") if isinstance(component, Mapping) else None
            consumption = interface.get("solver_consumption") if isinstance(interface, Mapping) else None
            if kind == "bend" and consumption == _BEND_SOLVER_CONSUMPTION:
                self.read_bend(component)
                continue
            self.refuse("MBF-SI-COMPONENT-UNSUPPORTED", f"component:{identity}",
                        f"component {identity!r} of kind {kind!r} (solver_consumption {consumption!r}) is not "
                        "exported; the pilot writer exports straight pipe segments and bends realized as "
                        f"{_BEND_SOLVER_CONSUMPTION!r} only")
        self.omit_orientation()

    def read_bend(self, component: Mapping[str, Any]) -> None:
        entity = f"component:{component.get('id')}"
        geometry = component.get("geometry")
        modifiers = component.get("modifiers")
        if not isinstance(geometry, Mapping):
            self.invalid(entity, f"{entity} bend needs a geometry block with bend_pipe_ref and bend_radius")
            return
        pipe_id = geometry.get("bend_pipe_ref")
        if pipe_id not in self.pipes:
            self.diag("MBF-SI-REFERENCE-UNRESOLVED", "error", entity,
                      f"{entity} bend_pipe_ref {pipe_id!r} is not an exported pipe segment")
            return
        if pipe_id in self.bends:
            self.refuse("MBF-SI-BEND-UNSUPPORTED", entity, f"{entity} realizes pipe:{pipe_id}, which another bend already realizes")
            return
        pipe = self.pipes[pipe_id]
        if component.get("node") not in (pipe["from"], pipe["to"]):
            self.diag("MBF-SI-REFERENCE-UNRESOLVED", "error", entity,
                      f"{entity} node {component.get('node')!r} is not an endpoint of pipe:{pipe_id}")
            return
        factor_record = modifiers.get("flexibility_factor_user_value") if isinstance(modifiers, Mapping) else None
        if factor_record is None:
            self.refuse("MBF-SI-BEND-FLEXIBILITY-MISSING", entity,
                        f"{entity} has no modifiers.flexibility_factor_user_value; the writer never supplies an "
                        "implicit or code-based bend flexibility factor")
            return
        factor = self.quantity(factor_record, _DIMENSIONLESS, "1", entity, "flexibility_factor_user_value")
        radius = self.quantity(geometry.get("bend_radius"), _LENGTH_TO_MM, "mm", entity, "bend_radius")
        angle = None
        if geometry.get("bend_angle") is not None:
            record = geometry["bend_angle"]
            unit = record.get("unit") if isinstance(record, Mapping) else None
            if unit not in ("rad", "deg"):
                self.refuse("MBF-SI-UNIT-UNSUPPORTED", entity, f"{entity} bend_angle unit {unit!r} is refused (rad or deg)")
                return
            angle = self.number(record.get("value"), entity, "bend_angle")
            if angle is not None and unit == "deg":
                angle = math.radians(angle)
        if factor is None or radius is None:
            return
        if not factor > 0 or not radius > 0:
            self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", entity,
                      f"{entity} flexibility factor and bend radius must be positive")
            return
        if not _is_vector(pipe["y_reference"]):
            self.invalid(entity, f"pipe:{pipe_id} needs a numeric y_reference to fix the bend plane")
            return
        self.bends[pipe_id] = {"component": component.get("id"), "entity": entity, "radius": radius, "k": factor,
                               "angle": angle}

    def resolve_bends(self) -> None:
        """Place each bend at the tangent intersection (TI) of its adjacent straights, or refuse it by name."""
        incident: dict[str, list[str]] = {}
        for pipe_id, pipe in self.pipes.items():
            incident.setdefault(pipe["from"], []).append(pipe_id)
            incident.setdefault(pipe["to"], []).append(pipe_id)
        loaded = {record["target"].get("node") for record in self.load_records.values()}
        for pipe_id, bend in sorted(self.bends.items()):
            entity = bend["entity"]
            pipe = self.pipes[pipe_id]
            straights: dict[str, str] = {}
            for tangent in (pipe["from"], pipe["to"]):
                others = [other for other in incident.get(tangent, []) if other != pipe_id]
                if tangent in self.node_restraints or tangent in loaded:
                    self.refuse("MBF-SI-BEND-UNSUPPORTED", entity,
                                f"{entity} tangent point {tangent!r} carries a support or load; the target program "
                                "reports bend tangent points as generated nodes, so the writer does not place one there")
                elif not others:
                    self.refuse("MBF-SI-BEND-UNSUPPORTED", entity,
                                f"{entity} ends a run at tangent point {tangent!r}; a bend needs a straight pipe on both sides")
                elif len(others) > 1:
                    self.refuse("MBF-SI-BEND-UNSUPPORTED", entity,
                                f"{entity} tangent point {tangent!r} joins {len(others) + 1} pipes; a branch at a bend "
                                "tangent point is not exported")
                elif others[0] in self.bends:
                    self.refuse("MBF-SI-BEND-UNSUPPORTED", entity,
                                f"{entity} meets another bend at {tangent!r}; consecutive bends without a straight pipe "
                                "between them are not exported")
                else:
                    straights[tangent] = others[0]
            if len(straights) != 2:
                continue
            ti = self.tangent_intersection(pipe_id, bend, straights)
            if ti is None:
                continue
            bend.update(ti=ti, straights=straights)
            for tangent in straights:
                self.tangent_points[tangent] = pipe_id

    def tangent_intersection(self, pipe_id: str, bend: dict[str, Any],
                             straights: dict[str, str]) -> tuple[float, float, float] | None:
        entity = bend["entity"]
        pipe = self.pipes[pipe_id]
        near, far = pipe["from"], pipe["to"]

        def far_end(tangent: str) -> str:
            straight = self.pipes[straights[tangent]]
            return straight["to"] if straight["from"] == tangent else straight["from"]

        p, q = self.nodes[near], self.nodes[far]
        incoming, outgoing, chord = _sub(p, self.nodes[far_end(near)]), _sub(self.nodes[far_end(far)], q), _sub(q, p)
        if min(_norm(incoming), _norm(outgoing), _norm(chord)) == 0:
            self.refuse("MBF-SI-GEOMETRY-UNSUPPORTED", entity, f"{entity}: a bend or adjacent straight has zero length")
            return None
        d1, d2 = _unit(incoming), _unit(outgoing)
        cosine = max(-1.0, min(1.0, _dot(d1, d2)))
        det = 1.0 - cosine * cosine
        if det <= _BEND_RELATIVE_TOLERANCE:
            self.refuse("MBF-SI-BEND-GEOMETRY-INCONSISTENT", entity,
                        f"{entity}: the straights at {near!r} and {far!r} are parallel; there is no tangent intersection")
            return None
        b1, b2 = _dot(d1, chord), _dot(d2, chord)
        t1 = (b1 - cosine * b2) / det
        t2 = (b2 - cosine * b1) / det
        ti = tuple(p[axis] + t1 * d1[axis] for axis in range(3))
        residual = _norm(_sub(_sub(q, tuple(t2 * value for value in d2)), ti))
        chord_length = _norm(chord)
        theta = math.acos(cosine)
        tangent_length = bend["radius"] * math.tan(theta / 2.0)
        tol = _BEND_RELATIVE_TOLERANCE
        if residual > tol * chord_length:
            self.refuse("MBF-SI-BEND-GEOMETRY-INCONSISTENT", entity,
                        f"{entity}: the straights at {near!r} and {far!r} do not intersect (gap {_num(residual)} mm)")
            return None
        if not (abs(t1 - tangent_length) <= tol * tangent_length and abs(t2 - tangent_length) <= tol * tangent_length):
            self.refuse("MBF-SI-BEND-GEOMETRY-INCONSISTENT", entity,
                        f"{entity}: tangent lengths {_num(t1)} mm and {_num(t2)} mm disagree with R tan(theta/2) = "
                        f"{_num(tangent_length)} mm (R = {_num(bend['radius'])} mm, theta = {_num(math.degrees(theta))} deg)")
            return None
        if bend["angle"] is not None and abs(bend["angle"] - theta) > tol * max(theta, 1.0):
            self.refuse("MBF-SI-BEND-GEOMETRY-INCONSISTENT", entity,
                        f"{entity}: bend_angle {_num(bend['angle'])} rad disagrees with the {_num(theta)} rad between "
                        "the adjacent straights")
            return None
        y_ref = self.pipes[pipe_id]["y_reference"]
        y_raw = tuple(float(y_ref[axis]) for axis in "xyz")
        chord_unit = _unit(chord)
        side = _sub(y_raw, tuple(_dot(y_raw, chord_unit) * value for value in chord_unit))
        middle = tuple((p[axis] + q[axis]) / 2.0 for axis in range(3))
        toward_ti = _sub(ti, middle)
        if _norm(side) == 0 or _norm(toward_ti) == 0:
            aligned = False
        else:
            side_unit, ti_unit = _unit(side), _unit(toward_ti)
            aligned = _dot(side_unit, ti_unit) > 0 and _norm(_cross(side_unit, ti_unit)) <= tol
        if not aligned:
            self.refuse("MBF-SI-BEND-GEOMETRY-INCONSISTENT", entity,
                        f"{entity}: pipe:{pipe_id} y_reference does not point from the chord toward the tangent "
                        "intersection in the plane of the adjacent straights, so SWBPIPE's arc is not the corner bend "
                        "the target program would form")
            return None
        bend["theta"] = theta
        return ti  # type: ignore[return-value]

    def read_supports(self) -> None:
        for support in _list(self.model.get("supports"), self, "model.supports"):
            if not isinstance(support, Mapping) or not isinstance(support.get("id"), str):
                self.invalid("model.supports", "each support needs a string id")
                continue
            entity = f"support:{support['id']}"
            if support["id"] in self.supports:
                self.invalid(entity, f"support id {support['id']!r} is repeated")
                continue
            node = support.get("node")
            if node not in self.nodes:
                self.diag("MBF-SI-REFERENCE-UNRESOLVED", "error", entity, f"{entity} is at unknown node {node!r}")
                continue
            family = support.get("family")
            special = [name for name in ("stiffness", "hanger", "nonlinear") if support.get(name) is not None]
            if family not in _RIGID_FAMILIES or special:
                what = f"family {family!r}" if family not in _RIGID_FAMILIES else ", ".join(special)
                self.refuse("MBF-SI-SUPPORT-UNSUPPORTED", entity,
                            f"{entity} ({what}) is not a rigid restraint; springs, hangers and nonlinear or "
                            "gapped supports are not exported by the pilot writer")
                continue
            restraints = support.get("restraints")
            if not isinstance(restraints, list):
                self.invalid(entity, f"{entity} restraints must be a list")
                continue
            dofs = set()
            for token in restraints:
                if token not in _DOF_TOKENS:
                    self.invalid(entity, f"{entity} restraint {token!r} is not a frame DOF")
                    break
                dofs.add(_DOF_TOKENS[token])
            else:
                if not dofs:
                    self.omit("MBF-SI-SUPPORT-EMPTY", entity, f"{entity} restrains no DOF and is not exported")
                    continue
                if family == "anchor" and len(dofs) != 6:
                    self.refuse("MBF-SI-SUPPORT-UNSUPPORTED", entity,
                                f"{entity} has family anchor but restrains only {', '.join(sorted(dofs))}")
                    continue
                self.supports[support["id"]] = {"node": node, "dofs": dofs}
                self.node_restraints.setdefault(node, set()).update(dofs)
                self.node_supports.setdefault(node, []).append(support["id"])
        for node, dofs in sorted(self.node_restraints.items()):
            rotations = sorted(dof for dof in dofs if dof.startswith("R"))
            if not rotations or len(dofs) == 6:
                continue
            for support_id in self.node_supports[node]:
                if any(dof.startswith("R") for dof in self.supports[support_id]["dofs"]):
                    self.refuse("MBF-SI-SUPPORT-UNSUPPORTED", f"support:{support_id}",
                                f"support:{support_id} at node {node!r} restrains rotation ({', '.join(rotations)}) "
                                "without being a full anchor; the target tool does not accept "
                                "rotational restraints (FIXR) in an imported MBF, so partial rotational restraints are "
                                "not exported")

    def is_anchor(self, node: Any) -> bool:
        return isinstance(node, str) and len(self.node_restraints.get(node, ())) == 6

    def read_load_cases(self) -> None:
        kinds: dict[str, str] = {}
        seen: set[str] = set()
        for case in _list(self.model.get("load_cases"), self, "model.load_cases"):
            if not isinstance(case, Mapping) or not isinstance(case.get("id"), str):
                self.invalid("model.load_cases", "each load case needs a string id")
                continue
            entity = f"load_case:{case['id']}"
            if case["id"] in seen:
                self.invalid(entity, f"load case id {case['id']!r} is repeated")
                continue
            seen.add(case["id"])
            if case.get("pressure_regions") is not None:
                self.refuse("MBF-SI-PRESSURE-UNSUPPORTED", entity, f"{entity} has pressure_regions; pressure is not exported")
            for name in ("equivalent_static", "modulus_basis_ref", "modulus_basis_temperature", "analysis_state"):
                if case.get(name) is not None:
                    self.refuse("MBF-SI-LOAD-CASE-FEATURE-UNSUPPORTED", entity, f"{entity} {name} is not exported")
            loads = case.get("primitive_loads") or []
            if not isinstance(loads, list):
                self.invalid(entity, f"{entity} primitive_loads must be a list")
                continue
            categories = {load.get("category") for load in loads if isinstance(load, Mapping)}
            unsupported = categories - _MECHANICAL_CATEGORIES - _THERMAL_CATEGORIES
            for load in loads:
                if isinstance(load, Mapping) and load.get("category") in unsupported:
                    code = "MBF-SI-PRESSURE-UNSUPPORTED" if load["category"] in {"pressure", "hydrotest"} else "MBF-SI-LOAD-UNSUPPORTED"
                    self.refuse(code, f"load:{case['id']}/{load.get('id')}",
                                f"load {load.get('id')!r} in {entity} has category {load['category']!r}, which the pilot "
                                "writer does not export")
            if not loads:
                self.omit("MBF-SI-LOAD-CASE-EMPTY", entity, f"{entity} has no primitive loads and is not exported")
                continue
            mechanical = bool(categories & _MECHANICAL_CATEGORIES)
            thermal = bool(categories & _THERMAL_CATEGORIES)
            if mechanical and thermal:
                self.refuse("MBF-SI-LOAD-CASE-MAPPING-UNSUPPORTED", entity,
                            f"{entity} mixes mechanical (force, moment, weight) and thermal/displacement primitives; "
                            f"each case must map to exactly one of {LOAD_CASE_LABELS['sustained']!r} or {LOAD_CASE_LABELS['expansion']!r}")
                continue
            if unsupported and not (mechanical or thermal):
                continue
            label = LOAD_CASE_LABELS["sustained" if mechanical else "expansion"]
            other = next((key for key, value in kinds.items() if value == label), None)
            if other is not None:
                self.refuse("MBF-SI-LOAD-CASE-MAPPING-UNSUPPORTED", entity,
                            f"{entity} and load_case:{other} both map to {label!r}; at most one case of each kind is exported")
                continue
            kinds[case["id"]] = label
            for load in loads:
                if isinstance(load, Mapping) and load.get("category") not in unsupported:
                    self.read_load(case["id"], label, load)
        self.case_labels = kinds
        for combination in _list(self.model.get("combinations"), self, "model.combinations"):
            identity = combination.get("id") if isinstance(combination, Mapping) else None
            self.omit("MBF-SI-COMBINATION-NOT-EXPORTED", f"combination:{identity}",
                      f"combination {identity!r} is not exported; the target program forms its own combinations")

    def read_load(self, case_id: str, label: str, load: Mapping[str, Any]) -> None:
        load_id = load.get("id")
        entity = f"load:{case_id}/{load_id}"
        category = load["category"]
        target = load.get("target")
        if not isinstance(target, Mapping):
            self.invalid(entity, f"{entity} target must be an object")
            return
        direction = load.get("direction")
        dimension = load.get("dimension")
        record: dict[str, Any] = {"load_case": case_id, "caepipe_load_case": label, "category": category}
        if category in {"concentrated_force", "concentrated_moment"}:
            force = category == "concentrated_force"
            node = target.get("node")
            if target.get("type") != "node" or node not in self.nodes:
                self.diag("MBF-SI-REFERENCE-UNRESOLVED", "error", entity, f"{entity} must target a model node")
                return
            directions = _FORCE_DIRECTIONS if force else _MOMENT_DIRECTIONS
            if direction not in directions or dimension != ("force" if force else "moment"):
                self.refuse("MBF-SI-LOAD-UNSUPPORTED", entity,
                            f"{entity} direction {direction!r} / dimension {dimension!r} is not exported; "
                            + ("global_x/y/z force" if force else "global RX/RY/RZ moment") + " only")
                return
            value = self.quantity(load.get("magnitude"), _FORCE_TO_N if force else _MOMENT_TO_NM,
                                  "N" if force else "N*m", entity, "magnitude")
            if value is None:
                return
            store = self.forces if force else self.moments
            store.setdefault(node, [0.0, 0.0, 0.0])[directions[direction]] += value
            record.update(target={"node": node}, mbf="FOR" if force else "MOM")
        elif category == "weight":
            pipe = target.get("pipe")
            if target.get("type") != "element" or pipe not in self.pipes:
                self.diag("MBF-SI-REFERENCE-UNRESOLVED", "error", entity, f"{entity} must target a model pipe segment")
                return
            if dimension != "force_per_length":
                self.refuse("MBF-SI-LOAD-UNSUPPORTED", entity, f"{entity} weight needs dimension force_per_length")
                return
            value = self.quantity(load.get("magnitude"), _FORCE_PER_LENGTH_TO_N_PER_M, "N/m", entity, "magnitude")
            if value is None:
                return
            if direction not in {"global_y", "GLOBAL_Y"} or value > 0:
                self.refuse("MBF-SI-LOAD-UNSUPPORTED", entity,
                            f"{entity} weight acts along {direction!r} with magnitude {_num(value)} N/m; only global -Y "
                            "weight (the target program's default gravity) is exported")
                return
            self.weight[pipe] = self.weight.get(pipe, 0.0) + value
            record.update(target={"pipe": pipe}, mbf="MATERIAL weight density")
        elif category == "thermal":
            pipe = target.get("pipe")
            if target.get("type") != "element" or pipe not in self.pipes:
                self.diag("MBF-SI-REFERENCE-UNRESOLVED", "error", entity, f"{entity} must target a model pipe segment")
                return
            if dimension not in {"temperature_change", "temperature_interval"}:
                self.refuse("MBF-SI-LOAD-UNSUPPORTED", entity, f"{entity} thermal needs a temperature-interval dimension")
                return
            value = self.quantity(load.get("magnitude"), _TEMPERATURE_INTERVAL_TO_C, "C", entity, "magnitude")
            if value is None:
                return
            material_id = self.pipes[pipe]["material"]
            if value != 0 and not self.materials[material_id]["has_alpha"]:
                self.refuse("MBF-SI-MATERIAL-EXPANSION-MISSING", entity,
                            f"{entity} changes the temperature of pipe:{pipe}, whose material {material_id!r} has no "
                            "thermal_expansion_coefficient")
                return
            self.delta_t[pipe] = self.delta_t.get(pipe, 0.0) + value
            record.update(target={"pipe": pipe}, mbf="LOADS Temperature 1")
        else:  # imposed_displacement
            support_id = target.get("support")
            dof = _DOF_TOKENS.get(target.get("dof"))
            support = self.supports.get(support_id)
            if target.get("type") != "support" or support is None:
                self.refuse("MBF-SI-IMPOSED-DISPLACEMENT-UNSUPPORTED", entity,
                            f"{entity} must target an exported rigid support (got {dict(target)!r})")
                return
            if dof is None or _DOF_TOKENS.get(direction) != dof:
                self.invalid(entity, f"{entity} target dof and direction must name the same frame DOF")
                return
            if not self.is_anchor(support["node"]) or dof not in support["dofs"]:
                self.refuse("MBF-SI-IMPOSED-DISPLACEMENT-UNSUPPORTED", entity,
                            f"{entity} moves support {support_id!r}, which does not map to an anchor restraining {dof}; "
                            "only anchor displacements are exported")
                return
            translation = dof.startswith("U")
            if dimension != ("displacement" if translation else "rotation"):
                self.refuse("MBF-SI-LOAD-UNSUPPORTED", entity,
                            f"{entity} {dof} needs dimension {'displacement' if translation else 'rotation'}")
                return
            value = self.quantity(load.get("magnitude"), _LENGTH_TO_MM if translation else _ROTATION_TO_DEG,
                                  "mm" if translation else "deg", entity, "magnitude")
            if value is None:
                return
            store = self.anchor_disp if translation else self.anchor_rot
            store.setdefault(support["node"], [0.0, 0.0, 0.0])[DOFS.index(dof) % 3] += value
            record.update(target={"support": support_id, "dof": dof}, mbf="D" if translation else "ROT")
        self.load_records[f"{case_id}/{load_id}"] = record

    def read_other_members(self) -> None:
        if self.model.get("model_settings") is not None:
            self.omit("MBF-SI-MODEL-SETTINGS-NOT-EXPORTED", "model.model_settings",
                      "model_settings (input validation settings) are not exported")

    # -- layout ---------------------------------------------------------
    def layout(self) -> list[dict[str, Any]]:
        incident: dict[str, list[tuple[str, str]]] = {}
        for pipe_id in sorted(self.pipes):
            pipe = self.pipes[pipe_id]
            incident.setdefault(pipe["from"], []).append((pipe_id, pipe["to"]))
            incident.setdefault(pipe["to"], []).append((pipe_id, pipe["from"]))
        for node in sorted(set(self.nodes) - set(incident)):
            entity = f"node:{node}"
            referenced = node in self.node_restraints or any(
                record.get("target", {}).get("node") == node for record in self.load_records.values()
            )
            if referenced:
                self.refuse("MBF-SI-TOPOLOGY-UNSUPPORTED", entity,
                            f"{entity} carries a support or load but no pipe segment")
            else:
                self.omit("MBF-SI-NODE-NOT-EXPORTED", entity, f"{entity} is not connected to any pipe segment")
        if len(incident) > _MAX_NODES:
            self.refuse("MBF-SI-NODE-NUMBER-LIMIT", "model.nodes",
                        f"{len(incident)} connected nodes exceed the {_MAX_NODES} node numbers 10..{_MAX_NODES * 10}")
        if not self.pipes:
            self.refuse("MBF-SI-TOPOLOGY-UNSUPPORTED", "model.pipe_segments", "the model has no exportable pipe segment")
        # Row keys: a model node id, or ("TI", bend pipe id) for a bend row at its tangent intersection.
        # A bend row stands for the incoming straight plus the bend and ends (for continuation) at the far
        # tangent point; its offsets end at the tangent intersection.
        rows: list[dict[str, Any]] = []
        written: set[str] = set()
        used: set[str] = set()
        while True:
            remaining = sorted(set(incident) - written)
            if not remaining:
                break
            anchors = [node for node in remaining if self.is_anchor(node)]
            plain = [node for node in remaining if node not in self.tangent_points]
            root = anchors[0] if anchors else (plain or remaining)[0]
            rows.append({"kind": "F", "node": root, "end": root, "element": None, "new": True})
            written.add(root)
            stack = [root]
            while stack:
                current = stack[-1]
                step = next(((pipe_id, other) for pipe_id, other in incident[current] if pipe_id not in used), None)
                if step is None:
                    stack.pop()
                    continue
                pipe_id, other = step
                used.add(pipe_id)
                if other in written:
                    self.refuse("MBF-SI-TOPOLOGY-UNSUPPORTED", f"pipe:{pipe_id}",
                                f"pipe:{pipe_id} closes a loop at node {other!r}; closed loops are not exported")
                    continue
                if pipe_id in self.bends:
                    self.refuse("MBF-SI-BEND-UNSUPPORTED", self.bends[pipe_id]["entity"],
                                f"{self.bends[pipe_id]['entity']} is reached from its tangent point {current!r} without "
                                "its incoming straight; the layout cannot place it")
                    continue
                if rows[-1]["end"] != current:
                    if current in self.tangent_points:
                        self.refuse("MBF-SI-BEND-UNSUPPORTED", f"node:{current}",
                                    f"the layout would restart at bend tangent point {current!r}")
                    rows.append({"kind": "F", "node": current, "end": current, "element": None, "new": False})
                bend_id = self.tangent_points.get(other)
                if bend_id is None:
                    rows.append({"kind": "T", "node": other, "end": other, "element": pipe_id, "new": True})
                    written.add(other)
                    stack.append(other)
                    continue
                bend_pipe = self.pipes[bend_id]
                far = bend_pipe["to"] if bend_pipe["from"] == other else bend_pipe["from"]
                if far in written:
                    self.refuse("MBF-SI-TOPOLOGY-UNSUPPORTED", f"pipe:{bend_id}",
                                f"pipe:{bend_id} closes a loop at node {far!r}; closed loops are not exported")
                    continue
                used.add(bend_id)
                rows.append({"kind": "T", "node": ("TI", bend_id), "end": far, "element": pipe_id, "bend": bend_id,
                             "near": other, "far": far, "new": True})
                written.update((other, far))
                stack.append(far)
        return rows

    def position(self, key: Any) -> tuple[float, float, float]:
        if isinstance(key, tuple):
            return self.bends[key[1]]["ti"]
        return self.nodes[key]

    def pipe_attributes(self, pipe_id: str, materials: dict, sections: dict, loads: dict,
                        material_map: dict, section_map: dict, load_entries: dict,
                        temperatures: list[float]) -> tuple[str, str, str]:
        pipe = self.pipes[pipe_id]
        material = self.materials[pipe["material"]]
        density = 0.0
        if pipe_id in self.weight:
            od_m, thk_m = pipe["od"] / 1000.0, pipe["thk"] / 1000.0
            area = math.pi / 4.0 * (od_m**2 - (od_m - 2.0 * thk_m) ** 2)
            density = -self.weight[pipe_id] / (area * STANDARD_GRAVITY)
        material_key = (pipe["material"], _num(density))
        if material_key not in materials:
            name = f"M{len(materials) + 1}"
            materials[material_key] = name
            material_map[name] = {
                "swbpipe_material": pipe["material"],
                "weight_density_kgf_per_m3": density,
                "poisson_ratio": material["nu"],
                "elastic_modulus_n_per_mm2": material["E"],
                "thermal_expansion_mm_per_mm_per_c": material["alpha"],
                "temperatures_c": temperatures,
                "elements": [],
            }
        section_key = (_num(pipe["od"]), _num(pipe["thk"]))
        if section_key not in sections:
            name = f"P{len(sections) + 1}"
            sections[section_key] = name
            section_map[name] = {"outside_diameter_mm": pipe["od"], "wall_thickness_mm": pipe["thk"], "elements": []}
        delta = self.delta_t.get(pipe_id, 0.0)
        if _num(delta) not in loads:
            name = f"L{len(loads) + 1}"
            loads[_num(delta)] = name
            load_entries[name] = self.t_ref + delta
        names = (materials[material_key], sections[section_key], loads[_num(delta)])
        material_map[names[0]]["elements"].append(pipe_id)
        section_map[names[1]]["elements"].append(pipe_id)
        return names

    # -- rendering ------------------------------------------------------
    def render(self, rows: list[dict[str, Any]]) -> MbfSiResult:
        numbers: dict[Any, int] = {}
        node_map: dict[str, str] = {}
        for row in rows:
            if row["node"] not in numbers:
                numbers[row["node"]] = 10 * (len(numbers) + 1)
            number = numbers[row["node"]]
            if "bend" in row:
                node_map[row["near"]] = f"{number}A"
                node_map[row["far"]] = f"{number}B"
            else:
                node_map.setdefault(row["node"], str(number))
        element_rows = [row for row in rows if row["kind"] == "T"]
        deltas = [self.delta_t.get(pipe_id, 0.0) for pipe_id in self.pipes]
        t_hi = self.t_ref + max(max(abs(value) for value in deltas), 100.0)
        t_lo = self.t_ref + min(0.0, min(deltas))
        temperatures = sorted({t_lo, self.t_ref, t_hi})

        materials: dict[tuple[str, str], str] = {}
        sections: dict[tuple[str, str], str] = {}
        loads: dict[str, str] = {}
        material_map: dict[str, dict[str, Any]] = {}
        section_map: dict[str, dict[str, Any]] = {}
        element_map: dict[str, dict[str, Any]] = {}
        load_entries: dict[str, float] = {}
        attributes: dict[str, tuple[str, str, str]] = {}
        order = 0
        start = rows[0]["end"]
        for layout_row, row in enumerate(rows):
            if row["kind"] == "F":
                start = row["end"]
                continue
            pipe_id = row["element"]
            names = self.pipe_attributes(pipe_id, materials, sections, loads, material_map, section_map,
                                         load_entries, temperatures)
            attributes[pipe_id] = names
            members = [(pipe_id, start, row["near"] if "bend" in row else row["node"])]
            if "bend" in row:
                bend_id = row["bend"]
                bend_names = self.pipe_attributes(bend_id, materials, sections, loads, material_map, section_map,
                                                  load_entries, temperatures)
                if bend_names != names:
                    self.refuse("MBF-SI-BEND-UNSUPPORTED", self.bends[bend_id]["entity"],
                                f"{self.bends[bend_id]['entity']}: pipe:{bend_id} and its incoming straight pipe:{pipe_id} "
                                "differ in material, weight, section or temperature; the target program writes them as "
                                "one layout row with one set of properties")
                attributes[bend_id] = bend_names
                members.append((bend_id, row["near"], row["far"]))
            for member_id, member_start, member_end in members:
                member = self.pipes[member_id]
                element_map[member_id] = {
                    "from": node_map[member_start],
                    "to": node_map[member_end],
                    "layout_order": order,
                    "layout_row": layout_row,
                    "reversed": member_start != member["from"],
                    "material": attributes[member_id][0],
                    "section": attributes[member_id][1],
                    "load": attributes[member_id][2],
                }
                order += 1
            if "bend" in row:
                bend = self.bends[row["bend"]]
                element_map[row["bend"]]["bend"] = {
                    "component": bend["component"],
                    "radius_mm": bend["radius"],
                    "angle_deg": math.degrees(bend["theta"]),
                    "flexibility_factor": bend["k"],
                    "tangent_intersection_mm": list(bend["ti"]),
                    "caepipe_row_node": str(numbers[row["node"]]),
                }
            start = row["end"]
        if self.has_errors():
            return self.refused()

        lines = [_title(self.title), "VERSION", MBF_VERSION, "OPTIONS", "HGRA,SI", "OPTEMP", _num(self.t_ref), "MATERIAL"]
        for name, record in material_map.items():
            description = _sanitize(record["swbpipe_material"], _DESCRIPTION_MAX) or name
            lines.append(f"{name},{_num(record['weight_density_kgf_per_m3'])},{_num(record['poisson_ratio'])},1,1,CS,{description}")
            for temperature in temperatures:
                lines.append(f"{name},{_num(temperature)},{_num(record['elastic_modulus_n_per_mm2'])},"
                             f"{_num(record['thermal_expansion_mm_per_mm_per_c'])}")
        lines.append("PIPE")
        for name, record in section_map.items():
            lines.append(f"{name},{_num(record['outside_diameter_mm'])},{_num(record['wall_thickness_mm'])},0,0,0,0,0,0")
        lines.append("LOADS")
        for name, temperature in load_entries.items():
            lines.append(f"{name},{_num(temperature)},0,0")
        lines.append("LAYOUT")

        carried: tuple[str | None, str | None, str | None] = (None, None, None)
        first_element = element_rows[0]["element"]
        for index, row in enumerate(rows):
            node = row["node"]
            bend = self.bends[row["bend"]] if "bend" in row else None
            fields = [f"{row['kind']}{numbers[node]}"]
            comments: list[str] = []
            if bend is not None:
                fields.append("KI")
                comments.append(f"FF={_num(bend['k'])}")
            if row["new"] and self.is_anchor(node):
                fields.append("KA")
            pipe_id = row["element"] if row["kind"] == "T" else (first_element if index == 0 else None)
            if pipe_id is not None:
                wanted = attributes[pipe_id]
                if wanted[0] != carried[0]:
                    fields.append(f"M{wanted[0]}")
                if wanted[1] != carried[1]:
                    fields.append(f"P{wanted[1]}")
                if wanted[2] != carried[2]:
                    comments.append(f"L={wanted[2]}")
                carried = wanted
            if bend is not None:
                fields.append(f"B{_num(bend['radius'])}")
            x, y, z = self.position(node)
            if row["kind"] == "F":
                if index > 0 or any(value != 0 for value in (x, y, z)):
                    fields.extend([f"X{_num(x)}", f"Y{_num(y)}", f"Z{_num(z)}"])
            else:
                px, py, pz = self.position(rows[index - 1]["node"])
                offsets = [(axis, value) for axis, value in zip("XYZ", (x - px, y - py, z - pz)) if value != 0]
                if not offsets:
                    self.refuse("MBF-SI-GEOMETRY-UNSUPPORTED", f"pipe:{pipe_id}", f"pipe:{pipe_id} has zero length")
                fields.extend(f"{axis}{_num(value)}" for axis, value in offsets)
            if row["new"] and bend is None:
                comments.extend(self.node_comments(node))
            if comments:
                comments[0] = "C" + comments[0]
            lines.append(",".join(fields + comments))
        if self.has_errors():
            return self.refused()

        layout_row_count = 1 + len(rows)
        if self.max_layout_rows is not None and layout_row_count > self.max_layout_rows:
            self.diag("MBF-SI-LAYOUT-ROW-LIMIT", "warning", "LAYOUT",
                      f"{layout_row_count} layout rows (title row plus {len(rows)} F/T rows) exceed {self.max_layout_rows}; "
                      "a row-limited (evaluation) build of the target program may refuse the file")

        support_map = {}
        for support_id, support in self.supports.items():
            support_map[support_id] = {
                "node": node_map[support["node"]],
                "caepipe_kind": "anchor" if self.is_anchor(support["node"]) else "restraint",
                "restrained": [dof for dof in DOFS if dof in support["dofs"]],
            }
        load_map = {}
        for key, record in self.load_records.items():
            entry = dict(record)
            target = entry["target"]
            if "node" in target:
                entry["caepipe_node"] = node_map[target["node"]]
            elif "pipe" in target:
                entry["caepipe_element"] = dict(element_map[target["pipe"]])
                if entry["category"] == "thermal":
                    entry["mbf"] = f"LOADS {element_map[target['pipe']]['load']} Temperature 1"
                else:
                    entry["mbf"] = f"MATERIAL {element_map[target['pipe']]['material']} weight density"
            else:
                entry["caepipe_node"] = node_map[self.supports[target["support"]]["node"]]
            load_map[key] = entry
        self.add_assumptions(temperatures)
        return MbfSiResult(
            status="written",
            text="".join(line + "\r\n" for line in lines),
            diagnostics=self.diagnostics,
            node_map=node_map,
            element_map=element_map,
            support_map=support_map,
            load_case_map=dict(self.case_labels),
            material_map=material_map,
            section_map=section_map,
            load_map=load_map,
            layout_row_count=layout_row_count,
            loss_report=self.loss,
            assumptions=self.assumptions,
        )

    def node_comments(self, node: str) -> list[str]:
        comments = []
        restrained = self.node_restraints.get(node, set())
        if restrained and not self.is_anchor(node):
            translations = [1 if dof in restrained else 0 for dof in DOFS[:3]]
            rotations = [1 if dof in restrained else 0 for dof in DOFS[3:]]
            if any(translations):
                comments.append("FIXD({},{},{})".format(*translations))
            if any(rotations):  # unreachable: read_supports refuses partial rotational restraints
                raise AssertionError("partial rotational restraint reached rendering")
        for code, store in (("FOR", self.forces), ("MOM", self.moments), ("D", self.anchor_disp), ("ROT", self.anchor_rot)):
            values = store.get(node)
            if values and any(value != 0 for value in values):
                comments.append(f"{code}({','.join(_num(value) for value in values)})")
        return comments

    def add_assumptions(self, temperatures: list[float]) -> None:
        notes = [
            f"Profile {MBF_SI_PROFILE_ID}: MBF version {MBF_VERSION}, OPTIONS SI with the default vertical axis Y; "
            "SWBPIPE global X, Y, Z are written unchanged as the target program's X, Y, Z, with gravity along -Y.",
            "Lengths are written in mm, forces in N, moments in N*m, modulus in N/mm2 (= MPa), temperatures in C; "
            "every source unit scales to these by an exact power of ten, and no other unit system is converted.",
            f"Load case mapping: mechanical cases (concentrated force, concentrated moment, weight) -> {LOAD_CASE_LABELS['sustained']!r}; "
            f"thermal and imposed-displacement cases -> {LOAD_CASE_LABELS['expansion']!r}.",
            "Materials are written temperature-independent: identical E and expansion coefficient at "
            + ", ".join(f"{_num(value)} C" for value in temperatures)
            + "; the optional allowable, yield and rupture columns are omitted.",
            "Every LOADS entry has Pressure 1 = 0 and specific gravity 0 (empty pipe); Temperature 1 = reference "
            "temperature + the element's SWBPIPE thermal change (summed when several primitives act on one element).",
        ]
        if self.weight:
            notes.append(
                f"Weight per length w (global -Y) is written as material weight density rho = |w| / (A * {STANDARD_GRAVITY} m/s^2) "
                "with A the bare metal area pi/4 (OD^2 - ID^2); kgf/m3 equals kg/m3 numerically. Elements without a "
                "weight primitive have density 0."
            )
        if any(not material["has_alpha"] for material in self.materials.values()):
            notes.append("A material without thermal_expansion_coefficient is written with expansion coefficient 0.")
        if self.rotation_converted:
            notes.append("Rotations authored in rad are written in degrees (x 180/pi).")
        if self.bends:
            notes.append(
                "Each curved bend (curved_bend_macro_element) is written as one layout row at the tangent intersection "
                "(TI) of its adjacent straights: 'T<n>,KI,B<radius mm>,<offsets to the TI>' with the user flexibility "
                "factor as the comment 'FF=<k>'; the incoming straight pipe and the bend share that row and its "
                "properties. The target program reports the near and far tangent points as nodes '<n>A' and '<n>B', "
                "which node_map assigns to the SWBPIPE tangent-point nodes; the next row's offsets start at the TI. "
                "The single user factor stands for SWBPIPE's in-plane and out-of-plane bending factor alike."
            )
        notes.append(
            "Model members the SWBPIPE solve does not read (analysis_status, data_boundary, diagnostics, lines, "
            "unresolved_assumptions, constraints, equipment_interfaces, rule_pack_refs) are not exported."
        )
        self.assumptions.extend(notes)


def _list(value: Any, writer: _Writer, entity: str) -> list[Any]:
    if value is None:
        return []
    if not isinstance(value, list):
        writer.invalid(entity, f"{entity} must be a list")
        return []
    return value


def _is_vector(value: Any) -> bool:
    return isinstance(value, Mapping) and all(
        not isinstance(value.get(axis), bool) and isinstance(value.get(axis), (int, float))
        and math.isfinite(value.get(axis)) for axis in "xyz"
    )


def _sub(a: tuple[float, ...], b: tuple[float, ...]) -> tuple[float, float, float]:
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2])


def _dot(a: tuple[float, ...], b: tuple[float, ...]) -> float:
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def _cross(a: tuple[float, ...], b: tuple[float, ...]) -> tuple[float, float, float]:
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def _norm(a: tuple[float, ...]) -> float:
    return math.sqrt(_dot(a, a))


def _unit(a: tuple[float, ...]) -> tuple[float, float, float]:
    length = _norm(a)
    return (a[0] / length, a[1] / length, a[2] / length)


def _scale(value: float, exponent: int) -> float:
    """Scale by an exact power of ten; division keeps down-scaling correctly rounded."""
    if exponent >= 0:
        return value * 10**exponent
    return value / 10 ** (-exponent)


def _num(value: float) -> str:
    if value == 0:
        return "0"
    return format(value, ".10g")


def _sanitize(text: Any, limit: int) -> str:
    cleaned = "".join(ch for ch in str(text) if 32 <= ord(ch) < 127 and ch != ",")
    return cleaned.strip()[:limit].strip()


def _title(text: Any) -> str:
    return _sanitize(text, _TITLE_MAX) or "SWBPIPE model"
