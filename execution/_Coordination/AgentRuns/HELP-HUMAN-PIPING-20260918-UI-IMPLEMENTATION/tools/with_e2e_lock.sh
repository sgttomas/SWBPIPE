#!/bin/sh
# One browser-test run at a time on this host, across every git worktree of this repository.
# The Playwright configurations use fixed ports (5174 source lane, 5175 dist lane) and reuse a
# server that is already listening, so two concurrent runs from two worktrees would test each
# other's build. Wrap every Playwright run, every dev or preview server, and the evidence sweep:
#   sh with_e2e_lock.sh <command> [args...]
# The lock is a directory in the shared git directory. A lock whose owner process is gone is cleared.
LOCK="$(git rev-parse --path-format=absolute --git-common-dir)/swbpipe-e2e.lock"
while ! mkdir "$LOCK" 2>/dev/null; do
  owner="$(cut -d' ' -f1 "$LOCK/owner" 2>/dev/null)"
  if [ -n "$owner" ] && ! kill -0 "$owner" 2>/dev/null; then rm -rf "$LOCK"; continue; fi
  sleep 15
done
echo "$$ $(date -u +%Y-%m-%dT%H:%M:%SZ)" > "$LOCK/owner"
trap 'rm -rf "$LOCK"' EXIT INT TERM
"$@"
rc=$?
exit $rc
