#!/bin/bash
# I92 suites: the whole vitest desktop suite and tsc --noEmit, each under the shared T3 lock.
# Usage: suites.sh <label> <P root> [ts|tsc ...]
set -u
WT=WT
S=$WT/scratch/i92_b1_sr_ts
LOCK=$WT/guard/cargo_job.lock
LABEL=$1; PROOT=$2; shift 2
mkdir -p $S/logs/$LABEL $S/tmp/$LABEL
for what in "$@"; do
  case $what in
    ts)
      echo "ts wait $(date -u +%FT%TZ)" >> $S/logs/$LABEL/rc.txt
      cd "$PROOT/apps/desktop" && /usr/bin/lockf -k $LOCK /bin/bash -c "echo \"ts start \$(date -u +%FT%TZ)\" >> $S/logs/$LABEL/rc.txt; env TMPDIR=$S/tmp/$LABEL ../../node_modules/.bin/vitest run --reporter=dot --reporter=json --outputFile.json=$S/logs/$LABEL/vitest.json > $S/logs/$LABEL/vitest.log 2>&1; echo \"ts rc=\$? end \$(date -u +%FT%TZ)\" >> $S/logs/$LABEL/rc.txt"
      rm -rf "${PROOT:?}/apps/desktop/node_modules/.vite"; rmdir "${PROOT:?}/apps/desktop/node_modules/.vite-temp" "${PROOT:?}/apps/desktop/node_modules" 2>/dev/null ;;
    tsc)
      echo "tsc wait $(date -u +%FT%TZ)" >> $S/logs/$LABEL/rc.txt
      cd "$PROOT/apps/desktop" && /usr/bin/lockf -k $LOCK /bin/bash -c "echo \"tsc start \$(date -u +%FT%TZ)\" >> $S/logs/$LABEL/rc.txt; env TMPDIR=$S/tmp/$LABEL ../../node_modules/.bin/tsc --noEmit -p tsconfig.json > $S/logs/$LABEL/tsc.log 2>&1; echo \"tsc rc=\$? end \$(date -u +%FT%TZ)\" >> $S/logs/$LABEL/rc.txt" ;;
  esac
done
echo done >> $S/logs/$LABEL/rc.txt
