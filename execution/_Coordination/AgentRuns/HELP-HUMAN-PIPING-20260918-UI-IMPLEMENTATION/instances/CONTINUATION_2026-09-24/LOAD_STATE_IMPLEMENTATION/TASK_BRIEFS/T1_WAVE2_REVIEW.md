# TASK brief — T1_WAVE2_REVIEW (independent review: T0R merge obligations, WP2 native, WP4 headless, WP2 desktop readers)

Read `_T1_COMMON.md` first, including its **Wave 2** section. Return folder: `LSI/T1_WAVE2_REVIEW/`.

You are a **fresh-context, non-author reviewer**. You wrote none of the reviewed bytes, and you delegate nothing.

## Candidate

The branch `codex/piping-load-states-20260925` at the commit named in the spawn request. Review:

- `a9528b2e1`: the T0R merge, as far as T1's S6 §6 obligations go (`CHECKPOINT_6.md`). This covers:
  - the fresh set in three languages;
  - the standing order;
  - routing of blocked 0.4.0 envelopes;
  - `preview: None`;
  - the schema branch order and the identity-keyed pins.
- `c5ec4dcba`: WP2 native persistence (`apps/desktop/src-tauri`).
- `61fb7b219`: WP4 headless and CLI tests.
- The WP2 desktop readers commit named in the spawn request (`apps/desktop/src/**`).

## Basis

- Returns:
  - `LSI/T1_WP2_NATIVE_PERSISTENCE/RETURN.md`;
  - `LSI/T1_WP4_HEADLESS/RETURN.md`;
  - `LSI/T1_WP2_DESKTOP_READERS/RETURN.md`;
  - each return's manager integration note.
- Rulings: `LSI/T1_WAVE1_RULINGS.md` §7, and §11–§13 with the ROOT addenda (the T6 routing and the validation-path condition).
- T0R design: `DEFAULT_ROUTE_DESIGN/DESIGN.md` §10 and `DEFAULT_ROUTE_DESIGN/IMPLEMENTATION/S6_RECORD.md` §6.

## Review

**Explicit items (ROOT asked for these).**

1. **The 32 surviving desktop mutants** (WP2 desktop RETURN §4.1).
   - Check every claim for the 8 reader mutants (7 in the load-reference-1 reader, 1 in the joined reader): is each really unreachable or behaviourally equivalent?
   - For the 24 ledger-port mutants, a reasoned sample is enough: at least one per stated reason (the closed receipt schema, the recipe enum, `outcome` const, criterion bound, pre-emption by an earlier check). Say which ones you sampled.
   - For every mutant you find reachable, give a concrete killing input. It gets a test before the PR.
2. **Solver-mode custody on receipt-less identities (ruling §13).** Confirm or refute the manager's probe (`T1_WP4_HEADLESS/_run_records/manager_mode_relabel_probe.*`). Is the relabel really accepted for physics-1 and preview-physics-1 as well as load-reference-1? Is there any binding that load-reference-1 alone lacks? If load-reference-1 alone lacks one, that is BLOCKING for T1.
3. **Desktop output refusals (§12, routed to T6).**
   - For each panel still refused (RETURN §5 (a), less the three reclassified below, plus stress-neutral and result export), is result data (values, evidence, hashes or the raw envelope) really written into a file, package, handoff or external request?
   - For each panel left unchanged, is it really model- or geometry-only, or display-only? This includes the three panels the manager reclassified as identifier-only: review geometry, operation diff preview and operation review ledger. For those three, check that the value-invariance tests prove what they claim.
   - Does any desktop surface still let load/reference-state result data out?
   - Does the refusal text ever call the result invalid or unsupported?
4. **Validation path unaffected (ROOT condition).** The WP2 desktop commit must not touch any of:
   - the headless runner or CLI;
   - `qualification_load_reference.py`;
   - `core/handoff/stress_neutral`;
   - any Python.

**General.**

5. **Readers.**
   - Is the TypeScript load-reference-1 reader a faithful check-for-check port of `core/analysis_runs/load_reference_evidence.py`?
   - Is the joined reader a faithful port of `load_reference_source.py`, including the invocation-free ledger port (ruling §12; the check table is in RETURN §4.3)?
   - Can a relabelled or cross-profile header dispatch as a load-reference identity?
   - Is standing correct in TypeScript: the joined result is always `needs_recompute`, placed after validation, and load-reference-1 goes through T0R's standing unchanged?
   - Are the parity logs (438/438 and 282/282) honest and complete?
6. **Lossless desktop types and projection.** Do 0.4.0 documents and results round-trip byte-exactly, or canonically where RETURN says so, through every projection point?
7. **Saved results.** Does a change to the reference basis or `analysis_state` make a saved result not Current?
8. **Native persistence.**
   - Is 0.4.0 accepted with nothing injected?
   - Are 0.4.1 and later refused?
   - Is a pre-0.4 document that carries 0.4.0 keys retained as authored?
   - Does the browser mirror match native for 0.4.0?
9. **WP4.**
   - Do the tests prove what they claim, including the §11 joined characterization, blocked routing, the pre-0.4 routes and the SF-1 fallback?
   - Are the artifact lanes honest, meaning written from actual solves?
10. **Merge obligations.** Is the standing order validation → T0R standing reason → the T1 joined early return in all three languages? Is any 0.4.0 envelope, blocked or solved, able to carry a non-T1 identity?

## Checks you may run

- Desktop vitest: the named files, or the full suite. The spawn request names a sibling `node_modules` to symlink; remove the links before you return.
- headless and result_export `cargo test`, and the Python reader suites.
- The `src-tauri` suite (it builds on this host).
- Your own probes and mutants, on a scratch copy only.

Use the shared target named in the spawn request with `CARGO_INCREMENTAL=0`. Check `df` first. Delete any private target you create. Make no edits in the worktree.

## Return

`LSI/T1_WAVE2_REVIEW/RETURN.md`:

- the verdict: CLEAR, FINDINGS or BLOCKING;
- a findings table: severity, location, concrete failure scenario, suggested repair;
- per explicit item 1–4, what you verified and how;
- limits.

Then `SendMessage` the manager.
