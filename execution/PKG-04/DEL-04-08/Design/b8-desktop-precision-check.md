# B8: the desktop precision check

This design covers B8, which was activation of the retained-precision method on the desktop. Paths are relative to `projects/chirality-piping/`. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Rulings and status

**Owner, 2026-10-23:** "R1: change the semantics so users are confused, the concept is worth doing now. Don't study it that won't tell me anything new that your theory doesn't already make clear. It's precisely the rare cases that really matter and you might not find one or take forever to find one, so just implement it." Corrected the same day: "I should have said \"so users aren't confused\"".

The owner accepted Agent 0's framing:

- **The common case.** In a well-conditioned model a verified bound is extremely tight. Its value is a near-always-green numerical health check: "these results are numerically sound to at least N significant figures", backed by a guarantee.
- **The rare case.** A wide or unestablished bound strongly signals that the ordinary numbers should not be relied on, often because of a modelling problem.
- **Presentation.** The check is shown as a per-result indicator with a clear warning on failure, not as ± ranges on every number. "Verified" never reads as engineering, code or professional verification (PRD §12.8, §21.2).

**Agent 0, 2026-10-23:**

- "Proceed with the caller-selected trigger as recommended".
- The indicator, its states (including grey "not available for this model (reason)") and the surfaces must not depend on which method produces the bound.
- Domain widening is Decision 22 and is not pursued.
- Later the same day: the owner is reconsidering whether verified arithmetic bounds justify their cost. Until that ruling, only the method-independent parts proceed, as documents. There is no implementation, D3, memory qualification or exclusive run. If the ruling goes toward a lighter "numerical health" indicator built from the existing residual and conditioning diagnostics, the state and wording design carries over.

**What is active and what is held:**

- **Part A is active.** It covers the method-independent indicator: the flow, the adapter contract, the states, the wording, the surfaces and the exports.
- **Part B is held.** It covers the retained-precision adapter: the trigger, typed outcomes, the bound arithmetic and qualification of the product build. It records where the two dual-review rounds left it.

**Review history:**

- Revision 1 (`bd58241f14`) and revision 2 (`24235063b5`) were each reviewed by an independent Claude reviewer and by Codex. Both rounds failed.
- Revision 3 (this file) answers both rounds.
- Part A's answers to the second round are §A3's figure, which is measured per kind; §A4's state for a failed ordinary solve; and §A5's units.

---

# Part A — the method-independent precision check

## A1. Premises

- **The ordinary result stays current.** It is shown at once. Every output surface, rule binding and export reads it exactly as today. The precision check attaches a statement about it, and only an explicit user act replaces it (§A6).
- **Most models are outside the retained method's reach.**
  - The D1 domain: at most 32 nodes, 32 members, 32 supports and 4 materials; 3 load cases of up to 128 loads each; 2 combinations.
  - Route E further requires nodal loads only, empty pressure regions and no combinations.
  - Route L requires a pressure-free model.

  So the honest common state for a production model under that method is grey "not available", with its reason. A lighter diagnostics adapter (§A8) would reach every solved model.
- **Wording.** The vocabulary is "confirmed" and "established" (nonlinear K-D5 ruling R5). "Certified", "verified", "valid", "approved", "compliant" and "correct" are never used in the indicator.

## A2. Flow

1. **An ordinary solve completes.** Its result becomes current. For each load case or combination ("result set"), the desktop asks the active adapter for a statement.
2. **A slow adapter runs in the background.** The retained method is one. Such an adapter:
   - reuses the solve's captured invocation;
   - carries the solve's generation token (session, solve job and capture);
   - is cancelled by a new solve, a model edit, a mode change or a session reset.

   A result arriving under a superseded generation is discarded. At most one check is queued or running, outside the ordinary queue's limit.
3. **Declines are cheap.** An adapter that can decline before doing work (domain, build or budget) does so without solving.
4. **Fast adapters answer immediately.** Such an adapter (§A8) reads only the current result.
5. **Saved results.** A result that is saved and reopened, or whose invocation is no longer live, reads "not current". No statement is formed from saved bytes as if it were live.

## A3. The adapter contract and the figure

An adapter returns, for each result set, either a bound statement or a typed non-bound state (§A4).

**A bound statement** gives, for each row:

