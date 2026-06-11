---
doc_id: OPS-SOFTWARE-DECOMP
package_role: working_surface
doc_kind: decomposition.software
status: current_basis
revision: 0.7
created: 2026-04-30
refs:
  - rel: depends_on
    to: OPS-INTENT
  - rel: depends_on
    to: OPS-PRD
  - rel: governed_by
    to: OPS-CONTRACT
---

# SOFTWARE_DECOMP — OpenPipeStress Agent-Executable Software Decomposition

## 1. Intake summary

OpenPipeStress is a proposed free/open-source, code-neutral piping stress analysis platform. It will implement open global piping flexibility mechanics, provide a GUI and reporting workflow, and require users to supply governing code data, material properties, component data, SIFs/flexibility factors, allowables, and rule-pack checks.

This revision incorporates the formal SOFTWARE_DECOMP review pass against `PRD.md` and `INTENT.md`, plus the v0.3 architecture-runway amendment. The v0.3 amendment was accepted by the human project authority on 2026-04-30 as the current downstream decomposition basis.

The v0.3 amendment adds `PKG-00 — Software Architecture Runway` as a pre-workflow gate before package-level implementation planning. `PKG-00` defines the product/software spine that keeps GUI, backend services, persistence, schemas, diagnostics, APIs, jobs, and tests coherent before Type 2 implementation proceeds across `PKG-01` through `PKG-12`.

Revision v0.4 records `SCA-001`, which authorizes `PKG-00` `SEMANTIC_READY` content as an architecture-basis candidate for sealed brief injection without treating `PKG-00` as `ISSUED`. It propagates the accepted architecture basis into downstream context/dispatch surfaces and resolves selected stack/runtime/schema/persistence/test TBDs where the architecture runway made them imperative.

Revision v0.5 records `SCA-002`, which admits `docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md` as the working product-design basis for a schema-backed piping design engine and stress-model authoring environment. The amendment preserves the full solver, IP boundary, rule-pack, validation, and professional-boundary scope, while adding decomposition coverage for physical design knowledge, constraints, model operations, immutable model states, analysis runs, deterministic comparison, handoff packages, and external-prover workflow boundaries. Downstream production documents, DAG files, implementation evidence, lifecycle states, and dependency registers remain stale until refreshed by their owning workflows.

Revision v0.6 records `SCA-003`, which resolves the MVP physical project package/container decision as a local SQLite-backed project store and index profile with rebuildable retrieval sidecars. The amendment preserves JSON Schema 2020-12, canonical JSON/JCS-compatible hashes, deterministic round-trip behavior, local-first privacy, and application-service no-bypass boundaries as the canonical persistence contract.

Revision v0.7 records `SCA-004`, which adds `PKG-17 — Export Format Interoperability` and makes deterministic export packages, export profiles, stable ID maps, loss reports, CAEPIPE MBF validation workflow, stress-neutral CSV/JSON, conservative PCF, GLB/glTF review geometry, and export adapter SDK work explicit under the existing interoperability, result-export, handoff, and external-prover boundaries. The amendment preserves the no-bundled-commercial-solver, no-proprietary-example, no-reverse-engineering, no-license-bypass, no-code-compliance-claim, and no-professional-acceptance-claim constraints.

This decomposition intentionally separates:

- **open mechanics** from **protected standards/code data**;
- **mechanics solved** from **user-rule checked** from **human professional approval**;
- **global centerline analysis** from **specialized local FEA handoff**.

## 2. References

- `INTENT.md` — project intent and governing philosophy.
- `PRD.md` — product requirements and release strategy.
- `MWK 1956.pdf` — historical/foundational technical reference for classical piping flexibility analysis.
- `AGENT_SOFTWARE_DECOMP.md` — decomposition method and required output structure.
- `PROFESSIONAL_ENGINEERING.md` and `CHIRALITY_FRAMEWORK.md` — professional accountability and epistemic governance lens.

## 3. Vocabulary map

|CanonicalTerm|Synonyms|Notes|
|---|---|---|
|OpenPipeStress|Open Pipe Stress, FOSS piping stress tool|Working product name; final name TBD.|
|Rule Pack|code pack, check pack, B31-style rules|User-supplied artifact containing formulas/allowables/checks.|
|Mechanics Solve|analysis solve, pipe stress run|Numerical result only, not compliance.|
|User Rule Check|code check, stress check|Computation under user-supplied rule pack.|
|Protected Standards Data|ASME data, code tables, proprietary tables|Excluded from public repository.|
|Private Library|company library, local database|User-owned materials/components/rules.|
|Centerline Model|beam model, line-element model|Default global piping analysis representation.|
|Local FEA Handoff|solid model export, shell analysis export|Specialized local analysis workflow.|
|Project Persistence|project file, project storage, round-trip serialization|Added in v0.2 from PRD FR-001.|
|MVP Local Storage Profile|SQLite project store, embedded local database, project index|Accepted by SCA-003 as the MVP physical storage substrate; canonical JSON/JCS remains the domain contract.|
|Retrieval Sidecar|BM25 index, FTS5 index, optional NumPy cache|Rebuildable local index/cache, not canonical storage or interchange truth.|
|External Artifact Reference|large-file reference, path-and-hash reference|Large files are referenced in place by path/URI plus hash/metadata by default; portable copy/export is a later explicit workflow.|
|Solver Diagnostics|singularity detection, conditioning warnings, convergence status|Added in v0.2 from PRD §11.8/§20/R1 exit criteria.|
|Software Architecture Runway|architecture spine, pre-implementation gate|Added in v0.3 to define cross-cutting software contracts before implementation.|
|Application Service Boundary|command/query/job model, GUI-to-core service layer|Coordinates GUI, domain core, solver, storage, reporting, and adapters.|
|Architecture Basis|accepted architecture constraint, brief-injection basis|`PKG-00` `SEMANTIC_READY` material approved by SCA-001 for downstream sealed briefs without changing lifecycle state to `ISSUED`.|
|Brief Injection|sealed-context propagation, dispatch-basis injection|Placement of applicable architecture basis IDs and resolved/TBD boundaries into downstream `_CONTEXT.md` and future TASK briefs.|
|Resolved Architecture Baseline|approved stack/runtime/schema/test baseline|Human-approved choices that downstream implementation briefs must respect unless changed by later scope change.|
|Design Knowledge|routing constraints, support zones, equipment interfaces, project design context|Added by SCA-002 from PRD v0.2 as user-supplied context for model authoring and validation.|
|Physical Model|editable design model, schema-backed source of truth|Added by SCA-002; richer than the analytical solver model.|
|Analytical Model|solver-ready idealization, analysis basis|Derived from the physical model with transformation warnings and traceability.|
|Model Operation|structured edit, proposed operation, accepted mutation|All GUI and agent changes route through validated operations.|
|Model State|immutable model snapshot, saved design state|Named state used for reproducibility, reports, comparisons, and handoff.|
|Analysis Run|solver execution record, result state|Binds results to a specific model state, solver version, settings, load cases, and rule/library references.|
|Comparison|state diff, result diff|Deterministic comparison of model states and/or analysis runs; not professional validation.|
|Handoff Package|external prover package, downstream export manifest|Schema-compliant export for downstream modeling and professional validation workflows.|
|Export Profile|target profile, writer profile, export target contract|Added by SCA-004; versioned target-specific configuration for deterministic export writers and validation harnesses.|
|Stable ID Map|id map, target mapping sidecar|Added by SCA-004; maps canonical model IDs to target file names/numbers/records when the target cannot carry canonical IDs directly.|
|Loss Report|unsupported behavior report, export limitation report|Added by SCA-004; machine-readable and human-readable record of exported, omitted, approximated, or delegated behavior.|
|External Run Harness|solver harness, target validation harness|Added by SCA-004; optional user-owned invocation path for licensed downstream tools, producing non-authoritative regression evidence only.|
|Review Geometry Export|GLB export, glTF export, visual handoff export|Added by SCA-004; lightweight geometry for visual review, not solver geometry or professional validation.|


## 4. Structured Scope of Work (SSOW)


