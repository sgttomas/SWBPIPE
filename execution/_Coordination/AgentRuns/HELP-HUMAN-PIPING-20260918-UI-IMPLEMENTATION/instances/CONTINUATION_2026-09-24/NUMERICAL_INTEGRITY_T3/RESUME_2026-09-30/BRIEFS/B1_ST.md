# B1-ST: T-4 at c = 1, decision 21, `NoTriggeredCase`, the re-basings, and the SA–SP seam

Read `R/BRIEFS/B1_COMMON.md` first.

**The specification** is PLAN_v2 §2.1, with §1's fence and the source-text guards, and §8's risks that apply to ST.

**Your worktree:** `WT/b1`, branch `codex/piping-t3-b1-20261007`, cut by ROOT from main `47a3bdfcf5`. You are I-P, and you will also own SP after ST.

**Also carry:**
- **RV107 A1-N-2:** the seam field `ProductCapture::late_loads_total` records no adapter event. Adapter counts are serialized into the receipt, so an event would change every c = 1 successor's bytes. Gate-state writes such as `late_refusal` record none, which is the precedent.
- **Verify with the source-text guards early** (§1): `s11f_site_test.rs`, PP-tests' `retained_precision_admission.rs`, and RE's carrier test.
- **Run the outputs with no product attempt in the Stale build too;** every output there is the plain bytes.

**Acceptance:** as PLAN_v2 §2.1 states:
- byte identity of every committed c = 1 successor pin, in both modes;
- the registered PP suite differs from base only in the listed tests;
- the guards pass;
- the classifier's unit tests pass;
- the five mutants are each killed by an assertion.

**Checkpoint R3.** Return when ST is complete, before any review. Give:
- the head and commits;
- the byte-identity evidence;
- the suite differences against base `47a3bdfcf5`;
- the guard results;
- the mutants;
- anything for ROOT.

RV-P round 1 follows, then I1.

**Budget:** 5–7 h.