- `E_SI`: an upper bound on |solver value − exact value of the adapter's stated problem|, in SI, rounded upward; or "no bound".
- The row's SI quantity kind: translation, rotation, force, moment or stress.
- The problem phrase. For a retained-precision result, it is "the linear problem formed from this model's recorded inputs". For retained-source recovery it is "the represented source problem" (PRD §12.8).

The adapter supplies E in SI. Bounds published in a row unit are normalized outward by the adapter: retained-source bounds, for example, are in mm for translations. Values are compared in SI, normalized exactly as the Rust reader normalizes them.

**The figure N is measured per kind, not per row.** A per-row relative measure would let small components (a minor transverse displacement, a moment near an inflection point) set the figure. It would show red on healthy models.

For each kind k present in the result set:

- `scale_k` = max over the kind's rows of |solver value|, in SI;
- `ratio_k` = max over the kind's bounded rows of `E_SI / scale_k`.

N is the largest integer n ≤ 9 such that `ratio_k ≤ 10⁻ⁿ` for every kind. It is decided by exact comparison of each `E_SI` against `scale_k · 10⁻ⁿ`, using an upward-rounded decimal constant for 10⁻ⁿ and directed rounding that is valid for subnormals. Floating-point logarithms are never used. The figure is never better than the guarantee.

A well-conditioned case under a 10⁻⁹ relative guarantee therefore reads 8. The extra unit of rounding margin makes the strict 9 unprovable. The copy promises only what N says.

Degenerate cases:

- **A kind with `scale_k` = 0 and every E = 0** (all values exactly zero by the guarantee) adds no constraint.
- **A kind with `scale_k` = 0 and some E > 0** is stated per row as "within {E} {unit}", and is excluded from N.
- **A result set with no bounded row** has no figure; its state is "no bound established".

**Where the figure is computed.** It is a reader statement. It is computed in Rust by the product reader (native and wasm) from validated inputs, with parity in the test-only Python verifier (AX-001, REQ-005). The desktop renders it. TypeScript computes no bound.

**Units on display.** Displayed and exported bounds are converted from SI to the row's display unit outward, with directed rounding, and printed rounded up.

## A4. States and exact wording

The label is **"Precision check"**. Each result set shows one state.

| State | Badge | Headline |
|---|---|---|
| running | neutral | "Precision check: running" |
| confirmed (N ≥ 6) | green | "Precision check: {N} significant figures confirmed" — or, when some values have no bound, "Precision check: {N} significant figures confirmed for {k} of {m} values" |
| low precision (N < 6) | red | "Precision check: only {N} significant figures confirmed" |
| not confirmed | red | "Precision check: not confirmed" |
| no bound established | grey | "Precision check: no bound established" |
| recovered | amber | "Precision check: values available from the check" |
| not available | grey | "Precision check: not available" |
| did not complete | grey | "Precision check: did not complete" |
| not current | grey | "Precision check: not current" |

The threshold of 6 is a proposed product value; it is part of the owner text.

**Header aggregation.** The result header takes the first that applies:

1. red, if any set is red;
2. amber, if any set is recovered;
3. running, if a check is running;
4. "Precision check: at least {min N} significant figures confirmed", if every set is confirmed;
5. "Precision check: {k} of {n} results confirmed", if some sets are confirmed;
6. otherwise the first applicable of did not complete, no bound established, not available and not current.

The shell chip carries the header's badge.

**Detail texts.** Values in braces are filled in.

- **running:** "The results shown are from the ordinary solve. A second, independent numerical check is running in the background."
- **confirmed:** "The solver's values (as stored and exported) agree with the exact solution of {problem} to at least {N} significant figures, measured against the largest value of each kind (displacement, rotation, force, moment, stress). This checks the solver's arithmetic only. It says nothing about whether the model, its inputs or its design are right." It adds:
  - "{n} values have no bound from this check and are marked "no bound"."
  - "{disclosure}", the adapter's disclosed difference between its problem and the authored inputs. For example, for uniform member loads: "Uniform member loads enter this problem as declared equivalent nodal loads; that small difference from the authored loads is disclosed, not bounded."
