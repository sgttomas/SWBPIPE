# I46 — retained Rx cannot transfer to the unchanged product source promise

**Result:** a future product projection that keeps the actual retained native rotation cannot make this loaded specimen pass the unchanged source/represented dual-cover gate. The native point is correctly rounded for exact admitted K properties; the source annulus gives a different torsion law. This is a failed proposed transfer to the product source promise, **not a false kernel publication against q_K** and not a realized false W1 product publication. The captured ordinary output has only a truthful private stricter-gate refusal.

TASK Type2 `/root/i46_product_refusal_diagnosis`, parent ROOT HELP_HUMAN `/root`, delegated-harness-native, no descendants. Receipt 2026-10-03 03:24:07 UTC; scheduled checkpoint 03:34:07, new-analysis cutoff 03:44:07, final deadline 03:49:07. This is an early finite return once the next dependency is settled. Write scope is `NUM/R/I46/product_refusal_01` only. `R` is the parent-supplied `RESUME_2026-09-30` run root. No solver, model, compiler, native producer or network ran. No source, Git/index or API write occurred.

## Frozen facts and checking method

The supplied I45 interim captures are not accepted implementation evidence:

- `CODE/R/I45/product_vertical_02/_run_records/pp09.log`: SHA-256 `bfab7d30ef79b41e72887b657aac6a9fc1f99b82d68a8187febcf832682ee528`.
- `CODE/R/I45/product_vertical_02/_run_records/oracle09.json`: SHA-256 `9216e414822d37c9738ef145b91365ced47574b93f662862dc4eebe886aa865e`.

Both were hashed before content inspection; hashes were rechecked unchanged at sealing. Only required parsed request/input/row/verdict/native/G5a/identity records are retained in `captured_records.json`, including original log line numbers. The independent checker never imports the I45 oracle or product code. The oracle was read only as supplied corroborating output, not as a numerical derivation. `basis.json` records Root/TASK/Piping/skill/contract origins and hashes. `source_origins.json` records selected old source at frozen `6ba653451f9fd27cdb852b7974753a3921f1c483`; mutable I45 code was not read.

`analyze.py` uses standard-library Fraction exact arithmetic. It derives pi from the Machin identity with 240 alternating terms per arctangent and rigorous signed next-term bounds (width < 2^-1100), and uses integer-square tests for root endpoints on a 2^-640 grid. It decodes actual normalized binary64 D/t/E/G, not intended decimal inputs. Annulus A=pi(Dt-t²), I=pi[D⁴-(D-2t)⁴]/64, J=2I, Z=I/(D/2). The represented branch uses separately lifted admitted E/G/A/I/J and positive postprocessing A/J/c, with both Z_hat and exact I_K/(D/2). Products E*A, E*I and G*J remain exact products of admitted bits. The captured unit-length cantilever has true tip Fx=Fy=Mx=1 (or all zero), six fixed base DOFs, and no intervening loads. Closed tip mechanics are ux=1/(EA), uy=1/(3EI), rx=1/(GJ), rz=1/(2EI); actions follow exact statics. Every one of the 73 mechanical ordinary rows is compared with both warranted truths, including raw and SI predicates. Computed classes/scales and combined numeric pass decisions exactly match the captured verdicts. This is an exact point/error investigation; it does not reproduce internal certificate enclosure widths or work counts.

## Ordinary versus retained point results

| Actual or explicitly algebraic object | Source point misses | Represented point misses |
|---|---:|---:|
| Actual ordinary zero case, 73 mechanical rows | 0 | 0 |
| Actual ordinary loaded case, 73 mechanical rows | 28 | 16 |
| Actual loaded retained native points, 52 mapped functionals | 5 | 0 |
| Algebraic direct-unit projection of those 52 captured points into existing product units | 5 | 0 |

The 16 ordinary represented misses are rows **2,10,14,20,22,33,34,40,46,52,55,59,63,67,71,73**. They are ordinary solve/recovery/postprocessing errors relative to the admitted law; this investigation does not allocate each last bit to assembly versus factor solve. The other 12 ordinary source misses are **9,12,53,56,57,60,61,64,65,68,69,72**: ux, rx, five axial stresses and five torsional stresses pass the represented tests but fail the source tests. Their source failure is already present despite acceptable represented values.

For example actual ordinary row34 (free-end bending moment) is -2^-52 N*m, whereas both exact truths are zero; its absolute bound is approximately 7.666467083416871e-20 N*m. The captured retained native end-J bending point is approximately -3.974129385689321e-39 N*m and passes that point test. Ordinary row59's normalized bending stress is approximately -1.3153403742280699e-11 Pa against exact zero and bound 2.322093008194721e-15 Pa: downstream readout does not remove the ordinary equilibrium error. These identify a real numerical-recovery bucket distinct from section-source discrepancy.

All 26 relative ordinary source misses fail both sharper tests while passing both 1e-9 raw/SI relative tests. Rows34 and59 fail their absolute tests. Thus these are actual point misses under the selected stricter predicates, not merely loose source intervals. Unit conversion does not explain the decisive Rx failure: its unit is rad, so normalization is identity. Native versus algebraically unit-projected point checks have the same five source misses: rows **2,9,10,12,14** (tip magnitude, ux, uy, rx, rz).

