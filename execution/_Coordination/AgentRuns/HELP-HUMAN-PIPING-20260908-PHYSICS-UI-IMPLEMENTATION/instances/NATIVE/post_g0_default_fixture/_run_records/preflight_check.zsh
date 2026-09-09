#!/bin/zsh
set -euo pipefail

repo_root=$(git rev-parse --show-toplevel)
working_root="$repo_root/projects/chirality-piping"
binding="$working_root/execution/_Change/PHYSICS_UI_IMPLEMENTATION_20260908/NATIVE_SOURCE_BINDING_FINAL_V2.json"
dist_inv="$working_root/execution/_Change/PHYSICS_UI_IMPLEMENTATION_20260908/_run_records/NATIVE_DIST_INVENTORY_V2.json"
bundle_inv="$working_root/execution/_Change/PHYSICS_UI_IMPLEMENTATION_20260908/_run_records/NATIVE_BUNDLE_INVENTORY_V2.json"
bundle="$working_root/apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Physics UI Post-G0 Witness 20260908 R2.app"
store_dir="/Users/ryan/Library/Application Support/org.openpipestress.technical-preview.walkthrough-physics-ui-post-g0-20260908-r2"
store="$store_dir/openpipestress-projects.sqlite3"

source_mismatch=0
source_count=0
source_stream=""
while IFS=$'\t' read -r relpath expected_bytes expected_sha classes; do
  source_count=$((source_count + 1))
  actual_bytes=$(stat -f %z "$repo_root/$relpath")
  actual_sha=$(shasum -a 256 "$repo_root/$relpath" | awk '{print $1}')
  if [[ "$actual_bytes" != "$expected_bytes" || "$actual_sha" != "$expected_sha" ]]; then
    print -r -- "SOURCE_MISMATCH path=$relpath expected_bytes=$expected_bytes actual_bytes=$actual_bytes expected_sha=$expected_sha actual_sha=$actual_sha"
    source_mismatch=$((source_mismatch + 1))
  fi
  source_stream+="$actual_sha  $classes  $relpath"$'\n'
done < <(jq -r '.binding.entries | sort_by(.path)[] | [.path, (.bytes|tostring), .sha256, (.classifications|join(","))] | @tsv' "$binding")
source_aggregate=$(print -rn -- "$source_stream" | shasum -a 256 | awk '{print $1}')
print -r -- "SOURCE count=$source_count mismatches=$source_mismatch aggregate=$source_aggregate"

compute_inventory() {
  local root="$1"
  local prefix="$2"
  local stream=""
  local count=0
  while IFS= read -r -d '' file; do
    local rel="${file#$prefix/}"
    local bytes=$(stat -f %z "$file")
    local sha=$(shasum -a 256 "$file" | awk '{print $1}')
    stream+="$sha  file  $bytes  $rel"$'\n'
    count=$((count + 1))
  done < <(find "$root" -type f -print0 | sort -z)
  local digest=$(print -rn -- "$stream" | shasum -a 256 | awk '{print $1}')
  print -r -- "$count $digest"
}

dist_result=$(compute_inventory "$working_root/apps/desktop/dist" "$working_root/apps/desktop/dist")
bundle_result=$(compute_inventory "$bundle" "$bundle")
print -r -- "DIST count=${dist_result%% *} digest=${dist_result#* }"
print -r -- "BUNDLE count=${bundle_result%% *} digest=${bundle_result#* }"
print -r -- "BINDING_SHA $(shasum -a 256 "$binding" | awk '{print $1}')"
print -r -- "BUILD_RESULT_SHA $(shasum -a 256 "$working_root/execution/_Change/PHYSICS_UI_IMPLEMENTATION_20260908/NATIVE_BUILD_RESULT_V2.json" | awk '{print $1}')"
print -r -- "DIST_INV_SHA $(shasum -a 256 "$dist_inv" | awk '{print $1}')"
print -r -- "BUNDLE_INV_SHA $(shasum -a 256 "$bundle_inv" | awk '{print $1}')"
print -r -- "INFO CFBundleIdentifier=$(plutil -extract CFBundleIdentifier raw -o - "$bundle/Contents/Info.plist")"
print -r -- "INFO CFBundleName=$(plutil -extract CFBundleName raw -o - "$bundle/Contents/Info.plist")"
print -r -- "INFO CFBundleExecutable=$(plutil -extract CFBundleExecutable raw -o - "$bundle/Contents/Info.plist")"
process_matches=$(ps -axo pid=,comm= | awk -v executable="$bundle/Contents/MacOS/openpipestress-desktop" '{ line=$0; sub(/^[[:space:]]*[0-9]+[[:space:]]+/, "", line); if (line == executable) print $0 }')
if [[ -n "$process_matches" ]]; then
  print -r -- "PROCESS_MATCHES_PRESENT"
  print -r -- "$process_matches"
else
  print -r -- "PROCESS_MATCHES_ABSENT"
fi
if [[ -e "$store_dir" || -e "$store" ]]; then
  print -r -- "DEDICATED_STORE_PRESENT"
else
  print -r -- "DEDICATED_STORE_ABSENT"
fi
print -r -- "NORMAL_STORE_INSPECTED=false"
print -r -- "NORMAL_STORE_ACCESSED=false"
