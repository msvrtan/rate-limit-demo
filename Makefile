SHELL := /bin/bash # Use bash syntax

.PHONY=*

check:
	cd slow-target && time cargo check

build:
	cd slow-target && time cargo build

fmt:
	cd slow-target && time cargo fmt

run-slow-target:
	cd slow-target && time cargo run
