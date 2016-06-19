dist:
	cargo build --release -j 8
	cp target/release/ckbackup.exe dist/
	-rsync -vza --progress --ignore-errors vendor/ dist

.PHONY: dist
