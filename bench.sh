#!/bin/bash

RUNS=100
APP_PATH="./target/release/note-app-rust"
DATA_FILE="timing_data.tmp"
export TIMEFORMAT="%R %U %S"

for i in $(seq 1 $RUNS); do
 (time $APP_PATH 1 add>/dev/null;)2>> "$DATA_FILE"
done

awk -v runs="$RUNS" '{
real_sum += $1;
user_sum += $2;
sys_sum += $3;
}
END {
printf("%8.8fs",real_sum)
printf "\n real sum |\n\
%8.8fs|\n \
",real_sum/runs
printf "\n user sum |\n\
%8.8fs|\n \
",user_sum/runs
printf "\n sys sum |\n\
%8.8fs|\n \
",sys_sum/runs
}' "$DATA_FILE"
