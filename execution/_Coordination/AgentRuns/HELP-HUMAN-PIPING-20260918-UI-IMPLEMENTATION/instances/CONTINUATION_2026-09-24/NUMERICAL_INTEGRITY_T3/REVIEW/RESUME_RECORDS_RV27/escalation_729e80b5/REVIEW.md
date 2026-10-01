# RV27 — C17 escalation records review

**PASS for the records delta at 729e80b5c2a33b278623a850d9c75c25372b688c.**
No new actionable records finding; RV27-S1 remains closed. **AUD-T3-01 is correctly
recorded as BLOCKING and is not repaired.** The numerical defect does not become
a pass through this records verdict. Required exact-final-head CI/DEC-025 remain
ROOT's gates before PR1068 merges.

Reviewed the complete eight-file delta from
`ea2aaee702624cd612eee23a7e1969fd3c670d3f`, including the conflict resolutions in
ROOT_RULINGS_V1.md and WORK_GRAPH.md. The local checkout is clean and live PR1068
matches the reviewed head against main `a38617d08bfbff6ea0a15b1cb129a16cbc82b2fc`.
Together with the prior full review/backchecks, this covers the current PR diff.

## C17 confirmation

Independently decoded the pinned primitive binary64 load and spring bits with
standard-library exact fractions. The fixed probe constructs two axial nodes,
one straight member, free coordinates 0/6/9, and a ground spring k=2^-33 at DOF 0.
The separate axial loads sum to `(2^-900 + 9h) - 2^-900 = 9h`, h=2^-1074.
Adding the axial equilibrium equations cancels member force, hence
`u0=9h/k=9*2^-1041`. The other node-0 translations are constrained to zero,
so its displacement magnitude equals |u0|. Independent substitution satisfies
both axial equations and the torsional equation exactly.

The raw source-level run is Selected at 128/256. D:0 and M:0 are non-input-derived
Translation rows publishing +0, AbsoluteVerified, bound bits `0000001000000000`
(2^-1038). Their actual error/b is 9/8. R7 §5.2 line 344 and the existing
compare_honest implementation corroborate the 128/256 publication factor
`1+2^-22`; the qualified allowance is exactly `4194305*2^-1060`.
Error/allowance is `4718592/4194305 > 1`. The binary64 allowance computation is
exact here and fails too. This is a violation of the rows' qualified claim,
not merely a direct-rounding mismatch or a substituted benchmark criterion.

The exact result agrees with the independently frozen TRUTH.json. This review
used its own force-balance calculation, not the old comparator as a truth
producer. It did not run the solver or adopt a proposed remedy. The bounded
claim is an admitted, unmutated kernel PrimitiveSource false publication; no
native/product false-publication witness or equivalent-input exclusion is proved.

## Provenance, ordering and stopped scope

- Evidence is pinned to `a6b40d2d036acac556e28f28f5b482a4adb39333` and the retained
  remote A1 branch independently resolves to it. Its 207 changed paths are
  diagnostic records under R; maintained source is unchanged.
- I22's raw stdout, the oracle's unchanged copy, and original scratch stdout all
  hash to `841167ed887a71b0e2a904647e1f217758fad304baaaf0d669f5c47035dc797a`.
  Headers bind C17, source `3bddc2b...`, fixed 100000000/100000000 limits and
  selection 128/256; source encodings before/after selection match. The probe
  calls PrimitiveSource::new, solve_case and publish directly, with no synthetic
  row replacement. Independent source-encoding decoding was not performed.
- The current original binary rehashes to
  `bcbe897204ec702b99529d25e6d0213d0132af5e6086e0397fae3a8f8ef8a08f`.
  All 141 G0 scratch entries verify; all 115 archived FK source files match the
  pinned Git source. FK tree identity matches audit, original product base,
  current main, evidence commit and reviewed records head. Probe main/cases and
  original two-package lock match the immutable response inputs. Recorded
  compiler fingerprints have no enabled features or rustflags; the run clears
  fault/flag/wrapper variables. No rebuild was needed or attempted.
- G0, B and C seals verify 9, 101 and 11 entries. The oracle freeze, addendum 01
  and C17 addendum verify 12, 20 and 8 files; the C17 addendum manifest is
  `7f58726a3f7c8f24e5aee839cfa4c31855684d86aff33a8b23ad40c89609900f`.
  Frozen truth and comparator hashes are unchanged.
- The preserved read-order record explicitly places independent truth before
  solver outputs. The freeze timestamp is 04:55:38Z; G1's 05:02:14Z committed
  brief pins that truth/manifest; G2 is committed at 05:12:23Z; C17's recorded
  launch is 05:13:49Z. This independently checks the available ordering records
  and hash bindings, not unseen original conversation history.
- C17 workload exit 0 and comparison exit 1 are preserved. Only C17 appears in
  the committed C packet and original C scratch directory. C18-C24 are explicitly
  unrun; the sequence stops at the witness. The post-comparison wrong-path read
  is disclosed and does not modify/repeat the output. Guard observations are
  before/after liveness, not a newly audited continuous guard or process cap.

## Record consistency and conflict resolution

All prior ruling bytes, including RV27-S1's correction and disposition, are
preserved as a prefix. The prior graph's historical progression is intact;
its current status now explicitly says BLOCKING, preserves F2a's hold and K6c's
unaccepted status, and supersedes the initial G0-only execution state with
later G1/G2/C17. Earlier SHOULD-FIX statements remain historical. There are no
conflict markers or numerical-source, validation, tool, policy or evidence edits.
The four imported briefs match their intended records scope; G1/G2 match their
original grant commits. The design brief hash matches the actual-dispatch row.

The new design assignment permits a bounded proof proposal only, with fresh
review before reliance; no bound, cutoff, domain, contract or implementation is
selected. I21 continuation keeps E_max unaccepted and A1-sensitive. Native
list_agents confirms the actual named design, oracle and I22 execution paths.
The design agent has since returned a proposal; that later return is not reviewed
or accepted here and does not invalidate the historical status_at_record field.

Original review, correction-backcheck and mergecheck seals verify 17/10/11
entries. The prior preservation seal remains unchanged and verifies 293 entries.
Fresh GEN-8 passed on this exact clean final head: **1 passed, 10 deselected**;
original stdout/timing is evidence/gen8.log. No new review defect was found.

## Boundary

Only this additive subtree and owned scratch were written. No Rust/solver run,
repeat, source edit, Git/index mutation, delegation or host-tool engineering
occurred. Every Git read set GIT_OPTIONAL_LOCKS=0. This review does not repair A1,
accept a design, qualify a native workflow, prove K6c, or finish CI/DEC-025.
Later candidate changes still require review coverage. No personal owner-review
or model-diversity claim is made.