|ScopeItemID|Status|Statement|SourceRef|Notes|
|---|---|---|---|---|
|SOW-001|IN|The project shall produce a source-available noncommercial piping stress analysis platform.|INTENT.md §Core intent; PRD §1, §4|License selected by human project authority on 2026-06-03: PolyForm-Noncommercial-1.0.0.|
|SOW-002|IN|The software shall remain code-neutral: the solver computes mechanics and user-supplied rule packs evaluate acceptability.|INTENT.md §Product identity; PRD §6.1|Do not encode a proprietary code as the default public product.|
|SOW-003|IN|The public repository shall not redistribute protected standards text, tables, figures, examples, material allowables, SIF tables, flexibility-factor tables, or protected dimensional tables.|INTENT.md §Copyright-respecting design principle; PRD §5, §17|Legal review required before accepting any public data contribution.|
|SOW-004|IN|The system shall require users to supply code-specific and project-specific data needed for code checks.|INTENT.md §User-supplied code and data layer; PRD §6.1, §12|Rule-pack completeness must be machine-checkable.|
|SOW-005|IN|The analytical core shall model piping as a 3D centerline/frame system with six degrees of freedom per node.|INTENT.md §Analytical engine intent; PRD §11.1-§11.2|Primary global model is line-element, not routine solid meshing.|
|SOW-006|IN|The solver shall implement straight pipe elements with open mechanics section and stiffness calculations.|PRD §11.3.1|Pipe dimensions come from user data or lawful libraries.|
|SOW-007|IN|The component model shall support bends and elbows with user-entered SIFs, flexibility factors, and bend geometry.|INTENT.md §Analytical engine intent; PRD §11.3.2|Do not bundle B31J-derived tables/equations.|
|SOW-008|IN|The component model shall support branch connections with user-entered reinforcement, SIF, flexibility, and local data fields.|PRD §11.3.3|Branch local checks may require future specialized modules.|
|SOW-009|IN|The component model shall support rigid and semi-rigid valves, flanges, reducers, and specialty items using user-supplied dimensions, weights, and centers of gravity.|PRD §11.3.4-§11.3.5|Public component data must be licensed.|
|SOW-010|IN|The component model shall support expansion joints using manufacturer/user-supplied stiffness, effective area, movement limits, and hardware data.|PRD §11.3.6|No invented expansion joint defaults.|
|SOW-011|IN|The solver shall support anchors, guides, line stops, vertical supports, springs, and imposed displacements.|PRD §11.4|Initial release may limit to linear supports.|
|SOW-012|IN|The solver shall support nonlinear support behavior such as one-way restraints, lift-off, gaps, and friction through a controlled iterative method.|PRD §11.4, §22.4|MVP may defer some nonlinear cases.|
|SOW-013|IN|The analysis engine shall support primitive load cases for weight, pressure, thermal expansion, imposed displacement, hydrotest, wind, seismic, and occasional loads.|PRD §11.5|Dynamic modules may be deferred.|
|SOW-014|IN|The analysis engine shall support unit-aware load-case algebra and user-defined combinations.|PRD §11.6|Code-specific combinations supplied by user rule packs.|
|SOW-015|IN|The stress module shall recover fundamental mechanics stresses from forces, moments, pressure, and section properties.|PRD §11.7|Code stress categories are rule-pack mappings.|
|SOW-016|IN|The product shall provide a private rule-pack schema for user-defined stress checks, allowables, formulas, and pass/fail criteria.|INTENT.md §Rule-pack intent; PRD §12|Public examples must use invented values.|
|SOW-017|IN|The product shall support private material libraries with temperature-dependent properties, allowables, and provenance fields.|PRD §13.1|No public ASME material allowable tables.|
|SOW-018|IN|The product shall support private pipe section and component libraries with provenance and redistribution status.|PRD §13.2-§13.4|Public data requires documented rights.|
|SOW-019|IN|The public project shall accept component data only when provenance and redistribution rights are documented.|PRD §13.4, §17.2|Contributor certification required.|
|SOW-020|IN|The GUI shall provide a 3D centerline modeler with model tree and piping component visualization.|PRD §14.1-§14.2|GUI baseline is Tauri 2 desktop shell with TypeScript/React/Vite and Three.js viewport; exact component/state libraries remain implementation TBD.|
|SOW-021|IN|The GUI shall provide editors for materials, sections, components, load cases, supports, rule packs, and private libraries.|PRD §14.3, §14.5-§14.6|Workflow should surface missing data early.|
|SOW-022|IN|The GUI shall distinguish data required for solving from data required for code checking and shall block/qualify results when required data is missing.|PRD §14.4|No silent engineering defaults.|
|SOW-023|IN|The GUI shall provide results review for displacements, rotations, forces, moments, restraint reactions, equipment loads, stresses, and ratios.|PRD §14.2, §15.1|Stress ratios depend on rule pack completeness.|
|SOW-024|IN|The product shall generate auditable calculation reports including inputs, sources, warnings, assumptions, results, rule-pack checksums, and limitations.|PRD §15|Reports must not reproduce protected standards content.|
|SOW-025|IN|All calculations, schemas, imports, exports, and rule evaluations shall be unit-aware.|PRD §6.6|Unit conversions must be deterministic and testable.|
|SOW-026|IN|The project shall maintain verification benchmarks, regression tests, and numerical quality checks for solver and stress recovery behavior.|PRD §16.2-§16.4|Benchmark sources must be public/original/permissive.|
|SOW-027|IN|The project shall maintain a validation manual that distinguishes mechanics verification from code compliance and professional signoff.|PRD §16.5|Human engineering judgment remains required.|
|SOW-028|IN|The repository shall include contributor governance, IP controls, and review procedures preventing copyrighted standards data from entering the public project.|PRD §17.1-§17.2|Use stop-and-escalate for suspected protected content.|
|SOW-029|IN|The product shall be local-first and shall protect private rule packs, private material data, private component data, and project models.|PRD §18.1, §18.3|Cloud services are out of MVP unless approved.|
|SOW-030|IN|The architecture shall support public APIs, plugins, and import/export adapters for integration with external tools.|PRD §19.3|API baseline is schema-first command/query/job result envelopes; SCA-004 adds deterministic export profiles, native JSON packages, CAEPIPE MBF, conservative PCF, GLB/glTF review geometry, and adapter SDK work under no-bypass plugin/adapter boundaries.|
|SOW-031|IN|The product shall support optional handoff of selected local-detail problems to external shell/solid FEA workflows.|INTENT.md §Analytical engine intent; PRD §4.2, §19|Local FEA is not the normal global analysis method.|
|SOW-032|IN|The project shall provide reproducible build, packaging, and CI/CD workflows for supported platforms.|PRD §22|Desktop packaging baseline follows Tauri-supported macOS/Windows/Linux targets; CI host and release matrix details remain TBD.|
|SOW-033|IN|The project shall provide user/developer documentation and invented-data examples for education and testing.|PRD §7.5, §16.5, §22|No protected examples.|
|SOW-034|IN|The product shall preserve professional responsibility boundaries: agents/software do not certify, approve, seal, or declare engineering code compliance for reliance.|PROFESSIONAL_ENGINEERING.md; CHIRALITY_FRAMEWORK.md; PRD §5, §17.4|Reports may show user-rule pass/fail, not human code compliance.|
|SOW-035|IN|The solver shall be designed for sparse numerical performance and reproducible results on practical piping models.|PRD §11.8, §20|Performance targets TBD.|
|SOW-036|IN|The GUI and reports shall support baseline accessibility and usability appropriate for engineering review.|PRD §21|Detailed WCAG target TBD.|
|SOW-037|IN|Telemetry, if implemented, shall be opt-in, privacy-preserving, and shall not transmit private project/code data.|PRD §18.2|MVP default is no telemetry.|
|SOW-038|IN|The software architecture shall be extensible without allowing plugins/adapters to bypass governance, unit safety, or data-boundary constraints.|PRD §19.1-§19.3|Plugin/adapter baseline is deny-bypass: units, provenance, diagnostics, rule sandboxing, and report controls remain mandatory; detailed permissions TBD.|
|SOW-039|IN|The product shall produce reproducibility artifacts such as model hashes, solver version, rule-pack checksum, and input manifest.|PRD §15.3|Canonical JSON with JCS-compatible hashing is the JSON payload hash basis; non-JSON/binary assets require manifest hashes.|
|SOW-040|IN|The product shall implement private data handling controls, including redaction/export safeguards where reports or shared models may expose protected/private values.|PRD §18.3|Detailed threat model TBD.|
|SOW-041|IN|The project shall define machine-readable project, model, material, component, load, result, and report schemas.|PRD §19.1, Appendix A|JSON Schema 2020-12 is the public schema/interchange baseline; code-generation tooling remains TBD.|
|SOW-042|IN|Rule packs shall be versioned, source-noted, checksum-addressed, and explicitly marked for redistribution status.|INTENT.md §Rule-pack intent; PRD §12.4|Private rule packs should not be committed publicly.|
|SOW-043|IN|The report system shall prohibit automatic inclusion of protected code text, copied standards tables, or proprietary formulas in generated public reports/templates.|PRD §15.2|User-private report templates remain user responsibility.|
|SOW-044|IN|The product shall provide import mechanisms that record source/provenance/license metadata for component and material data.|PRD §13.5|Importer may reject/flag missing provenance.|
|SOW-045|IN|The rule-pack evaluator shall be sandboxed and unit-aware, not arbitrary executable code.|PRD §12.3|Expression language TBD.|
|SOW-046|IN|The product shall support result exports suitable for review, regression comparison, and downstream tooling.|PRD §15.1, §19.3|Structured result export baseline is schema-first JSON result envelopes for review/regression; SCA-004 adds stress-neutral CSV/JSON packages and external-harness result parsing for regression and downstream tooling, without code-compliance or professional-acceptance claims.|
|SOW-047|IN|The product shall clearly distinguish mechanics solved, rule-pack checked, and professionally code-compliant/human-approved states.|INTENT.md §GUI intent; PRD §6.2, §17.4|Human approval is outside software authority.|
|SOW-048|IN|The project shall define project license, governance, release, and maintainer policies.|PRD §17.2, §22|License selected by human project authority on 2026-06-03: PolyForm-Noncommercial-1.0.0.|
|SOW-049|IN|The project shall define criteria for when global beam analysis is sufficient and when local shell/solid analysis handoff is needed.|INTENT.md §Analytical engine intent; PRD §4.2|Criteria are guidance, not automatic certification.|
|SOW-050|IN|The product shall support create/open/save/versioned project persistence with deterministic round-trip serialization of models, units, loads, rule-pack references, and provenance metadata.|PRD §10 FR-001; PRD §27|Persistence baseline is versioned JSON-schema-governed canonical JSON with migration status and JCS-compatible hashes; MVP physical storage profile is local SQLite-backed project store/index with rebuildable sidecars.|
|SOW-051|IN|The product shall calculate pipe section and mass properties from user-entered dimensions and material data with unit checks.|PRD §10 FR-005; PRD §13.2|No protected pipe dimensional tables are introduced.|
|SOW-052|IN|The load engine shall support concentrated forces, concentrated moments, and distributed user loads in addition to core primitive piping load categories.|PRD §10 FR-007; PRD §11.5|Code-specific load combinations remain user/rule-pack supplied.|
|SOW-053|IN|The solver shall detect and report singular, ill-conditioned, and nonconverged analysis states with deterministic diagnostics.|PRD §11.8; PRD §20; PRD §22.2|Supports the R1 exit criterion that the solver detects singular models.|
|SOW-054|IN|The project shall provide a headless command-line or structured I/O analysis runner for early solver verification, regression, and automation workflows.|PRD §19.3; PRD §22.1|Provides the R0/R1 path before full GUI maturity.|
|SOW-055|IN|The GUI shall keep solve execution reviewable with background solve execution, progress, cancellation, and diagnostic presentation.|PRD §20; PRD §21; PRD §22.3|Keeps GUI responsiveness and diagnostic traceability visible to users.|
|SOW-056|IN|The project shall select and record implementation stack, runtime, GUI framework, solver-library, and packaging target decisions before implementation work proceeds.|Architecture runway amendment v0.3; PRD §19.2, §22, §25 Open Question 3|Architecture baseline selected by SCA-001; exact dependency versions and solver numerical library remain implementation-level TBD.|
|SOW-057|IN|The project shall define repository/module architecture, ownership boundaries, dependency rules, and layer responsibilities before package-level implementation proceeds.|Architecture runway amendment v0.3; SPEC §1-§2|Repository/module decisions are injected through SCA-001 architecture basis; concrete folder names may refine during implementation without changing layer responsibilities.|
|SOW-058|IN|The project shall define application-service command, query, background job, cancellation, and transaction boundaries between GUI, headless runner, domain core, solver, storage, and reporting.|Architecture runway amendment v0.3; PRD §20, §22.1-§22.3; SPEC §1|Command/query/job and result-envelope boundaries are dispatch constraints; concrete interface language details remain implementation-level TBD.|
|SOW-059|IN|The project shall define persistence format, schema-versioning, migration, canonicalization, and deterministic round-trip strategy before project-file implementation proceeds.|Architecture runway amendment v0.3; SOW-050; PRD §27|Persistence architecture basis selected by SCA-001 and MVP local storage profile selected by SCA-003; migration framework remains TBD.|
|SOW-060|IN|The project shall define GUI state, editing, selection, undo/redo, viewport integration, and user workflow architecture before GUI feature slices proceed.|Architecture runway amendment v0.3; PRD §14, §20, §21|GUI state basis selected by SCA-001: durable/transient split, service-command mutation route, and scoped undo/redo; exact state library remains TBD.|
|SOW-061|IN|The project shall define diagnostics, warning classes, errors, result envelopes, and user-facing/machine-readable failure contracts across solver, rule packs, GUI, CLI, reports, and adapters.|Architecture runway amendment v0.3; SOW-022, SOW-053, SOW-055; SPEC §7|Diagnostics/result-envelope basis selected by SCA-001; exact schema syntax and localization policy remain TBD.|
|SOW-062|IN|The project shall define public and internal API boundaries between GUI, application services, domain core, solver, storage, reporting, private libraries, plugins, and adapters.|Architecture runway amendment v0.3; SOW-030, SOW-038; SPEC §1|API boundary basis selected by SCA-001: schema-first envelopes and no-bypass adapters/plugins; public transport protocol remains TBD.|
|SOW-063|IN|The project shall define layered software test strategy and acceptance gates for architecture, schemas, services, solver, GUI, CLI, reports, packaging, security, and regressions.|Architecture runway amendment v0.3; SOW-026, SOW-032; PRD §16, §22|Layered test basis selected by SCA-001: Cargo, Vitest, Playwright, validation and protected-content gates; CI provider and thresholds remain TBD.|
|SOW-064|IN|The product shall be scoped as an analysis-grade piping design engine and stress-model authoring environment with a full internal solver and non-authoritative professional reliance boundary.|PRD v0.2 §1-§5; Scope Change Brief §1-§2|Supersedes the narrower product framing while preserving solver seriousness and professional-boundary limits.|
|SOW-065|IN|The schema-backed physical model shall be the source of truth for editable piping design data, including geometry, components, supports, equipment interfaces, loads, constraints, provenance, unresolved assumptions, and warnings.|PRD v0.2 §5.2, §8.2, FR-MOD-002, FR-MOD-005, FR-MOD-006|Physical model is richer than the solver-ready analytical model.|
|SOW-066|IN|The product shall transform the physical model into a solver-ready analytical model deterministically and record transformation warnings when physical design data cannot be represented analytically.|PRD v0.2 §8.3, FR-MOD-007|Transformation is a first-class contract between design authoring and solver execution.|
|SOW-067|IN|The product shall support user-supplied design knowledge such as endpoints, line data, routing corridors, no-go volumes, supportable zones, equipment interfaces, access constraints, slope/drain/vent requirements, and owner/project metadata.|PRD v0.2 §8.1, §11.2, §13.1|Design knowledge remains user/project supplied; no protected owner or standards data is bundled.|
|SOW-068|IN|The constraint system shall validate available design knowledge for connectivity, route conflicts, clearance conflicts, support-zone conflicts, slope/drain/vent conflicts, and missing required data with provenance-aware messages.|PRD v0.2 FR-KNOW-002 through FR-KNOW-004; §13.2|Constraint failures are validation messages, not hidden report prose or agent text.|
|SOW-069|IN|All GUI and agent edits shall be represented as structured model operations that pass schema validation, constraint validation, diff preview, and controlled application through the model engine.|PRD v0.2 §5.3, §10.3, FR-AGENT-001 through FR-AGENT-003|Agents may propose operations; they do not mutate accepted engineering state directly.|
|SOW-070|IN|Accepted model operations shall preserve operation history, rationale, assumptions, affected entities, and audit metadata needed for reproducible model-state review.|PRD v0.2 §10.3, FR-AGENT-004, §15.1|Operation history supports design iteration without creating professional approval claims.|
|SOW-071|IN|The product shall save named immutable model states with tags, notes, external references, unresolved assumptions, warnings, and deterministic hashes.|PRD v0.2 §8.6, FR-CMP-001, §15.1|Model states are flexible metadata records, not formal prover approval states.|
|SOW-072|IN|The product shall save analysis runs bound to exact model states, solver versions, settings, units, load cases, diagnostics, results, rule-pack references, library references, and result hashes.|PRD v0.2 §8.7, FR-CMP-002, §15.2|Results attach to immutable analysis runs rather than a mutable current model only.|
|SOW-073|IN|The product shall compare two model states and/or two analysis runs deterministically using stable IDs, manual mappings where required, unit-normalized result deltas, and tolerance profiles.|PRD v0.2 §8.8, FR-CMP-003 through FR-CMP-010, §15.3|Comparison is diagnostic/audit functionality, not automatic external validation or acceptance.|
|SOW-074|IN|The product shall generate schema-compliant handoff packages with model hash, units manifest, entity IDs, library/rule references, unresolved assumptions, warnings, target mapping metadata, and unsupported-target flags.|PRD v0.2 §16.1, FR-HAND-001 through FR-HAND-004|Handoff quality is central; SCA-004 makes stable ID maps, export manifests, loss reports, target mapping metadata, unsupported/approximated/delegated behavior reporting, CAEPIPE MBF, conservative PCF, GLB/glTF review geometry, and native JSON export packages explicit scope.|
|SOW-075|IN|The product shall support external-prover workflow metadata without forcing a formal prover-status lifecycle, automatic professional acceptance record, or comprehensive commercial-tool result ingestion in the MVP.|PRD v0.2 §4.2, §16.2-§16.4, §23|Use flexible names, tags, notes, external references, comparison reports, and optional user-owned external harness metadata instead of hard-coded approval statuses; SCA-004 permits CAEPIPE harness records only as non-authoritative external-run evidence.|
|SOW-076|IN|The GUI shall support design-authoring and comparison workflows, including design knowledge panels, constraint/warning panels, state/run browsers, comparison tables, and graphical comparison overlays.|PRD v0.2 §11.3, §14.5, §15.3|Expands GUI from stress input editing into design iteration and review.|

