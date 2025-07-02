# ROTD CLI Makefile
# Configuration files are in config/ directory

.PHONY: help build test check install clean release dev

# Include config from config directory
CONFIG_DIR = config

# Default target
help: ## Show this help message
	@echo "ROTD CLI Build System"
	@echo "====================="
	@echo ""
	@echo "Available targets:"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## Build the CLI in debug mode
	cargo build

release: ## Build optimized release binary
	cargo build --release

test: ## Run all tests
	cargo test

check: ## Run code quality checks (fmt, clippy, test)
	cargo fmt -- --check
	cargo clippy -- -D warnings
	cargo test

install: ## Install CLI locally
	cargo install --path .

clean: ## Clean build artifacts
	cargo clean
	rm -rf target/

dev: ## Start development with auto-rebuild
	cargo watch -x build

test-watch: ## Run tests with auto-rebuild
	cargo watch -x test

docs: ## Generate and open documentation
	cargo doc --open

validate-schemas: ## Validate JSON schemas with examples
	@echo "Installing ajv-cli..."
	@npm install -g ajv-cli 2>/dev/null || echo "ajv-cli already installed"
	@echo "Validating schemas..."
	ajv validate -s schema/task.schema.json -d examples/tasks.jsonl --spec=draft7
	ajv validate -s schema/pss_score.schema.json -d examples/pss_score.jsonl --spec=draft7
	@echo "✓ All schemas valid"

test-cli: build ## Test CLI functionality
	@echo "Testing CLI functionality..."
	@mkdir -p /tmp/rotd-test
	@cd /tmp/rotd-test && $(PWD)/target/debug/rotd init --force
	@cd /tmp/rotd-test && $(PWD)/target/debug/rotd check
	@cd /tmp/rotd-test && $(PWD)/target/debug/rotd agent info
	@rm -rf /tmp/rotd-test
	@echo "✓ CLI tests passed"

all: check build test validate-schemas test-cli ## Run all checks and builds

# Development helpers
fmt: ## Format code
	cargo fmt

clippy: ## Run clippy lints
	cargo clippy

audit: ## Run security audit
	cargo audit

# Installation
install-deps: ## Install development dependencies
	@echo "Installing Rust development tools..."
	cargo install cargo-watch cargo-audit
	@echo "Installing Node.js tools for schema validation..."
	npm install -g ajv-cli

# Quick commands
quick-test: ## Quick test without full checks
	cargo test --lib

quick-build: ## Quick build without optimizations
	cargo build

integration-test: build ## Run integration tests only
	cargo test integration

# Distribution
dist: release ## Create distribution package
	@mkdir -p dist
	@cp target/release/rotd dist/
	@cp README.md LICENSE dist/
	@tar -czf dist/rotd-$(shell uname -s)-$(shell uname -m).tar.gz -C dist rotd README.md LICENSE
	@echo "✓ Distribution package created in dist/"