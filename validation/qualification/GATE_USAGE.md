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
sparse only. The presence of newer producer, numerical, formulation, physical,
recovery or foreign-carrier namespaces is refused even when their value is null.
Main's six-decimal output remains that producer; this adapter cannot upgrade it.

The explicit successor `ordinary_pressure_1_cli_1.0_raw0.2` covers only the two
selected original static cases, using the actual CLI `solve --input -
--solver-mode sparse_interactive|dense_scrutiny` interface. It requires ordinary
`openpipestress.result_semantics/0.3.0/pressure-2` (`exact_pressure_v3`), finite
binary64 publication and passing numerical standing. B7b retired physics-1
(`2.0.0/exact_straight_pressure_v2`) and the 0.1.0-0.4.0 model documents: the two
frozen, hash-bound runner inputs are never edited; the gate checks their bytes and
submits a declared re-authored copy (model 0.8.0, the current
`3.0.0/exact_pressure_v3` contract with `shear_model: euler_bernoulli`, and an
explicit zero `corrosion_allowance` on the section; nothing else changes), records
both hashes in the case ledger, and reads the raw against the copy.

T4-U6 (D-5; DEL-05-03 `section-bases-and-timoshenko.md` §2.1-§2.5) publishes
every exact stress row on the nominal and on the reduced section, each naming
its basis, with per-basis maxima, coverage and headlines and the D-5
`pipe_sections` shape. The first-static frozen selectors,
`STRUCTURAL_NUMERIC_ADDENDUM` and `STRUCTURAL_EXPECTATIONS` bytes are never
edited either; the gate reads them through a declared re-authored copy, as it
does the runner input (rule `openpipestress.section_basis_selector_reauthoring/1`,
`qualification_physics.reauthor_selectors`):

- each stress-kind selector (the 21 frozen `stress.*` assertions per case)
  becomes its nominal twin and its reduced twin: assertion ID
  `<frozen id>.nominal_section` / `.reduced_section`, result ID with the §2.5
  basis segment (`:nominal-section` on element-stress rows, `:nominal_section`
  on `result:elastic-maximum:` rows), `metadata.basis` with
  `; section_basis=<basis>`, and the same kind, unit, entity, case basis,
  dimension, criterion rule and expected reference value. Every other selector
  is unchanged. Each case's 73 frozen selectors are 94 derived obligations;
- the addendum's nine flat section quantities are read at their D-5 locations
  (`outside_diameter_m`, `ro_m`; the effective wall at `wall_thickness_m`;
  `ri_m`, `As_m2`, `I_m4`, `J_m4`, `Z_m3` in `nominal_section`; `Ai_m2` in
  `pressure_bore`) with coincident bases required: zero mill tolerance and
  allowance, Euler-Bernoulli, and the same value at every alias on the reduced
  section and the pressure bore. Its frozen selection still names the retired
  `authored_normalized_od_wall_v1`; the derived reading requires
  `authored_od_wall_d5_sections_v1`;
- the structure checker reads per-basis extrema (nominal then reduced, each
  bound to its own maximum row and equal apart from basis and row), the
  two-entry per-basis coverage, a null `max_open_formula_stress` and both
  per-basis headlines.

The case ledger records `selector_reauthoring` with the frozen and derived
selector and reference-value hashes, beside `runner_input_reauthoring`. No
frozen first-static or load-reference input authors a nonzero mill tolerance or
corrosion allowance, so no reference basis changes under D-5. The load-reference
package selects no stress row and has no section addendum; it is read
unchanged. Source-block,
pressure-source/composite, the retired identities, sensitive standing, legacy
fallback and unknown future profiles are refused. The CLI emits a raw envelope,
not a canonical0.3 document.

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
all 73 frozen scalar selectors, read as 94 derived obligations, plus ten separately
reported structural obligations. The complete ordered case inventory is required.
The authoritative two-case/94-ID derived ledger is predeclared even when a
malformed submission omits or duplicates a
case/assertion; submitted inventory is recorded separately. Failures retain the
nine section identities as unavailable/error details per case, and malformed
checker returns cannot shrink those nested obligations. Displacement magnitudes genuinely
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
reviewed reader binding: 376 derived scalar (292 frozen) and 40 structured
obligations across four prospective runs, plus 36 section subchecks inside the
existing obligations.
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

The ordinary adapter takes the owned pressure-2 reader's verdict
(`validate_pressure_evidence`) from the Rust product reader, the only product
reader (b7-plan R2): an explicitly built `openpipestress-runner read pressure`
named by `OPENPIPESTRESS_RUNNER_BIN`, captured as a supervised process. The
binding (`openpipestress.ordinary_physics_consistency_binding/3`; versions 1 and 2,
which named the former Python and the retired physics-1 readers, are refused) selects the
reader's source, the current pressure-2 semantic table and unit authority by hash; they are checked in
the candidate before and after each reading and copied to the run's retained
basis. The reader executable is recorded by digest only; it is not proof of a
build from those sources. The unit authority is bound read-only, without unit
conversion/build/network fallback. Review records and finite admission records
are also byte-bound and retained. The reader input is a derived parsed snapshot,
distinct from the original runner stdout bytes. Its verdict establishes internal
physical consistency only; actual process provenance and analytical comparisons
remain separate evidence. The load/reference-state adapter (B7b: pressure-2 with
that group, transport `pressure_1_load_state_cli_1.0_raw0.2`, binding version 3)
and the exact_pressure_1 adapter take `read pressure`, the Rust reader's pressure-2
route (`validate_pressure_evidence`; `openpipestress.exact_pressure_consistency_binding/2`),
the same way; both submit the declared re-authored copy of each frozen case input.

T4-U6 (manager ruling, 2026-10-16) adds one exact_pressure_1 case: the D-5
re-authored T4-I8 MILLTOL documents and their declared Timoshenko variant,
authored complete on model 0.5.0 under `3.0.0/exact_pressure_v3`, with an
explicit `shear_model` (`euler_bernoulli` or `timoshenko_annulus_energy_v1`) and
an explicit `corrosion_allowance` on every section. Since model 0.6.0 such a
document is kept as bound and submitted as its copy re-authored by its version
alone: its `runner_input_reauthoring` record names the same rule, the frozen and
submitted sha256, versions 0.5.0 and 0.8.0 (the current version), the same
contract on both sides, and `explicit_zero_corrosion_allowance: false`. A package
document authored on 0.6.0 or 0.7.0 is re-authored the same way; one authored
on 0.8.0 would be submitted unchanged under the identity record. Every other package
document must still be a frozen 0.3.0/0.4.0 document under the former
`3.0.0/exact_pressure_v3` contract object, submitted as its re-authored copy.

The ten structural checks cover transport/mode, input binding, ordinary contract,
case/material/section, numerical standing, complete unique scalar coverage,
normal maximum evidence, governing location and both summary bindings (the
maximum, location and stress-headline obligations hold on each section basis). Section
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
criteria, structural files, and reader input/output streams. A failed reader or
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