Zero-case source and represented truth are exactly zero. The captured G5a resolution E_force=E_moment=0 therefore requires positive-zero force/moment rows. Row35 is actually `8000000000000000`; rows35–52 share that defect. Independent arithmetic reproduces the first failure at35. This is a signed-bit publication rule, not a numerical error or interval issue. Loaded G5a zero, sanity, lower-bound and summary scalar checks pass against its captured fields. Unpublished E/estimate/charge/B ownership is supplied evidence, not independently replayed. A future producer's positive-zero behavior requires its own explicit scope; captured ordinary values were not rewritten.

## Decisive exact Rx counterexample

From loaded `I45_INPUT` at log120, ordinary `I45_ROWS` at121 row12, and actual `I45_NATIVE_ROWS` at132 ordinal9:

- D=`3fb999999999999a`, effective t=`3f747ae147ae147b`, G=`4232a05f20000000`.
- Admitted J_K=`3ecc52664442210e` = approximately 3.3762303549047834e-6.
- Actual native and ordinary Rx both equal n=`3ecf0ebea4a79227` = approximately 3.7023540120243145e-6 rad.
- Exact source J_s=pi[D⁴-(D-2t)⁴]/32 is approximately 3.3762303549047817e-6. J_K exceeds J_s by relative 5.417548220793653e-16. This uses actual D/t bits and no rounded inner diameter.
- q_K=1/(G*J_K) is exact rational; n is its correctly rounded RN64 value, proved by strict containment between the midpoints to n's two binary64 neighbors. |n-q_K|=approximately 2.860019757011872e-23 rad.
- q_s=1/(G*J_s) is approximately 3.7023540120243166e-6 rad. Its rigorous lower source-error bound is approximately 2.0343683366291755e-21 rad.

The coupled scale was recomputed from the actual final ordinary row universe, and separately from all 52 algebraically unit-projected native functionals. Stress rows do not contribute to the four primary maxima. InputDerived exclusions, original-operand coupling and actual p=128 (no p512 floor) are retained. Both calculations produce:

`S_tr=S_ro=3ecf0ebea4a79227`, `S_fo=S_mo=3ff6a09e667f3bcd`.

The recomputed stress scales are also unchanged: k=1 `40e4ea61c624e70a`; circular k `40fd27f079f052fe`. These are recomputed results, not scales copied from the ordinary case. The projected Rx row remains relative.

For m=max(|n|,S)=n, the full exact allowance is

`A_exact = m*2^-64 + m*2^-85 + |n|*2^-53 + 2^-1074`.

The independent checker also forms the binary64 allowance with the five separate operations from pinned adaptive.rs:3256–3260: a0=RN64(2^-64*m); a1=RN64(a0*(1+2^-21)); u0=RN64(2^-53*|n|); u1=RN64(u0+2^-1074); A_f64=RN64(a1+u1). Both are approximately **4.1124457205513273e-22 rad**. The exact source error lower bound exceeds both; its ratio to A_exact is **4.946857599755607**. Both public 1e-9 relative predicates pass. `rx_counterexample.json` preserves the exact fractions, pi-derived endpoint fractions, native bits and RN midpoint proof.

This proves why more faithful retained-K projection alone cannot certify this source claim. It neither questions the correctness of q_K rounding nor creates a new source definition.

## Nearby dual-cover incompatibility and next dependency

The source/represented exact Rx gap has lower bound approximately **2.0057681390590568e-21**. Let y be any nearby candidate output satisfying even the existing public relative K test `|y-q_K| <= |y|/10^9`. Then y>0 and

`q_K/(1+10^-9) <= y <= ymax=q_K/(1-10^-9)`.

With the actual recomputed scale held fixed, the sharper allowance is monotone on this range. The same bound also applies if only Rx changes and the coupled scales are recomputed from all unchanged other projected rows: the lower candidate bound exceeds their largest translation/rotation value (~1.4104205760092627e-6), so S_tr=S_ro=y. Any y that satisfied both source and represented sharper tests would need

`q_s-q_K <= 2*((2^-64+2^-85+2^-53)*ymax + 2^-1074)`.

The right side is approximately **8.224891449327546e-22**, strictly less than the rigorous gap lower bound by factor **2.4386560618049797**. Hence these two promised truths cannot both fit such a candidate under the unchanged sharper test. This statement permits neither arbitrary inflation of other row scales nor changes to row universe, source or predicate. No candidate output is selected.

**Smallest next step:** ROOT's fresh independent review of this frozen scalar witness and its actual-row associations, then the owning disposition of the conflict before presenting the future W1 projection as a successful source-transfer witness. An actual later producer may legitimately return this numeric refusal. The counterexample already blocks an all-pass claim for a projection preserving native Rx; another solver run or new factorization is not needed to establish that point. The owning decision is whether and how the intended product promise proceeds given this conflict, not an agent-selected relaxation or alternate source contract.

There is no captured future W1 product producer, new final 73-row projected envelope, projected maximum evidence or complete new producer certificate. Native publication radii/internal enclosures are not in these captures. These absences prevent a full future-product pass claim but do not weaken this lower-bound refusal. I46 has not derived hypothetical stress/maximum output values or relabelled algebraic unit conversion as execution. All predicate, source, output, acceptance, release and downstream-adoption choices remain unchanged.

Reproduce within this evidence folder with `python3 analyze.py`; it uses only preserved parsed inputs and writes `analysis_results.json` and `rx_counterexample.json`. The final execution log and artifact hashes are retained alongside this return. No later broad investigation is necessary for this bounded question.
