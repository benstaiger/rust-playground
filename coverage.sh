#!/bin/bash

RUSTFLAGS="-C instrument-coverage" LLVM_PROFILE_FILE="default.profraw" cargo run

llvm-profdata merge -sparse default.profraw -o default.profdata

# llvm-cov report \
#    --use-color --ignore-filename-regex='/.cargo/registry' \
#    --instr-profile=default.profdata \
llvm-cov show \
    target/debug/rust-playground \
    --Xdemangler=rustfilt \
    --use-color --ignore-filename-regex='/.cargo/registry' \
    --instr-profile=default.profdata \
    --show-instantiations --show-line-counts-or-regions \
    --Xdemangler=rustfilt | less -R