- **low precision:** "The solver's values agree with the exact solution of {problem} to only {N} significant figures, measured against the largest value of each kind. Do not rely on them beyond that precision. This often points to the model: a mechanism or weakly restrained direction, a very short or very stiff element next to flexible ones, or stiffnesses that differ by many orders of magnitude." When confirmed values exist and can be adopted (§A6), it adds: "The check's own solution is confirmed to {N'} significant figures."
- **not confirmed:** "The check found evidence that this problem is close to singular, and could not establish a bound. Do not rely on these values until the cause is understood. This often points to the model: a mechanism or weakly restrained direction, a very short or very stiff element next to flexible ones, or stiffnesses that differ by many orders of magnitude. Review the model and the solver diagnostics, then solve again."
- **no bound established:** "The check ran but could not state a bound for these values within its fixed limits. That alone does not mean the values are wrong, and it does not confirm them."
- **recovered:** "The ordinary solve did not produce values for this case, but the check produced values confirmed to {N'} significant figures. They are not used unless you choose Use the confirmed values."
- **not available** gives a reason sentence:
  - size: "This model has {observed} {fact}; the precision check supports models with at most {cap}."
  - feature: "This model uses {feature}, which the precision check does not support."
  - build: "The precision check is not set up for this build of the application."
  - budget: "The per-solve memory budget is below the {required} the precision check needs."
  - another case failed: "The ordinary solve failed for {case}; the precision check runs only when every load case solves."
  - combination: "This combination is outside the range the precision check supports ({reason})."
  - not covered by the method that ran: "The check that ran does not cover this case."
  - otherwise: "This model is outside the range the precision check supports."
- **did not complete:** "The check stopped before reaching a conclusion ({cause}). The ordinary results are unchanged." The causes are "not enough free memory at the moment", "it was cancelled" and "an internal check failed". The first two add "Try again with Check precision."
- **not current:** "This result was not checked in this session, or the model has changed since. Solve again to check it."

**Rules for red.** A red state needs evidence that the adapter itself classifies as conditioning or mechanism evidence, or a confirmed figure below the threshold. Any ambiguous cause is grey "no bound established", never red. This covers method limits, publication-enclosure failures, budget or work exhaustion, and an exhausted search whose underlying reasons are not conditioning evidence. Internal and resource failures are "did not complete".

**Rows.** A row without a bound is marked "no bound". A zero-scale row with a bound shows "within {E} {unit}". No ± range is shown on any other row.

**Actions.**

- "Check precision" retries after transient causes only.
- "Cancel check" stops a running check.
- "Use the confirmed values" is offered only as §A6 allows.

**Existing lines.** The precision check is shown first. The existing standing lines ("Numerical integrity: {status}", the reader standing) keep their wording. Under a red or grey state they are not repeated as reassurance.

**One home for the texts.** Every literal is in `apps/desktop/src/features/results/precisionCheckText.ts`. GEN-13 scans it, and `tests/test_claims_language_surface.py` pins it. The registry entry is `NQ-PRECISION` (§A7).

## A5. Output surfaces and exports

**While the ordinary result is current** (the normal case), every surface admits exactly what it admits today. Documents that carry result values gain a precision-check section: handoff, local-fea, native-package, report, rendered-report, headless-runner, and the result JSON and stress-neutral exports. The section contains:

- the per-set headline;
- the boundary sentence, for confirmed and low-precision states;
- the reliance warning, for red states;
- the adapter's disclosure;
- a keyed list of the document's emitted rows that have no bound;
- the per-row "within {E} {unit}" of zero-scale rows.

Where a document has per-row structure (report and rendered-report rows), the mark goes on the row. Rows the document does not emit are not listed. Exporting a result that is not confirmed is never blocked; the section states its state. Tests check the emitted artifacts.

**The check's own publication** can be exported as "Precision check evidence (JSON)". It is retained-precision evidence or retained-source evidence, with its receipt.

## A6. Adopting confirmed values (explicit, rare)

"Use the confirmed values" is offered only when all of the following hold:

- the adapter's publication is a complete, numerically eligible result in its own right (for the retained method: the Rust reader's eligible standing with the live capture);
- the current values are confirmed below the threshold, or the ordinary solve produced no values for the case (the recovered state).

**The offer states its consequence:** "The confirmed values will replace the ordinary results. Values without a bound from the check (for example intensified stresses) can then no longer be used by rule checks. Outputs already made keep the ordinary values."

**Adopting** makes the publication current through the existing successor path:

- it is registered against the live capture;
- the analysis record is rebuilt;
- rule-check, proposal and report state is invalidated, and the bundle is published together.

The adopted result's own statement then follows §A3.

**Surface admission of an adopted retained successor.** It is admitted on exactly the surfaces that admit its base identity, at eligible standing with the live capture:

| Retained identity | Base identity | Admission |
|---|---|---|
| `preview-physics-retained-1` | `preview-physics-2` | ungated except report-package |
| `physics-retained-1` | `pressure-2` | result-export, stress-neutral and rule-check |

- Rule checks bind only rows with a verified class (the existing rule).
- The report package refuses every fresh identity.
- `physics-retained-1` joins both fresh-identity sets.
- `LoadReferenceOutputGate` reads standing.
- A refused cell uses the base identity's refusal text, which retires `RETAINED-PRECISION-OUTPUT-NOT-YET-AVAILABLE`.
- Each of the 17 policy cells has its own test.

## A7. Claims registry (owner text)

A new entry, `NQ-PRECISION`, registers the display forms of §A4 and §A8 as the only precision-check forms, together with this boundary sentence:

> "This checks the solver's arithmetic only. It says nothing about whether the model, its inputs or its design are right."

Rule 3 of §2.1 is amended to say that these forms are neither analysis statuses nor evidence-status labels.

## A8. A diagnostics-only adapter (if the owner chooses the lighter indicator)

The existing ordinary diagnostics publish no error bound. For each case they give:

- `solve_quality` (`checks_passed`, `sensitive`, `unresolved`, `failed`);
- the residual checks and the reciprocal condition estimate rcond;
- `accuracy_evidence: not_claimed`.

The kernel marks a case `sensitive` when rcond < √ε (about 1.5·10⁻⁸) or a load-fidelity loss is detected. It marks it `unresolved` when the refined residual check fails.

An adapter built on these can never produce a "confirmed" state, because it has no guarantee. It uses the same label, badges, aggregation, rows, surfaces and exports, with its own three headlines:

| Diagnostic | Badge | Headline | Detail |
|---|---|---|---|
| `checks_passed` | green | "Precision check: no numerical problems detected" | "The solver's residual and conditioning checks passed. These checks look for numerical trouble; they do not bound the error. This says nothing about whether the model, its inputs or its design are right." |
| `sensitive` | amber | "Precision check: sensitive" | "The model's stiffness is poorly conditioned (estimated condition number about {10^k}). The results may have lost about {k} of their roughly 16 significant figures; this is an estimate, not a bound. This often points to the model: …" (the same causes as above) |
| `unresolved`, `failed` | red | "Precision check: not resolved" | "The solver could not confirm that its solution satisfies the equations to working precision. Do not rely on these values until the cause is understood. This often points to the model: …" |
| not assessed | grey | "Precision check: not available" | "The solver did not assess this case's numerical quality." |

Under this adapter nothing runs in the background, adoption never arises, and every solved model has a state. If the owner later wants both indicators, the bounding adapter's statement takes precedence where it exists, and the diagnostic one fills the rest. The detail then names which kind of statement it is.

---

# Part B — the retained-precision adapter (held)

Held by Agent 0 on 2026-10-23, pending the owner's ruling on whether verified arithmetic bounds justify their cost. Nothing here is implemented. This part records the design as it stood after two review rounds, so the work can resume without rediscovery.

## B1. As designed

- **Trigger.** A caller-selected `RetainedTrigger`.
  - `OrdinaryNotPassed` is today's rule.
  - `EveryCase` is the desktop's: every requested case not excluded by DN §4.3.
  - The receipt records it in a required, hashed field, `retained_precision.body.trigger` (`"ordinary_not_checks_passed"` or `"every_case"`).
  - Reader G5's D6b is keyed on that field, and so is the Python verifier. No numerical gate, limit or domain changes.
  - This is sound per both reviewers.
- **Decline.** `retained_direct_admission` runs the imposed-displacement refusal, parse and `admit` with no solve, at L = min(budget, M). It returns a typed `RetainedDecline` (`Build`, `Size`, `Feature(token)`, `Budget`, `Other`). The mapping is total over `AdmissionRefusal`.
- **Outcomes.** A per-case `case_outcomes()` is preserved before `w1_transaction` reduces failures to the furthest stage (`core/product_physics/src/lib.rs:3678-3686`). It feeds one typed cause→state table, used for the out-of-band outcomes and for receipt `unavailable` reasons alike, under §A4's rule for red.
- **The figure.** It follows §A3:
  - for `relative_verified` rows, `E_SI = |o − s| + 10⁻⁹·|s|`;
  - for `absolute_verified` rows, `E_SI = |o − s| + b`;
  - `input_derived` rows are excluded;
  - `not_covered` and `non_quantity` rows have no bound.

  Rows are paired as a bijection on id, kind, unit and `basis_ref`. Both reviewers found the transfer sound in exact arithmetic.