## 5. Objectives


|ObjectiveID|Statement|Mapped Scope Items|Mapped Deliverables|
|---|---|---|---|
|OBJ-001|Deliver an open, auditable piping stress analysis platform that engineers can inspect and extend.|SOW-001, SOW-002, SOW-024, SOW-033, SOW-041, SOW-050|DEL-01-01, DEL-02-01, DEL-02-02, DEL-02-03, DEL-02-05, DEL-11-01, DEL-11-02, DEL-11-03, DEL-11-04, DEL-11-05|
|OBJ-002|Protect standards-body and vendor intellectual property by separating open mechanics from user-supplied code/data.|SOW-003, SOW-004, SOW-016, SOW-028, SOW-042, SOW-048|DEL-01-01, DEL-01-02, DEL-01-03, DEL-03-07, DEL-06-03, DEL-06-04, DEL-08-05, DEL-11-02, DEL-11-05|
|OBJ-003|Implement a robust global centerline/frame solver for practical piping flexibility analysis.|SOW-005, SOW-006, SOW-011, SOW-012, SOW-013, SOW-014, SOW-015, SOW-035, SOW-052, SOW-053|DEL-04-01, DEL-04-02, DEL-04-03, DEL-04-04, DEL-04-05, DEL-04-06, DEL-05-01, DEL-05-02, DEL-05-03, DEL-05-05, DEL-11-03|
|OBJ-004|Support piping-specific components and private libraries without bundling protected data.|SOW-007, SOW-008, SOW-009, SOW-010, SOW-017, SOW-018, SOW-019, SOW-044, SOW-051|DEL-03-01, DEL-03-02, DEL-03-03, DEL-03-04, DEL-03-05, DEL-03-06, DEL-03-07, DEL-03-08|
|OBJ-005|Provide user-defined rule packs that evaluate solver results against user-owned design bases.|SOW-004, SOW-014, SOW-016, SOW-042, SOW-045, SOW-047|DEL-05-02, DEL-05-04, DEL-06-01, DEL-06-02, DEL-06-03, DEL-06-04, DEL-06-05|
|OBJ-006|Provide a GUI workflow that makes model creation, missing data, results, and assumptions visible.|SOW-020, SOW-021, SOW-022, SOW-023, SOW-036, SOW-055|DEL-07-01, DEL-07-02, DEL-07-03, DEL-07-04, DEL-07-05, DEL-07-06, DEL-07-07|
|OBJ-007|Generate reproducible reports and result exports suitable for professional review.|SOW-024, SOW-039, SOW-043, SOW-046, SOW-055|DEL-07-05, DEL-07-07, DEL-08-01, DEL-08-02, DEL-08-03, DEL-08-04, DEL-08-05, DEL-08-06, DEL-17-05, DEL-17-06|
|OBJ-008|Maintain rigorous verification, validation, regression testing, and release gates.|SOW-026, SOW-027, SOW-032, SOW-053, SOW-054|DEL-04-05, DEL-04-06, DEL-09-01, DEL-09-02, DEL-09-03, DEL-09-04, DEL-09-05, DEL-10-04, DEL-10-05, DEL-11-04|
|OBJ-009|Enable interoperability and extensibility while preserving governance boundaries.|SOW-030, SOW-031, SOW-038, SOW-049, SOW-054|DEL-02-04, DEL-08-04, DEL-10-01, DEL-10-02, DEL-10-03, DEL-10-04, DEL-10-05, DEL-17-01, DEL-17-02, DEL-17-03, DEL-17-04, DEL-17-05, DEL-17-07, DEL-17-08, DEL-17-09|
|OBJ-010|Protect private project, code, rule-pack, and component data in local-first workflows.|SOW-029, SOW-037, SOW-040|DEL-12-01, DEL-12-02, DEL-12-03, DEL-12-04, DEL-12-05|
|OBJ-011|Preserve professional responsibility: software assists analysis but does not authenticate engineering work.|SOW-034, SOW-047|DEL-01-04, DEL-02-03, DEL-05-04, DEL-06-05, DEL-07-04, DEL-08-03, DEL-09-04, DEL-11-01|
|OBJ-012|Ensure unit-safe, deterministic, and reproducible model data flow across persistence, solving, rule evaluation, automation, and reporting.|SOW-025, SOW-041, SOW-050, SOW-051, SOW-052, SOW-053, SOW-054|DEL-02-01, DEL-02-02, DEL-02-05, DEL-03-08, DEL-04-06, DEL-05-05, DEL-08-02, DEL-10-05|
|OBJ-013|Establish a cohesive software architecture spine for UI, backend services, persistence, jobs, APIs, diagnostics, and software-quality gates before package-level implementation proceeds.|SOW-056, SOW-057, SOW-058, SOW-059, SOW-060, SOW-061, SOW-062, SOW-063|DEL-00-01, DEL-00-02, DEL-00-03, DEL-00-04, DEL-00-05, DEL-00-06, DEL-00-07, DEL-00-08|
|OBJ-014|Provide a schema-backed piping design model that captures physical design context, design knowledge, constraints, assumptions, and analytical transformation traceability.|SOW-064, SOW-065, SOW-066, SOW-067, SOW-068|DEL-02-01, DEL-13-01, DEL-13-02, DEL-13-03, DEL-13-04|
|OBJ-015|Support controlled model authoring where GUI and agent changes become validated, reviewable, auditable model operations.|SOW-069, SOW-070, SOW-076|DEL-07-08, DEL-16-01, DEL-16-02, DEL-16-03, DEL-16-04|
|OBJ-016|Manage immutable model states, analysis runs, and deterministic comparisons as first-class product records for design iteration and review.|SOW-071, SOW-072, SOW-073, SOW-076|DEL-07-08, DEL-08-06, DEL-14-01, DEL-14-02, DEL-14-03, DEL-14-04, DEL-14-05|
|OBJ-017|Produce traceable handoff packages for downstream modeling and professional validation workflows without creating automatic professional approval states.|SOW-074, SOW-075|DEL-08-06, DEL-15-01, DEL-15-02, DEL-15-03, DEL-15-04, DEL-17-01, DEL-17-02, DEL-17-03, DEL-17-04, DEL-17-05, DEL-17-06, DEL-17-07, DEL-17-08, DEL-17-09|
|OBJ-018|Preserve professional and IP boundaries across design knowledge, constraints, operations, comparisons, reports, handoff packages, export workflows, and agent rationale.|SOW-064, SOW-067, SOW-069, SOW-073, SOW-074, SOW-075|DEL-01-04, DEL-08-06, DEL-12-02, DEL-15-04, DEL-16-04, DEL-17-01, DEL-17-04, DEL-17-05, DEL-17-06, DEL-17-07, DEL-17-09|

## 6. Packages


|PackageID|Name|Scope Description|Assigned Scope Items|Exclusions|
|---|---|---|---|---|
|PKG-00|Software Architecture Runway|Defines the implementation spine, architecture decisions, service/API boundaries, persistence/versioning strategy, GUI state model, diagnostics contract, and layered software-quality gates required before package-level implementation proceeds.|SOW-056, SOW-057, SOW-058, SOW-059, SOW-060, SOW-061, SOW-062, SOW-063|Does not implement product features, solver mechanics, GUI screens, or protected standards data.|
|PKG-01|Governance, IP Boundary, and Professional Responsibility|Defines the product’s legal, governance, contribution, and professional-boundary rules.|SOW-001, SOW-003, SOW-028, SOW-034, SOW-048, SOW-064|Does not implement solver or GUI behavior except through requirements and policies.|
|PKG-02|Domain Model, Units, and Core Schemas|Defines the canonical software entities, unit system, persistence/serialization contracts, and extensibility boundaries.|SOW-002, SOW-025, SOW-038, SOW-041, SOW-050, SOW-065|Does not implement numerical solving or GUI views.|
|PKG-03|Piping Components, Materials, and Library Data Model|Defines material/component/section/library models and public/private data governance at the data-object level.|SOW-007, SOW-008, SOW-009, SOW-010, SOW-017, SOW-018, SOW-019, SOW-044, SOW-051|Does not implement the rule evaluator or global solver.|
|PKG-04|Solver Core and Numerical Methods|Implements global 3D centerline/frame mechanics, straight pipe behavior, supports, nonlinear support logic, diagnostics, and performance harnesses.|SOW-005, SOW-006, SOW-011, SOW-012, SOW-035, SOW-053|Does not decide code compliance; produces mechanical results.|
|PKG-05|Loads, Load Cases, and Stress Recovery|Implements primitive loads, concentrated/distributed user loads, load-case algebra, mechanical stress recovery, and analysis-status semantics.|SOW-013, SOW-014, SOW-015, SOW-047, SOW-052|Does not contain proprietary code load combinations or allowables.|
|PKG-06|Rule Packs and User-Supplied Code Check Engine|Implements the schema, sandboxed evaluator, required-input checks, and private lifecycle for rule packs.|SOW-004, SOW-016, SOW-042, SOW-045|Does not ship ASME or other proprietary rule content.|
|PKG-07|Graphical User Interface and Engineering Workflow|Implements the interactive modeler, editors, warning UX, solve-execution UX, and results views.|SOW-020, SOW-021, SOW-022, SOW-023, SOW-036, SOW-055, SOW-076|Does not silently supply missing code data.|
|PKG-08|Reporting, Audit, and Reproducibility|Implements calculation reports, audit manifests, hashes, report-content guardrails, and exports.|SOW-024, SOW-039, SOW-043, SOW-046|Does not authenticate or certify engineering work.|
|PKG-09|Verification, Validation, and Quality Oracles|Implements mechanics benchmarks, regression suites, validation manual structure, and release quality gates.|SOW-026, SOW-027|Does not replace professional review for project-specific reliance.|
|PKG-10|Build, Packaging, API, and Interoperability|Implements public API/plugin boundaries, import/export adapters, headless execution, local FEA handoff contracts, and release packaging.|SOW-030, SOW-031, SOW-032, SOW-049, SOW-054|Does not embed external proprietary tool behavior.|
|PKG-11|Documentation, Examples, and Education|Produces user/developer guides, theory notes, tutorials, and invented-data example models.|SOW-033|Does not publish protected standards examples or commercial software examples.|
|PKG-12|Security, Privacy, and Private Data Handling|Implements local-first storage policies, redaction/export controls, telemetry constraints, and threat modeling.|SOW-029, SOW-037, SOW-040|Does not operate as a cloud service unless separately authorized.|
|PKG-13|Physical Design Knowledge and Constraint Engine|Implements schema-backed design knowledge, constraint provenance, constraint validation, and physical-to-analytical transformation contracts.|SOW-066, SOW-067, SOW-068|Does not bundle owner standards, protected code data, or final engineering acceptance logic.|
|PKG-14|Model States, Analysis Runs, and Comparison|Implements immutable model-state records, analysis-run records, deterministic state/run comparison, mappings, tolerances, and comparison exports.|SOW-071, SOW-072, SOW-073|Does not ingest commercial prover outputs comprehensively or determine external validation.|
|PKG-15|Handoff and External Prover Workflow|Implements canonical handoff package structure, manifests, target mapping metadata, unsupported-target flags, and non-authoritative external-prover metadata boundaries.|SOW-074, SOW-075|Does not force a prover-status lifecycle or generate professional approval records.|
|PKG-16|Model Operation and Agent Proposal Framework|Implements structured model operations, validation/diff preview, user acceptance/audit trail, and agent rationale/professional-boundary controls.|SOW-069, SOW-070|Does not allow hidden model mutations or autonomous engineering acceptance.|
|PKG-17|Export Format Interoperability|Implements deterministic export-format contracts, source-evidence basis, target profiles, stable ID maps, loss reports, external validation harness boundaries, review geometry exports, and adapter SDK surfaces for downstream interoperability.|SOW-030, SOW-046, SOW-074, SOW-075|Does not bundle commercial solvers, embed proprietary examples, reverse engineer protected formats, bypass licenses, or claim code compliance, professional acceptance, or formal solver validation.|

