# W5 Dispatch Transcript — DEL-09-04 Reproduction Wave (R14)

**Recorded by:** WORKING_ITEMS (Agent 1, W5 manager), 2026-07-20 (UTC)
**Provenance:** dispatch received from HELP_HUMAN (Agent 0) at W5 session
start. Recorded here as a faithful transcript for chain provenance (the
W4 brief-verification defect D1 established the durable-dispatch-artifact
expectation). Machine-absolute filesystem prefixes are redacted to
`<machine-root>`; no other content is altered. Template placeholders in
the original (e.g., a bracketed UTC segment inside the run-ID form) are
part of the dispatched text, not live path anchors.

---

You are the DEL-09-04 REPRODUCTION WAVE MANAGER (governed Agent 1,
WORKING_ITEMS role, node W5) of campaign run
`HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` in the chirality-piping
work loop, reporting to HELP_HUMAN (Agent 0). Children via your Agent tool;
durable artifacts are the authoritative reply channel.

REPO_ROOT = `<machine-root>` (the active worktree checkout)
WORKING_ROOT = REPO_ROOT/projects/chirality-piping
Branch `claude/piping-r14-w5-clean-repro` at `a5235340a` (post-wave-3 main;
this SHA is the natural SOURCE_COMMIT). Offline only; missing prerequisite
= truthful blocked result. The worktree carries offline provisioning
(node_modules, per-crate Cargo.lock files) — a clean CLONE of this repo
will NOT have the gitignored lockfiles; the R11-precedent brief handled
this (clone from local REPO_ROOT only, CARGO_NET_OFFLINE, offline cache);
if the clean clone hits the missing-lockfile class, the recorded precedent
`HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12/ENVIRONMENT_REPAIR_DISPOSITION.md`
(offline copy of ignored state from the primary tree) may be adapted ONLY
as a recorded, offline, ignored-paths-only step inside the executor's run
record.

## Objective

One fresh actor-neutral clean-checkout reproduction of the CURRENT
documented validation-manual procedure at SOURCE_COMMIT `a5235340a`,
producing a new immutable `INTERNALLY_VERIFIED` bundle under
`validation/evidence/reproduction/` (run-ID form
`REPRO_DEL0904_` + UTC stamp + `_` + 12-hex source SHA), restoring
current-head R6-criterion evidence after the R14 mechanics waves. This is
triggered by the R11 brief's §8 rerun clause (procedure + runner + solver
changed); that brief's pinned predicates are stale, so you author a NEW
brief.

## Required grounding (live tree wins)

- The refreshed manual `docs/validation_manual/headless_runner_reproduction.md`
  (post-T7: dated case-1 and case-3 notes; Part 2 bound-path section with
  the five `del1005_payload_binding_*` cases). The reproduction executes
  ALL documented cases: the three frozen tp_runner_015 E1 cases with
  their CURRENT expected outcomes (case 1: exit 0 COMPLETED, non-empty
  result_refs, with the documented non-blocking
  `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` warning — NOT diagnostic-clean;
  case 2: exit 1 `HEADLESS_RUNNER_LOAD_BASIS_MISSING`; case 3: exit 1
  `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` per the dated note) plus
  the five bound-path cases (exits 0/0/0/1/1, outputs byte-compared to
  committed witnesses).
- The R11 precedent brief
  `execution/_Coordination/CANDIDATE_BRIEF_2026-07-18_DEL-09-04_CLEAN_REPRODUCTION.md`
  — reuse its §3 execution mechanics (mktemp clone from REPO_ROOT only,
  detached checkout, cleanliness proofs, offline cargo, separate
  argv/stdout/stderr/exit capture, immutable bundle layout §3.5, checksum
  discipline) and §4.2 profile-check list; correct only what the current
  tree changed. DEC-080 fixes the evidence home. R11 bundle
  `validation/evidence/reproduction/REPRO_DEL0904_20260719T202023Z_23eeaabc9040/`
  stays immutable.
- D-52 overlay + D-54 packet for the classification; §10 attribution per
  the R14 wave briefs (HELD → EFFECTIVE after your verifier).

## Chain

1. YOU author
   `execution/_Coordination/CANDIDATE_BRIEF_2026-07-20_DEL-09-04_CLEAN_REPRO_R14.md`
   + D-54 rationale under `instances/W5/` (10-class screen; the
   reproduction performs NO acceptance — bundle label
   `INTERNALLY_VERIFIED` only; enumerated claims incl. every expected
   exit/diagnostic anchored to the live manual + witnesses).
2. Fresh-context adversarial brief verifier child → COMMIT-SAFE (cure or
   park on BLOCK).
3. §10 EFFECTIVE (DEC-085/D-52 §2, SHA
   f14fa77518a06f112ae72a8fcce4de0fab958d47, OwnerCaseSelection NONE) →
   executor child: execute the reproduction per the brief (clean clone,
   offline build, all documented cases, review checks per brief, bundle +
   SHA256SUMS, profile-selected registered checks incl. the single
   evidence-sweep for `validation/**` writes with one-file-delta proof;
   DEL-09-04 History + MEMORY + one run record; strike NO Remaining
   rows).
4. Fresh-context implementation verifier child (verify bundle internals,
   hashes, predicate verdicts, containment, frozen-surface byte-identity)
   → COMMIT-SAFE.
5. YOU commit (`evidence(piping): DEL-09-04 clean reproduction at R14 head
   (R14-W5)` + the model co-author trailer), then wave closeout:
   remaining registered checks (piping-pytest, harness-pytest,
   harness-self-check; JSONs to `instances/W5/`), commit any residue as
   `chore(piping): R14-W5 wave evidence`, manager RETURN at
   `instances/W5/RETURN.md` (dispositions, tallies, pointers, enumerated
   wave claims, model attribution — children on claude-fable-5). No
   push/PR/merge/rebase/receipt.

FAIL/BLOCKED at any case = truthful failed/blocked bundle per the
precedent's §7 (keep it, no repair in-tranche, return to HELP_HUMAN).
Final text: disposition with SHAs, bundle RUN_ID + overall_status, check
tally, sweep filename, manager-return path. Claim-calibrated; standard
claim fence sentence in every durable narrative artifact; the bundle
README must state adoption basis and that no owner acceptance occurred.

---

**Manager notes (not part of the dispatch):** the dispatch's `a5235340a`
resolves to `a5235340aae3c41cf227f5617e593b268936f6b3` (merge of PR #294,
the third merged R14 wave; branch up to date with origin/main; tree clean
at session start). Receipt cursor at session start: `Receipt-63`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
