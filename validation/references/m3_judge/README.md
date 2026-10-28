# M3 judge: the K-D5 M3 criterion on a closed evidence view, both routes

This directory holds the one M3 judge shared by the exact-route set (`../kd5_timoshenko/`, K-D5 M3-T)
and the pressure-free set (PF M3-T, #1663). It judges raw runner outputs against an exact reference,
with the acceptance of
[kd5-member-relative-bound.md](../../../execution/PKG-04/DEL-04-01/Design/kd5-member-relative-bound.md)
section 4.3. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Its verdicts are
evidence for the owner's review: they record no acceptance, issuance or lifecycle standing (F-PIP-3).
Every model is invented.

**Why it exists (refutation round 5).** In the earlier judges the validator and the comparison read
different data. `kd5t_compare.py` searched the whole raw record for evidence, while the PF schema
checked only the subtree it knew. An upper bound placed under `support_attribution.nested` turned four
C2 failures into a pass. Three variants had the same root:

- a duplicate extremum or evidence case overwrote the first;
- units were not checked;
- an interval with lower > upper was accepted.

Agent 0 adopted the structure: "one judge, built once. The validator emits a closed canonical evidence
view: typed, case-bound, unique identities, a unit per kind and component from the public contract,
lo ≤ hi, and duplicates refused. The comparison consumes only that view and never touches the raw
record."

**Independence.** Standard-library Python. No product code is imported or read. The formats come from
the public contract (`fixtures/results/semantic_contract_v0_3_*.json`), the committed result fixtures
(`fixtures/results/*mechanics*.json`, `preview_physics_*.json`) and the genuine PF probe records. The
runner (`runner/`) is harness, not reference.

## Files

| File | Role |
|---|---|
| `m3_json.py` | Strict JSON reading: a duplicate key, NaN or Infinity is refused. The only JSON parser of inputs. |
| `m3_types.py` | The canonical view: frozen, typed dataclasses checked on construction. No JSON reader. |
| `m3_view.py` | The validator, one per route (`pf`, `exact`): raw record → `RunView`, reference line → `ModelView` and `Expected`. `RAW_SCHEMA` states every rule. |
| `m3_compare.py` | The comparison: the K-D5 M3 rules on the view only (`judge_model`). |
| `m3_report.py` | The judging pass: provenance, inventory, M3, design conformance, nonvacuity, `OVERALL`. |
| `run.sh` | The driver: builds the runner from a clean checkout, runs every chunk, judges each variant. |
| `runner/` | The shared runner: `Cargo.toml`, `Cargo.lock`, `build.rs`, `src/main.rs`, `build.sh`. |
| `m3_selftest.py`, `m3_regress.py`, `m3_synth.py` | Self-test: isolation, contract units, rules on both views, and the round 1–6 regression table. |
| `selftest/pf_set.jsonl` | 20 lines copied verbatim from `pf_timoshenko_m3t/frozen/pft_frozen_subset.jsonl` (sha256 `1a6984ec…`) for the PF view's self-test, since that set is not on main yet. |
| `REFERENCE_SHA256SUMS` | The committed digests of the reference files the judge may judge against (provenance). |
| `SHA256SUMS` | Digests of the files above, except this README; the self-test verifies them. |

## The canonical view

A raw record becomes a `RunView` only when every field the judge reads is present, typed, bound to its
model and unique. Otherwise it is refused whole: C0, and an inventory failure. Fields the judge does
not read are type-checked where the public format fixes their shape, then dropped.
`m3_view.RAW_SCHEMA` is the full statement.

**Both routes:**

- **Outcome.** Exactly one of:
  - a runner error;
  - a refusal: one code of the route's closed set, with no rows;
  - one standing per load case, from the closed set of DEL-07-05 numerical-health-indicator.md
    section 1, with `affected_refs` exactly that case, and at least one row.

  Diagnostic ids are unique, and the same refusal may not be given twice.
- **Rows.** Each row is `Row(case, kind, entity, location, component, basis, unit, value, source)`.
  - `(kind, component, unit)` must be a row of the route's contract table.
  - The metadata basis must be one of that row's declared bases, or any basis without a section-basis
    token where the table declares none.
  - Metadata is null exactly on `displacement_magnitude`. Otherwise it has exactly the five members,
    agreeing with the row's location and component, in the fixed frame of its kind.
  - The entity is a node, pipe or support of the model, by kind; the solver row names
    `solver:linear_static_preview`.
  - The case is a load case of the model.
  - Values are finite binary64 numbers or null. Null is serde's non-finite and is judged as C1.
  - Row ids are unique, and so are identities `(case, kind, entity, location, component, basis)`.
- **Headlines.** Each is `Headline(basis, is_null, value)`, with unit `Pa`. Its `result_ref` must be
  the id of a maximum row of its `location_ref` and basis.
- **Extrema.** Each is `Extremum(case, pipe, basis, lower, upper)`:
  - exactly one evidence case object per load case;
  - `result_id` the id of the maximum row of the same case, pipe and basis;
  - lower ≤ upper whenever both are published;
  - `(case, pipe, basis)` unique.

  Nothing else in `contract_evidence` enters the view. An entry placed anywhere else, nested or in a
  well-typed but unread member, does not exist for judging.

**PF** (`semantic_contract_v0_3_preview_physics_2.json`, `product_preview_mechanics_v1`):

- one section family: no row may carry a section-basis token, and the view's basis is `None`;
- `contract_evidence` is `{combination_gates: [], preview_cases}`, each case
  `{load_case_id, pipe_stress_extrema, intensified_measures: [], stress_maximum_coverage,
  support_attribution}` in the fixtures' closed shapes;
- headline `max_open_formula_stress`;
- refusals: the five codes of pf-timoshenko.md.

**Exact** (`semantic_contract_v0_3_pressure_2.json`, `exact_pressure_v3`):

- both D-5 bases, `nominal_section` and `reduced_section`: every judged stress kind carries one,
  through the contract's declared token, and no other judged kind does;
- `contract_evidence` is `{exact_cases, connector, pressure}`, with each extremum's `section_basis`
  required;
- `max_open_formula_stress` must be null, beside one headline per basis;
- refusals: the six codes the exact route's designs name. The two codes U6 and U8 removed are not
  among them.

The derived rows (global end actions, σ±, τ_v,max) are judged as information (C11) on PF. On the exact
route the K-D5 reference does not model them, so they are counted, not judged.

**References (the judge's final round).** Agent 0: "every datum the judge uses is either (a) derived
from the hash-bound request, parsed into the view and unique, or (b) read from a reference file whose
sha256 the judge verifies against the committed SHA256SUMS as part of provenance. Nothing else in a
reference is read." Admission is as total as a run's: with any problem nothing is judged.

| Datum | Source | Check |
|---|---|---|
| Node ids and positions | request `model.nodes` | ids unique; positions finite binary64 (m) |
| L_b (coupled scales, S*) | request node positions | the bounding-box diagonal, formed as before; equal bit for bit to the former `ref.nodes` value on all 2,054 lines of the six reference files |
| Connected bodies | request pipes | more than one body, or any component: C0 |
| Pipe ids and ends | request `model.pipe_segments` | ids unique; `from` and `to` defined, distinct nodes |
| Support ids and nodes | request `model.supports` | ids unique; `node` defined |
| Load cases | request `model.load_cases` | ids unique; at least one |
| Entity namespace | the request | no id names two entities (nor the solver entity) |
| Request binding | sha256 of the request line | every run's `request_sha256` must equal it (provenance) |
| Reference file | the set file | sha256 committed for the route in `REFERENCE_SHA256SUMS`, itself tracked and unchanged in the judge's checkout (provenance) |
| Controls file | `controls.jsonl` | the same, kind `controls` |
| Family, theory, twin, reporting keys | `meta` of the verified line | `design`, `family`, `theory`, `placement`, `section_variant` and `twin` are strings |
| Section properties | `ref.sections[pipe]`, verified | exactly the request's pipes; `ro` and per basis `A`, `Z`, `J` decimal texts; PF also `I`, `tau_c` and `E`, `G`, `od`, `wall`, `mill` |
| Expected rows and maxima | `ref.cases[case]`, verified | exactly the request's load cases; every row's entity a request entity of its kind; section basis by route; keys unique |
| PF admission | `ref.expect`, `ref.refusal.code`, `ref.expected_warnings`, verified | `rows` or `refused`; a code of the PF refusals; strings |
| Twin reference | the twin's line in the same verified file | present; admitted against this model's request |
| Control reference | `pertE` of the verified controls file | every entry `{tag, pertE}` only, its tag a model of the set and unique, its `pertE` admitted as a reference whether or not it is used |

`ref.nodes`, `ref.theory`, `phi`, the section members not listed, `region_members`, `notes` and every
other member are not read. The view holds only exact immutable types: `type(x) is ...` gates on every
field and container, so no subclass of `str`, `float`, `Fraction`, `tuple`, `frozenset` or a view type
is admitted.

`REFERENCE_SHA256SUMS` lists the six reference files: each route's full set (and the exact route's
controls), which regenerate byte for byte from their committed generators, and the two frozen
subsets and the PF selftest set. A new or changed reference set is judged only after its digest is
committed there. The judging library's `references` argument lets the self-test declare its synthetic
sets; the CLI and `run.sh` always read the committed file.

The `twin` variant takes the twin's reference. The exact route's `control` variant takes the
`controls.jsonl` `pertE` reference; models without a control are set aside and counted.

## How the comparison is kept from the raw record

1. **Types.** `m3_compare.judge_model(model, expected, runs)` requires exactly a `ModelView`, an
   `Expected` and a `tuple` of `RunView` of that model (`type(x) is ...`). Anything else raises
   `TypeError`: a raw dictionary, a list, or a subclass of a view type. The view types are frozen and
   checked on construction with exact types.
2. **Imports.** `m3_compare.py` imports only `__future__`, `collections`, `math`, `fractions` and
   `m3_types`. `m3_types.py` imports no JSON reader.
3. **A test asserts both.** The self-test parses `m3_compare.py` and checks its imports. In a fresh
   interpreter it imports `m3_compare` and checks that `json`, `m3_json`, `m3_view` and `m3_report`
   were not loaded. It passes a raw record, a raw reference line, a raw reference and a list to
   `judge_model` and expects `TypeError` each time.
4. **Round 5 replayed.** `shadow_evidence_unread_member` and `exact_shadow_evidence_unread` place a
   good extremum in a well-typed member the view does not read, and set the read one to 0. Both fail
   C2: the comparison never sees the good one.

## The rules

`m3_compare.py`'s docstring states every rule with its source. These are `kd5t_compare.py`'s
reconstruction (`../kd5_timoshenko/`, refutation rounds 1–4), ported unchanged in meaning, with PF's
information clauses (C11, C11tau_b, C11ops) and design conformance (C10). The view changes eight
things:

- **C9.** A repeated row identity is refused by the view, whatever its value, order or standing. The
  record is schema-invalid, so C0 and an inventory failure. Before, a same-value repeat was C6i
  information and so was a conflict on a non-Passed run.
- **Section bases.** A basis-free stress row on the exact route, or a section-basis token on PF, is
  refused. Before, it was C6i information.
- **Exact headline.** A non-null `max_open_formula_stress` on the exact route is refused. Before, it
  was C7 information.
- **Units, frames and metadata.** A wrong unit, frame, or metadata location or component is refused.
  Before, none was checked.
- **Null first** (the final round). A published null of any kind, derived or not judged included, is
  C1 on a Passed case, checked before any kind-based skip, as in `kd5t_compare.py`'s original judge.
- **L_b from the request.** The coupled scales use the request's node positions, not `ref.nodes`.
- **Infinite values.** An infinite value cannot be published; strict JSON already refused one from a
  file. A null value is still C1 on a Passed case.
- **Standings per load case.** Each load case is judged under its own standing. On these single-case
  sets this is the same as before.

## Nonvacuity (Agent 0, decided)

The judging pass counts the judged groups on Passed cases, overall and per family, beside the runs,
Passed runs and Sensitive runs. These counts appear in the text report and in the JSON (`nonvacuity`).

NONVACUITY fails when the overall count is 0, or when any family of the reference set has 0. Only a
family named in `m3_report.EXEMPT` is excused, and each such family carries its reason.

**Exempt families: none.** Neither set has a family that is all-Sensitive or all-refused by
construction:

- the near-Sensitive families straddle the gate's boundary (r̂ ≈ 2^-26), so they are tuned to give
  both standings;
- PF's expected refusals belong to `near_cancellation`, beside its admitted models.

A family found all-Sensitive by construction would be named here with its reason and added to
`EXEMPT`. The exemption mechanism is self-tested (`nonvacuity_exempt_family_mechanism`).

## The judging pass (`m3_report.py`)

```
python3 -I -B m3_report.py --route pf|exact <set.jsonl> <raw outputs...> --runner-sha256 <hex64> --product-head <hex40> \
        --checkout <checkout> [--ref own|twin|control --controls <controls.jsonl>] [--workers N] [--eval-out <new>] [--json <new>] [--by family]
```

One invocation judges the whole collection from the raw outputs. Evaluation records are an output,
never an input.

1. **PROVENANCE.** The reference file (and the controls file) must be committed in
   `REFERENCE_SHA256SUMS` for the route. The runner sha256 and head are required. The head must resolve with
   `git rev-parse --verify <head>^{commit}` in `--checkout`. Every record must carry exactly that
   runner and head, and its request sha256 must equal its model's runner-input line. The reference
   must be admitted (above).
2. **INVENTORY.** Every model × entry × mode exactly once, as a successful view, and nothing else.
3. **M3 VERDICT.** C0–C4, C6, C8 and C9.
4. **DESIGN-CONFORMANCE VERDICT.** C10, PF only.
5. **NONVACUITY.**

`OVERALL: PASS` and exit 0 only when all five pass. Outputs are created exclusively: an existing path,
or one that aliases an input through a symlink or a hard link, is refused before anything is written.
The pass is total: an unforeseen exception is a named failure, never a traceback.

## Runner, `run.sh` and `runner/build.sh`

`runner/` is the PF M3-T runner, which is the K-D5 runner plus the provenance of each record. It is
one crate for both routes (`m3_judge_runner`, `M3J_PRODUCT_HEAD`).

**`run.sh <route> <checkout> <set dir> <run dir> own [twin] [control]`:**

- builds the runner with `runner/build.sh`, from the checkout's committed runner;
- runs every chunk fresh, failing on any non-zero runner exit;
- judges each variant.

A sensitivity control counts only from a completed report: provenance and inventory PASS, M3 FAIL.

**`build.sh`:**

- refuses a dirty `core/`, `fixtures/`, `schemas/` or `validation/` (exit 3);
- `M3J_ALLOW_DIRTY=1` builds anyway with the head marked `-dirty`, which the judge refuses;
- builds with `--locked`.

`../kd5_timoshenko/run.sh` calls this driver with `exact`.

## Threat model

Inputs are the committed runner's output on a clean build at the stated head; the judge fails closed on malformed, contradictory, duplicated, missing or misbound input. It is not authentication against a rebuilt, lying runner.

### Outside the model

Findings that a later round places outside this model, each with its reason:

- **Raw hash and head fields are assertions: identical forged values pass** (round 4,
  `forged_identical_provenance`). A record's `runner_sha256` and `product_head` are what the runner
  says. The judge binds them to `--runner-sha256` and a head that is a commit of `--checkout`, and
  `run.sh` binds those to the binary it built from that clean checkout. A party who writes records
  with the same forged values, and states them, is a lying runner.

## Self-test

`python3 -I -B m3_selftest.py` checks 1,362 of 1,362. It covers:

- isolation (above), the exact-type gates (ten subclass probes, refused), and the contract units of
  the judged kinds;
- every rule and clause on both views: CRIT from both sides on every group kind, C3 from both sides,
  F1–F4, C2, C4/C4b, C6/C6i, C7, C8, C10, C11, the τ defect, σ± and τ operations, the twin control,
  the Lamé and nominal/reduced mutants;
- 223 regression cases.

`--quick` runs only the isolation checks and the regression cases. Each regression case prints
`TABLE round | attack | case | outcome`. By round:

| Round | Cases |
|---|---|
| Positive controls | 12 |
| 1 | 9 |
| 2 | 6 |
| 3 | 23 |
| 4 | 55 |
| 5 | 82 |
| 6 (the final round) | 36 |

- **PF view.** Rounds 1–4 are the PF judge's table, replayed. Round 5 covers:
  - the shadow evidence: nested, and in an unread member;
  - duplicate extrema and evidence cases;
  - units: rows and headlines;
  - inverted intervals, contradicting metadata and frames, and repeated rows;
  - every standing and double standing, every refusal code and repeated refusal;
  - duplicate JSON keys and malformed solver rows;
  - malformed or unbound summary and evidence fields;
  - nonvacuity.
- **The final round, both routes.** A reference node moved to 1e20 or duplicated there, with a C3
  defect (M3 still fails: `ref.nodes` is not read; and the edited file is not committed); a request
  node moved (request sha256), a duplicate node id, an undefined pipe end or support node, an id naming
  two entities, a reference row of no request entity and a malformed section (reference refused); a
  contract-valid derived row published null on Passed runs (C1, null first); every controls defect
  (unknown entry, malformed `pertE`, both, duplicate tag, extra key, file not committed); the committed
  digests themselves (untracked, edited); a reference not committed, a hash mismatch and the other
  route's file, through the CLI and `run.sh`.
- **Exact view.** Agent 0's required three (null error, missing run, duplicate run), and round 5's
  four: shadow, duplicate, unit and inverted interval. Also the twin and control variants, and
  cross-route records.

Passing by design:

- the positive controls;
- `forged_identical_provenance` (outside the model);
- `refusal_PF_TIMOSHENKO_SHEAR_FACTOR_UNAVAILABLE`;
- `standing_CHECKS_PASSED`;
- `nonvacuity_exempt_family_mechanism`;
- the run.sh recoveries.

`bad_rows_sensitive_only`, which passed through round 4, now fails nonvacuity.

The rules were also checked on genuine format. All 28 genuine PF probe records validate into the PF
view, with request hashes matching. The six exact-route straight-member result fixtures validate into
the exact view. The fixtures that do not validate have arcs, components or nonlinear supports, which
neither set contains.

## Not covered

- Arcs, components (C0), more than one body (C0), nonlinear supports and rows without a load case:
  outside both sets.
- The exact route's derived rows: counted, not judged, because the K-D5 reference does not model
  them.
- Anything outside the threat model (above).

## Reproduce

```
python3 -I -B m3_selftest.py                         # isolation, contract units, rules on both views, rounds 1-5
python3 -I -B m3_selftest.py --quick | grep '^TABLE'  # the regression table
./run.sh exact <clean checkout> <kd5 set dir> <run dir> own twin control
./run.sh pf <clean checkout> <pf set dir> <run dir> own twin
```