## 7. Deliverables


### PKG-00 — Software Architecture Runway

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-00-01|Architecture decision record baseline|DOC_UPDATE|SOW-056|OBJ-013|M|Decision surface for stack/runtime/framework choices; may contain `TBD` pending human ruling.|
|DEL-00-02|Repository and module boundary architecture|API_CONTRACT|SOW-057,SOW-062|OBJ-013|M|Defines code organization and ownership boundaries before implementation.|
|DEL-00-03|Application service command-query-job model|API_CONTRACT|SOW-058,SOW-062|OBJ-013|L|Cross-cutting application spine for GUI, CLI, solver, storage, reports, and adapters.|
|DEL-00-04|Persistence and schema versioning architecture|DATA_MODEL_CHANGE|SOW-059|OBJ-013|L|Architecture contract for deterministic project files, schema versions, migrations, canonicalization, and the SCA-003 local SQLite storage profile.|
|DEL-00-05|GUI state and interaction architecture|UX_UI_SLICE|SOW-060|OBJ-013|L|Defines GUI state/editing model before independent GUI slices proceed.|
|DEL-00-06|Diagnostics, warning, and result-envelope contract|DATA_MODEL_CHANGE|SOW-061|OBJ-013|M|Unifies user-facing and machine-readable diagnostics across layers.|
|DEL-00-07|API boundary and adapter contract map|API_CONTRACT|SOW-062|OBJ-013|M|Defines internal/public API seams and adapter/plugin governance constraints.|
|DEL-00-08|Layered software test and acceptance strategy|TEST_SUITE|SOW-063|OBJ-013|M|Extends quality gates to software architecture and product workflows.|

### PKG-01 — Governance, IP Boundary, and Professional Responsibility

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-01-01|Project governance baseline|DOC_UPDATE|SOW-001,SOW-048|OBJ-001,OBJ-002|M|Single governance surface; policy language only.|
|DEL-01-02|Copyright and protected-data boundary policy|DOC_UPDATE|SOW-003,SOW-028|OBJ-002|M|Requires careful wording and legal-review flags.|
|DEL-01-03|Contributor certification workflow|DOC_UPDATE|SOW-028,SOW-048|OBJ-002|S|Bounded policy artifact.|
|DEL-01-04|Professional responsibility and product-claims policy|DOC_UPDATE|SOW-034,SOW-064|OBJ-011,OBJ-018|S|Policy wording and product-claims boundaries; no code.|

### PKG-02 — Domain Model, Units, and Core Schemas

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-02-01|Canonical domain model schema|DATA_MODEL_CHANGE|SOW-041,SOW-065|OBJ-001,OBJ-012,OBJ-014|M|Core schema and physical-model source-of-truth contract remain one domain.|
|DEL-02-02|Unit system and dimensional-analysis core contract|BACKEND_FEATURE_SLICE|SOW-025|OBJ-001,OBJ-012|M|Foundation for solver and rule engine.|
|DEL-02-03|Code-neutral analysis boundary model|DATA_MODEL_CHANGE|SOW-002|OBJ-001,OBJ-011|S|Important semantic boundary; limited implementation surface.|
|DEL-02-04|Plugin and extension domain contracts|API_CONTRACT|SOW-038|OBJ-009|M|Cross-cutting but kept to domain/API definitions.|
|DEL-02-05|Project persistence and round-trip serialization|DATA_MODEL_CHANGE|SOW-050,SOW-041|OBJ-001,OBJ-012|M|Single data persistence surface; implement canonical JSON/JCS payloads over the SCA-003 local SQLite storage profile.|

### PKG-03 — Piping Components, Materials, and Library Data Model

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-03-01|Material library schema with provenance|DATA_MODEL_CHANGE|SOW-017|OBJ-004|M|Schema plus tests; no proprietary data.|
|DEL-03-02|Pipe section and component library schema|DATA_MODEL_CHANGE|SOW-018|OBJ-004|M|Library schema only; no protected tables.|
|DEL-03-03|Bend and elbow component model fields|BACKEND_FEATURE_SLICE|SOW-007|OBJ-004|S|Single component family.|
|DEL-03-04|Branch connection component model fields|BACKEND_FEATURE_SLICE|SOW-008|OBJ-004|M|Component model has more local fields but remains bounded.|
|DEL-03-05|Rigid component models for valves, flanges, reducers, and specialty items|BACKEND_FEATURE_SLICE|SOW-009|OBJ-004|M|Multiple related rigid components; no data tables.|
|DEL-03-06|Expansion joint component model|BACKEND_FEATURE_SLICE|SOW-010|OBJ-004|M|Specialized but bounded component type.|
|DEL-03-07|Public/private library import provenance checker|BACKEND_FEATURE_SLICE|SOW-019,SOW-044|OBJ-002,OBJ-004|M|Connects schema with governance; no external import formats yet.|
|DEL-03-08|Pipe section property and mass-property calculator|BACKEND_FEATURE_SLICE|SOW-051,SOW-018|OBJ-004,OBJ-012|M|Bounded calculation module using user-entered dimensions only.|

### PKG-04 — Solver Core and Numerical Methods

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-04-01|3D frame stiffness kernel|BACKEND_FEATURE_SLICE|SOW-005,SOW-035|OBJ-003|L|Central solver kernel; large but single domain and foundational.|
|DEL-04-02|Straight pipe element|BACKEND_FEATURE_SLICE|SOW-006|OBJ-003|M|Single element type.|
|DEL-04-03|Linear support and restraint models|BACKEND_FEATURE_SLICE|SOW-011|OBJ-003|M|Support families are related and can share validation.|
|DEL-04-04|Nonlinear support active-set solver|BACKEND_FEATURE_SLICE|SOW-012|OBJ-003|L|Complex; remains one numerical domain.|
|DEL-04-05|Sparse solver performance harness|TEST_SUITE|SOW-035|OBJ-003,OBJ-008|M|Performance harness separate from solver logic.|
|DEL-04-06|Solver diagnostics and singularity detection|BACKEND_FEATURE_SLICE|SOW-053,SOW-035|OBJ-003,OBJ-008,OBJ-012|M|Focused diagnostic layer separate from the stiffness kernel.|

### PKG-05 — Loads, Load Cases, and Stress Recovery

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-05-01|Primitive load case engine|BACKEND_FEATURE_SLICE|SOW-013|OBJ-003|M|Primitive categories; code combinations remain external.|
|DEL-05-02|Load-case algebra engine|BACKEND_FEATURE_SLICE|SOW-014|OBJ-003,OBJ-005|M|Bounded algebra surface.|
|DEL-05-03|Fundamental stress recovery module|BACKEND_FEATURE_SLICE|SOW-015|OBJ-003|M|Does not encode code stress equations.|
|DEL-05-04|Analysis status semantics|DATA_MODEL_CHANGE|SOW-047|OBJ-005,OBJ-011|S|Small state model with high governance value.|
|DEL-05-05|Concentrated and distributed user load application|BACKEND_FEATURE_SLICE|SOW-052,SOW-013|OBJ-003,OBJ-012|M|Keeps general user loads separate from code-specific combinations.|

### PKG-06 — Rule Packs and User-Supplied Code Check Engine

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-06-01|Rule-pack schema|DATA_MODEL_CHANGE|SOW-016,SOW-042|OBJ-005|M|Schema and examples only.|
|DEL-06-02|Sandboxed unit-aware expression evaluator|BACKEND_FEATURE_SLICE|SOW-045|OBJ-005|L|Security-sensitive and numerically important.|
|DEL-06-03|Required-input completeness checker|BACKEND_FEATURE_SLICE|SOW-004|OBJ-002,OBJ-005|M|Connects user data to rule-pack gating.|
|DEL-06-04|Private rule-pack lifecycle and checksum handling|BACKEND_FEATURE_SLICE|SOW-042|OBJ-002,OBJ-005|M|Local lifecycle only; access control details defer to PKG-12.|
|DEL-06-05|Invented non-code example rule pack|DOC_UPDATE|SOW-016|OBJ-005,OBJ-011|S|Example only; no proprietary content.|

### PKG-07 — Graphical User Interface and Engineering Workflow

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-07-01|3D viewport and centerline editor|UX_UI_SLICE|SOW-020|OBJ-006|L|GUI surface is broad but bounded to viewport/editor.|
|DEL-07-02|Model tree and property inspector|UX_UI_SLICE|SOW-020,SOW-021|OBJ-006|M|Single UI work surface.|
|DEL-07-03|Material, component, and rule-pack editors|UX_UI_SLICE|SOW-021|OBJ-006|L|Multiple editors but same GUI domain; may split later.|
|DEL-07-04|Missing-data warning and blocking UX|UX_UI_SLICE|SOW-022|OBJ-006,OBJ-011|M|High-value UX slice.|
|DEL-07-05|Results viewer|UX_UI_SLICE|SOW-023|OBJ-006,OBJ-007|L|Results surface is substantial but one domain.|
|DEL-07-06|Accessibility and usability baseline|UX_UI_SLICE|SOW-036|OBJ-006|M|May refine once GUI framework chosen.|
|DEL-07-07|Solve execution UX: progress, cancellation, and diagnostics|UX_UI_SLICE|SOW-055|OBJ-006,OBJ-007|M|Single GUI workflow slice for running and inspecting solves.|
|DEL-07-08|Design-authoring state and comparison workspace|UX_UI_SLICE|SOW-076|OBJ-015,OBJ-016|L|Adds design knowledge panels, operation/diff review, state/run browsers, comparison tables, and graphical overlays to the GUI workflow.|

### PKG-08 — Reporting, Audit, and Reproducibility

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-08-01|Calculation report generator|BACKEND_FEATURE_SLICE|SOW-024|OBJ-007|L|Report is broad but single artifact family.|
|DEL-08-02|Audit manifest and model hash|BACKEND_FEATURE_SLICE|SOW-039|OBJ-007,OBJ-012|M|Canonicalization TBD but bounded.|
|DEL-08-03|Warnings, assumptions, and provenance report section|BACKEND_FEATURE_SLICE|SOW-024|OBJ-007,OBJ-011|M|Leverages warning data from GUI/core.|
|DEL-08-04|Result export format|API_CONTRACT|SOW-046|OBJ-007,OBJ-009|M|Format TBD but bounded.|
|DEL-08-05|Report protected-content linter|TEST_SUITE|SOW-043|OBJ-002,OBJ-007|M|Heuristic plus review; cannot be sole legal control.|
|DEL-08-06|State, comparison, and handoff report sections|BACKEND_FEATURE_SLICE|SOW-024|OBJ-007,OBJ-016,OBJ-017,OBJ-018|M|Reports draw from model states, analysis runs, comparisons, and handoff manifests without implying professional validation.|

### PKG-09 — Verification, Validation, and Quality Oracles

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-09-01|Mechanics benchmark suite|TEST_SUITE|SOW-026|OBJ-008|M|Uses original/public mechanics cases.|
|DEL-09-02|Stress recovery benchmark suite|TEST_SUITE|SOW-026|OBJ-008|M|No code formulas.|
|DEL-09-03|Nonlinear support regression suite|TEST_SUITE|SOW-026|OBJ-008|M|Depends on nonlinear solver maturity.|
|DEL-09-04|Validation manual skeleton|DOC_UPDATE|SOW-027|OBJ-008,OBJ-011|M|Documentation-oriented.|
|DEL-09-05|Release quality gate checklist|CI_CD_CHANGE|SOW-026,SOW-027|OBJ-008|M|Process + CI; bounded.|

### PKG-10 — Build, Packaging, API, and Interoperability

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-10-01|Public API and plugin boundary|API_CONTRACT|SOW-030|OBJ-009|M|API details TBD.|
|DEL-10-02|Import/export adapter framework|BACKEND_FEATURE_SLICE|SOW-030|OBJ-009|L|Actual external formats can be later deliverables.|
|DEL-10-03|Local FEA handoff data contract|API_CONTRACT|SOW-031,SOW-049|OBJ-009|M|Guidance only, no external FEA implementation.|
|DEL-10-04|Build, packaging, and CI/CD pipeline|CI_CD_CHANGE|SOW-032|OBJ-008,OBJ-009|L|Platform list TBD.|
|DEL-10-05|Headless CLI and structured I/O analysis runner|BACKEND_FEATURE_SLICE|SOW-054,SOW-032|OBJ-008,OBJ-009,OBJ-012|M|Automation surface aligned to R0/R1 release milestones.|

