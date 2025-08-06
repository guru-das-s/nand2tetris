#!/bin/bash
# Test the `vmt` VM Translator
# 	Has to be executed from the repo's root folder.
set -e

cleanup() {
	find projects/8/ -iname *.asm -exec rm {} \;
}

trap cleanup EXIT

run_vmt() {
	local vm_file="$1"
	cargo run --release --bin vmt -- -i $vm_file
}

run_tst() {
	local vm_file="$1"
	local tst_file

	if [ -d $vm_file ]; then
        tst_file=$vm_file/$(basename $vm_file).tst
    else
        tst_file="${vm_file%.vm}.tst"
    fi

	./tools/CPUEmulator.sh $tst_file
}

PRJ_8_PATHS=(
    "projects/8/ProgramFlow/BasicLoop/BasicLoop.vm"
    "projects/8/ProgramFlow/FibonacciSeries/FibonacciSeries.vm"
    "projects/8/FunctionCalls/SimpleFunction/SimpleFunction.vm"
    "projects/8/FunctionCalls/NestedCall/"
    "projects/8/FunctionCalls/FibonacciElement/"
    "projects/8/FunctionCalls/StaticsTest/"
)

for file in "${PRJ_8_PATHS[@]}"; do
    # First, run the `vmt` VM Translator to produce asm output files
	run_vmt $file

	# Then, run the `tst` test script on the CPU Emulator
	run_tst $file
done
