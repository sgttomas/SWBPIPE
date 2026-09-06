# P5 bounded planning — R05/R09 V1

Status: planning only. Shared product source remains P4-owned; no tests or source edited. Source observation 2be412ccea62bdc4bd96deb082c46d7a792076ea. Root accepted S1 V3/R1 PASS and activation REPAIR_ACTIVATION_V1 are the derivative upstream basis; this plan does not amend decomposition truth.

## Scope and constraints

PKG-05 selected deliverables DEL-05-01, DEL-05-02, DEL-05-03, DEL-05-05 are SOW_V1 and IN_PROGRESS. Read live ScopeOfWork, contexts, status and paired memory; empty Remaining sections do not establish closure. Package scope SOW-013/014/015/052, OBJ-003/005/012. DEL05-01 OUT001/AC001 requires explicit missing-input findings; DEL05-02 OUT001/AC001 requires compatible deterministic unit-aware expression evaluation/provenance; DEL05-03 RQ003/004/005 requires units, missing-input findings and source edges. Invariants OPS-K-DATA-2, UNIT-1, SOLVER-1, REPORT-1, MECH-2, AGENT-1/2/3 apply. F-PIP-2/DEC081 and D01–D06 hold. No public schema, lower kernel or PKG03 truth edits.

## R05 plan

At product solve_load_case, validate each thermal load against the already-selected effective material map before build_thermal_element_loads can silently skip alpha. Preserve existing base-input requirements, exact point and interpolation selection rules; require alpha only on targeted thermal materials. Emit blocking existing diagnostic mechanism with case/load/material references. Do not invent alpha or make absence on unrelated nonthermal materials a failure. Frozen I1-C06 witness remains the pre-repair reference; missing selected alpha blocks and valid selected alpha gives independent EA alpha deltaT response. Do not remove existing base-alpha validation under this repair.

At append_equivalent_static_generated_loads, require whole_span_pipe_indices.insert(pipe_index) to succeed before exposed.push. Match normal author's existing duplicate whole-pipe marking prohibition, preserve allowed disjoint partial extents and existing overlap/whole-plus-partial checks. I1-C05 duplicate request must block rather than double response, single occurrence unchanged.

## R09 plan

Keep one internal full-precision numeric result layer until all case and combination operations complete. Existing ResultItem type can remain internal too: remove premature numeric producer round6 and use a single publication pass after append_combination_results and other numeric producers. Keep rounded_scalar textual provenance formatting separate. Final finite checks must cover arithmetic and publication, since naive value*1e6 can overflow on finite values. Preserve six-decimal public presentation and round summaries at the same terminal boundary. Internal threshold computations should consume full precision; existing values/thresholds unchanged.

Within each case create an original-ID to emitted-ID map before output. Rewrite only source_result_refs present in that map; preserve source load IDs and unrelated provenance. This repairs the later-case twenty-edge witness without new IDs or schema.

For arithmetic sum/subtraction combinations, combine signed component rows then derive displacement/reaction force magnitude from matching vector components. Never algebraically sum primitive positive norms. If required components missing, withhold the derived magnitude with explicit existing diagnostics; do not default missing components to zero. Preserve separately authored range/envelope semantics, which are scalar extrema operations. Existing scalar stress summaries remain excluded. Source refs for derived magnitudes should point to actual combined signed component rows. Test cancellation of +350/-350, reordered operands, complete vectors, deliberate missing component, range regression and all supported magnitude families.

Precision witnesses: preserve independent frozen .00035 N primitive, factor 2 versus .0007 N direct, factor 1000000 versus 350 N direct in both solver modes. Full-precision arithmetic must precede six-decimal output; do not adjust old independent expected quantities.

## Exact likely targets and dependency interface

Production: core/product_physics/src/lib.rs only, subject to accepted P4 transfer hash and exact implementation seal. Targeted tests: same module or core/product_physics/tests/p5_physics_audit.rs; P9 owns validation/** and tests/** exclusively. Lower APIs read-only.
P4 must expose the same selected displacement state to all recoveries and genuine support action components for reaction magnitudes. P5 owns case source-edge qualification and final numerical publication after algebra. Any intermediate rounded nonlinear row must not become a mechanics input.

## Gates

Existing user authorization covers internal choices only. No source author dispatch until accepted P4 predecessor and explicit root transfer. One integration author, then fresh TASK software-code-review over 100% frozen source/test diff; manager independent validation; original witnesses and independent P9 tests; root clean DEC025/native final checks. No build until root schedules slot. R08/R11 detailed chain supplied by separate read-only reconnaissance child before author seal. Package records updated only after bounded source closure without lifecycle promotion.
