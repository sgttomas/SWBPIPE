# G4 addendum: the admission maximum at l ≤ 128

**The ruling:** RR "U4 G4: the margin rule trips; l ≤ 128 adopted; D-6 lock record extended". The primitive-load cap in the one case drops from 192 to 128. n, m and g stay at 32.

**The result.** The admission maximum is now **0.8510 M (sparse) and 0.8559 M (dense)**, at phase W3. **The 0.9 M margin rule holds**, with **197,370,757 B (0.049 M) to spare sparse and 177,660,309 B (0.044 M) dense**. To M, the headroom is 600.0 MB and 580.3 MB.

**Method.**
- `_run_records/sens.py` reran the whole cap-priced chain once, at `G4_CAPS = {"l": 128}`, with ε = 2 and every other cap at D1:
  - composite text;
  - the four T08 text runs to the D fixpoint;
  - t08_closure, producer_caps, t07_repair, t25_g4, ordinary_caps;
  - g4_caps.
- It used the packet's scripts byte for byte. The basis, call graph, lexicon and illustrative strides are unchanged (NUM `b1f80234dc`).
- Its outputs are under `_run_records/` with `.l128` before the extension:
  - `g4_caps.caps.eps2.l128.out.json`;
  - `text_budget{,_env,_X,_W}.caps.l128.out.json`;
  - `text_closure.caps.l128.json`;
  - `t25_g4.caps.eps2.l128.out.json` (= `t25_caps.caps.l128.out.json`);
  - `ordinary_caps.caps.l128.out.json`, `producer_caps.caps.l128.out.json`, `t07_repair.caps.l128.out.json`;
  - `composite_text.caps.l128.json`, `t08_closure.caps.l128.log`;
  - the one-line summary `sens.caps.eps2.l128.out.json`.
- **Cross-check:** the summary is identical to the `{"l": 128}` row of `sensitivity.out.jsonl`.
- The text runs are complete: no unmapped loop and no unclassified argument.

## 1. The phases (caps, ε = 2, l = 128)

| Phase | Sparse E_mov + R | Fraction of M | Dense E_mov + R | Fraction of M |
|---|---|---|---|---|
| X1 ordinary span + T25 | 3,202,811,763 | 0.7954 | 3,222,522,211 | 0.8003 |
| X2 X completion | 1,724,701,981 | 0.4283 | 1,744,412,429 | 0.4332 |
| W1 ordinary span | 1,828,027,943 | 0.4540 | 1,847,738,391 | 0.4589 |
| W2 G-B, G-C, T12–T15, N1 reserve | 1,897,033,817 | 0.4711 | 1,916,744,265 | 0.4760 |
| **W3 publication (T16) + staged copy** | **3,426,507,899** | **0.8510** | **3,446,218,347** | **0.8559** |
| W4 precommit (T17) + successor + invocation + statics | 3,419,812,922 | 0.8493 | 3,439,523,370 | 0.8542 |
| W5 transfer and Direct completion | 2,074,055,017 | 0.5151 | 2,093,765,465 | 0.5200 |

