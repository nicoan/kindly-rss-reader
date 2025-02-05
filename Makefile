# Extract the version from Cargo.toml
PACKAGE_VERSION=$(shell cat Cargo.toml | grep version | head -n 1 | awk '{print $$3}' | sed -e 's/"//g')

docker-build:
	docker build --tag nicoan/kindly-rss-reader:latest --tag nicoan/kindly-rss-reader:$(PACKAGE_VERSION) .

docker-push:
	docker push nicoan/kindly-rss-reader:latest
	docker push nicoan/kindly-rss-reader:$(PACKAGE_VERSION)

.PHONY: build-docker push-docker
