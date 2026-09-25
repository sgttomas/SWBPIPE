# Independent strict-gap nonlinear join review

**Disposition: final candidate is cleared for focused compilation/tests only after the NGR-01 source repair. No further runtime-source blocker was found. NGR-02 remains open and blocks Current/public-method qualification.** This review does not establish that the new nonlinear tests execute or pass, nor qualify the product, physical formulation or general extended source-energy method.

The complete actor delta, ROOT-adopted decimal oracle delta, and exact manager compile repair were reviewed together. Final source identities are:

| File under core/solver/nonlinear_integration/src | SHA-256 |
|---|---|
| lib.rs | a5a3f7104b8a77c9d9709a9d4abd5123e14aedd4fa31f2d3763cc0101677034f |
| structural_adapter.rs | 79272e2f731be9853a087c38d83260b6ba112d6b0a0ed48256a71b40f9bd942f |
| product_equilibrium.rs, unchanged | d18ed7e007e22390105670c89a647c3ab06441b4d27ed685b01e9bba0a8bc53e |

The frame parent bb3923fb and exact-boundary helper 700338b9 are unchanged. [Final candidate](_run_records/FINAL_CANDIDATE.json), [combined-delta verification](_run_records/SOURCE_CHECK.json), [NGR-01 backcheck](_run_records/NGR01_BACKCHECK.json), source snapshots and [scope check](_run_records/scope_check.json) retain the boundaries. The initial reviewed lib.rs hash 4162ebe8 remains preserved; the only later edit removes the invalid test unwrap.

Independent TASK Type2 /root/solver_manager/kernel_review, actual parent /root/solver_manager, delegated-harness-native, same gpt-6-astra xhigh dispatch. Full Root/TASK/Piping/LOOP_INIT and software-code-review bodies were reread. [Origins](_run_records/ORIGINS.json) and [inputs](_run_records/INPUT_HASHES.json) bind actual instruction/source/reference/log origins. No descendants, source/product/frame edit, Git mutation, Cargo, Node, native/browser or Rust test execution. Only NONLINEAR_REVIEW evidence was written. Product source was inspected as a read-only caller dependency and captured at its observed hash, not reviewed as a complete product diff.

## Findings and dispositions

### NGR-01 — P1 test-target compilation defect: closed by source backcheck

Initial lib.rs lines 4645–4647 called unwrap() on NonlinearSupport::one_way. The actual constructor returns Self and has no unwrap method. This prevents the test target from compiling, including filtered test runs.

The manager removed only that unwrap at the declared edit boundary. The preserved repair delta exactly transforms 4162ebe8 into a5a3f710; no method, oracle, numeric criterion or production logic changed. The source defect is closed; successful compilation still requires the parent's actual run.

### NGR-02 — P1 before Current/public qualification: exact recovery basis is discarded

Locations: structural_adapter.rs lines 453–474, 587–603, 692–701 and 738–750; nonlinear lib.rs private completion at lines 345–357 and 750–764. Current caller dependency: product_physics/src/lib.rs lines 2342–2345, 1705–1711 and 2496–2511 in the captured caller snapshot.

The runtime Context and Response own and prove the exact represented equations, ratios and source-bound projections. The adapter copies only decisions, projected values/intervals/bounds, a text source identity, partition and ordinary displacements into ExactGapReport. It retains neither exact numerator/denominator terms nor the exact contribution/source snapshot or positive-minor witness. Context and Response then drop. Private completion retains ordinary input, springs, aggregated K/f and this flattened report, so it can detect public report mutation, but cannot retain evidence that the adapter did not copy.

This is **not a refutation of the runtime signs or qualified projections**. It is a concrete missing retained/replay basis under ROOT's new recovery-evidence requirement. The implementation return's statement that the report separately records exact minors/ratios is not borne out by its fields.

The distinction matters numerically. For g1 equal to the predecessor of 1/8, g2 released and F=25/2, exact u2 is 1/4−2^-56, which publishes as 1/4. The correct contact reaction is −25/2^54. Exact K times the *published* u instead gives −25/2^53; ordinary binary64 evaluation gives approximately −3.5527e-15. Reconstructing the correct reaction from the public displacement is therefore invalid. [Independent control](_run_records/CONTROL_CHECKS.json) preserves all three quantities.

The current product caller accepts the expanded boolean qualifier and publishes the projected nonlinear response, while numerical diagnostics still carry the ordinary M03 structural report plus projected-u equilibrium report. It does not publish an exact-method receipt or the dropped ratio/source evidence. The field value_representation=finite_binary64 remains truthful for public values; it must not be interpreted as binary64 internal solving/recovery. A future receipt declaring recovery from binary64 internal/public displacement for these exact-ratio reactions would be false.

Required repair before Current/public-method qualification:

- Retain an immutable exact source snapshot or resolvable hash-bound source artifact: ordered represented stiffness/force contributions, aggregates, actual partition/prescribed values, source level and same-state support/spring identity.
- Retain the relevant exact ratio numerator/positive-denominator expansions and exact block/minor warrant, tied to each recovered quantity and its projection/error record.
- Bind this retained evidence through private completion and any public method receipt. Distinguish the ordinary floating precondition, exact represented-source current response, qualified binary64 projection, and any later member/stress recovery that still uses projected displacement.
- Exercise source/ratio-tail/method/binding mutations and post-context-drop recovery/replay. Do not substitute a string identity or projected value for the exact basis.

