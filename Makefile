.PHONY: build test
build test clean::
	cargo $@

test::
	yamllint .github/workflows/*.yml

.PHONY: install
install:
	cargo install --path .

.PHONY: clean
clean::
	rm -f *~ *.bak
