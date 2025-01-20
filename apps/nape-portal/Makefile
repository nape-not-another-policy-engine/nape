.PHONY: build-release clean fmt serve update-deps  upgrade-deps

build-release:
	trunk build --release

clean:
	trunk clean
	cago cleam

fmt:
	leptosfmt ./**/*.rs
	cargo clippy

serve:
	trunk serve

update-deps:
	cargo update --verbose

upgrade-deps:
	cargo upgrade --verbose