#!/bin/sh
set -eu
label="$1"
shift
record="$PWD/../../execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/B3-CODEX/ci-tooltip-repair/_run_records/$label"
mkdir -p "$record"
export PLAYWRIGHT_WORKERS=1
PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH="$(node -e 'console.log(require("@playwright/test").chromium.executablePath())')"
export PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH
printf '%s\n' "$PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH" > "$record/executable.txt"
"$PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH" --version > "$record/version.txt"
printf '%s\n' "$@" > "$record/arguments.txt"
sh ../../execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/tools/with_e2e_lock.sh "$@" > "$record/result.log" 2>&1
