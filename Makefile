build:
	cd reversejp-rust; pwd; make build
	(cd reversejp-python && uv build --out-dir ./dist)

test:
	cd reversejp-rust; make test
	cd reversejp-python; make test

clean:
	cd reversejp-rust; cargo clean
	cd reversejp-python; cargo clean
	rm -rf reversejp-python/dist
	rm -rf dist

lint:
	cargo fmt --check
	cargo clippy --fix --allow-dirty --allow-staged
	ruff check

fmt:
	cargo fmt
	ruff check
	ruff format
	ruff check --select I --fix .

cloc:
	cloc `pwd` --exclude-dir=target,data,.venv

download-data:
	uv run scripts/download.py
	deno fmt reversejp-rust/data
