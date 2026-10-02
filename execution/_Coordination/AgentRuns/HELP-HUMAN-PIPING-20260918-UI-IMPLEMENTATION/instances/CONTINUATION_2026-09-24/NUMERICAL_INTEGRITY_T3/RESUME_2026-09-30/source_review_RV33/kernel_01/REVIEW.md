# RV33 — independent shared-kernel implementation review

**No actionable finding. Suitable for bounded manager fan-in.** Review covers candidate
`4a6cb3402f95d98c6f46f702055fe4b0c5245c56` against
`60a52da9467b73d25898e01312e20d7a4c533902`, limited to the three maintained kernel
files. Neither blocking nor non-blocking defects were identified. This is a
conditional arithmetic implementation review; it supplies no H/VR admission,
full E_max, executable qualification, numerical outcome, or release acceptance.

Here H means `projects/chirality-piping/core/solver/performance_harness`; R is the
enclosing `RESUME_2026-09-30` directory. Exact source origins, hashes, commands,
raw outputs, scratch locations, compiler details and actual host limits are under
`_run_records`. No maintained file or Git index was changed by this reviewer.

## Scope and source findings

The complete 1,184-line `H/src/k6/w1/envelope.rs`, 487-line
`H/tests/k6c_envelope.rs`, and module-export/documentation diff were inspected.
The repository scope validator passed for the three-file maintained delta;
additional candidate paths are run evidence already owned by other participants.
The three working files matched their exact candidate blobs and author-bound
hashes at the initial binding check. During sealing, later H-adapter edits appeared
in envelope.rs/mod.rs; they were preserved and excluded from this review. The
archived exact candidate and its complete compiled core closure remain unchanged. All eight consumed seal manifests and their 89 payloads verify, including
the relocated author packet seal `0cef2a5bdca59d63d69d90a92b3dc691f9ec80d6a11282f1dcba41717b0d5503`.
The author's six fixture bindings also match their sealed source records.

| Area and implementation anchor | Review result |
|---|---|
| Descriptor contract, envelope.rs:110–268 | Derived n/f/q/encoding relationships, z/h population limits, public encoding widths, nonempty ID aggregate/max relationships, H construction restrictions, exact versus upper B/b, and missing support/prescribed proofs fail explicitly. Private validated fields preserve the scalar checks. Real source validity, IDs and count/source correspondence remain clearly external. |
| Checked arithmetic, :433–565 | Checked u128 add/subtract/multiply/divide and next-power-of-two preserve failure through both min and max. Final exposed byte values enforce the named 64-bit reference request ceiling. Pair addition is max(M1+R2,R1+M2), preserving only one active old request. Fixed constants and four/three-slot indices are bounded. |
| Capacities and trackers, :490–613 | Zero/minimum/growth/old/sort/tree operators agree with the accepted operators. Tracker retained and prune/rebuild alternatives retain separate old/new identities while active realloc surcharges remain alternatives. No public current-host sizeof is mixed into the fixed private historical reference basis. |
| Generic preparation and owners, :688–891 | H exact maps and formatted IDs remain distinct from VR pushed arrays and cloned IDs; PREP's clone is separately retained. Axis springs, directional operators, retained loads/ID bytes, grounds and directional-list alternatives are preserved. B/b uppers do not reconstruct z/h. B0 and S/U/V match accepted source05/06/09 plus kernel22/27 corrections. |
| Schedules and aliases, :926–1184 | Full four-solve/three-verification and shortened S/U128,256+V256 rosters retain their minimum backings. RES/TOP/HATCHECK, canceled PassRest, report-summary alias, current/next shift vectors, shared Arcs, R7, publication and finish ownership preserve the reviewed identities. Stationary selected excludes separate current Report/Decision; refused/unresolved retain Kpad. No impossible-path zero or numeric-outcome inference enters the bounds. |
| API and dependency boundary, :1–36, :893–924; mod.rs | The sole named immutable profile identifies mathematical premises. Documentation explicitly separates executable correspondence and caller/admission composition. Evaluation uses scalars, fixed arrays, slices and static error strings. Inspection finds no estimator heap allocation, environment gate, evidence-file reader, graph/model/encoder/solver execution or hidden formatting. This is a source-inspection conclusion, not an allocator measurement. |
| Maintained tests | Literal axis/directional/canceling-load, H/VR, B=1/B=5, full/short, stationary and failure fixtures protect meaningful behavior without duplicating the full formula or reading execution evidence. The complete original nine-test file passed on the clean candidate archive. |

