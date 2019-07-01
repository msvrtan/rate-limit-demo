SHELL := /bin/bash # Use bash syntax

.PHONY=*

check:
	cd proxy && time cargo check
	cd slow-target && time cargo check

build:
	cd proxy && time cargo build
	cd slow-target && time cargo build

fmt:
	cd proxy && time cargo fmt
	cd slow-target && time cargo fmt

run-proxy:
	cd proxy && time cargo run

run-slow-target:
	cd slow-target && time cargo run
