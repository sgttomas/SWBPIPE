# I61: scoping the shortest path to one real retained-precision receipt

**Recommendation: a disposable projection experiment.**
- **What:** a test-only emitter, run in archive copies, turns the existing private prepared
  producer's actual output for RF-SKEW-T-CANT-OFF-122-r1e-04 into receipt JSON for both modes.
  Each of the three readers' existing draft validators then reads it.
- **What it records:** a field-by-field provenance map and every reader mismatch.
- **Cost:** about one bounded TASK, roughly 3–5 h of agent time in two grants. Host load is
  negligible.
- **Why not the real serializer:** it cannot be finished now. It needs the facade wiring and a
  permit/M that handoff step 4 still blocks, so starting it would leave the premise untested the
  longest.

This was a read-only scoping run with no code, Cargo, solver, native job or Git write. It ran from
2026-10-04T00:38:37Z to 00:48Z. The memory guard (PID 5387) was running.

**Sources:**
- **NUM (producer):** `175dad6e67`, read through `git show`.
- **READER:** `b36739112a`, read through `git show`.

**Path abbreviations:**
- P = projects/chirality-piping
- PP = P/core/product_physics/src
- FK = P/core/solver/frame_kernel/src/structural/retained
- PY = P/core/analysis_runs/retained_precision.py
- RS = P/core/reporting/result_export/src/retained_precision.rs
- TS = P/apps/desktop/src/features/results/retainedPrecision.ts

All file:line references are to NUM unless marked READER. The basis files and their hashes are listed at the end.

## 1. What already exists on the producer side (NUM `175dad6e67`)

