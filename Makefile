.PHONY: help setup build install test test-all clean

help: ## Show available targets
	@echo "Usage:"
	@echo "  make setup      Run setup script (clone deps, build)"
	@echo "  make build      Build in release mode"
	@echo "  make install    Install to ~/.cargo/bin"
	@echo "  make test       Run fast tests"
	@echo "  make test-all   Run all tests including slow ones"
	@echo "  make clean      Remove build artifacts"
	@echo "  make help       Show this help message"

setup: ## Run setup script
	./setup.sh

build: ## Build in release mode
	cargo build --release

install: ## Install to ~/.cargo/bin
	cargo install --path .

test: ## Run fast tests
	cargo test

test-all: ## Run all tests including slow exercise verification
	cargo test -- --include-ignored

clean: ## Remove build artifacts and sandbox cache
	cargo clean
	rm -rf .rust-game-cache
