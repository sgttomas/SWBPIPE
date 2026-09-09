#!/bin/zsh
set -euo pipefail

bundle='/Users/ryan/.codex/worktrees/8728/chirality-physics-ui-implementation-20260908/projects/chirality-piping/apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Physics UI Post-G0 Witness 20260908 R2.app'
executable="$bundle/Contents/MacOS/openpipestress-desktop"
store_dir='/Users/ryan/Library/Application Support/org.openpipestress.technical-preview.walkthrough-physics-ui-post-g0-20260908-r2'
expected_store_dir='/Users/ryan/Library/Application Support/org.openpipestress.technical-preview.walkthrough-physics-ui-post-g0-20260908-r2'
allowed_prefix='/Users/ryan/Library/Application Support/org.openpipestress.technical-preview.walkthrough-physics-ui-post-g0-20260908-r2'
db_uri='file:///Users/ryan/Library/Application%20Support/org.openpipestress.technical-preview.walkthrough-physics-ui-post-g0-20260908-r2/openpipestress-projects.sqlite3?mode=ro&immutable=1'

process_matches=$(ps -axo pid=,comm= | awk -v executable="$executable" '{ line=$0; sub(/^[[:space:]]*[0-9]+[[:space:]]+/, "", line); if (line == executable) print $0 }')
if [[ -n "$process_matches" ]]; then
  print -r -- 'HOLD_PROCESS_MATCHES_PRESENT'
  print -r -- "$process_matches"
  exit 10
fi
print -r -- 'PRE_PROCESS_MATCHES_ABSENT'

if [[ "$store_dir" != "$expected_store_dir" || "$store_dir" != "$allowed_prefix" ]]; then
  print -r -- 'HOLD_LITERAL_OR_PREFIX_GUARD_FAILED'
  exit 11
fi
if [[ ! -d "$store_dir" ]]; then
  print -r -- 'HOLD_DEDICATED_STORE_DIR_ABSENT_BEFORE_CLEANUP'
  exit 12
fi

member_count=0
while IFS= read -r -d '' member; do
  member_count=$((member_count + 1))
  base=${member:t}
  if [[ "$member" != "$store_dir/$base" ]]; then
    print -r -- "HOLD_MEMBER_CONTAINMENT_FAILED $member"
    exit 13
  fi
  case "$base" in
    openpipestress-projects.sqlite3|openpipestress-projects.sqlite3-wal|openpipestress-projects.sqlite3-shm) ;;
    *) print -r -- "HOLD_UNEXPECTED_MEMBER $base"; exit 14 ;;
  esac
  print -r -- "PRE_STORE_MEMBER $base|$(stat -f %z "$member")|$(shasum -a 256 "$member" | awk '{print $1}')"
done < <(find "$store_dir" -mindepth 1 -maxdepth 1 -type f -print0 | sort -z)

if [[ "$member_count" -lt 1 || "$member_count" -gt 3 ]]; then
  print -r -- "HOLD_MEMBER_COUNT $member_count"
  exit 15
fi
print -r -- "PRE_STORE_MEMBER_COUNT $member_count"
print -r -- "PRE_SQLITE_INTEGRITY $(sqlite3 "$db_uri" 'pragma integrity_check;')"
print -r -- "PRE_LOCAL_PROJECT_ROWS $(sqlite3 "$db_uri" 'select count(*) from local_projects;')"

for base in openpipestress-projects.sqlite3-wal openpipestress-projects.sqlite3-shm openpipestress-projects.sqlite3; do
  member="$store_dir/$base"
  if [[ -e "$member" ]]; then
    rm -- "$member"
    print -r -- "REMOVED_EXACT_MEMBER $base"
  fi
done
rmdir -- "$store_dir"
print -r -- 'REMOVED_EXACT_DEDICATED_DIRECTORY'

post_process_matches=$(ps -axo pid=,comm= | awk -v executable="$executable" '{ line=$0; sub(/^[[:space:]]*[0-9]+[[:space:]]+/, "", line); if (line == executable) print $0 }')
if [[ -n "$post_process_matches" ]]; then
  print -r -- 'POST_PROCESS_MATCHES_PRESENT'
  exit 16
fi
print -r -- 'POST_PROCESS_MATCHES_ABSENT'
if [[ -e "$store_dir" ]]; then
  print -r -- 'POST_DEDICATED_STORE_PRESENT'
  exit 17
fi
print -r -- 'POST_DEDICATED_STORE_ABSENT'
print -r -- 'NORMAL_STORE_INSPECTED=false'
print -r -- 'NORMAL_STORE_ACCESSED=false'
print -r -- 'NORMAL_STORE_MODIFIED=false'