### PKG-11 — Documentation, Examples, and Education

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-11-01|User guide skeleton|DOC_UPDATE|SOW-033|OBJ-001,OBJ-011|M|Documentation only.|
|DEL-11-02|Developer guide for solver and rule packs|DOC_UPDATE|SOW-033|OBJ-001,OBJ-002|M|Documentation only.|
|DEL-11-03|Theory notes: classical to modern centerline analysis|DOC_UPDATE|SOW-033|OBJ-001,OBJ-003|M|Should cite public/permissive sources; no formula copying from standards.|
|DEL-11-04|Invented educational example models|DOC_UPDATE|SOW-033|OBJ-001,OBJ-008|M|Examples must be clearly non-code.|
|DEL-11-05|Contributor tutorial and onboarding|DOC_UPDATE|SOW-033|OBJ-001,OBJ-002|S|Documentation only.|

### PKG-12 — Security, Privacy, and Private Data Handling

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-12-01|Local-first storage and private data paths|SECURITY_CONTROL|SOW-029|OBJ-010|M|Local-first policy must align with the SCA-003 local SQLite storage profile and no-network/no-cloud MVP posture.|
|DEL-12-02|Private data redaction and export controls|SECURITY_CONTROL|SOW-040|OBJ-010|M|Security-sensitive but bounded.|
|DEL-12-03|Telemetry off-by-default design|SECURITY_CONTROL|SOW-037|OBJ-010|S|May be no-op in MVP.|
|DEL-12-04|Secret and private-library handling|SECURITY_CONTROL|SOW-040,SOW-029|OBJ-010|M|Avoids cloud assumptions.|
|DEL-12-05|Security threat model|DOC_UPDATE|SOW-040|OBJ-010|M|Document now; update as architecture matures.|

### PKG-13 — Physical Design Knowledge and Constraint Engine

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-13-01|Design knowledge schema and provenance model|DATA_MODEL_CHANGE|SOW-067|OBJ-014|M|Defines endpoints, line data, corridors, zones, equipment interfaces, access/slope requirements, source notes, and unresolved assumptions without bundling protected data.|
|DEL-13-02|Constraint entity and provenance model|DATA_MODEL_CHANGE|SOW-068,SOW-067|OBJ-014,OBJ-018|M|Represents connectivity, clearance, no-go, support-zone, slope/drain/vent, and missing-data constraints with user/project/agent/source provenance.|
|DEL-13-03|Constraint validation engine|BACKEND_FEATURE_SLICE|SOW-068|OBJ-014|M|Produces deterministic validation messages for available constraint data; does not infer hidden owner standards or code requirements.|
|DEL-13-04|Physical-to-analytical transformation contract|BACKEND_FEATURE_SLICE|SOW-066|OBJ-014|L|Derives solver-ready analytical models from physical models and records transformation warnings, omissions, and traceability.|

### PKG-14 — Model States, Analysis Runs, and Comparison

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-14-01|Immutable model state records|DATA_MODEL_CHANGE|SOW-071|OBJ-016|M|Defines state naming, tags, notes, external references, assumptions, warnings, hashes, and read-only snapshot semantics.|
|DEL-14-02|Analysis run records|DATA_MODEL_CHANGE|SOW-072|OBJ-016|M|Binds results to exact model state, solver version, settings, units, load cases, diagnostics, rule/library refs, and result hashes.|
|DEL-14-03|Model-state comparison engine|BACKEND_FEATURE_SLICE|SOW-073,SOW-071|OBJ-016|L|Computes deterministic added/removed/changed/unchanged entity diffs using stable IDs and explicit mapping records.|
|DEL-14-04|Analysis-run comparison engine|BACKEND_FEATURE_SLICE|SOW-073,SOW-072|OBJ-016|L|Computes unit-normalized result deltas for mapped nodes, elements, supports, terminals, stress/result locations, diagnostics, and settings.|
|DEL-14-05|Comparison mapping, tolerance, and export contracts|API_CONTRACT|SOW-073|OBJ-016|M|Defines manual mappings, unmatched classifications, tolerance profiles, and CSV/JSON/report-section export semantics.|

### PKG-15 — Handoff and External Prover Workflow

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-15-01|Canonical handoff package schema and manifest|API_CONTRACT|SOW-074|OBJ-017|M|Exports schema-compliant model data with hashes, units, entity IDs, library/rule refs, warnings, unresolved assumptions, and provenance.|
|DEL-15-02|Target mapping and unsupported-behavior contract|API_CONTRACT|SOW-074|OBJ-017|M|Records how internal entities map to target fields and flags unsupported or approximate target behavior.|
|DEL-15-03|Downstream modeling export workflow|BACKEND_FEATURE_SLICE|SOW-074|OBJ-017|L|Implements the generic handoff export path before target-specific commercial-tool parsers are in scope.|
|DEL-15-04|External prover boundary metadata|DATA_MODEL_CHANGE|SOW-075|OBJ-017,OBJ-018|M|Allows flexible names, tags, notes, external references, and attachments without hard-coded approval/certification statuses.|

### PKG-16 — Model Operation and Agent Proposal Framework

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-16-01|Structured model operation schema|DATA_MODEL_CHANGE|SOW-069|OBJ-015|M|Defines add/move/modify/delete/reconnect operations for model entities, constraints, loads, supports, and design knowledge.|
|DEL-16-02|Operation validation and diff preview|BACKEND_FEATURE_SLICE|SOW-069|OBJ-015|M|Runs schema and constraint validation, creates deterministic previews, and blocks invalid operations before application.|
|DEL-16-03|User acceptance and operation audit trail|BACKEND_FEATURE_SLICE|SOW-069,SOW-070|OBJ-015|M|Records accepted/rejected operations, affected entities, timestamps, actor/source metadata, and assumptions.|
|DEL-16-04|Agent rationale and professional-boundary controls|SECURITY_CONTROL|SOW-070|OBJ-015,OBJ-018|S|Captures agent rationale and unresolved assumptions while preventing certification, approval, or code-compliance claims.|

### PKG-17 — Export Format Interoperability

|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|
|---|---|---|---|---|---|---|
|DEL-17-01|CAEPIPE and export-format source basis|DOC_UPDATE|SOW-030,SOW-074,SOW-075|OBJ-009,OBJ-017,OBJ-018|M|Source-gathering and claim-boundary deliverable; unanswered vendor questions remain TBD and gate target-specific claims.|
|DEL-17-02|Export package, profile, and stable ID map contracts|API_CONTRACT|SOW-030,SOW-074|OBJ-009,OBJ-017|L|Foundation contract for all target exporters; confirm scope and split if concrete implementation expands.|
|DEL-17-03|Native open JSON export package|BACKEND_FEATURE_SLICE|SOW-030,SOW-074|OBJ-009,OBJ-017|L|Native export package is a first-wave implementation target and canonical adapter input.|
|DEL-17-04|CAEPIPE MBF export profile and deterministic writer|BACKEND_FEATURE_SLICE|SOW-030,SOW-074,SOW-075|OBJ-009,OBJ-017,OBJ-018|L|First solver-specific target; code/pass-through options must not become local code-checking logic.|
|DEL-17-05|CAEPIPE external run harness and CSV parser|BACKEND_FEATURE_SLICE|SOW-030,SOW-046,SOW-075|OBJ-007,OBJ-009,OBJ-017,OBJ-018|L|Harness remains optional and user-owned; no solver binary, license bypass, or professional acceptance claim.|
|DEL-17-06|Stress-neutral CSV/JSON package|BACKEND_FEATURE_SLICE|SOW-046,SOW-074|OBJ-007,OBJ-017,OBJ-018|M|Result/export assurance target sequenced after export package contracts and run/result records.|
|DEL-17-07|Conservative PCF subset exporter|BACKEND_FEATURE_SLICE|SOW-030,SOW-074|OBJ-009,OBJ-017,OBJ-018|L|PCF is useful for interoperability but not the first validation backbone; translator behavior remains explicitly bounded.|
|DEL-17-08|GLB/glTF review geometry export|BACKEND_FEATURE_SLICE|SOW-030,SOW-074|OBJ-009,OBJ-017|M|Review geometry is visual handoff support only, not solver geometry or professional validation.|
|DEL-17-09|Export adapter SDK and additional targets|API_CONTRACT|SOW-030,SOW-074,SOW-075|OBJ-009,OBJ-017,OBJ-018|L|Adapter extensibility target sequenced after common profile and package contracts.|

## 8. Architecture basis and brief-injection rules

`SCA-001` authorizes `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` as an architecture-basis candidate for downstream sealed brief injection. This does not change the lifecycle state of `PKG-00` to `ISSUED`, and it does not make the `PKG-00` production artifacts directly authoritative over downstream artifacts outside a sealed brief.

### 8.1 Architecture basis register

|BasisID|Source Deliverable|Dispatchable Constraint|Primary Downstream Reach|
|---|---|---|---|
|AB-00-01|DEL-00-01|Use an ADR index/template; record accepted decisions, affected packages, reconsideration triggers, and code-neutral/protected-data-free rationale. Stack/runtime/framework/library/package choices remain TBD only where explicitly deferred.|All downstream sealed briefs.|
|AB-00-02|DEL-00-02|Preserve layer/module responsibilities for GUI, application services, domain core, solver, rules, reports, adapters, storage, schemas, validation, and tests. Dependencies must point inward toward domain contracts; adapters/plugins cannot bypass governance, validation, or diagnostics.|All implementation and architecture-facing deliverables.|
|AB-00-03|DEL-00-03|Separate commands, queries, background jobs, transactions, cancellation/progress, reproducibility metadata, and result envelopes. Preserve distinctions among mechanics solved, user-rule checked, and human-approved states.|PKG-02, PKG-04, PKG-05, PKG-06, PKG-07, PKG-08, PKG-10, PKG-12, plus validation/docs where these contracts are referenced.|
|AB-00-04|DEL-00-04|Project persistence must be deterministic, versioned, unit-aware, provenance-preserving, schema-governed, migration-aware, and round-trip testable. JSON payload hashes use canonical JSON with JCS-compatible canonicalization. The MVP physical storage profile is a local SQLite-backed project store/index with rebuildable SQLite FTS5/BM25 retrieval sidecars; optional NumPy sidecars are non-authoritative caches only if later justified. Large external files are referenced in place by path/URI plus hash and metadata by default. Direct plugin/adapter SQL access is prohibited; all mutation routes through application services.|PKG-02, PKG-03, PKG-06, PKG-08, PKG-09, PKG-10, PKG-12, PKG-14, PKG-15, PKG-16.|
|AB-00-05|DEL-00-05|Separate durable project state from transient session, viewport, selection, and job-progress state. GUI mutations route through application-service commands; undo/redo is scoped to reversible model edits and preserves diagnostics.|PKG-07 and GUI-adjacent services/tests.|
|AB-00-06|DEL-00-06|Diagnostics and result envelopes must carry code, class, severity, source, affected object, message, remediation, and provenance. Warning classes include `SOLVE_BLOCKING`, `RULE_CHECK_BLOCKING`, `PROVENANCE_WARNING`, `ASSUMPTION_WARNING`, `NONLINEAR_WARNING`, and `IP_BOUNDARY_WARNING`; outputs must not claim certification or compliance.|PKG-04, PKG-05, PKG-06, PKG-07, PKG-08, PKG-09, PKG-10, PKG-12.|
|AB-00-07|DEL-00-07|Maintain internal/public API boundaries. Adapters validate units, provenance, redistribution, diagnostics, and public/private data boundaries. Plugins/adapters cannot bypass validation, sandboxing, envelopes, or report controls. External transport protocol and concrete import/export formats remain TBD.|PKG-02, PKG-03, PKG-06, PKG-08, PKG-10, PKG-11, PKG-12.|
|AB-00-08|DEL-00-08|Use layered tests for architecture, schemas, units, services, solver, load/stress behavior, rule packs, GUI, CLI, reports, adapters, packaging, security, and regressions. Solver/rule deterministic verification is required before release use; protected-content/provenance gates are required where relevant.|All downstream sealed briefs.|

### 8.2 Resolved architecture baseline

