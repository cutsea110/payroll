.PHONY: all
all:
	cargo build

.PHONY: clean
clean:
	cargo clean

.PHONY: run
run:
	cargo run -p payroll-cli

.PHONY: test
test:
	cargo test

.PHONY: scenario
scenario:
	cargo build --bin payroll-cli
	cargo run -p payroll-test -- $(shell find ./scenario -name 'test*.scr' | sort)
