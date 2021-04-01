.PHONY: docker

export CLOUDSDK_ACTIVE_CONFIG_NAME = nui

amd64:
	python3 build.py --target amd64 --strip --dist

arm64:
	python3 build.py --target arm64 --strip --dist

arm:
	python3 build.py --target arm --strip --dist

armv7:
	python3 build.py --target armv7 --strip --dist

clean:
	rm -rf target.cross

build-all: amd64 arm64 arm armv7

rm-dist:
	rm -rf dist

deploy:
	gsutil -m -h "Cache-Control: no-store, no-transform" rsync dist/ gs://nmk.nuimk.com/

ci: rm-dist build-all deploy