|Area|Resolved by SCA-001|Still Implementation-Level TBD|
|---|---|---|
|Runtime and GUI|Rust core/application services; Tauri 2 desktop shell; TypeScript/React/Vite GUI; Three.js viewport for 3D interaction.|Exact dependency versions, component/state-management libraries, platform release matrix.|
|Schema and API contracts|JSON Schema 2020-12 for public schemas/interchange; schema-first command/query/job/result envelopes.|Public API transport protocol, code-generation tooling, exact schema file layout.|
|Persistence and reproducibility|Versioned JSON-schema-governed persistence; canonical JSON with JCS-compatible hashing for JSON payloads; manifest hashes for non-JSON/binary assets; MVP physical storage profile is local SQLite project store/index with rebuildable SQLite FTS5/BM25 retrieval sidecars, optional non-authoritative NumPy caches only if later justified, and path/URI-plus-hash references for large files by default.|Migration framework/tooling, DB migration implementation details, binary asset/export packaging, portable project export behavior, and optional retrieval-cache implementation details.|
|Testing and acceptance|Cargo tests, Vitest, Playwright, validation gates, protected-content/provenance gates, deterministic solver/rule verification gates.|CI provider, coverage thresholds, performance thresholds, exact test harness libraries beyond the baseline.|
|Solver and rule internals|Layer boundaries and deterministic diagnostics/result envelopes are binding.|Sparse/numerical solver library, rule-pack expression grammar/library, nonlinear algorithm tolerances.|

### 8.3 Brief-injection rules

Every future TASK brief for `PKG-01` through `PKG-12` must include the applicable `AB-00-*` rows, the resolved architecture baseline, and the remaining-TBD boundary. Briefs should inject applicable constraints, not full `PKG-00` prose.

If a downstream deliverable needs to depart from an applicable architecture basis item, execution must stop and escalate to `SCOPE_CHANGE` or the human project authority. `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` files are evidence aids, not independent engineering authority.

## 9. Scope ledger summary

The authoritative companion register is `_Registers/ScopeLedger.csv`.


