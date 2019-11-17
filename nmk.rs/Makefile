BIN_NAME := nmk
DIST_DIR ?= dist

RELEASE_ENV := RUSTFLAGS="-C link-arg=-s"
RELEASE_BUILD := $(RELEASE_ENV) cargo build --release

AMD64_LINUX_MUSL_TARGET := x86_64-unknown-linux-musl
AMD64_LINUX_MUSL_BIN := target/$(AMD64_LINUX_MUSL_TARGET)/release/$(BIN_NAME)
AMD64_LINUX_MUSL_OUT := $(DIST_DIR)/nmk.rs-amd64-linux-musl

ARMV7_LINUX_TARGET := armv7-unknown-linux-gnueabihf
ARMV7_LINUX_BIN := target/$(ARMV7_LINUX_TARGET)/release/$(BIN_NAME)
ARMV7_LINUX_OUT := $(DIST_DIR)/nmk.rs-armv7-linux

build:
	cargo build

arm-cross:
	cross build --target $(ARMV7_LINUX_TARGET) --release

release:
	$(RELEASE_BUILD)
	du target/release/$(BIN_NAME)

test:
	$(RELEASE_ENV) cargo test --release

clean:
	cargo clean

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
