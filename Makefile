SHELL               = bash

RUSTC               = rustc
RUSTDOC             = rustdoc

SRC_FILE            = levenshtein
TEST_FILE           = $(SRC_FILE)
DEMO_FILE           = demo

CRATE_NAME          = $(shell $(RUSTC) --crate-name $(SRC_FILE))
CRATE_FILES         = $(shell $(RUSTC) --crate-file-name $(SRC_FILE))

LIB_DIR             = target
TEST_DIR            = $(LIB_DIR)/test
DOC_DIR             = doc

.PHONY: all lib test bench check doc clean help

all: lib

clean:
	@echo "--- Removing generated files:"
	rm -rf $(LIB_DIR)
	rm -rf $(DOC_DIR)
	rm -rf $(DEMO_FILE)

help:
	@echo "--- Available Options:"
	@echo "make             - Build the library & documentation."
	@echo "make lib         - Build the library."
	@echo "make test        - Run the unit tests."
	@echo "make bench       - Run benchmarks."
	@echo "make doc         - Builds the library's documentation."
	@echo "make demo        - Builds the demo program."
	@echo "make clean       - Removes all generated files."

# Library

lib: $(SRC_FILE).rs
	@echo "--- Building library."
	@mkdir -p $(LIB_DIR)
	@$(RUSTC) --out-dir=$(LIB_DIR) -O $(SRC_FILE).rs

# Testing and Benchmarking

test: $(TEST_FILE).rs
	@echo "--- Building tests."
	@mkdir -p $(TEST_DIR)
	@$(RUSTC) --out-dir=$(TEST_DIR) -O --test $(TEST_FILE).rs
	@echo "--- Running tests:"
	@$(TEST_DIR)/$(TEST_FILE)

bench: $(TEST_DIR)/$(TEST_FILE)
	@echo "--- Running benchmarks:"
	$(TEST_DIR)/$(TEST_FILE) --bench

# Documentation

doc:
	@echo "--- Generating documentation."
	@mkdir -p $(DOC_DIR)
	@$(RUSTDOC) -o $(DOC_DIR) $(SRC_FILE)

# Demo

demo: $(DEMO_FILE).rs lib
	@echo "--- Building demos."
	@mkdir -p $(TEST_DIR)
	@$(RUSTC) -L $(LIB_DIR) --out-dir=. -O $(DEMO_FILE).rs
