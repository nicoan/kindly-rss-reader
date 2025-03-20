# Extract the version from Cargo.toml
PACKAGE_VERSION=$(shell cat Cargo.toml | grep version | head -n 1 | awk '{print $$3}' | sed -e 's/"//g')


# Build for different archs
# I opted to use multiple Dockerfiles to take advantage of the layer caching mechanism.
docker-build:
	docker build \
		--tag nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-x86_64 \
		-f ./dockerfiles/Dockerfile.x86_64 \
		--platform linux/amd64 \
		.
	docker build \
		--tag nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-arm64v8 \
		-f ./dockerfiles/Dockerfile.armv8 \
		--platform linux/arm64/v8 \
		.
	docker build \
		--tag nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-armv6 \
		--platform linux/arm/v6 \
		-f ./dockerfiles/Dockerfile.armv6 \
		.
	docker build \
		--tag nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-armv7 \
		--platform linux/arm/v7 \
		-f ./dockerfiles/Dockerfile.armv7 \
		.

# Builds an image with different linkers to be able to build
# for different architectures
docker-prepare-build-image:
	docker build \
		--tag nicoan/kindly-rss-builder \
		-f ./dockerfiles/Dockerfile.build \
		.

# Push new versions
docker-push:
	# Push different architecture images
	docker push nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-x86_64
	docker push nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-arm64v8
	docker push nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-armv7
	docker push nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-armv6
	# Create manifest for the package version and push
	docker manifest create nicoan/kindly-rss-reader:$(PACKAGE_VERSION) \
		--amend nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-x86_64 \
		--amend nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-arm64v8 \
		--amend nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-armv7 \
		--amend nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-armv6
	docker manifest push nicoan/kindly-rss-reader:$(PACKAGE_VERSION)
	# Create manifest for the latest tag and push
	docker manifest rm nicoan/kindly-rss-reader:latest
	docker manifest create nicoan/kindly-rss-reader:latest \
		--amend nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-x86_64 \
		--amend nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-arm64v8 \
		--amend nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-armv7 \
		--amend nicoan/kindly-rss-reader:$(PACKAGE_VERSION)-armv6
	docker manifest push nicoan/kindly-rss-reader:latest


git-tag-and-push:
	git tag v$(PACKAGE_VERSION)
	git push origin v$(PACKAGE_VERSION)

.PHONY: build-docker docker-push docker-prepare-build-image git-tag-and-push
