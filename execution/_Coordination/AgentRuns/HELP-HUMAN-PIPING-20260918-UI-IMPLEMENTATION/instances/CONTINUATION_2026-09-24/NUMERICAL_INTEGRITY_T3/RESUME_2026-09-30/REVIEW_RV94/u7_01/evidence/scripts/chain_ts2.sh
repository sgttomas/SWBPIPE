#!/bin/bash
I=WT/scratch/rv94_u7_01/impl
while [ ! -f $I/impl_mutants_r5_u7.json ]; do sleep 15; done
cd $I && python3 mutants_r5.py $I/impl_mutants_r5_rest.json $(cat $I/r5_rest_ids.txt) > $I/logs/impl_mutants_r5_rest.log 2>&1
echo CHAIN_TS2_DONE
