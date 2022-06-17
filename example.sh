#!/bin/bash

start1=$(date +%s)
success=0
for((i=1;i<4000;i++));
do
    start=$(date +%s)
    a="(${i}:nat32)"
    dfx canister call example hello ${a}
    echo ${i} time; 
    end=$(date +%s)
    interval=$((end-start))
    if (($(echo "$interval > 1.5" |bc -l) )); then
     success=$((success+1))
    fi
done
end1=$(date +%s)
take=$(( end1 - start1 ))
echo Time taken to execute commands is ${take} seconds.
echo Successful times are ${success}