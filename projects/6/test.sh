#!/bin/bash
# Test the `has` assembler
# 	Has to be executed from the repo's root folder.
set -e

cleanup() {
	find projects/6/ -iname *.hack -exec rm {} \;
}

trap cleanup EXIT

run_has() {
	local asm_file="$1"
	local has_hack_file="${asm_file%.asm}.has.hack"
	cargo run --release --bin has -- -f $asm_file -o $has_hack_file
}

diff_output() {
	local asm_file="$1"
	local has_hack_file="${asm_file%.asm}.has.hack"
	local ground_truth_file="${asm_file%.asm}.hack"

	diff -q $has_hack_file $ground_truth_file
}

# First, generate ground truth files from provided Assembler.sh
find projects/6/ -iname *.asm -exec ./tools/Assembler.sh {} \;

find projects/6/ -iname *.asm | while read -r file; do
	# Run `has` on the same asm files as above
	run_has $file

	# Compare with ground truth files
	diff_output $file
done
