# RV29 D/C backcheck — frozen40129

**Backcheck complete; no new blocking source/test finding in this bounded delta.**
This is not final A1 PASS or merge acceptance. Candidate40129a225d73860ac2a53da9a2fa73869df668f3
contains final new-test hashb04eb95536437274e400b85050bf082bf193587b5f0d2f78524fb78a0e699c0d.
The diff from RV29's f118 review has four test files: the three D files plus the
separately granted S11 repair. Production adaptive.rs and structural.rs remain
dd1-identical. REVIEWED.diff, BASIS.json and VERIFICATION.json bind the actual
bytes inspected; no moving maintained test was reviewed. No Rust was run by RV29.

## Historical criteria and component isolation

Independent byte comparison confirms all bytes outside golden_work_counts and
kf1_golden_stop_rule_work_where_collapses_occur are unchanged. Their old tables,
A3B_WORK, counts, cost literals and unrelated-stage/selection assertions are
preserved. The changes do not replace any golden with an observed inclusive total.

replay_certificate_components obtains the exact stored candidate/verifier Arcs
and same prep/group, reconstructs a transient bound report with local setup
work, and verifies pairing. It reruns compare_states directly. Only after that
R7 result passes does it separately meter the certificate. It asserts total=
context+sum, inclusive stop_rule=R7+certificate, stage/attempt identity and fresh
case/invocation closure. Accepted replay compares raw publication and radius
bytes; rejected replay compares exact row/predicate. Setup charges never enter
the recorded solve. This is component isolation, not an independent certificate
price oracle; the frozen PC40–43 and future PM16 runs retain that role.

KF1 keeps its tracker/seed Setting alive for both solve and replay. T=infinity
and512 still match old R7 literals; T64 retains exact768*17506 and640*17506
excesses. Certificate components and raw publication/radii must be invariant
across settings, with same_solve/same_attempts still checking other state/stages.
The A3b test subtracts only replayed certificate context/sum and uses direct
R7 total; every historical literal remains unchanged. The actual captured
one-test golden and KF1 results each passed, at the pre-accessor f93 new-test
hash. Their results are not relabelled as full40129 execution.

## Specific review gaps addressed

- T02-1 is repaired exactly as authorized: the prescription tail/expectation is
 2^-600 and its comment names128/256/512. The magnitude subcheck is unchanged.
 The later affected prescription filter passed once; D's subsequent24-test
 filter also passed.
- S11 contains only run_schedule count5→6 and the explicit integer certificate-
 rejection increment explanation. Its three-test target passed. The production
 source/site count is unchanged by D.
- The verified relative fixture uses the exact independent Wide limbs for H,
 real RelativeVerified classification and x=S bits3ff0000000000401. certify_rows
 accepts H while RU64(H) exceeds the exact sharper allowance, with exact distinct
 positive radii3ca0020000100402 and3000000000000401. Raw identity comparisons
 preserve tags, signed zero, subnormal precision, row identity, class/bound and
 scale bits through finalization and Clone. Both final solve and Clone accessors
 return the independently fixed positive radius and matching typed identity.
- Later M:1 rejection/missing-field and conversion-budget failure occur after
 D:6's positive radius; a subsequent fresh draft restores both radii. Dynamic
 prefix work only locates the control boundary, and is correctly labelled as
 such. Arc counts establish prep ownership, not heap measurement. Source ownership
 inspection establishes that unsuccessful local radii do not escape Accepted.
- C17 now checks stable D:0/Translation/AbsoluteBound rejection and p256 reuse
 without duplicate precision records, plus actual case-budget stopping and stage/
 invocation closure. EXTRA-ZR checks the128/256/512 D9/Rotation rejections, roles,
1024 Solved verification, Ceiling and absence of Accepted/Verified. This is
 schedule/refusal evidence, not accuracy on a nonselected case.

The positive relative fixture deliberately supplies controlled certificate values
and report fields. Its manual finalization setup is not evidence that those
synthetic fields pass the earlier R7 gates or arise from source equilibrium.
Its assertions exercise the private certificate/finalization seams; actual
source scheduling is separately exercised by C17 and the fixed extras. Explicit
corrupted-copy comparisons demonstrate the identity comparator's sensitivity,
not execution or killing of production PM15 mutants.

