.PHONY = help build test

.DEFAULT_GOAL = help

LIGHT_PURPLE = "\033[1;1;35m"
NO_COLOR = "\033[0m"

define ECHO_STEP
	@echo -e $(LIGHT_PURPLE)\> $(1)$(NO_COLOR)
endef

define HELP_LIST_TARGETS
Build project
    $$ make build
Build project and run tests
    $$ make test
endef

export HELP_LIST_TARGETS

help:
	@echo "$$HELP_LIST_TARGETS"

build:
	$(call ECHO_STEP,Building project using cargo)
	@cargo build --verbose --jobs=12

test: build
	$(call ECHO_STEP,Testing project using pytest)
	@python3 -m pytest --verbose
