# Required-output development gate

`tools/validation/qualification_gate.py` is the thin VP-HARNESS adapter selected
by ROOT. It preserves a nonempty required case/assertion ledger before execution,
runs an explicitly selected foreground executable, retains bounded raw data and
reports every required outcome. It does not build a solver, download references,
regenerate targets, set app Current, mint an opaque producer token, or authorize
engineering/release use. No material/component library or code rule is supplied.

The preserved transport is `main_sparse_cli_1.0_raw0.1`: the existing
`openpipestress-runner solve` command, implicit sparse_interactive dispatch,
ControlledExport wrapper, CLI1.0 and raw mechanics0.1. The known semantic table
and existing comparison implementation are hash-pinned. This transport remains
sparse only.
Main's six-decimal output remains that producer; this adapter cannot upgrade it.

The explicit successor `ordinary_physics_1_cli_1.0_raw0.2` covers only the two
selected original static cases, using the actual CLI `solve --input -
--solver-mode sparse_interactive|dense_scrutiny` interface. It requires ordinary
`openpipestress.result_semantics/0.3.0/physics-1`, finite binary64 publication,
passing numerical standing and the exact-pressure formulation. Source-block,
physics-source/composite, sensitive standing, legacy fallback and unknown future
profiles are refused. The CLI emits a raw envelope, not a canonical0.3 document.

## Existing records, narrow selection

The selection file is an obligation manifest, not a second model/result schema.
Its `format` is `openpipestress.qualification_selection/1`. It contains:

- `profile_id`, `purpose` (`harness_development` or `development_comparison`),
  `transport`, and `runner` with exact candidate commit/executable SHA256 and
  `solver_mode: sparse_interactive`.
- Nonempty unique `cases`, each with an ID and SHA256/path bindings to the actual
  CLI input, independently prepared reference values and existing PKG-14
  tolerance-profile document. Paths are explicit, relative to the selection
  file or absolute. The gate reads them; it never modifies them.
- Nonempty unique assertions per case. Each assertion has its ID, criterion
  rule ID and an exact selector: raw result ID/kind/unit/entity, case basis,
  all five source metadata fields, and explicit physical dimension. Basis tokens
  follow the pinned producer: `load_case` or `combination`. Duplicate
  required selectors and ambiguous/missing result IDs cannot reduce coverage.

The ordinary transport additionally binds `physics_binding`, `reference_basis`,
and boolean `runner.explicit_local_private_intent`. Each of its two cases retains
all 73 scalar selectors plus ten separately reported structural obligations. The
complete ordered case inventory is required. Displacement magnitudes genuinely
lack metadata: `metadata: null` in their selectors requires an absent raw field,
not a raw field containing null. Each case binds the structural obligations,
original section reference and independently checked numeric addendum. Nine
section comparisons are details of the existing material/section obligation.

The input remains the existing `request` plus `solve.preview_model` object; no
new product field is injected. Reference files use the small
`openpipestress.qualification_reference_values/1` record: `case_id`,
`reference_kind`, `readiness`, `basis`, and exactly one `{assertion_id,value,unit}`
for each assertion. Non-synthetic development comparisons also require an
`independent_review_ref`. Those labels record supplied provenance; the gate does
not authenticate an agent/human review or make an unadmitted oracle ready.
External source candidates remain unavailable for scoring until their real
independent input/oracle/selector/criterion package is frozen.

The criterion file is the existing `tolerance_profile` structure at declared
schema0.1: reviewed profile, unique rules, dimension/family/unit, same-unit
normalization, numeric nonnegative single-absolute or relative/absolute values,
review and provenance. The adapter deliberately supports the required consumed
fields, not a substitute whole-document schema validator. Each rule must match
the asserted physical quantity. It calls the existing PKG-14 classifier with its
unchanged symmetric maximum policy. It does not adopt the supplied reports'
additive/reference-sided formula or a default epsilon. Conversion is currently
unsupported; quantities must already share explicit units. No near-zero floor
is inferred. Missing or unclassified criteria block the case.

For the first-static successor, original inputs, selectors, reference values,
scalar rules and section criteria are hash-bound. The derivation helper
`tools/validation/build_first_static_selection.py` changes only selected
review/admission metadata. It verifies the original packet and Root's technical
selection/review records and refuses existing output directories. Generate one
two-case manifest per mode with the real candidate commit, executable digest and
reviewed reader binding: 292 scalar and 40 structured obligations across four
prospective runs, plus 36 section subchecks inside the existing obligations.
Preparation is not execution. Root's selection is agent technical selection
under owner delegation, not personal human equation review or release acceptance.

`tests/test_qualification_gate.py` provides executable **synthetic harness-only**
examples of the complete selection and data objects. Those numbers/criteria are
test inputs, not analytical targets or production policy. It also checks refusal
of an actual preserved legacy unwrapped packet; no legacy output is silently
promoted to the new transport.

## Explicit execution and evidence

