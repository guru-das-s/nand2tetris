#!/bin/bash
# Test the `vmt` VM Translator
# 	Has to be executed from the repo's root folder.
set -e

cleanup() {
	find projects/7/ -not -path "projects/7/handcode/*" -iname *.asm -exec rm {} \;
}

trap cleanup EXIT

run_vmt() {
	local vm_file="$1"
	cargo run --release --bin vmt -- -f $vm_file
}

run_tst() {
	local vm_file="$1"
	local tst_file="${vm_file%.vm}.tst"
	./tools/CPUEmulator.sh $tst_file
}

find projects/7/ -iname *.vm | while read -r file; do
    # First, run the `vmt` VM Translator to produce asm output files
	run_vmt $file

	# Then, run the `tst` test script on the CPU Emulator
	run_tst $file
done
