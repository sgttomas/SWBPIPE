from pathlib import Path
import csv,json,hashlib
r=Path.cwd();s=r/'projects/chirality-piping/execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/V1/children/V1-R';base='projects/chirality-piping/'
def csvout(name,columns,rows):
 with (s/name).open('w',newline='') as f:
  w=csv.writer(f);w.writerow(columns);w.writerows(rows)
findings=[
['V1R-01','Reference independence','OBSERVATION','medium','DEL-09-01','Generic TP014/015A expected-value records derive from the production solve; fixed-number validator remains independent. Generic record agreement alone does not verify physics.','validation/benchmarks/mechanics/src/lib.rs:2086;:2180;:5883;:7741;REFERENCE_CORRECTIONS_AND_LIMITS.md','CONFIRMED_STATIC_LINEAGE','WORKING_ITEMS PKG09','Freeze separate hand-calculated expected records and retain fixed-number validator; mutation test generic runner'],
['V1R-02','Observation reachability','OBSERVATION','medium','DEL-09-01','Cantilever generic moment observation is literal6*10 rather than recovered reaction; a green moment comparison does not test reaction extraction. Thermal/spring observation reachability requires separate accounting.','validation/benchmarks/mechanics/src/lib.rs:805;EARLY_REFERENCE_CRITERIA.md','CONFIRMED_STATIC_LINEAGE','WORKING_ITEMS PKG09','Compare actual solved moment to frozen equilibrium oracle; retain independent thermal integration checks'],
['V1R-03','Nonlinear reference depth','UNKNOWN','high','DEL-09-03','Iteration/delta envelopes in nonlinear hand calculations are algorithm observations; a complete independently derived physical solution is not supplied for every multi-support fixture. State stability alone cannot prove equilibrium/complementarity.','validation/hand_calcs/nonlinear/convergence_observations.md;validation/hand_calcs/nonlinear/assembled_multi_support_multi_dof_acceptance.md;REFERENCES.csv','REFERENCE_COVERAGE_GAP_NOT_ENGINE_DEFECT','EVALUATION numerical mechanics and WORKING_ITEMS PKG09','Independent feasible-state equilibrium and complementarity checks for mixed supports; keep new engineering policies Owner-gated'],
['V1R-04','Approximation and tolerance applicability','UNKNOWN','medium','DEL-09-01','Expansion-loop bending-only oracle compared through finite axial boost1e5 uses5e-7 measured-approximation tolerance; this does not independently validate unboosted finite-axial response or establish a production tolerance.','validation/benchmarks/mechanics/src/lib.rs:3096;validation/hand_calcs/mechanics/expansion_loop_curved_bend_thermal.md;POLICY_RECORDS.json','SCOPED_EVIDENCE_LIMIT','EVALUATION numerical mechanics','Validate finite-axial curved frame with independent energy integration; resolve any new tolerance through existing Owner instrument'],
['V1R-05','Reference corroboration','OBSERVATION','info','DEL-09-01','Independent vector-energy Simpson oracle corroborates42 rounded free-tip/reaction/station entries across four curved-bend distributed cases; no production runtime comparison performed by this specialist.','independent_oracles.py;INDEPENDENT_ORACLE_RESULTS.json;ARC_REFERENCE_COMPARISON.json','INDEPENDENT_REFERENCE_CORROBORATED','EVALUATION numerical mechanics','Replay script; compare production separately under declared benchmark policy'],
['V1R-06','Formal witness lineage','OBSERVATION','info','DEL-03-08 and DEL-09-02','Decimal formula witness is independently interpreted and separately bound to production section calculator; generated Markdown is derivative rather than another independent oracle.','tests/test_calculation_witness.py:66;validation/witness/tools/witness_validator.py;validation/hand_calcs/stress/README.md','CONFIRMED_STATIC_LINEAGE','EVALUATION verification','Retain production-calculator versus independent witness checks; distinguish from generated rendering reproducibility'],
]
csvout('FINDINGS.csv',['FindingID','Concern','Classification','Severity','Scope','Claim','EvidenceRefs','Status','RecommendedOwner','RerunRequirement'],findings)
cover=[]
def row(fam,oracle,status,limit,evidence):cover.append([fam,oracle,status,limit,evidence])
row('Inputs/units','Equivalent SI systems; translation/rotation covariance','CRITERIA_FROZEN','Actual unit replay owned by integration; no production verdict here','EARLY_REFERENCE_CRITERIA.md:5')
row('Sections/materials','Annulus integrals and linear interpolation; witness Decimal interpreter','REFERENCE_LINEAGE_REVIEWED','No empirical material curve validation','REFERENCES.csv;REFERENCE_CORRECTIONS_AND_LIMITS.md')
row('Axial/bending/torsion','EA/L;PL3/(3EI);TL/GJ; exact rational elementary cases','INDEPENDENT_ORACLES_EVALUATED','Does not cover every element orientation or formulation','INDEPENDENT_ORACLE_RESULTS.json')
row('Curved bends','Vector equilibrium + numerical strain-energy integration','FOUR_REFERENCE_CASES_CORROBORATED','Quarter circle k1/k2 only; no shear/ovalization','ARC_REFERENCE_COMPARISON.json')
row('Rigid/user-stiffness components','Rigid motion compatibility; specified matrix symmetry/work f=Kdelta','CRITERIA_AVAILABLE_RUNTIME_UNKNOWN','Not a constitutive oracle for unsupported supplied stiffness; no separate handcalc family in73inventory','REFERENCES.csv;EARLY_REFERENCE_CRITERIA.md')
row('Linear supports','Parallel spring and prescribed reaction equilibrium','INDEPENDENT_ORACLES_EVALUATED','Full stability/rank solve belongs numerical manager','INDEPENDENT_ORACLE_RESULTS.json')
row('Nonlinear supports','Scalar contact complementarity and bounded friction balance','REFERENCE_DEPTH_GAP_VISIBLE','Multisupport iteration envelopes not independent all-vector solutions','REFERENCES.csv;FINDINGS.csv:V1R-03')
row('Loads','Force/moment integration; Hermite work-equivalent distribution; uniform/partial spans','ANALYTICAL_REFERENCE_REVIEWED','Preview force-lumping contract distinct from work equivalence','EARLY_REFERENCE_CRITERIA.md;REFERENCES.csv')
row('Thermal/pressure','Thermal strain/constraint force and membrane equilibrium','INDEPENDENT_SIMPLE_CASES_EVALUATED','Thin-wall assumptions and pressure radius explicit; thermal history not adopted','INDEPENDENT_ORACLE_RESULTS.json;REFERENCES.csv')
row('Wind/seismic','User factors times area/mass; force centroid conservation','INDEPENDENT_WIND_CASE_EVALUATED','Equivalent static only; no dynamics or code coefficients','INDEPENDENT_ORACLE_RESULTS.json;REFERENCES.csv')
row('Linear numerics','Residual/backward error; exact diagonal and singular conditioning; Toeplitz eigenvalues','CRITERIA_FROZEN','No tolerance invention; norm conventions explicit','REFERENCE_CORRECTIONS_AND_LIMITS.md')
row('Stress/recovery','N/A;M/Z;Tr/J; section equilibrium','INDEPENDENT_SIMPLE_CASES_EVALUATED','Downstream stress notes share mechanics inputs; no fatigue/code acceptance','INDEPENDENT_ORACLE_RESULTS.json;REFERENCES.csv')
row('Combinations/publication','Linear superposition only compatible bases; distinguish signed states and selected solution','CRITERIA_AVAILABLE_RUNTIME_UNKNOWN','No history/friction superposition assumption; identity whole-contract review elsewhere','EARLY_REFERENCE_CRITERIA.md;REFERENCES.csv')
row('Rust robustness','Finite representation and dimensioned diagnostic boundary','CRITERIA_AVAILABLE_RUNTIME_UNKNOWN','No unsafe/FFI audit performed by reference specialist','REFERENCE_CORRECTIONS_AND_LIMITS.md')
for i in range(1,8):row('Mandatory witness '+str(i),'Numbered criterion in early packet','CRITERIA_FROZEN_PRODUCTION_OUTCOME_WITH_SIBLING','No independently reproduced production defect claimed by reference specialist','EARLY_REFERENCE_CRITERIA.md')
csvout('COVERAGE.csv',['Family','Oracle','Disposition','Limit','EvidenceRefs'],cover)
(s/'EVALUATION_PROTOCOL.md').write_text('''# Reference audit protocol

Accepted basis: frozen source2be412ccea62bdc4bd96deb082c46d7a792076ea; decomp0.12/SCA009/DAG010; root APPROVED_PLAN.md and USER_CHOICES.json. TASK+evaluation-protocol under EVALUATION V1. Scope: all existing hand-calculation lineage, numerical-policy applicability, independent criteria for seven leads and representative implemented families. Writes exact V1-R only. No children, heavy builds, existing suites, source edits, or Git mutations. Parent owns production comparisons and accepts fan-in. No rubric or score.

Freeze criteria before comparisons; inventory and hash all73handcalcs; inspect source executable lineage; evaluate independent exact-rational elementary and quadrature arc references; retain separate algorithm-observation and physical-oracle classifications; preserve explicit gaps. External primary sources corroborate method only. New convergence/engineering acceptance thresholds and physics assumptions route to Owner; no threshold is adopted here.
''')
(s/'EVALUATION_REPORT.md').write_text('''# V1-R — Independent references and oracle audit

## Basis

Frozen production source `2be412ccea62bdc4bd96deb082c46d7a792076ea`, decomp0.12/SCA009/DAG010 and the parent-approved comprehensive audit plan. This is a derivative evaluation packet, not decomposition truth or an engineering acceptance act. `INIT-TASK.md` supplies exact authority. Production files and existing expectations remained read-only.

## Method

Applied TASK plus evaluation-protocol and all three companions. Inventory contains **73 hand-calculation Markdown documents**, each with its own path, SHA256, family, reference lineage, disposition, applicability limits and inspected section inventory in `REFERENCES.csv`. Read the mechanical/stress analytical derivations and nonlinear state/observation sections; inspected corresponding executable benchmark constructors, observation seams, fixed-number checks, calculation-witness evaluator and production-calculator binding. `REFERENCE_INPUT_MANIFEST.json` and `ADDITIONAL_SOURCE_MANIFEST.json` freeze that input basis.

The initial packet froze seven mechanical/contract identities before new numerical evaluation. `independent_oracles.py` imports only Python standard library, computes25exact-rational elementary examples, and independently integrates four curved-beam energy cases using vector free-body equilibrium and Simpson quadrature. It never imports production/benchmark code. `INDEPENDENT_ORACLE_RESULTS.json` preserves exact rational values, floating representations, full arc vectors/stations and1024/2048 refinement evidence. `ARC_REFERENCE_COMPARISON.json` compares42independently evaluated entries to the existing rounded hand-calculation table mirrored in source. Largest entry difference is2.561364453868009e-10 in that entry's units; individual values retain units by field. Largest reaction refinement difference is4.986077328794636e-9. Neither number is an invented acceptance threshold. No existing suite or production runtime was executed by this specialist.

Primary-source method corroboration: [LAPACK Standard Error Analysis](https://www.netlib.org/lapack/lug/node78.html) explains why conditioning matters alongside backward error; it does not certify this Rust solver. [MIT OCW structural mechanics textbook index](https://ocw.mit.edu/courses/2-080j-structural-mechanics-fall-2013/pages/open-textbook/) identifies beam and energy chapters; index search retrieval succeeded but direct page fetch timed out, so no chapter-level equation verification is claimed. The new oracle is derived in the audit code, rather than attributed to an unread chapter. No piping-design extracted equations were used.

## Coverage

`COVERAGE.csv` lists14technical families and all7mandatory leads. All73existing documents have individual lineage dispositions; this is **not** a claim that every arithmetic line in all73was independently recomputed.25representative exact-rational results cover straight-beam/branch/thermal/spring/friction/stress/wind identities; four numerical arc cases corroborate in-plane/out-of-plane k1/k2 reference tables. Rigid/user-stiffness components and full mixed-support feasible states lack a separately evaluated all-vector oracle in this reference slice; source/experiment verdicts belong the numerical and integration managers. Full portal numerical response is explicitly implementation-derived regression in its own note. TP002Yresponse is likewise distinguished from its independently derived axial/lumped quantities.

## Validated-return inventory

No child dispatches. This Agent2 does not delegate. Parent V1 validates this return; no parent acceptance is presumed. Current record uses delegated-harness-native with nondelegation instruction+config asserted. Actual model identity is unknown where runtime does not expose it.

## Findings

Six register entries in `FINDINGS.csv` separate confirmed source-lineage facts, reference gaps and positive independent corroboration. The main evidence weakness is **uneven numerical independence**, not absence of useful benchmarks.

- Generic TP014/015A expected records are populated from executed solve/envelope helpers. A generic comparison against those records can be circular. However, the independent fixed-number validator at5883–5918and invoking test at7741protect known loads, displacements, reactions and stations. Do not call the entire fixture circular.
- Cantilever generic moment observation is literal6*10rather than a recovered moment. It cannot alone detect broken reaction extraction. Similar helper-level thermal/spring identity checks must not be counted as full assembled-solve reachability; sibling coverage evaluates those paths and their separate integration tests.
- Nonlinear iterations, deltas and residual envelopes are explicitly implementation-measured regression observations. They cannot replace independently derived equilibrium, compatibility and complementarity of all published vectors. The scalar bounded-friction/gap calculations are useful independent physical checks within their declared static assumptions; an expected iteration count is an algorithm contract, not a physical theorem.
- The expansion-loop reference deliberately neglects axial strain, whereas its benchmark approximates that limit by boosting axial rigidity1e5. The5e-7relative comparison and absolute equilibrium floors document measured approximation/conditioning limits. This does not verify ordinary finite-axial response or set a universal tolerance; retain its scope and add independent finite-axial evidence.
- The independently written arc quadrature corroborates the distributed curved-bend hand-calculation table. It shares the declared energy assumptions, not the production executable formulas.
- The formal Decimal witness is independently interpreted and separately compared with the production section calculator in `test_calculation_witness.py:66`. This binding strengthens independence; it does not generate the oracle through the production calculator. Generated Markdown/MathML provide reproducible presentation, not additional independent physics evidence.

## Conflicts / unknowns

Later accepted policy must take precedence over stale narrative TBD statements. `POLICY_RECORDS.json` preserves14policy documents verbatim with hashes. DEC026governs reference classes and relative+absolute per-kind criteria; incomplete floors remain unresolved. DEC046validation seed and multisupport policies have narrow observation scopes. The later C-Brelease record promotes count0/max4only to named public-benchmark release scope; it does not establish production-wide equilibrium or a release act. `REFERENCE_CORRECTIONS_AND_LIMITS.md` adds this correction without rewriting the early packet.

No third-party experimental or commercial results were independently acquired. That is a limit of the evidence, not proof of wrong formulas. No new convergence or engineering threshold is approved. Finite representation, diagonal/singular conditioning and coherent vector selection have exact mathematical criteria requiring no new threshold. Toeplitz spectral condition numbers must not be confused with a different norm's declared condition number.

## Optional scorecard

None requested or produced.

## Recommendations

Repair reference plumbing separately from production physics: retain frozen independent expected values, label implementation-derived regression records, extract actual solver reactions rather than restating expected formulas, and retain positive-control mutation checks. Preserve fixed-number integration tests already providing independent checks. Extend nonlinear references with independently solved feasible states and explicit load-history limits. Add finite-axial curved-frame references before treating bending-only limit agreement as full formulation coverage. Carry quantity-specific units and near-zero criteria through report generation.

## Decision queue

No Owner decision is needed to report the lineage facts or reuse exact algebraic identities. New production convergence/acceptance policies, friction history assumptions, unsupported physics and public compatibility changes remain Owner decisions under the approved plan. Parent should combine sibling evidence before proposing concrete repairs or amendments. No scope amendment is asserted by this reference audit.

## Handoff summary

Reference audit output is complete with explicit depth limits. Parent numerical/integration returns supply actual production outcomes for the seven leads; this packet supplies independently defensible expected behavior. All73source documents plus captured executable/policy inputs are hash-bound. Re-run independent_oracles.py then build_reference_evidence.py from the repository root to regenerate analytical evidence in this isolated subtree; do not overwrite accepted snapshots. Parent must independently review/freeze the entire baseline before any production repair. This packet changes no lifecycle, release or professional hold.
''')
(s/'HANDOFF.md').write_text('''# Reference audit handoff

Upstream: source2be412ccea62bdc4bd96deb082c46d7a792076ea; decomp0.12/SCA009/DAG010; approved audit plan and exact user choices under root AgentRuns. Derivative-package status: evaluation only. Closure verdict: reference slice complete with declared coverage gaps; parent acceptance pending. No production correctness or lifecycle closure claimed.

Current artifacts:73reference rows;14policy records;25rational results;4arc quadrature cases/42table comparisons;21coverage rows;6findings. Early packet retained unchanged; additive correction controls refined policy interpretation. No children and no production execution. Actual model unknown; Agent2 role nondelegation instruction+config asserted.

Remaining blockers: parent needs numerical/integration production results, full baseline independent review, and concrete Owner decisions only where repair crosses approved boundaries. Unknown all-vector multi-support reference and finite-axial curved-frame depth remain visible. EVALUATION owns fan-in; WORKING_ITEMS PKG09owns bounded reference repairs after baseline acceptance; numerical/integration managers own engine findings. No new engineering tolerance or scope amendment is proposed as already accepted.

Rerun source hash validation and both audit-only scripts; compare original fixtures without editing expectations. A changed production/reference/policy source needs renewed lineage review. All executed commands wrote only V1-Rartifacts; existing sources were read-only. Baseline and future repair results must occupy separate immutable snapshots.
''')
# Verify all captured read-only source inputs before sealing.
checks={}
for name in ['REFERENCE_INPUT_MANIFEST.json','ADDITIONAL_SOURCE_MANIFEST.json']:
 for p,h in json.loads((s/name).read_text()).items():checks[p]=hashlib.sha256((r/p).read_bytes()).hexdigest()==h
for item in json.loads((s/'POLICY_RECORDS.json').read_text()):checks[item['path']]=hashlib.sha256((r/item['path']).read_bytes()).hexdigest()==item['sha256']
(s/'QA.json').write_text(json.dumps({'source_hash_checks':checks,'all_sources_unchanged':all(checks.values()),'reference_rows':73,'finding_schema_columns':10,'no_production_execution':True,'no_delegation':True},indent=2)+'\n')
run=s/'_run_records/TASK_RUN_2026-09-05_V1-R.md';t=run.read_text().replace('run-status: PENDING','run-status: SUCCESS').replace('(pending)','Completed; see EVALUATION_REPORT.md and QA.json.')
t+='\nToolsUsed: shell git/read/rg/Python; web primary-source search/open. ToolPolicyCompliance: PASS. AppliedChanges: own output subtree only. Missing: complete independent nonlinear all-vector and finite-axial loop references not derived in this slice. NeedsHumanRuling: none for reference findings; new engineering policies remain gated.\n'
run.write_text(t)
print('QA',all(checks.values()),len(checks))