| Piece | Where | What it gives the receipt |
|---|---|---|
| Invocation capture and digest | PP/source_receipt.rs:102–135 (`CapturedInvocation::parse`), hash helper :33–41 | The raw request, the mode, and H(`source_blocks_invocation_v1`, `{request, solver_mode}`) over canonical JSON. These are exactly the domain and shape READER's G8 recomputes (READER PY:1346–1349). This is `body.invocation`. |
| Public facade | PP/lib.rs:2161–2229 (`…_with_mode`, `…_with_retained_direct`, `…_with_retained_headless`, dispatch) | The ordinary envelope plus a non-wire `RetainedAdmissionReport`. **No `ProductCapture` is ever installed on this path**; the prepared producer is reached only from tests. |
| Admission/memory census | PP/retained_memory.rs:1–2 ("no registered production profile or constructible capture permit"), :212–237 (`MissingAdmissionTerm`, `ProfileStatus::Missing`), :336–337 (`Unselected`) | Evidence only. No permit can be minted. |
| Prepared private producer (test-reachable) | PP/retained_product.rs:3265 `prepare_observed`, :3272 `solve_native` (`RecordedInvocation::new(60e9)`, `CaseLimit 20e9` at :3282–3283, matching the readers' G0 work limits), :3456 `project_candidate`, :3337/:3341 refusal/candidate | The overlay envelope, the certified proof and the native owner for the milestone case. It passes both modes (I51 prepared_producer_completion_04 RETURN:3; I61 `i61_ready…` both modes). |
| Capture facts | PP/retained_product.rs:81–135 (`ProductCapture`) | Node, member, material, selection, term, support and spring maps; `facts`; `source`; `native` (RecordedInvocation, RecordedCase); `operational`; `verdicts`; `g5a_work`; `adapter`. These are the raw material for CaseSource, MaterialBasis and ProductAttempt. |
| Typed C3 trace | PP/retained_receipt.rs:1 ("No serializer, public receipt…"), :102–132 (`SummaryCoverage` and its rules), :134–147 (`PreparedAttemptView`), :148 `project` | Per attempt: stages[10], checks[3], old/new operational, preparation members/work, proof trace, completion, adapter snapshot, overlay/g5a work and summary coverage. Reached through `typed_trace` at PP/retained_product.rs:3606–3634. |
| Proof trace and coverage | FK/product_certificate/final_case.rs:232 (`ProductProofTrace`, the borrowed `summary_coverage`), :263 `rederive_coverage`, :315 `check_summary_coverage` | `ProofTrace` lanes, numeric, LME, visits, conversions, projection outcomes, capacities and coverage. |
| Kernel origins | FK/origins.rs:147 BuildOrigin, :183 RunOrigins, :199 RecordedCase, :216 GroupOrigin, :253 CallOrigin, :301–346 `RecordedInvocation` (`meter`, `calls`, `sources`, `groups`, `builds`, `runs`) | `calls`, `groups`, `builds`, `Run.origin`, cache before/after, and `work.charged` and `execution_order`. |
| Selection evidence | FK/adaptive.rs:3660–3696 (`RetainedEvidence`: attempts, stop_rule, floor_ratio, body_scales, input_derived_dofs, resolution_scale, estimate, charge, theta, certified_bound, floor, `ledger_encoding`, `retained_state_encoding`), :2730 `AttemptRecord` | `selection.*` and the physical records. Per C2 (I32 f2a_wire_c2 CONTRACT_DELTA:91), `ledger_sha256` is the raw SHA256 of the K4LED bytes. |
| Native source bytes | FK/source.rs:777 `encoding` (K4SRC), :812 `stiffness_encoding` (K4STF) | `kernel_source_sha256` and `stiffness_sha256` as raw SHA256, which READER G8 recomputes (READER PY:1394). |
| Base semantic id | PP/preview_physics.rs:13 (`preview-physics-1`) | The base contract that the readers' G7 projection must satisfy. |

## 2. What is missing to emit a receipt matching READER's schema

The schema is READER `P/schemas/retained_precision_mp_v2.schema.json`, sha256
`07951edacfedd410c153929ee75bb5bada15dbd222369ec63240c678b233b61c`. Its top level is
`{body, receipt_sha256}`. `Body` has 18 required members, and the schema defines 66 types.

**A. No serializer or envelope member.**
- `MechanicsEnvelope` (PP/lib.rs:814–832) has no `retained_precision` member.
- No code maps the typed seam, the capture or the kernel evidence to JSON.
- No code computes H(`retained_precision_receipt_mp_v2`, body) or the publication hash
  H(`retained_precision_publication_mp_v2`, envelope minus the member) (READER PY:1553–1554).
- No code computes the source identity H(`retained_precision_source_mp_v2`) (READER PY:306–307)
  or the preparation hash H(`retained_precision_preparation_v1`) (READER PY:362–368, 1568).
- The canonicalizer itself exists: `canonical_json_checked_v1_text`, through `source_receipt::hash`.

**B. Envelope transformation.** The readers require the following, none of which the producer emits:
- `producer.semantic_contract_id = …/preview-physics-retained-1` and `formulation_basis.profile_id = product_preview_retained_w1a_v2` (READER PY:19–20, 1531);
- `recovery_method = contribution_preserving_multiprecision_v1` on every row of a selected case, and none on any other row (G6, READER PY:1623–1625);
- exactly one `RETAINED_PRECISION_SELECTED` or `RETAINED_PRECISION_UNAVAILABLE` diagnostic per case, with `affected_refs == [case_id]`, and no `SOURCE_BLOCK_RECOVERY_SELECTED` (G4, READER PY:1611–1621);
- `product_attempts[].definition_id = RP-PREPARED-ORDINARY-DUAL-v1`.

The producer also needs the fixed contract artefacts. They exist **only in READER**, not in NUM:
- the definition fixture `retained_precision_prepared_ordinary_v1.json`, whose formation hash must equal `DEFINITION_HASH`;
- the semantic table `semantic_contract_v0_3_preview_physics_retained_1.json`, whose sha256 must equal `TABLE_HASH`;
- the schema itself.

**C. Body sections with no producer mapping yet.** These are typed facts that exist but have never been projected:
- `work` (`charged`, `execution_order`);
- `cases` (Case selected/unavailable/not_required with `run`, `selection`, `source_identity_sha256` and `ordinary.{attempt_ref, quality_binding}`);
- `Run` (records, attempts, charges, invocation_before/increment/after);
- `Reason` (the closed translation of AttemptReason, AttemptStop, Unresolved and Refusal; never Debug text);
- `sources` (CaseSource: id_maps, body_membership, layout, stations, supports, constraints, nodal_terms, section_terms, preparation);
- `material_bases`;
- `calls`, `groups` and `builds`;
- `product_attempts` (the `PreparedAttemptView` needs index assignment, CheckRef/FailureRef to `Check`/`ProductError`, adapter, operational and work);
- `combinations` (empty for this case).

**D. `ordinary_attempts`, partly unsourced.** `initial` (report or structural/formation failure), `w2`, `formation` (`load_row_finding`, `d5_diagnostic_ref`) and `legacy_source` have no typed capture in PP/retained_product.rs. Today they exist only as envelope diagnostics and contract_evidence. The emitter must read them back from the actual ordinary envelope, or new typed capture is needed. This is the most likely place for a provenance gap.

**E. Reader-ruling obligations.**
- **D6a** (ruling "Checkpoint A", ROOT_RULINGS_V1:8116): `ordinary.diagnostic_refs` holds exactly the case's actual ordinary diagnostics, each once. PP has no per-case diagnostic attribution at all (no `affected_refs` handling in retained_product.rs). The rule for invocation-level diagnostics that name no case is undefined.
- **D9b** (:8127): an unavailable case carries an explicit `source_ref: null` (`null | U`, with `source_decline` only alongside null). The milestone case is *selected* in both modes, so its receipt never exercises D9b. An actual unavailable receipt is reachable from an existing control: a preparation/helper refusal, which has no CaseSource (`prepared_trace_actual_helper_refusal…`).

**F. Handoff step-4 items that bear on this receipt** (HANDOFF_2026-10-03_TO_NEXT_ROOT.md:158–160):
- **Memory/profile/M:** no production profile, allowance or permit exists (retained_memory.rs). Step 4 says to finish the admitted receipt transaction only after M, and not to enable a permit on partially priced terms. A **public** emitter is therefore blocked; a test-only experiment is not, because it publishes nothing.
- **RV77-N4 owner binding:** proof↔owner is by custody only. `ProofAnchor::matches_owner` is `pub(super)` (FK/final_case.rs:1585), and `certificate` is a crate-visible field (PP/retained_product.rs:3339, :3342). The real serializer should bind the anchor (an accessor on `CertifiedProductProof`) and make those fields private. The experiment can rely on custody and record that limit.
- **Deferred producer-solved witnesses** (ruling 05b, ROOT_RULINGS_V1:7617ff): the native Ceiling row and the L = 0 base wait for real receipts. The milestone receipt does **not** supply them: it is single-body, p128/P256 and L ≠ 0. They need other cases after the first receipt.
- **The facade route itself:** the public facade never builds the prepared attempt (§1). Reaching the milestone through "the actual captured facade" (handoff:49–52) needs wiring that today is test-only.

## 3. Feasibility on this host (from existing records; nothing run now)

- **The native solve of the case is tiny.** The kernel sweeps that include RF-SKEW-T-CANT-OFF-122-r1e-04 report it `selected Some(128) 0.00s`:
  - R/I22/vk_f03_diagnostic_01/D01_NONE_BEFORE/stderr.txt:52, 67–84: the whole ~54-case sweep took 0.25 s real, with a maximum RSS of 10.96 MB and a peak footprint of 8.98 MB;
  - R/I22/vk_f17_diagnostic_01/D01_NONE_BEFORE/stderr.txt:22, 37–54: 0.07 s and 7.4 MB.
- **The full prepared producer path passes both modes in seconds (debug).**
  - I51 prepared_producer_completion_04 (RETURN:3): p = 128 / P = 256, both modes pass.
  - I61 ready test, both modes (R/I61/coverage_producer_01, 02_pp_i61_focus): four tests, including about 10 bisection reruns of the full path, took 8.5 s.
  - I50 first_publishing_component_02/EXECUTION.json:78: the named-case suite, 18 tests in 9.05 s.
- **The memory guard never tripped.** Its floor is 35% of 128 GiB, and its log since 2026-09-27 shows only its two start lines and no kills (WT/guard/memguard.log).
- **The gap:** there is no recorded peak RSS for the *facade/prepared* path in either mode, only test wall times. The experiment should capture `/usr/bin/time -l` for its two producer runs. Expect seconds and well under 1 GB.

## 4. The smallest bounded unit: a disposable projection experiment

**Goal:** test the premise that the readers' wire contract matches what the producer emits, using the producer's actual output.

**Deliverables:**
1. A provenance map: every schema member mapped to its producer source (file:line), or `MISSING` / `DERIVED` / `CONTRACT-ONLY`.
2. The emitted receipt JSON for sparse_interactive and dense_scrutiny, with the matching invocation objects. Optionally a third, an actual preparation-refusal receipt, to exercise D9b and the unavailable branch.
3. Each reader's draft verdict per receipt:
   - Python: `_validate_draft(source, invocation)`;
   - Rust: `retained_precision::validate`, whose eligibility stays false (RS:4090–4093);
   - TypeScript: the TS `validate`.
4. A mismatch ledger. For each mismatch, the first failing gate and code, its cause, and a classification:
   - **emitter bug:** fixed inside the experiment;
   - **producer gap:** a missing typed fact;
   - **contract tension:** escalated.
   
   Iterate until all three readers pass G0–G8 (standing `needs_recompute`, eligibility disabled), or until only escalated tensions remain.

**Write fence:**
- WT/scratch/<id>/: a `git archive` of NUM `175dad6e67` and of READER `b36739112a` (read-only Git), the disposable harness files inside those copies, the emitted receipts and the logs;
- WT/targets/<id>/{product_physics, result_export};
- NUM/R/<id>/ for the records.

It excludes NUM, READER, CODE and every live worktree, and makes no Git writes. Inside the PP archive copy, the emitter is a `#[cfg(test)]` module (one `mod` line plus one file). It drives `PreparedCase::prepare_observed → solve_native → project_candidate → typed_trace` exactly as `i61_ready…` does, then serializes from `PreparedAttemptView`, `ProductCapture`, `RecordedInvocation`/`RecordedCase`, `RetainedEvidence` and the envelope.

**Cost:**
- **Emitter:** about 800–1,200 lines of test-only Rust. Run/Reason, CaseSource and Ordinary are the bulk. The reader harnesses are about 30 lines each.
- **Two grants:**
  - (a) the provenance map and an emitter that passes G0–G2 in all three readers, about 2 h;
  - (b) G3–G8 iteration and the ledger, about 1–3 h.
- **Host:** one cargo job at a time; each producer test run is a few seconds plus incremental compile. Rust reader compile is minutes; Python and vitest take seconds.

**Decisions needed from ROOT:**
1. **The unit.** Approve the disposable experiment over the real serializer start.
2. **Permission to run** the milestone producer path in both modes (private prepared driver) and the three readers' draft validators, through disposable harnesses in archive copies. This is a small native solve under the guard.
3. **The route.** Accept that the experiment uses the test-only prepared driver, because the public facade never installs a `ProductCapture`. It tests the wire contract, not facade custody, admission or M.
4. **Provisional readings** the emitter may use, each to be recorded as an assumption, not a ruling:
   - `retained_state_sha256` = raw SHA256(`retained_state_encoding`). `ledger_sha256` = raw SHA256(K4LED) is already C2:91. I found no explicit retained-state rule in C1.
   - the D6a attribution for invocation-level diagnostics;
   - the source for Ordinary `initial`, `w2`, `formation` and `legacy_source` (reading the envelope back versus new capture);
   - fixture provenance: the experiment hashes READER's definition and semantic table, which NUM does not carry.
5. **The optional refusal receipt.** Include the actual refusal receipt (D9b/unavailable), at about +20% cost.

**Why not start the real serializer now.**
- It needs the facade wiring, an admission permit and M (step 4), the RV77-N4 structural binding, and the contract artefacts moved into the producer tree. All of these are gated or reviewed work.
- Its first reader contact would come at the end, so it would not test the premise early.
- The handoff (:164–166) also says not to commission a new serializer merely to bypass a known boundary. A disposable, test-only experiment bypasses nothing and publishes nothing.

## 5. Contract tensions already visible

1. **The readers' public APIs refuse by design.** `_IMPLEMENTATION_COMPLETE = False` (READER PY:29) makes `validate_retained_precision` reject at G0 (:1520–1522), and RS:4090 holds eligibility off. "Validated by all three readers" can therefore only mean the draft paths now, with standing `needs_recompute`. The closure condition in ROOT's workflow ruling should say which.
2. **The contract artefacts are split across trees.** The schema, definition, semantic table and corpus are in READER only. The producer at NUM cannot reference `DEFINITION_HASH`/`TABLE_HASH` without importing them. A cross-tree adoption is needed before any real serializer.
3. **The facade versus the private driver.** The milestone is defined "through the actual captured facade" (handoff:49–52), but the facade has no prepared route (PP/lib.rs:2201–2229). Either the facade wiring is in the receipt transaction's scope, or the milestone definition is narrowed. That is ROOT's or the owner's call.
4. **D6a has no producer basis.** PP has no per-case diagnostic attribution, and invocation-level diagnostics have no specified home. Readers check uniqueness and resolution, not completeness, so a wrong but consistent list would pass. Only the producer can be held to "exactly".
5. **The Ordinary members are untyped in the producer** (§2D). G5 ORDINARY binds them to the envelope's diagnostics, so the emitter must reproduce them from text-level envelope facts. If the readers' expected shapes assume typed native facts, mismatches will surface here.
6. **G7 projects the retained envelope back to preview-physics-1** (READER PY:1626–1636), with the overlay (prepared) values left in the rows. Whether the base contract accepts the overlay values and maxima patches unchanged is untested. Only synthetic envelopes have passed it.
7. **The milestone receipt exercises little of the hardened surface.** It is single-case, single-body, p128, selected. It cannot exercise D9b, unavailable/refusal branches, p512 floors, Ceiling, L = 0 or multi-body. The first real receipt therefore tests the core wire shape and hashes, not most of the hardened gate logic. Expect a second, refusal receipt to matter.
8. **Owner binding (RV77-N4).** A receipt built from a proof and a different owner with identical public facts would pass every reader (I61 `I61_FOREIGN_OWNER … ok=true`, asserted). The experiment inherits this limit. The real transaction needs the structural binding.

## Basis read (sha256 of the bytes at the named commit)

| File | Commit | sha256 |
|---|---|---|
| T3/ROOT_RULINGS_V1.md (rulings 7617–7638, 7694–7701, 7941–7951, 8104–8134, 8248–8269) | NUM 175dad6e67 | a0072223875402c0be0098390afb13edc8b5b2360854bdb884a250b69dfd4aff |
| T3/HANDOFF_2026-10-03_TO_NEXT_ROOT.md | NUM 175dad6e67 | dae9ca702734d380dd5e812e29c5d587632bc06ae5d49c077c79b09658badb57 |
| PP/retained_receipt.rs | NUM 175dad6e67 | 55dc8cb50526104186c944f9265f54395c3dc206e70fa7c43be2fc2982dee517 |
| PP/retained_product.rs | NUM 175dad6e67 | d07383fc026e61e494a2b0a307271eaf2eb0329c333da95533f4b39c5d52af1a |
| PP/source_receipt.rs | NUM 175dad6e67 | c2fce1458f646b866886b9e1ffae07569d34ea517c7a7309d6b8140522f305fa |
| PP/retained_memory.rs | NUM 175dad6e67 | 002ef8584f6db37572ddbbff760665539e3245d27b83b87b9f79d9a2fa3ffd99 |
| PP/lib.rs | NUM 175dad6e67 | 4fff1a331c754f666a0ddff2438418e02c90f19ec21d1a234cb65ed55985f594 |
| P/schemas/retained_precision_mp_v2.schema.json | READER b36739112a | 07951edacfedd410c153929ee75bb5bada15dbd222369ec63240c678b233b61c |
| PY (reader) | READER b36739112a | 9669b88359f0f40756d0d27d0aa503f2ddd35ce4ac1174363671e12b33c8611e |
| P/fixtures/results/retained_precision_cases.json (07a corpus, structure only) | READER b36739112a | a6fa398731c35245baca098e322a9846399c7535b2d4cd882a5058010de456c9 |
| R/I32/f2a_wire_c1/WIRE_CONTRACT.md §4, R/I32/f2a_wire_c2/CONTRACT_DELTA.md:86–96 | NUM 175dad6e67 | c8ab2318457bd3897e207888af80f54f1a889071bf630ff1815939d922a567e3 / 923da0b97eb5becad7e7c1373c5a6f362568dc28ac8eab029c9170677c890869 |

Also consulted, by grep and excerpt: FK final_case.rs, origins.rs, adaptive.rs and source.rs; RS and
TS (entry points only); I22, I50 and I51 execution records; and WT/guard/memguard.sh and its log.
No other role body or workflow body was loaded.
