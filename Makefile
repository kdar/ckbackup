SHELL:=/bin/bash

dist:
	cargo build --release -j 8
	cp target/release/ckbackup.exe dist/
	-rsync -vza --progress --ignore-errors --exclude 'borg' vendor dist

github: clean dist
	$(eval describe := $(shell git describe))
	$(eval FILE := ckbackup-x86_64-pc-windows-gnu-$(describe).zip)
	cd dist && zip -9 -r "$(FILE)" .
	github-release release --user kdar --repo ckbackup --tag "$(describe)" --name "$(describe)"
	github-release upload --user kdar --repo ckbackup --tag "$(describe)" --name "$(FILE)" --file "dist\$(FILE)"

clean:
	rm -rf dist/*

.PHONY: dist
