# Numerical health indicator

This design gives each desktop result a plain numerical-health indicator, built from diagnostics the engine already computes for an ordinary solve. Paths are relative to `projects/chirality-piping/`. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Basis

**Owner, 2026-10-23,** accepting Agent 0's recommendation: "I agree with your recommendation. Proceed accordingly." The recommendation said:

- **No verified bounds in v0.1.** The retained-precision method stays dormant and owner-held: B8 is not activated, and DEL-04-08 AX-004 and PRD §12.8's last bullet stand.
- **A health indicator instead.** This unit becomes a numerical health indicator built from the residual, a conditioning estimate expressed roughly as "digits lost", and the specific cause where known.
- **Plain and not a guarantee.** The indicator covers every model size at small cost and has plain green, red and grey states. It is not a guarantee and must not say "verified", "certified" or "bound".

The indicator serves PRD FR-SOL-015: "Solver reports singularities, ill-conditioning, residuals, convergence issues, and invalid elements." It is a results-viewer presentation (DEL-07-05) of solver diagnostics. DEL-04-01's kernel and DEL-04-06's diagnostics produce them, and DEL-04-07 publishes them.

**Revisions 2 and 3** answer the dual reviews of `b71c3a23ad` and `e06b2f26f5`, by an independent Claude reviewer and Codex.

## 1. What the engine and desktop already provide

### 1.1 Per load case, from the ordinary solve

**The verdict.** It is published in `numerical_quality.cases[].solve_quality`, with `structural_status`, and is derived from the case's integrity diagnostic code (`core/product_physics/src/lib.rs` `append_integrity_report`, `integrity_failure_code`, `assessed_numerical_quality`):

| `solve_quality` | Codes |
|---|---|
| `checks_passed` | `NUMERICAL_INTEGRITY_CHECKS_PASSED` |
| `sensitive` | `NUMERICAL_INTEGRITY_SENSITIVE`, including formation-guard demotions, which change the code but not the report's Debug `quality` |
| `unresolved` | `NUMERICAL_INTEGRITY_UNRESOLVED` and `_ASSEMBLY_UNRESOLVED` (covering a zero diagonal, an rcond at the working-precision boundary, the pivot screen, assembly amplification and the residual after refinement), and the nonlinear `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED` (solved inspection results whose recovery qualification is unresolved) |
| `failed` | `NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM`, `_NEGATIVE_ENERGY`, `_FAILED` |
| `not_assessed` | — |

`numerical_quality.status` is the worst case in the engine's order: passed < sensitive < not_assessed < unresolved < failed.

**The kernel's thresholds.** They are policy `M03-INTEGRITY-v1` (`core/solver/frame_kernel/src/structural.rs` `finish_checked_factor`; DEL-04-01 Design `kd5-member-relative-bound.md`). A case is `sensitive` in either situation:

- the reciprocal condition estimate rcond is below √ε (about 1.5·10⁻⁸);
- a load-fidelity loss, a K-D5 demotion or an S11-G demotion applies.

A case whose rcond is at or below ε is refused as unresolved.

**rcond.** For a solved case, rcond is published only as text. It sits inside the integrity diagnostic's message, in the `Debug` form of `StructuralReport`: `… reciprocal_condition_estimate: 1.0578136350861028e-11, residual_rows: […] …`. Its properties:

- It is a Hager–Higham estimate of the 1-norm condition of the power-of-two diagonally scaled matrix.
- It approximates the condition and can understate it. It is not a bound.
- Failed and refused cases carry no report.
- In the committed fixtures, every passing and sensitive ordinary case carries exactly one rcond. The four unresolved source-recovery cases carry none.

**Located causes**, already named:

- **In message text:**
  - the K-D5 line `formation_check: reason=…; row={node}:{DOF}`;
  - S11-G's named rows (at most 6);
  - a mechanism's direction;
  - DOF indices.
- **In `affected_refs`:**
  - `LOAD_CONTRIBUTION_ABSORBED` (its ledger sources);
  - `SOLVER_SYSTEM_BLOCKED` (`supports`);
  - `CURVED_BEND_SHORT_RELATIVE_TO_NEIGHBOURS`;
  - node and pipe input errors.

**Not computed:** stiffness contrast between straight members, rigid-element stiffness and near-duplicate nodes.

