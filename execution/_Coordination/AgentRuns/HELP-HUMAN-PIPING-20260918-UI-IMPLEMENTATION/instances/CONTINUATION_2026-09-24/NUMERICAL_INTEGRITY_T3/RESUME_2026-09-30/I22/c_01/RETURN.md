# I22 G2 — BLOCKING C17 false publication; tranche stopped

Exactly one C17 ran. The unchanged solver selected p128, verified at P256, with
1,065,658 LME and zero corrections. Workload exit 0; frozen strict comparator
exit 1. **D:0 and M:0 publish +0 with a false absolute bound.**
C18, C19, C20, C21, C22, C23 and C24 are **UNRUN**. No retry, repair, extra
witness gathering, build or unit experiment occurred. No owned job is running.

## Exact violation

Both affected rows are non-input-derived Translation quantities in body 0:
node 0 Ux displacement and node 0 displacement magnitude.

| Field | Observed / independently frozen value |
|---|---|
| Published value | +0, bits 0000000000000000 |
| Class | AbsoluteVerified |
| Published b | bits 0000001000000000 = 2^-1038 |
| Exact truth and absolute error | 9·2^-1041 |
| Error / b | 9/8 |
| Accepted p128 claim factor | 1+2^-22 |
| Qualified allowance | 4194305·2^-1060 |
| Error / qualified allowance | 4718592/4194305 > 1 |

Both exact and source-binary64 predicates fail for both rows. Scale, class,
grammar and remaining applicable row predicates pass. Full comparison covers
36 rows; four RelativeVerified rows pass their relative predicates. Spring
S:1:0 also differs from direct truth but remains within its own claimed bound.
This is an unmutated admitted-source false publication, not merely the earlier
abstract proof example. It is not a product/native application witness.

C17_INPUT.json preserves the fixed raw primitive bits and separate load IDs.
The model has L=2^100, E=G=Iy=Iz=1, A=2^176, J=2^102, spring k=2^-33 on
global DOF 0, free DOFs 0/6/9, and zero constraints elsewhere. Separate loads
are F=2^-900 and t=9·2^-1074 at DOF 0, −F at DOF 6, and 5·2^-1074 at DOF 9.
The frozen source-derived exact axial equilibrium gives u0=t/k=9·2^-1041.
The inherited provisional input hash is
690864638ae69bd4ecf3784f0705c49bb6128c846816b971d0e3e6b1f481fb92;
the new manifest independently binds the exact extracted input artifact.

## Actual source and selection path

Source revision 3bddc2b05f6106e969c7cf43373b230845c7cc66 remains FK-identical to
product basis d01ad98a754698631f927709d08284c272de85e8. G0 binary remains
bcbe897204ec702b99529d25e6d0213d0132af5e6086e0397fae3a8f8ef8a08f;
probe main/cases and lock remain unchanged and G0 enabled features remain empty.
SOURCE_COMMIT, C17 case ID, exact 100000000/100000000 limits and source encoding
before/after selection all match. No independent source-encoding decoder or
retained-state replay is claimed; raw encodings are preserved.

Observed p128 Candidate is Accepted at Coalesced gate with zero corrections;
P256 Verification is Verified, also with zero corrections. No bound refusal,
missing Uc or shift factorization is reported. Public RCM-on-derived-pattern is
[2,1,0]; it does not expose the private factor directly. Translation scale is
0310000000000000 (=2^-974); rotation scale is h. Complete attempt, stop-rule,
estimate, charge, theta/B, resolution, residual and geometry records are in raw
stdout. No private W-plus/t1/t3 value is inferred from the public summaries.

## Runtime and evidence

Same TASK /root/t3_recovery_manager/i22_a1 under the recovery manager; no
delegation. G2 commit d5bee8a74c79e806e907572b34cb689db4f0e32b and grant hash
1b254724f6092e874db39cb82685d184bb1c13fd8dd33d3c72c3fd0c3c12034c are recorded.
Only I22/c_01/** and owned scratch/c_01/** were written.

One fixed argv used G1's explicit environment, existing memguard, tool-managed
PTY and /usr/bin/time -l:
<WT>/scratch/i22/target/debug/a1_public_probe C17 100000000 100000000 C.
Exact commands/environment are in cases/C17/CASE_RESULT.json. The existing
ROOT-bound <VENV>/bin/python -B ran the unchanged strict comparator once.
Both processes completed in their initial tool yield; no continuing session
ID was returned and no interruption occurred.

Existing guard PID 5387 was observed before/after and at final check.
Time reported 0.02 s real, 0.01 user, 0.00 system, 3,997,696 B maximum RSS,
1,999,184 B peak footprint, zero swaps. These are command resource observations,
not process-group bounds. No automatic deadline or per-process hard cap is claimed.
No owned solver/compiler/comparator process remained at the final query.

The first post-comparison inspection mistakenly read b_01/C17 paths, yielding
empty displayed records and false local identity flags. BLOCKING was reported
immediately from the actual strict report. Reading the unchanged c_01/C17 raw
files corrected that inspection and all identity checks passed. Neither the
solver nor comparator was rerun; no output was edited. The read-path mistake
and original flags are disclosed in CASE_RESULT.json.

All 115 archived-source and 141 original G0 scratch entries, 32 frozen-oracle
entries, and prior checkpoint/G0/B seals verify after the stop. Tracked/cached
Git diffs are empty. Every Git read used GIT_OPTIONAL_LOCKS=0. No source,
oracle, protected gate, guard, tool, configuration or Git/index mutation occurred.

| Evidence | SHA256 |
|---|---|
| cases/C17/stdout.tsv | 841167ed887a71b0e2a904647e1f217758fad304baaaf0d669f5c47035dc797a |
| cases/C17/COMPARISON.json | d22b3134ac678966fca673f858c51296cdf4ee625d9dbae9d56da2be816edf76 |
| Frozen TRUTH.json | 1772d703e032a71587b922a2f6d825718f777c9dae78d1041fbd874210ee87ea |
| Frozen exact_oracle.py | fa8ea6f303148d9babb5d9fe6c53f64377b13cb130d03d076d4cec7d3f4c15e0 |
| Strict validate_compare.py | 3fada0d22ec59b950af85098996de5e34befba678907394a33af025766d0ca43 |

Raw stdout/stderr/time and comparison stdout/stderr are preserved without
normalization. FALSE_PUBLICATION.json isolates the exact violation; COMMANDS.json
retains pre/post evidence; SCRATCH_SHA256SUMS inventories four raw scratch files.
Final SHA256SUMS excludes itself.

Return to ROOT for independent checking and consequence/design selection.
Remaining C cases need a new grant; no continued search is implied. Complete
publication guarantee, raw force/moment coverage, unit/source-unit and G5a/D2/
receipt consequences remain with their owners. No repair, new physical cutoff,
contract amendment, general availability claim or F2a reliance is selected here.

