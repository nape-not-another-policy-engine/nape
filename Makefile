.PHONY: ma-release-build build-release install clean clean-build clean-test-data update test test-nape-cli test-domain docs-v2-smoke standards-check product-projection-check qualification-live
PWD := $(shell pwd)
BUILD_DIR := $(dir $(PWD))builds
PROJECT := ''
OS_ARCH_TARGET := ''
PYTHON ?= python3


ma-release-build:
	@echo "\n\033[1;96m Starting the Multi-Architecture Process \033[0m\n"
	make clean-build
	@echo "\n\033[1;96m Executing the Cargo Build process. \033[0m\n"
	cargo build --release --package $(PROJECT) --target $(OS_ARCH_TARGET)
	@echo "\n\033[1;96m Build COMPLETE! \033[0m\n"

build-release:
	@echo "\n\033[1;96m Starting the Release Build \033[0m\n"
	make clean-build
	mkdir -p $(BUILD_DIR)
	@echo "\n\033[1;96m Executing the Cargo Release Build process. \033[0m\n"
	cargo build --release --target-dir $(BUILD_DIR)
	@echo "\n\033[1;96m Release Build COMPLETE! \033[0m\n"

install:
	@echo "\n\033[1;96m Copying the NAPE binary to /usr/local/bin \033[0m\n"
	cp ../builds/release/nape /usr/local/bin
	@echo "\n\033[1;96m Copying COMPLETE! \033[0m\n"

clean:
	@echo "\n\033[1;96m Cleaning all target data \033[0m\n"
	cargo clean
	make clean-test-data

clean-build:
	@echo "\n\033[1;96m Removing existing files from the build directory. \033[0m\n"
	rm -rf $(BUILD_DIR)/*
	@echo "\n\033[1;96m All files removed from the build directory. \033[0m\n"

clean-test-data:
	@echo "\n\033[1;96m Deleting all previous testing outputs \033[0m\n"
	find . -type f -name "*.profraw" -exec rm -f {} +
	find . -type f -name "*.profdata" -exec rm -f {} +

update:
	@echo "\n\033[1;96m Updating Rust Dependencies \033[0m\n"
	cargo update

test:
	make clean-test-data
	@echo "\n\033[1;96m Running NAPE CLI 2.0 Workspace Tests \033[0m\n"
	cargo test --workspace --all-targets -- --test-threads=8

test-nape-cli:
	@echo "\n\033[1;96m Running Tests - NAPE CLI 2.0 Only \033[0m\n"
	cargo test -p nape_cli --all-targets -- --test-threads=8

test-domain:
	@echo "\n\033[1;96m Running Tests - Verification Domain Only \033[0m\n"
	cargo test -p nape_domain --lib -- --test-threads=8

docs-v2-smoke:
	@echo "\n\033[1;96m Running NAPE CLI 2.0 documentation smoke \033[0m\n"
	$(PYTHON) scripts/docs-v2-smoke.py

product-projection-check:
	$(PYTHON) contracts/attestify-nape-evaluator-action-invocation-v2/tools/verify_projection.py
	$(PYTHON) contracts/attestify-nape-evaluator-action-invocation-v3/verify_projection.py
	$(PYTHON) contracts/attestify-nape-verification-engine-contract-projection-v1/verify_projection.py
	$(PYTHON) contracts/attestify-nape-verification-engine-contract-projection-v2/verify_projection.py

standards-check:
	cargo fmt --all -- --check
	cargo check --workspace --all-targets
	cargo clippy --workspace --all-targets --all-features -- -D warnings
	RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items
	cargo test --workspace --all-targets -- --test-threads=8
	$(PYTHON) tools/engineering_standards_check.py
	$(PYTHON) scripts/docs-v2-smoke.py
	$(MAKE) product-projection-check

qualification-live:
	@test -n "$(REGISTRY_ENDPOINT)" || (echo "REGISTRY_ENDPOINT is required" >&2; exit 2)
	$(PYTHON) tools/verification_v2_lifecycle_smoke.py --registry-endpoint "$(REGISTRY_ENDPOINT)"
