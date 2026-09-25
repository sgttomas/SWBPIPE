# N05 admission recovery diagnosis

**Finding: the completed real product N05 observations fail the unchanged analytical relative 1e-9 criterion and the pre-containment TS/Python numerical admission functions nevertheless admit them.** This is a blocking accuracy/admission gap for a cut claiming qualified Current. It is the actual product manifestation of the previously explicit N05/NP-A recovery obligation, not a newly discovered physical instability. N06 already blocks as assembly unresolved. No warning-only closure is supported.

## Exact observed status and error

The original observer completed with exit 0 and source locks unchanged. Exit 0 only means observation completed: `src/main.rs` makes no passing assertions. This child did not rerun it. Original `stdout.jsonl` SHA-256 is `36d717a2e3eff94f6d60092ddee19a5c12b52469aa951e3464e97a7839f2e61c`; selected product records were checked equal to records in that raw output.

N05 is a physical positive spring with root RX free, k=1e-4 N*m/rad and tip T=1e-8 N*m. Frozen source-intended root rotation is exactly 1e-4 rad; tip is 1e-4 + T/(687800*pi). Comparison uses 100-digit Decimal and exact decoding of each emitted binary64 value, without altering the frozen reference.

| Real product mode | Published root RX (rad) | Root relative error | Tip relative error | Standing before containment |
|---|---:|---:|---:|---|
| dense_scrutiny | 9.999970420819728e-5 | -2.957918027219e-6 | -2.957918027202e-6 | MECHANICS_SOLVED / sensitive |
| sparse_interactive | 1.0000016987352618e-4 | +1.698735261799e-6 | +1.698735261736e-6 | MECHANICS_SOLVED / sensitive |

All four errors exceed 1e-9. Independently evaluating physical spring action -k*published root RX inherits the same approximately 3.0/1.7 ppm errors. This derived action is explicitly **not** asserted to be an emitted support-moment row. Actual emitted signed member torsion also fails: end-j is 9.999979511121637e-9 N*m (dense; relative -2.048887836281e-6), and 1.0000007932831068e-8 N*m (sparse; +7.932831067592e-7), versus +1e-8. The i-end has the opposite expected sign; interior section actions have the positive expected sign.

Both real N05 carriers declare `represented_equations_retained`, `passive_model_basis`, `accuracy_evidence=not_claimed`, and warning `NUMERICAL_INTEGRITY_SENSITIVE`. Their diagnostic reports show original/intended residual screens passed, zero refinement attempts, rcond about 1.0578e-11, and assembly amplification estimates about 3.3975e-6. These fields truthfully avoid a guaranteed-forward-accuracy claim; they do not cure the observed accuracy failure.

## What admission was actually exercised

[TS_ADMISSION.json](TS_ADMISSION.json) invokes the actual `numericalResultStanding` and `sourceContract` functions on the **unchanged actual product carrier and authored input** from each saved observation. Both N05 modes return `{contract: precision, status: integrity_checked, eligible: true, findings: []}`. [PYTHON_COMPARISON_ADMISSION.json](PYTHON_COMPARISON_ADMISSION.json) calls actual Python `numerical_use_standing` with the actual authored load-case basis; both return `numerically_eligible`. N06 returns `needs_recompute` in both languages; its real carrier has no results and an assembly-loss blocking diagnostic.

These are numerical admission results, **not a claim that an actual UI session displayed Current or an export completed**. The observer did not produce an input manifest, AnalysisRun or authenticated build binding. `resultsSessionState.ts:50–64` additionally requires those model/run/manifest/semantic/producer bindings. The final conjunct delegates numerical eligibility to the accepting TS function. `resultExportAdapter.ts:153` and `StressNeutralExportPanel.tsx:443` use that same numerical gate before their other authentication checks. Thus authenticated paths are exposed to the admission gap; their other bindings were not fabricated or simulated here.

Both admission sources were captured immediately after the original probes in `*.observed-source` with [PROBE_SOURCE_FREEZE.json](PROBE_SOURCE_FREEZE.json). After parent notified intent to repair admission, frozen replay reproduced both original outputs byte-for-byte at the JSON-value level. Current working source may now differ because the parent owns containment. This return characterizes the recorded pre-containment candidate.

## Assembly loss versus solver error

NP-A rows contain only debug strings from generic `solve_dense` and legacy `solve_symmetric_system` matrix calls (`main.rs:50–54`). They have no product envelope or Current/consumer status. None was invented. Their frozen a is `0x1.07c49b6ac7e22p+21`; it must not be mistaken for the product's coefficient.

The product diagnostic independently retains its root diagonal expansion `[1.6987323761465545e-10, 2160787.4272390613]` and difference expansion `[1.6987323761465545e-10]`. For the actual x-aligned one-frame/one-spring input, the source's torsion block is [[a,-a],[-a,a]]. Subtracting the exact represented input k from that retained diagonal expansion infers product a=`0x1.07c49b6ac7e26p+21`, four ulps above NP-A's a. This is a **source-traced exact reconstruction**, not a claim that a complete product matrix was captured.

The reconstructed stored spring increment is nevertheless the identical `0.00009999983012676239013671875`. [ARITHMETIC_DECOMPOSITION.json](ARITHMETIC_DECOMPOSITION.json) distinguishes:

