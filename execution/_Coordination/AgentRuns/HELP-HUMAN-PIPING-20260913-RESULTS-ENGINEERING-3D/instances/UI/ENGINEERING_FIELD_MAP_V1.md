# Engineering field and availability map V1

This is a derivative UI design map from approved CANDIDATE_PLAN_V3 §B/C, awaiting root relay of the frozen coupled engineering contract. It creates no engineering version, default, solver input or acceptance. New pressure/connector fields appear as disabled backfill references in the prototype. Existing ordinary pressure/support/component fields remain within their current scoped operation paths.

| Task | Inspector group | Explicit field concepts | Availability and non-inference |
|---|---|---|---|
| Choose pressure formulation | Formulation and model basis | legacy_pressure_v1; candidate pressure version and applicable document version | Existing legacy path may be illustrated. Candidate mode is unavailable until its version/material/region contract is accepted and implemented |
| Choose material basis | Material | authoritative E and nu, temperature/basis reference; derived G readout | No independent E/G/nu editing that violates the proposed relation; no silently derived nu from old records |
| Define pressure region | Region and ends | connected collinear equal-bore source set; region identity; inside/outside pressure; explicit terminal transfer/closure treatment | No automatic region completion or pressure inference from visible connection alone |
| Inspect pressure effect | Load-path review | wall force, effective force, cap transfer, pressure eigenload, thermal eigenload, source quantity/basis | Separate quantities; no double counted legacy longitudinal stress; no apparent zero for unavailable quantities |
| Define connector geometry | End A / End B | stable end refs; named local frames, offsets, constitutive reference | Candidate symmetric_midpoint_small_rotation_v1 only; no implicit world-frame assumption or raw endpoint difference |
| Define connector stiffness | Constitutive data | coupled stiffness entries, units/scaling, source and PSD-check feedback | Missing coupling data remains missing; no diagonalization or symmetry repair behind UI |
| Define reference/prestress | Reference and initial state | constitutive reference and initial residual/prestress, named basis | Current shape cannot silently become stress-free reference |
| Select connection topology | Topology | replacement, series or parallel relationship; exact member/source refs | No whole-span parallel stiffness inferred from a visual connector symbol |
| Inspect connector result | Relative motion and force | work-conjugate coordinates and matching generalized forces, end frames, units/signs | No manufacturer equivalence, finite-rotation or composed pressure-joint behavior |

The production design should visualize end frames and offset arrows when inspecting a connector. A compact read-only diagram can accompany fields, but it must distinguish geometric anchors, constitutive reference and chosen topology. Any active transform preview should list what moves and which attached references block it before Apply. That geometry UI backfill is specific; it does not delay current route/workspace design.

Root contract relay may replace this derivative map with exact field keys/availability. Preserve this V1 and publish a successor rather than imply these labels are final schema names. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