|ScopeItemID|Status|Statement|SourceRef|PackageID|DeliverableID(s)|ObjectiveID(s)|OpenIssue|Notes|
|---|---|---|---|---|---|---|---|---|
|SOW-001|IN|The project shall produce a source-available noncommercial piping stress analysis platform.|INTENT.md §Core intent; PRD §1, §4|PKG-01|DEL-01-01|OBJ-001|FALSE|License selected by human project authority on 2026-06-03: PolyForm-Noncommercial-1.0.0.|
|SOW-002|IN|The software shall remain code-neutral: the solver computes mechanics and user-supplied rule packs evaluate acceptability.|INTENT.md §Product identity; PRD §6.1|PKG-02|DEL-02-03|OBJ-001|FALSE|Do not encode a proprietary code as the default public product.|
|SOW-003|IN|The public repository shall not redistribute protected standards text, tables, figures, examples, material allowables, SIF tables, flexibility-factor tables, or protected dimensional tables.|INTENT.md §Copyright-respecting design principle; PRD §5, §17|PKG-01|DEL-01-02|OBJ-002|FALSE|Legal review required before accepting any public data contribution.|
|SOW-004|IN|The system shall require users to supply code-specific and project-specific data needed for code checks.|INTENT.md §User-supplied code and data layer; PRD §6.1, §12|PKG-06|DEL-06-03|OBJ-002, OBJ-005|FALSE|Rule-pack completeness must be machine-checkable.|
|SOW-005|IN|The analytical core shall model piping as a 3D centerline/frame system with six degrees of freedom per node.|INTENT.md §Analytical engine intent; PRD §11.1-§11.2|PKG-04|DEL-04-01|OBJ-003|FALSE|Primary global model is line-element, not routine solid meshing.|
|SOW-006|IN|The solver shall implement straight pipe elements with open mechanics section and stiffness calculations.|PRD §11.3.1|PKG-04|DEL-04-02|OBJ-003|FALSE|Pipe dimensions come from user data or lawful libraries.|
|SOW-007|IN|The component model shall support bends and elbows with user-entered SIFs, flexibility factors, and bend geometry.|INTENT.md §Analytical engine intent; PRD §11.3.2|PKG-03|DEL-03-03|OBJ-004|FALSE|Do not bundle B31J-derived tables/equations.|
|SOW-008|IN|The component model shall support branch connections with user-entered reinforcement, SIF, flexibility, and local data fields.|PRD §11.3.3|PKG-03|DEL-03-04|OBJ-004|FALSE|Branch local checks may require future specialized modules.|
|SOW-009|IN|The component model shall support rigid and semi-rigid valves, flanges, reducers, and specialty items using user-supplied dimensions, weights, and centers of gravity.|PRD §11.3.4-§11.3.5|PKG-03|DEL-03-05|OBJ-004|FALSE|Public component data must be licensed.|
|SOW-010|IN|The component model shall support expansion joints using manufacturer/user-supplied stiffness, effective area, movement limits, and hardware data.|PRD §11.3.6|PKG-03|DEL-03-06|OBJ-004|FALSE|No invented expansion joint defaults.|
|SOW-011|IN|The solver shall support anchors, guides, line stops, vertical supports, springs, and imposed displacements.|PRD §11.4|PKG-04|DEL-04-03|OBJ-003|FALSE|Initial release may limit to linear supports.|
|SOW-012|IN|The solver shall support nonlinear support behavior such as one-way restraints, lift-off, gaps, and friction through a controlled iterative method.|PRD §11.4, §22.4|PKG-04|DEL-04-04|OBJ-003|FALSE|MVP may defer some nonlinear cases.|
|SOW-013|IN|The analysis engine shall support primitive load cases for weight, pressure, thermal expansion, imposed displacement, hydrotest, wind, seismic, and occasional loads.|PRD §11.5|PKG-05|DEL-05-01, DEL-05-05|OBJ-003|FALSE|Dynamic modules may be deferred.|
|SOW-014|IN|The analysis engine shall support unit-aware load-case algebra and user-defined combinations.|PRD §11.6|PKG-05|DEL-05-02|OBJ-003, OBJ-005|FALSE|Code-specific combinations supplied by user rule packs.|
|SOW-015|IN|The stress module shall recover fundamental mechanics stresses from forces, moments, pressure, and section properties.|PRD §11.7|PKG-05|DEL-05-03|OBJ-003|FALSE|Code stress categories are rule-pack mappings.|
|SOW-016|IN|The product shall provide a private rule-pack schema for user-defined stress checks, allowables, formulas, and pass/fail criteria.|INTENT.md §Rule-pack intent; PRD §12|PKG-06|DEL-06-01, DEL-06-05|OBJ-002, OBJ-005|FALSE|Public examples must use invented values.|
|SOW-017|IN|The product shall support private material libraries with temperature-dependent properties, allowables, and provenance fields.|PRD §13.1|PKG-03|DEL-03-01|OBJ-004|FALSE|No public ASME material allowable tables.|
|SOW-018|IN|The product shall support private pipe section and component libraries with provenance and redistribution status.|PRD §13.2-§13.4|PKG-03|DEL-03-02, DEL-03-08|OBJ-004|FALSE|Public data requires documented rights.|
|SOW-019|IN|The public project shall accept component data only when provenance and redistribution rights are documented.|PRD §13.4, §17.2|PKG-03|DEL-03-07|OBJ-004|FALSE|Contributor certification required.|
|SOW-020|IN|The GUI shall provide a 3D centerline modeler with model tree and piping component visualization.|PRD §14.1-§14.2|PKG-07|DEL-07-01, DEL-07-02|OBJ-006|FALSE|GUI baseline is Tauri 2 desktop shell with TypeScript/React/Vite and Three.js viewport; exact component/state libraries remain implementation TBD.|
|SOW-021|IN|The GUI shall provide editors for materials, sections, components, load cases, supports, rule packs, and private libraries.|PRD §14.3, §14.5-§14.6|PKG-07|DEL-07-02, DEL-07-03|OBJ-006|FALSE|Workflow should surface missing data early.|
|SOW-022|IN|The GUI shall distinguish data required for solving from data required for code checking and shall block/qualify results when required data is missing.|PRD §14.4|PKG-07|DEL-07-04|OBJ-006|FALSE|No silent engineering defaults.|
|SOW-023|IN|The GUI shall provide results review for displacements, rotations, forces, moments, restraint reactions, equipment loads, stresses, and ratios.|PRD §14.2, §15.1|PKG-07|DEL-07-05|OBJ-006|FALSE|Stress ratios depend on rule pack completeness.|
|SOW-024|IN|The product shall generate auditable calculation reports including inputs, sources, warnings, assumptions, results, rule-pack checksums, and limitations.|PRD §15|PKG-08|DEL-08-01, DEL-08-03, DEL-08-06|OBJ-001, OBJ-007|FALSE|Reports must not reproduce protected standards content.|
|SOW-025|IN|All calculations, schemas, imports, exports, and rule evaluations shall be unit-aware.|PRD §6.6|PKG-02|DEL-02-02|OBJ-012|FALSE|Unit conversions must be deterministic and testable.|
|SOW-026|IN|The project shall maintain verification benchmarks, regression tests, and numerical quality checks for solver and stress recovery behavior.|PRD §16.2-§16.4|PKG-09|DEL-09-01, DEL-09-02, DEL-09-03, DEL-09-05|OBJ-008|FALSE|Benchmark sources must be public/original/permissive.|
|SOW-027|IN|The project shall maintain a validation manual that distinguishes mechanics verification from code compliance and professional signoff.|PRD §16.5|PKG-09|DEL-09-04, DEL-09-05|OBJ-008|FALSE|Human engineering judgment remains required.|
|SOW-028|IN|The repository shall include contributor governance, IP controls, and review procedures preventing copyrighted standards data from entering the public project.|PRD §17.1-§17.2|PKG-01|DEL-01-02, DEL-01-03|OBJ-002|FALSE|Use stop-and-escalate for suspected protected content.|
|SOW-029|IN|The product shall be local-first and shall protect private rule packs, private material data, private component data, and project models.|PRD §18.1, §18.3|PKG-12|DEL-12-01, DEL-12-04|OBJ-010|FALSE|Cloud services are out of MVP unless approved.|
|SOW-030|IN|The architecture shall support public APIs, plugins, and import/export adapters for integration with external tools.|PRD §19.3|PKG-10, PKG-17|DEL-10-01, DEL-10-02, DEL-17-01, DEL-17-02, DEL-17-03, DEL-17-04, DEL-17-05, DEL-17-07, DEL-17-08, DEL-17-09|OBJ-009, OBJ-018|FALSE|API baseline is schema-first command/query/job result envelopes; SCA-004 adds deterministic export profiles, native JSON packages, CAEPIPE MBF, conservative PCF, GLB/glTF review geometry, and adapter SDK work under no-bypass plugin/adapter boundaries.|
|SOW-031|IN|The product shall support optional handoff of selected local-detail problems to external shell/solid FEA workflows.|INTENT.md §Analytical engine intent; PRD §4.2, §19|PKG-10|DEL-10-03|OBJ-009|FALSE|Local FEA is not the normal global analysis method.|
|SOW-032|IN|The project shall provide reproducible build, packaging, and CI/CD workflows for supported platforms.|PRD §22|PKG-10|DEL-10-04, DEL-10-05|OBJ-008|FALSE|Desktop packaging baseline follows Tauri-supported macOS/Windows/Linux targets; CI host and release matrix details remain TBD.|
|SOW-033|IN|The project shall provide user/developer documentation and invented-data examples for education and testing.|PRD §7.5, §16.5, §22|PKG-11|DEL-11-01, DEL-11-02, DEL-11-03, DEL-11-04, DEL-11-05|OBJ-001|FALSE|No protected examples.|
|SOW-034|IN|The product shall preserve professional responsibility boundaries: agents/software do not certify, approve, seal, or declare engineering code compliance for reliance.|PROFESSIONAL_ENGINEERING.md; CHIRALITY_FRAMEWORK.md; PRD §5, §17.4|PKG-01|DEL-01-04|OBJ-011|FALSE|Reports may show user-rule pass/fail, not human code compliance.|
|SOW-035|IN|The solver shall be designed for sparse numerical performance and reproducible results on practical piping models.|PRD §11.8, §20|PKG-04|DEL-04-01, DEL-04-05, DEL-04-06|OBJ-003|FALSE|Performance targets TBD.|
|SOW-036|IN|The GUI and reports shall support baseline accessibility and usability appropriate for engineering review.|PRD §21|PKG-07|DEL-07-06|OBJ-006|FALSE|Detailed WCAG target TBD.|
|SOW-037|IN|Telemetry, if implemented, shall be opt-in, privacy-preserving, and shall not transmit private project/code data.|PRD §18.2|PKG-12|DEL-12-03|OBJ-010|FALSE|MVP default is no telemetry.|
|SOW-038|IN|The software architecture shall be extensible without allowing plugins/adapters to bypass governance, unit safety, or data-boundary constraints.|PRD §19.1-§19.3|PKG-02|DEL-02-04|OBJ-009|FALSE|Plugin/adapter baseline is deny-bypass: units, provenance, diagnostics, rule sandboxing, and report controls remain mandatory; detailed permissions TBD.|
|SOW-039|IN|The product shall produce reproducibility artifacts such as model hashes, solver version, rule-pack checksum, and input manifest.|PRD §15.3|PKG-08|DEL-08-02|OBJ-007|FALSE|Canonical JSON with JCS-compatible hashing is the JSON payload hash basis; non-JSON/binary assets require manifest hashes.|
|SOW-040|IN|The product shall implement private data handling controls, including redaction/export safeguards where reports or shared models may expose protected/private values.|PRD §18.3|PKG-12|DEL-12-02, DEL-12-04, DEL-12-05|OBJ-010|FALSE|Detailed threat model TBD.|
|SOW-041|IN|The project shall define machine-readable project, model, material, component, load, result, and report schemas.|PRD §19.1, Appendix A|PKG-02|DEL-02-01, DEL-02-05|OBJ-001, OBJ-012|FALSE|JSON Schema 2020-12 is the public schema/interchange baseline; code-generation tooling remains TBD.|
|SOW-042|IN|Rule packs shall be versioned, source-noted, checksum-addressed, and explicitly marked for redistribution status.|INTENT.md §Rule-pack intent; PRD §12.4|PKG-06|DEL-06-01, DEL-06-04|OBJ-002, OBJ-005|FALSE|Private rule packs should not be committed publicly.|
|SOW-043|IN|The report system shall prohibit automatic inclusion of protected code text, copied standards tables, or proprietary formulas in generated public reports/templates.|PRD §15.2|PKG-08|DEL-08-05|OBJ-007|FALSE|User-private report templates remain user responsibility.|
|SOW-044|IN|The product shall provide import mechanisms that record source/provenance/license metadata for component and material data.|PRD §13.5|PKG-03|DEL-03-07|OBJ-004|FALSE|Importer may reject/flag missing provenance.|
|SOW-045|IN|The rule-pack evaluator shall be sandboxed and unit-aware, not arbitrary executable code.|PRD §12.3|PKG-06|DEL-06-02|OBJ-005|FALSE|Expression language TBD.|
|SOW-046|IN|The product shall support result exports suitable for review, regression comparison, and downstream tooling.|PRD §15.1, §19.3|PKG-08, PKG-17|DEL-08-04, DEL-17-05, DEL-17-06|OBJ-007, OBJ-017|FALSE|Structured result export baseline is schema-first JSON result envelopes for review/regression; SCA-004 adds stress-neutral CSV/JSON packages and external-harness result parsing for regression and downstream tooling, without code-compliance or professional-acceptance claims.|
|SOW-047|IN|The product shall clearly distinguish mechanics solved, rule-pack checked, and professionally code-compliant/human-approved states.|INTENT.md §GUI intent; PRD §6.2, §17.4|PKG-05|DEL-05-04|OBJ-005, OBJ-011|FALSE|Human approval is outside software authority.|
|SOW-048|IN|The project shall define project license, governance, release, and maintainer policies.|PRD §17.2, §22|PKG-01|DEL-01-01, DEL-01-03|OBJ-002|FALSE|License selected by human project authority on 2026-06-03: PolyForm-Noncommercial-1.0.0.|
|SOW-049|IN|The project shall define criteria for when global beam analysis is sufficient and when local shell/solid analysis handoff is needed.|INTENT.md §Analytical engine intent; PRD §4.2|PKG-10|DEL-10-03|OBJ-009|FALSE|Criteria are guidance, not automatic certification.|
|SOW-050|IN|The product shall support create/open/save/versioned project persistence with deterministic round-trip serialization of models, units, loads, rule-pack references, and provenance metadata.|PRD §10 FR-001; PRD §27|PKG-02|DEL-02-05|OBJ-001, OBJ-012|FALSE|Persistence baseline is versioned JSON-schema-governed canonical JSON with migration status and JCS-compatible hashes; MVP physical storage profile is local SQLite-backed project store/index with rebuildable sidecars.|
|SOW-051|IN|The product shall calculate pipe section and mass properties from user-entered dimensions and material data with unit checks.|PRD §10 FR-005; PRD §13.2|PKG-03|DEL-03-08|OBJ-004, OBJ-012|FALSE|No protected pipe dimensional tables are introduced.|
|SOW-052|IN|The load engine shall support concentrated forces, concentrated moments, and distributed user loads in addition to core primitive piping load categories.|PRD §10 FR-007; PRD §11.5|PKG-05|DEL-05-05|OBJ-003, OBJ-012|FALSE|Code-specific load combinations remain user/rule-pack supplied.|
|SOW-053|IN|The solver shall detect and report singular, ill-conditioned, and nonconverged analysis states with deterministic diagnostics.|PRD §11.8; PRD §20; PRD §22.2|PKG-04|DEL-04-06|OBJ-003, OBJ-008, OBJ-012|FALSE|Supports the R1 exit criterion that the solver detects singular models.|
|SOW-054|IN|The project shall provide a headless command-line or structured I/O analysis runner for early solver verification, regression, and automation workflows.|PRD §19.3; PRD §22.1|PKG-10|DEL-10-05|OBJ-008, OBJ-009, OBJ-012|FALSE|Provides the R0/R1 path before full GUI maturity.|
|SOW-055|IN|The GUI shall keep solve execution reviewable with background solve execution, progress, cancellation, and diagnostic presentation.|PRD §20; PRD §21; PRD §22.3|PKG-07|DEL-07-07|OBJ-006, OBJ-007|FALSE|Keeps GUI responsiveness and diagnostic traceability visible to users.|
|SOW-056|IN|The project shall select and record implementation stack, runtime, GUI framework, solver-library, and packaging target decisions before implementation work proceeds.|Architecture runway amendment v0.3; PRD §19.2, §22, §25 Open Question 3|PKG-00|DEL-00-01|OBJ-013|FALSE|Architecture baseline selected by SCA-001; exact dependency versions and solver numerical library remain implementation-level TBD.|
|SOW-057|IN|The project shall define repository/module architecture, ownership boundaries, dependency rules, and layer responsibilities before package-level implementation proceeds.|Architecture runway amendment v0.3; SPEC §1-§2|PKG-00|DEL-00-02|OBJ-013|FALSE|Repository/module decisions are injected through SCA-001 architecture basis; concrete folder names may refine during implementation without changing layer responsibilities.|
|SOW-058|IN|The project shall define application-service command, query, background job, cancellation, and transaction boundaries between GUI, headless runner, domain core, solver, storage, and reporting.|Architecture runway amendment v0.3; PRD §20, §22.1-§22.3; SPEC §1|PKG-00|DEL-00-03|OBJ-013|FALSE|Command/query/job and result-envelope boundaries are dispatch constraints; concrete interface language details remain implementation-level TBD.|
|SOW-059|IN|The project shall define persistence format, schema-versioning, migration, canonicalization, and deterministic round-trip strategy before project-file implementation proceeds.|Architecture runway amendment v0.3; SOW-050; PRD §27|PKG-00|DEL-00-04|OBJ-013|FALSE|Persistence architecture basis selected by SCA-001 and MVP local storage profile selected by SCA-003; migration framework remains TBD.|
|SOW-060|IN|The project shall define GUI state, editing, selection, undo/redo, viewport integration, and user workflow architecture before GUI feature slices proceed.|Architecture runway amendment v0.3; PRD §14, §20, §21|PKG-00|DEL-00-05|OBJ-013|FALSE|GUI state basis selected by SCA-001: durable/transient split, service-command mutation route, and scoped undo/redo; exact state library remains TBD.|
|SOW-061|IN|The project shall define diagnostics, warning classes, errors, result envelopes, and user-facing/machine-readable failure contracts across solver, rule packs, GUI, CLI, reports, and adapters.|Architecture runway amendment v0.3; SOW-022, SOW-053, SOW-055; SPEC §7|PKG-00|DEL-00-06|OBJ-013|FALSE|Diagnostics/result-envelope basis selected by SCA-001; exact schema syntax and localization policy remain TBD.|
|SOW-062|IN|The project shall define public and internal API boundaries between GUI, application services, domain core, solver, storage, reporting, private libraries, plugins, and adapters.|Architecture runway amendment v0.3; SOW-030, SOW-038; SPEC §1|PKG-00|DEL-00-02, DEL-00-03, DEL-00-07|OBJ-013|FALSE|API boundary basis selected by SCA-001: schema-first envelopes and no-bypass adapters/plugins; public transport protocol remains TBD.|
|SOW-063|IN|The project shall define layered software test strategy and acceptance gates for architecture, schemas, services, solver, GUI, CLI, reports, packaging, security, and regressions.|Architecture runway amendment v0.3; SOW-026, SOW-032; PRD §16, §22|PKG-00|DEL-00-08|OBJ-013|FALSE|Layered test basis selected by SCA-001: Cargo, Vitest, Playwright, validation and protected-content gates; CI provider and thresholds remain TBD.|
|SOW-064|IN|The product shall be scoped as an analysis-grade piping design engine and stress-model authoring environment with a full internal solver and non-authoritative professional reliance boundary.|PRD v0.2 §1-§5; Scope Change Brief §1-§2|PKG-01|DEL-01-04|OBJ-014, OBJ-018|FALSE|Supersedes the narrower product framing while preserving solver seriousness and professional-boundary limits.|
|SOW-065|IN|The schema-backed physical model shall be the source of truth for editable piping design data, including geometry, components, supports, equipment interfaces, loads, constraints, provenance, unresolved assumptions, and warnings.|PRD v0.2 §5.2, §8.2, FR-MOD-002, FR-MOD-005, FR-MOD-006|PKG-02|DEL-02-01|OBJ-014|FALSE|Physical model is richer than the solver-ready analytical model.|
|SOW-066|IN|The product shall transform the physical model into a solver-ready analytical model deterministically and record transformation warnings when physical design data cannot be represented analytically.|PRD v0.2 §8.3, FR-MOD-007|PKG-13|DEL-13-04|OBJ-014|FALSE|Transformation is a first-class contract between design authoring and solver execution.|
|SOW-067|IN|The product shall support user-supplied design knowledge such as endpoints, line data, routing corridors, no-go volumes, supportable zones, equipment interfaces, access constraints, slope/drain/vent requirements, and owner/project metadata.|PRD v0.2 §8.1, §11.2, §13.1|PKG-13|DEL-13-01, DEL-13-02|OBJ-014, OBJ-018|FALSE|Design knowledge remains user/project supplied; no protected owner or standards data is bundled.|
|SOW-068|IN|The constraint system shall validate available design knowledge for connectivity, route conflicts, clearance conflicts, support-zone conflicts, slope/drain/vent conflicts, and missing required data with provenance-aware messages.|PRD v0.2 FR-KNOW-002 through FR-KNOW-004; §13.2|PKG-13|DEL-13-02, DEL-13-03|OBJ-014|FALSE|Constraint failures are validation messages, not hidden report prose or agent text.|
|SOW-069|IN|All GUI and agent edits shall be represented as structured model operations that pass schema validation, constraint validation, diff preview, and controlled application through the model engine.|PRD v0.2 §5.3, §10.3, FR-AGENT-001 through FR-AGENT-003|PKG-16|DEL-16-01, DEL-16-02, DEL-16-03|OBJ-015|FALSE|Agents may propose operations; they do not mutate accepted engineering state directly.|
|SOW-070|IN|Accepted model operations shall preserve operation history, rationale, assumptions, affected entities, and audit metadata needed for reproducible model-state review.|PRD v0.2 §10.3, FR-AGENT-004, §15.1|PKG-16|DEL-16-03, DEL-16-04|OBJ-015, OBJ-018|FALSE|Operation history supports design iteration without creating professional approval claims.|
|SOW-071|IN|The product shall save named immutable model states with tags, notes, external references, unresolved assumptions, warnings, and deterministic hashes.|PRD v0.2 §8.6, FR-CMP-001, §15.1|PKG-14|DEL-14-01, DEL-14-03|OBJ-016|FALSE|Model states are flexible metadata records, not formal prover approval states.|
|SOW-072|IN|The product shall save analysis runs bound to exact model states, solver versions, settings, units, load cases, diagnostics, results, rule-pack references, library references, and result hashes.|PRD v0.2 §8.7, FR-CMP-002, §15.2|PKG-14|DEL-14-02, DEL-14-04|OBJ-016|FALSE|Results attach to immutable analysis runs rather than a mutable current model only.|
|SOW-073|IN|The product shall compare two model states and/or two analysis runs deterministically using stable IDs, manual mappings where required, unit-normalized result deltas, and tolerance profiles.|PRD v0.2 §8.8, FR-CMP-003 through FR-CMP-010, §15.3|PKG-14|DEL-14-03, DEL-14-04, DEL-14-05|OBJ-016, OBJ-018|FALSE|Comparison is diagnostic/audit functionality, not automatic external validation or acceptance.|
|SOW-074|IN|The product shall generate schema-compliant handoff packages with model hash, units manifest, entity IDs, library/rule references, unresolved assumptions, warnings, target mapping metadata, and unsupported-target flags.|PRD v0.2 §16.1, FR-HAND-001 through FR-HAND-004|PKG-15, PKG-17|DEL-15-01, DEL-15-02, DEL-15-03, DEL-17-01, DEL-17-02, DEL-17-03, DEL-17-04, DEL-17-06, DEL-17-07, DEL-17-08, DEL-17-09|OBJ-017, OBJ-018|FALSE|Handoff quality is central; SCA-004 makes stable ID maps, export manifests, loss reports, target mapping metadata, unsupported/approximated/delegated behavior reporting, CAEPIPE MBF, conservative PCF, GLB/glTF review geometry, and native JSON export packages explicit scope.|
|SOW-075|IN|The product shall support external-prover workflow metadata without forcing a formal prover-status lifecycle, automatic professional acceptance record, or comprehensive commercial-tool result ingestion in the MVP.|PRD v0.2 §4.2, §16.2-§16.4, §23|PKG-15, PKG-17|DEL-15-04, DEL-17-01, DEL-17-04, DEL-17-05, DEL-17-09|OBJ-017, OBJ-018|FALSE|Use flexible names, tags, notes, external references, comparison reports, and optional user-owned external harness metadata instead of hard-coded approval statuses; SCA-004 permits CAEPIPE harness records only as non-authoritative external-run evidence.|
|SOW-076|IN|The GUI shall support design-authoring and comparison workflows, including design knowledge panels, constraint/warning panels, state/run browsers, comparison tables, and graphical comparison overlays.|PRD v0.2 §11.3, §14.5, §15.3|PKG-07|DEL-07-08|OBJ-015, OBJ-016|FALSE|Expands GUI from stress input editing into design iteration and review.|

