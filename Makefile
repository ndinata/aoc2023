@PHONY: run test

# example: make run day=1 part=2
run:
	@cargo run --package day$(day) --bin part$(part)

# example: make test day=1
test:
	@cargo test --package day$(day)
