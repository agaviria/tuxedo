#!/bin/bash
watchexec -w src -w app/elm -w app/static -w app/scss --restart "cargo script setup && cargo build && RUST_BACKTRACE=1 cargo run"