**The maximum, W3:** requested 3,170,117,330 / 3,189,827,778 B, plus moving 189,281,705 B (the publication text's last growth), plus R = 67,108,864 B.

**Against the rule:** 0.9 M = 3,623,878,656 B, so the rule holds in both modes.

**Still available, not adopted:** the phase-aware span lever (COMPOSITION_G4.md §5) would give 0.7960 / 0.8003 M. X1 would then be the maximum.

## 2. D and the text totals

| Quantity | l = 192 (G4) | **l = 128** |
|---|---|---|
| D (every diagnostics vector) | 17,574 | **14,694** |
| D_env | 11,408 | **9,360** |
| Text(diag_env) | 78,860,051 | **68,709,540** |
| Text(diag_total) | 109,065,359 | **94,478,624** |
| TAV, whole invocation | 2,144,966,676 | **2,044,161,940** |
| TAV moving | 2,147,566,638 | **2,046,761,902** |
| TAV_X | 1,445,193,238 | **1,356,903,574** |
| TAV_W | 1,583,158,378 | **1,511,208,042** |
| TAV, envelope-only variant | 662,151,226 | **615,069,498** |

**Unchanged:**
- largest single site, 2,599,962;
- reachable functions, 2,698;
- sites 2,797 / reached 2,716 / positive 1,426.

## 3. Every G4 figure that changes (caps, ε = 2)

Every figure not listed here is unchanged. In particular these stay as they were:
- the invocation Value (15,782,080);
- the statics (5,397,696);
- T19 (10,240);
- T18.3;
- R and k;
- every milestone figure, because the milestone has l = 3;
- the integrity-message L_max (2,549,385);
- Text(row);
- P_final;
- the stack bounds.

### COMPOSITION_G4.md

| Figure | G4 | l = 128 |
|---|---|---|
| O without T25 (sparse / dense) | 245,257,549 / 264,967,997 | 241,156,429 / 260,866,877 |
| T25 | 1,475,185,857 | 1,348,418,972 |
| TAV_X / TAV_W | as §2 | as §2 |
| T11 | 204,912 | 166,000 |
| T12–T15 | 68,293,632 | 67,936,832 (T12 1,685,066; T13 42,730,598; T14 + T15 unchanged) |
| T16 peak | 1,389,829,540 | 1,253,264,671 |
| Staged copy (T18.1) | 105,778,121 | 95,316,314 |
| T17 peak | 1,275,668,333 | 1,149,092,968 |
| Successor | 190,637,407 | 171,613,264 |
| N1 reserve (T18.2) | 757,746 | **1,069,042** (see the note below) |
| Moving candidates | T25 publication text 209,737,314; T16/T17 publication text 209,961,095; N1 old backing 1,734,016 | 189,057,924; **189,281,705** (the maximum's); 1,422,720 |
| Phase table and margin | 0.9115 / 0.9164 M, rule fails | §1: 0.8510 / 0.8559 M, **rule holds** |

**The N1 reserve rises as l falls.**
- **Why:** G4 prices it as push-growth headroom, s(Diagnostic)·(PushCap(D_env + 1) − D_env) + 1,394 B of strings.
  - At D_env = 9,360 that headroom is 7,024 slots.
  - At 11,408 it was 4,976.
- **The bound is sound, but it is not monotone in D_env.** The admission formulas don't rely on monotonicity at fixed caps.
- **Grant 1b reserves exactly** (`try_reserve_exact(1)`, TRANSFER_COMPLETION.md §7), so the true growth is ≤ s(Diagnostic) = 152 B. G5 may price it that way.

**The sensitivity table** (COMPOSITION_G4.md §4) was not re-run around the new base. Its `l = 128` row is this run.

### PUBLICATION_READER.md

| Figure | G4 | l = 128 |
|---|---|---|
| ENV_S slots / objects / entries / string bytes / key bytes (numbers 2,495 unchanged) | 68,420 / 17,845 / 103,007 / 103,306,441 / 1,916,909 | 57,988 / 15,797 / 90,719 / 93,155,930 / 1,720,301 |
| ENV_S e | 209,961,095 | 189,281,705 |
| BODY slots / objects / entries / string bytes / key bytes / numbers | 23,752 / 18,724 / 56,951 / 30,180,344 / 1,822,432 / 16,898 | 21,640 / 18,596 / 56,503 / 25,395,192 / 1,808,096 / 16,642 |
| BODY e | 63,234,304 | 53,623,040 |
| SOURCE slots / objects / entries / string bytes / key bytes / numbers | 2,880 / 5,540 / 20,503 / 374,144 / 656,096 / 8,163 | 2,816 / 5,412 / 20,055 / 360,832 / 641,760 / 7,907 |
| T16 P1 / **P2** / P3 / P4 | 181,768,169 / 1,389,829,540 / 618,835,036 / 305,322,181 | 166,849,338 / **1,253,264,671** / 545,995,821 / 276,307,798 |
| T16 moving | 209,961,095 | 189,281,705 |
| T17 V1 / **V2** / V3 / V4 / V5 / V6 | 368,440,399 / 1,275,143,949 / 33,395,612 / 12,442,149 / 193,966,444 / 210,247,486 | 319,588,047 / **1,148,568,584** / 32,598,012 / 12,205,989 / 174,706,141 / 195,635,151 |
| T17 | 1,275,668,333 | 1,149,092,968 |
| U1 text part: retained_wire.rs / retained_product.rs / PP lib.rs U3 region / result_export | 465,938 / see §5 / 11,857,176 / 276,447,390 | 463,890 / 600,843,722 / 11,692,824 / 264,343,582 |

### TRANSFER_COMPLETION.md

| Figure | G4 | l = 128 |
|---|---|---|
| T18.1 / B-2 staged copy | 105,778,121 | 95,316,314 |
| T18.2 / B-6 N1 reserve | 757,746 | 1,069,042 |
| N1 moving term s(Diagnostic)·D_env | 1,734,016 | 1,422,720 |
| B-3 successor | 190,637,407 | 171,613,264 |
| B-5 precommit reader | 1,275,668,333 | 1,149,092,968 (+ 5,397,696 statics) |

### API_G4.md (the gate facts)

| Fact | G4 bound | l = 128 bound |
|---|---|---|
| G-B `case_loads` | l = 192 | **128** |
| G-B `observation_bytes` (T11 without the late capture) | 150,000 | 125,424 |
| G-C `observation_bytes` (T11) | 204,912 | 166,000 |
| G-C `late_capture_bytes` (T11.P1) | 54,912 | 40,576 |
| G-C `envelope_diagnostics` | D_env = 11,408 | **9,360** |
| G-C `envelope_diagnostic_text_bytes` = 2·Text(diag_env) | 157,720,102 | **137,419,080** |

### G3_REPAIRS.md

| Figure | G4 | l = 128 |
|---|---|---|
| §2.2 TAV, D, D_env, Text(diag_env), Text(diag_total) | as §2 | as §2 |
| §4 S-3 temporary on the T25 payload route (L_max = source identity JSON) | L 1,797,413; about 7.2 MB | L 1,495,973; about 6.0 MB |
| §5 T25 (ε = 2) requested / moving | 1,475,185,857 / 209,737,314 | 1,348,418,972 / 189,057,924 |
| §5 stages S4 / I1 | 689,105,305 / 1,423,546,387 | 682,820,665 / 1,296,779,502 |
| §5 N-6 descriptor units #1 | 41,760 | 41,632: still above 16,384, so selection is still infeasible at the caps (X only; X passes) |
| §6 R-2: pressure_runtime.rs TAV | 25,622,784 | 17,081,856 |
| T07 requested / moving (identity JSON) | 39,805,125 / 41,602,538 (1,797,413) | 38,482,117 / 39,978,090 (1,495,973) |
| §9 S-2's loops, +97.3 MB | — | not re-derived separately. They sit inside the re-derived totals of §2 |

**Not re-run at l = 128:**
- **ε = 6:** D1.11 is adopted.
- **The milestone column:** it does not depend on the caps.

## 4. G2 amendment: the cap row (for DOMAIN.md §2)

| Replaces | New text |
|---|---|
| DOMAIN.md §2, row **l**: "`load_cases[0].primitive_loads.len()` \| ≤ 192 \| 3" | "`load_cases[0].primitive_loads.len()` \| **≤ 128** \| 3". Ruled in RR "U4 G4: the margin rule trips; l ≤ 128 adopted" |
| G3 G2_AMENDMENTS.md §2 typed capacities: "`load_cases[0].primitive_loads` \| capacity ≤ 192" | "capacity **≤ 128**" |
| DOMAIN.md §2 derived quantities: "source_count = 144m+s+l \| 4,832" | "4,768" (limit 16,384, unchanged) |

**What else in DOMAIN.md is untouched:**
- Every other cap row and derived quantity: none of them contains l.
- D1.9, which still reads "every fact in §2 is within its cap", so the clause text needs no change.
- §4's witness reading: n, m and g stay at 32 for the U8 Ceiling witness, and the milestone's three loads sit far inside 128.

**For G5:** the census constant for l is 128, in retained_memory.rs's D1 predicate. A refusal above it is `Cap(l, observed)` → `resource_admission`.

## 5. Errata to the sealed G4 records (the totals are unaffected)

The composition reads its totals from the outputs, and those reproduce byte for byte. Four counts quoted in prose predate the last two lexicon and rule edits: N-2's `from_literal` rows, and S-2's `ambiguous_supports` rule. The D1-caps outputs give these values:

| Record | Printed | Output (D1 caps) |
|---|---|---|
| PUBLICATION_READER.md §4 (and RETURN.md, "capture 593 MB"): retained_product.rs | 156 sites, 593,234,896 B | **207 sites, 601,040,330 B** (includes `from_literal`, 7,805,434 B) |
| G3_REPAIRS.md §2.2: sites inventoried / reached / positive | 2,712 / 2,631 / 1,372 | **2,797 / 2,716 / 1,426** |
| G3_REPAIRS.md §8: `from_literal` positive rows and bytes | 52; 7.8 MB | **53; 7,886,920 B** |
| G3_REPAIRS.md §9: headers matched by a zero rule | 57 | **56** (after the `ambiguous_supports` rule) |

**Execution.**
- TASK I65, records only.
- Memory guard PID 5387 was running.
- Stdlib Python; no Cargo; no Git writes.
- Scratch was WT/scratch/i65_u4_g4_01/sens_l128/.
- New files only: this addendum and the 14 `.l128` run records. SHA256SUMS was appended. No existing file was changed.
