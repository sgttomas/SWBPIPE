# T05 The ordinary span at the D1 caps, per mode

**Result (ASSUMED layout, caps):**

| Mode | O_req, everything priced | of which T25 (ε = 6) | O_req without T25 |
|---|---|---|---|
| `sparse_interactive` | 3,319,281,959 B | 3,074,072,410 | **245,209,549 B** |
| `dense_scrutiny` | 3,338,992,407 B | 3,074,072,410 | **264,919,997 B** |

**Moving, O_mov.** O_req plus the largest single old backing in the span. At ε = 6 that is the T25 publication text's last growth, 616,260,332 B; at ε = 2 it is 207,573,348 B. One reallocation is in flight at a time, so the moving term adds one backing, not one per owner. Text (T08) is composed separately in COMPOSITION.md.

`_run_records/ordinary_caps.py` transcribes every accepted I54 family at generic counts and evaluates it at the caps and at the milestone. Output is in `ordinary_caps.caps.out.json`, as integer coefficients on layout atoms plus a byte constant, and in `ordinary_caps.milestone.out.json`.

## 1. Counts at the caps

- n = m = g = 32, s ≤ g = 32, r = l = 192.
- N = 192, and F ≤ N = 192 is taken as an independent upper (F = N − k is not monotone in k).
- k ≤ min(N, r) = 192; C = 144m + s = 4,640; Z = min(N², C) = 4,640; Zf = min(Z, F²) = 4,640.
- E = 78m + s = 2,528; H = F(F+1)/2 = 18,528; d ≤ m + s = 64.
- Ps ≤ 4Z + 2C, because Σ PushCap(8, c) ≤ Σ max(4, 2c); Pd ≤ 6Z + 2C.
- P_final = 2,115; R0 = 1,891.
- Materials: 4 model + 4 request, 16 points each. Text atoms come from TEXT.md §5.

## 2. Families and sources

Rows in the same phase are summed: a conservative sum of phase maxima (I54 BOUND:16–26; RV75). The two modes differ only in the typed core and the dense parity tail.

| Family | I54 source | Caps, sparse / dense |
|---|---|---|
| R_raw, the captured request | T02 (G2) | 15,780,608 |
| T_resident: the typed request at the typed capacity caps, plus the `project.units` Value ≤ R_raw | T03 + G2_AMENDMENTS §2 | 16,292,992 |
| Material copies: working clone and default-basis clone | BOUND:52 | 150,272 |
| **O-N**: `normalize_model_units`, and `resolve_shared_sections` with no sections (S-4) | new (PP/lib.rs:7514–7600, 8347–8446) | 1,664 |
| BuiltModel resident; built helper maps; Boundary | BOUND:53–55 | 47,991; 3,669; 29,184 |
| Sparse K; pattern construction and assembly retention | BOUND:56–57 | 150,024; 240,072 |
| Primitive loads and application; AssembledForce; sparse reduction | BOUND:58–60 | 127,150; 70,656; 21,504 |
| Formation-guard Bodies; late maps and RecoveryFinding | ADDENDUM "Formation guard" | 5,719; 10,485,575 |
| Typed core, sparse: pattern-copy AssemblyEvidence, sparse preparation, skyline factor and RCM, audit, report, load fidelity, intended action, formation, rigid, solve vectors | BOUND:61–73 + ADDENDUM | 6,066,698 in total |
| Typed core, dense: dense AssemblyEvidence and three dense views, dense preparation and Cholesky, audit with N² headers, … | BOUND:62, 64, … + ADDENDUM | 11,951,554 in total |
| Legacy profile observation | BOUND:65, 68 | 1,146,384 |
| W2: two evaluations, unscale, return tail and direction error | W2_RETURN | 15,128,759 / 26,898,471 |
| Dense parity tail | BOUND:69 | — / 2,055,880 |
| Straight member recovery and stress supplement; scalar maximum | BOUND:75–76 + ADDENDUM | 1,633,487; 16,793,872 |
| Source row qualification; the existing ordinary or failed source case | BOUND:77–78 | 22,964,841; 27,065,494 |
| Preview rewrite; final aggregation; preview tree final copy | BOUND:79–81 | 6,288,422; 59,564,547; 224,328 |
| Retained diagnostics: the `Vec<Diagnostic>` backing only, at D = 25,544 (its strings are in T08's TAV) | BOUND "Diagnostics_upper" + TEXT.md | 4,980,736 |
| ErrorPrefix: the blocked envelope's own owners | BOUND:25 | 139,776 |
| **T07** deep legacy-exact (RESIDUALS_G3 §T07) | `LegacyRecoveryPrefix_or_Exact` | 39,805,125 |
| **T25** selected source-blocks finalization (RESIDUALS_G3 §T25) | new (B-1) | 3,074,072,410 (ε = 6) / 1,439,324,474 (ε = 2) |

**Milestone cross-check.** Six transcribed sub-expressions reproduce I54's published constants at the named fixture's counts exactly (`milestone_cross_checks`):
- CSR K 4,712;
- sparse prepared arrays 4,796;
- evidence pattern copy 4,712 and 7,016;
- the rigid witness 2,992;
- the audit children 12,752.

## 3. Monotonicity lemma, one per family class

Every family above is built from these primitives, each nondecreasing in each count taken separately:
- nonnegative-coefficient polynomials;
- `PushCap(h)`, `max(4, 2h)`, `p(h)`, `B(h)` and `G(h)`;
- hashbrown `buckets(h)`;
- the BTree node count 1 + ⌊(h−1)/5⌋;
- stable-sort scratch (0 for h ≤ 20, else s·max(h, 48));
- `max` and `min`.

Sums, products of nonnegative terms, and compositions of nondecreasing functions are nondecreasing. Every count has its own cap, and the joint quantities are replaced by independent monotone uppers:
- F ≤ N (instead of N − k);
- Z, Zf ≤ min(…);
- d ≤ m + s;
- Ps and Pd by the max(4, 2c) sums;
- h_delta, h_I and h_L by their loose uppers 2·min(C+Z, k(d+1)), 1 + 2C and 1 + 2C + 2l.

So for every D1 input, every family is at most its value at the cap vector. Two families are not polynomials:
- **T07** has its own lemma (RESIDUALS_G3 §T07). Descriptor generations #2 and #3 are bounded by the 16,384-unit law, which does not depend on counts.
- **T25** takes a maximum over stage expressions, each nondecreasing. The text atoms it uses are cap-evaluated upper bounds.

Mode is a parameter, not a count: each mode is evaluated separately.

## 4. Notes

- **Default basis only (D1.5).** No second cached basis is priced. A selected modulus basis is outside D1.
- **Owners counted in more than one family** are summed, which is conservative. Examples: the raw clone in T25's `requested()` and R_raw; a text atom counted again on top of TAV.
- **The data `.clone()` sites** that TEXT.md lists as "not text" are inside these families (PP model, material, force, report and record clones) or inside T07, T13 and T25.
- **Every stride is an ASSUMED illustrative value** (`g3lib.py`), apart from the two derived from I54's DWARF node observations (ResultItem 296, RowTreatment 104). G5 evaluates each atom in-build (BUILD.md §3), and its symbolic coefficients are in the output files.
