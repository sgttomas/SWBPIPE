# I7: implement facade slice F1 (sparse wiring, W2 at formation, the D-5 evidence line, SUP-17)

This is an implementation TASK. Read `_COMMON.md` first; this brief overrides it where they differ.

## Status: a dependency question for ROOT before spawn

The design's §6 table places F1 **after S11-F** on the facade path, but its write set **consumes two kernel slices that have not landed:**
- **"the kernel's sparse assembly"** is K1 (W3 kernel sparse representation and sparse M03 gate);
- **"K2b's formation-time scaling"** (SCALE-W) is K2b.

The kernel order is S11-K → K3a → K-D5 → K2a → **K1 → K2b** → K5. So the core of F1 cannot start until K1 and K2b are on main.

**Proposed split, for ROOT's ruling:**
- **F1a (can start after K-D5 merges; facade only, with no kernel dependency):**
  - (i) **the D-5 evidence line.** K-D5's `FormationCheck` record, present only when the check demotes, is rendered as one evidence line in the integrity diagnostic (§4.3.1, D5C-3, §5 item 5a). It applies only to demoted cases, so no committed byte changes (to be confirmed by the fixture diff).
  - (ii) **SUP-17,** a message text change only (§4.9): `PP:1604`'s "missing global rigid-body DOF classes" becomes the proposed text. One existing test pins the old text. Any committed fixture carrying the old text changes, which is a fixture-stop-rule item needing ROOT's regeneration approval.
- **F1b (after K1 and K2b):** the facade sparse wiring and W2 at formation, as the rest of this brief describes.

**If ROOT keeps F1 whole,** this brief is used unchanged once K1 and K2b are on main. **If ROOT splits it,** F1a's items are the §F1a write set and tests below, and F1b is the remainder.

## Basis (read in this order)

1. `T3/DESIGN_NUMERICS/DESIGN.md` revision 5a.2 (`fb62ef4a…`; hash-pinned):
   - the F1 row of §6;
   - §4.7 (W2: the normative b-rule, formation-time scaling, unscaling for publication, the evidence line `range_scaling: …`);
   - §4.8 (W3: the facade after T1, the parity protocol, the dense-scrutiny resource guard);
   - §4.9 (SUP-17);
   - §4.3.1 and D5C-3 (the D-5 evidence line);
   - §5 items 5a and 6.
2. `T3/ROOT_RULINGS_V1.md`: the K-D5 sections, including the M31b withdrawal and the large-coordinate finding; the S11-G sections (the integrity diagnostic, the no-op rule, routing, D22-1); and any ruling on this brief's split.
3. The K1 and K2b records on main, once they land: their kernel APIs (the sparse `AssemblyEvidence`, `SparsePattern`, the scaled-formation entry taking b).
4. S11-G's `formation_guard.rs` and `append_integrity_report` on main. F1's evidence lines join the same integrity diagnostic and must respect its byte layout and no-op rule.

## Base, worktree and branch

- ROOT creates the worktree from main after the dependency (per ROOT's ruling) has merged.
- F1 is its own PR with full gates: complete-diff independent review, hosted CI with the dispatch, a clean DEC-025 sweep, and the committed-fixture diff with its stop rule.
- No Git writes. The manager commits.

## Write set

**F1a** (the D-5 evidence line and SUP-17):
- `PP`:
  - where the integrity diagnostic is composed, render K-D5's `FormationCheck` evidence as one line, only when present, after S11-G's guard sentence, if any. The ordering and the no-op rule are S11-G's;
  - the SUP-17 message text at the `PP:1604` site (re-locate it);
  - the one test pinning the old SUP-17 text.
- No change to `StructuralReport` (D5C-3), and no new field, code or enum value.

**F1b** (sparse wiring and W2 at formation), from the design's F1 row:
- `PP`:
  - assembly (`:1620`, `:1751` in the design) moves to the kernel's sparse assembly with K2b's formation-time scaling;
  - the reduction partition maps (`:2330-2342`);
  - reactions from sparse rows (`:2697`);
  - `solve_preview_reduced_system` takes the pattern;
  - the `range_scaling: force_scale_exponent=<b>; basis=exact power-of-two` evidence line, only when b ≠ 0;
  - unscaling for publication per §4.7 step 5 (subnormal outcome stated; underflow or overflow makes the case UNRESOLVED; never flushed).
- `source_recovery.rs`: refuse scaled evidence.
- The nonlinear loop in `nonlinear_integration/src/lib.rs`, **with T5** (ROOT serializes T5's area). If T5's owner has not agreed, stop and ask the manager before touching it.
- **Enumerate every caller** of every changed function by lexer scan (`_run_records/callers.txt`).

## Tests

**F1a:**
- a K-D5-demoted case (for example RF-SKEW-T-CANT-OFF-122-r1e-04, both entries and both modes) carries exactly one D-5 evidence line;
- a non-demoted case carries none;
- a case demoted by both S11-G and K-D5 has the correct composition;
- SUP-17's new text;
- the committed-fixture diff: expected unchanged for the evidence line. Any SUP-17 byte change triggers the stop rule.

**F1b** (from the design's F1 row and §4.8):
- the full product suites;
- **the PHYS-R4 public fixture;**
- **LEF-small and LEF-large solved** (they were refused by K2a until now);
- **the parity protocol** of §4.8: bitwise coalesced K between the pattern and dense assembly for every fixture; M03 outcome-class parity on N01–N09, R01–R07, NP-B, NP-D, the T0R references and the R1 families; published quantities within the DEC-053 basis; a two-modulus-basis model; the nonlinear gap, one-way and friction models; relabelling and permutation;
- **the dense-scrutiny resource guard** (ROOT picks the ceilings from measurement);
- the both-entry gate against main's empty lists, two-part, with zero trusted breaches;
- mutations of the design's §7.3 that touch F1 (for example the b = 0 bit-identity, and unscaling), and your own;
- the committed-fixture diff with its stop rule.

## Disclosure, return and running things

These are as in `I6_K2A_IMPLEMENTATION.md`: CHANGE_RECORD and RETURN under `T3/IMPLEMENTATION/F1/` (or `F1A/` and `F1B/` if split), sanitized records, SHA256SUMS, GEN-8, the cargo token from the manager, one heavy job at a time, the disk floor, clean mutation targets, no rewriting of committed evidence, and no skipped tests or raised timeouts.

## Addendum 1 (ROOT, 2026-09-27): the LEF expectation for F1b is unreachable as written

I6 (K2a) found that **RF-RANGE LEF-small and LEF-large never reach `local_stiffness` at product level.** LEF-small is refused earlier on geometry (DegenerateAxis, L < 1e-12 m), and LEF-large is refused at capture, consistent with P1. So the design's F1 test "LEF-small and LEF-large solved" (the F1 row of §6, and the F1b tests above) **cannot be met as written at product level.**

**Before F1b spawns,** this brief must restate the LEF expectation at the level where it applies. That is either:
- kernel level, through K2b's scaled formation; or
- whatever product-reachable formation-range case K2a's product-reach probe establishes (section or material values with normal geometry).

It is on the T3-close decision list. F1a is unaffected.