- **TBD-001 and TBD-002** (proposed; reviewed inputs; D3).
  - TBD-001: user-facing texts drop "certified", "certificate" and "certify". `SELECTED_MESSAGE`, `SELECTED_MEMBER_LOAD_MESSAGE` and `COMBINATION_SELECTED_MESSAGE` use "verified classes", "the verified bounds apply to that declared-load problem" and "its own solve and error proof". The member-load limitation reads "Retained-precision bounds apply to that declared-load problem". The `pressure-source-1` maxima limitation reads "do not bound primitive geometry or material accuracy". `formation_warrant` and the formation definitions keep "certificate".
  - TBD-002: the `physics-retained-1` scope sentence reads "model 0.6.0 with the 3.0.0/exact_pressure_v3 contract".
- **Product build.** A second `REGISTERED_PROFILES` entry for the desktop's release identity.
  - It keeps the same M, margins, TEXT reserve and R.
  - The identity is read from the actual Tauri build.
  - The desktop lock is first aligned to the PP lock. `cfg-if`, `libc` and `typenum` differ today.
  - D3 and the in-build checks run in the release identity.
  - Stop condition: if anything does not fit, it goes to the owner as Decision 22. Nothing is changed to make it fit.

## B2. Open findings to resolve before Part B resumes

From the second review round. Codex's verdict on `24235063b5` was fail, and so was the Claude reviewer's.

1. **`Ceiling` is ambiguous** (Codex, blocking).
   - `adaptive.rs:4857-4907` reaches `Ceiling` after publication-predicate rejection, including `AbsoluteBound`, `SharperExact` and `SharperBinary64`.
   - `tests/retained_k4/publication_tests.rs:1190-1227` pins an absolute-bound ceiling.
   - Under §A4's rule, the typed attempt reasons beneath a `Ceiling` must be preserved. Only independently supported conditioning or mechanism evidence is red. That ceiling control joins the state tests.
2. **The release safeguards must be bound per crate** (Codex, blocking; Claude, should-fix).
   - The identity and a single probe cover `product_physics` only. Each protected crate needs a crate-local witness (`cfg!(debug_assertions)` and an overflowing ordinary `+` through `black_box`, caught under unwind), checked in `build_status`. The minimum set is `open_pipe_stress_product_physics`, `_frame_kernel`, `_correct_norm`, `_result_export`, `_units` and `_canonical_json`. The full closure is generated from `cargo metadata` and tested, since Cargo only warns on an unmatched override spec.
   - LTO and codegen-units are not visible to build scripts. Use a maintained manifest test and `CARGO_PROFILE_RELEASE_*` checks.
   - Apply the settings at every evidence build root: standalone crate tests and the wasm reader.
   - The overrides also change the ordinary solver's code and timing. That must be stated, and G10's owner-held interpretation considered.
3. **Failed-ordinary recovery** (Codex, blocking). Pairing by row must not block REQ-003 (d). When ordinary values are absent, validate the successor's captured manifest through G0–G8 and offer §A4's recovered state and §A6's adoption, without manufacturing an ordinary-value figure.
4. **`CaptureError` variants** (Claude). `Source`, `PreparedProof` and `PreparedArithmetic` (`retained_product.rs:168-179`) are domain or proof outcomes, not internal faults. Kernel causes not named in the table (`GeometryUnavailable`, `Structure`, `PublicationCertificate`, `CertifiedBoundUnencodable`) are pinned in the enumeration test.
5. **Normalization of s** (Claude). Show that the producer's SI value is bit-identical to the reader's normalization (`retained_precision.rs:3699-3707`), or add one unit in the last place of s as well.
6. **Test artifact vs shipped binary.** `cargo test` also enables frame_kernel's `exact-operation-count` dev feature. That is a stated residual beside `carry_test_hooks`. The shipped binary is exercised by the native smoke.
7. **The owner bundle** gains, beyond revision 2's list:
   - the figure's definition and threshold (§A3, §A4);
   - the per-adapter problem phrase;
   - §A6's adoption warning;
   - the qualified statement on ordinary-route timing.

The draft owner text against revision 2 is kept unmerged on branch `claude/piping-b8-owner-text-draft` (`7eb80aa953`). It is revised once the owner rules.
