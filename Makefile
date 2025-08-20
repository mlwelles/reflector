.PHONY: build test
build test clean::
	cargo $@

.PHONY: install
install:
	cargo install --path .

.PHONY: clean
clean::
	rm -f *~ *.bak
