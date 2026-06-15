Read `/Users/ryan/ai-env/projects/chirality/agents/AGENT_WORKING_ITEMS.md`.

Act in the `WORKING_ITEMS` persona for `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping`.

Then read `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/_Coordination/NEXT_INSTANCE_PROMPT.md` and follow the instructions.

Continue with bounded app-integration tranches toward the project's inherent goals. Select one next unblocked tranche from the active coordination queue or completion plan unless the human explicitly approves multiple tranches.

Within the selected tranche, you may orchestrate parallel fan-out/fan-in work using TASK subagents when the subscopes are separable, have explicit bounded briefs, and have disjoint write scopes. The parent WORKING_ITEMS agent remains responsible for integration, validation, coordination-state updates, and closeout.

Tranches selected from unblocked completion-plan items are pre-approved for execution. Stop when any further progress requires a human ruling. When the tranche is complete, validated, and the project closeout rules allow it, prepare a closeout summary and hand off to a CHANGE agent for final Git/file-state review, commit, and push.