## Executed evidence checked

RV29 verified six manifests and their entries, not only RETURN labels. Raw stdout
confirms C's23 publication tests,2 H export tests,3 S11 tests and1 prescription
test passed. D's24 publication tests and two exact golden filters passed on f93.
The final accessor-only delta has a separate1-test pass and binary hash
096dcce8110092a71e7dc3217451b4def920bb0a51a8aac45b83dc1c5b500f38, matching the
preserved current executable. The final test source matches its b04 after-hash.
The earlier compiler-only failure and narrow repair remain preserved; none is
counted as a numerical failure or hidden rerun. A complete exact-final-candidate
suite/gates is still required.

## C probe/source/binary binding and independent matrix

The preserved probe source/Cargo/lock and executable match their recorded hashes.
All116 archived FK files match immutable dd1f70d8ba85b19f7d948bca6ee08a44bbb12ae1.
RV29 reconstructed PROBE.diff exactly from the old520d source and preserved new
probe, after reading the old response scope handoff. It only adds the three
fixed extra constructors/spring multiplicity, tranche routing and truthful source/
path metadata; original B/C constructors and lock remain unchanged.

The verbose build names the immutable archived FK src/lib.rs and links that
compiled library into the probe. Both recorded and preserved physical Cargo
fingerprints have empty enabled features and rustflags. Declared availability
of mutation-controls in check-cfg is not enablement. The build is lib/bin, not
test configuration. Probe binary hash is
dcac1835f4702508c45fe3b3fcda21978c57e37d84dd1511c8c726d9c1e9c637; every case
record binds that same hash. Exact once-each order/argv is C17, B01–B16, FM/MF/ZR
with fixed100000000 case/invocation limits and recorded guard presence. These
checks support the execution binding independently of printed SOURCE_COMMIT.

Each of the20 author raw outputs is byte-identical to the corresponding released
independently checked raw file. Comparator/truth manifests are unchanged; report
hashes, source binding, empty failure lists and row counts agree. Together C17
and the remaining-fixed packet establish19 selected outputs /708 rows under
strict bare-b/public/both-sharper output checks, plus source-bound EXTRA-ZR
Ceiling with numeric_accuracy_pass=null and zero compared rows. No checker was
rerun here, and output-claim comparison is not proof of private H, floor/bound
construction, execution provenance by itself, or product qualification.

Direct raw old/new B comparison independently confirms ROOT's reconciliation:
all16 remain selected; values, range tags, classes and subnormal outcome details
are identical. Twelve now select512/P1024, both force/moment floors are h, and
144 bound fields change0→2h. Their corresponding formerly-zero force or moment
scales become h. B03/B04/B09/B10 remain128 with unchanged scale/bound fields.
Work and metadata change. Thus output bytes/bounds are not unchanged, although
the production class/floor/bound formulas were not altered by this test delta.
B_FIELD_BACKCHECK.json preserves this field-level result without inventing new
expected selections or numerical oracles.

## Remaining boundary

No additional semantic repair is requested from this bounded backcheck. Required
PM patches and actual intended-failure executions remain: especially post-
certificate/raw-radius/accessor PM15 variants and PM16 omission, full-clone
duplication and reaches-only16c. Independent ledgers, restored controls and
relevant predicate failures must establish kills; compilation errors or unrelated
snapshot drift do not. Old R7/A1/A2/V-K obligations remain criterion-specific.
Broad H/VR consumer checks and exact-candidate source/CI/DEC-025/GEN-8 gates are
still outstanding; any real consumer regression blocks its relevant check.

The current kernel-only reach disposition still holds. As ROOT explicitly ruled,
full K6c E_max/measurement/admission work is the next separate slice, not an
additional implicit A1 merge gate. A1's new Box payload/header, partial draft
lifetimes and retained/cloned ownership remain routed to that accounting. No
A1 review or future merge establishes W1 limits, K6c completion, F2a qualification,
product/native acceptance, lifecycle issuance or release.
