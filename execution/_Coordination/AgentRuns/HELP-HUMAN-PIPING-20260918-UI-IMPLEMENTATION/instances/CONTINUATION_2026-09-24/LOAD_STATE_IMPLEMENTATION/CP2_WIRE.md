# Checkpoint-2 wire shape (frozen for parallel test authoring)

Manager-owned implementation spec for the first connected load/reference-state
route. It binds the reviewed design (Git `9e8a55d`, CORRECTNESS_DESIGN/
LOAD_REFERENCE_STATES INTERFACE.md) to concrete JSON field names. ROOT-selected
identities are unchanged. Paths are WORKING_ROOT-relative. Quantities use the
existing `{value, unit}` shape and the existing units catalog. All new objects
are closed (`deny_unknown_fields`); unknown discriminants fail at the typed
boundary, so the request is rejected before solve. Branches known to the
reviewed interface but not implemented parse and then block with a targeted
diagnostic.

## Document selection

- `schema_version: "0.4.0"`, `document_kind: "openpipestress.product_preview.model"`.
- `pressure_contract: {"version":"2.0.0","mode":"exact_straight_pressure_v2"}` is
  required; the exact straight route and its rules (straight circular pipes, no
  components, linear restraints/springs only, no combinations, no
  equivalent-static, `pressure_regions` required per case, `[]` allowed) apply
  unchanged.
- Every load case requires `analysis_state`. `modulus_basis_ref` and
  `modulus_basis_temperature` must be absent. The request-level `materials`
  array must be empty; model `materials` are authoritative.
- A 0.1-0.3 document carrying `reference_configurations`, any material
  `expansion_laws` or any case `analysis_state` blocks with
  `LOAD_STATE_CONTRACT_VERSION_MISMATCH`. It is not ignored.

## Model-level owners

```json
"reference_configurations": [{
  "id": "reference:installed",
  "label": "Installed reference",
  "geometry_ref": {"kind": "authored_model_geometry"},
  "member_references": [{
    "pipe_ref": "pipe:a",
    "basis": {"kind": "temperature_reference",
              "installation_temperature": {"value": 20, "unit": "degC"}}
          | {"kind": "direct_strain_reference"},
    "fit": {"kind": "none"}
         | {"kind": "natural_length_change", "length_change": {"value": -2, "unit": "mm"}}
         | {"kind": "fit_strain", "strain": {"value": -0.0002, "unit": "1"}},
    "provenance": "invented test reference"
  }],
  "provenance": "invented test reference"
}]
```

`authored_model_geometry` means the model's own normalized nodes, connectivity
and pipe frames. Its projection hash is computed and published. Every pipe must
appear exactly once in the selected configuration's `member_references`. For
`natural_length_change`, the reference length is the member's authored chord
length.

Material records may carry `expansion_laws` (they are otherwise unchanged):

```json
{"id": "law:secant", "definition": "engineering_secant",
 "datum_temperature": {"value": 20, "unit": "degC"},
 "data": {"kind": "constant", "coefficient": {"value": 1e-5, "unit": "1/K"}}
       | {"kind": "table", "interpolation": "linear_coefficient",
          "points": [{"temperature": {...}, "coefficient": {...}}]},
 "provenance": "invented"}
{"id": "...", "definition": "engineering_dilation", "datum_temperature": {...},
 "data": {"kind": "table", "interpolation": "linear_dilation",
          "points": [{"temperature": {...}, "dilation": {"value": 0.0, "unit": "1"}}]},
 "provenance": "..."}
{"id": "...", "definition": "differential_per_datum_length" | "logarithmic_per_current_length",
 "datum_temperature": {...},
 "data": {"kind": "table", "interpolation": "linear_coefficient", "points": [...]},
 "provenance": "..."}
```

## Case `analysis_state`

