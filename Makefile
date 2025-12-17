.PHONY: help install build build-release test test-cov lint fmt bench clean pre-commit-install pre-commit-run

help:  ## Show this help message
	@echo "polarcap development commands:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

install:  ## Install dependencies and set up development environment
	uv sync --group dev
	$(MAKE) pre-commit-install

build:  ## Build the Rust extension for development
	uv run maturin develop

build-release:  ## Build optimized release binary/wheel
	uv run maturin build --release

pre-commit-install:  ## Install pre-commit hooks
	uv run pre-commit install

pre-commit-run:  ## Run all pre-commit hooks on all files
	uv run pre-commit run --all-files

test:  ## Run Python tests
	uv run pytest

test-cov:  ## Run tests with coverage
	uv run pytest --cov=polarcap --cov-report=html --cov-report=term

lint:  ## Run linters (cargo clippy and ruff)
	cargo clippy -- -D warnings
	uv run ruff check python/

fmt:  ## Format code (cargo fmt and ruff)
	cargo fmt
	uv run ruff format python/

bench:  ## Run Rust benchmarks
	cargo bench

clean:  ## Clean build artifacts
	cargo clean
	rm -rf target/
	rm -rf .pytest_cache/
	rm -rf htmlcov/
	find . -name "*.pyc" -delete
	find . -name "__pycache__" -delete
