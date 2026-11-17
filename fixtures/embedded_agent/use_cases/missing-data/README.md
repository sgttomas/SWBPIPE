# Missing data (DEL-07-06 AC-007)

The use case: the person opens a model that lacks values a solve requires.
Every missing value is shown as a finding on its entity. From each finding the
person reaches the entity and enters the value. The solve, refused before,
then completes.

This directory holds that use case's input and its expected outcome. The
offline judge `tools/agent_missing_data_judge.py` compares a trace with them.
No driver runs this use case yet; the desktop run waits for the use-case
driver of PR 251. Nothing here is a run record.

## Files

- `start_model.json`: the stated start, opened through the product's own File
  > Open. It is an invented, pressure-free model: two 1 m pipe spans from an
  anchor, one invented steel-like material, and one seismic static-equivalent
  load case (0.4 g lateral, gravity entered as 9.81 m/s^2). Neither pipe states
  its `section.material_density`, which the seismic case needs. Nothing else
  is missing.
- `scenario.json`: the requests typed to the agent, and the expectations: the
  exact missing values as entity and field. Only `requests` reaches the
  provider.
- `person_entered.json`: for the harness only. These are the values the
  harness, standing for the person, enters through the property inspector:
  7850 kg/m^3 inboard and 8000 kg/m^3 outboard. They are invented and come
  from no standard or catalog. The values are not in the requests, so the
  agent cannot supply them.

The expected final model is the start model with those two entries and no
other change. `product_managed_paths` is empty: no change the product makes
on its own is admitted until it is documented here with its reason and given
a comparison rule.

## Why this model

The authored lateral acceleration requires each span's mass, so both material
densities are required inputs. Their values are independent authored facts in
`person_entered.json`, not values inferred from solver output. The inspector's
optional-quantity operation can enter each absent density with its stated unit.
A refused solve names both pipes with `material_density`
(`EQUIVALENT_STATIC_INPUT_MISSING`).

The scenario explicitly supplies gravity and all three axis factors. The
generator requires at least one entered axis factor; an absent axis is
unselected, while no entered factors produces a blocking finding. Separately,
the scalar inspector refuses an absent material `shear_modulus.value` as
`OP-QUANTITY-OBJECT-MISSING`; that is not a refusal by the whole applier, whose
constitutive-properties operation can supply G. Neither behavior changes this
scenario's authored missing values.

The desktop's on-demand model checks do not run the static-equivalent
generation, so before the first solve they show no finding for these values.
Each missing value is shown as a finding once that solve is refused. This is a
product gap against DEL-07-07 REQ-008 and DEL-07-02 REQ-004, not evidence that
pre-solve detection is complete; the scenario does not waive those requirements.
`tools/agent_missing_data_judge_test.py` solves the start and the expected
final model with the headless runner. The start must end `MODEL_INCOMPLETE`,
with blocking findings that name exactly the two missing values, and the
final model must solve. That is a check of the route, not of the mechanics.

## What the driver must observe

The judge reads one observation
(`swbpipe-agent-missing-data-observation/2`). Every member except
`agent_report` is what the harness observed itself.

The contract: every fact the judge relies on comes from the trusted observer,
bound to the model it concerns.

- An executed solve, whether it is refused or not, carries the model binding
  of the job that started, and custody that contradicts that binding is
  rejected.
- A withheld Solve is evidenced by the harness-observed request identity and
  the gate's own record. No native job is required, because none should
  exist, and a native job that appears for a withheld Solve is itself a
  finding.
- Blocked native custody stays distinct from a run handle the agent can read.

These rules bind every execution, whatever its position (first, any in
between, last) and whatever its outcome. The only difference is the one
stated above for a withheld Solve. The scenario's own rules for the first and
the last execution apply on top of them.

The members:

- `runner_mode`: only `agent with a scripted provider` is admitted.
  `observer`: `desktop_harness`.
- `opened_model_text`: the exact text handed to File > Open. It must equal
  `start_model.json`.
- `executions`: every solve request the harness observed, in order. Each
  gives:
  - `request_id`: the harness-observed identity of the request;
  - `outcome`: one of withheld, refused, model_incomplete, completed, failed,
    cancelled;
  - `job_id`: the host's native job identity. It is required for every
    outcome except withheld, and must be null for withheld;
  - `gate_record`: for withheld only, the gate's own record
    (`solve_offered` false). It is null for an executed solve;
  - `model_text_sha256`: required, as 64 lowercase hexadecimal characters.
    It is the SHA-256 of the exact UTF-8 model text the job received or, for
    withheld, the text the gate evaluated. The first execution must be bound
    to `start_model.json`;
  - `custody_match`: the host's match of the submitted model text (`model`)
    and result (`result`, null when the job holds none) against the job
    (DEL-07-07 REQ-015), or null. Where it is recorded, neither member may be
    false, and `result` is null exactly when no result is held;
  - `native_custody`: `none`, `held` or `blocked`. `blocked` is a result held
    natively that the agent cannot read. Only a completed execution holds a
    result;
  - `result_handle`: the run handle the agent can read, or null. It is only
    given over `held` custody;
  - `findings`: the findings this execution, or the gate, supplied;
  - `results`: values the harness read from this execution's held result,
    empty where none is held.
- `initial_execution` (the first execution) and `final_execution` (the last):
  `request_id`s of two different executions. Request identities, native job
  identities and run handles are each unique across the trace. The final one must be completed,
  `held`, readable through its handle, matched by the host for both model and
  result, and bound to `final_model_text`.
- `findings_shown`: the Diagnostics list after the refusal. Each finding gives
  `finding_id`, `entity_ref`, `field_path` and `blocks_solve`, as the harness
  read them from the finding. The product supplies the entity and the field
  leaf in `affected_refs`.
- `steps`: in order, one pair per missing value. First the activation of the
  finding (`navigate`: `finding_id`, `selected_entity_ref`). Then the entry
  (`enter`: `actor` harness, `control` property_inspector, `entity_ref`,
  `field_path`, `value`, `unit`, `applied` true). Any other step fails, and
  so does an agent operation that changes the model.
- `final_model_text`: the exact text the final job received. It must be a
  model document (a JSON object) equal to the start with the person's entries.
- `agent_report`: what the agent said: the job and run handle it cites, the
  result values it quotes (two distinct values, as the request asks), and the
  missing values it reported.

The judge never binds by a producer `run_id` or by equal numbers. A quoted
value is accepted only from the final execution, by job id and handle, even
when another solve gives the same numbers. Every member is checked before it
is used. A malformed one fails by name, and the judge never raises on it.

## Limits

Truthful refusal reporting, harness entry of the person's repairs and a subsequent
solved model do not establish pre-solve detection or completion of exit criterion
8's Missing data use case while the pre-solve finding gap remains.

The judge cannot authenticate the harness: a forged observation that is
consistent with itself passes. A pass is
`trace consistent offline; route only under a scripted provider`. It is not
live or native evidence, and under a scripted provider it shows nothing about
what a model does. It is not acceptance or the owner's verdict. The traces in
the tests are synthetic inputs to the judge.
