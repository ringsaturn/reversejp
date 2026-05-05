
install:
	uv run maturin develop --release

build:
	uv build

test: install
	uv run --no-sync pytest tests/
