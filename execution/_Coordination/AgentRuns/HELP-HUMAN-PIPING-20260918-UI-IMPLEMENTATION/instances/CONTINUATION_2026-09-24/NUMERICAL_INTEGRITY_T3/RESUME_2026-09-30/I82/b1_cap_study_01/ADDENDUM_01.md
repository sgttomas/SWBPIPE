# I82 B1-S ADDENDUM_01: the options within M ≤ 12 GiB

TASK (Type 2), I82, for ROOT, who is the return path. 2026-10-07 UTC. No descendants. This addendum extends `STUDY.md` (sha256 `d8b18220…7188`), which is unchanged.

**The request:** ROOT's message re-scoped this addendum to options within 12 GiB. It follows RR "Owner decision: M's practical limit is 12 GiB; target machines", which I read before pricing.
- M ≤ 12 GiB (12,884,901,888 B) is ROOT's; above that is the owner's.
- The target is 32 GB workstations. The floor is 16 GB workstations "still solving within practical timeframes".

An earlier, wider request (up to 64 GiB) was superseded before I wrote anything. Its extra runs at c = 5–8 are kept in the run records, but not used here.

**The method is STUDY.md's, unchanged:**
- the same multi-case chain (`b1_mc_chain.py`), rebuilt from a fresh `git archive` of main `d8c88774d0`;
- the same in-build evaluator and atoms (I72's law record).

I re-checked first: at c = 1 the tree is byte-identical to the registered one and to STUDY's recorded `d1_c1`.

**What I ran.** Python only: no cargo, solver, native or at-scale run, no Git write, and nothing in the system temp directory. Every point's TEXT is complete, its D fixpoint converged, and no atom is missing.

**The budgets** under the margin rule E_mov,max + R ≤ 0.9 M:

| M | 0.9 M (B) |
|---|---|
| 12 GiB | 11,596,411,699 |
| 11 GiB | 10,630,044,057 |
| 10.5 GiB (11,274,289,152) | 10,146,860,236 |

**Text-error budget** means margin / TAV_W, QUAL §3's measure. **"5 % M"** is the smallest M with E+R + 5 %·TAV_W ≤ 0.9 M.

## 1. A single tier at D1's full model caps, C = 3

**Caps:** n, m, g ≤ 32; Σr ≤ 192; l ≤ 128 per case; L = C·l = 384, which is implied; a = c = 3.

| Mode | E+R (B) | At 11 GiB | At 12 GiB |
|---|---|---|---|
| Dense | 9,747,725,678 | 882,318,379 B under (21.1 %) | 1,848,686,021 B under (44.1 %) |
| Sparse | 9,688,594,334 | 941,449,723 B under (22.5 %) | 1,907,817,365 B under (45.5 %) |

- TAV_W is 4,189,696,338 B.
- **Smallest M:** 10,830,806,309 B (10.09 GiB).
- **5 % M:** 11,063,567,217 B (10.30 GiB).
- **The smallest 256 MiB step with 5 % or more is 10.5 GiB** (11,274,289,152 B):

  | Mode | Fraction of M | Under 0.9 M by (B) | Text-error budget |
  |---|---|---|---|
  | Dense | 0.8646 | 399,134,558 | 9.53 % |
  | Sparse | 0.8594 | 458,265,902 | 10.94 % |

**What binds:** W3, the publication with the staged copy, in both modes.

| W3 term (dense) | Bytes |
|---|---|
| TAV_W | 4,189,696,338 |
| T16 at stage P2, the body `json!` plus hash(publication) | 3,589,009,045 |
| O_base | 812,438,740 |
| T16_moving | 505,455,619 |
| T12_T15 | 323,428,560 |
| STAGED | 254,737,894 |

- T16's other stages: P3 is 2,794,566,243 and P4 is 1,121,058,618.
- **Next behind W3:** W4 is 215,185,577 B below it (T17 V2_hash is 3,039,970,834), and X1 is 837,555,310 B below.

## 2. C = 4 in a single tier: the least costly trim that fits at 12 GiB

At D1's full caps, C = 4 needs 13,060,814,532 B (dense), which is 1,464,402,833 B over 0.9 × 12 GiB.

**Single-cap trims** (dense E+R, B; a = c = 4, L = 4·l unless shown):

| Trim | Dense E+R | Under 0.9 × 12 GiB by | Text-error budget at 12 GiB | 5 % M |
|---|---|---|---|---|
| **l ≤ 32** | 11,213,200,292 | 383,211,407 | **7.52 %** | 12,742,106,883 (11.87 GiB) |
| **m ≤ 24** | 11,348,074,996 | 248,336,703 | **5.27 %** | 12,870,592,159 (11.99 GiB) |
| m ≤ 20 | 10,512,635,788 | 1,083,775,911 | 25.3 % | 11,918,290,171 |
| m ≤ 16 | 9,680,060,143 | 1,916,351,556 | 49.9 % | 10,969,178,250 |
| l ≤ 48 | 11,518,161,636 | 78,250,063 | 1.51 % | 13,085,235,802 (over 12 GiB) |
| l ≤ 64 | 11,824,149,188 | 227,737,489 over | — | — |
| l ≤ 96 | 12,440,234,148 | 843,822,449 over | — | — |
| L ≤ 128 (l = 128) | 11,921,898,948 | 325,487,249 over | — | — |
| L ≤ 256 / 384 | 12,301,537,476 / 12,681,176,004 | over | — | — |
| n ≤ 16 / 24 | 11,902,156,140 / 12,486,979,596 | over | — | — |
| g ≤ 16 / 24 (and s, r) | 12,087,892,388 / 12,553,487,236 | over | — | — |

**Only two single trims fit with at least 5 %:** l ≤ 32 and m ≤ 24. Both need M = 12 GiB to hold 5 %.

**The least costly trim is l ≤ 32 loads per case.**
- It keeps the full 32-node, 32-member, 32-support model. It also has the larger margin: 383 MB against 248 MB.
- Under D1.7, a case's loads are nodal force and moment primitives only, so 32 per case still allows one component at every node.
- By contrast, m ≤ 24 cuts the modelled pipe by a quarter.

**The total-loads cap is the weakest lever.**
- It saves about 2.97 MB per load: 379,638,528 B per 128 loads.
- Even L = 128 leaves C = 4 over budget.
- At C = 3 it is not needed. So **L should stay C·l** (stated, not binding).

**Effect on W2 and W2b.** Their committed input, `law_tests::cap_maximal`, has 32 nodes, 32 members, 32 supports and 128 loads in one case.
- Under either single-tier trim it leaves the domain: l ≤ 32 refuses its 128 loads, and m ≤ 24 refuses its 32 members.
- Both witnesses would be re-based on a cap-maximal input at the trimmed caps.
- The other committed c = 1 witnesses (the milestone, W1, W2-deep, W3–W7, headroom, U8's inputs) have at most 3 loads and 1 or 2 members, so they stay.
- A two-tier variant (tier 1 = D1; tier 2 = 2 ≤ c ≤ 4 with l ≤ 32) would keep W2 and W2b at the same M, at two tiers' complexity.

## 3. The comparison

| | **S3: one tier, D1 caps, C = 3** | S4: one tier, C = 4, l ≤ 32 | S4′: one tier, C = 4, m ≤ 24 | P1: two tiers (STUDY §4.1) |
|---|---|---|---|---|
| Model size | n, m, g ≤ 32; Σr ≤ 192; l ≤ 128 | 32/32/32; Σr ≤ 192; **l ≤ 32** | n, g ≤ 32, **m ≤ 24**; l ≤ 128 | c = 1: 32/32/32, l 128. c = 2–3: **16/16/16**, Σr 96, l 64 |
| C | 3 | 4 | 4 | 3 |
| Dense / sparse E+R (B) | 9,747,725,678 / 9,688,594,334 | 11,213,200,292 / 11,134,358,500 | 11,348,074,996 / 11,265,743,412 | 4,906,282,942 / 4,894,190,830 (tier 2) |
| Smallest M; 5 % M | 10.09 GiB; 10.30 GiB | 11.60 GiB; 11.87 GiB | 11.74 GiB; 11.99 GiB | 5.08 GiB; 5.18 GiB |
| M proposed | **10.5 GiB** (11,274,289,152) | 12 GiB (the owner's ceiling) | 12 GiB | 5.25 GiB (5,637,144,576) |
| Margin to 0.9 M (dense) | 399,134,558 B | 383,211,407 B | 248,336,703 B | 167,147,176 B |
| Text-error budget (dense) | **9.53 %** (21.1 % at 11 GiB; 44.1 % at 12 GiB) | 7.52 % | 5.27 % | 8.78 % (at 12 GiB: 351.6 %) |
| Headroom under 12 GiB without the owner | 1.5 GiB of M, so 44.1 % of TAV_W absorbable | none | none | 6.75 GiB |
| Complexity | **One tier:** DESIGN_v2 §6's rows (c ≤ 3, l_i ≤ 128, Σ ≤ 384); one profile; one set of G-B and G-C bounds (EnvelopeResults ≤ 3·P_final, etc.) | One tier, as S3 | One tier, as S3 | **Two tiers:** tier selection from c, two cap tables, two form sets with the profile's maximum, G-B and G-C bounds per tier, witnesses at both tiers' cap-maximal inputs |
| Committed witnesses in the domain | **All:** every c = 1 witness, W2 and W2b included, and W-C2 | All except **W2 and W2b**, which are re-based to 32 loads | All except **W2 and W2b**, which are re-based to 24 members | All: tier 1 is D1; W-C2 is in tier 2 |
| New cap-maximal witness B1 needs | 3 cases × 128 loads at 32/32/32 | 4 × 32 loads at 32/32/32 | 4 × 128 loads at 32/24/32 | Tier 2: 3 × 64 loads at 16/16/16 |
| **Priced worst-case heap of one W1 invocation at the caps** (E_mov,max: requested plus moving bytes; the 64 MiB reserved stack is on top) | 9,680,616,814 B = **9.02 GiB** | 11,146,091,428 B = 10.38 GiB | 11,280,966,132 B = 10.51 GiB | 4,839,174,078 B = 4.51 GiB (c = 1: 3.29 GiB) |
| That heap against 32 GiB of RAM / 16 GiB of RAM | 28.2 % / 56.3 % | 32.4 % / 64.9 % | 32.8 % / 65.7 % | 14.1 % / 28.2 % |

### 3.1 Product reach, stated plainly

**What M is.** M is the registered build's per-invocation W1 admission threshold. An admitted invocation's priced need, E_mov,max + R, is at most 0.9 M. The caps decide that need; the choice of M within its range does not.

**The priced figure is a conservative worst case, not a measured peak.**
- It sums every owner of a phase as if live, and every per-case owner c times.
- For scale, the committed challenge measured the milestone's actual peak at 3,541,898 B, against today's bound of 3.6 GB.
- The actual peak of a cap-maximal W1 invocation has never been measured. RR's new evidence obligation (peak resident memory and run time, both modes) supplies it at B1.

**What the figure does not include** (D-7's non-claims):
- allocator overhead and fragmentation, or RSS;
- the rest of the process and the OS;
- concurrent invocations;
- invocations refused at admission. They take the ordinary route, which M does not bound.

**What this means for each option, with no machine claim:**
- **On a 16 GiB machine,** S3's priced worst case would need 9.02 GiB of heap. That is 56 % of physical memory, before the OS and the application.
- **S4 and S4′ would need about 65 %.** P1's would need 28 %.
- **On a 32 GiB target workstation,** the figures are 28 %, 32–33 % and 14 %.
- **Whether 16 GiB machines solve "within practical timeframes"** depends on measured peaks and run times, which this study does not price. A three-case cap-maximal W1 invocation runs three full native runs.

## 4. Recommendation

**S3: one tier at D1's full model caps with C = 3, and L stated as C·l = 384.**
- **M:** select at B1's G6 the smallest 256 MiB step with at least a 5 % text-error budget, expected to be **10.5 GiB** (11,274,289,152 B).
- **Expected at 10.5 GiB:** dense 0.8646 M (9.53 %), sparse 0.8594 M (10.94 %).
- **12 GiB is the ceiling** within ROOT's authority, at 44.1 %.

**Why:**
1. **It is one tier** (ROOT's preference) and fits with at least 5 % well below 12 GiB.
2. **It trims nothing,** so D1's domain is unchanged for c = 1. W1–W7, W2-deep, W2 and W2b, headroom and U8's inputs all stay in the domain, and W-C2 (C = 3) fits exactly.
3. **It is the only one-tier option that leaves headroom under 12 GiB.** B1's real-code G5 may differ from this emulated pricing. S3 can absorb up to 44 % of TAV_W by raising M within ROOT's authority, without asking the owner. S4 and S4′ use the whole 12 GiB for C = 4, with 5–7.5 % budgets and no headroom.
4. **Its priced worst case, 9.02 GiB,** is below a 16 GiB machine's memory, though not by much. It is under a third of the 32 GB target. The measured peak, due at B1, decides the floor.

**If C = 4 is wanted,** take S4: one tier with l ≤ 32 at M = 12 GiB. It is the least costly trim, but it leaves no headroom and re-bases W2 and W2b. A two-tier variant keeps W2 and W2b.

**P1 remains the low-memory option.** Its priced worst case is 4.51 GiB, but it has half the model size for c ≥ 2 and two tiers' complexity.

## 5. Records

`_run_records/addendum_01/`:
- `addendum_report.json`: every point's per-mode E+R, heap, phases, binding terms, smallest M and 5 % M, and budgets at 5.25, 10, 11 and 12 GiB;
- `sweep_E.*`: the superseded c = 4–8 points; only `d1_c4` and `d1_c4_L*` are used here;
- `sweep_F.*`: C = 3 and the C = 4 trims;
- `profile_trees/`.

`_run_records/` also holds:
- `points_E.json`, `points_F.json` and `points_addendum.json`;
- `b1_addendum_report.py`.

The STUDY tools are reused unchanged. Placeholder paths only.

**Scratch:** the fresh archive copy and the per-point runs in `WT/scratch/i82_b1_study/` are deleted after sealing, as before.
