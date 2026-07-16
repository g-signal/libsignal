#
# Copyright (C) 2020 Signal Messenger, LLC.
# SPDX-License-Identifier: AGPL-3.0-only
#

DOCKER ?= docker

.PHONY: docker java_build publish_java test_signing

default: java_build

DOCKER_IMAGE := libsignal-builder
DOCKER_TTY_FLAG := $$(test -t 0 && echo -it)
GRADLE_OPTIONS ?= --dependency-verification strict --stacktrace
CROSS_COMPILE_SERVER ?= -PcrossCompileServer

docker_image:
	cd .. && $(DOCKER) build --build-arg UID=$$(id -u) --build-arg GID=$$(id -g) -t $(DOCKER_IMAGE) -f java/Dockerfile .

java_build: DOCKER_EXTRA=$(shell [ -L build ] && P=$$(readlink build) && echo -v $$P/:$$P )
java_build: docker_image
	$(DOCKER) run $(DOCKER_TTY_FLAG) --init --rm --user $$(id -u):$$(id -g) \
		--env LIBSIGNAL_TESTING_RUN_NONHERMETIC_TESTS \
		--env LIBSIGNAL_TESTING_IGNORE_KT_TESTS \
		--env LIBSIGNAL_TESTING_PROXY_SERVER \
        -v `cd .. && pwd`/:/home/libsignal/src $(DOCKER_EXTRA) $(DOCKER_IMAGE) \
        sh -c "cd src/java; ./gradlew $(GRADLE_OPTIONS) build $(CROSS_COMPILE_SERVER)"

publish_java: DOCKER_EXTRA = $(shell [ -L build ] && P=$$(readlink build) && echo -v $$P/:$$P )
publish_java: docker_image
	@echo "========================================="
	@echo "Publishing Configuration Diagnostics"
	@echo "========================================="
	@echo "Sonatype Username: $${ORG_GRADLE_PROJECT_sonatypeUsername:-<not set>}"
	@echo "Sonatype Password: $$(if [ -n "$$ORG_GRADLE_PROJECT_sonatypePassword" ]; then echo "<set, length=$${#ORG_GRADLE_PROJECT_sonatypePassword}>"; else echo "<not set>"; fi)"
	@echo "Signing Key ID: $${ORG_GRADLE_PROJECT_signingKeyId:-<not set>}"
	@echo "Signing Password: $$(if [ -n "$$ORG_GRADLE_PROJECT_signingPassword" ]; then echo "<set, length=$${#ORG_GRADLE_PROJECT_signingPassword}>"; else echo "<not set>"; fi)"
	@echo "Signing Key: $$(if [ -n "$$ORG_GRADLE_PROJECT_signingKey" ]; then \
		KEY_LEN=$${#ORG_GRADLE_PROJECT_signingKey}; \
		KEY_START=$$(echo "$$ORG_GRADLE_PROJECT_signingKey" | head -c 50); \
		KEY_END=$$(echo "$$ORG_GRADLE_PROJECT_signingKey" | tail -c 50); \
		echo "<set, length=$$KEY_LEN, starts with: $$KEY_START..., ends with: ...$$KEY_END>"; \
	else echo "<not set>"; fi)"
	@echo "========================================="
	$(DOCKER) run --rm --user $$(id -u):$$(id -g) \
		-e ORG_GRADLE_PROJECT_signingKeyId \
		-e ORG_GRADLE_PROJECT_signingPassword \
		-e ORG_GRADLE_PROJECT_signingKey \
		-e CLOUDSDK_AUTH_ACCESS_TOKEN \
		-v `cd .. && pwd`/:/home/libsignal/src $(DOCKER_EXTRA) \
		$(DOCKER_IMAGE) \
		sh -c "cd src/java; ./gradlew $(GRADLE_OPTIONS) publish $(CROSS_COMPILE_SERVER)"

test_signing: DOCKER_EXTRA = $(shell [ -L build ] && P=$$(readlink build) && echo -v $$P/:$$P )
test_signing: docker_image
	@echo "========================================="
	@echo "Test Signing Configuration Diagnostics"
	@echo "========================================="
	@echo "Sonatype Username: $${ORG_GRADLE_PROJECT_sonatypeUsername:-<not set>}"
	@echo "Sonatype Password: $$(if [ -n "$$ORG_GRADLE_PROJECT_sonatypePassword" ]; then echo "<set, length=$${#ORG_GRADLE_PROJECT_sonatypePassword}>"; else echo "<not set>"; fi)"
	@echo "Signing Key ID: $${ORG_GRADLE_PROJECT_signingKeyId:-<not set>}"
	@echo "Signing Password: $$(if [ -n "$$ORG_GRADLE_PROJECT_signingPassword" ]; then echo "<set, length=$${#ORG_GRADLE_PROJECT_signingPassword}>"; else echo "<not set>"; fi)"
	@echo "Signing Key: $$(if [ -n "$$ORG_GRADLE_PROJECT_signingKey" ]; then \
		KEY_LEN=$${#ORG_GRADLE_PROJECT_signingKey}; \
		KEY_START=$$(echo "$$ORG_GRADLE_PROJECT_signingKey" | head -c 50); \
		KEY_END=$$(echo "$$ORG_GRADLE_PROJECT_signingKey" | tail -c 50); \
		echo "<set, length=$$KEY_LEN, starts with: $$KEY_START..., ends with: ...$$KEY_END>"; \
	else echo "<not set>"; fi)"
	@echo "========================================="
	$(DOCKER) run --rm --user $$(id -u):$$(id -g) \
		-v `cd .. && pwd`/:/home/libsignal/src $(DOCKER_EXTRA) \
		-e ORG_GRADLE_PROJECT_sonatypeUsername \
		-e ORG_GRADLE_PROJECT_sonatypePassword \
		-e ORG_GRADLE_PROJECT_signingKeyId \
		-e ORG_GRADLE_PROJECT_signingPassword \
		-e ORG_GRADLE_PROJECT_signingKey \
		$(DOCKER_IMAGE) \
		sh -c "cd src/java; ./gradlew $(GRADLE_OPTIONS) build :android:signMavenJavaPublication :client:signMavenJavaPublication :server:signMavenJavaPublication $(CROSS_COMPILE_SERVER)"

# We could run these through Docker, but they would have the same result anyway.

clean:
	./gradlew clean

format_java:
	./gradlew spotlessApply
