# B1-SC: corpus 07n (one writer), then the harness pins

Read `R/BRIEFS/B1_COMMON.md` first. **The specification is PLAN_v2 §2.5** (`R/I84/b1_plan_01/PLAN_v2.md`), with the items RR has routed to SC since (below). You are I-PY, the one corpus writer. When 07n is written, I-RS and I-TS update their harness pins and counts, by ROOT's message.

## Where

- **`WT/b1` after I4,** at `{I4 commit}`, where SP, SA and the three readers' rounds are merged.
- **CORPUS** (`P/fixtures/results/retained_precision_cases.json`) is append-only after B6's 07m. **Its 07m entries do not change.**

## 07n's content

**From PLAN_v2 §2.5:**
1. **Bases:**
   - `w_c2_sparse_interactive` and `w_c2_dense_scrutiny`, as D-U6-5 copies of I85's pinned W-C2 successors (the fixtures committed at I3: sparse `7922e3e5…`, dense `f2800bd4…`);
   - `d38_beside_selected`, synthetic and labelled "not producer-emittable under T-8", derived from a W-C2 base by a records script as §2.5 lists.
2. **Must-pass:**
   - the two W-C2 bases, which also pin RV113 N-1's (4a) positive witness;
   - `d38_beside_selected` with its invocation (G0–G8 pass; standing `needs_recompute`);
   - L = 0's entries, unchanged.
3. **Mutations:** D38 m1–m8; F-1's five; and `not_required` on W-C2 case B, three entries.
   - **The non-null `product_attempt_ref` entry is built with case B's own product attempt,** so that it reaches G5 `ATTEMPT` rather than stopping at G3 (RR "RV113 passes SR-RS with S-1; …", ruling 1).
4. **07n uses no tampered-preparation successor as a must-pass base** (RR "SP returned before I3; …").

**Routed since, each as entries with their designed first failure:**

5. **The producer's out-of-order-authored base** (RR "I98's B2-W verified; …", ruling 4): `cause_milestone_reversed`'s successor in both modes, a must-pass base. I85 pinned it at I3: sparse receipt `b79f691a…`, dense `c6b03683…`.
6. **SF-2's orderings:** the (C, B, A) and (A, A2) successors (I85's I3 pins) as must-pass bases. Note that the reader accepts RV109's M15, so 07n cannot pin staging order. It stays the producer's test.
7. **The three-reader alignment set** (RR "RV113's three returns verified; …" and "I91's and I92's rounds verified; …"):
   - (f)'s family, bound and unbound, at G3 COVERAGE, and the ordinary basis reference at G5 ATTEMPT. 07m's `integral_float_integers_and_references` stays a must-pass.
   - (g)'s model-scope members at G8 INVOCATION: `reference_configurations` null and `[]`; `pressure_contract` `{}` and `false`; `combinations` `{"x":1}`, `[{}]` and `"x"`; and `components` `"x"`.
   - **Each C2 cause branch, satisfied and broken,** including every `precondition` key with a wrong code and a wrong phase. No 07m entry or probe carries a precondition cause.
   - **The transport header at G2,** with the metadata at G7. Use RV108 N5's header branch sub-conditions, each pinned, including a per-reader `carrier_evidence` entry.
8. **S-2's (b) and (c) entries:** use inputs whose first failure is the same in all three readers after the alignment set. Use I91's REPAIR_02 §6 for the three that differed.
9. **I91's false-accept repairs** (RR "I91's SR-PY verified; …"): an entry for each of (a), (b), (c), (d1) and (d2), expected the same in all three readers.
10. **I90's first-failure notes** (RR "I90's SR-RS verified; …", ruling 3): m4 at G3 COVERAGE, m7 at G5 ATTEMPT, m1's whole error value, and a copied parity row's `recovery_method`.
11. **RV108's notes on B6** (RR "Owner decision: SI1c is option D, …"):
    - corpus entries for N1 and N2 (PY's type and key guards) and N4 (TS's `null` raw row on transport);
    - N3, declared with I83 §7 item 6;
    - N6(b): the case file's transport scope sentence.
12. **I83's notes** (RR "I86's SW probe accepted; I83's B6 return verified and ruled; …"), as per-reader entries with scope sentences, and no reader code change:
    - §7 item 6, a hash-consistent change of `formulation_basis.limitations`: PY and TS give `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`, and RS gives `SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS`;
    - mutant T9's two-defect order;
    - §7 item 8: pin Rust's `slice_outcomes` ids in the case file.
13. **Declared per-reader differences** stay declared, each with its scope sentence:
    - entry 139 `g7_maximum_off_enclosure`'s G7 code;
    - the compound N6 probes' raw G7 codes.

**Expectations.** Each first failure is fixed from the three readers' agreement on the I4 head. A disagreement is declared per reader only by ROOT's ruling. Return to ROOT with any that are new.

## Acceptance

- All three readers pass 07n: RS, PY and TS, run on the I4 head.
- **Counts move only by added entries.** The 07m census is unchanged on every verdict.
- 07n's sha256 is recorded, and the D38 base's records script and its reseal are kept.
- **PY and TS check SP's several-notice bytes** (T-12: `final/t12_bytes`).
- **Then:** I-RS and I-TS update their harness pins and counts. ROOT sends them the 07n hash. RV113 (RV-R) reviews SC.

## Host

- As B1_COMMON:
  - every cargo goes through `WT/tools/t3_cargo.sh`;
  - heavy pytest, vitest and test binaries go through `WT/tools/t3_slot.sh <command>`;
  - pytest under `P/tests` sets the checked binaries to your own builds.
- **One heavy job of yours at a time;** one wait per job, ending when its process has gone.
- **Paths:** absolute paths only, with your shell working in your own scratch (`WT/scratch/<id>_b1_sc/`).
- **Records:**
  - no symlink, and no folder named `build`;
  - remove the `hostname` attribute from junit output;
  - screen with the strict pattern and the machine's host name, decompressing `.gz` files;
  - run `git status --ignored`, and force-add any sealed file an ignore rule hides.

## Output

- **Commit** on `codex/piping-t3-b1-20261007` in `WT/b1`. ROOT pushes.
- **The record:** `R/<id>/b1_sc_01/RETURN.md`, with `_run_records/` and SHA256SUMS. It lists:
  - every 07n entry, with its kind, base, mutation and first failure per reader;
  - the declared differences;
  - the census;
  - 07n's sha256.
- **Budget:** 6–9 h.
- **End your turn with:**
  - the head;
  - RETURN.md's sha256;
  - 07n's entry counts by kind;
  - any new disagreement for ROOT;
  - the acceptance results.
