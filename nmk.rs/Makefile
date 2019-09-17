BIN_NAME := nmk
RELEASE_BIN := release/$(BIN_NAME)
DIST_DIR ?= dist
EPOCHSECONDS := $(shell date +%s)
BUILD_ENV := EPOCHSECONDS=$(EPOCHSECONDS)

BUILD := env $(BUILD_ENV) cargo build

ALPINE_TARGET := x86_64-unknown-linux-musl
ALPINE_BIN := target/$(ALPINE_TARGET)/$(RELEASE_BIN)
ALPINE_OUT := $(DIST_DIR)/nmk.rs-amd64-linux-musl

ARMV7_TARGET := armv7-unknown-linux-gnueabihf
ARMV7_BIN := target/$(ARMV7_TARGET)/$(RELEASE_BIN)
ARMV7_OUT := $(DIST_DIR)/nmk.rs-armv7-linux

all: build

arm-cross:
	$(BUILD_ENV) cross build --target $(ARMV7_TARGET) --release

build:
	$(BUILD)

release:
	$(BUILD) --release

clean:
	cargo clean

amd64-alpine:
	$(BUILD) --release --target $(ALPINE_TARGET)
	cp $(ALPINE_BIN) $(ALPINE_OUT)
	strip $(ALPINE_OUT)
	gzip -k $(ALPINE_OUT)

armv7:
	. /root/.cargo/env && $(BUILD) --release --target $(ARMV7_TARGET)
	cp $(ARMV7_BIN) $(ARMV7_OUT)
	gzip -k $(ARMV7_OUT)