## 10. Coverage and telemetry


|Metric|Value|
|---|---|
|ScopeItemCount|76|
|PackageCount|18|
|DeliverableCount|101|
|ObjectiveCount|18|
|UnassignedScopeItems|0|
|ScopeItemsWithoutDeliverableMapping|0|
|UnmappedObjectives|0|
|ContextEnvelopeCounts|S=9, M=70, L=22, XL=0|
|OpenIssues|17|
|Revision|0.7 — 2026-05-18|

## 11. Open issues


|OpenIssueID|Issue|Related Scope|Owner/Resolution Path|
|---|---|---|---|
|OI-001|License selection is TBD: permissive vs copyleft requires human decision.|SOW-001,SOW-048|Human project authority|
|OI-002|Implementation architecture baseline selected by SCA-001; exact dependency versions, solver numerical library, and platform-specific packaging details remain TBD.|SOW-032,SOW-036,SOW-056|Implementation planning / human ruling|
|OI-003|Legal review is required for public component/material data policy before accepting community data.|SOW-003,SOW-019,SOW-028|Legal/human review|
|OI-004|Specific supported import/export formats are now decomposed through PKG-17 for native JSON, CAEPIPE MBF, stress-neutral CSV/JSON, conservative PCF, GLB/glTF review geometry, and adapter SDK work; exact target field coverage remains deliverable-local TBD until source-basis and profile contracts mature.|SOW-030,SOW-046,SOW-062,SOW-074|PKG-17 / ORCHESTRATOR workflow|
|OI-005|Numerical tolerances and performance targets are TBD pending solver prototype.|SOW-026,SOW-035,SOW-053|Solver lead/human|
|OI-006|Rule-pack evaluator must be sandboxed, unit-aware, and declarative; exact expression grammar/library remains TBD.|SOW-045|PKG-06 / human architecture decision|
|OI-007|Human approval workflow for project-specific calculation reports is TBD and outside solver authority.|SOW-034,SOW-047|Human/project governance|
|OI-008|Telemetry policy defaults to no telemetry; any telemetry design requires explicit human approval.|SOW-037|Security/product decision|
|OI-009|Dynamic analysis modules such as modal or response-spectrum analysis are PRD Could/post-MVP items and are not decomposed for current execution.|PRD FR-024|Scope change if promoted|
|OI-010|Private rule-pack encryption default requires human/security decision.|PRD §25 Open Question 7|Security/product decision|
|OI-011|Canonical persistence/hash basis selected by SCA-001; MVP physical storage profile selected by SCA-003 as local SQLite-backed project store/index with rebuildable sidecars; migration implementation details remain TBD.|SOW-050,SOW-059|PKG-02 / downstream TASK implementation|
|OI-012|Physical model schema boundaries, operation granularity, and physical-to-analytical transformation loss classes require technical architecture detail before implementation briefs are sealed.|SOW-065,SOW-066,SOW-069|ORCHESTRATOR / SOFTWARE_DECOMP follow-up architecture brief|
|OI-013|Design knowledge and constraint taxonomy must avoid importing owner standards, protected code criteria, or proprietary project data into public examples.|SOW-067,SOW-068|PKG-13 / security-privacy / IP governance review|
|OI-014|Comparison tolerance defaults and mapping workflows are TBD pending solver/result schema prototypes.|SOW-073|PKG-14 / validation-qa / human product decision|
|OI-015|Handoff target list now starts with CAEPIPE MBF as first validation target plus native JSON, stress-neutral CSV/JSON, conservative PCF, GLB/glTF review geometry, and adapter SDK surfaces; concrete mappings, unsupported behavior taxonomy, and target field coverage remain gated by DEL-17-01 and DEL-17-02.|SOW-074,SOW-075|PKG-15 / PKG-17 / interop-build|
|OI-017|CAEPIPE developer-team clarifications, supported version assumptions, MBF field coverage, batch/harness operating expectations, CSV result semantics, PCF translator caveats, and redistribution-safe source evidence must be gathered before target-specific implementation claims mature.|SOW-030,SOW-074,SOW-075|DEL-17-01 / CAEPIPE developer-team question dossier|
|OI-016|Agent operation autonomy level remains TBD; default SCA-002 scope requires user acceptance and audit trail for proposed changes.|SOW-069,SOW-070|PKG-16 / security-privacy / human product decision|

## 12. Decision log


|DecisionID|Decision|Basis|Status|
|---|---|---|---|
|DEC-001|Adopt code-neutral architecture: mechanics in public core, code-specific values in user/private rule packs.|Conversation intent; INTENT.md; PRD §6.1|Accepted as current draft design basis.|
|DEC-002|Adopt 3D centerline/frame solver as primary global analysis model.|Conversation intent; MWK 1956 lineage; PRD §11|Accepted as current draft design basis; local FEA remains specialized handoff.|
|DEC-003|Exclude protected standards/code data from public examples, libraries, reports, and rule packs.|INTENT.md; PRD §5, §17|Accepted as draft design basis; legal review required.|
|DEC-004|Use Chirality-style flat package/deliverable decomposition with context envelopes.|AGENT_SOFTWARE_DECOMP.md|Accepted for this draft decomposition.|
|DEC-005|Treat generated software results as decision support; human professional responsibility remains non-transferable.|PROFESSIONAL_ENGINEERING.md; CHIRALITY_FRAMEWORK.md|Accepted as professional-boundary principle.|
|DEC-006|Add PRD-derived coverage corrections for persistence, section properties, concentrated loads, solver diagnostics, headless execution, and solve-execution UX.|Formal SOFTWARE_DECOMP review pass against PRD FR-001, FR-005, FR-007, PRD §20, and release milestones.|Accepted by human project authority on 2026-04-30.|
|DEC-007|Add `PKG-00 — Software Architecture Runway` as the first workflow gate before remaining package-level document drafting or implementation planning.|Human ORCHESTRATOR review after PREPARATION; architecture coherence concern raised on 2026-04-30.|Accepted by human project authority on 2026-04-30.|
|DEC-008|Use `PKG-00` `SEMANTIC_READY` content as SCA-001 architecture-basis candidate for sealed brief injection, without treating it as `ISSUED`.|Human approval of SCA-001 Gate 3 and Gate 4 on 2026-04-30.|Accepted for downstream context propagation.|
|DEC-009|Adopt Rust core/application services, Tauri 2 desktop shell, TypeScript/React/Vite GUI, and Three.js viewport as the implementation runtime/UI baseline.|SCA-001 Gate 3 amendment preview and human approval on 2026-04-30.|Accepted; exact dependency versions and component/state libraries remain TBD.|
|DEC-010|Adopt JSON Schema 2020-12, schema-first command/query/job/result envelopes, and canonical JSON with JCS-compatible hashing as the schema/API/persistence baseline.|SCA-001 Gate 3 amendment preview and human approval on 2026-04-30.|Accepted; public transport protocol, import/export formats, and migration framework remain TBD; MVP physical storage profile resolved by DEC-017.|
|DEC-011|Adopt Cargo tests, Vitest, Playwright, validation gates, protected-content/provenance gates, and deterministic solver/rule verification gates as the software test baseline.|SCA-001 Gate 3 amendment preview and human approval on 2026-04-30.|Accepted; CI provider, coverage thresholds, and performance thresholds remain TBD.|
|DEC-012|Keep exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider, coverage thresholds, and physical project package/container as implementation-level TBDs unless a sealed brief or later human ruling resolves them.|SCA-001 Gate 3 amendment preview and human approval on 2026-04-30.|Accepted as the SCA-001 remaining-TBD boundary; physical project package/container portion superseded by DEC-017.|
|DEC-013|Admit PRD v0.2 as the SCA-002 working design basis and shift product framing to an analysis-grade piping design engine and stress-model authoring environment with a full internal solver.|docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md; Scope Change Brief §1-§2.|Accepted for decomposition revision 0.5.|
|DEC-014|Add PKG-13 through PKG-16 for design knowledge/constraints, states-runs-comparison, handoff/prover workflow, and model operation/agent proposal framework while preserving existing package IDs.|SCA-002 impact assessment and stable-ID invariant.|Accepted; downstream folders and production docs require PREPARATION/ORCHESTRATOR refresh.|
|DEC-015|Treat comparison and external-prover metadata as diagnostic/handoff support, not automatic professional validation, approval, certification, or code-compliance status.|PRD v0.2 §4.3, §16.2-§16.4, §23.|Accepted as professional-boundary constraint.|
|DEC-016|Defer commercial stress software output parsers, formal prover lifecycle/status enforcement, automatic professional acceptance records, advanced route optimization, and dynamic analysis promotion unless later SCA explicitly reprioritizes them.|PRD v0.2 §7, §16, §23-§24; Scope Change Brief §22.|Accepted as MVP/downstream boundary.|
|DEC-017|Accept the MVP local storage profile: SQLite is the local project store/index substrate, canonical JSON/JCS remains domain truth, SQLite FTS5/BM25 retrieval sidecars are rebuildable, NumPy sidecars are optional non-authoritative caches only if later justified, large files are referenced in place by path/URI plus hash/metadata by default, and no hosted DB, daemon, required network, cloud sync, telemetry path, or direct plugin/adapter SQL access is authorized.|SCA-003 human request and approved implementation plan on 2026-05-17; local-first/product-storage discussion; AB-00-04 persistence baseline.|Accepted for MVP; downstream schema/docs/code implementation remains gated through bounded TASK work.|
|DEC-018|Accept the D-01 unit-catalog recommendation: SI-canonical internal unit set with a dual display catalog, exact definitional conversion constants recorded as governed values, dual absolute/interval temperature semantics, explicit gauge/absolute pressure quantity kinds with no silent atmospheric default, and a two-tier conversion-witness tolerance policy.|Human project authority ruling on 2026-06-10 accepting `execution/_Coordination/_DECISIONS/D-01_unit_catalog_acceptance.md` as proposed (“Use SI-canonical with dual catalog”).|Accepted; unblocks completion-plan Phase B (FR-002); implementation remains bounded-tranche work.|
|DEC-019|Accept the D-08 model-document schema-migration recommendation: two-track versioning (store schema via the SQLite `user_version` ledger; model documents via per-document semver with an explicit transform chain), migrate-in-memory-on-open with persist-on-save, and explicit-operation evidence guarantees with no silent destructive rewrites.|Human project authority ruling on 2026-06-10 accepting `execution/_Coordination/_DECISIONS/D-08_model_document_schema_migration.md` as proposed (“Use per-document semver with migrate-on-open semantics”).|Accepted; unblocks completion-plan Phase A item A2 (model-document persistence); implementation remains bounded-tranche work.|
## 13. Gate posture

This v0.7 decomposition is the accepted current decomposition basis after SCA-004.

PREPARATION may scaffold package and deliverable folders from this decomposition and the companion registers after ORCHESTRATOR plans the accepted downstream refreshes. `PKG-00` remains `SEMANTIC_READY`, not `ISSUED`, but its SCA-001 architecture basis as amended by SCA-003 and SCA-004 may be injected into `PKG-01` through `PKG-17` sealed contexts and future TASK briefs. Type 2 execution still requires one sealed deliverable context, explicit write scope, applicable invariants, and acceptance criteria. Existing DEV-001 dispatch, DAG/blocker, lifecycle, dependency, implementation-evidence, schema, docs, and code surfaces may be stale relative to revision 0.7 until refreshed by their owning workflows; SCA-004 does not directly update downstream schema/code/runtime artifacts or approve DAG-005 as active graph authority.