```json
"analysis_state": {
  "contract": "openpipestress.load_reference_state/1.0.0",
  "reference_configuration_ref": "reference:installed",
  "element_states": [{
    "pipe_ref": "pipe:a",
    "operating_temperature": {"value": 100, "unit": "degC"},
    "material_selection":
        {"kind": "explicit_base_properties", "material_ref": "material:x",
         "applicability_reference": "user basis"}
      | {"kind": "exact_point", "material_ref": "material:x", "point_ref": "point:hot"}
      | {"kind": "temperature_interpolation", "material_ref": "material:x",
         "temperature": {...}, "interpolation": "piecewise_linear",
         "extrapolation": "forbidden"},
    "thermal_state":
        {"kind": "unchanged_reference", "provenance": "..."}
      | {"kind": "explicit_interval_strain", "strain": {"value": 0.001, "unit": "1"},
         "interval_reference": "...", "provenance": "..."}
      | {"kind": "constant_alpha_interval", "coefficient": {"value": 1e-5, "unit": "1/K"},
         "temperature_change": {"value": 80, "unit": "K"},
         "coefficient_meaning": "engineering_interval", "provenance": "..."}
      | {"kind": "free_length_state", "expansion_law_ref": "law:secant"},
    "analysis_basis_override": {"reason": "...", "provenance": "..."},
    "mass_state_ref": "..."
  }],
  "support_states": [{
    "support_ref": "support:root",
    "participation": {"kind": "active_model_device"},
    "boundary_motion": [{"dof": "UX", "value": {"value": 0.1, "unit": "mm"},
                         "meaning": "absolute_reference_displacement"}]
  }],
  "load_sources": [{"source_ref": "load:weight", "factor": 1.0}],
  "history": {"kind": "independent_equilibrium"},
  "provenance": "invented test case"
}
```

`operating_temperature`, `analysis_basis_override` and `boundary_motion` are
optional. The following parse but block as not implemented in this
capability: `mass_state_ref`, the `inactive` and `locked_equivalent_support`
participation kinds, `base_motion` and `device_reference`.

Rules enforced before assembly (each a targeted blocking diagnostic):

- **Element coverage:** exactly one `element_states` record per pipe.
  `material_ref` must equal the pipe's own `material`. Selection follows the
  private selector: an E/nu basis, derived G, no aliasing, and known
  actual-versus-selected temperature mismatch needs an override.
- **Free-length state:** `free_length_state` requires `operating_temperature`,
  a `temperature_reference` member basis, and a law on the same material.
- **Thermal/fit composition:** `ε* = λ_fit·λ_th − 1`. It is assembled once as
  the axial eigenload `E·A_s·ε*` of the member's resolved E, using the exact
  annulus wall area. Legacy `thermal` primitives referenced by `load_sources`
  block; unreferenced ones are excluded and reported.
- **Support coverage:** exactly one `support_states` record per model support.
  `boundary_motion` applies only to DOFs the support already restrains
  rigidly. Translations take a length unit (m) and rotations an angle unit
  (rad). Each DOF appears at most once. Unlisted restrained DOFs stay at
  explicit zero. Springs cannot take `boundary_motion`.
- **Load sources:** `load_sources` is the complete inclusion list for the
  owning case's stored `primitive_loads`. Every `source_ref` must name a
  primitive stored in that case. Duplicate `source_ref` values block
  (`LOAD_STATE_SOURCE_DUPLICATE`). The factor must be finite and nonzero; it
  scales the primitive magnitude. Unreferenced stored primitives are not
  applied and appear as `excluded`.
- **History:** `independent_equilibrium` is the only history kind.

## Solve and results

The exact route is used unchanged, except:

- per-member E/nu/G drives stiffness and pressure recovery;
- prescribed tuples go through the structural solve and the nonzero reduction;
- reactions are `K·u − f` from the complete `u`, including prescribed values.

Result rows keep their exact-profile kinds and IDs: `result:disp:*`,
`support_reaction_component_v2` (support-on-pipe, global), element end forces
and the station/stress rows. Producer semantics are
`openpipestress.result_semantics/0.3.0/load-reference-1`. The formulation
profile is `resolved_straight_load_state_v1`. Evidence is published as
`contract_evidence.load_reference_states[]`, one record per case, alongside the
existing `pressure` and `exact_cases`.

Retained-source recovery is not attempted for 0.4.0. Each case carries
`LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED` and no `source_block_recovery` is
published.
