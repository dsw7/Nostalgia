.PHONY = help lint check debug release test clean

.DEFAULT_GOAL = help

LIGHT_PURPLE = "\033[1;1;35m"
NO_COLOR = "\033[0m"

define ECHO_STEP
	@echo -e $(LIGHT_PURPLE)\> $(1)$(NO_COLOR)
endef

define HELP_LIST_TARGETS
Lint code
    $$ make lint
Check if project can compile
    $$ make check
Compile a debug binary
    $$ make debug
Compile a release binary
    $$ make release
Build project and run tests
    $$ make test
Remove target directory (in order to save space)
    $$ make clean
endef

export HELP_LIST_TARGETS

help:
	@echo "$$HELP_LIST_TARGETS"

lint:
	$(call ECHO_STEP,Linting project using cargo clippy)
	@cargo clippy --verbose

check:
	$(call ECHO_STEP,Checking project using cargo)
	@cargo check --verbose --jobs=12

debug:
	$(call ECHO_STEP,Compiling debug binary using cargo)
	@cargo build --verbose --jobs=12

release:
	$(call ECHO_STEP,Compiling release binary using cargo)
	@cargo build --verbose --jobs=12 --release

test: debug
	$(call ECHO_STEP,Testing project using pytest)
	@python3 -m pytest --verbose --capture=no

clean:
	$(call ECHO_STEP,Removing "target" directory)
	@cargo clean --verbose