- Assembly alone biases the exact stored root solution upward by 1.698735261843e-6 relative to the contribution-preserved solution.
- Sparse's published root differs from that exact stored solution by only -1.7585e-17 relative. Its main error is solving the already rounded spring contribution.
- Dense's published root has an additional -4.656645378627e-6 relative stored-system solve error. Combined with assembly loss this yields the observed -2.957918027219e-6 against the analytical answer.
- The source coefficient's ordinary binary64 representation is distinct from the much larger amplification of rounding in the soft spring. Neither a faithful stored-system solve nor a tiny backward residual establishes the source-intended answer.

`structural.rs:893–940` only blocks nonfinite/amplification>=1, then accepts passing original and contribution-action residuals and classifies rcond<sqrt(EPSILON) as Sensitive. These normwise operational tests are insufficient for this 1e-9 forward-quantity obligation. No new threshold or inverse certificate is proposed here.

## Bounded remedy direction

The parent's announced shared admission containment—exclude existing Sensitive state from qualified numerical use while retaining inspection/status—is consistent with the measured gap. It prevents warning-only promotion, but **does not complete N05/N06 recovery or the numerical programme**. Do not alter the oracle, criterion, spring, fixture ID handling or physical-mechanism classification.

For recovery, use complete source contributions rather than their rounded coalesced diagonal, retain a private source-bound stronger response, and qualify each published response/recovered quantity against the accepted accuracy requirement. The existing `structural/exact_boundary.rs` offers a bounded candidate seam: complete represented contribution/load ownership, exact positive free blocks of order <=2, exact rational response/reactions, and error-bounded projection. Default helper budgets are 256 total DOFs, 16,384 source terms, 256 expansion terms, and 2,000,000 charged operations; every free connected block must still have order <=2. N05's uncoupled free block pattern fits that algebraic scope. This is a design lead only; no new no-gap product integration was tested here, and represented source arithmetic is not automatically primitive-engineering exactness. Unsupported block/source/range/budget cases must remain explicit.

Do not recover member torque from only the projected binary64 rotations. The tiny twist is about 4.63e-15 rad between rotations near 1e-4; subtraction/recovery can lose the required digits even after displacement repair. Carry the stronger internal response or precise relative deformation through member/spring recovery, then project the final quantities. `Context::new`/`Context::solve`, private `Response`, and `project_displacement`/`project_reaction` are the existing helper seams. `Response::reaction` is the total original-equation action at a DOF; it is zero on free root RX after equilibrium and therefore is **not** the separate ground-spring action. The repair needs a source-bound exact per-spring/member linear-functional recovery and projection API, with its own retained method/source receipt; existing reaction projection alone does not supply this. For N06 a valid stronger source proof must distinguish the positive intended system from singular rounded assembly; rejection must not be silently bypassed by generic LU.

Required regression evidence: unchanged actual N05/N06 product observers in both modes; source-intended root/tip rotations and signed spring/member actions at relative 1e-9; retained source and stronger-method/projection receipts; generic NP-A ULP family kept separately; mutation, unsupported/range/budget rejection, existing structural controls and actual consumer/native bindings under parent lane ownership. No scope-wide mandatory comparison-inverse certificate is reintroduced.

## Prior status and provenance

The earlier `NUMERICAL_INTEGRITY/KERNEL/RETURN.md` already says NP-A/N05 intended accuracy is not recovered and partial rounding may pass screening. `KERNEL_REVIEW/BACKCHECK/RETURN.md:60` expressly retains that requirement and rejects warning-only acceptance. `REFERENCE_REVIEW/RETURN.md` distinguishes NP-A coefficient identity and requires contribution-preserving/reformulated/higher-precision recovery. This diagnosis adds measured actual product and admission evidence to that known obligation; it supplies no new acceptance or release authority.

TASK `/root/numerical_resume/n05_admission`, parent `/root/numerical_resume`, harness-native execution, no descendants. Only this `RECOVERY_DIAGNOSIS/` evidence directory was written. Root/Piping/LOOP_INIT/TASK and diagnosis skill origins/hashes, input hashes, actual commands, lane release, source-freeze limits and replay checks are in [PROVENANCE.json](PROVENANCE.json). No Cargo/build/browser/native run, source edit, original-record edit, or Git mutation occurred. Rerun `audit.py --frozen`, `audit.mjs --frozen`, and `decompose.py` from the recorded checkout root to preserve the pre-containment comparison; rerunning without `--frozen` intentionally observes current admission sources and should use a separate result destination.

## Post-containment saved-carrier backcheck

After the parent announced source-gates-ready for freeze03, the same unchanged real carriers and authored inputs were passed again to current actual TS/Python functions. Both commands exited 0. [POST_CONTAINMENT_TS.json](POST_CONTAINMENT_TS.json) now rejects N05 in both modes with `needs_recompute`, `eligible=false`, `NUMERICAL_INTEGRITY_NOT_QUALIFIED` and `NUMERICAL_CASE_EVIDENCE_INCOMPLETE`. [POST_CONTAINMENT_PYTHON.json](POST_CONTAINMENT_PYTHON.json) returns `needs_recompute` for both. N06 remains rejected in both languages.

This closes the **observed direct TS/Python numerical eligibility bypass** for these authentic carriers. It does not demonstrate numerical recovery, complete Rust/headless route coverage, actual UI Current, complete exports, or native behavior; parent owns those checks. Raw outputs, frozen references, original prechecks and original source snapshots were verified unchanged. Directly invoked post-containment source bytes/hashes and the parent's candidate identity are in [POST_CONTAINMENT_PROVENANCE.json](POST_CONTAINMENT_PROVENANCE.json). The original pre-containment probes remain intact.
