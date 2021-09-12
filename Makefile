.PHONY: clean
clean:
	rm -rf target

.PHONY: dev
dev: format lint test doc
	@cargo build --quiet

.PHONY: d
d:
	@watchexec -c 'make dev'

.PHONY: format
format:
	@rustup run nightly cargo fmt --quiet

.PHONY: lint
lint:
	@cargo clippy --all-targets --quiet

.PHONY: test
test:
	@cargo test --all-targets --quiet

.PHONY: doc
doc:
	@cargo doc --quiet
