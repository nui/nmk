.PHONY: docker
.ONESHELL: clean

export CLOUDSDK_ACTIVE_CONFIG_NAME = nui

define remove_releases =
for target in aarch64-unknown-linux-musl arm-unknown-linux-musleabi armv7-unknown-linux-musleabihf x86_64-unknown-linux-musl; do
	rm -rf target/$target
done
endef

amd64:
	python3 build.py --target amd64 --strip --dist

arm64:
	python3 build.py --target arm64 --strip --dist

arm:
	python3 build.py --target arm --strip --dist

armv7:
	python3 build.py --target armv7 --strip --dist

clean:
	$(value remove_releases)

build-all: amd64 arm64 arm armv7

rm-dist:
	rm -rf dist

deploy:
	gsutil -m -h "Cache-Control: no-store, no-transform" rsync dist/ gs://nmk.nuimk.com/

ci: rm-dist build-all deploy

