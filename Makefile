build:
	cd reversejp-rust; pwd; make build
	(cd reversejp-python && uv build --out-dir ./dist)
	(cd reversejp-wasm && make build)

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
	uv run ruff check

fmt:
	cargo fmt
	uv run ruff check
	uv run ruff format
	uv run ruff check --select I --fix .

cloc:
	cloc `pwd` --exclude-dir=target,data,.venv

download-data:
	uv run scripts/download.py
	deno fmt reversejp-rust/data
