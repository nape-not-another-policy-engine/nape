.PHONY: test build-release clean clean-build clean-test-data update test-domain test-io test-kernel test-tfw install
PWD := $(shell pwd)
BUILD_DIR := $(dir $(PWD))builds

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
	@echo "\n\033[1;96m Running Tests \033[0m\n"
	RUSTFLAGS="-A dead_code -A unused_imports -C instrument-coverage" cargo test  -- --test-threads=8

test-nape-cli:
	@echo "\n\033[1;96m Running Tests - NAPE Collection CLI Only \033[0m\n"
	RUSTFLAGS="-A dead_code -A unused_imports" cargo test -p nape_collection_cli --lib -- --test-threads=8

test-domain:
	@echo "\n\033[1;96m Running Tests - Domain Package  Only \033[0m\n"
	RUSTFLAGS="-A dead_code -A unused_imports" cargo test -p nape_domain --lib -- --test-threads=8

test-kernel:
	@echo "\n\033[1;96m Running Tests - Kernel Package Only \033[0m\n"
	RUSTFLAGS="-A dead_code -A unused_imports" cargo test -p nape_kernel --lib  -- --test-threads=8
