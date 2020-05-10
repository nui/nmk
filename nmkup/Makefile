.PHONY: dist nmkup export-build-env invalidate-cache

NMKUP_PACKAGE := nmkup
DIST_DIR ?= dist

RELEASE_ENV := RUSTFLAGS="-C link-arg=-s"
RELEASE_BUILD := $(RELEASE_ENV) cargo build --release

AMD64_LINUX_MUSL_TARGET := x86_64-unknown-linux-musl
ARMV7_LINUX_TARGET := armv7-unknown-linux-gnueabihf

.DEFAULT_GOAL := build

export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER = arm-linux-gnueabihf-gcc

export-build-env:
	$(eval export GIT_SHORT_SHA = $(shell git rev-parse --short HEAD))

build: invalidate-cache
	cargo build

invalidate-cache:
	touch src/main.rs


armv7-linux: invalidate-cache
	$(RELEASE_BUILD) --target $(ARMV7_LINUX_TARGET)

release: invalidate-cache
	$(RELEASE_BUILD)
	du target/release/$(NMKUP_PACKAGE)

test:
	cargo test

clean:
	cargo clean

show-build-info:
	rustup show

amd64-linux-musl: invalidate-cache
	$(RELEASE_BUILD) --target $(AMD64_LINUX_MUSL_TARGET)

cross-amd64-linux-musl: invalidate-cache
	$(RELEASE_ENV) cross build --release --target $(AMD64_LINUX_MUSL_TARGET)

cross-armv7-linux: export-build-env invalidate-cache
	$(RELEASE_ENV) cross build --release --target $(ARMV7_LINUX_TARGET)

dist:
	<target/$(AMD64_LINUX_MUSL_TARGET)/release/$(NMKUP_PACKAGE)   gzip > $(DIST_DIR)/nmk-amd64-linux-musl.gz
	cp target/$(AMD64_LINUX_MUSL_TARGET)/release/$(NMKUP_PACKAGE) $(DIST_DIR)/nmkup-amd64-linux-musl
	<target/$(ARMV7_LINUX_TARGET)/release/$(NMKUP_PACKAGE)   gzip > $(DIST_DIR)/nmk-armv7-linux.gz
	cp target/$(ARMV7_LINUX_TARGET)/release/$(NMKUP_PACKAGE) $(DIST_DIR)/nmkup-armv7-linux