Invoke the adapter with `--selection`, `--executable` (absolute), `--source-root`
and a new `--output-dir`. Optional `--timeout-seconds` and
`--output-limit-bytes` control bounded process observation. The script adds only
`solve` to the selected executable, supplies exact input bytes on stdin, uses no
shell, and writes only the explicit output directory. Existing output directories
are rejected. The ordinary path adds the explicit input/mode flags above, plus
`--explicit-local-private-intent` only when selected. Runtime discovery/build/
network/reference acquisition is absent.

Before each process, the ledger and reference/criterion snapshots exist. Exact
immutable input bytes are delivered through a pipe, multiplexed with output; the
archive file is never used as the process input source. Captured
stdin/stdout/stderr, exit or signal, executable before/after hashes, timing and
selected thread environment are retained. Timeouts/output-limit failures terminate
the owned process group and keep partial raw bytes with explicit incomplete-capture
status. Delivered-byte hashes and retained archive hashes are checked separately;
changed archives cannot support a passing record. This records byte delivery,
not an attestation of how the executable internally uses it. The consumer performs one bounded size/hash-checked read of captured stdout
and decodes exactly that returned byte object. It never parses an unchecked
later reread. Missing executable, wrong hashes, unsupported versions, bad semantics and
unready references preserve the full required denominator as blocked/error rows.
Interruption leaves later rows not_run. Ordinary completion never rewrites a target.

The run records actual checkout revision/dirty state, executable and harness
identity, selection/input/reference/criterion hashes and the bound source semantics.
A selected source checkout and executable digest are not proof that this binary was
built from those exact sources; a real build manifest/distributed candidate witness
remains required for that claim. This matters especially while the engine is being
integrated. Model/request/mode identity and process capture are necessary evidence,
not an engineering acceptance certificate.

The ordinary adapter calls the existing owned `validate_physics_evidence`
function in a fixed Python `-I -S` helper. Its exact module, semantic table and
unit authority hashes are selected together. The module/table are copied to a
private snapshot; same-named modules in another checkout or `PYTHONPATH` cannot
replace them. Only the fixed ordinary entry executes; the unit authority is
bound read-only, without unit conversion/build/network fallback. Review records
and finite admission records are also byte-bound and retained. The helper input
is a derived parsed snapshot, distinct from the original runner stdout bytes.
Its verdict establishes internal physical consistency only; actual process
provenance and analytical comparisons remain separate evidence.

The ten structural checks cover transport/mode, input binding, ordinary contract,
case/material/section, numerical standing, complete unique scalar coverage,
normal maximum evidence, governing location and both summary bindings. Section
OD/wall use selected exact same-unit identity; other positive section quantities
use the addendum's explicit relative rule without an inferred absolute floor.
Enclosure ordering/containment and summary identity use exact relations; the
owned validator checks coefficient-gap validity. The ideal-decimal analytical
oracle need not lie inside a coefficient-only binary64 enclosure.

`ledger.json` is updated atomically within the new run directory. Before final
publication, every completed case's retained input/reference/criterion/stdout/
stderr snapshots, original bindings and retained selection are rechecked against
their captured in-memory identities. This also detects a later process changing
an earlier case. Interim ledgers cannot publish an all-matched outcome. Raw
captures and bound copies are retained observations; the filesystem is not locked,
and the check does not prevent changes after the observation or after publication.
Consumers must verify the recorded hashes again before later reliance. `summary.md` derives
from that same ledger. Exit0 means only all selected required assertions matched;
exit1 means obligations remain unsatisfied; invalid selection/host setup returns2.
Decoded nonzero numeric tokens that underflow to zero are refused. Current
uppercase/lowercase diagnostic severities and wrapper counts are checked, and
unknown raw row fields cannot silently contradict the pinned semantics.
The final ledger records the custody observation time and these limits.
Reports explicitly state `qualification: not_established_by_this_harness`.
For the ordinary path, final custody observation includes reader dependencies/
reviews, finite selection records, original selected packet files, derived
criteria, structural files, and helper input/output streams. A failed reader or
malformed structural return cannot remove the predeclared obligations.

## Verification and remaining integration

Run the focused stdlib suite with the project's Python environment:

```sh
PYTHONDONTWRITEBYTECODE=1 python -m unittest discover -s tests -p test_qualification_gate.py -v
PYTHONDONTWRITEBYTECODE=1 python -m unittest discover -s tests -p 'test_qualification_physics*.py' -v
PYTHONDONTWRITEBYTECODE=1 python -m unittest discover -s tests -p test_first_static_selection.py -v
```

This route avoids the general pytest setup's unrelated Rust build. It starts only
short synthetic executables in temporary directories, including timeout/output
limit/nonzero/malformed cases. It checks exact denominator retention, criteria,
semantics, identity, immutable inputs, interruption and raw capture. Passing this
suite is harness evidence only. No actual Piping solve is claimed by this slice.

The physics suites include pure synthetic observations, deliberately patched
reader stubs and supervised synthetic solver processes. Their passes test harness
composition/custody and cannot score the real solver. Received packets and real
builds are separately recorded in undertaking evidence. Before four selected
comparisons, the complete adapter receives independent review and the real
executable/source/build identity is selected. Full Q1, missing direct transverse-
shear and signed circumferential outputs, later interaction/dynamic profiles,
native/human/agent witnesses, licensed reference execution and release remain
separate obligations. A first-static pass does not close those gaps.
