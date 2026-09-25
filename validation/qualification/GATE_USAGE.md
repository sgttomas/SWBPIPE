# Required-output development gate

`tools/validation/qualification_gate.py` is the thin VP-HARNESS adapter selected
by ROOT. It preserves a nonempty required case/assertion ledger before execution,
runs an explicitly selected foreground executable, retains bounded raw data and
reports every required outcome. It does not build a solver, download references,
regenerate targets, set app Current, mint an opaque producer token, or authorize
engineering/release use. No material/component library or code rule is supplied.

The initial closed transport is `main_sparse_cli_1.0_raw0.1`: the existing
`openpipestress-runner solve` command, implicit sparse_interactive dispatch,
ControlledExport wrapper, CLI1.0 and raw mechanics0.1. The known semantic table
and existing comparison implementation are hash-pinned. Dense, source-block,
physics/composite and future profiles are refused until explicitly integrated.
Main's six-decimal output remains that producer; this adapter cannot upgrade it.

The physics owner is implementing a closed `--solver-mode` extension, but this
adapter intentionally does not call the proposed flag before the actual reviewed
interface/build is supplied. Mode-independent machinery does not depend on it.

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
are rejected. Runtime discovery/build/network/reference acquisition is absent.

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

## Verification and remaining integration

Run the focused stdlib suite with the project's Python environment:

```sh
PYTHONDONTWRITEBYTECODE=1 python -m unittest discover -s tests -p test_qualification_gate.py -v
```

This route avoids the general pytest setup's unrelated Rust build. It starts only
short synthetic executables in temporary directories, including timeout/output
limit/nonzero/malformed cases. It checks exact denominator retention, criteria,
semantics, identity, immutable inputs, interruption and raw capture. Passing this
suite is harness evidence only. No actual Piping solve is claimed by this slice.

Next: independently freeze the first real axial and bending/torsion request,
reference, selector and criterion bindings; select the real reviewed solver build;
add its closed mode/raw/semantic dispatch; then execute and assess. Signed-support,
precise/source/physics carrier gaps go to their source owners. Native/human/agent
workflow witnesses, licensed reference execution and release remain separate.