### 1.2 Standing and outputs (unchanged by this design)

Standing differs by route:

- **Generic route** (`preview-physics-2`, `pressure-2`). The Rust reader's `ordinary_findings` refuses any non-`checks_passed` case (`NUMERICAL_INTEGRITY_NOT_QUALIFIED`, with `NUMERICAL_CASE_EVIDENCE_INCOMPLETE`; `core/reporting/result_export/src/reader_verdict.rs:469`, `:515-543`). Such a result is not Current, so its outputs refuse.
- **`pressure-source-1` route.** Standing comes from the validated recovery evidence, not from the ordinary verdict (`reader_verdict.rs:419-454`). A result whose ordinary case is `sensitive` or `unresolved` can be numerically eligible and exportable. For example, `fixtures/product_preview/pressure_source/physics_source/n05-dense_scrutiny.raw.json` and `n06-…` are eligible (`exactOutputTwins.test.tsx:216`).
- **Retained successors.** These are dormant; no product caller delivers one.

**Today's desktop presentation:**

- **Results panel** (`ResultsPanel.tsx:110`): "Numerical integrity: {status}. … Numerical checks do not establish engineering correctness."
- **Solve panel:** a diagnostics count tile.
- **Diagnostics panel:** the full list. Selection resolves direct entity ids only (`resultInterpretation.ts:125-134`).
- **Shell chip and rail caption** (`shellLayout.ts:305`, `:314`, `:396`, `:434`): "Needs recompute" whenever there is no qualified Current result. For a generic-route result held back only by its numerical verdict, this misleads, because recomputing the unchanged model gives the same verdict.

## 2. Decisions

1. **It reports the ordinary solve's health. It does not decide outputs.**
   - The indicator states what the ordinary solve's numerical checks found, per load case.
   - Output availability and Current standing stay exactly as the Rust reader and output policy decide them today.
   - The indicator reports availability only by quoting that standing. It never infers it from the verdict.
2. **No new threshold.**
   - States follow `solve_quality` under the existing `M03-INTEGRITY-v1` thresholds.
   - DEL-04-06's condition-ratio thresholds (`classify_condition_ratio`, used today only by the performance harness) stay `TBD` (DEL-04-06 CLM-029, "a governed policy decision"). This indicator neither sets them nor closes that item.
   - The conditioning figure is informational and drives no state.
   - This is recorded as within agent authority, because it presents existing governed policy.
3. **The engine and the published format are unchanged.**
   - The conditioning figure is read from the integrity message by one Rust function in the wasm exports crate, `core/model_operations/operation_applier` (`numerical_health_json`). That crate already hosts the Rust reader's wasm wrappers and is outside the retained-pricing sources (`tools/retained_pricing/base/crate_dirs.txt`), so neither D3 nor AX-001 is engaged. TypeScript only renders.
   - Only rcond is read. The parse contract is §2.6.
   - The alternative was a structured `conditioning` field in `numerical_quality.cases[]`. It was not taken for v0.1, for three reasons:
     - it changes the persisted result schemas, including a reviewed input of the dormant retained profile;
     - it changes priced `product_physics` source, which requires D3;
     - it regenerates every committed result fixture.

     It is the follow-on if the text seam proves brittle.
4. **The conditioning figure** is d = −log₁₀ rcond, printed to one decimal and only in the detail text (§3). It is never in a headline, chip or export. The wording names it as an estimate for the scaled linear solve and gives the solver's own reference point: the sensitivity line √ε ≈ 1.5·10⁻⁸, which is d ≈ 7.8.
   - It is shown for linear cases only. In v0.1 nonlinear cases show no figure.
   - **The cause is decided on the parsed value.** The conditioning cause applies only when the parsed rcond satisfies the kernel's own comparison, `rcond < f64::EPSILON.sqrt()` (`structural.rs:1959`), evaluated in Rust. Rounding is for display only.
   - **Display.** d is printed to one decimal, and to two decimals when 7.7 ≤ d ≤ 7.9, so that values near the line are not shown as equal to it; the state, not the printed figure, is what tells the two sides apart. When rcond = 1, d is printed as "below 0.1".
