# RV53 — private certificate helpers review

**CLEAR for manager fan-in of this bounded source slice. No actionable blocking or non-blocking findings.** Reviewed candidate `82fc4ebdca043c25c701d435e09b8830640fe950` against `fdae294643b798c1849da8b2e643085562593686`. This is independent TASK Type 2 review under ROOT, using the software-code-review skill, with no implementation contribution, source edits, Git/index/API writes, or delegation.

## Scope and source conclusions

The scope validator passes. The complete maintained diff is exactly five paths: one private module-registration hunk in `directed.rs`, the new helper module, its tests, and the two new fixture files. Existing APIs, tests, vector/oracle bytes and tolerances are unchanged. No product caller, certificate activation, tariff or allowance is added.

- **Sqrt:** fresh p1024 owners, native negative-input refusal, canonical +0 for either zero sign, one nearest root, exact q²−a side test, and at most one existing parent adjacent step implement the selected construction. For nonnegative q, sign(q²−a)=sign(q−sqrt(a)); the nearest result is one of the two bracketing endpoints. Power-of-two predecessor spacing remains the parent step's smaller unit. Exponent failures in the exact product residual remain refusals rather than false endpoints.
- **Small bound:** admission is exactly finite value and finite 0<scale<2^-988. The two divisions and two scale-back multiplications are separate expressions and match existing absolute_bound/row_bound. The final sum is compared exactly, never rounded to p1024 first. Positive finite encodings are monotone; h>0 gives lo<sum, the MAX check gives sum<=hi, and each branch preserves that invariant. MAX<2^63 and width shrinks to at most ceil(width/2), so 63 bisections suffice after the maximum comparison. The u8 count cannot wrap.
- **Custody:** collection is outside the sqrt fallible closure and follows every local comparison even when it fails. clear retains sum work. Context and sum totals are joined through checked addition, including composition overflow. Original numeric errors remain observable alongside non-exact work; successful values are exposed only after joined status passes. Private fields prevent caller-created qualified counts. The four explicit binary64 operations and search count stay separate from any future tariff.

## Independent numerical evidence

`_run_records/independent_checks.py` does not import or run the author generator. It checks every stored sqrt vector using exact squared lower/upper inequalities, symbolic exponents, lattice adjacency, and midpoint-square nearest selection; no exponent-sized integer is allocated. All **14** vectors agree, including the power-of-two crossing and extreme symbolic exponents.

For every one of the **18** small-bound vectors, independent integer arithmetic in units of h=2^-1074 recomputes both separately rounded scale-back expressions and direct final quantum ceiling. Nearest results also satisfy exact predecessor/successor midpoint inequalities. Every expected result satisfies predecessor<exact_sum<=result. Both final midpoint ties, normal/subnormal cases, equality and maximum finite input agree.

The double-rounding discriminator is independently confirmed: value=16 and scale=h give r=2^-49 and b0+h=2^-1073, exactly half a p1024 ulp at r. Nearest p1024 loses that tail to the even r, while direct RU64 requires r's successor. The search-bound arithmetic independently reaches width one after 63 bisections.

## Executed checks and evidence

| Check | Result |
|---|---|
| Original I39 SHA256SUMS, run from relocated _run_records/original | All 33 entries pass |
| Scope validator | PASS; no violations |
| Focused debug directed tests | 10/10 pass, including 8 new helpers and 2 existing compatibility controls |
| Optimized new helper tests | 8/8 pass |
| Independent exact fixture checks | 14 sqrt + 18 small-bound cases pass |
| Maintained-file diff whitespace | PASS |
| Post-check source hashes and source status | Unchanged; clean candidate at stated HEAD |

Both Cargo commands used --locked --offline, four build jobs, two test threads, a dedicated per-manifest target, and 1200-second limits. Exact argv/cwd/environment, source hash references, PIDs, exits and raw output are under `_run_records`. No review Rust harness was needed. The supplied source tests explicitly exercise negative/range prefixes, invalid admission, failed partial comparisons, adjacent-step refusal, independently coexisting numerical/work errors, and blocked success extraction.

The whole-candidate whitespace command exits 2 solely on whitespace retained inside original raw logs. Its output is preserved; the maintained-code check passes. This is not a numerical defect or evidence alteration, and no sealed evidence bytes were edited. The original relocation map and original seal remain valid.

## Limits and return

This clears only the two-helper implementation on the supplied RV51-cleared bounded 64-bit prerequisite. Full product/profile gates, source adapters, runtime policy, wider custody gaps, availability and acceptance remain separate. No heavy sweep, solver/model run, native witness or merge/acceptance action was performed.

Preliminary discovery had two recoverable missing-path reads and an overbroad file-name listing; no unrelated contents supplied authority. Initial Git status omitted GIT_OPTIONAL_LOCKS=0; no explicit index operation occurred, and no index mutation was observed. Subsequent recorded status/scope reads disable optional locks. Origins and this disclosure retain the execution boundary.

Both owned Cargo PIDs exited. At 2026-10-02T23:55:48.999634Z, no Cargo/rustc/frame-kernel test process remained; memguard 5387 was still running. **Cargo lane released to ROOT.** Review completed before the 20-minute checkpoint, without reaching the expansion cutoff. Hash manifest runs from this review directory.

