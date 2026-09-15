# Future-freeze revisits

These files are excluded from the draft policy delta because their owning compatibility/native work or Root qualification is still active. Their current hashes are observations only and are not preapproved.

- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE/briefs/FINAL_NATIVE_CONSUMER_PREPARATION_V4.md` — current role `UNCLASSIFIED`, current SHA-256 `4690f79e2afce405306a64be04fbdb97f2cc65594036b4627690a79d2ad1b415`, 2 occurrence(s).
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE/briefs/NATIVE_SIGNATURE_COVERAGE_PREPARATION.md` — current role `UNCLASSIFIED`, current SHA-256 `23671cf541a3b85487747358f2fd3991485175ef2802a7597d1be5d8fb9396f8`, 1 occurrence(s).
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE/instances/COMPATIBILITY/ANALYSIS_CANONICALIZER_CHECKPOINT_V1_QUALIFICATION.json` — current role `UNCLASSIFIED`, current SHA-256 `41ebea74705f315b1959a03514776b1a9ffc9b58c063815bf800629b5774e007`, 3 occurrence(s).
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE/instances/COMPATIBILITY/CONTRACT_AND_TEST_BASIS.md` — current role `UNCLASSIFIED`, current SHA-256 `4197943bffc1e28fa46b2909d529eff609ca6cb882f0a99bae08172ae6464757`, 1 occurrence(s).
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE/instances/COMPATIBILITY/TASK_WRITER/ANALYSIS_CANONICALIZER_CHECKPOINT_V1.json` — current role `UNCLASSIFIED`, current SHA-256 `1981bc63979a36a26596699da2ec52e9fb04e7e18346845f9ccf27a996e27847`, 5 occurrence(s).
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE/instances/COMPATIBILITY/TASK_WRITER/BRIEF.md` — current role `CONTROL`, current SHA-256 `cd87caacf6622a979c37ee7577de3ffa1ca255cbc2ca556ed75f9975904112d6`, 1 occurrence(s).

Also recheck these mutable or state-conflicted evidence surfaces even though the classifier already treats them as evidence and they need no policy entry:

- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE/instances/ROOT/_run_records/DISPATCH.json` — live Root dispatch ledger; modified in the working tree at audit time.
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE/instances/COMPATIBILITY/TASK_WRITER/STATUS.json` — structurally evidence, but it currently says `RUNNING` while `RETURN.md` says complete with Root-held qualification. Root must resolve the final state before freeze.
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE/instances/COMPATIBILITY/TASK_WRITER/RETURN.md` and `_run_records/FINAL_FOCUSED_EVIDENCE_V2.json` — returned evidence that remains subject to Root’s held integration qualification.
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE/instances/NATIVE/_run_records/SIGNATURE_PREPARATION_V2/**` and any later final native consumer outputs — preparation is in progress; inventory exact files and hashes only after completion.

Portable-successor requirement:

- Replace the active `FINAL_NATIVE_CONSUMER_PREPARATION_V4.md` with a versioned control whose checkout is resolved from the supplied working context (for example, `git rev-parse --show-toplevel`) and whose compatibility-source binding is a Root-supplied repo-relative path plus accepted checkpoint hash.
- If `NATIVE_SIGNATURE_COVERAGE_PREPARATION.md` remains consumable, issue a versioned portable control that resolves the repository root at runtime; otherwise freeze it as an executed/superseded control and add an exact control exception after its completion evidence exists.
- Until Root completes compatibility qualification, treat `CONTRACT_AND_TEST_BASIS.md`, the qualification record, writer checkpoint, and writer `BRIEF.md` as active. A portable successor must replace the literal temporary Python path with the repository builder entry point and a runtime-resolved interpreter/controller binding. Once superseded and frozen, classify the executed brief with an exact control exception and the immutable result records with historical overrides using their final hashes.
