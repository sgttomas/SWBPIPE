# Failed command 003

Purpose: final packet verification. The shell exited `0`, but review identified two command-construction errors: the e2e path was mistyped as `apps/desktopd/e2e/...`, and `>> = "$out/..."` created an unintended repository-root file named `=` rather than appending the tracked-diff marker. The attempt output is preserved in `_run_records/final-verification-attempt1.txt`; the stray `=` file was immediately removed with an exact path guard before the corrected rerun.

```sh
set -eu
run_root='projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION'
out="$run_root/instances/RU/successors/RU-F1-F5-BACKCHECK"
: > "$out/_run_records/final-verification.txt"
for required in REVIEW.md REVIEWED_INVENTORY.sha256 VALIDATION.md RETURN.md STATUS.json; do
  test -f "$out/$required"
  printf 'PRESENT\t%s\n' "$required" >> "$out/_run_records/final-verification.txt"
done
jq . "$out/STATUS.json" >/dev/null
printf 'STATUS_JSON_VALID\n' >> "$out/_run_records/final-verification.txt"
while IFS= read -r line; do
  case "$line" in ''|'#'*) continue;; esac
  expected=${line%%  *}
  member=${line#*  }
  actual=$(/usr/bin/shasum -a 256 "$member" | /usr/bin/awk '{print $1}')
  if [ "$actual" != "$expected" ]; then
    printf 'MISMATCH\t%s\texpected=%s\tactual=%s\n' "$member" "$expected" "$actual" | tee -a "$out/_run_records/final-verification.txt"
    exit 1
  fi
  printf 'MATCH\t%s\t%s\n' "$member" "$actual" >> "$out/_run_records/final-verification.txt"
done < "$out/REVIEWED_INVENTORY.sha256"
test ! -e projects/chirality-piping/apps/desktop/test-results
printf 'PLAYWRIGHT_MARKER_CLEAN\n' >> "$out/_run_records/final-verification.txt"
git diff --check -- projects/chirality-piping/apps/desktop/src/App.tsx projects/chirality-piping/apps/desktop/src/App.test.tsx projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport.tsx projects/chirality-piping/apps/desktop/src/features/viewport/routeDraft.ts projects/chirality-piping/apps/desktop/src/features/viewport/routeDraft.test.ts projects/chirality-piping/apps/desktop/src/features/model-tree/PropertyInspector.tsx projects/chirality-piping/apps/desktop/src/features/model-tree/typedInspector.test.tsx projects/chirality-piping/apps/desktop/src/styles.css projects/chirality-piping/apps/desktopd/e2e/linear-authoring.spec.ts
printf 'TRACKED_DIFF_CHECK_CLEAN\n' >> = "$out/_run_records/final-verification.txt"
/usr/bin/shasum -a 256 "$out/REVIEW.md" "$out/REVIEWED_INVENTORY.sha256" "$out/VALIDATION.md" "$out/RETURN.md" "$out/STATUS.json"
git status --short -- projects/chirality-piping/apps/desktop "$out"
```
