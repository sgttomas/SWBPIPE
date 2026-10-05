#!/bin/zsh
# Usage: run_dump9.sh <lane> <out_json> <inputs_json> <cases_json>
S=WT/scratch/i67_u6d
LANE=$1; OUT=$2
D=$S/lanes/$LANE/projects/chirality-piping/apps/desktop
cp $S/oracle7/zzI67U7fOracle.test.ts $D/src/features/results/
cd $D
TMPDIR=$S/tmp I67_ORACLE_INPUTS=$3 I67_ORACLE_CASES=$4 I67_ORACLE_OUT=$OUT ../../node_modules/.bin/vitest run src/features/results/zzI67U7fOracle.test.ts > $OUT.log 2>&1
echo "exit=$?" >> $OUT.log
rm $D/src/features/results/zzI67U7fOracle.test.ts
