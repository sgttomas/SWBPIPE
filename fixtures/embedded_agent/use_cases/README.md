# Embedded-agent use cases

Each directory here is one named use case that `tools/agent_journey_fixture.py`
runs in the native-smoke desktop (`--use-case <id>`). It holds two files. The
controller validates both before anything is launched, and
`tools/agent_journey_fixture_test.py` validates every directory offline.

A directory holds both files together, so the use case, its scripted output
and any later inputs (a model document to open, a stored project) stay in one
place. The definition is never mixed with model output.

## `use_case.json`: the person's side

Nothing in it may depend on which provider answers. A member named `provider`,
`endpoint`, `model`, `key`, `apiKey`, `contextWindow`, `stub` or
`scripted_provider` anywhere in it is refused.

- `schema`: `swbpipe-embedded-agent-use-case/1`. `id`: the directory name.
  `summary` and `data` are prose.
- `start`: how the arrangement reaches the stated start before the agent
  starts. Kinds the driver implements (`STARTS` in
  `apps/desktop/native-smoke/agent-journey.js`, `START_KINDS` in the
  controller): `new_blank` (File > New Blank Project and its units dialog).
- `sequence`: in order, `{"request": "<text typed to the agent>"}` and
  `{"act": "<kind>"}`. An act is the test arrangement standing in for the
  person, only through the desktop's own controls. Kinds implemented (`ACTS`,
  `ACT_KINDS`): `apply_agent_batch` (Batch review's own Validate, when its
  preview gate asks, and Apply entire batch). No request text may contain
  another, because requests are recognised by their text.
  `{"requested_act": "<act>"}` is a requested act: an act the agent asks the
  person to perform, which the stand-in performs at the desktop's own control.
  Its kinds are a closed list: `save_model_state`, `record_run`,
  `save_project`, `export_table`, `save_report`, `print_report`,
  `user_release`. The driver performs none yet (`PERFORMS`,
  `PERFORMED_BY_DRIVER`), so such a definition validates but is refused at
  launch.
- `observe`: what the driver reads from the window at the end: the model
  tree's rows of `tree_types`, and the Results section filtered by
  `result_filter`, with its solve proof.
- `references`: named inputs to the expectations. Each is either `{"file",
  "path"}`, a member of a repository JSON file, or `{"file", "case"}`, a
  benchmark case of a reference file.
- `expectations`: what the judge applies. Each has an `id`, a `kind` the judge
  implements (`CHECKS` in the controller), `on` (the `window`, the
  `conversation` or their `agreement`, which must be the kind's own) and
  exactly that kind's parameters. Every expectation must make at least one
  comparison, or the run fails. The window, the conversation and their
  agreement must each have an expectation that is judged in every mode.
  `tool_outcomes_agree_with_window` needs the tool outcomes that only the
  scripted provider sees. A live run reports it under `notJudged`.

## Requested acts, things a person observes, and the result's outcome

A request is settled as performed only on the owning route's receipt, and a
route that gives no receipt leaves its request short of performed. So a use
case with a requested act is complete for the agent only if the conversation
shows the agent was given the standing `performed` for each requested act.
Each requested act must have exactly one `requested_act_standing`
expectation, and any other standing, or none seen, leaves the use case
unfinished for the agent. A live run cannot see the agent's tool outcomes,
so it always reports such a use case as unfinished.

- **Print.** A print request reaches `sent to the print dialog` only on the
  route's receipt of the hand-over of the captured report. That standing is
  still short of performed and leaves the use case unfinished. A print
  request reported as `performed` is a failure.
- **Reachable standings.** The catalog lists the standings each act's
  request can reach. The judge reads the standing it is given and assumes
  none is reachable.
- **Provisional location.** The judge reads the standing from tool outcomes
  at `data.requestedAct` and `data.requestStanding`. That location is
  provisional until the catalog defines the request observation.

Some things only a person's hands observe; the runner cannot. An
`observed_by_person` expectation names one, from a closed list:
- `copy_pressed_on_selected_table`: pressing Copy on a selected table.
- `print_dialog_outcome`: what the person did in the system print dialog.

It must also give:
- `machine_observed`: a sentence saying what the machine observed short of
  that thing;
- `machine_check`: an earlier expectation the machine judges.

The validator refuses one without either, so it cannot replace a check the
machine can make. The judge never passes or fails it. It lists it under
`expectations.observedByPerson` so the list can be handed to a person.

The result's `outcome` is one word. Precedence runs from first to last:

| `outcome` | Meaning | Exit status |
| --- | --- | --- |
| `failed` | Something failed; `failures` and `failure` say what. | 1 |
| `unfinished` | The route passed, but a requested act was not shown to the agent as performed. | 3 |
| `awaiting a person` | The route passed and every requested act was performed, but something listed is for a person to observe. | 4 |
| `passed` | None of the above. | 0 |

The two kinds of shortfall close differently. `unfinished` closes only on a
receipt from the owning route, given to the agent as `performed`.
`awaiting a person` closes only when a person observes each listed thing. For
both, `passed` is false with no failure named, and the summary line carries
`outcome` and both lists.

A scripted provider run examines the route only and shows nothing about what
a model does.

## `scripted_provider.json`: model output for stub mode only

The controller reads this file only in stub mode, and a live run refuses it.
It replays model output on loopback so the route is examined under a scripted
provider. That shows nothing about what a model does.

- `schema`: `swbpipe-embedded-agent-scripted-provider/1`. `use_case`: the id.
- `inputs`: named members of repository files that a call may carry, such as
  a proposal's operations.
- `requests`: one entry per request of the definition, in order. Each entry
  holds:
  - `steps`: the tool calls the scripted model makes, one per answer. Each
    step has a `call` (a tool name suffix), `arguments`, and a `bind`, the name
    its outcome is known by later.
  - Optionally, `expect` (dotted field path to value) with an `unexpected`
    message. The message may name outcome fields as `{status}`.
  - `reply`: the final message. It may include `records` built from a bound
    list.
  - `requires`: checks on outcomes bound by earlier requests.
- Inside `arguments`, `{"$bound": name, "path": [...]}` takes part of an
  earlier outcome. `{"$input": name, "path": [...]}` takes part of an input,
  and `"*"` in a path maps over a list.
- Outcomes are checked field by field, never by equality with a whole object,
  so members the product adds later do not disturb a script. An outcome that
  departs from its step's `expect` ends the turn and fails the run.
