.PHONY: dist

NMK_PACKAGE := nmk
NMKUP_PACKAGE := nmkup
DIST_DIR ?= dist

RELEASE_ENV := RUSTFLAGS="-C link-arg=-s"
RELEASE_BUILD := $(RELEASE_ENV) cargo build --release

AMD64_LINUX_MUSL_TARGET := x86_64-unknown-linux-musl
ARMV7_LINUX_TARGET := armv7-unknown-linux-gnueabihf

CLEAN_ALL_PACKAGES := cargo clean --package $(NMK_PACKAGE) --package $(NMKUP_PACKAGE) --package common

export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER = arm-linux-gnueabihf-gcc

build:
	$(CLEAN_ALL_PACKAGES)
	cargo build

arm-cross:
	cross build --target $(ARMV7_LINUX_TARGET) --release

release:
	$(CLEAN_ALL_PACKAGES) --release
	$(RELEASE_BUILD)
	du target/release/$(NMK_PACKAGE)

test:
	$(RELEASE_ENV) cargo test --release

clean:
	cargo clean

show-build-info:
	rustup show

amd64-linux-musl:
	$(CLEAN_ALL_PACKAGES) --release --target $(AMD64_LINUX_MUSL_TARGET)
	$(RELEASE_BUILD) --target $(AMD64_LINUX_MUSL_TARGET)

armv7-linux:
	$(CLEAN_ALL_PACKAGES) --release --target $(ARMV7_LINUX_TARGET)
	$(RELEASE_BUILD) --target $(ARMV7_LINUX_TARGET)

dist:
	<target/$(AMD64_LINUX_MUSL_TARGET)/release/$(NMK_PACKAGE)   gzip > $(DIST_DIR)/nmk-amd64-linux-musl.gz
	<target/$(AMD64_LINUX_MUSL_TARGET)/release/$(NMKUP_PACKAGE) gzip > $(DIST_DIR)/nmkup-amd64-linux-musl.gz
	<target/$(ARMV7_LINUX_TARGET)/release/$(NMK_PACKAGE)   gzip > $(DIST_DIR)/nmk-armv7-linux.gz
	<target/$(ARMV7_LINUX_TARGET)/release/$(NMKUP_PACKAGE) gzip > $(DIST_DIR)/nmkup-armv7-linux.gz