The historical open-cell language in source05/source09 remains historical. The
review relies on the accepted corrections, conditional profile choice12, and
independent kernel27 disposition supplied in this assignment; it does not
re-derive or expand closed library/layout proofs.

## Independent compiled checks

A clean `git archive` of the exact candidate's `projects/chirality-piping/core`
subtree supplied the complete local Cargo dependency closure. Execution evidence
was excluded. All 562 archived files remained unchanged after building. The
archive contained no target artifacts; compilation used a fresh dedicated target,
installed Rust 1.97.1, release, offline/locked, incremental disabled, four build
jobs, two test threads, and the existing live memory guard. Cargo fingerprints
record empty feature and rustflags sets. This is a clean core-closure build,
not a claim that the entire repository was archived or qualified.

- **9 maintained tests passed.**
- **250 independent literal-reference tests passed:** all 193 reference-upper
  rows, all 24 exact CLI rows, and all 33 H arithmetic rows. The overlay generator
  projected sealed table values without running an author's formula calculator.
  It checked 31,500 complete phase requested/moving pairs and 7,250 additional
  scalar/pair assertions, including 1,000 per-precision R7 pairs. Constructors,
  original/PREP source, B0, S/U/V, full/short Kpad and maxima, phase identities,
  dominant-phase consistency and stationary return values matched.
- **5 scratch boundary tests passed.** Exact candidate implementation bytes were
  copied to a separate integration-test root with a test suffix; the original
  archived and maintained source stayed untouched. Checks cover poisoned max/min,
  arithmetic and request-width failures, minima and capacity transitions, first
  allocation versus old grow, tree/sort boundaries, one-active-old pair addition,
  fixed phase capacity, zero/small/free-count transitions through 513, exact/upper
  phase dominance, and invalid populations/IDs/count widths.

Reference IDs use sealed byte metadata. For external CLI rows, the reviewed
c<index> grammar supplies maximum ID length and its byte sum was independently
matched to the stored aggregate. H comparisons preserve the existing reference
constructor's decimal-ID maximum upper; they do not newly attest which actual
load attains it. These binding details are explicit in `OVERLAY_BINDING.json`.
Synthetic boundary descriptors test scalar arithmetic only, not physical source
existence. No production graph, model, solver or encoder algorithm ran. No timing
or measured-memory inference is made.

## Limits and return

The tests establish this translation under the named conditional facts; they do
not prove current production allocator/request equivalence, actual graph/count
provenance, or all possible arithmetic inputs. H/VR descriptor and caller/result
adapters, full-global composition, original semantic/mutation programme,
artifact/input/launch qualification, chronological admission and performance
measurements remain separately owned. Broad CI/DEC-025 is outside this bounded
review and was not represented as passed here. The parent reported that A1's
DEC-025 run is separate; its result is not consumed by this review.

No correction was requested, no author repair/backcheck cycle was needed, and no
follow-on or time extension was initiated. Concurrent later working changes are
recorded in `_run_records/LATER_WORKING_DELTA.json` and `.diff`; they need their
own review coverage and are not included in this candidate disposition. See `_run_records/VERIFICATION.json`
for completion, `_run_records/BUILD_COMMAND.json` and `BOUNDARY_COMMAND.json` for
full commands, `BUILD.log` and `BOUNDARY_BUILD.log` for raw outputs, and
`_run_records/SHA256SUMS` for the packet seal (paths resolve from this packet root).
