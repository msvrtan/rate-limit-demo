SHELL := /bin/bash # Use bash syntax

.PHONY=*

run-slow-target:
	cd slow-target && time cargo run
