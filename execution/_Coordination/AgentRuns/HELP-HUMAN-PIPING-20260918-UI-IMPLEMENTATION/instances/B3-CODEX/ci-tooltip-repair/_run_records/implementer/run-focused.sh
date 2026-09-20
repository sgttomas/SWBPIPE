#!/bin/sh
set -eu
# Run from projects/chirality-piping/apps/desktop; first argument is evidence label.
label="$1"
shift
record="$PWD/../../execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/B3-CODEX/ci-tooltip-repair/_run_records/implementer/$label"
mkdir -p "$record"
export PLAYWRIGHT_WORKERS=1
PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH="$(node -e 'console.log(require("@playwright/test").chromium.executablePath())')"
export PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH
export B3_A11Y_EVIDENCE_DIR="$record"
printf '%s\n' "$PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH" > "$record/executable.txt"
"$PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH" --version > "$record/version.txt"
printf '%s\n' "$@" > "$record/arguments.txt"
sh ../../execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/tools/with_e2e_lock.sh npx playwright test "$@" --output "$record/artifacts" > "$record/result.log" 2>&1
