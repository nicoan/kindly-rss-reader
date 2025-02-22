# Extract the version from Cargo.toml
# PACKAGE_VERSION=$(shell cat Cargo.toml | grep version | head -n 1 | awk '{print $$3}' | sed -e 's/"//g')

PACKAGE_VERSION=test

docker-build:
	docker build --tag nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-arm64v8 -f ./dockerfiles/Dockerfile .
	docker build --tag nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-armv6 --platform linux/arm/v6 -f ./dockerfiles/Dockerfile.armv6 .
	docker build --tag nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-armv7 --platform linux/arm/v7 -f ./dockerfiles/Dockerfile.armv7 .

docker-push:
	docker push nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-arm64v8
	docker push nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-armv7
	docker push nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-armv6
	docker manifest create nicoan/kindly-rss-reader:test \
		--amend nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-arm64v8 \
		--amend nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-armv7 \
		--amend nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-armv6
	docker manifest push nicoan/kindly-rss-reader:test
	#docker push nicoan/kindly-rss-reader:latest
	#docker push nicoan/kindly-rss-reader:$(PACKAGE_VERSION)


git-tag-and-push:
	git tag v$(PACKAGE_VERSION)
	git push origin v$(PACKAGE_VERSION)

.PHONY: build-docker push-docker git-tag-and-push
