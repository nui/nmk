BIN_NAME := nmk
RELEASE_BIN := release/$(BIN_NAME)
DIST_DIR ?= dist
EPOCHSECONDS := $(shell date +%s)

DEBUG_BUILD_ENV := EPOCHSECONDS=$(EPOCHSECONDS)
RELEASE_BUILD_ENV := EPOCHSECONDS=$(EPOCHSECONDS) RUSTFLAGS="-C link-arg=-s"

DEBUG_BUILD := $(DEBUG_BUILD_ENV) cargo build
RELEASE_BUILD := $(RELEASE_BUILD_ENV) cargo build --release

AMD64_LINUX_MUSL_TARGET := x86_64-unknown-linux-musl
AMD64_LINUX_MUSL_BIN := target/$(AMD64_LINUX_MUSL_TARGET)/$(RELEASE_BIN)
AMD64_LINUX_MUSL_OUT := $(DIST_DIR)/nmk.rs-amd64-linux-musl

ARMV7_LINUX_TARGET := armv7-unknown-linux-gnueabihf
ARMV7_LINUX_BIN := target/$(ARMV7_LINUX_TARGET)/$(RELEASE_BIN)
ARMV7_LINUX_OUT := $(DIST_DIR)/nmk.rs-armv7-linux

build:
	$(DEBUG_BUILD)

arm-cross:
	$(DEBUG_BUILD_ENV) cross build --target $(ARMV7_LINUX_TARGET) --release

release:
	$(RELEASE_BUILD)

clean:
	cargo clean

test:
	cargo test

build-info:
	rustup show

amd64-linux-musl:
	$(RELEASE_BUILD) --target $(AMD64_LINUX_MUSL_TARGET)
	cp $(AMD64_LINUX_MUSL_BIN) $(AMD64_LINUX_MUSL_OUT)
	gzip -k $(AMD64_LINUX_MUSL_OUT)

armv7-linux:
	$(RELEASE_BUILD) --target $(ARMV7_LINUX_TARGET)
	cp $(ARMV7_LINUX_BIN) $(ARMV7_LINUX_OUT)
	gzip -k $(ARMV7_LINUX_OUT)

