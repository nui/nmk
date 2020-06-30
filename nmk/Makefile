.PHONY: ci export-build-env invalidate-cache

NMKUP_PACKAGE := nmkup
NMK_PACKAGE := nmk

RELEASE_ENV := RUSTFLAGS="-C link-arg=-s"
RELEASE_BUILD := $(RELEASE_ENV) cargo build --release

AMD64_LINUX_MUSL_TARGET := x86_64-unknown-linux-musl
ARMV7_LINUX_TARGET := armv7-unknown-linux-gnueabihf
ARM64_LINUX_MUSL_TARGET := aarch64-unknown-linux-musl

.DEFAULT_GOAL := build

export CLOUDSDK_ACTIVE_CONFIG_NAME = nui

export-build-env:
	$(eval export GIT_SHORT_SHA = $(shell git rev-parse --short HEAD))

build: invalidate-cache
	cargo build

invalidate-cache:
	touch src/bin/nmk/main.rs
	touch src/bin/nmkup/main.rs
	touch src/nmk/lib.rs

release: invalidate-cache
	$(RELEASE_BUILD)

cross-amd64-linux-musl: export-build-env invalidate-cache
	$(RELEASE_ENV) cross build --release --target $(AMD64_LINUX_MUSL_TARGET)

cross-armv7-linux: export-build-env invalidate-cache
	$(RELEASE_ENV) cross build --release --target $(ARMV7_LINUX_TARGET)

cross-arm64-linux-musl: export-build-env invalidate-cache
	RUSTFLAGS="-C link-arg=-s -C link-arg=-lgcc -C target-feature=+crt-static" cross build --release --target $(ARM64_LINUX_MUSL_TARGET)

ci: invalidate-cache cross-amd64-linux-musl cross-armv7-linux cross-arm64-linux-musl
	$(eval UPLOAD_TMP_DIR = $(shell mktemp -d))
	<target/$(AMD64_LINUX_MUSL_TARGET)/release/$(NMKUP_PACKAGE)		xz >	$(UPLOAD_TMP_DIR)/nmkup-amd64-linux-musl.xz
	<target/$(AMD64_LINUX_MUSL_TARGET)/release/$(NMK_PACKAGE)		xz >	$(UPLOAD_TMP_DIR)/nmk-amd64-linux-musl.xz
	<target/$(ARM64_LINUX_MUSL_TARGET)/release/$(NMKUP_PACKAGE)		xz >	$(UPLOAD_TMP_DIR)/nmkup-arm64-linux-musl.xz
	<target/$(ARM64_LINUX_MUSL_TARGET)/release/$(NMK_PACKAGE)		xz >	$(UPLOAD_TMP_DIR)/nmk-arm64-linux-musl.xz
	<target/$(ARMV7_LINUX_TARGET)/release/$(NMKUP_PACKAGE)			xz >	$(UPLOAD_TMP_DIR)/nmkup-armv7-linux.xz
	<target/$(ARMV7_LINUX_TARGET)/release/$(NMK_PACKAGE)			xz >	$(UPLOAD_TMP_DIR)/nmk-armv7-linux.xz
	cp target/$(AMD64_LINUX_MUSL_TARGET)/release/$(NMKUP_PACKAGE)			$(UPLOAD_TMP_DIR)/nmkup-amd64-linux-musl
	cp target/$(ARM64_LINUX_MUSL_TARGET)/release/$(NMKUP_PACKAGE)			$(UPLOAD_TMP_DIR)/nmkup-arm64-linux-musl
	cp target/$(ARMV7_LINUX_TARGET)/release/$(NMKUP_PACKAGE)				$(UPLOAD_TMP_DIR)/nmkup-armv7-linux
	gsutil -m -h "Cache-Control: no-store, no-transform" cp $(UPLOAD_TMP_DIR)/* gs://nmk.nuimk.com/
	rm -rf $(UPLOAD_TMP_DIR)