5. **Combinations** are formed by result algebra, not a solve, so they inherit health from their load cases and say so (§3). A withheld combination keeps its existing finding and gets no badge of its own. Header counts are of load cases, each counted once.
6. **Stiffness contrast is not added in v0.1.** The engine does not compute it for straight members, and a desktop computation would re-derive section stiffness outside the engine. The causes shown are those the existing diagnostics name, including `CURVED_BEND_SHORT_RELATIVE_TO_NEIGHBOURS` where it is present.

### 2.6 The parse contract

`numerical_health_json(envelope)` returns, per load case:

- the state (from `solve_quality`);
- its code;
- the optional d, with a Rust-computed `conditioning_sensitive` flag (the kernel's comparison);
- located causes (§3);
- the integrity diagnostic id, when one exists.

It also returns one result-level flag, `verdict_only`. The flag is true only when all of these hold:

- the result is on a generic route (`preview-physics-2` or `pressure-2`); on every other route the flag is false;
- every `numerical_quality.cases[]` entry has non-empty `evidence_refs`, each naming an emitted diagnostic;
- the case ids are unique, every requested load case is covered, and every emitted diagnostic id is unique;
- `numerical_quality.status` equals the cases' aggregate;
- each case's verdict-linked fields (`structural_status`, `model_matrix_fidelity`, `accuracy_evidence`) equal exactly the values `assessed_numerical_quality` derives from its `solve_quality` and integrity code;
- the reader's findings for the result, passed in, are exactly the pair `NUMERICAL_INTEGRITY_NOT_QUALIFIED` and `NUMERICAL_CASE_EVIDENCE_INCOMPLETE`.

`verdict_only` is a presentation projection computed in Rust. It never takes part in admission or standing. A maintained test ties it to the reader: for every corpus input with `verdict_only = true`, the reader's findings are exactly `NUMERICAL_INTEGRITY_NOT_QUALIFIED` and `NUMERICAL_CASE_EVIDENCE_INCOMPLETE`. A non-passing case, or a passing companion case, whose `evidence_refs` is emptied or renamed gives `verdict_only = false`, as does a duplicated diagnostic id, as do a pressure-source input and a retained input with consistent `numerical_quality`.

The figure is returned only when **all** of these hold:

- exactly one diagnostic has id `diagnostic:numerical-integrity:{case}` and `affected_refs` equal to `[case]`;
- its code is `_CHECKS_PASSED` or `_SENSITIVE`;
- its message begins with the exact lead `M03-INTEGRITY-v1 represented original-equation structural evidence for load case {case}: StructuralReport { policy: "M03-INTEGRITY-v1", quality: `, built from the known case id;
- within the report span (up to `; global_dof_map=`), the text `, reciprocal_condition_estimate: ` occurs exactly once, followed by a number token and then `, residual_rows: `;
- the token parses (Rust's shortest round-trip `f64` form) to a finite value with 0 < rcond ≤ 1.

Otherwise the case keeps its state and has no figure. A missing figure never changes a state.

**Grey** is reserved for cases that genuinely lack an assessment: `not_assessed`, or no `numerical_quality` entry.

## 3. States and wording

The label is **"Numerical health"**. Each load case has one state:

| State | `solve_quality` | Badge | Headline |
|---|---|---|---|
| checks passed | `checks_passed` | green | "Numerical health: checks passed" |
| sensitive | `sensitive` | red | "Numerical health: sensitive" |
| quality unresolved | `unresolved` | red | "Numerical health: quality unresolved" |
| solve failed | `failed` | red | "Numerical health: solve failed" |
| not assessed | `not_assessed` or absent | grey | "Numerical health: not assessed" |

On the `pressure-source-1` route the headline is prefixed "Ordinary solve —", so that it is not read as the recovered result's standing.

- **Where the reader's standing is numerically eligible** (the recovery passed), the detail adds: "The values shown come from the producer's recovery of this case, which passed its own checks. This badge describes the ordinary solve only."
- **No row badge on recovered rows.** The badge is not placed on rows whose values are recovered; it stays on the header and the per-case detail.
- The existing recovery-evidence text stays.

**Result header and shell chip.**

- The badge is red if any load case is red. Otherwise it is grey if any is grey, and green only if all pass.
- The headline counts states: "Numerical health: {n} sensitive, {m} checks passed", listing non-zero counts in the order failed, unresolved, sensitive, not assessed, checks passed. When every case passes it reads "Numerical health: checks passed".

**Replacing "Needs recompute".** The chip and the rail caption show the health header instead of "Needs recompute" when the result is held back only by its numerical verdict. That means all of these hold:

- every independent Current requirement in `resultsSessionState.ts:57-73`, other than numerical eligibility, holds: manifest, run, producer, native invocation and record binding. This uses one shared session predicate, factored out of today's conjunction;
- `numerical_health_json`'s `verdict_only` is true (§2.6);
- some load case is red or grey.

Missing or malformed evidence findings keep "Needs recompute". Admission is unchanged.

**Output sentence.** When the generic-route result is not Current because of the verdict, the header adds: "No results of this run can be exported or used by rule checks while {n} load case(s) are not checks passed." Where standing comes from recovery evidence, no output sentence is added. That route's existing text states its standing.

**Detail texts.** All literals are in `apps/desktop/src/features/results/numericalHealthText.ts`. Values in braces are filled in.

- **checks passed:** "The solver's residual and conditioning checks passed for {case}. These checks look for numerical trouble; they do not measure the error, and they say nothing about whether the model, its inputs or its design are right."
- **sensitive:** "The solver's checks flagged {case} as numerically sensitive." It adds "Its values are kept for inspection." only where the result's standing is not numerically eligible. Then the cause sentence for each reason present, in this order:
  - **conditioning** (`conditioning_sensitive`): "The scaled stiffness is poorly conditioned."
  - **load fidelity** (`LOAD_CONTRIBUTION_ABSORBED` present): "The solver could not confirm that every applied load was carried at full precision ({sources})."
  - **K-D5 demotion, or an S11-G load-row demotion:** "The solver's formation check could not confirm some stiffness or load terms at full precision ({rows})."
  - **S11-G recovery demotion:** "The solver could not confirm some recovered member or support quantities at full precision ({groups})." The groups are listed as text.

  If no reason is identifiable: "See the solver diagnostic for the reason." It ends with: "Solving the unchanged model again in the same solver mode gives the same result. A very short or very stiff element next to flexible ones, stiffnesses that differ by many orders of magnitude, or a weakly restrained direction are common causes."
- **quality unresolved:** "The solver could not confirm the numerical quality of {case} at working precision. See the solver diagnostic for the reason. A near-mechanism or extreme stiffness contrast are common causes."
  - For the nonlinear `RECOVERY_BASIS_UNQUALIFIED`: "Its support-state solution is kept for inspection, but its numerical quality could not be confirmed."
- **solve failed:** "The solver could not complete a solution for {case}." Then:
  - mechanism: "The model can move without resistance (a mechanism). Review connectivity, member releases and supports."
  - negative energy: "The assembled stiffness is not positive in some direction. Review element and support properties."
  - otherwise: "See the solver diagnostic for the reason."
- **not assessed:** "The solver did not assess the numerical quality of {case}."
- **Combinations:** "Inherited from its load cases: {case}: {state}; …". The combination takes the worst of its load cases' badges. A combination that names a case missing from `numerical_quality` reads "not assessed".
- **Conditioning detail** (checks passed and sensitive, when d is present): "Estimated sensitivity of the scaled linear solve: rounding can be magnified by roughly {d} of the about 16 decimal digits the computer carries (the solver marks a case sensitive at about 7.8). This is an estimate; it does not estimate the error in individual results."

**Located causes and navigation.**

- Each named location is listed and selectable: a K-D5 or S11-G row label `{node}:{DOF}`, a load-fidelity source, or a curved-bend reference. A row label is shown as "node {id}, {direction}" and resolves to its node, through a new mapping in `resultInterpretation.ts` that keeps the existing direct-id path.
- DOF indices and mechanism directions are not mapped in v0.1.
- "Show solver diagnostic" appears only when an integrity diagnostic exists. It opens the Diagnostics panel filtered to that diagnostic.

**Vocabulary.** The texts never use "verified", "certified", "bound", "guaranteed", "valid", "approved", "compliant", "correct" or "reliable". A focused test pins the module's literals against that list, and GEN-13 scans it.

## 4. Placement

- **Results panel.** The health header is a separate element placed above the `numerical-result-standing` line, on the generic and pressure-source routes. That line keeps its text byte for byte, because the native smoke's `expected.json` pins its prefix from the G10 checklist and G10's interpretation is owner-held. The order is header line, then the per-case detail (collapsed when every case passes), then the standing line. The standing line carries a long limitations text, and the detail must not sit below it. Each load case and combination row carries its badge, except recovered rows on the pressure-source route (§3). The retained and unsupported texts are unchanged.
- **Shell chip and rail caption.** They show the replacement described in §3. The rail uses the short form "Numerical health: {worst state}".
- **Solve panel.** The diagnostics tile gains the header badge.
- **Diagnostics panel.** It is unchanged, apart from the filtered open.

## 5. Exports and reports

- **One derived presentation object** (the parse output plus scoped state and texts) feeds every surface. Admission and refusals are unchanged.
- **Result JSON and stress-neutral exports** are unchanged. They already carry `numerical_quality` and the integrity diagnostics verbatim.
- **The report and rendered report** gain one line per load case before the diagnostics section: "Numerical health of the ordinary solve: {state}." They admit only Current generic-route results, so in v0.1 the line reads "checks passed". The conditioning figure is not exported.
- **Exact-route reports, handoffs and the report package** keep their existing refusals. Since the owner opened the rendered report and the report package for pressure-2 and pressure-source-1 (W-11, 2026-10-26), an exact report carries no numerical-health line (DEL-08-01 Design `exact-route-report-sections.md` §6.1); handoffs and the report JSON packet keep their refusals.

## 6. Claims registry (owner text)

This is presented to the owner as a reserved choice, with the exact text sent separately. The registry gains `NQ-HEALTH`:

- **Display forms:**
  - "Numerical health: checks passed", "…: sensitive", "…: quality unresolved", "…: solve failed" and "…: not assessed";
  - the "Ordinary solve —" prefix;
  - the header's count form.
- **Boundary sentence:** "These checks look for numerical trouble; they do not measure the error, and they say nothing about whether the model, its inputs or its design are right."

§2.1 rule 3 is amended to say that these forms are numerical-quality indicators, not analysis statuses or evidence-status labels.

**If the owner declines,** the indicator ships without a registry entry. Its headline then reuses the existing token text, "Numerical integrity: {status}", and only the detail texts and placement change.

No ScopeOfWork or PRD text changes. The indicator implements PRD FR-SOL-015 and SPEC §5.5 within DEL-07-05's results-viewer contract.

## 7. Verification

- **Rust parse tests** (`operation_applier`):
  - **Unit tests.** Every parse-contract clause, plus spoofing: a case id containing the lead, the field name in a case id, `intended_residual_rows`, two diagnostics with the same id, and the Debug `quality: Passed` under code `_SENSITIVE`.
  - **Live producer tests.** These run the product entry (`run_linear_static_preview_value_with_mode`) in both solver modes over committed requests: linear passing, sensitive, a formation demotion, a source-selected ordinary failure, several cases, and a nonlinear normal publication with recovery-basis refusal.
    - They assert that every linear passing or sensitive case yields exactly one finite figure, and that no case yields an ambiguous one.
    - They assert that the state equals `solve_quality`.
    - Where the kernel's report is reachable through a public API, the parsed rcond equals it bit for bit.
  - **A fixture-wide test** checks every committed fixture.
- **Desktop tests:**
  - states, header counts and combination inheritance;
  - the chip and rail predicate on the generic, pressure-source and retained routes, including that missing evidence keeps "Needs recompute";
  - the pressure-source eligible wording, with no row badges on recovered rows;
  - the output sentence only under the verdict-only condition;
  - text and vocabulary pins, and GEN-13;
  - located-cause navigation;
  - the report line on the emitted document.

  Then the full `desktop-test` with the wasm engine rebuilt, compared with main.
- **Python.** The full registered `piping-pytest` with the binary presets for the head, compared with main.
- **Real desktop.** Screenshots in the actual app, recorded in the pull request, of three results:
  - a green result (the milestone model);
  - a red generic-route result (a sensitive invented fixture);
  - an eligible pressure-source result whose ordinary case is sensitive (`n05`). The open path is unchanged: no new job and no new solve.
- **Review.** Dual review on the final head (independent Claude and Codex), because the conditioning text carries numerical meaning.

## 8. Unchanged

- The engine, the persisted result formats, every threshold, every reader verdict and all output admission.
- The retained-precision method (dormant, owner-held).
- G10 and the frozen UI benchmark.
