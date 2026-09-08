# Failed command 003

Working directory: repository root.

Exact command (newlines preserved):

```sh
set -o pipefail
manifest='projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/PHYSICS_UI_IMPLEMENTATION_20260908/FROZEN_NINE_PATH_MANIFEST_V1.json'
out='projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/_run_records/hash-verification.txt'
: > "$out"
printf '%s\n' 'MANIFEST_DECLARED_19_MEMBER_CHECK' | tee -a "$out"
jq -r '(.source_files + .evidence_files)[] | [.sha256,.path] | @tsv' "$manifest" | while IFS=$'\t' read -r expected path; do actual=$(shasum -a 256 "$path" | awk '{print $1}'); if [ "$actual" = "$expected" ]; then verdict=MATCH; else verdict=MISMATCH; fi; printf '%s  %s  %s\n' "$actual" "$path" "$verdict"; done | tee -a "$out"
printf '%s\n' 'BASIS_AND_REVIEW_BINDINGS' | tee -a "$out"
shasum -a 256 projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/LAUNCH_BRIEF_V1.md projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/OWNER_ACT.md projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/amendments/U7/SOURCE_RELEASE_V1.md 'projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/PHYSICS_UI_EXECUTION_20260908/IMPLEMENTATION_BRIEF_V1.md' 'projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/PHYSICS_UI_EXECUTION_20260908/CONSUMED_INTERFACE_EVIDENCE_V1.md' projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-EXECUTION/instances/PS/PS1/RETURN.md projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/U7/MANAGER_RETURN_V1.md projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/U7/children/I1/REPAIR_RETURN_V1.md | tee -a "$out"
printf '%s\n' 'DIFF_SHAPE' | tee -a "$out"
wc -l 'projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/PHYSICS_UI_IMPLEMENTATION_20260908/U7_FINAL_NINE_PATH.diff' | tee -a "$out"
rg -c '^diff --git ' 'projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/PHYSICS_UI_IMPLEMENTATION_20260908/U7_FINAL_NINE_PATH.diff' | tee -a "$out"
if rg -q 'MISMATCH' "$out"; then exit 1; fi
```

Exit was `1`. Complete command output is preserved in `hash-verification-attempt1.txt`.

Cause: the zsh loop variable named `path` shadowed zsh's special `PATH` array, so commands inside the loop were reported unavailable and all 19 entries were falsely printed as mismatches. The basis hashes and diff-shape checks later in the same command completed normally. The rerun uses an inert loop variable plus absolute executable paths.
