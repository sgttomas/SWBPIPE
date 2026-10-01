# Executed preservation operations

Paths below use README.md's portable aliases. Shell operations ran with
`set -euo pipefail` under /bin/bash. Read-only discovery used cat, sed, head, rg,
find, stat, wc, jq, shasum and Git's rev-parse/show/status/check-ignore. The
coordination HEAD was confirmed as 3446bdf51d90b8f1812d161cc638d85c97bbef16;
COMMON.md and EVIDENCE.md were also read with git show at that identity.
No Git or index mutation was performed.

For each six selected sweep directories, sources were suites/*, meta.txt,
SWEEP*.json, suites.log and suites_vs_baseline.txt. Each source was copied with
`cp -p` to <LOCAL>/raw/sweeps/<source-relative-path>, then checked with
`cmp -s`, `stat -f '%z'`, and `shasum -a 256`. The originals were hashed again
in the final copy verification. Values were written to ORIGINALS.tsv.

The following filter was executed for each sanitized copy; AUDIT_WT, AUDIT_VENV
and AUDIT_HOME held the supplied historical host prefixes. The same filter was
used on the gate JSONL input. Re-running it for every suite source and comparing
its stdout with the corresponding sanitized copy succeeded.

```sh
perl -pe 's/\Q$ENV{AUDIT_VENV}\E/<VENV>/g; s/\Q$ENV{AUDIT_WT}\E/<wt>/g; s/\Q$ENV{AUDIT_HOME}\E/<home>/g; s{/(?:private/)?var/folders/[^/]+/[^/]+/T/}{<tmp>/}g; s{/(?:private/)?tmp/}{<tmp>/}g; s/\e\][^\a\e]*(?:\a|\e\\)//g; s/\e\[[0-?]*[ -\/]*[@-~]//g' <SOURCE> > <SANITIZED>
```

For SWEEP identity checks, jq read .git.commit_hash and .git.working_tree_dirty.
The hash of each raw SWEEP was compared with its previously committed
sweep_json_original_sha256.txt. Original suites.log was byte-compared with the
corresponding committed merge-folder suites.log. Every numbered log was matched
to manifests.txt's zero-based ordinal and slash-to-underscore manifest path.
For each log, recorded totals were compared with:

```sh
awk '/^test result:/{p+=$4; f+=$6; i+=$8} END{printf "passed=%d failed=%d ignored=%d",p,f,i}' <LOG>
```

The `^test .* \.\.\. FAILED` lines were retained separately as failure identities.
Earlier-attempt SWEEP identities were inventoried without selecting their logs.
Neither operation reran a test.

For each gate side:

```sh
gzip -9 -n -c <SOURCE_JSONL> > <LOCAL_RAW_GZIP>
gzip -t <LOCAL_RAW_GZIP>
gzip -dc <LOCAL_RAW_GZIP> | cmp -s <SOURCE_JSONL> -
# The filter above streamed SOURCE_JSONL into gzip -9 -n -c for LOCAL_SANITIZED_GZIP.
gzip -t <LOCAL_SANITIZED_GZIP>
cmp -s <LOCAL_RAW_GZIP> <LOCAL_SANITIZED_GZIP>
gzip -dc <LOCAL_SANITIZED_GZIP> | cmp -s <SOURCE_JSONL> -
jq -e 'type == "object"' <SOURCE_JSONL> > /dev/null
wc -l <SOURCE_JSONL>
shasum -a 256 <SOURCE_JSONL> <LOCAL_RAW_GZIP> <LOCAL_SANITIZED_GZIP>
gzip -dc <LOCAL_SANITIZED_GZIP> | shasum -a 256
gzip -dc <LOCAL_SANITIZED_GZIP> | wc -c
cp -p <LOCAL_SANITIZED_GZIP> <PACKET>/gate/part1_SIDE.runs.jsonl.gz
cmp -s <LOCAL_SANITIZED_GZIP> <PACKET>/gate/part1_SIDE.runs.jsonl.gz
gzip -t <PACKET>/gate/part1_SIDE.runs.jsonl.gz
gzip -dc <PACKET>/gate/part1_SIDE.runs.jsonl.gz | cmp -s <SOURCE_JSONL> -
```

Original gate hashes were compared with the exact side's pre-existing KF2
uncommitted_sha256.txt entry. Both record counts equaled 884. Sanitized suite and
provenance text was searched for /Users/, /home/, /var/folders/,
/private/var/folders/, /tmp/, and literal ESC. Gate source bytes additionally
were searched for JSON-escaped \u001b and \u009b (either case for B).
No match remained. Since raw and sanitized gate bytes compare equal, the source
check also applies to the decompressed packet archives.

Environment/version observations are in checks/ENVIRONMENT.txt and
checks/GZIP_VERSION.txt. The byte-size policy was inspected in
tools/validation/validate_run_record_leaks.py and its CI invocation; this TASK did
not run the committed-diff leak check or CI because it made no commit.

After completing the checks, SHA256SUMS was generated over every packet file
except itself with sorted packet-relative paths. `shasum -a 256 -c SHA256SUMS`
verified the seal. The complete packet was copied to <LOCAL>/packet and compared
with `diff -qr`. Local SHA256SUMS covers raw/, sanitized/, and packet/; its check
output lives separately under <LOCAL>/checks.