The parent has reserved a separate source/API delta for this issue. It remains open; the compile fix does not close it. Focused nonlinear execution can proceed to observe the bounded runtime path while Current/public-method qualification stays held.

## Runtime source conclusions

- **Ordinary gate first:** each actual current-state solve still uses the same full retained K/f/free/prescribed equations, positive structural factor, conditioning, original residual and contribution-fidelity gate. Exact scrutiny follows the successful ordinary solve. The existing one permitted first-iteration all-active recovery trial remains the only extra contact-state solve; its actual boundary and states are passed to scrutiny.
- **Actual exact source:** the adapter supplies full assembly frame/spring contributions, all original K_fc terms, current prescribed values and the complement free map. Force terms explicitly represent the received aggregate input.force entries. Mixed friction/derived affine source is rejected before qualification; the unused affine-building code cannot secretly qualify that family. No primitive-load exactness is claimed.
- **Signs and selected values:** every supported gap obtains exact penetration and normalized reaction signs from one live Response. Inactive closes iff penetration is nonnegative; Active bears iff normalized reaction is nonpositive. Both displacement and reaction vectors come from response/quantity/DOF-bound projections at the existing 1e-9 criterion. Reaction recovery does not recompute K*u from rounded projected u.
- **No partial response replacement:** projection/source/budget/equilibrium failures occur before changing the selected vectors. Only after all quantities and projected-u original equilibrium pass are both linearized and selected reactions/displacements replaced.
- **Report separation:** ordinary_displacements and the ordinary structural report retain the pre-projection state. product_equilibrium is recomputed on the selected projected u. Allowing their residual vectors to differ on the explicit exact path is mathematically appropriate; treating the old structural residual as the new projected residual would be wrong.
- **Count/cap/deltas:** exact_gap_iteration preserves actual prior states, changed-support counting, input tolerance, max iteration and the existing convergence diagnostic. The product qualifier still demands zero state changes and matching actual/final boundaries. At a cap with a changed state it cannot gain product qualification. Displacement/reaction deltas compare the selected response with the previous selected iteration.
- **Unsupported scope:** no-gap inputs retain their old path. Mixed devices, curved/user matrices, duplicate gap DOFs, exact source asymmetry, larger connected free blocks, range and budget failures receive explicit Unsupported standing. Any unsupported gap iteration prevents final exact qualification even if a later partition is smaller. An ordinary count-converged result is not thereby exact-law qualified.
- **Work:** successful preparation/solve reservations are included once. Each sign/projection receives only the remaining per-iteration allowance. Accepted failed-call charges are retained; rejected unexecuted reservations remain separate. A preparation/solve failure without returned accounting reserves the full attempted cap. The 2,000,000 per-iteration and 8,000,000 run caps are numerical-work reservations, not hardware operation or wall-clock guarantees.
- **Mutation resistance:** the private completion snapshot includes the new report through iteration equality, plus input, ordered springs, selected vectors and reports. Changed source, report signs/projections, maps or result fields cannot update that private snapshot through public mutation. New expected-state references independently test contact-law behavior; receipt rejection alone is not evidence that the classifier's mathematics is correct.
- **Caller behavior:** the inspected product uses returned nonlinear reactions directly at lines 1758–1766 and support publication at 2496–2511. It does not reconstruct those nodal reactions in that path. Other later member/spring/stress recoveries remain their actual separate floating implementations; no blanket exact-recovery claim is licensed.

## Oracle, controls and observed evidence

ROOT_ORACLE_DISPOSITION authorizes only the magnitude-5 expected-state correction in the decimal-gap test. The applied delta changes the local test condition from magnitude<5 to magnitude<=5 with an explanatory comment. Original 0.05 input, k, loads 4/5/6, senses/seeds/modes, convergence and 1e-14 quantity checks remain unchanged. Existing dyadic equality controls remain Active. The author delta, adopted oracle delta and manager API repair were independently verified against their preserved preimages; no hidden criterion change was found.

[_run_records/check_controls.py](_run_records/check_controls.py) imports no production solver. It matches all 27 authored neighbor state, displacement and reaction expectation rows against the frozen independent Fraction reference, including binary64 encodings of tiny nonzero reactions. It confirms the spring and nonzero prescribed-root controls with exact equations and demonstrates the reaction-recovery distinction above. It also checks representative reservation/debit invariants. [Output](_run_records/CONTROL_CHECKS.json) is retained. The authored neighbor test calls 432 cases plus same-mode repeats, or 864 solves; the physical companion adds sixteen combinations. **These are source-prepared counts, not observed nonlinear execution.**

The actual helper PROJECTION_CHECKS raw log was inspected: all seventeen tests passed with sixty-five filtered, including the 2,604 scalar projections. That covers helper700338b9 only. It does not make the new join, receipt behavior, negative controls or broader affine/contact pipeline passing evidence.

Next: run the focused strict_gap tests and adopted decimal/dyadic checks on final a5a3f710/79272e2f; repair/backcheck failures before expanding to relevant affine/contact-recovery and full nonlinear checks. NGR-02, public method/receipt declaration, broader unsupported families, NP-A physical recovery, protected pipeline/native evidence and release qualification remain separate open work.

